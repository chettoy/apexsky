use std::time::Duration;

use apexsky::global_state::G_STATE;
use apexsky_proto::pb::apexlegends::{
    AimEntityData, AimTargetItem, AimTargetList, AimbotState, EspData, EspDataOption, EspSettings,
    EspVisualsFlag, GSettings, Loots, Matrix4x4, Players, SpectatorList,
};
use apexsky_proto::pb::esp_service::esp_service_server::{EspService, EspServiceServer};
use apexsky_proto::pb::esp_service::{
    EchoRequest, EchoResponse, GetLootsRequest, GetPlayersRequest,
};
use futures_util::FutureExt;
use obfstr::obfstr as s;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::{sync::watch, time::sleep};
use tonic::codec::CompressionEncoding;
use tonic::{transport::Server, Request, Response, Status};
use tracing::instrument;

use crate::api_impl::GameApiHandle;
use crate::PRINT_LATENCY;

#[tonic::async_trait]
impl EspService for GameApiHandle {
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
        let reply = {
            let players = self.state.players.read().clone();
            let data_timestamp = self.state.update_time.lock().to_owned();
            Players {
                version: 0,
                players: players
                    .into_iter()
                    .map(|(_, pl)| pl.get_buf().clone())
                    .collect(),
                data_timestamp,
            }
        };
        Ok(Response::new(reply))
    }

    async fn get_loots(
        &self,
        request: Request<GetLootsRequest>,
    ) -> Result<Response<Loots>, Status> {
        let req = request.into_inner();
        let filter_dist = req.max_distance > 0.0;
        let filter_id = !req.wish_list.is_empty();
        let reply = {
            let treasure_clues = self.state.treasure_clues.read().clone().into_values();
            let data_timestamp = self.state.update_time.lock().to_owned();
            Loots {
                version: 0,
                loots: match (filter_dist, filter_id) {
                    (true, true) => treasure_clues
                        .filter(|clue| {
                            clue.distance <= req.max_distance
                                && req.wish_list.contains(&clue.item_id)
                        })
                        .collect(),
                    (true, false) => treasure_clues
                        .filter(|clue| clue.distance <= req.max_distance)
                        .collect(),
                    (false, true) => treasure_clues
                        .filter(|clue| req.wish_list.contains(&clue.item_id))
                        .collect(),
                    (false, false) => treasure_clues.collect(),
                },
                data_timestamp,
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
            let state = &self.state;
            let aim_entities = state.aim_entities.read().clone();
            let aim_target = state.aim_target.lock().clone();
            let aimbot_state = state.aimbot_state.lock().clone();
            let game_fps = state.game_fps.lock().to_owned();
            let players = state.players.read().clone();
            let spectators = state.spectator_list.lock().clone();
            let teammates = state.teammates.lock().clone();
            let update_time = self.state.update_time.lock().to_owned();
            let view_matrix = state.view_matrix.lock().to_vec();

            if PRINT_LATENCY {
                println!(
                    "{}{:.1}",
                    s!("esp_service data latency "),
                    apexsky::aimbot::get_unix_timestamp_in_millis() as f64 - update_time * 1000.0
                );
            }

            let (allied_spectator_list, spectator_list) = spectators;
            let local_player = state
                .get_local_player_ptr()
                .and_then(|ptr| players.get(&ptr));
            let view_player = state
                .get_view_player_ptr()
                .and_then(|ptr| players.get(&ptr));

            let aim_targets: Vec<AimTargetItem> = self
                .channels
                .aim_select_rx
                .borrow()
                .iter()
                .filter_map(|target_info| {
                    let Some(entity) = aim_entities.get(&target_info.entity_ptr) else {
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
                            players
                                .get(&target_info.entity_ptr)
                                .map(|pl| pl.get_buf().clone())
                        },
                    })
                })
                .collect();

            EspData {
                ready: state.get_game_baseaddr().is_some(),
                in_game: state.is_world_ready(),
                tick_num: state.tick_num.load(std::sync::atomic::Ordering::Acquire),
                frame_count: state
                    .frame_count
                    .load(std::sync::atomic::Ordering::Acquire)
                    .try_into()
                    .unwrap(),
                view_matrix: Some(Matrix4x4 {
                    elements: view_matrix,
                }),
                view_player: view_player.map(|pl| pl.get_buf()).cloned(),
                local_player: local_player.map(|pl| pl.get_buf()).cloned(),
                aimbot: aimbot_state.as_ref().map(|(state, duration)| AimbotState {
                    version: 0,
                    serialized_data: if op.full_aimbot_state {
                        serde_json::to_string(state).ok()
                    } else {
                        None
                    },
                    loop_duration: duration.as_millis().try_into().unwrap(),
                    target_position: Some(aim_target.into()),
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
                    players: teammates,
                    data_timestamp: update_time,
                }),
                spectators: Some(SpectatorList {
                    elements: [spectator_list, allied_spectator_list].concat(),
                }),
                duration_tick: state
                    .tick_duration
                    .load(std::sync::atomic::Ordering::Relaxed),
                duration_actions: state
                    .actions_duration
                    .load(std::sync::atomic::Ordering::Relaxed),
                data_timestamp: update_time,
                game_fps: game_fps,
                current_zoom_fov: {
                    aimbot_state
                        .is_some_and(|(aimbot, _)| aimbot.get_zoom_state() > 0)
                        .then(|| {
                            view_player
                                .and_then(|pl| pl.get_active_weapon())
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
            let g_settings = G_STATE.lock().unwrap().config.settings.clone();
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
                desired_loots: vec![194, 198, 199, 219, 222, 223, 247, 248, 252, 256, 267],
            }
        };
        Ok(Response::new(reply))
    }

    async fn get_global_settings(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GSettings>, Status> {
        let reply = {
            let g_settings = G_STATE.lock().unwrap().config.settings.clone();
            GSettings {
                ver_stamp: 0,
                serialized_data: serde_json::to_string(&g_settings).ok(),
            }
        };
        Ok(Response::new(reply))
    }
}

#[instrument(skip_all)]
pub async fn esp_loop(
    mut active: watch::Receiver<bool>,
    game_api: GameApiHandle,
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
                let service = EspServiceServer::new(game_api.clone())
                    .accept_compressed(CompressionEncoding::Zstd)
                    .accept_compressed(CompressionEncoding::Gzip)
                    .send_compressed(CompressionEncoding::Zstd)
                    .send_compressed(CompressionEncoding::Gzip);
                let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
                let task = tokio::spawn(
                    Server::builder()
                        .trace_fn(|_| tracing::info_span!("esp_server"))
                        .accept_http1(config.accept_http1)
                        .add_service(tonic_web::enable(service))
                        .serve_with_shutdown(config.listen, shutdown_rx.map(drop)),
                );
                server_task = Some((task, shutdown_tx));
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}
