use std::vec;

use apexsky::aimbot::{get_unix_timestamp_in_millis, AimEntity, HitboxData};
use apexsky_proto::pb::apexlegends::{Badge, GradeFlag, PlayerState, TreasureClue};
use obfstr::obfstr as s;

use crate::apexdream::sdk::ScriptNetVarName;
use crate::apexdream::*;

use self::base::math;
use self::state::entities::{BaseNPCEntity, Entity, PlayerEntity, WeaponXEntity};
use self::state::GameState;

#[derive(Debug, Clone)]
pub struct GamePlayer {
    buf: PlayerState,
    state: PlayerEntity,
    active_weapon: Option<WeaponXEntity>,
}

impl GamePlayer {
    pub fn new(
        value: PlayerEntity,
        game_state: &GameState,
        config: &mut apexsky::config::Config,
    ) -> Self {
        let state = value;
        let active_weapon = state.active_weapon(game_state).map(|v| v.clone());
        let player_name = game_state
            .get_player_name(state.get_info().handle)
            .unwrap()
            .to_string();
        let love_status = apexsky::love_players::check_my_heart(
            config,
            state.platform_uid,
            state.eadp_uid,
            &player_name,
            state.entity_ptr.into_raw(),
        );
        let buf = PlayerState {
            origin: Some(state.origin.into()),
            velocity: Some(state.velocity.into()),
            accel: Some(state.accel.into()),
            health: state.get_health(),
            shield: state.get_shield_health(),
            max_health: state.get_max_health(),
            max_shield: state.get_max_shield_health(),
            max_speed: state.max_speed,
            shadow_shield_active: state.shadow_shield_active,
            temp_shield_health: state.temp_shield_health,
            extra_shield_health: state.extra_shield_health,
            extra_shield_tier: state.extra_shield_tier,
            is_performing_boost_action: state.is_performing_boost_action,
            helmet_type: state.helmet_type,
            armor_type: state.armor_type,
            team_num: state.team_num,
            team_member_index: state.team_member_index,
            squad_id: state.squad_id,
            grade_flags: state.grade,
            model_name: state.model_name.string.to_owned(),
            head_position: Some(state.get_bone_position_by_hitbox(0).into()),
            camera_origin: Some(state.camera_origin.into()),
            camera_angles: Some(state.camera_angles.into()),
            time_base: state.time_base,
            server_angles: Some(state.server_angles.into()),
            view_offset: Some(state.view_offset.into()),
            view_origin: Some(state.view_origin.into()),
            view_angles: Some(state.view_angles.into()),
            xp: state.xp,
            flags: state.flags as i32,
            is_alive: state.is_alive(),
            is_knocked: state.is_knocked(),
            love_status: love_status as i32,
            active_weapon: active_weapon
                .as_ref()
                .map(|weap| weap.weapon_name_index)
                .unwrap_or(-1),
            player_name,
            platform_uid: state.platform_uid,
            eadp_uid: state.eadp_uid,
            controller_active: state.controller_active == 1,
            character_index: game_state
                .read_script_value(
                    ScriptNetVarName::characterIndex,
                    state.script_net_data_global,
                )
                .to_int()
                .unwrap_or(-1),
            badges: [
                (
                    ScriptNetVarName::firstBadgeIndex,
                    ScriptNetVarName::firstBadgeDataInt,
                ),
                (
                    ScriptNetVarName::secondBadgeIndex,
                    ScriptNetVarName::secondBadgeDataInt,
                ),
                (
                    ScriptNetVarName::thirdBadgeIndex,
                    ScriptNetVarName::thirdBadgeDataInt,
                ),
            ]
            .into_iter()
            .map(|(netvar_badge_index, netvar_badge_data)| Badge {
                badge_index: game_state
                    .read_script_value(netvar_badge_index, state.script_net_data_global)
                    .to_int()
                    .unwrap_or(-1),
                badge_data: game_state
                    .read_script_value(netvar_badge_data, state.script_net_data_global)
                    .to_int()
                    .unwrap_or(-1),
            })
            .collect(),
            kills: game_state
                .read_script_value(ScriptNetVarName::kills, state.script_net_data_global)
                .to_word()
                .map(|x| x as i32)
                .unwrap_or(-1),
            damage_dealt: game_state
                .read_script_value(ScriptNetVarName::damageDealt, state.script_net_data_global)
                .to_int()
                .unwrap_or(-1),
            winning_team: false,
            yaw: state.yaw,
            skydive_state: state.skydive_state,
        };

        Self {
            buf,
            state,
            active_weapon,
        }
    }

