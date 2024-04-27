use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use ambisonic::rodio::Source;
use ambisonic::{AmbisonicBuilder, SoundController};
use apexsky::aimbot::AimEntity;
use bevy::asset::embedded_asset;
//use bevy::audio::{SpatialScale, Volume};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, WindowLevel, WindowMode};
use bevy_egui::EguiPlugin;
use bevy_health_bar3d::prelude as hpbar;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, RwLock};
use tokio::sync::watch;

use crate::overlay::asset::{Blob, BlobAssetLoader};
use crate::workers::aim::PreSelectedTarget;
use crate::{SharedState, TaskChannels};

mod asset;
mod ui;

pub(crate) fn main(shared_state: Arc<RwLock<SharedState>>, task_channels: Option<TaskChannels>) {
    let (sound_ent_tx, mut sound_ent_rx) = watch::channel(Vec::<SoundEntity>::new());
    let (sound_src_tx, mut sound_src_rx) = watch::channel::<Option<AudioSource>>(None);
    // let (sonic_active, sonic_t) = {
    //     let active = Arc::new(Mutex::new(true));
    //     (
    //         active.clone(),
    //         std::thread::spawn(move || {
    //             let ambisonic_scene = AmbisonicBuilder::default().build();
    //             let mut sound_entities = HashMap::<u64, SoundController>::new();
    //             tracing::debug!("{}", s!("sonic task start"));
    //             while *active.lock() {
    //                 std::thread::sleep(Duration::from_millis(15));
    //                 //let audio_src = ambisonic::rodio::source::SineWave::new(440);

    //                 if !sound_ent_rx.has_changed().unwrap_or_else(|e| {
    //                     // If it is not because of a normal exit, an error is displayed
    //                     if *active.lock() {
    //                         tracing::error!(%e, "{}", s!("sound_ent_rx if changed"));
    //                     }
    //                     false
    //                 }) {
    //                     continue;
    //                 }

    //                 let ents = sound_ent_rx.borrow_and_update();
    //                 let mut data: HashMap<u64, _> = ents
    //                     .iter()
    //                     .map(|obj| (obj.target.entity_ptr, obj))
    //                     .collect();
    //                 sound_entities = sound_entities
    //                     .into_iter()
    //                     .filter_map(|(ptr, mut sound)| {
    //                         if let Some(ent) = data.remove(&ptr) {
    //                             sound.adjust_position(ent.relative);
    //                             Some((ptr, sound))
    //                         } else {
    //                             sound.stop();
    //                             None
    //                         }
    //                     })
    //                     .collect();
    //                 data.into_iter().for_each(|(ptr, ent)| {
    //                     if let Some(src) = sound_src_rx.borrow_and_update().as_ref().cloned() {
    //                         let src =
    //                             ambisonic::rodio::Decoder::new(std::io::Cursor::new(src.clone()))
    //                                 .unwrap();
    //                         let sound = ambisonic_scene
    //                             .play_at(src.convert_samples().repeat_infinite(), ent.relative);
    //                         sound_entities.insert(ptr, sound);
    //                     }
    //                 });
    //             }
    //             //tracing::debug!("{}", s!("sonic task end"));
    //         }),
    //     )
    // };

    static S_TITLE: Lazy<String> =
        Lazy::new(|| s!("Absolutely Not Cheating.exe - Totally Legit Gameplay 😇").to_string());
    App::new()
        .register_type::<Health>()
        .register_type::<Mana>()
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
            EguiPlugin,
            hpbar::HealthBarPlugin::<Health>::default(),
            hpbar::HealthBarPlugin::<Mana>::default(),
        ))
        .init_asset::<Blob>()
        .init_asset_loader::<BlobAssetLoader>()
        .insert_resource(
            hpbar::ColorScheme::<Health>::new()
                .foreground_color(hpbar::ForegroundColor::Static(Color::GREEN))
                .background_color(Color::RED),
        )
        .insert_resource(
            hpbar::ColorScheme::<Mana>::new()
                .foreground_color(hpbar::ForegroundColor::Static(Color::BISQUE)),
        )
        .insert_resource(MyOverlayState {
            shared_state,
            task_channels,
            sound_handle: Default::default(),
            font_blob: Default::default(),
            font_loaded: false,
            sound_loaded: false,
            data_latency: 0.0,
            update_latency: 0.0,
            target_count: 0,
            sound_ent_tx,
            sound_src_tx,
        })
        // ClearColor must have 0 alpha, otherwise some color will bleed through
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(Update, ui::toggle_mouse_passthrough)
        .add_systems(Update, ui::ui_system)
        // .add_systems(Update, load_sound)
        // .add_systems(Update, update_positions)
        // .add_systems(Update, update_listener)
        .add_systems(Update, follow_game_state)
        .add_systems(Update, despawn_dead_targets)
        .run();

    // *sonic_active.lock() = false;
    // sonic_t.join().unwrap();
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

