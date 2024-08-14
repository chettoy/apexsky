use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::ConditionalSendFuture,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
struct CustomAsset {
    #[allow(dead_code)]
    value: i32,
}

#[derive(Default)]
struct CustomAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CustomAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CustomAssetLoader {
    type Asset = CustomAsset;
    type Settings = ();
    type Error = CustomAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture
           + std::future::Future<
        Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>,
    > {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<CustomAsset>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct Blob {
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct BlobAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BlobAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for BlobAssetLoader {
    type Asset = Blob;
    type Settings = ();
    type Error = BlobAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture
           + std::future::Future<
        Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>,
    > {
        Box::pin(async move {
            debug!("Loading Blob...");
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            Ok(Blob { bytes })
        })
    }
}
