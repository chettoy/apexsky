fn main() {
    tonic_build::configure()
        .build_client(cfg!(feature = "tonic"))
        .build_server(cfg!(feature = "tonic"))
        .build_transport(cfg!(feature = "tonic"))
        .type_attribute(
            ".",
            "#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]",
        )
        .type_attribute(".", "#[rkyv(compare(PartialEq), derive(Debug))]")
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_protos(
            &[
                "proto/com/chettoy/apexsky/apexlegends/aimbot.proto",
                "proto/com/chettoy/apexsky/apexlegends/skydream.proto",
                "proto/com/chettoy/apexsky/apexlegends/esp_data.proto",
                "proto/com/chettoy/apexsky/apexlegends/player.proto",
                "proto/com/chettoy/apexsky/apexlegends/spectator.proto",
                "proto/com/chettoy/apexsky/esp/esp_service.proto",
            ],
            &["proto"],
        )
        .unwrap();

    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/apexlegends/aimbot.proto");
    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/apexlegends/skydream.proto");
    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/apexlegends/esp_data.proto");
    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/apexlegends/player.proto");
    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/apexlegends/spectator.proto");
    println!("cargo::rerun-if-changed=proto/com/chettoy/apexsky/esp/esp_service.proto");
}
