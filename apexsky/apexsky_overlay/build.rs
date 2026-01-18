fn main() {
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(!cfg!(feature = "web-wasm"))
        .compile_protos(
            &[
                "../apex1_common/proto/com/chettoy/apexsky/apexlegends/aimbot.proto",
                "../apex1_common/proto/com/chettoy/apexsky/apexlegends/esp_data.proto",
                "../apex1_common/proto/com/chettoy/apexsky/apexlegends/player.proto",
                "../apex1_common/proto/com/chettoy/apexsky/apexlegends/spectator.proto",
                "../apex1_common/proto/com/chettoy/apexsky/esp/esp_service.proto",
            ],
            &["../apex1_common/proto"],
        )
        .unwrap();

    #[cfg(feature = "apex1-inspect")]
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(!cfg!(feature = "web-wasm"))
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["../apexsky_run/proto/app/ohosky/inspect/inspect_service.proto"],
            &["../apexsky_run/proto"],
        )
        .unwrap();
}
