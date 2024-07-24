use std::collections::HashMap;
use std::sync::Arc;

use bevy::color::palettes;
use bevy::prelude::*;
use bevy_health_bar3d::prelude as hpbar;
use instant::{Duration, Instant};
use obfstr::obfstr as s;
use parking_lot::Mutex;
use url::Url;

use crate::overlay::model::{Health, Mana, MyCameraMarker, MyOverlayState, TokioRuntime};
use crate::overlay::{DRY_RUN, PRINT_LATENCY};
use crate::pb::apexlegends::{
    AimEntityData, AimTargetInfo, AimTargetItem, EspData, EspDataOption, EspSettings, Loots,
    LoveStatusCode, PlayerState,
};
use crate::pb::esp_service::esp_service_client::EspServiceClient;
use crate::pb::esp_service::GetLootsRequest;

#[cfg(feature = "web-wasm")]
pub type EspClient = EspServiceClient<tonic_web_wasm_client::Client>;
#[cfg(not(feature = "web-wasm"))]
pub type EspClient = EspServiceClient<tonic::transport::Channel>;

#[derive(Debug, Clone)]
pub(crate) struct EspServiceAddr {
    endpoint: String,
    last_retry: Option<Instant>,
}

impl EspServiceAddr {
    pub(crate) fn from_str(addr: &str) -> Option<Self> {
        Url::parse(addr).ok()?;
        if addr.ends_with("/") {
            return None;
        }
        Some(Self {
            endpoint: addr.to_owned(),
            last_retry: None,
        })
    }

    pub(crate) fn get_addr(&self) -> &str {
        &self.endpoint
    }

    pub(crate) fn record_retry(&mut self) -> bool {
        if self
            .last_retry
            .is_some_and(|last_retry| last_retry.elapsed().as_millis() < 500)
        {
            false
        } else {
            self.last_retry = Some(Instant::now());
            true
        }
    }
}

pub fn init_grpc_client(server_url: String) -> Option<EspClient> {
    #[cfg(feature = "web-wasm")]
    {
        Some(EspServiceClient::new(tonic_web_wasm_client::Client::new(
            server_url,
        )))
    }
    #[cfg(not(feature = "web-wasm"))]
    {
        match bevy::tasks::block_on(EspServiceClient::connect(server_url)) {
            Ok(client) => Some(
                client
                    .accept_compressed(tonic::codec::CompressionEncoding::Zstd)
                    .send_compressed(tonic::codec::CompressionEncoding::Zstd),
            ),
            Err(e) => {
                tracing::error!(%e, ?e);
                None
            }
        }
    }
}

#[derive(Debug)]
struct EspFreshData {
    request_time: Instant,
    response_time: Instant,
    new_esp_data: Option<EspData>,
    new_esp_settings: Option<EspSettings>,
    new_esp_loots: Option<Loots>,
}

#[derive(Resource)]
pub(crate) struct EspSystem {
    server_endpoint: String,
    rpc_client: EspClient,
    connect_time: Instant,
    esp_data: EspData,
    esp_settings: EspSettings,
    esp_loots: Loots,
    fresh_data: Arc<Mutex<Option<EspFreshData>>>,
    update_latency: f64,
    target_count: usize,
    last_settings_fetch_time: Option<Instant>,
    pub(crate) last_data_response_time: Option<Instant>,
    pub(crate) last_data_traffic_time: Option<Duration>,
    view_teammate_index: Option<usize>,
}

