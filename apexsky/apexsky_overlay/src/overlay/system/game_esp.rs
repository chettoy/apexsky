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
    AimEntityData, AimTargetInfo, AimTargetItem, EspData, EspDataOption, EspSettings,
    EspVisualsFlag, Loots, LoveStatusCode, PlayerState,
};
#[cfg(feature = "apex1-inspect")]
use crate::pb::app::ohosky::inspect::inspect_service_client::InspectServiceClient;
use crate::pb::esp_service::GetLootsRequest;
#[cfg(not(feature = "apex1-inspect"))]
use crate::pb::esp_service::esp_service_client::EspServiceClient;

#[cfg(feature = "web-wasm")]
pub type EspClientTransport = tonic_web_wasm_client::Client;
#[cfg(not(feature = "web-wasm"))]
pub type EspClientTransport = tonic::transport::Channel;

#[cfg(feature = "apex1-inspect")]
pub type EspClient = InspectServiceClient<EspClientTransport>;
#[cfg(not(feature = "apex1-inspect"))]
pub type EspClient = EspServiceClient<EspClientTransport>;

#[cfg(feature = "apex1-inspect")]
mod apex1_adapter {
    use super::*;
    use crate::pb::app::ohosky::inspect::{GlobalGetRequest, GlobalListRequest, SharedStorageGet};
    use apex1_common::{
        global::{
            ISharedStore, RecActionsDuration, RecAimTargetsMap, RecAimbotState, RecCurrentZoomFov,
            RecFrameCount, RecGameBaseAddr, RecGameFps, RecLocalPlayerBuf, RecSpectatorList,
            RecTeammates, RecTickDuration, RecTickNum, RecTickUpdateTimestamp, RecViewMatrix,
            RecViewPlayerBuf, RecWorldReady, Record, StoreBackend,
        },
        pb::apexlegends::{AimTargetList, EspVisualsFlag},
    };
    use once_cell::sync::Lazy;
    use tonic::{Response, Status};

    static CACHE: Lazy<Mutex<HashMap<u64, HashMap<u64, Vec<u8>>>>> =
        Lazy::new(|| Mutex::new(Default::default()));

    struct CacheStore;

    impl StoreBackend for CacheStore {}
    impl ISharedStore for CacheStore {
        #[inline]
        fn set(_id: u64, _data: Vec<u8>) {}
        #[inline]
        fn get(id: u64) -> Option<Vec<u8>> {
            CACHE.lock().get(&0).and_then(|m| m.get(&id).cloned())
        }
        #[inline]
        fn has(id: u64) -> bool {
            CACHE.lock().get(&0).is_some_and(|m| m.contains_key(&id))
        }
        #[inline]
        fn del(_id: u64) -> bool {
            false
        }
        #[inline]
        fn set_child(_id: u64, _child_id: u64, _data: Vec<u8>) {}
        #[inline]
        fn get_child(id: u64, child_id: u64) -> Option<Vec<u8>> {
            CACHE
                .lock()
                .get(&id)
                .and_then(|m| m.get(&child_id).cloned())
        }
        #[inline]
        fn has_child(id: u64, child_id: u64) -> bool {
            CACHE
                .lock()
                .get(&id)
                .is_some_and(|m| m.contains_key(&child_id))
        }
        #[inline]
        fn del_child(_id: u64, _child_id: u64) -> bool {
            false
        }
        #[inline]
        fn insert_children(_id: u64, _entries: Vec<(u64, Vec<u8>)>) {}
        #[inline]
        fn get_children(id: u64) -> Vec<(u64, Vec<u8>)> {
            CACHE
                .lock()
                .get(&id)
                .and_then(|m| Some(m.iter().map(|(k, v)| (*k, v.to_owned())).collect()))
                .unwrap_or_default()
        }
        #[inline]
        fn list_children(id: u64) -> Vec<u64> {
            CACHE
                .lock()
                .get(&id)
                .and_then(|m| Some(m.keys().cloned().collect()))
                .unwrap_or_default()
        }
        #[inline]
        fn swap_children(_id: u64, _entries: Vec<(u64, Vec<u8>)>) {}
        #[inline]
        fn clear_children(_id: u64) -> usize {
            0
        }
    }

