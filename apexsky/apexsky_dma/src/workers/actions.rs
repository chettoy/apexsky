use anyhow::Context;
use apexsky::noobfstr as s;
use apexsky::{
    aimbot::{calc_angle, calc_fov, normalize_angles, AimEntity},
    config::Settings,
    global_state::G_STATE,
    init_spec_checker, is_spec,
    love_players::LoveStatus,
    offsets::G_OFFSETS,
};
use ndarray::arr1;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::mem::size_of;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Instant};
use tracing::{instrument, trace, trace_span, Instrument};

use crate::apexdream::{
    base::math,
    sdk::HighlightBits,
    state::entities::{BaseNPCEntity, LootEntity},
};
use crate::game::{data::*, player::GamePlayer};
use crate::workers::access::{AccessType, PendingAccessRequest, PendingMemRead, PendingMemWrite};
use crate::{SharedState, TreasureClue};

use super::access::MemApi;
use super::aim::{AimKeyStatus, AimbotAction, PreSelectedTarget};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectatorInfo {
    pub ptr: u64,
    pub name: String,
    pub is_teammate: bool,
    pub love_status: LoveStatus,
}

const ENABLE_MEM_AIM: bool = true;

#[instrument(skip_all)]
pub async fn actions_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<RwLock<SharedState>>,
    access_tx: MemApi,
    aim_key_tx: watch::Sender<AimKeyStatus>,
    aim_select_tx: watch::Sender<Vec<PreSelectedTarget>>,
    aim_delta_angles_tx: watch::Sender<[f32; 3]>,
    mut aim_action_rx: mpsc::Receiver<AimbotAction>,
    mut aim_select_rx: watch::Receiver<Vec<PreSelectedTarget>>,
    mut items_glow_rx: watch::Receiver<Vec<(u64, u8)>>,
) -> anyhow::Result<()> {
    let mut apexdream = crate::apexdream::Instance::new();
    let mut start_instant = Instant::now();
    let mut fps_checkpoint_instant = Instant::now();
    let mut last_checkpoint_frame: i32 = 0;
    let mut prev_lplayer_ptr: u64 = 0;
    let mut actions_tick: i64 = -1;
    let mut log_items: usize = 0;
    let mut world_ready: bool = false;
    let mut player_ready: bool = false;
    let mut prev_view_angles: Option<[f32; 3]> = None;

    tracing::debug!("{}", s!("task start"));

    while *active.borrow_and_update() {
        sleep(Duration::from_secs(2)).await;

        if AccessType::mem_baseaddr()
            .with_priority(100)
            .dispatch(&access_tx)
            .await?
            .await?
            .is_none()
        {
            shared_state.write().game_attached = false;
            continue;
        }

        if !shared_state.read().game_attached {
            shared_state.write().game_attached = true;
        }

        while *active.borrow_and_update() {
            sleep(Duration::from_millis(2)).await; // don't change xD

            let loop_duration = start_instant.elapsed().as_millis();
            start_instant = Instant::now();

            let Some(apex_base) = AccessType::mem_baseaddr()
                .with_priority(100)
                .dispatch(&access_tx)
                .await?
                .await?
            else {
                shared_state.write().game_attached = false;
                break;
            };
            let mem = &access_tx;

            /* Hot Variables Update Begin */

            actions_tick += 1;
            let verbose = actions_tick % 1_000 == 0;

            // Tick game state
            let tick_duration = {
                let mut api = crate::apexdream::api::Api {
                    apex_base: apex_base.into(),
                    mem_access: mem.clone(),
                };
                let tick_start = Instant::now();
                apexdream.tick_state(&mut api).await;
                tick_start.elapsed().as_millis()
            };
            let apex_state = apexdream.get_state();
            if verbose {
                trace!(loop_duration, tick_duration, ?apex_state.client, "{}", s!("game state update"));
            }

            // Update spectator checker realtime
            if actions_tick % 15 == 0 && player_ready {
                let lplayer_ptr = apex_state.client.local_player_ptr;
                // Init spectator checker
                if prev_lplayer_ptr != lplayer_ptr {
                    init_spec_checker(lplayer_ptr);
                    prev_lplayer_ptr = lplayer_ptr;
                }
                // Update yaw to spec checker
                apex_state.players().for_each(|pl| {
                    if pl.eadp_uid == 0 || pl.team_num < 0 || pl.team_num > 50 {
                        return;
                    }
                    apexsky::tick_yaw(pl.entity_ptr.into_raw(), pl.yaw);
                });
            }

            // Calc game FPS
            let game_fps_update = {
                let delta_frame = apex_state.client.framecount - last_checkpoint_frame;
                if delta_frame < 90 {
                    None
                } else if delta_frame > 120 {
                    last_checkpoint_frame = apex_state.client.framecount;
                    fps_checkpoint_instant = Instant::now();
                    None
                } else {
                    let duration = fps_checkpoint_instant.elapsed().as_millis();
                    //trace!(delta_frame, duration, "{}", s!("calc game fps"));

                    last_checkpoint_frame = apex_state.client.framecount;
                    fps_checkpoint_instant = Instant::now();

                    Some(delta_frame as f32 * 1000.0 / duration as f32)
                }
            };

            // To hold only one lock at a time, clone G_STATE.config.settings
            let g_settings = G_STATE.lock().unwrap().config.settings.clone();
            // if verbose {
            //     trace!(?g_settings);
            // }

            trace_span!("Update client state").in_scope(|| {
                world_ready = apex_state.is_in_game() && apex_state.local_player().is_some();

                let mut wlock = shared_state.write();

                wlock.world_ready = world_ready;
                player_ready = world_ready && wlock.local_player.is_some();

                wlock.frame_count = apex_state.client.framecount;
                wlock.view_matrix = apex_state.client.view_matrix;
                wlock.update_time = apex_state.time;
                wlock.update_duration = (loop_duration, tick_duration);

                if !player_ready {
                    wlock.spectator_list.clear();
                    wlock.allied_spectator_name.clear();
                }

                if let Some(fps_update) = game_fps_update {
                    wlock.game_fps = fps_update;
                    trace!(fps_update);
                } else if wlock.game_fps < f32::EPSILON {
                    wlock.game_fps = g_settings.game_fps;
                }

                if verbose {
                    trace!(shared_state = ?wlock);
                }
            });

            // Log WeaponId
            if apexdream.is_newly_connected() {
                tracing::info!(?apex_state.string_tables.weapon_names);
            }

            // Perform aimbot actions
            async fn perform_aimbot_actions(
                lplayer: &GamePlayer,
                mem: &MemApi,
                apex_base: u64,
                prev_view_angles: &mut Option<[f32; 3]>,
                aim_delta_angles_tx: &watch::Sender<[f32; 3]>,
                aim_action_rx: &mut mpsc::Receiver<AimbotAction>,
                apex_state: &crate::apexdream::state::GameState,
            ) {
                let lplayer = lplayer.get_entity();
                let ptr = lplayer.entity_ptr.into_raw();

                // read view_angles
                let view_angles = match read_viewangles(mem, ptr).await {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!(%e, "{}", s!("err read viewangles"));
                        return;
                    }
                };

                if view_angles[2].abs() > 1.0 {
                    tracing::warn!(?view_angles, "{}", s!("got invalid view_angles"));
                    return;
                }
                // send delta_view_angles
                let delta_view_angles = prev_view_angles
                    .map(|prev| math::sub(view_angles, prev))
                    .unwrap_or([0.0, 0.0, 0.0]);
                aim_delta_angles_tx
                    .send(delta_view_angles)
                    .unwrap_or_else(|e| {
                        tracing::error!(%e, ?aim_delta_angles_tx, "{}", s!("send delta_view_angles"));
                    });
                *prev_view_angles = Some(view_angles);

                // aimbot actions
                match aim_action_rx.try_recv() {
                    Ok(action) => {
                        if ENABLE_MEM_AIM {
                            if let Some(delta) = action.shift_angles {
                                // calc and check target view angles
                                let mut update_angles = math::add(view_angles, delta);
                                if update_angles[0].abs() > 360.0
                                    || update_angles[1].abs() > 360.0
                                    || update_angles[2].abs() > 1.0
                                {
                                    tracing::warn!(
                                        ?update_angles,
                                        "{}",
                                        s!("got invalid target view_angles")
                                    );
                                    return;
                                }
                                normalize_angles(&mut update_angles);

                                // write target view angles
                                if let Err(e) = write_viewangles(mem, ptr, &update_angles).await {
                                    tracing::warn!(%e, "{}", s!("err write viewangles"));
                                    return;
                                }
                                *prev_view_angles = Some(update_angles);
                            }
                            if let Some(trigger) = action.force_attack {
                                if trigger != apex_state.in_attack_state() {
                                    let force_attack = if trigger { 5 } else { 4 };
                                    if let Err(e) =
                                        write_attack_button(mem, apex_base, force_attack).await
                                    {
                                        tracing::warn!(%e, "{}", s!("err write force_attack"));
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        use tokio::sync::mpsc::error::TryRecvError;
                        if e != TryRecvError::Empty {
                            tracing::error!(%e, ?aim_action_rx, "{}", s!("perform aimbot actions"));
                        }
                    }
                }
            }
            if player_ready {
                if let Some(lplayer) = &shared_state.read().local_player {
                    perform_aimbot_actions(
                        lplayer,
                        mem,
                        apex_base,
                        &mut prev_view_angles,
                        &aim_delta_angles_tx,
                        &mut aim_action_rx,
                        apex_state,
                    )
                    .instrument(trace_span!("Perform aimbot actions"))
                    .await;
                } else {
                    tracing::warn!("{}", s!("UNREACHABLE: invalid localplayer"));
                }
            }

            trace_span!("Send key status to aimbot worker").in_scope(|| {
                aim_key_tx
                    .send(AimKeyStatus {
                        aimbot_hotkey_1: if apex_state.is_button_down(g_settings.aimbot_hot_key_1) {
                            g_settings.aimbot_hot_key_1
                        } else {
                            0
                        },
                        aimbot_hotkey_2: if apex_state.is_button_down(g_settings.aimbot_hot_key_2) {
                            g_settings.aimbot_hot_key_2
                        } else {
                            0
                        },
                        attack_button: apex_state.buttons.in_attack.down[0],
                        zoom_button: apex_state.buttons.in_zoom.down[0],
                        triggerbot_hotkey: if apex_state
                            .is_button_down(g_settings.trigger_bot_hot_key)
                        {
                            g_settings.trigger_bot_hot_key
                        } else {
                            0
                        },
                        attack_state: match apex_state.buttons.in_attack.state.try_into() {
                            Ok(v) => v,
                            Err(_) => {
                                tracing::error!("{}", s!("err convert in_attack to i32"));
                                return;
                            }
                        },
                    })
                    .unwrap_or_else(|e| {
                        tracing::error!(%e, ?aim_key_tx, "{}", s!("send key status"));
                    });
            });

            if player_ready {
                trace_span!("Update view_player hot data").in_scope(|| {
                    if let Some(value) = apex_state.local_player() {
                        shared_state
                            .write()
                            .view_player
                            .as_mut()
                            .map(|view_player| view_player.update_buf_hotdata(value));
                    }
                });

                trace_span!("Update entities hot data").in_scope(|| {
                    let player_entities = apex_state.players();
                    let local_player_ptr = apex_state.client.local_player_ptr;

                    let mut players: HashMap<u64, GamePlayer> = shared_state.read().players.clone();
                    let mut aim_entities = shared_state.read().aim_entities.clone();

                    player_entities.for_each(|pl| {
                        let entity_ptr = pl.entity_ptr.into_raw();
                        if pl.eadp_uid == 0 || pl.team_num < 0 || pl.team_num > 50 {
                            players.remove(&entity_ptr);
                            return;
                        }
                        aim_entities.insert(entity_ptr, Arc::new(pl.clone()));
                        if let Some(player) = players.get_mut(&entity_ptr) {
                            player.update_buf_hotdata(&pl);
                        }
                    });

                    let local_player: Option<GamePlayer> = players.get(&local_player_ptr).cloned();

                    let mut state_wlock = shared_state.write();

                    state_wlock.local_player = local_player;
                    state_wlock.players = players;
                    state_wlock.aim_entities = aim_entities;
                });
            }

            /* Hot Variables Update End */

            if actions_tick % 2 == 0 {
                // at least 30ms // don't change xD
            } else if actions_tick % 30_000 == 0 {
                actions_tick = 0;
            } else {
                continue;
            }

            /* Cold Variables Update Start */

            trace_span!("Update state in global settings").in_scope(|| {
                let firing_range_mode = apex_state.is_firing_range();
                let g_state = &mut G_STATE.lock().unwrap();
                if g_state.config.settings.firing_range != firing_range_mode {
                    g_state.config.settings.firing_range = firing_range_mode;
                    g_state.tui_forceupdate = true;
                }
                if g_state.config.settings.calc_game_fps {
                    if let Some(fps_update) = game_fps_update {
                        g_state.config.settings.game_fps = fps_update;
                    }
                }
            });

            trace_span!("Update entities").in_scope(|| {
                if world_ready {
                    let view_player: Option<GamePlayer> = apex_state.local_player().map(|entity| {
                        GamePlayer::new(
                            entity.clone(),
                            apex_state,
                            &mut G_STATE.lock().unwrap().config,
                        )
                    });

                    let mut players: HashMap<u64, GamePlayer> = HashMap::new();
                    let mut aim_entities: HashMap<u64, Arc<dyn AimEntity>> = HashMap::new();

                    apex_state.players().for_each(|pl| {
                        // FIXME: skip wrong entity
                        if pl.eadp_uid == 0 || pl.team_num < 0 || pl.team_num > 50 {
                            return;
                        }
                        let game_player = GamePlayer::new(
                            pl.clone(),
                            apex_state,
                            &mut G_STATE.lock().unwrap().config,
                        );
                        players.insert(pl.entity_ptr.into_raw(), game_player);
                        aim_entities.insert(pl.entity_ptr.into_raw(), Arc::new(pl.clone()));
                    });
                    apex_state
                        .entities_as::<BaseNPCEntity>()
                        .for_each(|entity| {
                            aim_entities
                                .insert(entity.entity_ptr.into_raw(), Arc::new(entity.clone()));
                        });

                    let player_count = players.len();
                    let entity_count = aim_entities.len();
                    tracing::trace!(player_count, entity_count, "{}", s!("AimEntities updated"));

                    let local_player: Option<GamePlayer> =
                        players.get(&apex_state.client.local_player_ptr).cloned();

                    let mut state_wlock = shared_state.write();
                    state_wlock.local_player = local_player;
                    state_wlock.view_player = view_player;
                    state_wlock.players = players;
                    state_wlock.aim_entities = aim_entities;
                } else {
                    let mut state_wlock = shared_state.write();
                    state_wlock.local_player = None;
                    state_wlock.view_player = None;
                    state_wlock.players.clear();
                    state_wlock.aim_entities.clear();
                }
            });

            trace_span!("Update loots").in_scope(|| {
                if world_ready {
                    let Some(local_position) = shared_state
                        .read()
                        .local_player
                        .as_ref()
                        .map(|l| arr1(&l.get_entity().origin))
                    else {
                        return;
                    };

                    let mut loots: Vec<TreasureClue> = Vec::new();

                    apex_state.entities_as::<LootEntity>().for_each(|entity| {
                        let distance = (arr1(&entity.origin) - &local_position)
                            .mapv(|x| x * x)
                            .sum()
                            .sqrt();
                        let clue = TreasureClue {
                            item_id: entity.custom_script_int,
                            custom_item_id: (entity.custom_script_int as u64
                                | (entity.survival_property as u64) << 32),
                            position: entity.origin,
                            distance,
                            entity_ptr: entity.entity_ptr.into_raw(),
                        };
                        loots.push(clue);
                    });

                    let loot_count = loots.len();
                    tracing::trace!(loot_count, "{}", s!("loots updated"));

                    if verbose {
                        tracing::trace!(?apex_state.entity_list.entities, "{}", s!("entity_list"));
                    }

                    // Log ItemId
                    if apex_state.is_firing_range() {
                        if loot_count == 0 {
                            tracing::debug!("{}", s!("wait items"));
                        } else if loot_count > log_items {
                            #[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
                            struct LootInt {
                                int: i32,
                                model: String,
                            }
                            let item_namelist = apex_state
                                .entities_as::<LootEntity>()
                                .map(|entity| LootInt {
                                    int: entity.custom_script_int,
                                    model: entity.model_name.string.clone(),
                                })
                                .collect::<HashSet<LootInt>>();
                            let mut item_namelist: Vec<_> = item_namelist.into_iter().collect();
                            item_namelist.sort_by(|a, b| a.int.cmp(&b.int));

                            match serde_json::to_string(&item_namelist) {
                                Ok(items_json) => {
                                    tracing::info!(items_json, "{loot_count}{}", s!(" items sorted"))
                                }
                                Err(e) => {
                                    tracing::warn!(%e, ?item_namelist, "{loot_count}{}", s!(" items sorted"))
                                }
                            }

                            log_items = loot_count;
                        }
                    }

                    shared_state.write().treasure_clues = loots;
                } else {
                    shared_state.write().treasure_clues.clear();
                }
            });

            trace_span!("Targeting").in_scope(|| {
                // Send aim targets
                aim_select_tx
                    .send(if player_ready {
                        // Iterate over all targetable entities
                        let mut aim_targets: Vec<PreSelectedTarget> = {
                            let state = shared_state.read();
                            if let Some(lplayer) = state.local_player.as_ref() {
                                state
                                    .aim_entities
                                    .values()
                                    .filter_map(|entity| {
                                        process_player(
                                            lplayer,
                                            entity.as_ref(),
                                            &state,
                                            &g_settings,
                                        )
                                    })
                                    .collect()
                            } else {
                                tracing::error!("{}", s!("UNREACHABLE: invalid localplayer"));
                                vec![]
                            }
                        };
                        aim_targets.sort_by(|a, b| {
                            a.distance.partial_cmp(&b.distance).unwrap_or_else(|| {
                                tracing::error!(?a, ?b, "{}", s!("sort"));
                                panic!()
                            })
                        });
                        aim_targets
                    } else {
                        vec![]
                    })
                    .unwrap_or_else(|e| {
                        tracing::error!(%e, ?aim_select_tx, "{}", s!("send aim targets"));
                    });
            });

            if player_ready {
                trace_span!("Spectator check").in_scope(|| {
                    let Some((lplayer_ptr, lplayer_alive, lplayer_team)) =
                        shared_state.read().local_player.as_ref().map(|p| {
                            (
                                p.get_entity().entity_ptr.into_raw(),
                                p.get_buf().is_alive,
                                p.get_buf().team_num,
                            )
                        })
                    else {
                        return;
                    };

                    let tdm_toggle = g_settings.tdm_toggle;
                    let shared_state = shared_state.clone();
                    {
                        let players = shared_state.read().players.clone();
                        let alter_local_team = shared_state.read().map_testing_local_team;

                        let is_teammate = |team_num| {
                            teammate_check(team_num, lplayer_team, alter_local_team, tdm_toggle)
                        };

                        // Update local entity yew
                        // let yew = mem.apex_mem_read::<f32>(lplayer_ptr + OFFSET_YAW)?;
                        // trace!(lplayer_ptr, ?yew);
                        // apexsky::tick_yew(lplayer_ptr, yew);

                        let mut teammates: Vec<_> = players
                            .iter()
                            .filter_map(|(_, target_entity)| {
                                let player_buf = target_entity.get_buf();
                                if is_teammate(player_buf.team_num) {
                                    Some(player_buf.to_owned())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        teammates.sort_by(|a, b| a.team_member_index.cmp(&b.team_member_index));

                        // Update spectator checker
                        let tmp_specs: Vec<SpectatorInfo> = players
                            .iter()
                            .filter_map(|(&target_ptr, target_entity)| {
                                let player_buf = target_entity.get_buf();
                                if player_buf.is_alive && lplayer_alive {
                                    None
                                } else {
                                    // Update yaw to spec checker
                                    //apexsky::tick_yaw(target_ptr, player_buf.yaw);

                                    // Exclude self from list when watching others
                                    if target_ptr != lplayer_ptr && is_spec(target_ptr) {
                                        Some(SpectatorInfo {
                                            ptr: target_ptr,
                                            name: player_buf.player_name.clone(),
                                            is_teammate: is_teammate(player_buf.team_num),
                                            love_status: player_buf
                                                .love_state
                                                .try_into()
                                                .unwrap_or_else(|_| {
                                                    tracing::error!(
                                                        love_state = player_buf.love_state,
                                                        ?player_buf
                                                    );
                                                    LoveStatus::Normal
                                                }),
                                        })
                                    } else {
                                        None
                                    }
                                }
                            })
                            .collect();

                        // Update spectator namelist
                        let (allied_spectators, spectators): (Vec<_>, Vec<_>) =
                            tmp_specs.into_iter().partition(|info| info.is_teammate);
                        let allied_spectator_name = allied_spectators
                            .into_iter()
                            .map(|info| info.name)
                            .collect();

                        {
                            let mut wlock = shared_state.write();
                            wlock.allied_spectator_name = allied_spectator_name;
                            wlock.spectator_list = spectators;
                            wlock.teammates = teammates;
                        }
                    };
                });

                // Inject highlight settings
                let highlight_injected = {
                    let mut injected = shared_state.read().highlight_injected;
                    if !injected {
                        let glow_fix_i32 =
                            AccessType::mem_read(apex_base + OFFSET_GLOW_FIX, size_of::<i32>(), 0)
                                .dispatch(mem)
                                .await?
                                .recv_for::<i32>()
                                .await?;
                        let glow_fix_u8 =
                            AccessType::mem_read(apex_base + OFFSET_GLOW_FIX, size_of::<u8>(), 0)
                                .dispatch(mem)
                                .await?
                                .recv_for::<u8>()
                                .await?;
                        tracing::trace!(glow_fix_i32, glow_fix_u8);
                    }
                    if (g_settings.player_glow || g_settings.item_glow) && player_ready {
                        match inject_highlight(mem, apex_state.client.framecount, &g_settings).await
                        {
                            Ok(_) => {
                                shared_state.write().highlight_injected = true;
                                injected = true;
                            }
                            Err(e) => {
                                tracing::debug!(%e, ?e, "{}", s!("Inject highlight settings"));
                            }
                        }
                    }
                    injected
                };

                // Write Player Glow
                if g_settings.player_glow
                    && highlight_injected
                    && aim_select_rx.has_changed().unwrap_or_else(|e| {
                        tracing::error!(%e, ?aim_select_rx, "{}", s!("perform player glow"));
                        false
                    })
                {
                    let reqs = aim_select_rx
                        .borrow_and_update()
                        .iter()
                        .map(|target| {
                            let target_ptr = target.entity_ptr;
                            let highlight_context_id = player_glow(
                                target,
                                apex_state.client.framecount,
                                g_settings.game_fps,
                                g_settings.player_glow_armor_color,
                                g_settings.player_glow_love_user,
                            );
                            (
                                AccessType::mem_write_typed::<u8>(
                                    target_ptr + G_OFFSETS.entity_highlight_generic_context - 1,
                                    &highlight_context_id,
                                    0,
                                ),
                                AccessType::mem_write_typed::<i32>(
                                    target_ptr + OFFSET_GLOW_VISIBLE_TYPE,
                                    &2,
                                    0,
                                ),
                                AccessType::mem_write_typed::<f32>(
                                    target_ptr + OFFSET_GLOW_DISTANCE,
                                    &8.0E+4,
                                    0,
                                ),
                                AccessType::mem_write_typed::<i32>(
                                    target_ptr + OFFSET_GLOW_FIX,
                                    &0,
                                    0,
                                ),
                            )
                        })
                        .collect::<Vec<_>>();
                    for (write_glow_id, write_glow_type, write_glow_dist, write_glow_fix) in reqs {
                        write_glow_id
                            .with_priority(0)
                            .dispatch(mem)
                            .await?
                            .spawn_err_handler();
                        write_glow_type
                            .with_priority(0)
                            .dispatch(mem)
                            .await?
                            .spawn_err_handler();
                        write_glow_dist
                            .with_priority(0)
                            .dispatch(mem)
                            .await?
                            .spawn_err_handler();
                        write_glow_fix
                            .with_priority(0)
                            .dispatch(mem)
                            .await?
                            .spawn_err_handler();
                    }
                }

                // Write Items Glow
                if g_settings.item_glow
                    && highlight_injected
                    && items_glow_rx.has_changed().unwrap_or_else(|e| {
                        tracing::error!(%e, ?items_glow_rx, "{}", s!("perform items glow"));
                        false
                    })
                {
                    let reqs = items_glow_rx
                        .borrow_and_update()
                        .iter()
                        .map(|(ptr, ctx_id)| {
                            AccessType::mem_write_typed(
                                ptr + G_OFFSETS.entity_highlight_generic_context - 1,
                                ctx_id,
                                0,
                            )
                        })
                        .collect::<Vec<_>>();
                    for req in reqs {
                        req.with_priority(0)
                            .dispatch(mem)
                            .await?
                            .spawn_err_handler();
                    }
                }

                // Weapon model glow
                // Not planned
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

#[instrument(skip_all)]
fn process_player<'a>(
    local_player: &GamePlayer,
    target_entity: &dyn AimEntity,
    state: &SharedState,
    g_settings: &Settings,
    //mem: &mut MemProcImpl<'a>,
) -> Option<PreSelectedTarget> {
    let lplayer_ptr = local_player.get_entity().get_entity_ptr();
    let target_ptr = target_entity.get_entity_ptr();

    let entity_team = target_entity.get_team_num();
    let local_team = local_player.get_buf().team_num;

    let is_teammate = teammate_check(
        entity_team,
        local_team,
        state.map_testing_local_team,
        g_settings.tdm_toggle,
    );
    // trace!(target_ptr, entity_team, is_teammate);

    // Exclude eliminated players
    if !target_entity.is_alive() {
        return None;
    }

    // Teammate and 1v1 check
    if !g_settings.onevone {
        if g_settings.firing_range {
            if target_entity.is_player() {
                return None;
            }
        } else {
            if is_teammate {
                return None;
            }
        }
    }

    // Exclude self
    if target_ptr == lplayer_ptr {
        return None;
    }

    // Exclude players in invalid team
    if target_entity.is_player() && (entity_team < 0 || entity_team > 50) {
        tracing::warn!(?entity_team, ?target_entity, "{}", s!("invalid team"));
        return None;
    }

    // Calc distance
    let distance = {
        let target_pos = target_entity.get_position();
        let local_pos = local_player.get_entity().get_position();
        math::dist(target_pos, local_pos)
    };

    // Excluding targets that are too far or too close
    if distance > g_settings.max_dist || distance < 20.0 {
        return None;
    }

    let target_player = if target_entity.is_player() {
        match state.players.get(&target_ptr) {
            Some(p) => Some(p),
            None => {
                tracing::error!(?target_ptr, "{}", s!("UNREACHABLE"));
                return None;
            }
        }
    } else {
        None
    };

    Some(PreSelectedTarget {
        fov: calculate_target_fov(local_player.get_entity(), target_entity),
        distance,
        is_visible: target_entity.is_visible(),
        is_knocked: target_entity.is_knocked(),
        health_points: { target_entity.get_shield_health() + target_entity.get_health() },
        love_status: {
            if let Some(target_player) = target_player {
                target_player
                    .get_buf()
                    .love_state
                    .try_into()
                    .unwrap_or_else(|_| {
                        tracing::error!(love_state = target_player.get_buf().love_state, player_buf = ?target_player.get_buf());
                        LoveStatus::Normal
                    })
            } else {
                // not player (dummy)
                if g_settings.yuan_p {
                    LoveStatus::Love
                } else {
                    LoveStatus::Normal
                }
            }
        },
        is_kill_leader: {
            if let Some(target_player) = target_player {
                GamePlayer::is_kill_leader(target_player.get_buf())
            } else {
                false
            }
        },
        entity_ptr: target_ptr,
    })
}

#[instrument(skip_all)]
fn player_glow(
    target: &PreSelectedTarget,
    frame_count: i32,
    game_fps: f32,
    player_glow_armor_color: bool,
    player_glow_love_user: bool,
) -> u8 {
    let mut setting_index = {
        if target.is_knocked {
            HIGHLIGHT_PLAYER_KNOCKED
        } else if target.is_visible {
            HIGHLIGHT_PLAYER_VISIBLE
        } else {
            if player_glow_armor_color {
                let hp = target.health_points;
                if hp <= 100 {
                    HIGHLIGHT_PLAYER_ORANGE
                } else if hp <= 150 {
                    HIGHLIGHT_PLAYER_WHITE
                } else if hp <= 175 {
                    HIGHLIGHT_PLAYER_BLUE
                } else if hp <= 200 {
                    HIGHLIGHT_PLAYER_PURPLE
                } else if hp <= 225 {
                    HIGHLIGHT_PLAYER_RED
                } else {
                    HIGHLIGHT_PLAYER_BLACK
                }
            } else {
                HIGHLIGHT_PLAYER_NOTVIZ
            }
        }
    };

    // love player glow
    if player_glow_love_user {
        let frame_frag = frame_count / game_fps as i32;
        if target.is_visible || frame_frag % 2 == 0 {
            match target.love_status {
                LoveStatus::Love => {
                    setting_index = HIGHLIGHT_PLAYER_RAINBOW;
                }
                LoveStatus::Hate => {
                    setting_index = HIGHLIGHT_PLAYER_BLACK;
                }
                _ => (),
            }
        }
    }

    // kill leader glow
    if target.is_kill_leader {
        let frame_frag = frame_count / game_fps as i32;
        if target.is_visible || frame_frag % 3 == 0 {
            setting_index = HIGHLIGHT_PLAYER_ORANGE;
        }
    }

    setting_index
}

#[instrument(skip_all)]
async fn inject_highlight(
    mem: &MemApi,
    frame_count: i32,
    g_settings: &Settings,
) -> anyhow::Result<()> {
    let bits_loot = HighlightBits::new(g_settings.loot_filled, 125, 64, 7, true, false);
    let bits_box = HighlightBits::new(0, 125, 64, 7, true, false);
    let bits_player_fill = HighlightBits::new(
        g_settings.player_glow_inside_value,
        6,
        g_settings.player_glow_outline_size,
        7,
        true,
        false,
    );
    let bits_player_outline =
        HighlightBits::new(0, 6, g_settings.player_glow_outline_size, 7, true, false);

    let highlight_settings_inject: [(u8, &HighlightBits, [f32; 3]); 20] = [
        (HIGHLIGHT_LOOT_HEAVY, &bits_loot, [0.0, 1.0, 1.0]),
        (HIGHLIGHT_LOOT_LIGHT, &bits_loot, [1.0, 0.5490, 0.0]),
        (HIGHLIGHT_LOOT_RED, &bits_loot, [1.0, 0.0, 0.0]),
        (HIGHLIGHT_LOOT_BLUE, &bits_loot, [0.0, 0.7490, 1.0]),
        (HIGHLIGHT_LOOT_GREY, &bits_loot, [0.6, 0.6, 0.6]),
        (HIGHLIGHT_LOOT_WHITE, &bits_loot, [1.0, 1.0, 1.0]),
        (HIGHLIGHT_LOOT_ENERGY, &bits_loot, [0.2, 1.0, 0.0]),
        (HIGHLIGHT_LOOT_PURPLE, &bits_loot, [0.2941, 0.0, 0.5098]),
        (HIGHLIGHT_LOOT_GOLD, &bits_loot, [1.0, 0.8431, 0.0]),
        (HIGHLIGHT_DEATH_BOX, &bits_box, [1.0, 0.0, 0.0]),
        (
            HIGHLIGHT_PLAYER_KNOCKED,
            &bits_player_outline,
            [
                g_settings.glow_r_knocked,
                g_settings.glow_g_knocked,
                g_settings.glow_b_knocked,
            ],
        ),
        (
            HIGHLIGHT_PLAYER_VISIBLE,
            &bits_player_outline,
            [
                g_settings.glow_r_viz,
                g_settings.glow_g_viz,
                g_settings.glow_b_viz,
            ],
        ),
        (
            HIGHLIGHT_PLAYER_NOTVIZ,
            &bits_player_fill,
            [
                g_settings.glow_r_not,
                g_settings.glow_g_not,
                g_settings.glow_b_not,
            ],
        ),
        (
            HIGHLIGHT_PLAYER_BLACK,
            &bits_player_fill,
            [2.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_ORANGE,
            &bits_player_fill,
            [255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_WHITE,
            &bits_player_fill,
            [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_BLUE,
            &bits_player_fill,
            [39.0 / 255.0, 178.0 / 255.0, 255.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_PURPLE,
            &bits_player_fill,
            [206.0 / 255.0, 59.0 / 255.0, 255.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_RED,
            &bits_player_fill,
            [219.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0],
        ),
        (
            HIGHLIGHT_PLAYER_RAINBOW,
            &bits_player_fill,
            rainbow_color(frame_count),
        ),
    ];

    let Some(base) = AccessType::mem_baseaddr().dispatch(mem).await?.await? else {
        return Ok(());
    };
    let highlight_settings_ptr =
        AccessType::mem_read(base + OFFSET_HIGHLIGHT_SETTINGS, size_of::<u64>(), 0)
            .dispatch(mem)
            .await?
            .recv_for::<u64>()
            .await?;
    for (context_id, bits, color) in highlight_settings_inject {
        let context_offset = highlight_settings_ptr + 0x34 * context_id as u64;
        AccessType::mem_write_typed::<HighlightBits>(context_offset, bits, 0)
            .dispatch(mem)
            .await?
            .await?
            .context(format!("{:?}", context_id))?;
        AccessType::mem_write_typed::<[f32; 3]>(context_offset + 4, &color, 0)
            .dispatch(mem)
            .await?
            .await?
            .context(format!("{:?}", context_id))?;
    }
    tracing::trace!(highlight_settings_ptr);
    AccessType::mem_write_typed::<i32>(base + OFFSET_GLOW_FIX, &1, 0)
        .dispatch(mem)
        .await?
        .await?
        .unwrap_or_else(|e| {
            tracing::debug!(%e, "{}", s!("err write glow fix"));
        });

    Ok(())
}

#[instrument(skip_all)]
fn calculate_target_fov(from: &dyn AimEntity, target: &dyn AimEntity) -> f32 {
    let view_angles = from.get_sway_angles();
    let local_camera = from.get_cam_pos();
    let entity_position = target.get_bone_position_by_hitbox(2);
    let angle = calc_angle(&local_camera, &entity_position);
    calc_fov(&view_angles, &angle)
}

fn rainbow_color(frame_count: i32) -> [f32; 3] {
    const FREQUENCY: f32 = 0.1; // Adjust the speed of color change
    const AMPLITUDE: f32 = 0.5; // Adjust the amplitude of color change

    // Use the sine function to generate rainbow color variation
    let frame_number = frame_count as f32;
    let r = (FREQUENCY * frame_number + 0.0).sin() * AMPLITUDE + 0.5;
    let g = (FREQUENCY * frame_number + 2.0).sin() * AMPLITUDE + 0.5;
    let b = (FREQUENCY * frame_number + 4.0).sin() * AMPLITUDE + 0.5;

    // Clamp the colors to the range [0, 1]
    [
        r.min(1.0).max(0.0),
        g.min(1.0).max(0.0),
        b.min(1.0).max(0.0),
    ]
}

fn teammate_check(
    entity_team: i32,
    local_team: i32,
    map_testing_local_team: i32,
    tdm_toggle: bool,
) -> bool {
    if tdm_toggle {
        let ent_team = if entity_team % 2 == 1 { 1 } else { 2 };
        let loc_team = if local_team % 2 == 1 { 1 } else { 2 };
        trace!(
            target_team = ent_team,
            local_team = loc_team,
            "{}",
            s!("TDM check")
        );
        ent_team == loc_team
    } else {
        entity_team == local_team
            || (map_testing_local_team != 0 && entity_team == map_testing_local_team)
    }
}

async fn read_viewangles(mem: &MemApi, ptr: u64) -> anyhow::Result<[f32; 3]> {
    AccessType::mem_read(ptr + G_OFFSETS.player_viewangles, size_of::<[f32; 3]>(), 0)
        .with_priority(50)
        .dispatch(mem)
        .await?
        .recv_for::<[f32; 3]>()
        .await
}

async fn write_viewangles(mem: &MemApi, ptr: u64, data: &[f32; 3]) -> anyhow::Result<()> {
    AccessType::mem_write_typed::<[f32; 3]>(ptr + G_OFFSETS.player_viewangles, data, 0)
        .with_priority(50)
        .dispatch(mem)
        .await?
        .await?
}

async fn write_attack_button(
    mem: &MemApi,
    apex_base: u64,
    force_attack_state: i32,
) -> anyhow::Result<()> {
    AccessType::mem_write_typed::<i32>(
        apex_base + G_OFFSETS.in_attack + 0x8,
        &force_attack_state,
        0,
    )
    .with_priority(50)
    .dispatch(mem)
    .await?
    .await?
}