impl EspSystem {
    pub(crate) fn get_endpoint(&self) -> &str {
        &self.server_endpoint
    }
    pub(crate) fn get_connect_time(&self) -> Instant {
        self.connect_time
    }
    pub(crate) fn get_esp_data(&self) -> &EspData {
        &self.esp_data
    }
    pub(crate) fn get_esp_settings(&self) -> &EspSettings {
        &self.esp_settings
    }
    pub(crate) fn get_esp_loots(&self) -> &Loots {
        &self.esp_loots
    }
    pub(crate) fn get_update_latency(&self) -> f64 {
        self.update_latency
    }
    pub(crate) fn get_target_count(&self) -> usize {
        self.target_count
    }
    pub(crate) fn get_view_player(&self) -> Option<&PlayerState> {
        if let Some(teammate_index) = self.view_teammate_index {
            self.esp_data
                .teammates
                .as_ref()?
                .players
                .get(teammate_index)
        } else {
            self.esp_data.view_player.as_ref()
        }
    }
    pub(crate) fn set_view_teammate(&mut self, teammate_index: Option<usize>) {
        self.view_teammate_index = teammate_index;
    }
    pub(crate) fn get_view_teammate(&self) -> Option<usize> {
        self.view_teammate_index
    }

    fn connect(server_url: String) -> Option<Self> {
        let now = Instant::now();
        Some(Self {
            server_endpoint: server_url.clone(),
            rpc_client: init_grpc_client(server_url)?,
            connect_time: now,
            esp_data: Default::default(),
            esp_settings: Default::default(),
            esp_loots: Default::default(),
            fresh_data: Arc::new(Mutex::new(Some(EspFreshData {
                request_time: now,
                response_time: now,
                new_esp_data: None,
                new_esp_settings: None,
                new_esp_loots: None,
            }))),
            update_latency: 0.0,
            target_count: 0,
            last_settings_fetch_time: None,
            last_data_response_time: None,
            last_data_traffic_time: None,
            view_teammate_index: None,
        })
    }
}

#[derive(Component, Default)]
pub(crate) struct AimTargetEntity {
    pub(crate) ptr: u64,
    pub(crate) data: Option<AimEntityData>,
}

