use crate::aimbot::AimEntity;
use crate::apexdream::*;
use crate::pb::apexlegends::{PlayerState, Vec3};
use obfstr::obfstr as s;

use self::base::math;
use self::state::entities::{BaseNPCEntity, Entity, PlayerEntity, WeaponXEntity};
use self::state::GameState;

#[derive(Debug, Clone)]
pub struct GamePlayer {
    buf: PlayerState,
    state: PlayerEntity,
    active_weapon: Option<WeaponXEntity>,
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl GamePlayer {
    pub fn new(
        value: &PlayerEntity,
        game_state: &GameState,
        config: &mut crate::config::Config,
    ) -> Self {
        let state = value.clone();
        let active_weapon = value.active_weapon(game_state).map(|v| v.clone());
        let player_name = game_state
            .get_player_name(state.get_info().handle)
            .unwrap()
            .to_string();
        let love_state = crate::love_players::check_my_heart(
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
            character_index: 0,
            badges: vec![],
            kills: 0,
            damage_dealt: 0,
            kill_leader: false,
            winning_team: false,
        };
        Self {
            buf,
            state,
            active_weapon,
        }
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
}

impl crate::aimbot::AimEntity for PlayerEntity {
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

    fn get_bone_position_by_hitbox(&self, id: u32) -> [f32; 3] {
        let id = if id == 0 {
            self.studio.bone_head
        } else if id == 2 {
            self.studio.bone_body
        } else {
            id.try_into().unwrap()
        };
        math::add(self.origin, self.bones.get_pos(id as usize))
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

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_shield_health(&self) -> i32 {
        self.shields
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

impl crate::aimbot::AimEntity for BaseNPCEntity {
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

    fn get_bone_position_by_hitbox(&self, id: u32) -> [f32; 3] {
        let id = if id == 0 {
            self.studio.bone_head
        } else if id == 2 {
            self.studio.bone_body
        } else {
            id.try_into().unwrap()
        };
        self.get_bone_pos(id as usize)
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

    fn is_alive(&self) -> bool {
        self.is_alive()
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