pub struct SoundEntity {
    target: PreSelectedTarget,
    relative: [f32; 3],
}

#[derive(Resource)]
pub(crate) struct MyOverlayState {
    shared_state: Arc<RwLock<SharedState>>,
    task_channels: Option<TaskChannels>,
    sound_handle: Handle<AudioSource>,
    font_blob: Handle<Blob>,
    font_loaded: bool,
    sound_loaded: bool,
    data_latency: f64,
    update_latency: f64,
    target_count: usize,
    sound_ent_tx: watch::Sender<Vec<SoundEntity>>,
    sound_src_tx: watch::Sender<Option<AudioSource>>,
}

#[derive(Component, Default)]
struct AimTarget {
    ptr: u64,
    data: Option<Arc<dyn AimEntity>>,
}

#[derive(Component)]
struct MyCameraMarker;

#[derive(Component, Reflect)]
struct Health {
    max: f32,
    current: f32,
}

impl hpbar::Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component, Reflect)]
struct Mana {
    max: f32,
    current: f32,
}

impl hpbar::Percentage for Mana {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

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

#[tracing::instrument(skip_all)]
pub fn load_sound(mut overlay_state: ResMut<MyOverlayState>, sounds: Res<Assets<AudioSource>>) {
    if !overlay_state.sound_loaded {
        if let Some(audio_src) = sounds.get(&overlay_state.sound_handle) {
            overlay_state
                .sound_src_tx
                .send(Some(audio_src.clone()))
                .unwrap_or_else(|e| {
                    tracing::error!(%e, ?e);
                });
            overlay_state.sound_loaded = true;
        }
    }
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

fn despawn_dead_targets(
    mut commands: Commands,
    mut aim_targets: Query<Entity, (With<AimTarget>, Without<Health>)>,
) {
    for entity in aim_targets.iter_mut() {
        commands.entity(entity).despawn();
    }
}

#[tracing::instrument(skip_all)]
fn follow_game_state(
    time: Res<Time>,
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
        (
            Entity,
            &mut Transform,
            &mut AimTarget,
            &mut Health,
            &mut Mana,
        ),
        (
            With<AimTarget>,
            Without<MyCameraMarker>,
            Without<SpatialListener>,
        ),
    >,
) {
    overlay_state.update_latency = time.delta_seconds_f64() * 1000.0;

    let (cam_proj, cam_trans) = query_camera.single_mut();
    let listener_trans = listeners.single_mut();
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = cam_proj.into_inner() else {
        return;
    };
    persp.fov = 90.0f32.to_radians();

    let cam_matrix = overlay_state
        .shared_state
        .read()
        .view_player
        .as_ref()
        .map(|view_player| {
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

            cam_transform.compute_matrix()
        });

    #[derive(Debug)]
    struct UpdateTarget {
        info: PreSelectedTarget,
        data: Option<Arc<dyn AimEntity>>,
        point_pos: Vec3,
        health: f32,
        max_health: f32,
        shield: f32,
        max_shield: f32,
    }

    // Get target entities
    let mut targets: HashMap<u64, UpdateTarget> =
        if let Some(channels) = &mut overlay_state.task_channels {
            let targets_rx = &mut channels.aim_select_rx;
            targets_rx
                .borrow_and_update()
                .iter()
                .map(|target| {
                    (
                        target.entity_ptr,
                        UpdateTarget {
                            info: target.clone(),
                            data: None,
                            point_pos: Vec3::default(),
                            health: 0.0,
                            max_health: 0.0,
                            shield: 0.0,
                            max_shield: 0.0,
                        },
                    )
                })
                .collect()
        } else {
            // test data
            let health = 100. - time.elapsed_seconds() * 5.0;
            if health < 1.0 {
                [].into()
            } else {
                [(
                    0,
                    UpdateTarget {
                        info: PreSelectedTarget {
                            fov: 1.0,
                            distance: 40.0,
                            is_visible: true,
                            is_knocked: false,
                            health_points: 150,
                            love_status: apexsky::love_players::LoveStatus::Normal,
                            is_kill_leader: false,
                            entity_ptr: 0,
                        },
                        data: None,
                        point_pos: Vec3 {
                            x: 0.,
                            y: -40.,
                            z: -40.,
                        },
                        health,
                        max_health: 100.,
                        shield: 50.,
                        max_shield: 150.,
                    },
                )]
                .into()
            }
        };

    // Get updated target data
    if overlay_state.task_channels.is_some() {
        let state = overlay_state.shared_state.read();
        targets.iter_mut().for_each(|(ptr, target)| {
            target.data = state.aim_entities.get(ptr).cloned();
            if let Some(target_data) = &target.data {
                let target_pos = target_data.get_bone_position_by_hitbox(0);
                target.point_pos = Vec3 {
                    x: -target_pos[1],
                    y: target_pos[2],
                    z: -target_pos[0],
                };
                target.health = target_data.get_health() as f32;
                target.max_health = target_data.get_max_health() as f32;
                target.shield = target_data.get_shield_health() as f32;
                target.max_shield = target_data.get_max_shield_health() as f32;
            } else {
                tracing::warn!(?target, "{}", s!("AimEntities[ptr]=None"));
            }
        });
    }
    overlay_state.target_count = targets.len();

    // // Update ambisonic
    // if let Some(cam_matrix) = cam_matrix {
    //     let sound_entities: Vec<_> = targets
    //         .values()
    //         .map(|target| {
    //             let relative = cam_matrix.transform_point(target.point_pos);
    //             SoundEntity {
    //                 target: target.info.clone(),
    //                 relative: [relative.x / 39.62, relative.y / 39.62, relative.z / 39.62],
    //             }
    //         })
    //         .collect();
    //     overlay_state
    //         .sound_ent_tx
    //         .send(sound_entities)
    //         .unwrap_or_else(|e| {
    //             tracing::error!(%e, "{}", s!("send sound_entity"));
    //         });
    // }

    // Update or despawn existing entities
    for (entity, mut target_transform, mut aim_target, mut health, mut mana) in
        aim_targets.iter_mut()
    {
        if let Some(target) = targets.remove(&aim_target.ptr) {
            target_transform.translation = target.point_pos;
            aim_target.data = target.data;
            health.max = target.max_health;
            health.current = target.health;
            mana.max = target.max_shield;
            mana.current = target.shield;
        } else {
            commands.entity(entity).remove::<(Health, Mana)>();
            //commands.entity(entity).despawn();
        }
    }

    // Create entities that do not yet exist
    targets.into_iter().for_each(|(ptr, target)| {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(6.0).mesh().uv(32, 18)),
                material: materials.add(Color::ORANGE_RED),
                transform: Transform::from_translation(target.point_pos),
                ..default()
            },
            AimTarget {
                ptr,
                data: target.data,
            },
            Health {
                max: target.max_health,
                current: target.health,
            },
            hpbar::BarSettings::<Health> {
                width: 12.,
                offset: 9.,
                orientation: hpbar::BarOrientation::Vertical,
                ..default()
            },
            Mana {
                max: target.max_shield,
                current: target.shield,
            },
            hpbar::BarSettings::<Mana> {
                width: 12.,
                offset: 12.,
                orientation: hpbar::BarOrientation::Vertical,
                ..default()
            },
            // AudioBundle {
            //     source: overlay_state.sound_handle.to_owned(),
            //     settings: PlaybackSettings::LOOP
            //         .with_spatial(true)
            //         .with_spatial_scale(SpatialScale::new(1.0 / 40.0))
            //         .with_volume(Volume::new(0.6)),
            // },
        ));
    });
}
