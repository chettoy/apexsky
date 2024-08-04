use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
#[cfg(feature = "native")]
use bevy::window::{WindowLevel, WindowMode};
use bevy::{color::palettes, window::CompositeAlphaMode};
use bevy_egui::EguiPlugin;
use bevy_health_bar3d::prelude as hpbar;
use model::{MyOverlayState, TokioRuntime};
use obfstr::obfstr as s;
use system::game_esp::EspServiceAddr;

use crate::overlay::asset::{Blob, BlobAssetLoader};

mod asset;
mod embedded;
mod model;
mod system;
mod ui;
mod utils;

const DRY_RUN: bool = false;
const PRINT_LATENCY: bool = false;

impl Default for EspServiceAddr {
    fn default() -> Self {
        Self::from_str(s!("http://[::1]:50051")).unwrap()
    }
}

pub(crate) fn main() {
    App::new()
        .register_type::<model::Health>()
        .register_type::<model::Mana>()
        .add_plugins((
            DefaultPlugins
                // .set(RenderPlugin {
                //     render_creation: WgpuSettings {
                //         backends: Some(Backends::GL),
                //         ..Default::default()
                //     }
                //     .into(),
                //     ..Default::default()
                // })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        #[cfg(feature = "native")]
                        mode: WindowMode::BorderlessFullscreen,
                        // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                        #[cfg(feature = "native")]
                        transparent: true,
                        #[cfg(feature = "native")]
                        focused: true,
                        #[cfg(feature = "native")]
                        window_level: WindowLevel::AlwaysOnTop,
                        // Disabling window decorations to make it feel more like a widget than a window
                        #[cfg(feature = "native")]
                        decorations: false,
                        #[cfg(target_os = "macos")]
                        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        #[cfg(target_os = "linux")]
                        composite_alpha_mode: {
                            let args: Vec<String> = std::env::args().collect();
                            if args.len() == 2 && args.get(1).is_some_and(|arg1| arg1 == s!("fixa"))
                            {
                                CompositeAlphaMode::Auto
                            } else {
                                CompositeAlphaMode::PreMultiplied
                            }
                        },
                        title: embedded::S_TITLE.to_owned(),
                        ..default()
                    }),
                    ..default()
                }),
            embedded::EmbeddedAssetPlugin,
            FrameTimeDiagnosticsPlugin,
            EguiPlugin,
            hpbar::HealthBarPlugin::<model::Health>::default(),
            hpbar::HealthBarPlugin::<model::Mana>::default(),
        ))
        .init_asset::<Blob>()
        .init_asset_loader::<BlobAssetLoader>()
        .insert_resource(
            hpbar::ColorScheme::<model::Health>::new()
                .foreground_color(hpbar::ForegroundColor::Static(Color::Srgba(
                    palettes::css::LIGHT_GREEN,
                )))
                .background_color(Color::Srgba(palettes::css::RED)),
        )
        .insert_resource(hpbar::ColorScheme::<model::Mana>::new().foreground_color(
            hpbar::ForegroundColor::Static(Color::Srgba(palettes::css::BISQUE)),
        ))
        .init_resource::<TokioRuntime>()
        .init_resource::<MyOverlayState>()
        .init_resource::<ui::UiPersistance>()
        .init_resource::<ui::UiState>()
        .init_resource::<system::game_esp::ShowEntityBall>()
        .init_resource::<system::sound::SoundSystem>()
        .init_non_send_resource::<system::sound::SoundBufRes>()
        .init_resource::<system::navigator::NavigatorSystem>()
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(Startup, system::navigator::setup_voice_navigator)
        .add_systems(Update, system::navigator::update_voice_navigator)
        .add_systems(Update, system::sound::load_test_sound)
        .add_systems(Update, system::sound::update_sound_objects)
        .add_systems(Update, system::game_esp::request_game_state)
        .add_systems(
            Update,
            system::game_esp::follow_game_state.after(system::game_esp::request_game_state),
        )
        .add_systems(Update, system::game_esp::despawn_dead_targets)
        .add_systems(Update, ui::toggle_mouse_passthrough)
        .add_systems(
            Update,
            ui::ui_system.after(system::game_esp::follow_game_state),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut overlay_state: ResMut<MyOverlayState>,
) {
    overlay_state.font_blob = asset_server.load(&*embedded::S_FONT_PATH);
    overlay_state.sound_handle = asset_server.load(&*embedded::S_SOUND_PATH);

    // Space between the two ears
    let gap = 12.0;

    let listener = SpatialListener::new(gap);
    commands
        .spawn((SpatialBundle::default(), listener.clone()))
        .with_children(|parent| {
            // left ear indicator
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                material: materials.add(StandardMaterial {
                    base_color: Color::Srgba(palettes::css::RED),
                    ..default()
                }),
                transform: Transform::from_translation(listener.left_ear_offset),
                ..default()
            });

            // right ear indicator
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                material: materials.add(StandardMaterial {
                    base_color: Color::Srgba(palettes::css::GREEN),
                    ..default()
                }),
                transform: Transform::from_translation(listener.right_ear_offset),
                ..default()
            });
        });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 1000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // // example instructions
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Up/Down/Left/Right: Move Listener\nSpace: Toggle Emitter Movement",
    //         TextStyle {
    //             font_size: 20.0,
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     }),
    // );

    // camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 90.0f32.to_radians(),
                far: 8000.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        model::MyCameraMarker,
    ));

    if cfg!(feature = "web-wasm") {
        overlay_state.black_background = true;
        commands.insert_resource(ClearColor(Color::BLACK));
    }
    if cfg!(feature = "native") {
        //overlay_state.user_gesture = true;

        #[cfg(feature = "native")]
        match ui::UiPersistance::load_persistance() {
            Ok(saved_ui_state) => {
                commands.insert_resource(saved_ui_state);
            }
            Err(e) => match e.downcast::<std::io::Error>() {
                Ok(e) => {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        tracing::error!(%e, ?e)
                    }
                }
                Err(e) => tracing::error!(%e, ?e),
            },
        }
    }
}
