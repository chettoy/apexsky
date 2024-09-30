use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use apexsky::aimbot::{
    calc_angle, calc_fov, normalize_angles, normalize_delta_angles, AimAngles, AimEntity, Aimbot,
    AimbotSettings, CurrentWeaponInfo, HitScanReport, TriggerBot,
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
    async fn update_aim_target_for_esp(
        &mut self,
        aim_result: AimAngles,
        hitscan_result: Option<HitScanReport>,
        target_pos: Option<[f32; 3]>,
    );
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

        let (aim_result, hitscan, target_pos): (
            AimAngles,
            Option<HitScanReport>,
            Option<[f32; 3]>,
        ) = if let Some(ref target_entity) = target_entity {
            // // debug target entity
            // if !target_entity.is_player() {
            //     let is_visible = target_entity.is_visible();
            //     trace!(is_visible, ?target_entity, "{}", s!("711aac39-e83c-444b"));
            // }

            if !(aimbot.is_aiming() || aimbot.is_triggerbot_ready()) {
                (
                    AimAngles::default(),
                    None,
                    Some(target_entity.get_position()),
                )
            } else if aimbot.get_gun_safety() {
                trace!("{}", s!("711aac39-e83c-4788 safety on"));
                //println!("{:?}", target_entity);
                (
                    AimAngles::default(),
                    None,
                    Some(target_entity.get_position()),
                )
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
                (
                    AimAngles::default(),
                    None,
                    Some(target_entity.get_position()),
                )
            } else {
                trace!("{}", s!("711aac39-e83c-4788 calc best aim"));
                let Some(view_angles) = read_view_angles(&mem_aim_helper).await else {
                    continue;
                };
                let (aim_angles, hitscan, position) = aimbot.calc_best_aim(
                    local_entity.as_ref(),
                    target_entity.as_ref(),
                    view_angles,
                );
                trace!(?aim_angles, "{}", s!("711aac39-e83c-4788 best aim"));
                (aim_angles, Some(hitscan), Some(position))
            }
        } else {
            //tracing::warn!(aim_entity_ptr, "{}", s!("targeted entity does not exist"));
            tracing::debug!(
                aim_entity_ptr,
                "{}",
                s!("711aac39-e83c-4788 targeted entity does not exist")
            );
            aimbot.cancel_locking();
            (AimAngles::default(), None, None)
        };

        if aiming {
            tracing::debug!(?aim_result, "711aac39-e83c-4788");
        }

        if let Some(pos) = target_pos {
            tracing::trace!(target_pos = ?pos, ?aim_result);
        }
        state
            .update_aim_target_for_esp(aim_result.clone(), hitscan, target_pos)
            .await;

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
            let view_angles = [aim_result.view_pitch, aim_result.view_yaw, 0.0];
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

trait BestAim {
    fn calc_best_aim(
        &self,
        from: &dyn AimEntity,
        target: &dyn AimEntity,
        view_angles: [f32; 3],
    ) -> (AimAngles, HitScanReport, [f32; 3]);

    fn hit_scan(
        &self,
        view_origin: [f32; 3],
        view_angles: [f32; 3],
        target_origin: [f32; 3],
        target_vel: [f32; 3],
        target_hitboxes: Vec<([f32; 3], ([f32; 3], [f32; 3]))>,
    ) -> HitScanReport;
}

impl BestAim for Aimbot {
    #[instrument(skip_all)]
    fn calc_best_aim(
        &self,
        from: &dyn AimEntity,
        target: &dyn AimEntity,
        //local_origin: [f32; 3],
        view_angles: [f32; 3],
        //target_origin: [f32; 3],
        //target_vel: [f32; 3],
    ) -> (AimAngles, HitScanReport, [f32; 3]) {
        let target_origin = target.get_position();
        let target_vel = target.get_abs_velocity();
        let local_origin = from.get_position();
        let view_origin = math::add(local_origin, from.get_view_offset());
        let camera_origin = from.get_cam_pos();
        //let view_angles = from.get_view_angles();
        let sway_angles = from.get_sway_angles();
        let distance = math::dist(camera_origin, target_origin);
        let aimbot_settings = self.get_settings();
        let weapon_info = self.get_weapon_info();
        let expect_headshot = self.is_headshot() && distance <= aimbot_settings.headshot_dist;

        let hitscan = self.hit_scan(
            view_origin,
            view_angles,
            target_origin,
            target_vel,
            target.get_hitbox(),
        );

        let target_head_pos = target.get_bone_position_by_hitbox(0);
        let (target_bone_position_min, target_bone_position_max): ([f32; 3], [f32; 3]) =
            if expect_headshot {
                (target_head_pos, target_head_pos)
            } else if aimbot_settings.bone_nearest {
                let lowest_aim_pos = math::muls(
                    math::add(target.get_bone_position_by_hitbox(3), target_origin),
                    0.5,
                );
                let best_hitbox = hitscan.nearest_hitbox.and_then(|hitbox| {
                    let (bone_pos, (bbmin, bbmax)) = hitbox;
                    if f32::min(bone_pos[2] + bbmin[2], bone_pos[2] + bbmax[2]) > lowest_aim_pos[2]
                    {
                        Some(hitbox)
                    } else {
                        None
                    }
                });
                let best_bone_pos = hitscan.nearest_bone_pos.and_then(|pos| {
                    if pos[2] > lowest_aim_pos[2] {
                        Some(pos)
                    } else {
                        None
                    }
                });
                match (best_hitbox, best_bone_pos, hitscan.hit) {
                    (Some(hitbox), _, false) => {
                        let (bone_pos, (bbmin, bbmax)) = hitbox;
                        let scale = if aimbot_settings.auto_shoot && self.is_triggerbot_ready() {
                            0.4
                        } else {
                            0.9
                        };
                        let min = math::add(bone_pos, math::muls(bbmin, scale));
                        let max = math::add(bone_pos, math::muls(bbmax, scale));
                        (min, max)
                    }
                    (None, Some(best_bone_pos), false) => (best_bone_pos, best_bone_pos),
                    _ => {
                        let min = [
                            target_head_pos[0],
                            target_head_pos[1],
                            target_head_pos[2] + 10.0,
                        ];
                        let max = if expect_headshot {
                            target.get_bone_position_by_hitbox(1)
                        } else {
                            lowest_aim_pos
                        };
                        (min, max)
                    }
                }
            } else if aimbot_settings.bone_auto {
                (target_head_pos, target_origin)
            } else {
                let fixed_bone_pos =
                    target.get_bone_position_by_hitbox(aimbot_settings.bone.try_into().unwrap());
                (fixed_bone_pos, fixed_bone_pos)
            };
        // tracing::trace!(
        //     ?target_bone_position_max,
        //     ?target_bone_position_min,
        //     "{}",
        //     s!("711aac39-e83c-4788")
        // );

        let aim_target: [f32; 3];

        if !self.is_grenade() {
            let fun_calc_angles =
                |local_camera_position: [f32; 3],
                 target_bone_position: [f32; 3],
                 target_vel: [f32; 3],
                 weapon_info: &CurrentWeaponInfo| {
                    let mut aim_target: [f32; 3] = target_bone_position;
                    let mut calculated_angles = None;

                    if self.get_quick_looting_ready() {
                        return (
                            calc_angle(&local_camera_position, &target_bone_position),
                            aim_target,
                        );
                    }

                    if weapon_info.bullet_speed > 1.0 {
                        let distance_to_target =
                            math::dist(target_bone_position, local_camera_position);
                        let time_to_target = distance_to_target / weapon_info.bullet_speed;
                        let target_pos_ahead = math::add(
                            target_bone_position,
                            math::muls(target_vel, time_to_target * 0.5),
                        );
                        // let target_pos_ahead = target_bone_position;

                        aim_target = target_pos_ahead;

                        calculated_angles = linear_predict(
                            weapon_info,
                            local_camera_position,
                            target_pos_ahead,
                            target_vel,
                        );
                    }

                    if calculated_angles.is_some() {
                        trace!(?calculated_angles);
                    } else {
                        let angles = calc_angle(&local_camera_position, &target_bone_position);
                        // tracing::debug!(
                        //     ?local_camera_position,
                        //     ?target_bone_position,
                        //     ?angles,
                        //     "{}",
                        //     s!("711aac39-e83c-4788")
                        // );
                        calculated_angles = (angles[0], angles[1]).into();
                    }
                    let calculated_angles = calculated_angles.unwrap();
                    ([calculated_angles.0, calculated_angles.1, 0.0], aim_target)
                };

            let (calculated_angles_min, _) = fun_calc_angles(
                camera_origin,
                target_bone_position_min,
                target_vel,
                weapon_info,
            );
            let (calculated_angles_max, aim_pos) = fun_calc_angles(
                camera_origin,
                target_bone_position_max,
                target_vel,
                weapon_info,
            );
            aim_target = aim_pos;

            let mut calculated_angles_min =
                math::sub(calculated_angles_min, math::sub(sway_angles, view_angles));
            let mut calculated_angles_max =
                math::sub(calculated_angles_max, math::sub(sway_angles, view_angles));
            normalize_angles(&mut calculated_angles_min);
            normalize_angles(&mut calculated_angles_max);
            let mut delta_min = math::sub(calculated_angles_min, view_angles);
            let mut delta_max = math::sub(calculated_angles_max, view_angles);
            normalize_delta_angles(&mut delta_min);
            normalize_delta_angles(&mut delta_max);

            let mut delta = [0.0, 0.0, 0.0];
            if (delta_min[0] * delta_max[0]).is_sign_positive() {
                delta[0] = (delta_min[0] + delta_max[0]) * 0.5;
            }
            if (delta_min[1] * delta_max[1]).is_sign_positive() {
                delta[1] = (delta_min[1] + delta_max[1]) * 0.5;
            }
            //println!("{:.2},{:.2}  {:.2},{:.2}", delta_min[0], delta_min[1], delta_max[0], delta_max[1]);

            let target_fov = calc_fov(&[0.0, 0.0, 0.0], &delta);
            let max_fov = {
                let mut fov = self.get_max_fov();
                if distance < 160.0 {
                    fov += 30.0;
                }
                if distance < 80.0 {
                    fov += 60.0;
                }
                let zoom_fov = weapon_info.weapon_zoom_fov;
                if zoom_fov.is_normal() && (zoom_fov - 1.0).abs() > f32::EPSILON {
                    fov *= zoom_fov / 90.0
                }
                // When autofire is enabled, add up to an additional 30 fov to meet the requirement
                if aimbot_settings.auto_shoot && self.is_triggerbot_ready() {
                    fov = f32::max(f32::min(fov + 30.0, target_fov), fov);
                }
                fov
            };

            if target_fov > max_fov {
                trace!(target_fov, ?delta, "ExceededFOVThreshold");
                (AimAngles::default(), hitscan, aim_target)
            } else if delta[0].is_nan() || delta[1].is_nan() {
                tracing::error!(
                    ?delta,
                    ?delta_min,
                    ?delta_max,
                    ?view_angles,
                    ?calculated_angles_min,
                    ?calculated_angles_max,
                    ?sway_angles,
                    ?camera_origin
                );
                (AimAngles::default(), hitscan, aim_target)
            } else {
                (
                    AimAngles {
                        valid: true,
                        hitscan: hitscan.hit,
                        view_pitch: view_angles[0],
                        view_yaw: view_angles[1],
                        delta_pitch: delta[0],
                        delta_yaw: delta[1],
                        delta_pitch_min: delta_min[0],
                        delta_pitch_max: delta_max[0],
                        delta_yaw_min: delta_min[1],
                        delta_yaw_max: delta_max[1],
                        distance,
                    },
                    hitscan,
                    aim_target,
                )
            }
        } else {
            let target_origin = target.get_position();
            aim_target = target_origin;

            let target_angle = calc_angle(&view_origin, &target_origin);
            if target_angle[0].abs() > 80.0 {
                trace!("ExceededPitchThreshold");
                return (AimAngles::default(), hitscan, aim_target);
            }

            let skynade_angles = apexsky::ffi::skynade_angle(
                weapon_info.weapon_id.try_into().unwrap(),
                weapon_info.weapon_mod_bitfield,
                weapon_info.bullet_gravity / 750.0,
                weapon_info.bullet_speed,
                view_origin[0],
                view_origin[1],
                view_origin[2],
                target_origin[0],
                target_origin[1],
                target_origin[2],
            );

            trace!(?view_angles, ?skynade_angles);
            if !skynade_angles.w.is_normal() {
                return (AimAngles::default(), hitscan, aim_target);
            }

            let target_aim_angles = [
                -skynade_angles.x.to_degrees(),
                skynade_angles.y.to_degrees(),
                0.0,
            ];
            trace!(weapon = ?weapon_info, ?target_aim_angles);

            let mut delta = math::sub(target_aim_angles, view_angles);
            normalize_delta_angles(&mut delta);

            let aim_angles = if delta[0].is_nan() || delta[1].is_nan() || delta[2].is_nan() {
                tracing::error!(
                    ?delta,
                    ?target_aim_angles,
                    ?view_angles,
                    ?skynade_angles,
                    ?view_origin,
                    ?target_origin
                );
                AimAngles::default()
            } else {
                AimAngles {
                    valid: true,
                    hitscan: false,
                    view_pitch: view_angles[0],
                    view_yaw: view_angles[1],
                    delta_pitch: delta[0],
                    delta_yaw: delta[1],
                    delta_pitch_min: delta[0],
                    delta_pitch_max: delta[0],
                    delta_yaw_min: delta[1],
                    delta_yaw_max: delta[1],
                    distance,
                }
            };
            (aim_angles, hitscan, aim_target)
        }
    }

    #[instrument(skip_all)]
    fn hit_scan(
        &self,
        view_origin: [f32; 3],
        view_angles: [f32; 3],
        target_origin: [f32; 3],
        target_vel: [f32; 3],
        target_hitboxes: Vec<([f32; 3], ([f32; 3], [f32; 3]))>,
    ) -> HitScanReport {
        let max_time = 2.0;
        let time_step = 0.00005;
        let radius_scale = 1.0;

        let view_direction = math::qvec(view_angles);
        let bone_origin = target_origin;

        let hitbox_radius = |(bbmin, bbmax): ([f32; 3], [f32; 3])| -> f32 {
            let size = math::sub(bbmax, bbmin);
            let volume = math::dot(size, size);
            f32::cbrt(volume / (4.0 / 3.0 * std::f32::consts::PI))
        };

        let hitpoints = {
            let mut hitpoints = Vec::with_capacity(256);
            if target_hitboxes.len() < 256 {
                for (hit_pos, hb) in &target_hitboxes {
                    let hit_radius = hitbox_radius(*hb);
                    let bone_pos = math::add(bone_origin, *hit_pos);
                    let radius = hit_radius * radius_scale;
                    hitpoints.push((bone_pos, radius));
                }
            } else {
                for i in 0..256 {
                    let fi = i as f32 / 256.0 * target_hitboxes.len() as i32 as f32;
                    let starti = fi.floor() as i32 as usize;
                    let endi = fi.ceil() as i32 as usize;
                    let t = fi.fract();

                    let Some(start) = target_hitboxes.get(starti) else {
                        break;
                    };
                    let Some(end) = target_hitboxes.get(endi) else {
                        break;
                    };

                    let start_pos = math::add(bone_origin, start.0);
                    let end_pos = math::add(bone_origin, end.0);

                    let bone_pos = math::lerp(start_pos, end_pos, t);

                    let start_radius = hitbox_radius(start.1);
                    let end_radius = hitbox_radius(end.1);
                    let radius = (start_radius + (end_radius - start_radius) * t) * radius_scale;

                    hitpoints.push((bone_pos, radius));
                }
            }
            hitpoints
        };

        let mut hit = false;
        let mut nearest_hitbox = None;
        let mut nearest_bone_pos = None;

        // raycast
        let mut min_bone_offset = f32::MAX;
        for (bone_pos, radius) in &hitpoints {
            let dist2 = math::dist2(
                math::project(view_origin, view_direction, *bone_pos),
                *bone_pos,
            );
            let offset = dist2 - radius * radius;

            if offset < min_bone_offset {
                nearest_bone_pos = Some(*bone_pos);
                min_bone_offset = offset;
            }
        }

        if self.get_quick_looting_ready() {
            return HitScanReport {
                hit: min_bone_offset < 0.0,
                nearest_hitbox: None,
                nearest_bone_pos: None,
            };
        }

        // projectile
        if min_bone_offset < 40.0 * 40.0 {
            let target_hitboxes: Vec<_> = target_hitboxes
                .iter()
                .map(|(hit_pos, (bbmin, bbmax))| {
                    let bbmin = math::muls(*bbmin, 0.9);
                    let bbmax = math::muls(*bbmax, 0.9);
                    (*hit_pos, (bbmin, bbmax))
                })
                .collect();

            let mut nearest_hitbox_index = None;
            let mut min_bone_dist2 = f32::MAX;

            let weapon_info = self.get_weapon_info();
            let v0 = math::muls(view_direction, weapon_info.bullet_speed);
            let g = weapon_info.bullet_gravity;

            let mut time = 0.0;
            while time < max_time {
                let projectile_pos = [
                    view_origin[0] + v0[0] * time,
                    view_origin[1] + v0[1] * time,
                    view_origin[2] + v0[2] * time - 0.5 * g * time * time,
                ];
                let equivalent_pos = math::sub(projectile_pos, math::muls(target_vel, time));
                if math::dist2(equivalent_pos, bone_origin) < (2.0 * 40.0) * (2.0 * 40.0) {
                    for (i, (hit_pos, (bbmin, bbmax))) in target_hitboxes.iter().enumerate() {
                        let bone_pos = math::add(bone_origin, *hit_pos);
                        let bone_dist2 = math::dist2(equivalent_pos, bone_pos);
                        if bone_dist2 < min_bone_dist2 {
                            nearest_hitbox_index = Some(i);
                            min_bone_dist2 = bone_dist2;
                        }
                        let pos_offset = math::sub(equivalent_pos, bone_pos);
                        if bbmin[0] < pos_offset[0]
                            && pos_offset[0] < bbmax[0]
                            && bbmin[1] < pos_offset[1]
                            && pos_offset[1] < bbmax[1]
                            && bbmin[2] < pos_offset[2]
                            && pos_offset[2] < bbmax[2]
                        {
                            hit = true;
                            break;
                        }
                    }
                    if hit {
                        break;
                    }
                }
                time += time_step;
            }
            if let Some(hb) = nearest_hitbox_index.and_then(|i| target_hitboxes.get(i)) {
                let (bone_pos, (bbmin, bbmax)) = hb;
                let bone_pos = math::add(bone_origin, *bone_pos);
                nearest_hitbox = Some((bone_pos, (*bbmin, *bbmax)));
                nearest_bone_pos = Some(bone_pos);
            }
        }
        HitScanReport {
            hit,
            nearest_hitbox,
            nearest_bone_pos,
        }
    }
}

fn linear_predict(
    weapon_info: &CurrentWeaponInfo,
    pos_origin: [f32; 3],
    pos_target: [f32; 3],
    vel_target: [f32; 3],
) -> Option<(f32, f32)> {
    use crate::apexdream::base::solver::{solve, Collection, LinearPredictor, ProjectileWeapon};
    use crate::apexdream::sdk::projectiles;
    use crate::game::data::WeaponId;

    let projectile = match WeaponId(weapon_info.weapon_id) {
        WeaponId::Bow => Some(projectiles::BOCEK),
        WeaponId::Devotion => Some(projectiles::DEVOTION),
        WeaponId::Flatline => Some(projectiles::FLATLINE),
        WeaponId::G7Scout => Some(projectiles::G7_SCOUT),
        WeaponId::Hemlock => Some(projectiles::HEMLOK),
        WeaponId::Kraber => Some(projectiles::KRABER),
        WeaponId::Longbow => Some(projectiles::LONGBOW),
        WeaponId::Prowler => Some(projectiles::PROWLER),
        WeaponId::R301 => Some(projectiles::R301),
        WeaponId::_3030Repeater => Some(projectiles::REPEATER),
        WeaponId::Sentinel => Some(projectiles::SENTINEL),
        WeaponId::Spitfire => Some(projectiles::SPITFIRE),
        WeaponId::ThrowingKnife => Some(projectiles::THROWING_KNIFE),
        WeaponId::Volt => Some(projectiles::VOLT),
        WeaponId::Wingman => Some(projectiles::WINGMAN),
        _ => None,
    };

    struct Weapon<'a>(f32, f32, Option<Collection<'a>>);
    impl<'a> ProjectileWeapon for Weapon<'a> {
        fn projectile_speed(&self) -> f32 {
            self.0
        }
        fn projectile_gravity(&self) -> f32 {
            self.1
        }
        fn projectile_collection(&self) -> Option<Collection> {
            self.2
        }
    }

    let weapon = Weapon(
        weapon_info.bullet_speed,
        weapon_info.bullet_gravity,
        projectile,
    );

    let predictor = LinearPredictor {
        origin: pos_target,
        velocity: vel_target,
    };

    if let Some(sol) = solve(&pos_origin, &weapon, &predictor) {
        // let hit = predictor.predict_position(sol.time);
        let pitch = -sol.pitch.to_degrees();
        let yaw = sol.yaw.to_degrees();
        Some((pitch, yaw))
    } else {
        None
    }
}
