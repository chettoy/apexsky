fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/pb")
        .compile(
            &[
                "../apexsky_dma/proto/com/chettoy/apexsky/apexlegends/aimbot.proto",
                "../apexsky_dma/proto/com/chettoy/apexsky/apexlegends/esp_data.proto",
                "../apexsky_dma/proto/com/chettoy/apexsky/apexlegends/player.proto",
                "../apexsky_dma/proto/com/chettoy/apexsky/apexlegends/spectator.proto",
                "../apexsky_dma/proto/com/chettoy/apexsky/esp/esp_service.proto",
            ],
            &["../apexsky_dma/proto"],
        )
        .unwrap();
}