    pub fn update_buf_hotdata(&mut self, value: &PlayerEntity) {
        let state = value;

        let buf = &mut self.buf;
        buf.origin = Some(state.origin.into());
        buf.view_angles = Some(state.view_angles.into());
        buf.velocity = Some(state.velocity.into());
        buf.yaw = state.yaw;
        buf.health = state.health;
        buf.shield = state.shields;
        buf.flags = state.flags as i32;
        buf.is_alive = state.is_alive();
        buf.is_knocked = state.is_knocked();
        buf.head_position = Some(state.get_bone_position_by_hitbox(0).into());

        self.state = state.clone();
    }

    pub fn get_buf(&self) -> &PlayerState {
        &self.buf
    }

    pub fn get_entity(&self) -> &PlayerEntity {
        &self.state
    }

    pub fn get_active_weapon(&self) -> Option<&WeaponXEntity> {
        self.active_weapon.as_ref()
    }

    pub fn is_kill_leader(buf: &PlayerState) -> bool {
        buf.grade_flags & GradeFlag::Killleader as i32 != 0
            || buf.grade_flags & GradeFlag::ChampKillleader as i32 != 0
    }
}

impl apexsky::aimbot::AimEntity for PlayerEntity {
    fn get_entity_ptr(&self) -> u64 {
        self.entity_ptr.into_raw()
    }

    fn get_view_angles(&self) -> [f32; 3] {
        self.view_angles
    }

    fn get_cam_pos(&self) -> [f32; 3] {
        self.camera_origin
    }

    fn get_sway_angles(&self) -> [f32; 3] {
        self.breath_angles
    }

    fn get_abs_velocity(&self) -> [f32; 3] {
        self.velocity
    }

