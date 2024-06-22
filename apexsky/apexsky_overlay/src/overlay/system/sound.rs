use bevy::prelude::*;
use fyrox_sound::buffer::SoundBufferResource;
use fyrox_sound::context::SoundContext;
use fyrox_sound::engine::SoundEngine;
use fyrox_sound::hrtf::HrirSphere;
use fyrox_sound::renderer::hrtf::{HrirSphereResource, HrirSphereResourceExt};
use instant::Instant;
use obfstr::obfstr as s;

use crate::overlay::model::MyOverlayState;

pub type SoundSrcHandle = fyrox_sound::pool::Handle<fyrox_sound::source::SoundSource>;

#[allow(dead_code)]
#[derive(Resource)]
pub(crate) struct SoundSystem {
    pub(crate) engine: SoundEngine,
    pub(crate) context: SoundContext,
}

impl SoundSystem {
    pub fn init() -> Option<Self> {
        let engine = match SoundEngine::new() {
            Ok(instance) => instance,
            Err(e) => {
                tracing::error!(%e, ?e, "{}", s!("Failed to create instance of the sound engine."));
                return None;
            }
        };

        let hrir = include_bytes!("../assets/hrir/IRC_1002_C.bin");
        let hrir_sphere = HrirSphere::new(&hrir[..], fyrox_sound::context::SAMPLE_RATE).unwrap();

        // Initialize new sound context with default output device.
        let context = SoundContext::new();

        engine.state().add_context(context.clone());

        // Set HRTF renderer instead of default.
        context
            .state()
            .set_renderer(fyrox_sound::renderer::Renderer::HrtfRenderer(
                fyrox_sound::renderer::hrtf::HrtfRenderer::new(
                    HrirSphereResource::from_hrir_sphere(hrir_sphere, Default::default()),
                ),
            ));

        Some(Self { engine, context })
    }
}

impl Default for SoundSystem {
    fn default() -> Self {
        Self::init().unwrap()
    }
}

pub struct SoundBufRes {
    pub door_open: SoundBufferResource,
    pub helicopter: SoundBufferResource,
}

impl Default for SoundBufRes {
    fn default() -> Self {
        use fyrox_sound::buffer::{DataSource, SoundBufferResourceExtension};
        Self {
            door_open: SoundBufferResource::new_generic(DataSource::from_memory(
                include_bytes!("../assets/sounds/door_open.wav").to_vec(),
            ))
            .unwrap(),
            helicopter: SoundBufferResource::new_generic(DataSource::from_memory(
                include_bytes!("../assets/sounds/helicopter.wav").to_vec(),
            ))
            .unwrap(),
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn load_test_sound(
    mut commands: Commands,
    mut overlay_state: ResMut<MyOverlayState>,
    sound_buffer: NonSend<SoundBufRes>,
    sound_system: Res<SoundSystem>,
) {
    use fyrox_sound::source::SoundSourceBuilder;

    if !overlay_state.test_sound {
        return;
    }

    let context = &sound_system.context;

    // Create some sounds.
    let source = SoundSourceBuilder::new()
        .with_buffer(sound_buffer.door_open.clone())
        .with_status(fyrox_sound::source::Status::Playing)
        .with_play_once(true)
        .build()
        .unwrap();
    context.state().add_source(source);

    let source = SoundSourceBuilder::new()
        .with_buffer(sound_buffer.helicopter.clone())
        .with_status(fyrox_sound::source::Status::Playing)
        .with_looping(true)
        .with_play_once(true)
        .build()
        .unwrap();
    let source_handle = context.state().add_source(source);

    // Move source sound around listener for some time.
    let start_time = Instant::now();
    let angle = 0.0f32;

    commands.spawn(Emitter {
        source_handle,
        start_time,
        angle,
        stopped: false,
    });

    overlay_state.test_sound = false;
}

#[derive(Component)]
pub(crate) struct Emitter {
    source_handle: SoundSrcHandle,
    start_time: Instant,
    angle: f32,
    stopped: bool,
}

pub fn update_sound_objects(
    mut commands: Commands,
    mut emitters: Query<(Entity, &mut Emitter)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    sound_system: Res<SoundSystem>,
    time: Res<Time>,
) {
    use fyrox_sound::algebra::UnitQuaternion;

    for (entity, mut emitter) in emitters.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space) {
            emitter.stopped = !emitter.stopped;
        }

        if !emitter.stopped {
            let axis = fyrox_sound::algebra::Vector3::y_axis();
            let rotation_matrix =
                UnitQuaternion::from_axis_angle(&axis, emitter.angle.to_radians()).to_homogeneous();
            sound_system
                .context
                .state()
                .source_mut(emitter.source_handle)
                .set_position(
                    rotation_matrix
                        .transform_point(&fyrox_sound::algebra::Point3::new(0.0, 0.0, 3.0))
                        .coords,
                );

            if emitter.angle > 360.0 {
                emitter.angle = 0.0;
            }
            emitter.angle += 16.0 / time.elapsed_seconds();

            tracing::debug!(
                "Sound render time {:?}",
                sound_system.context.state().full_render_duration()
            );
        }

        if emitter.start_time.elapsed().as_secs() > 30 {
            sound_system
                .context
                .state()
                .source_mut(emitter.source_handle)
                .stop()
                .map_err(|e| {
                    tracing::error!(%e, ?e);
                })
                .unwrap();
            sound_system
                .context
                .state()
                .remove_source(emitter.source_handle);
            commands.entity(entity).despawn();
        }
    }
}

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
