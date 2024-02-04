use apexsky::{
    aimbot::{calc_angle, calc_fov, normalize_angles, AimEntity},
    apexdream::{
        base::math,
        sdk::HighlightBits,
        state::entities::{BaseNPCEntity, LootEntity, PlayerEntity},
    },
    config::Settings,
    games::apex::{data::*, player::GamePlayer},
    global_state::G_STATE,
    init_spec_checker, is_spec,
    love_players::LoveStatus,
    mem::{
        memflow_impl::MemflowOs, memprocfs_impl::MemProcFsOs, ApexMem, MemOs, MemProc, MemProcImpl,
        ProcessStatus,
    },
    offsets::G_OFFSETS,
};
use ndarray::arr1;
use obfstr::obfstr as s;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{self, error::TryRecvError},
        watch, Mutex,
    },
    time::{sleep, Instant},
};
use tracing::{instrument, trace};

use crate::{press_to_exit, SharedState, TreasureClue};

use super::aim::{AimKeyStatus, AimbotAction, PreSelectedTarget};

pub trait ContextForActions {}

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

const ENABLE_MEM_AIM: bool = true;

#[instrument(skip_all)]
pub async fn actions_loop(
    mut active: watch::Receiver<bool>,
    shared_state: Arc<Mutex<SharedState>>,
    aim_key_tx: watch::Sender<AimKeyStatus>,
    aim_select_tx: watch::Sender<Vec<PreSelectedTarget>>,
    mut aim_action_rx: mpsc::Receiver<AimbotAction>,
) -> anyhow::Result<()> {
    let mut apexdream = apexsky::apexdream::Instance::new();
    let mut start_instant = Instant::now();
    let mut fps_checkpoint_instant = Instant::now();
    let mut last_checkpoint_frame: i32 = 0;
    let mut prev_lplayer_ptr: u64 = 0;
    let mut actions_tick: i64 = -1;
    let mut log_items: usize = 0;
    let mut world_ready: bool;

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
        tracing::info!(parent: None, "{}", s!("Searching for apex process..."));
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
            shared_state.lock().await.game_attached = false;
            continue;
        };
        if mem.check_proc_status() != ProcessStatus::FoundReady {
            shared_state.lock().await.game_attached = false;
            continue;
        }

        if !shared_state.lock().await.game_attached {
            println!("{}", s!("Apex process found"));
            println!("{}{:x}", s!("Base: 0x"), mem.get_proc_baseaddr());

            tracing::debug!("{}", s!("speed_test"));
            mem.speed_test();
            println!("{}", s!("Press enter to continue.."));
            tracing::debug!("{}", s!("press to continue"));
            let _ = std::io::stdin().read_line(&mut String::new());

            shared_state.lock().await.game_attached = true;
        }

        while *active.borrow_and_update() {
            sleep(Duration::from_millis(2)).await; // don't change xD

            let loop_duration = start_instant.elapsed().as_millis();
            start_instant = Instant::now();

            if mem.check_proc_status() != ProcessStatus::FoundReady {
                shared_state.lock().await.game_attached = false;
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

                let mut wlock = shared_state.lock().await;

                wlock.world_ready = world_ready;

                wlock.frame_count = apex_state.client.framecount;
                wlock.view_matrix = apex_state.client.view_matrix;

                if !world_ready {
                    wlock.spectator_count = 0;
                    wlock.allied_spectator_count = 0;
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
            }

            // Log WeaponId and ItemId
            if apexdream.is_newly_connected() {
                tracing::debug!(?apex_state.string_tables.weapon_names);
            }
            if apex_state.is_firing_range() && world_ready {
                let mut item_list = Vec::new();
                apex_state.entities_as::<LootEntity>().for_each(|entity| {
                    item_list.push((entity.custom_script_int, entity.model_name.string.clone()));
                });
                if item_list.is_empty() {
                    tracing::debug!("{}", s!("wait items"));
                } else if item_list.len() > log_items {
                    tracing::debug!(?item_list);
                    item_list.sort_by(|(a, _), (b, _)| a.cmp(b));
                    tracing::debug!(?item_list, "{}", s!("items sorted"));
                    log_items = item_list.len();
                }
            }

            // Perform aimbot actions
            match aim_action_rx.try_recv() {
                Ok(action) => {
                    if world_ready && ENABLE_MEM_AIM {
                        if let Some(delta) = action.shift_angles {
                            if let Some(lplayer) = apex_state.local_player() {
                                let ptr = lplayer.entity_ptr.into_raw();
                                let view_angles = mem
                                    .apex_mem_read::<[f32; 3]>(ptr + G_OFFSETS.player_viewangles)?;
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
                                )?;
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
                                )?;
                            }
                        }
                    }
                }
                Err(e) => {
                    if e != TryRecvError::Empty {
                        tracing::error!(%e, ?aim_action_rx, "{}", s!("perform aimbot actions"));
                    }
                }
            }

            // Send key status to aimbot worker
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
                    triggerbot_hotkey: if apex_state.is_button_down(g_settings.trigger_bot_hot_key)
                    {
                        g_settings.trigger_bot_hot_key
                    } else {
                        0
                    },
                    attack_state: apex_state.buttons.in_attack.state.try_into()?,
                })
                .unwrap_or_else(|e| {
                    tracing::error!(%e, ?aim_key_tx, "{}", s!("send key status"));
                });

            /* Hot Variables Update End */

            if actions_tick % 15 == 0 {
                // at least 30ms // don't change xD
            } else if actions_tick % 30_000 == 0 {
                actions_tick = 0;
            } else {
                continue;
            }

            /* Cold Variables Update Start */

            // Update global settings
            {
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
            }

            if !world_ready {
                continue;
            }

            // Update entities
            if world_ready {
                let local_player: Option<GamePlayer> =
                    apex_state.local_player().map(|local_player| {
                        GamePlayer::new(
                            local_player,
                            apex_state,
                            &mut G_STATE.lock().unwrap().config,
                        )
                    });
                let Some(local_position) = local_player.as_ref().map(|l| l.get_entity().origin)
                else {
                    tracing::error!(?apex_state.client, ?local_player, "{}", s!("UNREACHABLE: localplayer=None"));
                    continue;
                };
                let local_position = arr1(&local_position);

                let mut players: HashMap<u64, GamePlayer> = HashMap::new();
                let mut aim_entities: HashMap<u64, Arc<dyn AimEntity>> = HashMap::new();
                let mut loots: Vec<TreasureClue> = Vec::new();

                apex_state.entities_as::<PlayerEntity>().for_each(|entity| {
                    // FIXME: skip wrong entity
                    if entity.eadp_uid == 0 {
                        return;
                    }
                    let game_player =
                        GamePlayer::new(entity, apex_state, &mut G_STATE.lock().unwrap().config);
                    players.insert(entity.entity_ptr.into_raw(), game_player);
                    aim_entities.insert(entity.entity_ptr.into_raw(), Arc::new(entity.clone()));
                });
                apex_state
                    .entities_as::<BaseNPCEntity>()
                    .for_each(|entity| {
                        aim_entities.insert(entity.entity_ptr.into_raw(), Arc::new(entity.clone()));
                    });
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

                let player_count = players.len();
                let entity_count = aim_entities.len();
                let loot_count = loots.len();
                tracing::debug!(
                    player_count,
                    entity_count,
                    loot_count,
                    "{}",
                    s!("entities updated")
                );

                if verbose {
                    tracing::trace!(?apex_state.entity_list.entities, "{}", s!("entity_list"));
                }

                let mut state_wlock = shared_state.lock().await;
                state_wlock.local_player = local_player;
                state_wlock.players = players;
                state_wlock.aim_entities = aim_entities;
                state_wlock.treasure_clues = loots;
            }

            // Init spectator checker
            if let Some(local_player) = &shared_state.lock().await.local_player {
                let local_entity = local_player.get_entity();
                let lplayer_ptr = local_entity.entity_ptr.into_raw();
                if prev_lplayer_ptr != lplayer_ptr {
                    init_spec_checker(lplayer_ptr);
                    prev_lplayer_ptr = lplayer_ptr;
                }
                // Update local entity yew
                let yew = mem.apex_mem_read::<f32>(lplayer_ptr + OFFSET_YAW)?;
                trace!(lplayer_ptr, ?yew);
                apexsky::tick_yew(lplayer_ptr, yew);
            } else {
                continue;
            }

            // Inject highlight settings
            if (g_settings.player_glow || g_settings.item_glow) && world_ready {
                match inject_highlight(apex_state.client.framecount, &g_settings, &mut mem) {
                    Ok(_) => {
                        shared_state.lock().await.highlight_injected = true;
                    }
                    Err(e) => {
                        tracing::debug!(%e, ?e, "{}", s!("Inject highlight settings"));
                    }
                }
            } else {
                let base = mem.apex_mem_baseaddr();
                let glow_fix = mem.apex_mem_read::<i32>(base + OFFSET_GLOW_FIX)?;
                tracing::trace!(glow_fix);
            }

            // Targeting of all eligible
            let mut aim_targets: Vec<PreSelectedTarget> = Vec::new();

            // Map(entity_ptr, is_teammate)
            let mut tmp_specs: HashSet<(u64, bool)> = HashSet::new();

            // Iterate over all targetable entities
            {
                let state = shared_state.lock().await;
                if let Some(lplayer) = state.local_player.as_ref() {
                    for entity in state.aim_entities.values() {
                        process_player(
                            lplayer,
                            entity.as_ref(),
                            &state,
                            &mut tmp_specs,
                            &mut aim_targets,
                            &g_settings,
                            &mut mem,
                        )?;
                    }
                } else {
                    tracing::error!("{}", s!("UNREACHABLE: invalid localplayer"));
                }
            }

            // Send aim targets
            aim_select_tx.send(aim_targets).unwrap_or_else(|e| {
                tracing::error!(%e, ?aim_select_tx, "{}", s!("send aim targets"));
            });

            // Update spectators count
            {
                let allied_spectator_count = tmp_specs
                    .iter()
                    .filter(|(_, is_teammate)| *is_teammate)
                    .count();
                let spectator_count = tmp_specs.len() - allied_spectator_count;

                let mut state = shared_state.lock().await;
                state.allied_spectator_count = allied_spectator_count;
                state.spectator_count = spectator_count;
            }

            // Weapon model glow
            // Not planned

            // Items glow
            // Iterate over all loots
            {
                let state = shared_state.lock().await;
                for clue in &state.treasure_clues {
                    process_loot(clue, &state, &g_settings, &mut mem)?;
                }
            }
        }
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
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
        mem.apex_mem_write::<HighlightBits>(context_offset, bits)?;
        mem.apex_mem_write::<[f32; 3]>(context_offset + 4, &color)?;
    }
    mem.apex_mem_write::<i32>(base + OFFSET_GLOW_FIX, &1)?;
    tracing::trace!(highlight_settings_ptr);

    Ok(())
}

