use obfstr::obfstr as s;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::manifest::ManifestDoc;
use crate::Manifest;

#[derive(Debug, Clone)]
pub struct UserMod {
    pub package_name: String,
    pub manifest: Manifest,
    pub source: String,
    pub checksum: String,
    pub file_path: PathBuf,
}

#[derive(Default)]
pub struct PackageManager {
    installed: HashMap<String, UserMod>,
}

impl PackageManager {
    pub async fn pack(&mut self, mut out: PathBuf, manifest_path: PathBuf) -> anyhow::Result<()> {
        use async_compression::tokio::write::ZstdEncoder;
        use std::io::BufReader;
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::io::BufWriter;

        if out.is_dir() {
            out.push(s!("out.spk"));
        }
        out.with_extension(s!("spk"));

        let manifest_path = std::fs::canonicalize(manifest_path)?;

        let manifest = {
            let mut file = std::fs::File::open(manifest_path.clone())?;
            let manifest_doc: ManifestDoc = serde_json::from_reader(BufReader::new(&mut file))?;
            Manifest::new(manifest_doc)?
        };

        let Some(main_module) = manifest.get_main_module() else {
            anyhow::bail!("{}", s!("Failed to determine main module"));
        };

        let Some(base_path) = manifest_path.parent() else {
            anyhow::bail!("{}", s!("Failed to determine directory path"));
        };

        let manifest_data = serde_json::to_vec(manifest.get_inner())?;
        let module_data = {
            let mut file = File::open(base_path.join(&main_module)).await?;
            let mut code_buf = Vec::<u8>::new();
            file.read_to_end(&mut code_buf).await?;

            let minify_fn1 = |code_buf: &[u8]| -> anyhow::Result<Vec<u8>> {
                let session = minify_js::Session::new();
                let mut code_out = Vec::<u8>::new();
                minify_js::minify(
                    &session,
                    minify_js::TopLevelMode::Module,
                    &code_buf,
                    &mut code_out,
                )
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                Ok(code_out)
            };

            let minify_fn2 = |code_buf: &[u8]| -> anyhow::Result<Vec<u8>> {
                use oxc::allocator::Allocator;
                use oxc::codegen::WhitespaceRemover;
                use oxc::minifier::{ManglerBuilder, Minifier, MinifierOptions, RemoveDeadCode};
                use oxc::parser::Parser;
                let allocator = Allocator::default();
                let ret = Parser::new(
                    &allocator,
                    std::str::from_utf8(&code_buf)?,
                    oxc::span::SourceType::from_path(&main_module)
                        .map_err(|e| anyhow::anyhow!("{:?}", e))?,
                )
                .parse();
                let program = allocator.alloc(ret.program);
                RemoveDeadCode::new(&allocator).build(program);
                Minifier::new(MinifierOptions {
                    mangle: true,
                    ..MinifierOptions::default()
                })
                .build(&allocator, program);
                let _mangler = ManglerBuilder.build(program);

                let code_out = WhitespaceRemover::new()
                    .build(program)
                    .source_text
                    .as_bytes()
                    .to_vec();
                Ok(code_out)
            };

            std::panic::catch_unwind(|| minify_fn1(&code_buf)).unwrap_or(minify_fn2(&code_buf))?
        };

        let mut archive_buf = Vec::<u8>::new();
        {
            let mut package_archive = tar::Builder::new(&mut archive_buf);
            let mut sha256 = Sha256::new();

            let mut header = tar::Header::new_gnu();
            header.set_size(manifest_data.len().try_into().unwrap());
            header.set_mode(0o444);
            header.set_cksum();
            package_archive.append_data(
                &mut header,
                s!("manifest.json"),
                manifest_data.as_slice(),
            )?;
            std::io::copy(&mut manifest_data.as_slice(), &mut sha256)?;

            let mut header = tar::Header::new_gnu();
            header.set_size(module_data.len().try_into().unwrap());
            header.set_mode(0o555);
            header.set_cksum();
            package_archive.append_data(&mut header, &main_module, module_data.as_slice())?;
            std::io::copy(&mut module_data.as_slice(), &mut sha256)?;

            let hash = sha256.finalize();
            let hash = hex::encode(hash);
            let hash_data = hash.as_bytes();

            let mut header = tar::Header::new_gnu();
            header.set_size(hash_data.len().try_into().unwrap());
            header.set_mode(0o444);
            header.set_cksum();
            package_archive.append_data(&mut header, s!(".checksum"), hash_data)?;

            package_archive.finish()?;
        }

        let mut package_file = File::create(out).await?;
        let mut encoder = ZstdEncoder::new(BufWriter::new(&mut package_file));
        encoder.write_all(&archive_buf).await?;
        encoder.shutdown().await?;

        Ok(())
    }