    impl EspClient {
        pub(super) async fn fetch_values(&mut self) -> anyhow::Result<()> {
            let aim_targets_list = self
                .global_list(GlobalListRequest {
                    version: 0,
                    parent_id: vec![RecAimTargetsMap::name_id()],
                })
                .await?
                .into_inner()
                .result
                .into_iter()
                .next()
                .map(|ret| ret.child_id)
                .unwrap_or_default();

            let data = self
                .global_get(GlobalGetRequest {
                    version: 0,
                    list: vec![
                        SharedStorageGet {
                            parent_id: 0,
                            ids: vec![
                                RecActionsDuration::name_id(),
                                RecAimbotState::name_id(),
                                RecCurrentZoomFov::name_id(),
                                RecFrameCount::name_id(),
                                RecGameBaseAddr::name_id(),
                                RecGameFps::name_id(),
                                RecLocalPlayerBuf::name_id(),
                                RecSpectatorList::name_id(),
                                RecTeammates::name_id(),
                                RecTickDuration::name_id(),
                                RecTickNum::name_id(),
                                RecTickUpdateTimestamp::name_id(),
                                RecViewMatrix::name_id(),
                                RecViewPlayerBuf::name_id(),
                                RecWorldReady::name_id(),
                            ],
                        },
                        SharedStorageGet {
                            parent_id: RecAimTargetsMap::name_id(),
                            ids: aim_targets_list,
                        },
                    ],
                })
                .await?
                .into_inner()
                .list
                .into_iter()
                .map(|ret| (ret.parent_id, ret.data))
                .collect();
            *CACHE.lock() = data;
            Ok(())
        }
        pub(super) async fn get_esp_settings(
            &mut self,
            _: (),
        ) -> Result<Response<EspSettings>, Status> {
            let reply = EspSettings {
                esp: 1,
                screen_width: 1920,
                screen_height: 1080,
                yuan_p: false,
                debug_mode: false,
                esp_visuals: EspVisualsFlag::Box.into(),
                mini_map_radar: true,
                main_map_radar: false,
                max_dist: 2000.0 * 40.0,
                aim_distance: 200.0 * 40.0,
                show_aim_target: true,
                glow_color_viz: Some([0.0, 1.0, 0.0].into()),
                glow_color_notviz: Some([1.0, 0.0, 0.0].into()),
                desired_loots: vec![],
            };
            Ok(Response::new(reply))
        }

