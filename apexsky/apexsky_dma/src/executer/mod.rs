mod kmbox_aim;
mod mem_aim;

pub use kmbox_aim::KmboxAimExecuter;
pub use mem_aim::{MemAimExecuter, MemAimHelper};

#[derive(Debug, Clone)]
pub struct AimbotAction {
    pub shift_angles: Option<[f32; 3]>,
    pub force_attack: Option<bool>,
}

pub trait AimExecuter {
    async fn perform(&mut self, action: AimbotAction) -> anyhow::Result<()>;
}
