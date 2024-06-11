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
use obfstr::obfstr as s;

use parking_lot::RwLock;
use tokio::sync::watch;
use tokio::time::{sleep, sleep_until, Instant};
use tracing::{instrument, trace};

use crate::actuator::{
    AimActuator, AimbotAction, DeviceAimActuator, KmboxAimActuator, MemAimHelper, QmpAimActuator,
};
use crate::apexdream::base::math;
use crate::pb::apexlegends::{AimKeyState, AimTargetInfo};
use crate::SharedState;

const ENABLE_MEM_AIM: bool = true;

pub trait ContextForAimbot {
    async fn get_aimbot_settings(&self) -> Option<AimbotSettings>;
    async fn get_entity(&self, target_ptr: u64) -> Option<Arc<dyn AimEntity>>;
    async fn get_frame_count(&self) -> u32;
    async fn get_game_fps(&self) -> f32;
    async fn get_held_id(&self) -> Option<i32>;
    async fn get_weapon_info(&self) -> Option<CurrentWeaponInfo>;
    async fn is_world_ready(&self) -> bool;
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
    mut state: Arc<RwLock<SharedState>>,
    access_tx: MemApi,
    mut aim_key_rx: watch::Receiver<AimKeyState>,
    mut aim_select_rx: watch::Receiver<Vec<AimTargetInfo>>,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    let mut start_instant = Instant::now();
    let mut aimbot = Aimbot::default();
    let mut natural_delta_viewangles: [f32; 3] = [0.0, 0.0, 0.0];
    let mut prev_recoil_angle: [f32; 3] = [0.0, 0.0, 0.0];
    let mut prev_view_angles: Option<[f32; 3]> = None;
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
        if !state.is_world_ready().await {
            tracing::trace!("{}", s!("waiting for world ready"));
            start_instant += Duration::from_millis(500);
            sleep_until(start_instant).await;
            continue;
        }
        // Check base_addr and local_player_ptr
        {
            let rlock = state.read();
            mem_aim_helper.apex_base = rlock.game_baseaddr.unwrap_or(0);
            mem_aim_helper.lplayer_ptr = rlock
                .local_player
                .as_ref()
                .map(|e| e.get_entity().entity_ptr.into_raw())
                .unwrap_or(0);
        }
        if !mem_aim_helper.ready() {
            tracing::trace!("{}", s!("waiting for mem_aim_executer ready"));
            start_instant += Duration::from_millis(500);
            sleep_until(start_instant).await;
            continue;
        }
        // Check local_player entity
        let Some(local_entity) = state.get_entity(mem_aim_helper.lplayer_ptr).await else {
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
                    t.entity_ptr,
                );
            });
            aimbot.finish_select_target();
        }

        // Update aimbot settings
        // Lower update frequency to reduce cpu usage
        if state.get_frame_count().await % 30 == 0 {
            if let Some(aimbot_settings) = state.get_aimbot_settings().await {
                aimbot.settings(aimbot_settings);
                trace!("{}", s!("aimbot_settings reload"));
            }
        }

        // Update Aimbot state
        aimbot.update(mem_aim_helper.lplayer_ptr, state.get_game_fps().await);

        let aiming = aimbot.is_aiming();
        //tracing::trace!(?aiming, "711aac39-e83c-4788");

        let mut target_pos: Option<[f32; 3]> = None;
        let aim_result = {
            let aim_entity_ptr = aimbot.get_aim_entity();
            if aim_entity_ptr == 0 {
                aimbot.cancel_locking();
                AimAngles::default()
            } else if let Some(target_entity) = state.get_entity(aim_entity_ptr).await {
                target_pos = Some(target_entity.get_position());

                // // debug target entity
                // if !target_entity.is_player() {
                //     let is_visible = target_entity.is_visible();
                //     trace!(is_visible, ?target_entity, "{}", s!("711aac39-e83c-444b"));
                // }

                if !(aimbot.is_aiming() || aimbot.is_triggerbot_ready()) {
                    AimAngles::default()
                } else if aimbot.get_gun_safety() {
                    trace!("{}", s!("711aac39-e83c-4788 safety on"));
                    //println!("{:?}", target_entity);
                    AimAngles::default()
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
                    AimAngles::default()
                } else {
                    trace!("{}", s!("711aac39-e83c-4788 calc best aim"));
                    let (aim_angles, position) =
                        aimbot.calc_best_aim(&*local_entity, &*target_entity);
                    trace!(?aim_angles, "{}", s!("711aac39-e83c-4788 best aim"));
                    target_pos = Some(position);
                    aim_angles
                }
            } else {
                //tracing::warn!(aim_entity_ptr, "{}", s!("targeted entity does not exist"));
                tracing::debug!(
                    aim_entity_ptr,
                    "{}",
                    s!("711aac39-e83c-4788 targeted entity does not exist")
                );
                aimbot.cancel_locking();
                AimAngles::default()
            }
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
        aimbot.triggerbot_update(&aim_result, aim_key_rx.borrow().attack_state);
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
                let natural_delta = natural_delta_viewangles;
                //println!("{:?}", natural_delta);

                fn check(natural_delta: f32, smoothed_delta: f32) -> f32 {
                    if !natural_delta.is_normal() {
                        return 0.0;
                    }
                    if (smoothed_delta.signum() * natural_delta.signum()).is_sign_positive() {
                        smoothed_delta.abs().min(natural_delta.abs() * 7.0)
                            * smoothed_delta.signum()
                    } else {
                        smoothed_delta.abs().min(natural_delta.abs() * 0.7)
                            * smoothed_delta.signum()
                    }
                }
                let assist_delta = [
                    check(natural_delta[0], smoothed_delta_angles[0]),
                    check(natural_delta[1], smoothed_delta_angles[1]),
                    smoothed_delta_angles[2],
                ];
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
        };

        // Update state for ESP
        state.write().aimbot_state = Some((aimbot.clone(), loop_duration));

        // Read view_angles
        let view_angles =
            match MemAimHelper::read_viewangles(&access_tx, mem_aim_helper.lplayer_ptr).await {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!(%e, "{}", s!("err read viewangles"));
                    continue;
                }
            };
        if !(view_angles[0].is_finite() && view_angles[1].is_finite() && view_angles[2].is_finite())
        {
            tracing::warn!(?view_angles, "{}", s!("got invalid view_angles"));
            continue;
        }

        // Calc delta_view_angles
        natural_delta_viewangles = prev_view_angles
            .map(|prev| math::sub(view_angles, prev))
            .unwrap_or([0.0, 0.0, 0.0]);

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
