fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["proto/apexlegends/player.proto"], &["proto"])
        .unwrap();
}
