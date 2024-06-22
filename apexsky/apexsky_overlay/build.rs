fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(!cfg!(feature = "web-wasm"))
        .out_dir("src/pb")
        .compile(
            &[
                "../apexsky_proto/proto/com/chettoy/apexsky/apexlegends/aimbot.proto",
                "../apexsky_proto/proto/com/chettoy/apexsky/apexlegends/esp_data.proto",
                "../apexsky_proto/proto/com/chettoy/apexsky/apexlegends/player.proto",
                "../apexsky_proto/proto/com/chettoy/apexsky/apexlegends/spectator.proto",
                "../apexsky_proto/proto/com/chettoy/apexsky/esp/esp_service.proto",
            ],
            &["../apexsky_proto/proto"],
        )
        .unwrap();
}
