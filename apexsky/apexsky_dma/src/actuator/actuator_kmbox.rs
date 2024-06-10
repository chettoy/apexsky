use std::net::SocketAddr;

use apexsky_kmbox::kmbox::{KmboxNet, SoftMouse};

use super::{delta_to_mouse_move, AimActuator};

#[derive(Debug)]
pub struct KmboxAimActuator {
    kmbox: KmboxNet,
}

impl KmboxAimActuator {
    pub async fn connect(addr: SocketAddr, mac: u32) -> anyhow::Result<Self> {
        let mut kmbox = KmboxNet::init(addr, mac).await?;
        kmbox.lcd_logo().await?;
        Ok(Self { kmbox })
    }
}

impl AimActuator for KmboxAimActuator {
    async fn perform(&mut self, action: super::AimbotAction) -> anyhow::Result<()> {
        match (action.shift_angles, action.force_attack) {
            (None, None) => Ok(()),
            (None, Some(press)) => {
                self.kmbox.mouse_left(press).await?;
                Ok(())
            }
            (Some(delta), None) => {
                let update = delta_to_mouse_move(delta);
                self.kmbox.mouse_move(update.0, update.1).await?;
                Ok(())
            }
            (Some(delta), Some(press)) => {
                let update = delta_to_mouse_move(delta);
                let mut soft_mouse = SoftMouse::default();
                soft_mouse.set_left_button(press);
                soft_mouse.set_move(update.0.into(), update.1.into());
                self.kmbox.mouse_all(soft_mouse).await?;
                Ok(())
            }
        }
    }
}
