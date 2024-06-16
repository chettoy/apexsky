use std::{sync::Arc, time::Duration};

use apexsky::global_state::G_STATE;
use apexsky_proto::pb::apexlegends::{
    AimEntityData, AimTargetItem, AimTargetList, AimbotState, EspData, EspDataOption, EspSettings,
    EspVisualsFlag, Loots, Matrix4x4, Players, SpectatorList,
};
use apexsky_proto::pb::esp_service::esp_service_server::{EspService, EspServiceServer};
use apexsky_proto::pb::esp_service::{
    EchoRequest, EchoResponse, GetLootsRequest, GetPlayersRequest,
};
use futures_util::FutureExt;
use obfstr::obfstr as s;
use parking_lot::RwLock;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::{sync::watch, time::sleep};
use tonic::codec::CompressionEncoding;
use tonic::{transport::Server, Request, Response, Status};
use tracing::instrument;

use crate::game::player::GamePlayer;
use crate::{SharedState, TaskChannels};

#[derive(Debug)]
pub struct MyEspService {
    state: Arc<RwLock<SharedState>>,
    channels: TaskChannels,
}

#[tonic::async_trait]
impl EspService for MyEspService {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        tracing::info!("Got a echo request from {:?}", request.remote_addr());
        let reply = EchoResponse {
            message: request.into_inner().message,
        };
        Ok(Response::new(reply))
    }

    async fn get_players(
        &self,
        request: Request<GetPlayersRequest>,
    ) -> Result<Response<Players>, Status> {
        tracing::info!("Got a get_players request from {:?}", request.remote_addr());
        let reply = {
            let lock = self.state.read();
            Players {
                version: 0,
                players: lock
                    .players
                    .values()
                    .map(GamePlayer::get_buf)
                    .cloned()
                    .collect(),
                data_timestamp: lock.update_time,
            }
        };
        Ok(Response::new(reply))
    }

    async fn get_loots(
        &self,
        request: Request<GetLootsRequest>,
    ) -> Result<Response<Loots>, Status> {
        tracing::info!("Got a get_loots request from {:?}", request.remote_addr());
        let req = request.into_inner();
        let filter_dist = req.max_distance > 0.0;
        let filter_id = !req.wish_list.is_empty();
        let reply = {
            let lock = self.state.read();
            Loots {
                version: 0,
                loots: match (filter_dist, filter_id) {
                    (true, true) => lock
                        .treasure_clues
                        .iter()
                        .filter(|clue| {
                            clue.distance <= req.max_distance
                                && req.wish_list.contains(&clue.item_id)
                        })
                        .cloned()
                        .collect(),
                    (true, false) => lock
                        .treasure_clues
                        .iter()
                        .filter(|clue| clue.distance <= req.max_distance)
                        .cloned()
                        .collect(),
                    (false, true) => lock
                        .treasure_clues
                        .iter()
                        .filter(|clue| req.wish_list.contains(&clue.item_id))
                        .cloned()
                        .collect(),
                    (false, false) => lock.treasure_clues.clone(),
                },
                data_timestamp: lock.update_time,
            }
        };
        Ok(Response::new(reply))
    }

    async fn get_esp_data(
        &self,
        request: Request<EspDataOption>,
    ) -> Result<Response<EspData>, Status> {
        let op = request.into_inner();

        let reply = {
            let lock = self.state.read();

            let aim_targets: Vec<AimTargetItem> = self
                .channels
                .aim_select_rx
                .borrow()
                .iter()
                .filter_map(|target_info| {
                    let Some(entity) = lock.aim_entities.get(&target_info.entity_ptr) else {
                        return None;
                    };
                    Some(AimTargetItem {
                        id: target_info.entity_ptr,
                        info: Some(target_info).cloned(),
                        data: Some(AimEntityData {
                            id: entity.get_entity_ptr(),
                            view_angles: Some(entity.get_view_angles().into()),
                            cam_pos: Some(entity.get_cam_pos().into()),
                            sway_angles: Some(entity.get_sway_angles().into()),
                            abs_velocity: Some(entity.get_abs_velocity().into()),
                            head_position: Some(entity.get_bone_position_by_hitbox(0).into()),
                            position: Some(entity.get_position().into()),
                            view_offset: Some(entity.get_view_offset().into()),
                            recoil_angles: Some(entity.get_recoil_angles().into()),
                            team_num: entity.get_team_num(),
                            health: entity.get_health(),
                            shield_health: entity.get_shield_health(),
                            max_health: entity.get_max_health(),
                            max_shield_health: entity.get_max_shield_health(),
                            is_alive: entity.is_alive(),
                            is_knocked: entity.is_knocked(),
                            is_player: entity.is_player(),
                            is_visible: entity.is_visible(),
                        }),
                        player_data: if target_info.is_npc {
                            None
                        } else {
                            lock.players
                                .get(&target_info.entity_ptr)
                                .map(|pl| pl.get_buf())
                                .cloned()
                        },
                    })
                })
                .collect();

            EspData {
                ready: lock.game_baseaddr.is_some(),
                in_game: lock.world_ready,
                tick_num: lock.tick_num,
                frame_count: lock.frame_count.try_into().unwrap(),
                view_matrix: Some(Matrix4x4 {
                    elements: lock.view_matrix.to_vec(),
                }),
                view_player: lock.view_player.as_ref().map(GamePlayer::get_buf).cloned(),
                local_player: lock.local_player.as_ref().map(GamePlayer::get_buf).cloned(),
                aimbot: lock
                    .aimbot_state
                    .as_ref()
                    .map(|(state, duration)| AimbotState {
                        version: 0,
                        serialized_data: if op.full_aimbot_state {
                            serde_json::to_string(state).ok()
                        } else {
                            None
                        },
                        loop_duration: duration.as_millis().try_into().unwrap(),
                        target_position: Some(lock.aim_target.into()),
                        aim_mode: state.get_settings().aim_mode,
                        aiming: state.is_aiming(),
                        gun_safety: state.get_gun_safety(),
                        target_locked: state.is_locked(),
                        aim_key_state: state.get_aim_key_state(),
                        held_id: state.get_held_id(),
                        held_grenade: state.is_grenade(),
                        weapon_id: state.get_weapon_id(),
                        max_fov: state.get_max_fov(),
                        aim_entity: state.get_aim_entity(),
                    }),
                target_count: aim_targets.len() as u64,
                targets: Some(AimTargetList {
                    version: 0,
                    elements: aim_targets,
                }),
                teammates: Some(Players {
                    version: 0,
                    players: lock.teammates.clone(),
                    data_timestamp: lock.update_time,
                }),
                spectators: Some(SpectatorList {
                    elements: [
                        lock.spectator_list.clone(),
                        lock.allied_spectator_list.clone(),
                    ]
                    .concat(),
                }),
                duration_tick: lock.update_duration.1.try_into().unwrap(),
                duration_actions: lock.update_duration.0.try_into().unwrap(),
                data_timestamp: lock.update_time,
                game_fps: lock.game_fps,
                current_zoom_fov: {
                    lock.aimbot_state
                        .as_ref()
                        .is_some_and(|(aimbot, _)| aimbot.get_zoom_state() > 0)
                        .then(|| {
                            lock.view_player
                                .as_ref()
                                .and_then(GamePlayer::get_active_weapon)
                                .map(|weapon| {
                                    let zoom_fov = weapon.cur_zoom_fov;
                                    if zoom_fov.is_normal() && (zoom_fov - 1.0).abs() > f32::EPSILON
                                    {
                                        zoom_fov
                                    } else {
                                        90.0
                                    }
                                })
                                .unwrap_or(90.0)
                        })
                        .unwrap_or(90.0)
                },
            }
        };
        Ok(Response::new(reply))
    }

    async fn get_esp_settings(
        &self,
        _request: Request<()>,
    ) -> Result<Response<EspSettings>, Status> {
        let reply = {
            let g_settings = &G_STATE.lock().unwrap().config.settings;
            EspSettings {
                esp: if g_settings.no_overlay { 0 } else { 1 },
                screen_width: g_settings.screen_width,
                screen_height: g_settings.screen_height,
                yuan_p: g_settings.yuan_p,
                debug_mode: g_settings.debug_mode,
                esp_visuals: {
                    let v = &g_settings.esp_visuals;
                    (if v.r#box {
                        EspVisualsFlag::Box.into()
                    } else {
                        0
                    }) + (if v.damage {
                        EspVisualsFlag::Damage.into()
                    } else {
                        0
                    }) + (if v.distance {
                        EspVisualsFlag::Distance.into()
                    } else {
                        0
                    }) + (if v.health_bar || v.shield_bar {
                        EspVisualsFlag::HealthBar.into()
                    } else {
                        0
                    }) + (if v.line {
                        EspVisualsFlag::Line.into()
                    } else {
                        0
                    }) + (if v.name {
                        EspVisualsFlag::Name.into()
                    } else {
                        0
                    })
                },
                mini_map_radar: g_settings.mini_map_radar,
                main_map_radar: g_settings.main_radar_map,
                max_dist: g_settings.max_dist,
                aim_distance: g_settings.aimbot_settings.aim_dist,
                show_aim_target: g_settings.show_aim_target,
                glow_color_viz: Some(
                    [
                        g_settings.glow_r_viz,
                        g_settings.glow_g_viz,
                        g_settings.glow_b_viz,
                    ]
                    .into(),
                ),
                glow_color_notviz: Some(
                    [
                        g_settings.glow_r_not,
                        g_settings.glow_g_not,
                        g_settings.glow_b_not,
                    ]
                    .into(),
                ),
            }
        };
        Ok(Response::new(reply))
    }
}

