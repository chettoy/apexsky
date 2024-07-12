use anyhow::Context;
use obfstr::obfstr as s;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::manifest::ManifestDoc;
use crate::Manifest;

#[derive(Debug, Clone)]
pub struct UserMod {
    pub package_name: String,
    pub manifest: Manifest,
    pub source: String,
}

#[derive(Default)]
pub struct InstallManager {
    installed: HashMap<String, UserMod>,
}

impl InstallManager {
    pub async fn pack(&mut self, out: PathBuf, manifest_path: PathBuf) -> anyhow::Result<()> {
        use async_compression::tokio::write::ZstdEncoder;
        use std::io::BufReader;
        use tokio::fs::File;
        use tokio::io::AsyncWriteExt;
        use tokio::io::BufWriter;

        let manifest_path = std::fs::canonicalize(manifest_path)?;
        let mut manifest_file = std::fs::File::open(manifest_path.clone())?;
        let manifest: ManifestDoc = serde_json::from_reader(BufReader::new(&mut manifest_file))?;
        let manifest = Manifest::new(manifest)?;

        let Some(main_module) = manifest.get_main_module() else {
            anyhow::bail!("{}", s!("Failed to determine main module"));
        };

        let Some(base_path) = manifest_path.parent() else {
            anyhow::bail!("{}", s!("Failed to determine directory path"));
        };
        let worker_path = base_path.join(&main_module);

        let mut buf = Vec::<u8>::new();
        {
            let mut package_archive = tar::Builder::new(&mut buf);

            let manifest_data = serde_json::to_vec_pretty(manifest.get_inner())?;
            let mut header = tar::Header::new_gnu();
            header.set_size(manifest_data.len().try_into().unwrap());
            header.set_mode(0o444);
            header.set_cksum();
            package_archive.append_data(
                &mut header,
                s!("manifest.json"),
                manifest_data.as_slice(),
            )?;

            let mut worker_file = std::fs::File::open(worker_path)?;
            package_archive.append_file(&main_module, &mut worker_file)?;

            package_archive.finish()?;
        }

        let mut package_file = File::create(out).await?;
        let mut encoder = ZstdEncoder::new(BufWriter::new(&mut package_file));
        encoder.write_all(&buf).await?;
        encoder.shutdown().await?;

        Ok(())
    }

    pub async fn install(&mut self, path: PathBuf) -> anyhow::Result<String> {
        use async_compression::tokio::bufread::ZstdDecoder;
        use std::io::Read;
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        use tokio::io::BufReader;

        let mut package_file = File::open(path).await?;
        let mut decoder = ZstdDecoder::new(BufReader::new(&mut package_file));
        let mut buf = Vec::<u8>::new();
        decoder.read_to_end(&mut buf).await?;

        let manifest_filename = s!("manifest.json").to_owned();
        let manifest = {
            let mut package_archive = tar::Archive::new(&buf[..]);
            let mut file = None;
            for entity in package_archive.entries()? {
                let entity = entity?;
                let filename = entity.header().path()?;
                if filename.to_str() == Some(manifest_filename.as_str()) {
                    file = Some(entity);
                    break;
                }
            }
            let Some(mut file) = file else {
                anyhow::bail!("{}", s!("Invalid package file"));
            };
            let manifest_doc = serde_json::from_reader::<_, ManifestDoc>(&mut file)
                .context(anyhow::anyhow!("{}", s!("Failed to parse manifest.json")))?;
            Manifest::new(manifest_doc)?
        };

        let Some(main_module) = manifest.get_main_module() else {
            anyhow::bail!("{}", s!("Failed to determine main module"));
        };
        let source = {
            let mut package_archive = tar::Archive::new(&buf[..]);
            let mut file = None;
            for entity in package_archive.entries()? {
                let entity = entity?;
                let filename = entity.header().path()?;
                if filename.to_str() == Some(main_module.as_str()) {
                    file = Some(entity);
                    break;
                }
            }
            let Some(mut file) = file else {
                anyhow::bail!("{}", s!("Failed to read main module"));
            };
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            s
        };

        let package_name = manifest.get_package_name().to_owned();

        self.installed.insert(
            package_name.clone(),
            UserMod {
                package_name: package_name.clone(),
                manifest,
                source,
            },
        );

        Ok(package_name)
    }

    pub fn get_installed<'a>(&'a self, package_name: &str) -> Option<&UserMod> {
        self.installed.get(package_name)
    }
}

#[tokio::test]
async fn test_parse_usermod_package() {
    let mut mgr = InstallManager::default();
    let package_name = mgr.install("./auto_sg.spk".into()).await.unwrap();
    println!("{} installed", package_name);
}
