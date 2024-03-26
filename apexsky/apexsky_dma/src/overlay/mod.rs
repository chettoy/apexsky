use std::collections::HashMap;
use std::sync::Arc;

use apexsky::aimbot::AimEntity;
use bevy::asset::embedded_asset;
use bevy::audio::{SpatialScale, Volume};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, WindowLevel, WindowMode};
use bevy_egui::EguiPlugin;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::overlay::asset::{Blob, BlobAssetLoader};
use crate::{SharedState, TaskChannels};

mod asset;
mod ui;

pub(crate) fn main(shared_state: Arc<RwLock<SharedState>>, task_channels: Option<TaskChannels>) {
    static S_TITLE: Lazy<String> =
        Lazy::new(|| s!("Absolutely Not Cheating.exe - Totally Legit Gameplay 😇").to_string());
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
                    title: S_TITLE.to_owned(),
                    ..default()
                }),
                ..default()
            }),
            EmbeddedAssetPlugin,
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(EguiPlugin)
        .init_asset::<Blob>()
        .init_asset_loader::<BlobAssetLoader>()
        .insert_resource(MyOverlayState {
            shared_state,
            task_channels,
            sound_handle: Default::default(),
            font_blob: Default::default(),
            font_loaded: false,
            data_latency: 0.0,
        })
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, ui::toggle_mouse_passthrough)
        .add_systems(Update, ui::ui_system)
        // .add_systems(Update, update_positions)
        // .add_systems(Update, update_listener)
        .add_systems(Update, follow_game_state)
        .run();
}

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        // We get to choose some prefix relative to the workspace root which
        // will be ignored in "embedded://" asset paths.
        static S_OMIT_PREFIX: Lazy<String> = Lazy::new(|| s!("src/overlay/").to_string());
        // Path to asset must be relative to this file, because that's how
        // include_bytes! works.
        embedded_asset!(app, &*S_OMIT_PREFIX, "assets/fonts/LXGWNeoXiHei.ttf");
        embedded_asset!(app, &*S_OMIT_PREFIX, "assets/sounds/Windless Slopes.ogg");
    }
}

#[derive(Resource)]
pub(crate) struct MyOverlayState {
    shared_state: Arc<RwLock<SharedState>>,
    task_channels: Option<TaskChannels>,
    sound_handle: Handle<AudioSource>,
    font_blob: Handle<Blob>,
    font_loaded: bool,
    data_latency: f64,
}

#[derive(Component, Default)]
struct AimTarget {
    ptr: u64,
    data: Option<Arc<dyn AimEntity>>,
}

#[derive(Component)]
struct MyCameraMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut overlay_state: ResMut<MyOverlayState>,
) {
    static S_FONT_PATH: Lazy<String> =
        Lazy::new(|| s!("embedded://apexsky_dma/assets/fonts/LXGWNeoXiHei.ttf").to_string());
    static S_SOUND_PATH: Lazy<String> =
        Lazy::new(|| s!("embedded://apexsky_dma/assets/sounds/Windless Slopes.ogg").to_string());

    overlay_state.font_blob = asset_server.load(&*S_FONT_PATH);
    overlay_state.sound_handle = asset_server.load(&*S_SOUND_PATH);

    // Space between the two ears
    let gap = 12.0;

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
        MyCameraMarker,
    ));
}

#[derive(Component, Default)]
struct Emitter {
    stopped: bool,
}

// fn update_positions(
//     time: Res<Time>,
//     mut emitters: Query<(&mut Transform, &mut Emitter), With<Emitter>>,
//     keyboard: Res<ButtonInput<KeyCode>>,
// ) {
//     for (mut emitter_transform, mut emitter) in emitters.iter_mut() {
//         if keyboard.just_pressed(KeyCode::Space) {
//             emitter.stopped = !emitter.stopped;
//         }

//         if !emitter.stopped {
//             emitter_transform.translation.x = time.elapsed_seconds().sin() * 3.0;
//             emitter_transform.translation.z = time.elapsed_seconds().cos() * 3.0;
//         }
//     }
// }

// fn update_listener(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut listeners: Query<&mut Transform, With<SpatialListener>>,
// ) {
//     let mut transform = listeners.single_mut();

//     let speed = 2.;

//     if keyboard.pressed(KeyCode::ArrowRight) {
//         transform.translation.x += speed * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::ArrowLeft) {
//         transform.translation.x -= speed * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::ArrowDown) {
//         transform.translation.z += speed * time.delta_seconds();
//     }
//     if keyboard.pressed(KeyCode::ArrowUp) {
//         transform.translation.z -= speed * time.delta_seconds();
//     }
// }

