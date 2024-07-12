use anyhow::Context;
use apexsky::{
    aimbot::{calc_angle, calc_fov, AimEntity},
    config::Settings,
    global_state::G_STATE,
    love_players::LoveStatus,
    offsets::G_OFFSETS,
};
use apexsky_dmalib::access::{
    AccessType, MemApi, PendingAccessRequest, PendingMemRead, PendingMemWrite,
};
use apexsky_proto::pb::apexlegends::{AimKeyState, AimTargetInfo, SpectatorInfo, TreasureClue};
use ndarray::arr1;
//use obfstr::obfstr as s;
use apexsky::noobfstr as s;
use std::mem::size_of;
use std::{collections::HashMap, sync::Arc, time::Duration};
use std::{collections::HashSet, sync::atomic::Ordering};
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Instant};

use crate::{
    apexdream::state::entities::Entity,
    game::player::QuickLooting,
    usermod_thr::{ActionTickData, UserModEvent},
    workers::items::LootInt,
    SharedStateWrapper,
};
use crate::{
    apexdream::state::GameState,
    game::{data::*, player::GamePlayer},
};
use crate::{
    apexdream::{
        base::math,
        sdk::HighlightBits,
        state::entities::{BaseNPCEntity, LootEntity},
    },
    PRINT_LATENCY,
};

