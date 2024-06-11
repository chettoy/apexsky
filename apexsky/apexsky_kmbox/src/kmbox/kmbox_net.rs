use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use embedded_graphics::pixelcolor::{IntoStorage, Rgb565, WebColors};
use rand::Rng;
use tokio::{net::UdpSocket, time::timeout};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

use super::KmboxError;

#[derive(Debug)]
#[repr(C)]
pub enum Cmd {
    Connect = 0xaf3c2828,       //ok连接盒子
    MouseMove = 0xaede7345,     //ok鼠标移动
    MouseLeft = 0x9823AE8D,     //ok鼠标左键控制
    MouseMiddle = 0x97a3AE8D,   //ok鼠标中键控制
    MouseRight = 0x238d8212,    //ok鼠标右键控制
    MouseWheel = 0xffeead38,    //ok鼠标滚轮控制
    MouseAutomove = 0xaede7346, //ok鼠标自动模拟人工移动控制
    KeyboardAll = 0x123c2c2f,   //ok键盘所有参数控制
    Reboot = 0xaa8855aa,        //ok盒子重启
    BazerMove = 0xa238455a,     //ok鼠标贝塞尔移动
    Monitor = 0x27388020,       //ok监控盒子上的物理键鼠数据
    Debug = 0x27382021,         //ok开启调试信息
    MaskMouse = 0x23234343,     //ok屏蔽物理键鼠
    UnmaskAll = 0x23344343,     //ok解除屏蔽物理键鼠
    SetConfig = 0x1d3d3323,     //ok设置IP配置信息
    SetVidPid = 0xffed3232,     //ok设置device端的vidpid
    ShowPic = 0x12334883,       //显示图片
    TraceEnable = 0xbbcdddac,   //使能硬件修正功能
}

#[derive(FromZeroes, FromBytes, AsBytes, Debug, Clone, Default)]
#[repr(C)]
pub struct SoftMouse {
    button: i32,
    x: i32,
    y: i32,
    wheel: i32,
    point: [i32; 10], //用于贝塞尔曲线控制(预留5阶导)
}

impl SoftMouse {
    pub fn set_move(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_left_button(&mut self, is_down: bool) {
        if is_down {
            self.button |= 0x01;
        } else {
            self.button &= !0x01;
        }
    }

    pub fn set_middle_button(&mut self, is_down: bool) {
        if is_down {
            self.button |= 0x04;
        } else {
            self.button &= !0x04;
        }
    }

    pub fn set_right_button(&mut self, is_down: bool) {
        if is_down {
            self.button |= 0x02;
        } else {
            self.button &= !0x02;
        }
    }

    pub fn set_wheel(&mut self, wheel: i32) {
        self.wheel = wheel;
    }
}

#[derive(FromZeroes, FromBytes, AsBytes, Debug, Clone)]
#[repr(C)]
pub struct SoftKeyboard {
    ctrl: u8,
    resvel: u8,
    button: [u8; 10],
}

#[derive(FromZeroes, FromBytes, AsBytes, Debug)]
#[repr(C)]
pub struct CmdHead {
    mac: u32,      //盒子的mac地址（必须）
    rand: u32,     //随机值
    indexpts: u32, //时间戳
    cmd: u32,      //指令码
}

#[derive(FromZeroes, FromBytes, AsBytes, Debug, Clone)]
#[repr(C)]
pub struct CmdDataU8 {
    buff: [u8; 1024],
}

#[derive(FromZeroes, FromBytes, AsBytes, Debug, Clone)]
#[repr(C)]
pub struct CmdDataU16 {
    buff: [u16; 512],
}

#[derive(Debug)]
#[repr(C)]
pub enum CmdBody {
    Empty,
    U8Buff(CmdDataU8),         //buff
    U16Buff(CmdDataU16),       //U16
    CmdMouse(SoftMouse),       //鼠标发送指令
    CmdKeyboard(SoftKeyboard), //键盘发送指令
}

#[derive(Debug)]
#[repr(C)]
pub struct ClientPacket {
    head: CmdHead,
    body: CmdBody,
}

impl ClientPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.head.as_bytes());

        match &self.body {
            CmdBody::Empty => (),
            CmdBody::U8Buff(data) => bytes.extend_from_slice(data.as_bytes()),
            CmdBody::U16Buff(data) => bytes.extend_from_slice(data.as_bytes()),
            CmdBody::CmdMouse(data) => bytes.extend_from_slice(data.as_bytes()),
            CmdBody::CmdKeyboard(data) => bytes.extend_from_slice(data.as_bytes()),
        }

        bytes
    }
}

