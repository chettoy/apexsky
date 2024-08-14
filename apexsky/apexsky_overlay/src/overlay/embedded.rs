use bevy::asset::embedded_asset;
use bevy::prelude::*;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;

use super::asset::Blob;

pub(super) struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        // We get to choose some prefix relative to the workspace root which
        // will be ignored in "embedded://" asset paths.
        static S_OMIT_PREFIX: Lazy<String> = Lazy::new(|| s!("src/overlay/").to_string());
        // Path to asset must be relative to this file, because that's how
        // include_bytes! works.
        embedded_asset!(app, &*S_OMIT_PREFIX, "assets/fonts/LXGWNeoXiHei.ttf");
        // embedded_asset!(app, &*S_OMIT_PREFIX, "assets/hrir/IRC_1002_C.bin");
        // embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/breakout_collision.ogg");
        // embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/helicopter.wav");
        // embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/sine_440hz.wav");
        // embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/waterfall.ogg");
        embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/Windless Slopes.ogg");
        embedded_asset!(app, &*S_OMIT_PREFIX, "assets/textures/hud_smart.png");
    }
}

pub(crate) static S_TITLE: Lazy<String> =
    Lazy::new(|| s!("Absolutely Not Cheating.exe - Totally Legit Gameplay 😇").to_string());
pub(crate) static S_FONT_PATH: Lazy<String> =
    Lazy::new(|| s!("embedded://apexsky_overlay/assets/fonts/LXGWNeoXiHei.ttf").to_string());
pub(crate) static S_SOUND_PATH: Lazy<String> =
    Lazy::new(|| s!("embedded://apexsky_overlay/assets/sounds/Windless Slopes.ogg").to_string());
pub(crate) static S_ESP_HUD_IMAGE_PATH: Lazy<String> =
    Lazy::new(|| s!("embedded://apexsky_overlay/assets/textures/hud_smart.png").to_string());

#[derive(Deref, Resource)]
pub struct EspHudImage(pub Handle<Image>);

#[derive(Deref, Resource)]
pub struct FontBlob(pub Handle<Blob>);

#[derive(Deref, Resource)]
pub struct Sound0(pub Handle<AudioSource>);

pub(crate) fn setup(
    mut egui_user_textures: ResMut<bevy_egui::EguiUserTextures>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(FontBlob(asset_server.load(&*S_FONT_PATH)));
    commands.insert_resource(Sound0(asset_server.load(&*S_SOUND_PATH)));
    commands.insert_resource({
        let image_handle: Handle<Image> = asset_server.load(&*S_ESP_HUD_IMAGE_PATH);
        egui_user_textures.add_image(image_handle.clone());
        EspHudImage(image_handle)
    });
}