#[tracing::instrument(skip_all)]
pub async fn actions_loop(
    mut active: watch::Receiver<bool>,
    shared_state: SharedStateWrapper,
    access_tx: MemApi,
    aim_key_tx: watch::Sender<AimKeyState>,
    aim_select_tx: watch::Sender<Vec<AimTargetInfo>>,
    usermod_event_tx: mpsc::UnboundedSender<UserModEvent>,
    mut aim_select_rx: watch::Receiver<Vec<AimTargetInfo>>,
    mut items_glow_rx: watch::Receiver<Vec<(u64, u8)>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let usermod_send_event = |event: UserModEvent| {
        if let Err(e) = usermod_event_tx.send(event) {
            tracing::error!(%e, "{}", s!("usermod_send_event"));
        };
    };

    let mut apexdream = crate::apexdream::Instance::new();
    let mut start_instant = Instant::now();
    let mut fps_checkpoint_instant = Instant::now();
    let mut last_checkpoint_frame: i32 = 0;
    let mut actions_tick: i64 = -1;
    let mut log_items: usize = 0;
    let mut world_ready: bool;
    let mut player_ready: bool;

    while *active.borrow_and_update() {
        sleep(Duration::from_secs(2)).await;

        match AccessType::mem_baseaddr()
            .with_priority(100)
            .dispatch(&access_tx)
            .await?
            .await?
        {
            Some(baseaddr) => {
                shared_state
                    .game_baseaddr
                    .store(baseaddr, Ordering::Release);
                usermod_send_event(UserModEvent::GameAttached);
            }
            None => {
                shared_state.game_baseaddr.store(0, Ordering::Release);
                continue;
            }
        }

        while *active.borrow_and_update() {
            sleep(Duration::from_millis(2)).await; // don't change xD

            let loop_duration = start_instant.elapsed().as_millis().try_into()?;
            start_instant = Instant::now();

            let apex_base = match AccessType::mem_baseaddr()
                .with_priority(100)
                .dispatch(&access_tx)
                .await?
                .await?
            {
                Some(addr) => {
                    shared_state.game_baseaddr.store(addr, Ordering::Release);
                    addr
                }
                None => {
                    shared_state.game_baseaddr.store(0, Ordering::Release);
                    usermod_send_event(UserModEvent::GameUnattached);
                    break;
                }
            };
            let mem = &access_tx;

            /* Hot Variables Update Begin */

            actions_tick += 1;
            let verbose = actions_tick % 1_000 == 0;

            // Tick game state
            let (tick_duration, apex_state, is_newly_connected) = {
                (
                    {
                        let mut api = crate::apexdream::api::Api {
                            apex_base: apex_base.into(),
                            mem_access: mem.clone(),
                        };
                        let tick_start = Instant::now();
                        apexdream.tick_state(&mut api).await;
                        tick_start.elapsed().as_millis().try_into()?
                    },
                    apexdream.get_state(),
                    apexdream.is_newly_connected(),
                )
            };

            if verbose {
                tracing::trace!(loop_duration, tick_duration, ?apex_state.client, "{}", s!("game state update"));
            }

            // // Update spectator checker
            // if actions_tick % 15 == 0 && player_ready {
            //     let lplayer_ptr = apex_state.client.local_player_ptr;
            //     // Init spectator checker
            //     if prev_lplayer_ptr != lplayer_ptr {
            //         init_spec_checker(lplayer_ptr);
            //         prev_lplayer_ptr = lplayer_ptr;
            //     }
            //     // Update yaw to spec checker
            //     apex_state.players().for_each(|pl| {
            //         if pl.eadp_uid == 0 || pl.team_num < 0 || pl.team_num > 50 {
            //             return;
            //         }
            //         apexsky::tick_yaw(pl.entity_ptr.into_raw(), pl.yaw);
            //     });
            // }

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

            // Update client state
            {
                world_ready = apex_state.is_in_game() && apex_state.local_player().is_some();
                player_ready = world_ready
                    && shared_state
                        .players
                        .read()
                        .get(&shared_state.local_player_ptr.load(Ordering::Acquire))
                        .is_some();

                shared_state
                    .world_ready
                    .store(world_ready, Ordering::Release);
                shared_state
                    .frame_count
                    .store(apex_state.client.framecount, Ordering::Release);
                shared_state
                    .tick_num
                    .store(actions_tick.try_into().unwrap(), Ordering::Release);
                shared_state
                    .tick_duration
                    .store(tick_duration, Ordering::Release);
                shared_state
                    .actions_duration
                    .store(loop_duration, Ordering::Release);

                {
                    let mut lock = shared_state.game_fps.lock();
                    if let Some(fps_update) = game_fps_update {
                        tracing::trace!(fps_update);
                        *lock = fps_update;
                    } else if *lock < f32::EPSILON {
                        *lock = g_settings.game_fps;
                    }
                }
                *shared_state.update_time.lock() = apex_state.time;
                *shared_state.view_matrix.lock() = apex_state.client.view_matrix;

                if PRINT_LATENCY {
                    println!(
                        "{}{:.1}",
                        s!("actions data latency "),
                        apexsky::aimbot::get_unix_timestamp_in_millis() as f64
                            - apex_state.time * 1000.0
                    );
                }

                if verbose {
                    tracing::trace!(?shared_state);
                }
            }

            // Send key status to aimbot worker
            aim_key_tx
                .send(AimKeyState {
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
                    triggerbot_hotkey: if apex_state.is_button_down(g_settings.trigger_bot_hot_key)
                    {
                        g_settings.trigger_bot_hot_key
                    } else {
                        0
                    },
                    attack_state: match apex_state.buttons.in_attack.state.try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            tracing::error!(?apex_state.buttons.in_attack.state,
                                            "{}", s!("err convert in_attack to i32"));
                            continue;
                        }
                    },
                    quick_looting_hotkey: if apex_state
                        .is_button_down(g_settings.quick_looting_hot_key)
                    {
                        g_settings.quick_looting_hot_key
                    } else {
                        0
                    },
                })
                .unwrap_or_else(|e| {
                    tracing::error!(%e, ?aim_key_tx, "{}", s!("send key status"));
                });

            // Update entities
            if world_ready {
                let mut players = HashMap::new();
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
                    let entity_ptr = pl.entity_ptr.into_raw();
                    let player = Arc::new(game_player);
                    players.insert(entity_ptr, player);
                });

                let local_player_ptr: u64 = players
                    .contains_key(&apex_state.client.local_player_ptr)
                    .then_some(apex_state.client.local_player_ptr)
                    .unwrap_or(0);
                let view_player_ptr: u64 = apex_state
                    .local_player()
                    .map(|entity| entity.get_entity_ptr())
                    .unwrap_or(0);

                if actions_tick % 2 != 0 {
                    let mut aim_entities = shared_state.aim_entities.write();
                    for (k, v) in &players {
                        aim_entities.insert(*k, v.clone());
                    }
                    *shared_state.players.write() = players;
                } else {
                    let mut npcs: HashMap<u64, Arc<dyn AimEntity>> = HashMap::new();
                    apex_state
                        .entities_as::<BaseNPCEntity>()
                        .for_each(|entity| {
                            npcs.insert(entity.entity_ptr.into_raw(), Arc::new(entity.clone()));
                        });

                    let mut treasure_clues: HashMap<u64, TreasureClue> = HashMap::new();
                    if let Some(local_position) = players
                        .get(&local_player_ptr)
                        .map(|l| arr1(&l.get_entity().origin))
                    {
                        apex_state.entities_as::<LootEntity>().for_each(|entity| {
                            let distance = (arr1(&entity.origin) - &local_position)
                                .mapv(|x| x * x)
                                .sum()
                                .sqrt();
                            let clue = TreasureClue {
                                entity_handle: entity.entity_ptr.into_raw(),
                                item_id: entity.custom_script_int,
                                custom_item_id: (entity.custom_script_int as u64
                                    | (entity.survival_property as u64) << 32),
                                position: Some(entity.origin.into()),
                                distance,
                            };
                            treasure_clues.insert(clue.entity_handle, clue);
                        });
                    }
                    let loot_count = treasure_clues.len();
                    tracing::trace!(loot_count, "{}", s!("loots updated"));

                    let mut aim_entities: HashMap<u64, Arc<dyn AimEntity>> = HashMap::new();
                    for (k, v) in &players {
                        aim_entities.insert(*k, v.clone());
                    }
                    for (k, v) in &npcs {
                        aim_entities.insert(*k, v.clone());
                    }
                    for (k, v) in &treasure_clues {
                        if v.distance > 40.0 * 3.0 {
                            continue;
                        }
                        aim_entities.insert(*k, Arc::new(QuickLooting(v.clone())));
                    }
                    let entity_count = aim_entities.len();
                    tracing::trace!(entity_count, "{}", s!("AimEntities updated"));

                    *shared_state.aim_entities.write() = aim_entities;
                    *shared_state.npcs.write() = npcs;
                    *shared_state.players.write() = players;
                    *shared_state.treasure_clues.write() = treasure_clues;
                }

                shared_state
                    .local_player_ptr
                    .store(local_player_ptr, Ordering::Release);
                shared_state
                    .view_player_ptr
                    .store(view_player_ptr, Ordering::Release);
            } else {
                shared_state.local_player_ptr.store(0, Ordering::Release);
                shared_state.view_player_ptr.store(0, Ordering::Release);
                shared_state.aim_entities.write().clear();
                shared_state.npcs.write().clear();
                shared_state.players.write().clear();
                shared_state.treasure_clues.write().clear();
            }

            // Log WeaponId
            if is_newly_connected {
                tracing::info!(?apex_state.string_tables.weapon_names);
                if apex_state.is_firing_range()
                    && apex_state.string_tables.weapon_names.len() != WEAPON_LIST.len()
                {
                    match (|| -> anyhow::Result<()> {
                        use std::fs;
                        use std::io::Write;

                        let weapons_json =
                            serde_json::to_string(&apex_state.string_tables.weapon_names)?;
                        let path = std::env::current_dir()?.join(s!("updated_weapon.json"));
                        let mut json_file = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(path)?;
                        write!(json_file, "{}", weapons_json)?;
                        Ok(())
                    })() {
                        Ok(()) => {
                            tracing::info!("{}", s!("Exported to updated_item.json"));
                        }
                        Err(e) => {
                            tracing::warn!(%e, ?e);
                        }
                    }
                }
            }

            // Log ItemId
            if actions_tick % 40 == 0 && apex_state.is_firing_range() {
                let loot_count = shared_state.treasure_clues.read().len();
                if loot_count == 0 {
                    tracing::debug!("{}", s!("wait items"));
                } else if loot_count > log_items {
                    // create new item list
                    let item_namelist = apex_state
                        .entities_as::<LootEntity>()
                        .map(|entity| LootInt {
                            int: entity.custom_script_int,
                            model: entity.model_name.string.clone(),
                        })
                        .collect::<HashSet<LootInt>>();
                    let mut item_namelist: Vec<LootInt> = item_namelist.into_iter().collect();
                    item_namelist.sort_by(|a, b| a.int.cmp(&b.int));
                    tracing::info!("{loot_count}{}", s!(" items sorted"));
                    log_items = loot_count;

                    match crate::workers::items::export_new_items(item_namelist) {
                        Ok(()) => {
                            tracing::info!("{}", s!("Exported to updated_item.json"));
                        }
                        Err(e) => {
                            let treasure_clues = shared_state.treasure_clues.read().clone();
                            tracing::warn!(%e, ?treasure_clues, "{loot_count}{}", s!(" items sorted"))
                        }
                    }
                }
            }

            usermod_send_event(UserModEvent::ActionTick(ActionTickData {
                input_state: apex_state.input_system.button_state,
            }));

            /* Hot Variables Update End */

            if actions_tick % 30_000 == 0 {
                actions_tick = 0;
            }
            if actions_tick % 2 != 0 {
                // at least 30ms // don't change xD
                continue;
            }

            /* Cold Variables Update Start */

            tracing::trace_span!("Update state in global settings").in_scope(|| {
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

            // Send aim targets
            aim_select_tx
                .send(collect_aim_targets(
                    shared_state.clone(),
                    apex_state,
                    &g_settings,
                    items_glow_rx.clone(),
                ))
                .unwrap_or_else(|e| {
                    tracing::error!(%e, ?aim_select_tx, "{}", s!("send aim targets"));
                });

            if actions_tick % 4 != 0 {
                tracing::trace_span!("Spectator check").in_scope(|| {
                    if player_ready {
                        let Some(lplayer_ptr) = shared_state.get_local_player_ptr() else {
                            return;
                        };
                        let players = shared_state.players.read().clone();
                        let Some(local_player) = players.get(&lplayer_ptr) else {
                            return;
                        };
                        let lplayer_team = local_player.get_buf().team_num;
                        let alter_local_team =
                            shared_state.map_testing_local_team.load(Ordering::Acquire);
                        let tdm_toggle = g_settings.tdm_toggle;
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
                            .filter_map(|(&player_ptr, player)| {
                                apex_state
                                    .get_observer_target(player.get_entity().get_info().handle)
                                    .and_then(|observer_target| {
                                        let target_ptr = observer_target.entity_ptr.into_raw();
                                        if player_ptr == lplayer_ptr {
                                            Some(target_ptr)
                                        } else if target_ptr == lplayer_ptr {
                                            Some(player_ptr)
                                        } else {
                                            None
                                        }
                                    })
                                    .and_then(|item_ptr| {
                                        let player_buf = players.get(&item_ptr)?.get_buf();
                                        Some(SpectatorInfo {
                                            ptr: item_ptr,
                                            name: player_buf.player_name.clone(),
                                            is_teammate: is_teammate(player_buf.team_num),
                                            love_status: player_buf
                                                .love_status
                                                .try_into()
                                                .unwrap_or_else(|_| {
                                                    tracing::error!(
                                                        love_state = player_buf.love_status,
                                                        ?player_buf
                                                    );
                                                    LoveStatus::Normal
                                                })
                                                as i32,
                                        })
                                    })
                            })
                            .collect();

                        // Update spectator namelist
                        let (allied_spectators, spectators): (Vec<_>, Vec<_>) =
                            tmp_specs.into_iter().partition(|info| info.is_teammate);

                        *shared_state.spectator_list.lock() = (allied_spectators, spectators);
                        *shared_state.teammates.lock() = teammates;
                    } else {
                        {
                            let mut lock = shared_state.spectator_list.lock();
                            lock.0.clear();
                            lock.1.clear();
                        }
                        //shared_state.teammates.lock().clear();
                    }
                });

                if player_ready {
                    // Inject highlight settings
                    let highlight_injected = {
                        let mut injected = shared_state.highlight_injected.load(Ordering::Acquire);
                        if (g_settings.player_glow || g_settings.item_glow) && player_ready {
                            match inject_highlight(mem, apex_state.client.framecount, &g_settings)
                                .await
                            {
                                Ok(_) => {
                                    shared_state
                                        .highlight_injected
                                        .store(true, Ordering::Release);
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
                        for (
                            req_write_glow_id,
                            req_write_glow_type,
                            req_write_glow_dist,
                            req_write_glow_fix,
                        ) in reqs
                        {
                            let (r0, r1, r2, r3) = tokio::try_join!(
                                req_write_glow_id.with_priority(0).dispatch(mem),
                                req_write_glow_type.with_priority(0).dispatch(mem),
                                req_write_glow_dist.with_priority(0).dispatch(mem),
                                req_write_glow_fix.with_priority(0).dispatch(mem),
                            )?;
                            r0.spawn_err_handler();
                            r1.spawn_err_handler();
                            r2.spawn_err_handler();
                            r3.spawn_err_handler();
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

                    // println!("in_use state {}", apex_state.buttons.in_use.state);
                }
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

#[tracing::instrument(skip_all)]
fn collect_aim_targets(
    state: SharedStateWrapper,
    apex_state: &GameState,
    g_settings: &Settings,
    items_glow_rx: watch::Receiver<Vec<(u64, u8)>>,
) -> Vec<AimTargetInfo> {
    if !state.is_world_ready() {
        return vec![];
    }

    let quick_looting = apex_state.is_button_down(g_settings.quick_looting_hot_key);

    let aim_entities = state.aim_entities.read().clone();
    let players = state.players.read().clone();
    let treasure_clues = if quick_looting {
        Some(state.treasure_clues.read().clone())
    } else {
        None
    };

    let Some(lplayer) = state
        .get_local_player_ptr()
        .and_then(|ptr| players.get(&ptr))
    else {
        return vec![];
    };

    let alter_local_team_id = state.map_testing_local_team.load(Ordering::Acquire);

    // Iterate over all targetable entities
    let mut aim_targets: Vec<AimTargetInfo> = aim_entities
        .values()
        .filter_map(|entity| {
            process_player(
                &lplayer,
                entity.as_ref(),
                &players,
                alter_local_team_id,
                g_settings,
            )
        })
        .collect();

    if let Some(loots) = treasure_clues {
        aim_targets.extend(items_glow_rx.borrow().iter().filter_map(|(ptr, _)| {
            let clue = loots.get(ptr)?;
            let distance = clue.distance;
            (distance < 40.0 * 2.5).then_some({
                let target_entity = QuickLooting(clue.clone());
                let fov = calculate_target_fov(lplayer.as_ref(), &target_entity);
                AimTargetInfo {
                    fov,
                    distance,
                    is_visible: target_entity.is_visible(),
                    is_knocked: target_entity.is_knocked(),
                    health_points: 1,
                    love_status: LoveStatus::Normal as i32,
                    is_kill_leader: false,
                    entity_ptr: target_entity.get_entity_ptr(),
                    is_npc: false,
                    is_loot: target_entity.is_loot(),
                }
            })
        }));
    }

    aim_targets.sort_by(|a, b| {
        a.distance.partial_cmp(&b.distance).unwrap_or_else(|| {
            tracing::error!(?a, ?b, "{}", s!("sort"));
            panic!()
        })
    });
    aim_targets
}

#[tracing::instrument(skip_all)]
fn process_player(
    local_player: &GamePlayer,
    target_entity: &dyn AimEntity,
    players: &HashMap<u64, Arc<GamePlayer>>,
    alter_local_team_id: i32,
    g_settings: &Settings,
) -> Option<AimTargetInfo> {
    if target_entity.is_loot() {
        return None;
    }

    let lplayer_ptr = local_player.get_entity().get_entity_ptr();
    let target_ptr = target_entity.get_entity_ptr();

    let entity_team = target_entity.get_team_num();
    let local_team = local_player.get_buf().team_num;

    let is_teammate = teammate_check(
        entity_team,
        local_team,
        alter_local_team_id,
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
    // Excluding targets that are too far or too close
    let distance = {
        let target_pos = target_entity.get_position();
        let local_pos = local_player.get_entity().get_position();
        math::dist(target_pos, local_pos)
    };

    if !distance.is_normal() || distance > g_settings.max_dist || distance < 10.0 {
        return None;
    }

    // Calc FOV
    let fov = calculate_target_fov(local_player.get_entity(), target_entity);
    if !fov.is_finite() {
        // inf/neg_inf/nan
        return None;
    }

    let target_player = target_entity.is_player().then(|| {
        let Some(p) = players.get(&target_ptr) else {
            tracing::error!(?target_ptr, "{}", s!("UNREACHABLE"));
            panic!();
        };
        p
    });

    Some(AimTargetInfo {
        fov,
        distance,
        is_visible: target_entity.is_visible(),
        is_knocked: target_entity.is_knocked(),
        health_points: { target_entity.get_shield_health() + target_entity.get_health() },
        love_status: {
            if let Some(target_player) = target_player {
                target_player.get_buf().love_status.try_into()
                .unwrap_or_else(|_| {
                    tracing::error!(love_state = target_player.get_buf().love_status, player_buf = ?target_player.get_buf());
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
        } as i32,
        is_kill_leader: {
            if let Some(target_player) = target_player {
                GamePlayer::is_kill_leader(target_player.get_buf())
            } else {
                false
            }
        },
        entity_ptr: target_ptr,
        is_npc: target_player.is_none(),
        is_loot: false,
    })
}

#[tracing::instrument(skip_all)]
fn player_glow(
    target: &AimTargetInfo,
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
                match target.health_points {
                    0..=100 => HIGHLIGHT_PLAYER_ORANGE,
                    101..=150 => HIGHLIGHT_PLAYER_WHITE,
                    151..=175 => HIGHLIGHT_PLAYER_BLUE,
                    176..=200 => HIGHLIGHT_PLAYER_PURPLE,
                    201..=225 => HIGHLIGHT_PLAYER_RED,
                    _ => HIGHLIGHT_PLAYER_BLACK,
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
            match target.love_status.try_into().unwrap_or(LoveStatus::Normal) {
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

#[tracing::instrument(skip_all)]
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
    let mut futs_write_highlight_settings: Vec<tokio::task::JoinHandle<anyhow::Result<()>>> =
        Vec::with_capacity(highlight_settings_inject.len());
    for (context_id, &bits, color) in highlight_settings_inject {
        let context_offset = highlight_settings_ptr + 0x34 * context_id as u64;
        futs_write_highlight_settings.push(tokio::spawn({
            let mem = mem.clone();
            async move {
                let (r1, r2) = tokio::try_join!(
                    AccessType::mem_write_typed::<HighlightBits>(context_offset, &bits, 0)
                        .dispatch(&mem)
                        .await?,
                    AccessType::mem_write_typed::<[f32; 3]>(context_offset + 4, &color, 0)
                        .dispatch(&mem)
                        .await?,
                )?;
                r1.context(format!("{:?}", context_id))?;
                r2.context(format!("{:?}", context_id))?;
                Ok(())
            }
        }));
    }
    for task in futs_write_highlight_settings {
        task.await??;
    }
    tracing::trace!(highlight_settings_ptr);

    Ok(())
}

#[tracing::instrument(skip_all)]
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
        tracing::trace!(
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
