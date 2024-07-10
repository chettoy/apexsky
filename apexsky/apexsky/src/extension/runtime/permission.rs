use serde::{Deserialize, Serialize};

use crate::extension::manifest::PermissionField;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimePermission {
    pub game_world_access: bool,
    pub game_input_access: bool,
    pub memory_access: bool,
    pub memory_modify: bool,
    pub settings_access: bool,
    pub settings_modify: bool,
    pub internet: bool,
}

impl From<&Vec<PermissionField>> for RuntimePermission {
    fn from(value: &Vec<PermissionField>) -> Self {
        let mut p = Self::default();
        for item in value {
            match item {
                PermissionField::AccessGameWorld(_) => p.game_world_access = true,
                PermissionField::AccessGameInput(_) => p.game_input_access = true,
                PermissionField::AccessGameMemory(_) => p.memory_access = true,
                PermissionField::AccessGlobalSettings(_) => p.settings_access = true,
                PermissionField::Bluetooth(_) => (),
                PermissionField::Camera(_) => (),
                PermissionField::ManageGlobalSettings(_) => p.settings_modify = true,
                PermissionField::Internet(_) => p.internet = true,
                PermissionField::ModifyMemory(_) => p.memory_modify = true,
                PermissionField::Overlay(_) => (),
                PermissionField::QueryAllPackages(_) => (),
                PermissionField::RecordAudio(_) => (),
                PermissionField::Storage(_) => (),
                PermissionField::SendAimbotActions(_) => (),
                PermissionField::SendInputActions(_) => (),
                PermissionField::ApexInjectHighlight(_) => (),
            }
        }
        p
    }
}
