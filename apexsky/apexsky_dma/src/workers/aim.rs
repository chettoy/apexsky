use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use apexsky::aimbot::{
    AimAngles, AimEntity, Aimbot, AimbotSettings, CurrentWeaponInfo, TriggerBot,
};
use apexsky::config::DeviceConfig;
use apexsky::global_state::G_STATE;
use apexsky::love_players::LoveStatus;
use apexsky_dmalib::access::MemApi;
use apexsky_kmbox::kmbox::{KmboxB, KmboxNet};
use apexsky_proto::pb::apexlegends::{AimKeyState, AimTargetInfo};
use obfstr::obfstr as s;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, sleep_until, Instant};
use tracing::{instrument, trace};

use crate::actuator::{
    AimActuator, AimbotAction, DeviceAimActuator, KmboxAimActuator, MemAimHelper, QmpAimActuator,
};
use crate::apexdream::base::math;
use crate::usermod_thr::UserModEvent;
use crate::SharedStateWrapper;

const ENABLE_MEM_AIM: bool = true;

pub trait ContextForAimbot {
    async fn get_aimbot_settings(&self) -> Option<AimbotSettings>;
    async fn get_entity(&self, target_ptr: u64) -> Option<Arc<dyn AimEntity>>;
    async fn get_held_id(&self) -> Option<i32>;
    async fn get_weapon_info(&self) -> Option<CurrentWeaponInfo>;
    async fn update_aim_target_for_esp(&mut self, position: [f32; 3]);
}

async fn create_aim_actuator_from_device(
    config: &DeviceConfig,
) -> anyhow::Result<Option<DeviceAimActuator>> {
    if config.use_kmbox_net {
        let (addr, mac) = (config.kmbox_net_addr, config.kmbox_net_mac);
        let mac = u32::from_str_radix(&hex::encode(mac), 16)?;
        let kmbox_aim = KmboxAimActuator::<KmboxNet>::connect(addr, mac).await?;
        Ok(Some(kmbox_aim.into()))
    } else if config.use_kmbox_b {
        let (serialport, baud) = (&config.kmbox_b_serialport, config.kmbox_b_baud);
        let kmbox_aim = KmboxAimActuator::<KmboxB>::connect(serialport, baud).await?;
        Ok(Some(kmbox_aim.into()))
    } else if config.use_qemu_qmp {
        let addr = &config.qemu_qmp_addr;
        let qmp_aim = QmpAimActuator::connect(addr).await?;
        Ok(Some(qmp_aim.into()))
    } else {
        Ok(None)
    }
}

