pub mod apexlegends {
    include!(concat!(
        env!("OUT_DIR"),
        "/com.chettoy.apexsky.apexlegends.rs"
    ));

    impl From<[f32; 3]> for Vec3 {
        fn from(value: [f32; 3]) -> Self {
            Self {
                x: value[0],
                y: value[1],
                z: value[2],
            }
        }
    }

    impl From<Vec3> for [f32; 3] {
        fn from(value: Vec3) -> Self {
            [value.x, value.y, value.z]
        }
    }

    impl From<ArchivedVec3> for [f32; 3] {
        fn from(value: ArchivedVec3) -> Self {
            [value.x.into(), value.y.into(), value.z.into()]
        }
    }

    impl From<&ArchivedVec3> for [f32; 3] {
        fn from(value: &ArchivedVec3) -> Self {
            [value.x.into(), value.y.into(), value.z.into()]
        }
    }

    impl Default for ArchivedVec3 {
        fn default() -> Self {
            ArchivedVec3 {
                x: 0.0.into(),
                y: 0.0.into(),
                z: 0.0.into(),
            }
        }
    }
}

pub mod esp_service {
    include!(concat!(env!("OUT_DIR"), "/com.chettoy.apexsky.esp.rs"));
}