    pub async fn install(
        &mut self,
        path: PathBuf,
        checksum: Option<String>,
    ) -> Result<String, PackageManagerError> {
        use async_compression::tokio::bufread::ZstdDecoder;
        use std::io::Read;
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;
        use tokio::io::BufReader;

        let mut package_file = File::open(path.clone()).await?;
        let mut decoder = ZstdDecoder::new(BufReader::new(&mut package_file));
        let mut buf = Vec::<u8>::new();
        decoder.read_to_end(&mut buf).await?;

        let mut sha256 = Sha256::new();

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
                return Err(PackageError::MissingManifest.into());
            };
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            std::io::copy(&mut buf.as_slice(), &mut sha256)?;
            let manifest_doc = serde_json::from_slice::<ManifestDoc>(&mut buf.as_slice())
                .map_err(PackageError::ManifestParseError)?;
            Manifest::new(manifest_doc).map_err(PackageError::InvalidManifest)?
        };

        let Some(main_module) = manifest.get_main_module() else {
            return Err(PackageError::MissingMainModule.into());
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
                return Err(PackageError::InvalidMainModule.into());
            };
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            std::io::copy(&mut s.as_bytes(), &mut sha256)?;
            s
        };

        let checksum_filename = s!(".checksum").to_string();
        let package_checksum = {
            let mut package_archive = tar::Archive::new(&buf[..]);
            let mut file = None;
            for entity in package_archive.entries()? {
                let entity = entity?;
                let filename = entity.header().path()?;
                if filename.to_str() == Some(&checksum_filename) {
                    file = Some(entity);
                    break;
                }
            }
            let Some(mut file) = file else {
                return Err(PackageError::MissingChecksum.into());
            };
            let mut s = String::new();
            file.read_to_string(&mut s)?;

            let checksum = hex::encode(sha256.finalize());
            if s != checksum {
                return Err(PackageError::InvalidChecksum.into());
            }

            s
        };

        if checksum.is_some_and(|c| package_checksum != c) {
            return Err(PackageManagerError::TargetChecksumMismatch);
        }

        let package_name = manifest.get_package_name().to_owned();

        self.installed.insert(
            package_name.clone(),
            UserMod {
                package_name: package_name.clone(),
                manifest,
                source,
                checksum: package_checksum,
                file_path: path,
            },
        );

        Ok(package_name)
    }

    pub fn remove(&mut self, package_name: &str) {
        self.installed.remove(package_name);
    }

    pub fn get_installed(&self, package_name: &str) -> Option<&UserMod> {
        self.installed.get(package_name)
    }

    pub fn get_all_installed(&self) -> &HashMap<String, UserMod> {
        &self.installed
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    #[error("missing manifest")]
    MissingManifest,
    #[error("failed to parse manifest")]
    ManifestParseError(#[source] serde_json::Error),
    #[error("invalid manifest")]
    InvalidManifest(#[source] anyhow::Error),
    #[error("missing main module")]
    MissingMainModule,
    #[error("invalid main module")]
    InvalidMainModule,
    #[error("missing checksum")]
    MissingChecksum,
    #[error("invalid checksum")]
    InvalidChecksum,
}

#[derive(thiserror::Error, Debug)]
pub enum PackageManagerError {
    #[error("invalid package file: {0}")]
    InvalidPackageFile(#[from] PackageError),
    #[error("target checksum mismatch")]
    TargetChecksumMismatch,
    #[error("io error {0:?}")]
    IO(#[from] std::io::Error),
    #[error("PackageManagerError: {0:?}")]
    AnyError(#[from] anyhow::Error),
}

#[tokio::test]
async fn test_parse_usermod_package() {
    let mut mgr = PackageManager::default();
    let package_name = mgr.install("./auto_sg.spk".into(), None).await.unwrap();
    println!("{} installed", package_name);
}