#[instrument(skip_all)]
pub async fn aimbot_loop(
    mut active: watch::Receiver<bool>,
    mut state: SharedStateWrapper,
    access_tx: MemApi,
    usermod_event_tx: mpsc::UnboundedSender<UserModEvent>,
    mut aim_key_rx: watch::Receiver<AimKeyState>,
    mut aim_select_rx: watch::Receiver<Vec<AimTargetInfo>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let usermod_send_event = |event: UserModEvent| {
        if let Err(e) = usermod_event_tx.send(event) {
            tracing::error!(%e, "{}", s!("usermod_send_event"));
        };
    };

    let mut start_instant = Instant::now();
    let mut aimbot = Aimbot::default();
    let mut assist_score = 0;
    let mut natural_delta_viewangles: VecDeque<[f32; 3]> = VecDeque::with_capacity(20);
    let mut prev_recoil_angle: [f32; 3] = [0.0, 0.0, 0.0];
    let mut prev_view_angles: Option<[f32; 3]> = None;
    let mut prev_aim_target_id: u64 = 0;
    let mut mem_aim_helper = MemAimHelper {
        mem: access_tx.clone(),
        apex_base: 0,
        lplayer_ptr: 0,
    };
    let mut aim_actuator = {
        let device_config = G_STATE.lock().unwrap().config.device.clone();
        create_aim_actuator_from_device(&device_config)
            .await
            .unwrap_or_else(|e| {
                tracing::error!(%e, ?e, "{}", s!("Unable to connect to actuator device."));
                None
            })
    };

    while *active.borrow_and_update() {
        sleep(Duration::from_millis(2)).await;

        let loop_duration = start_instant.elapsed();
        start_instant = Instant::now();

        // Check game_attached and world_ready
        if !state.is_world_ready() {
            tracing::trace!("{}", s!("waiting for world ready"));
            start_instant += Duration::from_millis(500);
            sleep_until(start_instant).await;
            continue;
        }
        // Check base_addr and local_player_ptr
        mem_aim_helper.apex_base = state.get_game_baseaddr().unwrap_or(0);
        mem_aim_helper.lplayer_ptr = state.get_local_player_ptr().unwrap_or(0);

        if !mem_aim_helper.ready() {
            tracing::trace!("{}", s!("waiting for mem_aim_executer ready"));
            start_instant += Duration::from_millis(500);
            sleep_until(start_instant).await;
            continue;
        }

        // Check local_player entity
        if !state
            .players
            .read()
            .contains_key(&mem_aim_helper.lplayer_ptr)
        {
            tracing::trace!("{}", s!("waiting for local player ready"));
            start_instant += Duration::from_millis(500);
            sleep_until(start_instant).await;
            continue;
        };

        // Calc smooth factor
        let smooth_factor = loop_duration.as_millis() as f32 / 1.054571726;
        trace!(%smooth_factor, loop_duration = loop_duration.as_millis());

        // Read held_id from shared_state
        if let Some(held_id) = state.get_held_id().await {
            aimbot.update_held_id(held_id);
        } else {
            tracing::error!("{}", s!("failed to get held_id"));
        }

        // Read weapon from shared_state
        if let Some(active_weapon) = state.get_weapon_info().await {
            aimbot.update_weapon_info(active_weapon);
        } else {
            tracing::trace!("{}", s!("active_weapon=None"));
        }

        // Receive key status and update it to aimbot
        if aim_key_rx.has_changed().unwrap_or_else(|e| {
            tracing::error!(%e, ?aim_key_rx, "{}", s!("recv key status"));
            false
        }) {
            let key_status = aim_key_rx.borrow_and_update();
            if key_status.aimbot_hotkey_1 > 0 {
                aimbot.update_aim_key_state(key_status.aimbot_hotkey_1);
            } else if key_status.aimbot_hotkey_2 > 0 {
                aimbot.update_aim_key_state(key_status.aimbot_hotkey_2);
            } else {
                aimbot.update_aim_key_state(0);
            }
            aimbot.update_attack_state(key_status.attack_button);
            aimbot.update_zoom_state(key_status.zoom_button);
            aimbot.update_triggerbot_key_state(key_status.triggerbot_hotkey);
            aimbot.update_quick_looting_key_state(key_status.quick_looting_hotkey);
        }

        // Receive pre-selected targets and update it to aimbot
        if aim_select_rx.has_changed().unwrap_or_else(|e| {
            tracing::error!(%e, ?aim_select_rx, "{}", s!("recv aim targets"));
            false
        }) {
            aimbot.start_select_target();
            aim_select_rx.borrow_and_update().iter().for_each(|t| {
                aimbot.add_select_target(
                    t.fov,
                    t.distance,
                    t.is_visible,
                    t.love_status == LoveStatus::Love as i32,
                    t.is_npc,
                    t.is_loot,
                    t.entity_ptr,
                );
            });
            aimbot.finish_select_target();
        }

        // Update aimbot settings
        // Lower update frequency to reduce cpu usage
        if state.get_frame_count() % 30 == 0 {
            if let Some(aimbot_settings) = state.get_aimbot_settings().await {
                aimbot.settings(aimbot_settings);
                trace!("{}", s!("aimbot_settings reload"));
            }
        }

        // Update Aimbot state
        aimbot.update(mem_aim_helper.lplayer_ptr, state.get_game_fps());

        let aiming = aimbot.is_aiming();
        //tracing::trace!(?aiming, "711aac39-e83c-4788");

        let aim_entity_ptr = aimbot.get_aim_entity();
        let target_entity: Option<Arc<dyn AimEntity>> = state.get_entity(aim_entity_ptr).await;
        let Some(local_entity) = state.get_entity(mem_aim_helper.lplayer_ptr).await else {
            continue;
        };

        let (aim_result, target_pos): (AimAngles, Option<[f32; 3]>) =
            if let Some(ref target_entity) = target_entity {
                // // debug target entity
                // if !target_entity.is_player() {
                //     let is_visible = target_entity.is_visible();
                //     trace!(is_visible, ?target_entity, "{}", s!("711aac39-e83c-444b"));
                // }

                if !(aimbot.is_aiming() || aimbot.is_triggerbot_ready()) {
                    (AimAngles::default(), Some(target_entity.get_position()))
                } else if aimbot.get_gun_safety() {
                    trace!("{}", s!("711aac39-e83c-4788 safety on"));
                    //println!("{:?}", target_entity);
                    (AimAngles::default(), Some(target_entity.get_position()))
                } else if local_entity.is_knocked()
                    || !target_entity.is_alive()
                    || target_entity.is_knocked()
                {
                    trace!(
                        ?target_entity,
                        ?local_entity,
                        "{}",
                        s!("711aac39-e83c-4788 not target")
                    );
                    aimbot.cancel_locking();
                    (AimAngles::default(), Some(target_entity.get_position()))
                } else {
                    trace!("{}", s!("711aac39-e83c-4788 calc best aim"));
                    let Some(view_angles) = read_view_angles(&mem_aim_helper).await else {
                        continue;
                    };
                    let (aim_angles, position) = aimbot.calc_best_aim(
                        local_entity.as_ref(),
                        target_entity.as_ref(),
                        view_angles,
                    );
                    trace!(?aim_angles, "{}", s!("711aac39-e83c-4788 best aim"));
                    (aim_angles, Some(position))
                }
            } else {
                //tracing::warn!(aim_entity_ptr, "{}", s!("targeted entity does not exist"));
                tracing::debug!(
                    aim_entity_ptr,
                    "{}",
                    s!("711aac39-e83c-4788 targeted entity does not exist")
                );
                aimbot.cancel_locking();
                (AimAngles::default(), None)
            };

        if aiming {
            tracing::debug!(?aim_result, "711aac39-e83c-4788");
        }

        if let Some(pos) = target_pos {
            tracing::trace!(target_pos = ?pos, ?aim_result);
            state.update_aim_target_for_esp(pos).await;
        }

        // Update Trigger Bot state
        // Ensure that the triggerbot is updated,
        // otherwise there may be issues with not canceling after firing.
        aimbot.triggerbot_update(target_entity, &aim_result, aim_key_rx.borrow().attack_state);
        if aiming {
            tracing::debug!("711aac39-e83c-4788 trigger updated");
        }

        let aimbot_settings = aimbot.get_settings();

        let mut shift_angles: Option<[f32; 3]> = None;

        // Aim Assist
        if aimbot.is_aiming() && aim_result.valid {
            let view_angles = [aim_result.view_pitch, aim_result.view_yew, 0.0];
            let smoothed_angles = aimbot.smooth_aim_angles(&aim_result, smooth_factor);
            let smoothed_angles = [smoothed_angles.0, smoothed_angles.1, 0.0];
            let smoothed_delta_angles = math::sub(smoothed_angles, view_angles);

            if aimbot_settings.aim_mode & 0x4 != 0 && !aimbot.is_grenade() {
                let natural_viewangle_vel = {
                    let arr = natural_delta_viewangles.clone();
                    let arr_len = arr.len() as usize;
                    let all_sum_values = arr
                        .into_iter()
                        .reduce(|acc, e| math::add(acc, e))
                        .unwrap_or_default();
                    let mean = math::muls(all_sum_values, 1.0 / arr_len as f32);
                    mean
                };

                //println!("{:?}", natural_delta);

                #[inline]
                fn check(natural_viewangle_vel: f32, smoothed_delta: f32, score: &mut i32) -> f32 {
                    //println!("{natural_viewangle_vel:?} {score:?}");

                    if !natural_viewangle_vel.is_normal() {
                        return 0.0;
                    }

                    let bingo = (smoothed_delta.signum() * natural_viewangle_vel.signum())
                        .is_sign_positive();

                    if bingo {
                        //println!("bingo");
                        if *score < 0 {
                            *score = 0;
                        }
                        *score += (natural_viewangle_vel * 1000.0).round().abs() as i32;
                    } else {
                        *score -= (natural_viewangle_vel * 1000.0).round().abs() as i32;
                    }

                    let _score_abs = {
                        let abs = score.abs();
                        if abs > 100 {
                            *score = score.signum() * 100;
                            100
                        } else {
                            abs
                        }
                    };

                    if *score > 0 {
                        if bingo {
                            let max_accel_x = 40.0;
                            smoothed_delta.signum()
                                * f32::min(
                                    smoothed_delta.abs(),
                                    natural_viewangle_vel.abs() * max_accel_x,
                                )
                        } else {
                            let max_decel_x = 20.0;
                            smoothed_delta.signum()
                                * f32::min(
                                    smoothed_delta.abs(),
                                    natural_viewangle_vel.abs() * max_decel_x,
                                )
                        }
                    } else {
                        0.0
                    }
                }

                if aim_entity_ptr != prev_aim_target_id {
                    assist_score = 0;
                    prev_aim_target_id = aim_entity_ptr;
                }

                let assist_delta = [
                    check(
                        natural_viewangle_vel[0],
                        smoothed_delta_angles[0],
                        &mut assist_score,
                    ),
                    check(
                        natural_viewangle_vel[1],
                        smoothed_delta_angles[1],
                        &mut assist_score,
                    ),
                    smoothed_delta_angles[2],
                ];
                //println!("{assist_score:?}");
                if assist_delta[0].is_normal() || assist_delta[1].is_normal() {
                    shift_angles = Some(assist_delta);
                }
            } else {
                shift_angles = Some(smoothed_delta_angles);
            }
        }

        if aiming {
            tracing::debug!(?shift_angles, "711aac39-e83c-4788");
        }

        // Reduce recoil
        if aimbot_settings.no_recoil {
            // get recoil angle
            let recoil_angles = local_entity.get_recoil_angles();
            trace!(?prev_recoil_angle, ?recoil_angles);

            if recoil_angles[0] < 0.0 {
                let mut delta_angle = [0.0, 0.0, 0.0];
                // removing recoil angles from player view angles
                delta_angle[0] = (prev_recoil_angle[0] - recoil_angles[0])
                    * (aimbot_settings.recoil_smooth_x / 100.0);
                delta_angle[1] = (prev_recoil_angle[1] - recoil_angles[1])
                    * (aimbot_settings.recoil_smooth_y / 100.0);

                // setting viewangles to new angles
                shift_angles = {
                    if let Some(shift_angles_aim) = shift_angles {
                        Some(math::add(shift_angles_aim, delta_angle))
                    } else {
                        Some(delta_angle)
                    }
                };
            }

            // setting old recoil angles to current recoil angles
            prev_recoil_angle = recoil_angles;
        } else {
            prev_recoil_angle = [0.0, 0.0, 0.0];
        }

        if aiming {
            tracing::debug!(?shift_angles, "711aac39-e83c-4788 rcs");
        }

        // Create aimbot action
        let aimbot_action = AimbotAction {
            shift_angles,
            force_attack: match aimbot.poll_trigger_action() {
                5 => Some(true),
                4 => Some(false),
                _ => None,
            },
            force_use: match aimbot.poll_looting_action() {
                5 => Some(true),
                4 => Some(false),
                _ => None,
            },
        };

        // Update state for ESP
        *state.aimbot_state.lock() = Some((aimbot.clone(), loop_duration));

        // Update state for UserMod
        usermod_send_event(UserModEvent::AimbotTick(aimbot.clone(), aim_result));

        // Read view_angles
        let Some(view_angles) = read_view_angles(&mem_aim_helper).await else {
            continue;
        };

        // Calc delta_view_angles
        if natural_delta_viewangles.len() >= 20 {
            natural_delta_viewangles.pop_front();
        }
        natural_delta_viewangles.push_back(
            prev_view_angles
                .map(|prev| math::sub(view_angles, prev))
                .unwrap_or([0.0, 0.0, 0.0]),
        );

        // Perform aimbot action
        if let Some(ref mut actuator) = aim_actuator {
            actuator.perform(aimbot_action).await.ok();
            prev_view_angles = Some(view_angles);
        } else if ENABLE_MEM_AIM {
            let mut actuator = mem_aim_helper.get_actuator(view_angles);
            actuator.perform(aimbot_action).await.ok();
            prev_view_angles = Some(actuator.get_updated_viewangles().unwrap_or(view_angles));
        } else {
            prev_view_angles = Some(view_angles);
        }
    }
    tracing::debug!("{}", s!("task end"));

    Ok(())
}