    #[tracing::instrument]
    fn get_bone_position_by_hitbox(&self, hitbox_id: u32) -> [f32; 3] {
        if self.studio.hitboxes.is_empty() {
            tracing::debug!(?hitbox_id, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
        }

        if let Some(bone) = self
            .studio
            .hitboxes
            .get(hitbox_id as usize)
            .map(|hitbox| hitbox.bone as usize)
        {
            math::add(self.origin, self.bones.get_pos(bone))
        } else {
            tracing::debug!(?hitbox_id, bone_head=self.studio.bone_head, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
            // fallback
            if hitbox_id == 0 {
                self.view_origin
            } else {
                self.origin
            }
        }
    }

    #[tracing::instrument]
    fn get_hitbox(&self) -> Vec<([f32; 3], ([f32; 3], [f32; 3]))> {
        self.studio
            .hitboxes
            .iter()
            .filter_map(|bbox| {
                self.bones.v.get(bbox.bone as usize).and_then(|matrix| {
                    Some(([matrix[3], matrix[7], matrix[11]], (bbox.bbmin, bbox.bbmax)))
                })
            })
            .collect()
    }

    #[tracing::instrument]
    fn get_bones_data(&self) -> Vec<HitboxData> {
        // self.studio
        //     .hitboxes
        //     .iter()
        //     .filter_map(|bbox| {
        //         self.bones.v.get(bbox.bone as usize).and_then(|matrix| {
        //             Some(HitboxData {
        //                 bone: bbox.bone as i32,
        //                 group: bbox.group as i32,
        //                 bbmin: bbox.bbmin,
        //                 bbmax: bbox.bbmax,
        //                 bone_origin: [matrix[3], matrix[7], matrix[11]],
        //                 bone_parent: self.studio.bones.get(bbox.bone as usize)?.parent as i32 - 1,
        //                 radius: bbox.radius(),
        //             })
        //         })
        //     })
        //     .collect()
        self.studio
            .bones
            .iter()
            .enumerate()
            .filter_map(|(bone, bbone)| {
                let matrix = self.bones.v.get(bone)?;
                if let Some(bbox) = self
                    .studio
                    .hb_lookup
                    .get(bone)
                    .and_then(|&hitbox_idx| self.studio.hitboxes.get(hitbox_idx as usize))
                {
                    Some(HitboxData {
                        bone: bone.try_into().unwrap(),
                        group: bbox.group as i32,
                        bbmin: bbox.bbmin,
                        bbmax: bbox.bbmax,
                        bone_origin: [matrix[3], matrix[7], matrix[11]],
                        bone_parent: bbone.parent as i32 - 1,
                        radius: bbox.radius(),
                    })
                } else {
                    Some(HitboxData {
                        bone: bone.try_into().unwrap(),
                        group: sdk::HITGROUP_GENERIC as i32,
                        bbmin: [0.0, 0.0, 0.0],
                        bbmax: [0.0, 0.0, 0.0],
                        bone_origin: [matrix[3], matrix[7], matrix[11]],
                        bone_parent: bbone.parent as i32 - 1,
                        radius: 0.0,
                    })
                }
            })
            .collect()
    }

    fn get_position(&self) -> [f32; 3] {
        self.origin
    }

    fn get_view_offset(&self) -> [f32; 3] {
        self.view_offset
    }

    fn get_recoil_angles(&self) -> [f32; 3] {
        self.weapon_punch
    }

    fn get_team_num(&self) -> i32 {
        self.team_num
    }

    #[tracing::instrument]
    fn get_health(&self) -> i32 {
        if self.health < 0 {
            tracing::debug!(?self);
            return 0;
        }
        self.health
    }

    #[tracing::instrument]
    fn get_shield_health(&self) -> i32 {
        if self.shields < 0 {
            tracing::debug!(?self);
            return 0;
        }
        self.shields + self.temp_shield_health + self.extra_shield_health
    }

    #[tracing::instrument]
    fn get_max_health(&self) -> i32 {
        if self.max_health < 0 {
            tracing::debug!(?self);
            return 0;
        }
        self.max_health
    }

    #[tracing::instrument]
    fn get_max_shield_health(&self) -> i32 {
        if self.max_shields < 0 {
            tracing::debug!(?self);
            return 0;
        }
        self.max_shields.max(self.get_shield_health())
    }

    fn get_visible_duration(&self) -> f64 {
        if self.is_visible {
            get_unix_timestamp_in_millis() as f64 / 1000.0 - self.visible_time
        } else {
            0.0
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive()
    }

    fn is_knocked(&self) -> bool {
        self.is_downed()
    }

    fn is_player(&self) -> bool {
        true
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn is_loot(&self) -> bool {
        false
    }
}

impl apexsky::aimbot::AimEntity for BaseNPCEntity {
    fn get_entity_ptr(&self) -> u64 {
        self.entity_ptr.into_raw()
    }

    fn get_view_angles(&self) -> [f32; 3] {
        self.angles
    }

    fn get_cam_pos(&self) -> [f32; 3] {
        self.get_bone_position_by_hitbox(0)
    }

    fn get_sway_angles(&self) -> [f32; 3] {
        self.get_view_angles()
    }

    fn get_abs_velocity(&self) -> [f32; 3] {
        self.velocity
    }

    #[tracing::instrument]
    fn get_bone_position_by_hitbox(&self, hitbox_id: u32) -> [f32; 3] {
        if self.studio.hitboxes.is_empty() {
            tracing::debug!(?hitbox_id, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
        }

        if let Some(bone) = self
            .studio
            .hitboxes
            .get(hitbox_id as usize)
            .map(|hitbox| hitbox.bone as usize)
        {
            self.get_bone_pos(bone)
        } else {
            tracing::debug!(?hitbox_id, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
            // fallback
            self.origin
        }
    }

    #[tracing::instrument]
    fn get_hitbox(&self) -> Vec<([f32; 3], ([f32; 3], [f32; 3]))> {
        self.studio
            .hitboxes
            .iter()
            .filter_map(|bbox| {
                self.bones.v.get(bbox.bone as usize).and_then(|matrix| {
                    Some(([matrix[3], matrix[7], matrix[11]], (bbox.bbmin, bbox.bbmax)))
                })
            })
            .collect()
    }

    #[tracing::instrument]
    fn get_bones_data(&self) -> Vec<HitboxData> {
        self.studio
            .bones
            .iter()
            .enumerate()
            .filter_map(|(bone, bbone)| {
                let matrix = self.bones.v.get(bone)?;
                if let Some(bbox) = self
                    .studio
                    .hb_lookup
                    .get(bone)
                    .and_then(|&hitbox_idx| self.studio.hitboxes.get(hitbox_idx as usize))
                {
                    Some(HitboxData {
                        bone: bone.try_into().unwrap(),
                        group: bbox.group as i32,
                        bbmin: bbox.bbmin,
                        bbmax: bbox.bbmax,
                        bone_origin: [matrix[3], matrix[7], matrix[11]],
                        bone_parent: bbone.parent as i32 - 1,
                        radius: bbox.radius(),
                    })
                } else {
                    Some(HitboxData {
                        bone: bone.try_into().unwrap(),
                        group: sdk::HITGROUP_GENERIC as i32,
                        bbmin: [0.0, 0.0, 0.0],
                        bbmax: [0.0, 0.0, 0.0],
                        bone_origin: [matrix[3], matrix[7], matrix[11]],
                        bone_parent: bbone.parent as i32 - 1,
                        radius: 0.0,
                    })
                }
            })
            .collect()
    }

    fn get_position(&self) -> [f32; 3] {
        self.origin
    }

    fn get_view_offset(&self) -> [f32; 3] {
        math::sub(self.get_bone_position_by_hitbox(0), self.origin)
    }

    fn get_recoil_angles(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    fn get_team_num(&self) -> i32 {
        self.team_num
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_shield_health(&self) -> i32 {
        self.shields
    }

    fn get_max_health(&self) -> i32 {
        self.max_health
    }

    fn get_max_shield_health(&self) -> i32 {
        self.max_shields
    }

    fn get_visible_duration(&self) -> f64 {
        if self.is_visible {
            get_unix_timestamp_in_millis() as f64 / 1000.0 - self.visible_time
        } else {
            0.0
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive() && self.get_health() > 0
    }

    fn is_knocked(&self) -> bool {
        false
    }

    fn is_player(&self) -> bool {
        false
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn is_loot(&self) -> bool {
        false
    }
}

impl AimEntity for GamePlayer {
    fn get_entity_ptr(&self) -> u64 {
        self.get_entity().get_entity_ptr()
    }

    fn get_view_angles(&self) -> [f32; 3] {
        self.get_entity().get_view_angles()
    }

    fn get_cam_pos(&self) -> [f32; 3] {
        self.get_entity().get_cam_pos()
    }

    fn get_sway_angles(&self) -> [f32; 3] {
        self.get_entity().get_sway_angles()
    }

    fn get_abs_velocity(&self) -> [f32; 3] {
        self.get_entity().get_abs_velocity()
    }

    fn get_bone_position_by_hitbox(&self, id: u32) -> [f32; 3] {
        self.get_entity().get_bone_position_by_hitbox(id)
    }

    fn get_bones_data(&self) -> Vec<HitboxData> {
        self.get_entity().get_bones_data()
    }

    fn get_hitbox(&self) -> Vec<([f32; 3], ([f32; 3], [f32; 3]))> {
        self.get_entity().get_hitbox()
    }

    fn get_position(&self) -> [f32; 3] {
        self.get_entity().get_position()
    }

    fn get_recoil_angles(&self) -> [f32; 3] {
        self.get_entity().get_recoil_angles()
    }

    fn get_view_offset(&self) -> [f32; 3] {
        self.get_entity().get_view_offset()
    }

    fn get_team_num(&self) -> i32 {
        self.get_entity().get_team_num()
    }

    fn get_health(&self) -> i32 {
        self.get_entity().get_health()
    }

    fn get_shield_health(&self) -> i32 {
        self.get_entity().get_shield_health()
    }

    fn get_max_health(&self) -> i32 {
        self.get_entity().get_max_health()
    }

    fn get_max_shield_health(&self) -> i32 {
        self.get_entity().get_max_shield_health()
    }

    fn get_visible_duration(&self) -> f64 {
        self.get_entity().get_visible_duration()
    }

    fn is_alive(&self) -> bool {
        self.get_entity().is_alive()
    }

    fn is_knocked(&self) -> bool {
        self.get_entity().is_knocked()
    }

    fn is_player(&self) -> bool {
        self.get_entity().is_player()
    }

    fn is_visible(&self) -> bool {
        self.get_entity().is_visible()
    }

    fn is_loot(&self) -> bool {
        self.get_entity().is_loot()
    }
}

#[derive(Debug, Clone)]
pub struct QuickLooting(pub TreasureClue);

impl AimEntity for QuickLooting {
    fn get_entity_ptr(&self) -> u64 {
        self.0.entity_handle
    }

    fn get_view_angles(&self) -> [f32; 3] {
        Default::default()
    }

    fn get_cam_pos(&self) -> [f32; 3] {
        self.get_position()
    }

    fn get_sway_angles(&self) -> [f32; 3] {
        Default::default()
    }

    fn get_abs_velocity(&self) -> [f32; 3] {
        Default::default()
    }

    fn get_bone_position_by_hitbox(&self, _id: u32) -> [f32; 3] {
        self.get_position()
    }

    fn get_bones_data(&self) -> Vec<HitboxData> {
        vec![HitboxData {
            bone: 0,
            group: 0,
            bbmin: [-6.0, -6.0, -6.0],
            bbmax: [6.0, 6.0, 6.0],
            bone_origin: [0.0, 0.0, 0.0],
            bone_parent: 0,
            radius: f32::sqrt(6.0 * 6.0 * 3.0),
        }]
    }

    fn get_hitbox(&self) -> Vec<([f32; 3], ([f32; 3], [f32; 3]))> {
        vec![([0.0, 0.0, 0.0], ([-6.0, -6.0, -6.0], [6.0, 6.0, 6.0]))]
    }

    fn get_position(&self) -> [f32; 3] {
        self.0
            .position
            .clone()
            .map(|pos| pos.into())
            .unwrap_or_default()
    }

    fn get_recoil_angles(&self) -> [f32; 3] {
        Default::default()
    }

    fn get_view_offset(&self) -> [f32; 3] {
        Default::default()
    }

    fn get_team_num(&self) -> i32 {
        Default::default()
    }

    fn get_health(&self) -> i32 {
        Default::default()
    }

    fn get_shield_health(&self) -> i32 {
        Default::default()
    }

    fn get_max_health(&self) -> i32 {
        Default::default()
    }

    fn get_max_shield_health(&self) -> i32 {
        Default::default()
    }

    fn get_visible_duration(&self) -> f64 {
        200.0
    }

    fn is_alive(&self) -> bool {
        true
    }

    fn is_knocked(&self) -> bool {
        false
    }

    fn is_player(&self) -> bool {
        false
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn is_loot(&self) -> bool {
        true
    }
}