#[instrument(skip_all)]
fn process_player<'a>(
    local_player: &GamePlayer,
    target_entity: &dyn AimEntity,
    state: &SharedState,
    tmp_specs: &mut HashSet<(u64, bool)>,
    aim_targets: &mut Vec<PreSelectedTarget>,
    g_settings: &Settings,
    mem: &mut MemProcImpl<'a>,
) -> anyhow::Result<()> {
    let lplayer_ptr = local_player.get_entity().get_entity_ptr();
    let target_ptr = target_entity.get_entity_ptr();

    let entity_team = target_entity.get_team_num();
    let local_team = local_player.get_buf().team_num;
    let is_teammate = if g_settings.tdm_toggle {
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
            || (state.map_testing_local_team != 0 && entity_team == state.map_testing_local_team)
    };
    // trace!(target_ptr, entity_team, is_teammate);

    if target_entity.is_player() && (!target_entity.is_alive() || !local_player.get_buf().is_alive)
    {
        // Update yew to spec checker
        let yew = mem.apex_mem_read::<f32>(target_ptr + OFFSET_YAW)?;
        apexsky::tick_yew(target_ptr, yew);
        // Exclude self from list when watching others
        if target_ptr != lplayer_ptr && is_spec(target_ptr) {
            tmp_specs.insert((target_ptr, is_teammate));
        }
        return Ok(());
    }

    // Teammate and 1v1 check
    if !g_settings.onevone {
        if g_settings.firing_range {
            if target_entity.is_player() {
                return Ok(());
            }
        } else {
            if is_teammate {
                return Ok(());
            }
        }
    }

    if target_ptr != lplayer_ptr {
        let selected_target = PreSelectedTarget {
            fov: calculate_target_fov(local_player.get_entity(), target_entity),
            distance: {
                let target_pos = target_entity.get_position();
                let local_pos = local_player.get_entity().get_position();
                math::dist(target_pos, local_pos)
            },
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
                        return Ok(());
                    };
                    const LOVESTATUS_AMBIVALENT: i32 = LoveStatus::Ambivalent as i32;
                    const LOVESTATUS_HATE: i32 = LoveStatus::Hate as i32;
                    const LOVESTATUS_LOVE: i32 = LoveStatus::Love as i32;
                    const LOVESTATUS_NORMAL: i32 = LoveStatus::Normal as i32;
                    match target_player.get_buf().love_state {
                        LOVESTATUS_AMBIVALENT => LoveStatus::Ambivalent,
                        LOVESTATUS_HATE => LoveStatus::Hate,
                        LOVESTATUS_LOVE => LoveStatus::Love,
                        LOVESTATUS_NORMAL => LoveStatus::Normal,
                        v => {
                            tracing::error!(love_state = v, player_buf = ?target_player.get_buf());
                            LoveStatus::Normal
                        }
                    }
                }
            },
            entity_ptr: target_ptr,
        };

        // Player Glow
        if g_settings.player_glow && state.highlight_injected && target_ptr > 0 {
            let highlight_context_id = player_glow(&selected_target, state.frame_count, g_settings);
            mem.apex_mem_write::<u8>(target_ptr + OFFSET_GLOW_CONTEXT_ID, &highlight_context_id)?;
            mem.apex_mem_write::<i32>(target_ptr + GLOW_VISIBLE_TYPE, &2)?;
            mem.apex_mem_write::<f32>(target_ptr + GLOW_DISTANCE, &8.0E+4)?;
        }

        // Targeting
        aim_targets.push(selected_target);
    }

    Ok(())
}

