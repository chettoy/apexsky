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
use crate::mem::{
    memflow_impl::MemflowOs, memprocfs_impl::MemProcFsOs, ApexMem, MemOs, MemProc, MemProcImpl,
    ProcessStatus,
};
use crate::{press_to_exit, SharedState, TreasureClue};

use super::aim::{AimKeyStatus, AimbotAction, PreSelectedTarget};

pub trait MemAccess {
    fn apex_mem_baseaddr(&mut self) -> u64;
    fn apex_mem_read<T: dataview::Pod + Sized + Default>(
        &mut self,
        offset: u64,
    ) -> anyhow::Result<T>;
    fn apex_mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        offset: u64,
        data: &T,
    ) -> anyhow::Result<()>;
}

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
    aim_key_tx: watch::Sender<AimKeyStatus>,
    aim_select_tx: watch::Sender<Vec<PreSelectedTarget>>,
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

    tracing::debug!("{}", s!("task start"));

    let mut connector = s!("dma").to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == s!("kvm") {
            connector = s!("kvm").to_string();
        } else if args[1] == s!("no-kvm")
            || args[1] == s!("nokvm")
            || args[1] == s!("nodma")
            || args[1] == s!("linux")
            || args[1] == s!("native")
        {
            connector = s!("native").to_string();
        }
    }
    // create OS instance
    let mut mem_os: Box<dyn MemOs> = if connector == s!("dma") {
        match MemProcFsOs::new(&connector) {
            Ok(os) => Box::new(os),
            Err(e) => {
                tracing::error!(?e, "{}", s!("open_os"));
                press_to_exit();
                return Ok(());
            }
        }
    } else {
        match MemflowOs::new(&connector) {
            Ok(os) => Box::new(os),
            Err(e) => {
                tracing::error!(?e, "{}", s!("open_os"));
                press_to_exit();
                return Ok(());
            }
        }
    };
    fn find_game_process(mem_os: &mut Box<dyn MemOs>) -> Option<MemProcImpl<'_>> {
        tracing::warn!(parent: None, "{}", s!("Searching for apex process..."));
        mem_os
            .open_proc(s!("r5apex.exe").to_string())
            .map(Some)
            .unwrap_or_else(|e| {
                tracing::trace!(?e, "{}", s!("open_proc"));
                None
            })
    }

    while *active.borrow_and_update() {
        sleep(Duration::from_secs(2)).await;
        let Some(mut mem) = find_game_process(&mut mem_os) else {
            shared_state.write().game_attached = false;
            continue;
        };
        if mem.check_proc_status() != ProcessStatus::FoundReady {
            shared_state.write().game_attached = false;
            continue;
        }

        if !shared_state.read().game_attached {
            println!("{}", s!("Apex process found"));
            println!("{}{:x}", s!("Base: 0x"), mem.get_proc_baseaddr());

            tracing::debug!("{}", s!("speed_test"));
            mem.speed_test();
            println!("{}", s!("Press enter to continue.."));
            tracing::debug!("{}", s!("press to continue"));
            let _ = std::io::stdin().read_line(&mut String::new());

            shared_state.write().game_attached = true;
        }

        while *active.borrow_and_update() {
            sleep(Duration::from_millis(2)).await; // don't change xD

            let loop_duration = start_instant.elapsed().as_millis();
            start_instant = Instant::now();

            if mem.check_proc_status() != ProcessStatus::FoundReady {
                shared_state.write().game_attached = false;
                break;
            }

            /* Hot Variables Update Begin */

            actions_tick += 1;
            let verbose = actions_tick % 1_000 == 0;

            // Tick game state
            let tick_duration = {
                let a = ApexMem::new(&mut mem);
                let tick_start = Instant::now();
                apexdream.tick_state(a);
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
                    if pl.eadp_uid == 0 {
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

            trace_span!("Perform aimbot actions").in_scope(|| match aim_action_rx.try_recv() {
                Ok(action) => {
                    if player_ready && ENABLE_MEM_AIM {
                        if let Some(delta) = action.shift_angles {
                            if let Some(lplayer) = &shared_state.read().local_player {
                                let lplayer = lplayer.get_entity();
                                let ptr = lplayer.entity_ptr.into_raw();
                                let view_angles = match mem
                                    .apex_mem_read::<[f32; 3]>(ptr + G_OFFSETS.player_viewangles)
                                {
                                    Ok(v) => v,
                                    Err(e) => {
                                        tracing::warn!(%e, "{}", s!("err read viewangles"));
                                        return;
                                    }
                                };
                                let mut update_angles = math::add(view_angles, delta);
                                if update_angles[0].abs() > 360.0
                                    || update_angles[1].abs() > 360.0
                                    || update_angles[2].abs() > 1.0
                                {
                                    tracing::warn!(?update_angles, "{}", s!("got invalid angles"));
                                }
                                normalize_angles(&mut update_angles);
                                mem.apex_mem_write::<[f32; 3]>(
                                    ptr + G_OFFSETS.player_viewangles,
                                    &update_angles,
                                )
                                .unwrap_or_else(|e| {
                                    tracing::warn!(%e, "{}", s!("err write viewangles"));
                                    return;
                                });
                            } else {
                                tracing::warn!(
                                    ?action,
                                    "{}",
                                    s!("UNREACHABLE: invalid localplayer")
                                );
                            }
                        }
                        if let Some(trigger) = action.force_attack {
                            if trigger != apex_state.in_attack_state() {
                                let force_attack = if trigger { 5 } else { 4 };
                                let base = mem.apex_mem_baseaddr();
                                mem.apex_mem_write::<i32>(
                                    base + G_OFFSETS.in_attack + 0x8,
                                    &force_attack,
                                )
                                .unwrap_or_else(|e| {
                                    tracing::warn!(%e, "{}", s!("err write force_attack"));
                                    return;
                                });
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
            });

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

                let span = trace_span!("Update entities hot data");

                let shared_state = shared_state.clone();
                let player_entities: Vec<_> = apex_state.players().cloned().collect();
                let local_player_ptr = apex_state.client.local_player_ptr;

                tokio::spawn(
                    async move {
                        let mut players: HashMap<u64, GamePlayer> =
                            shared_state.read().players.clone();
                        let mut aim_entities = shared_state.read().aim_entities.clone();

                        player_entities.iter().for_each(|entity| {
                            if entity.eadp_uid == 0 {
                                return;
                            }
                            let entity_ptr = entity.entity_ptr.into_raw();
                            aim_entities.insert(entity_ptr, Arc::new(entity.clone()));
                            if let Some(player) = players.get_mut(&entity_ptr) {
                                player.update_buf_hotdata(&entity);
                            }
                        });

                        let local_player: Option<GamePlayer> =
                            players.get(&local_player_ptr).cloned();

                        let mut state_wlock = shared_state.write();
                        state_wlock.local_player = local_player;
                        state_wlock.players = players;
                        state_wlock.aim_entities = aim_entities;
                    }
                    .instrument(span),
                );
            }

            /* Hot Variables Update End */

            if actions_tick % 15 == 0 {
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

            if !world_ready {
                continue;
            }

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

                    apex_state.players().for_each(|entity| {
                        // FIXME: skip wrong entity
                        if entity.eadp_uid == 0 {
                            return;
                        }
                        let game_player = GamePlayer::new(
                            entity.clone(),
                            apex_state,
                            &mut G_STATE.lock().unwrap().config,
                        );
                        players.insert(entity.entity_ptr.into_raw(), game_player);
                        aim_entities.insert(entity.entity_ptr.into_raw(), Arc::new(entity.clone()));
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

            trace_span!("Update loots").in_scope(||{
                if player_ready {
                    let Some(local_position) = shared_state
                        .read()
                        .local_player
                        .as_ref()
                        .map(|l| arr1(&l.get_entity().origin))
                    else {
                        tracing::error!(?apex_state.client, ?shared_state, "{}", s!("UNREACHABLE: localplayer=None"));
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
                            let item_namelist = 
                            apex_state.entities_as::<LootEntity>().map(|entity| {
                                (entity.custom_script_int, entity.model_name.string.clone())
                            }).collect::<HashSet<(i32, String)>>();
                            let mut item_namelist: Vec<_> = item_namelist.into_iter().collect();
                            item_namelist.sort_by(|(a, _), (b, _)| a.cmp(b));
                            tracing::info!(?item_namelist, "{}", s!("items sorted"));
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
                aim_select_tx.send(
                    if player_ready {
                        // Iterate over all targetable entities
                        let mut aim_targets: Vec<PreSelectedTarget> = {
                            let state = shared_state.read();
                            if let Some(lplayer) = state.local_player.as_ref() {
                                state
                                    .aim_entities
                                    .values()
                                    .filter_map(|entity| {
                                        process_player(lplayer, entity.as_ref(), &state, &g_settings)
                                        // .unwrap_or_else(|e|{
                                        //     tracing::error!(%e, ?e, ?entity, "{}", s!("error process player"));
                                        //     None
                                        // })
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
                    }).unwrap_or_else(|e| {
                        tracing::error!(%e, ?aim_select_tx, "{}", s!("send aim targets"));
                    }
                );
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
                        let base = mem.apex_mem_baseaddr();
                        let glow_fix_i32 = mem.apex_mem_read::<i32>(base + OFFSET_GLOW_FIX)?;
                        let glow_fix_u8 = mem.apex_mem_read::<u8>(base + OFFSET_GLOW_FIX)?;
                        tracing::trace!(glow_fix_i32, glow_fix_u8);
                    }
                    if (g_settings.player_glow || g_settings.item_glow) && player_ready {
                        match inject_highlight(apex_state.client.framecount, &g_settings, &mut mem)
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
                    for target in aim_select_rx.borrow_and_update().iter() {
                        let target_ptr = target.entity_ptr;
                        let highlight_context_id =
                            player_glow(target, apex_state.client.framecount, &g_settings);
                        mem.apex_mem_write::<u8>(
                            target_ptr + OFFSET_GLOW_CONTEXT_ID,
                            &highlight_context_id,
                        )?;
                        mem.apex_mem_write::<i32>(target_ptr + OFFSET_GLOW_VISIBLE_TYPE, &2)?;
                        mem.apex_mem_write::<f32>(target_ptr + OFFSET_GLOW_DISTANCE, &8.0E+4)?;
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
                    for &(ptr, ctx_id) in items_glow_rx.borrow_and_update().iter() {
                        mem.apex_mem_write::<u8>(ptr + OFFSET_GLOW_CONTEXT_ID, &ctx_id)?;
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

    Some(PreSelectedTarget {
        fov: calculate_target_fov(local_player.get_entity(), target_entity),
        distance,
        is_visible: target_entity.is_visible(),
        is_knocked: target_entity.is_knocked(),
        health_points: { target_entity.get_shield_health() + target_entity.get_health() },
        love_status: {
            if !target_entity.is_player() {
                if g_settings.yuan_p {
                    LoveStatus::Love
                } else {
                    LoveStatus::Normal
                }
            } else {
                let Some(target_player) = state.players.get(&target_ptr) else {
                    tracing::error!(?target_ptr, "{}", s!("UNREACHABLE"));
                    return None;
                };
                target_player
                    .get_buf()
                    .love_state
                    .try_into()
                    .unwrap_or_else(|_| {
                        tracing::error!(love_state = target_player.get_buf().love_state, player_buf = ?target_player.get_buf());
                        LoveStatus::Normal
                    })
            }
        },
        entity_ptr: target_ptr,
    })
}

#[instrument(skip_all)]
fn player_glow(target: &PreSelectedTarget, frame_count: i32, g_settings: &Settings) -> u8 {
    let mut setting_index = {
        if target.is_knocked {
            HIGHLIGHT_PLAYER_KNOCKED
        } else if target.is_visible {
            HIGHLIGHT_PLAYER_VISIBLE
        } else {
            if g_settings.player_glow_armor_color {
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
    if g_settings.player_glow_love_user {
        let frame_frag = frame_count / g_settings.game_fps as i32;
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

    setting_index
}

#[instrument(skip_all)]
fn inject_highlight(
    frame_count: i32,
    g_settings: &Settings,
    mem: &mut MemProcImpl,
) -> anyhow::Result<()> {
    let bits_loot = HighlightBits::new(g_settings.loot_filled, 125, 64, 7, true, false);
    let bits_box = HighlightBits::new(0, 125, 64, 7, true, false);
    let bits_player_fill = HighlightBits::new(
        g_settings.player_glow_inside_value,
        169,
        g_settings.player_glow_outline_size,
        7,
        true,
        false,
    );
    let bits_player_outline =
        HighlightBits::new(0, 169, g_settings.player_glow_outline_size, 7, true, false);

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

    let base = mem.apex_mem_baseaddr();
    let highlight_settings_ptr = mem.apex_mem_read::<u64>(base + OFFSET_HIGHLIGHT_SETTINGS)?;
    for (context_id, bits, color) in highlight_settings_inject {
        let context_offset = highlight_settings_ptr + 0x34 * context_id as u64;
        mem.apex_mem_write::<HighlightBits>(context_offset, bits)
            .context(format!("{:?}", context_id))?;
        mem.apex_mem_write::<[f32; 3]>(context_offset + 4, &color)
            .context(format!("{:?}", context_id))?;
    }
    tracing::trace!(highlight_settings_ptr);
    mem.apex_mem_write::<i32>(base + OFFSET_GLOW_FIX, &1)
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
