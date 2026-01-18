use bevy::prelude::*;
#[cfg(feature = "native")]
use bevy::window::{WindowLevel, WindowMode};
use bevy::{color::palettes, window::CompositeAlphaMode};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, winit::WinitSettings};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
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
pub mod utils;

const DRY_RUN: bool = false;
const PRINT_LATENCY: bool = false;

impl Default for EspServiceAddr {
    #[cfg(feature = "native")]
    fn default() -> Self {
        Self::from_str(s!("http://[::1]:50051")).unwrap()
    }

    #[cfg(feature = "web-wasm")]
    fn default() -> Self {
        use once_cell::sync::Lazy;

        static DEFAULT_CONNECT_ADDR: Lazy<EspServiceAddr> = Lazy::new(|| {
            // Parse connect address from URL "?connect=*"
            let location = web_sys::window().unwrap().location();
            if let Ok(params) = location
                .search()
                .and_then(|query_string| web_sys::UrlSearchParams::new_with_str(&query_string))
            {
                if let Some(addr) = params
                    .get("connect")
                    .and_then(|addr| EspServiceAddr::from_str(&addr))
                {
                    return addr;
                }
            }
            EspServiceAddr::from_str(s!("http://[::1]:50051")).unwrap()
        });

        DEFAULT_CONNECT_ADDR.clone()
    }
}

pub(crate) fn main() {
    App::new()
        .register_type::<model::Health>()
        .register_type::<model::Mana>()
        .add_plugins(
            DefaultPlugins
                // // Uncomment to force use of OpenGL Backend
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
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),

                        title: embedded::S_TITLE.to_owned(),

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

                        // Disabling window decorations to make it feel more like a widget than a window
                        #[cfg(feature = "native")]
                        decorations: false,

                        // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                        #[cfg(feature = "native")]
                        transparent: true,

                        #[cfg(feature = "native")]
                        focused: true,

                        #[cfg(feature = "native")]
                        window_level: WindowLevel::AlwaysOnTop,

                        fit_canvas_to_parent: false,

                        // Stop events from propagating out of the canvas element
                        // This value has no effect on non-web platforms.
                        prevent_default_event_handling: true,

                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(embedded::EmbeddedAssetPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins((
            hpbar::HealthBarPlugin::<model::Health>::default(),
            hpbar::HealthBarPlugin::<model::Mana>::default(),
        ))
        .init_asset::<Blob>()
        .init_asset_loader::<BlobAssetLoader>()
        .init_resource::<TokioRuntime>()
        .init_resource::<MyOverlayState>()
        .init_resource::<ui::UiPersistance>()
        .init_resource::<ui::UiState>()
        .init_resource::<system::game_esp::ShowEntityBall>()
        .init_resource::<system::sound::SoundSystem>()
        .init_non_send_resource::<system::sound::SoundBufRes>()
        .init_resource::<system::navigator::NavigatorSystem>()
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        })
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
        .add_systems(Startup, setup)
        .add_systems(Startup, ui::configure_egui_res_system)
        .add_systems(Startup, system::navigator::setup_voice_navigator)
        .add_systems(Update, system::game_esp::request_game_state)
        .add_systems(
            Update,
            system::game_esp::follow_game_state.after(system::game_esp::request_game_state),
        )
        .add_systems(Update, ui::resize_canvas)
        .add_systems(
            EguiPrimaryContextPass,
            ui::ui_system.after(system::game_esp::follow_game_state),
        )
        .add_systems(Update, system::game_esp::despawn_dead_targets)
        .add_systems(Update, ui::toggle_mouse_passthrough)
        .add_systems(Update, system::navigator::update_voice_navigator)
        .add_systems(Update, system::sound::load_test_sound)
        .add_systems(Update, system::sound::update_sound_objects)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut overlay_state: ResMut<MyOverlayState>,
    #[cfg(feature = "web-wasm")] mut windows: Query<&mut Window>,
) {
    #[cfg(feature = "web-wasm")]
    {
        let screen = web_sys::window().unwrap().screen().unwrap();
        let mut window = windows
            .single_mut()
            .expect("Error: Could not find a single window.");

        let scale_factor = window.resolution.base_scale_factor();

        let web_wh: (u32, u32) = (
            screen.width().unwrap().try_into().unwrap(),
            screen.height().unwrap().try_into().unwrap(),
        );
        let web_phys_wh: (u32, u32) = (
            (web_wh.0 as f32 * scale_factor) as u32,
            (web_wh.1 as f32 * scale_factor) as u32,
        );
        tracing::info!(?web_wh, scale_factor, ?web_phys_wh);

        window.resolution.set_scale_factor(1.0);
        window
            .resolution
            .set_physical_resolution(web_phys_wh.0, web_phys_wh.1);
    }

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

    // Space between the two ears
    let gap = 12.0;

    let listener = SpatialListener::new(gap);
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            listener.clone(),
        ))
        .with_children(|parent| {
            // left ear indicator
            parent.spawn((
                Mesh3d::from(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
                MeshMaterial3d::from(materials.add(StandardMaterial {
                    base_color: Color::Srgba(palettes::css::RED),
                    ..default()
                })),
                Transform::from_translation(listener.left_ear_offset),
            ));

            // right ear indicator
            parent.spawn((
                Mesh3d::from(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
                MeshMaterial3d::from(materials.add(StandardMaterial {
                    base_color: Color::Srgba(palettes::css::GREEN),
                    ..default()
                })),
                Transform::from_translation(listener.right_ear_offset),
            ));
        });

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 1000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

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
    commands.spawn((Camera2d, IsDefaultUiCamera));
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Projection::from(PerspectiveProjection {
            fov: 90.0f32.to_radians(),
            far: 8000.0,
            ..Default::default()
        }),
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        model::MyCameraMarker,
    ));
}
