pub mod apexlegends {
    include!("com.chettoy.apexsky.apexlegends.rs");
}

pub mod esp_service {
    include!("com.chettoy.apexsky.esp.rs");
}

impl From<[f32; 3]> for apexlegends::Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<apexlegends::Vec3> for [f32; 3] {
    fn from(value: apexlegends::Vec3) -> Self {
        [value.x, value.y, value.z]
    }
}
