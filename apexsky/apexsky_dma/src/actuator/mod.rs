mod actuator_kmbox;
mod actuator_mem;
mod actuator_qmp;

pub use actuator_kmbox::KmboxAimActuator;
pub use actuator_mem::MemAimHelper;
pub use actuator_qmp::QmpAimActuator;

#[derive(Debug, Clone)]
pub struct AimbotAction {
    pub shift_angles: Option<[f32; 3]>,
    pub force_attack: Option<bool>,
}

pub trait AimActuator {
    async fn perform(&mut self, action: AimbotAction) -> anyhow::Result<()>;
}

pub enum DeviceAimActuator {
    KmboxNet(KmboxAimActuator),
    QemuQmp(QmpAimActuator),
}

impl AimActuator for DeviceAimActuator {
    async fn perform(&mut self, action: AimbotAction) -> anyhow::Result<()> {
        match self {
            DeviceAimActuator::KmboxNet(inner) => inner.perform(action).await,
            DeviceAimActuator::QemuQmp(inner) => inner.perform(action).await,
        }
    }
}

pub(self) fn delta_to_mouse_move(delta: [f32; 3]) -> (i16, i16) {
    (
        (delta[1] * -32.0).round() as i16,
        (delta[0] * 32.0).round() as i16,
    )
}