pub enum TrajectoryCorrectionType {
    /// 贝塞尔曲线
    BazerCurve = 0,
    /// 导弹追踪曲线
    MissileTrackingCurve = 1,
    /// 贝塞尔实时
    BazerRealTimeCurve = 2,
    /// RM-RT
    RmRt = 3,
}

/// 数值越大则曲线越平滑。但耗时越高。
pub struct TrajectoryCorrectionStrength(i32);

impl TrajectoryCorrectionStrength {
    pub fn off() -> Self {
        Self(0)
    }

    /// 推荐取值范围16到50之间。
    /// Between 16 and 50
    pub fn recommended(value: u8) -> Self {
        Self(value.max(16).min(50).into())
    }

    /// 最大可以到100。
    /// Between 1 and 100
    pub fn enabled(value: u8) -> Self {
        Self(value.max(1).min(100).into())
    }
}

#[derive(Debug)]
pub struct KmboxNet {
    addr: SocketAddr,
    socket: UdpSocket,
    tx_packet: ClientPacket,
    soft_mouse: SoftMouse,
    soft_keyboard: SoftKeyboard,
}

impl KmboxNet {
    fn check_rx_packet_head(&self, rx_head: CmdHead) -> Result<(), KmboxError> {
        if self.tx_packet.head.cmd != rx_head.cmd {
            Err(KmboxError::NetCmd)
        } else if self.tx_packet.head.indexpts != rx_head.indexpts {
            Err(KmboxError::NetPts)
        } else {
            Ok(())
        }
    }

    pub async fn init(addr: SocketAddr, mac: u32) -> Result<Self, KmboxError> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(KmboxError::CreateSocket)?;

        let tx_packet = ClientPacket {
            head: CmdHead {
                mac,
                rand: rand::thread_rng().gen(),
                indexpts: 0,
                cmd: Cmd::Connect as u32,
            },
            body: CmdBody::Empty,
        };
        socket
            .send_to(&tx_packet.to_bytes(), addr)
            .await
            .map_err(KmboxError::NetTx)?;