#[instrument(skip_all, fields(clue))]
fn process_loot<'a>(
    clue: &TreasureClue,
    state: &SharedState,
    g_settings: &Settings,
    mem: &mut MemProcImpl<'a>,
) -> anyhow::Result<()> {
    let ptr = clue.entity_ptr;
    if ptr <= 0 {
        tracing::error!(?clue);
        return Ok(());
    }
    let mut w = |ctx_id: u8| -> anyhow::Result<()> {
        mem.apex_mem_write::<u8>(ptr + OFFSET_GLOW_CONTEXT_ID, &ctx_id)
    };

    let item_id = ItemId::try_from(clue.custom_item_id).unwrap_or_else(|_e| {
        //tracing::warn!(?clue, "{}", s!("unknown item id"));
        ItemId::Unknown
    });

    if !(g_settings.item_glow && state.highlight_injected) {
        return Ok(());
    }

    let select = &g_settings.loot;

    match item_id {
        // Backpacks
        ItemId::LightBackpack if select.lightbackpack => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::MedBackpack if select.medbackpack => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::HeavyBackpack if select.heavybackpack => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::GoldBackpack if select.goldbackpack => w(HIGHLIGHT_LOOT_GOLD)?,

        // Shields
        ItemId::ShieldUpgrade1_0 | ItemId::ShieldUpgrade1_1 if select.shieldupgrade1 => {
            w(HIGHLIGHT_LOOT_WHITE)?
        }
        ItemId::ShieldUpgrade2_0 | ItemId::ShieldUpgrade2_1 | ItemId::ArmorCore1
            if select.shieldupgrade2 =>
        {
            w(HIGHLIGHT_LOOT_BLUE)?
        }
        ItemId::ShieldUpgrade3_0 | ItemId::ShieldUpgrade3_1 | ItemId::ArmorCore2
            if select.shieldupgrade3 =>
        {
            w(HIGHLIGHT_LOOT_PURPLE)?
        }
        ItemId::ShieldUpgrade4 | ItemId::ArmorCore3 if select.shieldupgrade4 => {
            w(HIGHLIGHT_LOOT_GOLD)?
        }
        ItemId::ShieldUpgrade5 | ItemId::ArmorCore4 if select.shieldupgrade5 => {
            w(HIGHLIGHT_LOOT_RED)?
        }
        ItemId::ShieldUpgradeHead1 if select.shieldupgradehead1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::ShieldUpgradeHead2 if select.shieldupgradehead2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::ShieldUpgradeHead3 if select.shieldupgradehead3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::ShieldUpgradeHead4 if select.shieldupgradehead4 => w(HIGHLIGHT_LOOT_GOLD)?,

        // Heals
        ItemId::Accelerant if select.accelerant => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::Phoenix if select.phoenix => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::HealthLarge if select.healthlarge => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::HealthSmall if select.healthsmall => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::ShieldBatterySmall if select.shieldbattsmall => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::ShieldBatteryLarge if select.shieldbattlarge => w(HIGHLIGHT_LOOT_BLUE)?,

        // Ammos
        ItemId::LightAmmo if select.lightammo => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::HeavyAmmo if select.heavyammo => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::EnergyAmmo if select.energyammo => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::SniperAmmo if select.sniperammo => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::ShotgunAmmo if select.shotgunammo => w(HIGHLIGHT_LOOT_RED)?,

        // Mags
        ItemId::LightAmmoMag1 if select.lightammomag1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::LightAmmoMag2 if select.lightammomag2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::LightAmmoMag3 if select.lightammomag3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::LightAmmoMag4 if select.lightammomag4 => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::HeavyAmmoMag1 if select.heavyammomag1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::HeavyAmmoMag2 if select.heavyammomag2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::HeavyAmmoMag3 if select.heavyammomag3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::HeavyAmmoMag4 if select.heavyammomag4 => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::SniperAmmoMag1 if select.sniperammomag1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::SniperAmmoMag2 if select.sniperammomag2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::SniperAmmoMag3 if select.sniperammomag3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::SniperAmmoMag4 if select.sniperammomag4 => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::EnergyAmmoMag1 if select.energyammomag1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::EnergyAmmoMag2 if select.energyammomag2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::EnergyAmmoMag3 if select.energyammomag3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::EnergyAmmoMag4 if select.energyammomag4 => w(HIGHLIGHT_LOOT_GOLD)?,

        // Stocks
        ItemId::StockSniper1 if select.stocksniper1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::StockSniper2 if select.stocksniper2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::StockSniper3 if select.stocksniper3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::StockRegular1 if select.stockregular1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::StockRegular2 if select.stockregular2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::StockRegular3 if select.stockregular3 => w(HIGHLIGHT_LOOT_PURPLE)?,

        // Down Shields
        ItemId::ShieldDown1 if select.shielddown1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::ShieldDown2 if select.shielddown2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::ShieldDown3 if select.shielddown3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::ShieldDown4 if select.shielddown4 => w(HIGHLIGHT_LOOT_GOLD)?,

        // Optics
        ItemId::Optic1xHCOG if select.optic1xhcog => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::Optic2xHCOG if select.optic2xhcog => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::OpticHolo1x if select.opticholo1x => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::OpticHolo1x2x if select.opticholo1x2x => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::OpticThreat if select.opticthreat => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::Optic3xHCOG if select.optic3xhcog => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::Optic2x4x if select.optic2x4x => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::OpticSniper6x if select.opticsniper6x => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::OpticSniper4x8x if select.opticsniper4x8x => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::OpticSniperThreat if select.opticsniperthreat => w(HIGHLIGHT_LOOT_GOLD)?,

        // Hop-ups
        ItemId::LaserSight1 if select.lasersight1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::LaserSight2 if select.lasersight2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::LaserSight3 if select.lasersight3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::Suppressor1 if select.suppressor1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::Suppressor2 if select.suppressor2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::Suppressor3 if select.suppressor3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::TurboCharger if select.turbo_charger => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::SkullPiecer if select.skull_piecer => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::HammerPoint if select.hammer_point => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::DisruptorRounds if select.disruptor_rounds => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::BoostedLoader if select.boosted_loader => w(HIGHLIGHT_LOOT_GOLD)?,
        ItemId::ShotgunBolt1 if select.shotgunbolt1 => w(HIGHLIGHT_LOOT_WHITE)?,
        ItemId::ShotgunBolt2 if select.shotgunbolt2 => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::ShotgunBolt3 if select.shotgunbolt3 => w(HIGHLIGHT_LOOT_PURPLE)?,
        ItemId::ShotgunBolt4 if select.shotgunbolt4 => w(HIGHLIGHT_LOOT_GOLD)?,

        // Nades
        ItemId::GrenadeFrag if select.grenade_frag => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::GrenadeThermite if select.grenade_thermite => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::GrenadeArcStar if select.grenade_arc_star => w(HIGHLIGHT_LOOT_GREY)?,

        // Weapons
        ItemId::WeaponKraber if select.weapon_kraber => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::WeaponMastiff if select.weapon_mastiff => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::WeaponLStar if select.weapon_lstar => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponNemesis if select.weapon_nemesis => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponHavoc if select.weapon_havoc => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponDevotion if select.weapon_devotion => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponTripleTake if select.weapon_triple_take => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponFlatline if select.weapon_flatline => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::WeaponHemlock if select.weapon_hemlock => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::WeaponG7Scout if select.weapon_g7_scout => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponAlternator if select.weapon_alternator => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponR99 if select.weapon_r99 => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponProwler if select.weapon_prowler => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::WeaponVolt if select.weapon_volt => w(HIGHLIGHT_LOOT_ENERGY)?,
        ItemId::WeaponLongbow if select.weapon_longbow => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::WeaponChargeRifle if select.weapon_charge_rifle => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::WeaponSpitfire if select.weapon_spitfire => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponR301 if select.weapon_r301 => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponEva8 if select.weapon_eva8 => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::WeaponPeacekeeper if select.weapon_peacekeeper => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::WeaponMozambique if select.weapon_mozambique => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::WeaponWingman if select.weapon_wingman => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::WeaponP2020 if select.weapon_p2020 => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponRE45 if select.weapon_re45 => w(HIGHLIGHT_LOOT_LIGHT)?,
        ItemId::WeaponSentinel if select.weapon_sentinel => w(HIGHLIGHT_LOOT_BLUE)?,
        ItemId::WeaponBow if select.weapon_bow => w(HIGHLIGHT_LOOT_RED)?,
        ItemId::Weapon3030Repeater if select.weapon_3030_repeater => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::WeaponRampage if select.weapon_rampage => w(HIGHLIGHT_LOOT_HEAVY)?,
        ItemId::WeaponCARSMG if select.weapon_car_smg => w(HIGHLIGHT_LOOT_HEAVY)?,

        _ => (),
    }

    Ok(())
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
