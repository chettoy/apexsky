use apexsky::aimbot::AimEntity;
use apexsky::pb::apexlegends::{Badge, GradeFlag, PlayerState};
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
        let love_state = apexsky::love_players::check_my_heart(
            config,
            state.platform_uid,
            state.eadp_uid,
            &player_name,
            state.entity_ptr.into_raw(),
        );
        let buf = PlayerState {
            origin: Some(state.origin.into()),
            view_angles: Some(state.view_angles.into()),
            velocity: Some(state.velocity.into()),
            health: state.health,
            shield: state.shields,
            max_health: state.max_health,
            max_shield: state.max_shields,
            helmet_type: state.helmet_type,
            armor_type: state.armor_type,
            team_num: state.team_num,
            xp: state.xp,
            flags: state.flags as i32,
            is_alive: state.is_alive(),
            is_knocked: state.is_knocked(),
            love_state: love_state as i32,
            active_weapon: active_weapon
                .as_ref()
                .map(|weap| weap.weapon_name_index)
                .unwrap_or(-1),
            player_name,
            platform_uid: state.platform_uid,
            eadp_uid: state.eadp_uid,
            head_position: Some(state.get_bone_position_by_hitbox(0).into()),
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
            grade_flags: state.grade,
            winning_team: false,
            yaw: state.yaw,
            team_member_index: state.team_member_index,
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

    fn get_bone_position_by_hitbox(&self, hitbox_id: u32) -> [f32; 3] {
        if self.studio.hitboxes.is_empty() {
            return self.origin;
        }

        let bone = self
            .studio
            .hitboxes
            .get(hitbox_id as usize)
            .map(|hitbox| hitbox.bone as usize)
            .unwrap_or_else(|| {
                tracing::error!(?hitbox_id, bone_head=self.studio.bone_head, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
                hitbox_id as usize
            });
        math::add(self.origin, self.bones.get_pos(bone))
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
            tracing::error!(?self);
        }
        self.health
    }

    #[tracing::instrument]
    fn get_shield_health(&self) -> i32 {
        if self.shields < 0 {
            tracing::error!(?self);
        }
        self.shields
    }

    #[tracing::instrument]
    fn get_max_health(&self) -> i32 {
        if self.max_health < 0 {
            tracing::error!(?self);
        }
        self.max_health
    }

    #[tracing::instrument]
    fn get_max_shield_health(&self) -> i32 {
        if self.max_shields < 0 {
            tracing::error!(?self);
        }
        self.max_shields
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

    fn get_bone_position_by_hitbox(&self, hitbox_id: u32) -> [f32; 3] {
        if self.studio.hitboxes.is_empty() {
            return self.origin;
        }

        let bone = self
            .studio
            .hitboxes
            .get(hitbox_id as usize)
            .map(|hitbox| hitbox.bone as usize)
            .unwrap_or_else(|| {
                tracing::error!(?hitbox_id, hitboxes=?self.studio.hitboxes, "{}", s!("invalid hitbox"));
                hitbox_id as usize
            });
        self.get_bone_pos(bone)
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
}
