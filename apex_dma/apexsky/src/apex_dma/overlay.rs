use std::sync::Arc;

use apexsky::aimbot::AimEntity;
use bevy::asset::embedded_asset;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, WindowLevel, WindowMode};
use bevy_egui::EguiPlugin;
use obfstr::obfstr as s;
use tokio::sync::Mutex;

use crate::SharedState;

mod ui;

pub(crate) fn main(shared_state: Arc<Mutex<SharedState>>) {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
                    transparent: true,
                    focused: true,
                    window_level: WindowLevel::AlwaysOnTop,
                    // Disabling window decorations to make it feel more like a widget than a window
                    decorations: false,
                    #[cfg(target_os = "macos")]
                    composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                    #[cfg(target_os = "linux")]
                    composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                    title: s!("Absolutely Not Cheating.exe - Totally Legit Gameplay 😇")
                        .to_string(),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin,
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(EguiPlugin)
        .insert_resource(MyOverlayState { shared_state })
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, ui::toggle_mouse_passthrough)
        .add_systems(Update, ui::ui_system)
        .add_systems(Update, update_positions)
        .add_systems(Update, update_listener)
        .add_systems(Update, follow_game_state)
        .run();
}

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        // We get to choose some prefix relative to the workspace root which
        // will be ignored in "embedded://" asset paths.
        let omit_prefix = s!("src/apex_dma/").to_string();
        // Path to asset must be relative to this file, because that's how
        // include_bytes! works.
        embedded_asset!(app, omit_prefix, "assets/fonts/LXGWNeoXiHei.ttf");
        embedded_asset!(app, omit_prefix, "assets/sounds/Windless Slopes.ogg");
    }
}

#[derive(Resource)]
pub(crate) struct MyOverlayState {
    shared_state: Arc<Mutex<SharedState>>,
}

#[derive(Component, Default)]
struct AimTarget {
    data: Option<Box<dyn AimEntity>>,
}

#[derive(Component)]
struct MyCameraMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Space between the two ears
    let gap = 4.0;

    // aim target
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(10.0).mesh().uv(32, 18)),
            material: materials.add(Color::GREEN),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        AimTarget::default(),
        AudioBundle {
            source: asset_server
                .load(s!("embedded://apexsky_dma/assets/sounds/Windless Slopes.ogg").to_string()),
            settings: PlaybackSettings::LOOP.with_spatial(true),
        },
    ));

    // sound emitter
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.2).mesh().uv(32, 18)),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Emitter::default(),
        AudioBundle {
            source: asset_server
                .load(s!("embedded://apexsky_dma/assets/sounds/Windless Slopes.ogg").to_string()),
            settings: PlaybackSettings::LOOP.with_spatial(true),
        },
    ));

    let listener = SpatialListener::new(gap);
    commands
        .spawn((SpatialBundle::default(), listener.clone()))
        .with_children(|parent| {
            // left ear indicator
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                material: materials.add(Color::RED),
                transform: Transform::from_translation(listener.left_ear_offset),
                ..default()
            });

            // right ear indicator
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                material: materials.add(Color::GREEN),
                transform: Transform::from_translation(listener.right_ear_offset),
                ..default()
            });
        });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // example instructions
    commands.spawn(
        TextBundle::from_section(
            "Up/Down/Left/Right: Move Listener\nSpace: Toggle Emitter Movement",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    // camera
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 90.0f32.to_radians(),
                far: 10000.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

#[derive(Component, Default)]
struct Emitter {
    stopped: bool,
}

fn update_positions(
    time: Res<Time>,
    mut emitters: Query<(&mut Transform, &mut Emitter), With<Emitter>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut emitter_transform, mut emitter) in emitters.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            emitter.stopped = !emitter.stopped;
        }

        if !emitter.stopped {
            emitter_transform.translation.x = time.elapsed_seconds().sin() * 3.0;
            emitter_transform.translation.z = time.elapsed_seconds().cos() * 3.0;
        }
    }
}

fn update_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut listeners: Query<&mut Transform, With<SpatialListener>>,
) {
    let mut transform = listeners.single_mut();

    let speed = 2.;

    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.z += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.z -= speed * time.delta_seconds();
    }
}

fn follow_game_state(
    overlay_state: Res<MyOverlayState>,
    mut query_camera: Query<
        (&mut Projection, &mut Transform),
        (With<MyCameraMarker>, Without<AimTarget>),
    >,
    mut aim_targets: Query<
        (&mut Transform, &mut AimTarget),
        (With<AimTarget>, Without<MyCameraMarker>),
    >,
) {
    let (cam_proj, cam_trans) = query_camera.single_mut();
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = cam_proj.into_inner() else {
        return;
    };
    persp.fov = 90.0f32.to_radians();

    let cam_transform = cam_trans.into_inner();
    let state = overlay_state.shared_state.blocking_lock();
    let Some(local_player) = state.local_player.as_ref() else {
        return;
    };
    let cam_pos = local_player.get_entity().camera_origin;
    let cam_angles = local_player.get_entity().camera_angles;

    cam_transform.translation.x = -cam_pos[1];
    cam_transform.translation.y = cam_pos[2];
    cam_transform.translation.z = -cam_pos[0];

    let pitch_quat = Quat::from_rotation_x(
        cam_angles[0].to_radians()
            * if cam_angles[1].abs() < 90.0 {
                -1.0
            } else {
                1.0
            },
    );
    let yaw_quat = Quat::from_rotation_y(cam_angles[1].to_radians());
    cam_transform.rotation = pitch_quat * yaw_quat;

    let aim_pos = state.aim_target;
    for (mut target_transform, mut _aim_target) in aim_targets.iter_mut() {
        target_transform.translation.x = -aim_pos[1];
        target_transform.translation.y = aim_pos[2];
        target_transform.translation.z = -aim_pos[0];
    }
}