#[tracing::instrument(skip_all)]
fn follow_game_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut overlay_state: ResMut<MyOverlayState>,
    mut query_camera: Query<
        (&mut Projection, &mut Transform),
        (
            With<MyCameraMarker>,
            Without<SpatialListener>,
            Without<AimTarget>,
        ),
    >,
    mut listeners: Query<
        &mut Transform,
        (
            With<SpatialListener>,
            Without<MyCameraMarker>,
            Without<AimTarget>,
        ),
    >,
    mut aim_targets: Query<
        (Entity, &mut Transform, &mut AimTarget),
        (
            With<AimTarget>,
            Without<MyCameraMarker>,
            Without<SpatialListener>,
        ),
    >,
) {
    let (cam_proj, cam_trans) = query_camera.single_mut();
    let listener_trans = listeners.single_mut();
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = cam_proj.into_inner() else {
        return;
    };
    persp.fov = 90.0f32.to_radians();

    if let Some(view_player) = overlay_state.shared_state.read().view_player.as_ref() {
        let cam_origin = view_player.get_entity().camera_origin;
        let cam_angles = view_player.get_entity().camera_angles;

        let (cam_pitch, cam_yew) = (cam_angles[0].to_radians(), cam_angles[1].to_radians());
        // pitch: top- bottom+, yew: left+ right-

        // game: x: forward, y: left, z: top
        // bevy: x: right, y: top, z: back
        let cam_position = Vec3 {
            x: -cam_origin[1],
            y: cam_origin[2],
            z: -cam_origin[0],
        };
        let cam_direction = Vec3 {
            x: -cam_pitch.cos() * cam_yew.sin(),
            y: -cam_pitch.sin(),
            z: -cam_pitch.cos() * cam_yew.cos(),
        };
        let cam_transform =
            Transform::from_translation(cam_position).looking_to(cam_direction, Vec3::Y);
        *cam_trans.into_inner() = cam_transform.clone();
        *listener_trans.into_inner() = cam_transform;
    }

    // Get updated target entities or return
    let mut targets: HashMap<u64, Option<Arc<dyn AimEntity>>> = {
        let Some(channels) = &mut overlay_state.task_channels else {
            return;
        };
        let targets_rx = &mut channels.aim_select_rx;
        // if !targets_rx.has_changed().unwrap_or_else(|e| {
        //     tracing::error!(%e, ?targets_rx, "{}", s!("overlay"));
        //     false
        // }) {
        //     return;
        // }
        targets_rx
            .borrow_and_update()
            .iter()
            .map(|target| (target.entity_ptr, None))
            .collect()
    };
    {
        let state = overlay_state.shared_state.read();
        targets.iter_mut().for_each(|(ptr, value)| {
            *value = state.aim_entities.get(ptr).cloned();
        });
    }

    // Update or despawn existing entities
    for (entity, mut target_transform, mut aim_target) in aim_targets.iter_mut() {
        if let Some(target) = targets.remove(&aim_target.ptr) {
            if let Some(target_data) = &target {
                let target_pos = target_data.get_bone_position_by_hitbox(0);
                target_transform.translation.x = -target_pos[1];
                target_transform.translation.y = target_pos[2];
                target_transform.translation.z = -target_pos[0];
            } else {
                tracing::debug!(?aim_target.ptr, ?target, "{}", s!("AimEntities[ptr]=None"));
            }
            aim_target.data = target;
        } else {
            commands.entity(entity).despawn();
        }
    }

    // Create entities that do not yet exist
    targets.into_iter().for_each(|(ptr, target)| {
        let Some(target_data) = &target else {
            tracing::warn!(?ptr, ?target, "{}", s!("AimEntities[ptr]=None"));
            return;
        };
        let target_pos = target_data.get_bone_position_by_hitbox(0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(6.0).mesh().uv(32, 18)),
                material: materials.add(Color::ORANGE_RED),
                transform: Transform::from_xyz(-target_pos[1], target_pos[2], -target_pos[0]),
                ..default()
            },
            AimTarget { ptr, data: target },
            AudioBundle {
                source: overlay_state.sound_handle.to_owned(),
                settings: PlaybackSettings::LOOP
                    .with_spatial(true)
                    .with_spatial_scale(SpatialScale::new(1.0 / 40.0))
                    .with_volume(Volume::new(1.0)),
            },
        ));
    });
}