async fn read_view_angles(mem_aim_helper: &MemAimHelper) -> Option<[f32; 3]> {
    let view_angles = match MemAimHelper::read_viewangles(
        &mem_aim_helper.mem,
        mem_aim_helper.lplayer_ptr,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(%e, "{}", s!("err read viewangles"));
            return None;
        }
    };
    if !(view_angles[0].is_finite() && view_angles[1].is_finite() && view_angles[2].is_finite()) {
        tracing::warn!(?view_angles, "{}", s!("got invalid view_angles"));
        return None;
    }
    Some(view_angles)
}

struct AimingInfo {
    pub local_origin: [f32; 3],
    pub view_angles: [f32; 3],
    pub target_origin: [f32; 3],
    pub target_vel: [f32; 3],
}

async fn read_aiming_info(
    mem_aim_helper: &MemAimHelper,
    target: &dyn AimEntity,
) -> anyhow::Result<AimingInfo> {
    use apexsky::offsets::G_OFFSETS;
    use apexsky_dmalib::access::{AccessType, PendingAccessRequest, PendingMemRead};
    use std::mem::size_of;

    let lplayer_ptr = mem_aim_helper.lplayer_ptr;
    let target_ptr = target.get_entity_ptr();
    let mem = &mem_aim_helper.mem;

    let reqs = (
        AccessType::mem_read(
            lplayer_ptr + G_OFFSETS.centity_origin,
            size_of::<[f32; 3]>(),
            0,
        ),
        AccessType::mem_read(
            lplayer_ptr + G_OFFSETS.player_viewangles,
            size_of::<[f32; 3]>(),
            0,
        ),
        AccessType::mem_read(
            target_ptr + G_OFFSETS.centity_origin,
            size_of::<[f32; 3]>(),
            0,
        ),
        AccessType::mem_read(
            target_ptr + G_OFFSETS.centity_velocity,
            size_of::<[f32; 3]>(),
            0,
        ),
    );
    let futs = tokio::try_join!(
        reqs.0.with_priority(10).dispatch(mem),
        reqs.1.with_priority(10).dispatch(mem),
        reqs.2.with_priority(10).dispatch(mem),
        reqs.3.with_priority(10).dispatch(mem),
    )?;
    let vals = tokio::try_join!(
        futs.0.recv_for::<[f32; 3]>(),
        futs.1.recv_for::<[f32; 3]>(),
        futs.2.recv_for::<[f32; 3]>(),
        futs.3.recv_for::<[f32; 3]>(),
    )?;
    Ok(AimingInfo {
        local_origin: vals.0,
        view_angles: vals.1,
        target_origin: vals.2,
        target_vel: if target.is_player() {
            vals.3
        } else {
            target.get_abs_velocity()
        },
    })
}