#[instrument(skip_all)]
pub async fn esp_loop(
    mut active: watch::Receiver<bool>,
    state: Arc<RwLock<SharedState>>,
    channels: TaskChannels,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let mut server_task: Option<(JoinHandle<Result<(), _>>, oneshot::Sender<()>)> = None;

    while *active.borrow_and_update() {
        sleep(Duration::from_secs(1)).await;
        if G_STATE.lock().unwrap().config.settings.no_overlay {
            if let Some((task, shutdown_tx)) = server_task {
                // Stop server
                if !task.is_finished() {
                    match shutdown_tx.send(()) {
                        Ok(_) => {
                            tracing::info!("{}", s!("Stopping ESP server..."));
                        }
                        Err(_) => {
                            task.abort();
                            tracing::info!("{}", s!("Aborting ESP server..."));
                        }
                    }
                    match task.await {
                        Ok(_) => {
                            tracing::info!("{}", s!("ESP server exit."));
                        }
                        Err(e) => {
                            if e.is_cancelled() {
                                tracing::info!("{}", s!("ESP server cancelled."));
                            } else {
                                tracing::error!(%e, ?e, "{}", s!("error stop esp_server"));
                            }
                        }
                    }
                }
                server_task = None;
            }
        } else {
            if let Some((task, _)) = &server_task {
                // Check task
                if task.is_finished() {
                    if let Some((task, _)) = server_task.take() {
                        match task.await {
                            Ok(r) => {
                                if let Err(e) = r {
                                    tracing::error!(%e, ?e);
                                } else {
                                    tracing::warn!("{}", s!("esp_server finished"));
                                }
                            }
                            Err(e) => {
                                tracing::error!(%e, ?e);
                                if let Ok(reason) = e.try_into_panic() {
                                    tracing::error!(?reason);
                                }
                            }
                        }
                    }
                }
            } else {
                // Start server
                let config = G_STATE.lock().unwrap().config.esp_service.clone();
                let service = EspServiceServer::new(MyEspService {
                    state: state.clone(),
                    channels: channels.clone(),
                })
                .accept_compressed(CompressionEncoding::Zstd)
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Zstd)
                .send_compressed(CompressionEncoding::Gzip);
                let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
                let task = tokio::spawn(
                    Server::builder()
                        .trace_fn(|_| tracing::info_span!("esp_server"))
                        .accept_http1(config.accept_http1)
                        .add_service(service)
                        .serve_with_shutdown(config.listen, shutdown_rx.map(drop)),
                );
                server_task = Some((task, shutdown_tx));
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