        pub(super) async fn get_esp_data(
            &mut self,
            _options: EspDataOption,
        ) -> Result<Response<EspData>, Status> {
            if let Err(e) = self.fetch_values().await {
                tracing::error!(?e);
                return Err(Status::unavailable(e.to_string()));
            }
            let reply = {
                let aim_targets: Vec<_> = RecAimTargetsMap::get_children::<CacheStore>()
                    .unwrap_or_default()
                    .into_values()
                    .collect();
                let aimbot = RecAimbotState::get::<CacheStore>().unwrap();
                let game_fps = RecGameFps::get::<CacheStore>().unwrap().unwrap_or_default();
                let spectators = RecSpectatorList::get::<CacheStore>().unwrap();
                let teammates = RecTeammates::get::<CacheStore>().unwrap();
                let view_matrix = RecViewMatrix::get::<CacheStore>().unwrap();
                let local_player = RecLocalPlayerBuf::get::<CacheStore>().unwrap();
                let view_player = RecViewPlayerBuf::get::<CacheStore>().unwrap();

                EspData {
                    ready: RecGameBaseAddr::get::<CacheStore>().unwrap().is_some(),
                    in_game: RecWorldReady::get::<CacheStore>()
                        .unwrap()
                        .unwrap_or_default(),
                    tick_num: RecTickNum::get::<CacheStore>()
                        .unwrap()
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or(0),
                    frame_count: RecFrameCount::get::<CacheStore>()
                        .unwrap()
                        .unwrap_or_default()
                        .try_into()
                        .unwrap_or_else(|e| {
                            let v = RecFrameCount::get::<CacheStore>();
                            tracing::debug!(?e, ?v);
                            0
                        }),
                    view_matrix,
                    view_player,
                    local_player,
                    aimbot,
                    target_count: aim_targets.len() as u64,
                    targets: Some(AimTargetList {
                        version: 0,
                        elements: aim_targets,
                    }),
                    teammates,
                    spectators,
                    duration_tick: RecTickDuration::get::<CacheStore>()
                        .unwrap()
                        .map(|secs| (secs * 1000.0) as u64)
                        .unwrap_or_default(),
                    duration_actions: RecActionsDuration::get::<CacheStore>()
                        .unwrap()
                        .map(|secs| (secs * 1000.0) as u64)
                        .unwrap_or_default(),
                    data_timestamp: RecTickUpdateTimestamp::get::<CacheStore>()
                        .unwrap()
                        .unwrap_or_default(),
                    game_fps,
                    current_zoom_fov: RecCurrentZoomFov::get::<CacheStore>()
                        .unwrap()
                        .unwrap_or(90.0),
                }
            };
            Ok(Response::new(reply))
        }
        pub(super) async fn get_loots(
            &mut self,
            _options: GetLootsRequest,
        ) -> Result<Response<Loots>, Status> {
            let reply = Loots {
                version: 0,
                loots: vec![],
                data_timestamp: RecTickUpdateTimestamp::get::<CacheStore>()
                    .unwrap()
                    .unwrap_or_default(),
            };
            Ok(Response::new(reply))
        }
    }
}

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
        Some(EspClient::new(tonic_web_wasm_client::Client::new(
            server_url,
        )))
    }
    #[cfg(not(feature = "web-wasm"))]
    {
        match bevy::tasks::block_on(EspClient::connect(server_url)) {
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
    pub(crate) fn get_esp_settings(&self) -> Option<&EspSettings> {
        self.last_settings_fetch_time
            .and_then(|_| Some(&self.esp_settings))
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

#[derive(Resource)]
pub(crate) struct ShowEntityBall(pub(crate) bool);
impl Default for ShowEntityBall {
    fn default() -> Self {
        Self(true)
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
    show_entity_ball: Res<ShowEntityBall>,
    esp_system: Option<ResMut<EspSystem>>,
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
        esp_system.esp_settings = esp_settings;
        esp_system.last_settings_fetch_time = Some(Instant::now());
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

    esp_system.update_latency = time.delta_secs_f64() * 1000.0;

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

    let esp_data_ready = esp_data.ready;

    // Get target entities
    let mut targets: HashMap<u64, UpdateTarget> = if esp_data_ready {
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
            [(0, UpdateTarget {
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
                    is_crosshair_target: false,
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
            })]
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
        let mut spawn_cmd = commands.spawn((
            Transform::from_translation(target.point_pos),
            AimTargetEntity {
                ptr,
                data: target.data,
            },
            Health {
                max: target.max_health,
                current: target.health,
            },
            Mana {
                max: target.max_shield,
                current: target.shield,
            },
        ));
        if show_entity_ball.0 {
            spawn_cmd.insert((
                Mesh3d::from(meshes.add(Sphere::new(6.0).mesh().uv(32, 18))),
                MeshMaterial3d::from(materials.add(StandardMaterial {
                    base_color: Color::Srgba(base_color),
                    ..Default::default()
                })),
            ));
        }
        if esp_system.esp_settings.esp_visuals & EspVisualsFlag::HealthBar as i32 != 0 {
            spawn_cmd.insert((
                hpbar::BarSettings::<Health> {
                    width: 12.,
                    offset: 9.,
                    orientation: hpbar::BarOrientation::Vertical,
                    ..default()
                },
                hpbar::BarSettings::<Mana> {
                    width: 12.,
                    offset: 12.,
                    orientation: hpbar::BarOrientation::Vertical,
                    ..default()
                },
            ));
        }
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