        let instance = Self {
            addr,
            socket,
            tx_packet,
            soft_mouse: SoftMouse::new_zeroed(),
            soft_keyboard: SoftKeyboard::new_zeroed(),
        };
        let rx_head = instance.recv_packet_head().await?;
        instance.check_rx_packet_head(rx_head)?;
        Ok(instance)
    }

    async fn recv_packet_head(&self) -> Result<CmdHead, KmboxError> {
        let mut rx_head = CmdHead::new_zeroed();
        match timeout(
            Duration::from_secs(10),
            self.socket.recv_from(rx_head.as_bytes_mut()),
        )
        .await
        {
            Ok(r) => match r {
                Ok((_number_of_bytes, _src_addr)) => Ok(rx_head),
                Err(e) => Err(KmboxError::NetRx(e)),
            },
            Err(e) => Err(KmboxError::NetRxTimeout(e)),
        }
    }

    async fn send_packet(
        &mut self,
        cmd: Cmd,
        rand_data: Option<u32>,
        body: CmdBody,
    ) -> Result<(), KmboxError> {
        self.tx_packet = ClientPacket {
            head: CmdHead {
                mac: self.tx_packet.head.mac,
                rand: rand_data.unwrap_or(rand::thread_rng().gen()),
                indexpts: self.tx_packet.head.indexpts + 1,
                cmd: cmd as u32,
            },
            body,
        };
        self.socket
            .send_to(&self.tx_packet.to_bytes(), self.addr)
            .await
            .map_err(KmboxError::NetTx)?;
        Ok(())
    }

    pub async fn reboot(&mut self) -> Result<(), KmboxError> {
        self.send_packet(Cmd::Reboot, None, CmdBody::Empty).await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)?;

        // TODO: Need to reset the state in tx_packet?

        Ok(())
    }

    /// 开启盒子内部打印信息并发送到指定端口（调试使用）
    ///
    /// Enables the box to print information internally and send it to a specified port (for debugging)
    pub async fn debug(&mut self, port: u16, enable: u8) -> Result<(), KmboxError> {
        self.send_packet(
            Cmd::Debug,
            Some((port as u32) | ((enable as u32) << 16)),
            CmdBody::Empty,
        )
        .await?;

        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 设置配置信息  改IP与端口号
    ///
    /// Setting Configuration Information
    /// Changing IP and Port Numbers
    pub async fn set_addr_config(&mut self, addr: Ipv4Addr, port: u16) -> Result<(), KmboxError> {
        let octets = addr.octets();
        let inet_addr = u32::from_be_bytes(octets);
        let mut buff: [u8; 1024] = [0; 1024];
        buff[0] = (port >> 8) as u8;
        buff[1] = (port >> 0) as u8;

        self.send_packet(
            Cmd::SetConfig,
            Some(inet_addr),
            CmdBody::U8Buff(CmdDataU8 { buff }),
        )
        .await?;

        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 设置盒子device端的VIDPID
    ///
    /// Setting the VID and the PID of the box device
    pub async fn set_vidpid(&mut self, vid: u16, pid: u16) -> Result<(), KmboxError> {
        self.send_packet(
            Cmd::SetVidPid,
            Some((vid as u32) | ((pid as u32) << 16)),
            CmdBody::Empty,
        )
        .await?;

        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    pub async fn lcd_color(&mut self, color: Rgb565) -> Result<(), KmboxError> {
        for y in 0..40 {
            self.send_packet(
                Cmd::ShowPic,
                Some(y * 4),
                CmdBody::U16Buff(CmdDataU16 {
                    buff: [color.into_storage(); 512],
                }),
            )
            .await?;
            self.check_rx_packet_head(self.recv_packet_head().await?)?;
        }
        Ok(())
    }

    pub async fn lcd_logo(&mut self) -> Result<(), KmboxError> {
        use embedded_graphics::{
            geometry::{Dimensions, Point},
            mock_display::MockDisplay,
            mono_font::{ascii::FONT_6X10, MonoTextStyle},
            pixelcolor::RgbColor,
            primitives::{Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
            text::{Alignment, Text},
            Drawable,
        };

        let draw = || -> Result<[u16; 128 * 160], anyhow::Error> {
            // Create a new mock display
            let mut display: MockDisplay<Rgb565> = MockDisplay::new();

            // Create styles used by the drawing operations.
            let _thin_stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 1);
            let _thick_stroke = PrimitiveStyle::with_stroke(Rgb565::WHITE, 3);
            let border_stroke = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::WHITE)
                .stroke_width(3)
                .stroke_alignment(StrokeAlignment::Inside)
                .build();
            let _fill = PrimitiveStyle::with_fill(Rgb565::WHITE);
            let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_LIGHT_BLUE);

            // Draw a 3px wide outline around the display.
            display
                .bounding_box()
                .into_styled(border_stroke)
                .draw(&mut display)?;

            // Draw centered text.
            obfstr::obfstr! {
                let text = "apexsky";
            }
            Text::with_alignment(
                text,
                display.bounding_box().center() + Point::new(0, 15),
                character_style,
                Alignment::Center,
            )
            .draw(&mut display)?;

            let mut pixels: [u16; 128 * 160] = [0; 128 * 160];
            for y in 0..160 {
                for x in 0..128 {
                    pixels[y * 128 + x] = display
                        .get_pixel(Point {
                            x: x.try_into()?,
                            y: y.try_into()?,
                        })
                        .unwrap_or(Rgb565::BLACK)
                        .into_storage();
                }
            }

            Ok(pixels)
        };

        let pixels = draw().map_err(KmboxError::AnyError)?;

        let (chunks, _) = pixels.as_chunks::<512>();
        for (y, data) in chunks.iter().enumerate() {
            self.send_packet(
                Cmd::ShowPic,
                Some((y * 4).try_into().unwrap()),
                CmdBody::U16Buff(CmdDataU16 { buff: *data }),
            )
            .await?;
            self.check_rx_packet_head(self.recv_packet_head().await?)?;
        }

        Ok(())
    }

    /// 鼠标移动x,y个单位。一次性移动。无轨迹模拟，速度最快.
    /// 自己写轨迹移动时使用此函数。
    ///
    /// Mouse moves x,y units. One move at a time. Fastest simulation without trajectory.
    /// Use this function when writing your own trajectory moves.
    pub async fn mouse_move(&mut self, x: i16, y: i16) -> Result<(), KmboxError> {
        self.soft_mouse.set_move(x.into(), y.into());
        self.send_packet(
            Cmd::MouseMove,
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标左键控制
    /// is_down :false松开 ，true按下
    ///
    /// Left mouse button control
    /// is_down: false=release, true=down
    pub async fn mouse_left(&mut self, is_down: bool) -> Result<(), KmboxError> {
        self.soft_mouse.set_left_button(is_down);
        self.send_packet(
            Cmd::MouseLeft,
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标中键控制
    /// is_down :false松开 ，true按下
    ///
    /// Middle mouse button control
    /// is_down: false=release, true=down
    pub async fn mouse_middle(&mut self, is_down: bool) -> Result<(), KmboxError> {
        self.soft_mouse.set_middle_button(is_down);
        self.send_packet(
            Cmd::MouseMiddle,
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标右键控制
    /// is_down :false松开 ，true按下
    ///
    /// Right mouse button control
    /// is_down: false=release, true=down
    pub async fn mouse_right(&mut self, is_down: bool) -> Result<(), KmboxError> {
        self.soft_mouse.set_right_button(is_down);
        self.send_packet(
            Cmd::MouseRight,
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标滚轮控制
    ///
    /// Mouse wheel control
    pub async fn mouse_wheel(&mut self, wheel: i32) -> Result<(), KmboxError> {
        self.soft_mouse.set_wheel(wheel);
        self.send_packet(
            Cmd::MouseWheel,
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标全报告控制函数
    ///
    /// Mouse Full Report Control Functions
    pub async fn mouse_all(&mut self, state: SoftMouse) -> Result<(), KmboxError> {
        self.soft_mouse = state;
        self.send_packet(
            Cmd::MouseWheel, //??
            None,
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 鼠标移动x,y个单位。模拟人为移动x,y个单位。不会出现键鼠异常的检测.
    /// 没有写移动曲线的推荐用此函数。此函数不会出现跳跃现象，按照最小步进逼近
    /// 目标点。耗时比kmNet_mouse_move高。
    /// ms是设置移动需要多少毫秒.注意ms给的值不要太小，太小一样会出现键鼠数据异常。
    /// 尽量像人操作。实际用时会比ms小。
    ///
    /// Mouse moves x,y units. Simulates human movement of x,y units. No keystroke anomalies will be detected.
    /// This function is recommended if you don't have a movement curve. This function does not jump, and approaches the
    /// The target point. It takes more time than kmNet_mouse_move.
    /// ms sets how many milliseconds the move takes. Be careful not to give too small a value for ms, as too small a value will result in keystroke data anomalies.
    /// Try to be as human-like as possible. In practice, it will be smaller than ms.
    pub async fn mouse_move_auto(&mut self, x: i32, y: i32, ms: u32) -> Result<(), KmboxError> {
        self.soft_mouse.set_move(x, y);
        self.send_packet(
            Cmd::MouseAutomove,
            Some(ms),
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.soft_mouse.set_move(0, 0);
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 二阶贝塞尔曲线控制
    /// x,y 	:目标点坐标
    /// ms		:拟合此过程用时（单位ms）
    /// x1,y1	:控制点p1点坐标
    /// x2,y2	:控制点p2点坐标
    ///
    /// Second-order Bézier curve control
    /// x,y : coordinates of the target point.
    /// ms :time to fit the process (in ms)
    /// x1,y1 :coordinates of control point p1
    /// x2,y2 :coordinates of point p2.
    pub async fn mouse_move_beizer(
        &mut self,
        x: i32,
        y: i32,
        ms: u32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Result<(), KmboxError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        self.soft_mouse.point[0] = x1;
        self.soft_mouse.point[1] = y1;
        self.soft_mouse.point[2] = x2;
        self.soft_mouse.point[3] = y2;

        self.send_packet(
            Cmd::BazerMove,
            Some(ms),
            CmdBody::CmdMouse(self.soft_mouse.clone()),
        )
        .await?;
        self.soft_mouse.set_move(0, 0);
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }

    /// 使能盒子的硬件修正功能
    pub async fn enable_trajectory_correction(
        &mut self,
        correction_type: TrajectoryCorrectionType,
        value: TrajectoryCorrectionStrength,
    ) -> Result<(), KmboxError> {
        self.send_packet(
            Cmd::TraceEnable,
            Some(((correction_type as i32) << 24 | value.0) as u32),
            CmdBody::Empty,
        )
        .await?;
        self.check_rx_packet_head(self.recv_packet_head().await?)
    }
}
