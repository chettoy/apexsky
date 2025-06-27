mod actuator_kmbox;
mod actuator_mem;
mod actuator_qmp;

pub use actuator_kmbox::KmboxAimActuator;
pub use actuator_mem::MemAimHelper;
pub use actuator_qmp::QmpAimActuator;

use enum_dispatch::enum_dispatch;
use ohosky_kmbox::kmbox::{KmboxB, KmboxNet};

#[derive(Debug, Clone)]
pub struct AimbotAction {
    pub shift_angles: Option<[f32; 3]>,
    pub force_attack: Option<bool>,
    pub force_use: Option<bool>,
}

#[enum_dispatch]
pub trait AimActuator {
    async fn perform(&mut self, action: AimbotAction) -> anyhow::Result<()>;
}

#[enum_dispatch(AimActuator)]
pub enum DeviceAimActuator {
    KmboxNet(Box<KmboxAimActuator<KmboxNet>>),
    KmboxB(KmboxAimActuator<KmboxB>),
    QemuQmp(QmpAimActuator),
}

fn delta_to_mouse_move(delta: [f32; 3]) -> (i16, i16) {
    (
        (delta[1] * -32.0).round() as i16,
        (delta[0] * 32.0).round() as i16,
    )
}
