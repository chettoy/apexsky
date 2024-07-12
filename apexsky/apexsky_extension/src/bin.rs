use apexsky_extension::InstallManager;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let usage = || println!("Usage: ./apexsky_extension_packer <out_path> <manifest_path>");

    if args.len() != 3 {
        return usage();
    }
    let Some(out_path) = args.get(1) else {
        return usage();
    };
    let Some(manifest_path) = args.get(2) else {
        return usage();
    };

    let mut mgr = InstallManager::default();

    if let Err(e) = mgr.pack(out_path.into(), manifest_path.into()).await {
        println!("{:?}", e);
    }
}