pub(crate) fn despawn_dead_targets(
    mut commands: Commands,
    mut aim_targets: Query<Entity, (With<AimTargetEntity>, Without<Health>)>,
) {
    for entity in aim_targets.iter_mut() {
        commands.entity(entity).despawn();
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn follow_game_state(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    esp_system: Option<ResMut<EspSystem>>,
    mut windows: Query<&mut Window>,
    mut query_camera: Query<
        (&mut Projection, &mut Transform),
        (
            With<MyCameraMarker>,
            Without<SpatialListener>,
            Without<AimTargetEntity>,
        ),
    >,
    mut listeners: Query<
        &mut Transform,
        (
            With<SpatialListener>,
            Without<MyCameraMarker>,
            Without<AimTargetEntity>,
        ),
    >,
    mut aim_targets: Query<
        (
            Entity,
            &mut Transform,
            &mut AimTargetEntity,
            &mut Health,
            &mut Mana,
        ),
        (
            With<AimTargetEntity>,
            Without<MyCameraMarker>,
            Without<SpatialListener>,
        ),
    >,
) {
    let Some(mut esp_system) = esp_system else {
        return;
    };

    let Some(mut fresh_data) = esp_system.fresh_data.lock().take() else {
        return;
    };

    if let Some(esp_settings) = fresh_data.new_esp_settings.take() {
        let screen_wh = (
            esp_settings.screen_width as f32,
            esp_settings.screen_height as f32,
        );
        esp_system.esp_settings = esp_settings;
        esp_system.last_settings_fetch_time = Some(Instant::now());

        let mut window = windows.single_mut();
        if (window.resolution.width() - screen_wh.0).abs() > f32::EPSILON
            || (window.resolution.height() - screen_wh.1).abs() > f32::EPSILON
        {
            window.resolution = screen_wh.into();
        }
    }
    if let Some(loots_data) = fresh_data.new_esp_loots.take() {
        esp_system.esp_loots = loots_data;
    }
    if let Some(esp_data) = fresh_data.new_esp_data.take() {
        esp_system.esp_data = esp_data;
        esp_system.last_data_response_time = (fresh_data.response_time > esp_system.connect_time)
            .then_some(fresh_data.response_time);
        esp_system.last_data_traffic_time = esp_system
            .last_data_response_time
            .and_then(|resp_time| Some(resp_time - fresh_data.request_time));

        if esp_system.get_view_player().is_none() && esp_system.esp_data.view_player.is_some() {
            esp_system.set_view_teammate(None);
        }
    }

    esp_system.update_latency = time.delta_seconds_f64() * 1000.0;

    let esp_data = &esp_system.esp_data;

    if PRINT_LATENCY {
        println!(
            "{}{:.1}",
            s!("esp task data latency "),
            crate::overlay::utils::get_unix_timestamp_in_millis() as f64
                - esp_data.data_timestamp * 1000.0
        );
    }

    let (cam_proj, cam_trans) = query_camera.single_mut();
    let listener_trans = listeners.single_mut();
    // assume perspective. do nothing if orthographic.
    let Projection::Perspective(persp) = cam_proj.into_inner() else {
        unreachable!()
    };
    persp.fov = if esp_data.current_zoom_fov.is_normal() {
        esp_data.current_zoom_fov
    } else {
        90.0f32
    }
    .to_radians();

    let _cam_matrix = esp_system.get_view_player().map(|view_player| {
        let cam_origin: [f32; 3] = view_player.camera_origin.clone().unwrap().into();
        let cam_angles: [f32; 3] = view_player.camera_angles.clone().unwrap().into();

        let (cam_pitch, cam_yaw) = (cam_angles[0].to_radians(), cam_angles[1].to_radians());
        // pitch: top- bottom+, yaw: left+ right-

        // game: x: forward, y: left, z: top
        // bevy: x: right, y: top, z: back
        let cam_position = Vec3 {
            x: -cam_origin[1],
            y: cam_origin[2],
            z: -cam_origin[0],
        };
        let cam_direction = Vec3 {
            x: -cam_pitch.cos() * cam_yaw.sin(),
            y: -cam_pitch.sin(),
            z: -cam_pitch.cos() * cam_yaw.cos(),
        };
        let cam_transform =
            Transform::from_translation(cam_position).looking_to(cam_direction, Vec3::Y);
        *cam_trans.into_inner() = cam_transform.clone();
        *listener_trans.into_inner() = cam_transform;

        cam_transform.compute_matrix()
    });

    #[derive(Debug)]
    struct UpdateTarget {
        info: AimTargetInfo,
        data: Option<AimEntityData>,
        point_pos: Vec3,
        health: f32,
        max_health: f32,
        shield: f32,
        max_shield: f32,
    }

    impl TryFrom<&AimTargetItem> for UpdateTarget {
        type Error = ();
        fn try_from(value: &AimTargetItem) -> Result<Self, Self::Error> {
            let convert = || {
                let info = value.info.clone()?;
                let data = value.data.clone()?;
                let target_pos: [f32; 3] = data.head_position.clone()?.into();
                Some(Self {
                    info,
                    data: Some(data.clone()),
                    point_pos: Vec3 {
                        x: -target_pos[1],
                        y: target_pos[2],
                        z: -target_pos[0],
                    },
                    health: data.health as f32,
                    max_health: data.max_health as f32,
                    shield: data.shield_health as f32,
                    max_shield: data.max_shield_health as f32,
                })
            };
            convert().ok_or(())
        }
    }

    // Get target entities
    let mut targets: HashMap<u64, UpdateTarget> = if esp_data.ready {
        esp_data
            .targets
            .as_ref()
            .map(|targets| &targets.elements)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|target| target.try_into().ok().map(|valid| (target.id, valid)))
            .collect()
    } else {
        // test data
        let health = 100. - esp_system.connect_time.elapsed().as_secs_f32() * 5.0;
        if health < 1.0 {
            [].into()
        } else {
            [(
                0,
                UpdateTarget {
                    info: AimTargetInfo {
                        fov: 1.0,
                        distance: 40.0,
                        is_visible: true,
                        is_knocked: false,
                        health_points: 150,
                        love_status: LoveStatusCode::Normal.into(),
                        is_kill_leader: false,
                        entity_ptr: 0,
                        is_npc: true,
                        is_loot: false,
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

    esp_system.target_count = targets.len();

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
        if target.info.is_loot {
            return;
        }
        let base_color = if target.info.is_loot {
            palettes::css::GOLD
        } else if target.info.is_npc {
            palettes::css::ORANGE_RED
        } else {
            palettes::css::ORANGE_RED
        };
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(6.0).mesh().uv(32, 18)),
                material: materials.add(StandardMaterial {
                    base_color: Color::Srgba(base_color),
                    ..Default::default()
                }),
                transform: Transform::from_translation(target.point_pos),
                ..default()
            },
            AimTargetEntity {
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

#[tracing::instrument(skip_all)]
pub(crate) fn request_game_state(
    mut commands: Commands,
    rt: Res<TokioRuntime>,
    mut overlay_state: ResMut<MyOverlayState>,
    esp_system: Option<Res<EspSystem>>,
) {
    let _enter = rt.0.enter();

    // Reconnect EspSystem
    if let Some(addr) = &mut overlay_state.override_esp_addr {
        if !addr.record_retry() {
            return;
        }
        if let Some(esp_system) = EspSystem::connect(addr.endpoint.clone()) {
            commands.insert_resource(esp_system);
            overlay_state.override_esp_addr = None;
        } else {
            if esp_system.is_some() {
                commands.remove_resource::<EspSystem>();
            }
        }
    }

    let Some(esp_system) = esp_system else {
        return;
    };

    let fetch = {
        // Retrieve settings every 2 seconds
        let update_settings = esp_system
            .last_settings_fetch_time
            .is_none_or(|t| t.elapsed().as_secs() >= 2);
        // Retrieve loots data every 0.2 seconds
        let update_loots =
            esp_system.esp_data.data_timestamp > esp_system.esp_loots.data_timestamp + 0.2;

        let mut client = esp_system.rpc_client.clone();
        let store = esp_system.fresh_data.clone();

        let wish_list = esp_system.esp_settings.desired_loots.clone();

        async move {
            fn unwrap_resp<T>(
                resp: std::result::Result<tonic::Response<T>, tonic::Status>,
            ) -> Option<T> {
                match resp {
                    Ok(data) => Some(data.into_inner()),
                    Err(e) => {
                        tracing::error!(%e, ?e);
                        None
                    }
                }
            }

            if !DRY_RUN {
                let request_time = Instant::now();
                let new_esp_settings = if update_settings {
                    unwrap_resp(client.get_esp_settings(()).await)
                } else {
                    None
                };
                let new_esp_data = unwrap_resp(
                    client
                        .get_esp_data(EspDataOption {
                            version: 0,
                            full_aimbot_state: false,
                            full_targets_list: false,
                            sync: true,
                        })
                        .await,
                );
                if PRINT_LATENCY {
                    if let Some(ref data) = new_esp_data {
                        println!(
                            "esp_client data latency {:.1}",
                            crate::overlay::utils::get_unix_timestamp_in_millis() as f64
                                - data.data_timestamp * 1000.0
                        );
                    }
                }
                let new_esp_loots = if update_loots {
                    unwrap_resp(
                        client
                            .get_loots(GetLootsRequest {
                                version: 0,
                                max_distance: 40.0 * 200.0,
                                wish_list,
                            })
                            .await,
                    )
                } else {
                    None
                };
                let response_time = Instant::now();
                let fresh_data = EspFreshData {
                    request_time,
                    response_time,
                    new_esp_data,
                    new_esp_settings,
                    new_esp_loots,
                };
                *store.lock() = Some(fresh_data);
            }
        }
    };

    #[cfg(feature = "web-wasm")]
    wasm_bindgen_futures::spawn_local(fetch);
    #[cfg(not(feature = "web-wasm"))]
    {
        let task_pool = bevy::tasks::AsyncComputeTaskPool::get();
        let task = task_pool.spawn(fetch);
        task.detach();
    }
}
