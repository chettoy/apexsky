use std::net::SocketAddr;

use apexsky_kmbox::kmbox::{KmboxNet, SoftMouse};

use super::AimExecuter;

#[derive(Debug)]
pub struct KmboxAimExecuter {
    kmbox: KmboxNet,
}

impl KmboxAimExecuter {
    pub async fn connect(addr: SocketAddr, mac: u32) -> anyhow::Result<Self> {
        let kmbox = KmboxNet::init(addr, mac).await?;
        Ok(Self { kmbox })
    }
}

impl AimExecuter for KmboxAimExecuter {
    async fn perform(&mut self, action: super::AimbotAction) -> anyhow::Result<()> {
        match (action.shift_angles, action.force_attack) {
            (None, None) => Ok(()),
            (None, Some(press)) => {
                self.kmbox.mouse_left(press).await?;
                Ok(())
            }
            (Some(delta), None) => {
                self.kmbox
                    .mouse_move(delta[1].round() as i16, delta[0].round() as i16)
                    .await?;
                Ok(())
            }
            (Some(delta), Some(press)) => {
                let mut soft_mouse = SoftMouse::default();
                soft_mouse.set_left_button(press);
                soft_mouse.set_move(delta[1].round() as i32, delta[0].round() as i32);
                self.kmbox.mouse_all(soft_mouse).await?;
                Ok(())
            }
        }
    }
}
