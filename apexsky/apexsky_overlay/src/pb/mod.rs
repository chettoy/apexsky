#[cfg(feature = "apex1-inspect")]
pub use apex1_common::pb::apexlegends;
#[cfg(not(feature = "apex1-inspect"))]
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
}

pub mod esp_service {
    include!(concat!(env!("OUT_DIR"), "/com.chettoy.apexsky.esp.rs"));
}

#[cfg(feature = "apex1-inspect")]
pub mod app {
    pub mod ohosky {
        pub mod inspect {
            include!(concat!(env!("OUT_DIR"), "/app.ohosky.inspect.rs"));
        }
    }
}
