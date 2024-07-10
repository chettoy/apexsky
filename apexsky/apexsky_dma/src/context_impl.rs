use apexsky::{aimbot::AimEntity, global_state::G_STATE};
use apexsky_proto::pb::apexlegends::TreasureClue;
use std::sync::Arc;
use tracing::instrument;

use crate::{
    apexdream::base::solver::ProjectileWeapon,
    game::{data::WeaponId, player::GamePlayer},
    workers::aim::ContextForAimbot,
    SharedState, SharedStateWrapper,
};

impl SharedState {
    pub fn get_game_baseaddr(&self) -> Option<u64> {
        match self
            .game_baseaddr
            .load(std::sync::atomic::Ordering::Acquire)
        {
            0 => None,
            v => Some(v),
        }
    }

    pub fn read_cached_player(&self, ptr: &u64) -> Option<Arc<GamePlayer>> {
        self.players.read().get(ptr).map(|v| v.clone())
    }

    pub fn read_cached_npc(&self, ptr: &u64) -> Option<Arc<dyn AimEntity>> {
        self.npcs.read().get(ptr).map(|v| v.clone())
    }

    pub fn read_cached_loot(&self, ptr: &u64) -> Option<TreasureClue> {
        self.treasure_clues.read().get(ptr).map(|v| v.clone())
    }

    pub fn read_cached_aim_entity(&self, ptr: &u64) -> Option<Arc<dyn AimEntity>> {
        self.aim_entities.read().get(ptr).map(|v| v.clone())
    }

    pub fn get_local_player_ptr(&self) -> Option<u64> {
        match self
            .local_player_ptr
            .load(std::sync::atomic::Ordering::Acquire)
        {
            0 => None,
            v => Some(v),
        }
    }

    pub fn get_view_player_ptr(&self) -> Option<u64> {
        match self
            .view_player_ptr
            .load(std::sync::atomic::Ordering::Acquire)
        {
            0 => None,
            v => Some(v),
        }
    }

    pub fn is_world_ready(&self) -> bool {
        self.world_ready.load(std::sync::atomic::Ordering::Acquire)
    }

    #[instrument]
    pub fn get_frame_count(&self) -> u32 {
        let frame_count = self.frame_count.load(std::sync::atomic::Ordering::Acquire);
        frame_count.try_into().unwrap_or_else(|e| {
            tracing::error!(%e, ?e, frame_count);
            0
        })
    }

    #[instrument]
    pub fn get_game_fps(&self) -> f32 {
        self.game_fps.lock().to_owned()
    }
}

impl ContextForAimbot for SharedStateWrapper {
    #[instrument]
    async fn get_aimbot_settings(&self) -> Option<apexsky::aimbot::AimbotSettings> {
        G_STATE
            .lock()
            .map(|g_state| Some(g_state.config.settings.aimbot_settings.clone()))
            .unwrap_or_else(|e| {
                tracing::error!(%e, ?e);
                None
            })
    }

    #[instrument]
    async fn get_entity(&self, target_ptr: u64) -> Option<Arc<dyn AimEntity>> {
        self.aim_entities.read().get(&target_ptr).cloned()
    }

    #[instrument]
    async fn get_held_id(&self) -> Option<i32> {
        self.get_local_player_ptr()
            .and_then(|ptr| self.read_cached_player(&ptr))
            .as_ref()
            .map(|p| p.get_entity().selected_slot as i32)
    }

    #[instrument]
    async fn get_weapon_info(&self) -> Option<apexsky::aimbot::CurrentWeaponInfo> {
        let weapon = self
            .get_local_player_ptr()
            .and_then(|ptr| self.read_cached_player(&ptr))?
            .get_active_weapon()?
            .clone();

        let mut weapon_info = apexsky::aimbot::CurrentWeaponInfo::default();
        weapon_info.weapon_id = weapon.weapon_name_index;
        weapon_info.bullet_speed = weapon.projectile_speed();
        weapon_info.bullet_gravity = weapon.projectile_gravity();
        weapon_info.weapon_zoom_fov = weapon.cur_zoom_fov;
        weapon_info.weapon_mod_bitfield = weapon.mod_bitfield;
        weapon_info.weapon_headshot = {
            match weapon_info.weapon_id {
                IDWEAPON_3030_REPEATER => true,
                IDWEAPON_BOW => true,
                IDWEAPON_CHARGE_RIFLE => true,
                IDWEAPON_G7_SCOUT => true,
                IDWEAPON_KRABER => true,
                IDWEAPON_LONGBOW => true,
                IDWEAPON_SENTINEL => true,
                IDWEAPON_P2020 => false,
                IDWEAPON_TRIPLE_TAKE => true,
                IDWEAPON_WINGMAN => true,
                _ => false,
            }
        };
        weapon_info.weapon_semi_auto = {
            match weapon_info.weapon_id {
                IDWEAPON_3030_REPEATER => true,
                IDWEAPON_BOW => true,
                IDWEAPON_CHARGE_RIFLE => false,
                IDWEAPON_G7_SCOUT => true,
                IDWEAPON_HEMLOCK => true,
                IDWEAPON_KRABER => false,
                IDWEAPON_LONGBOW => true,
                IDWEAPON_SENTINEL => false,
                IDWEAPON_P2020 => true,
                IDWEAPON_TRIPLE_TAKE => true,
                IDWEAPON_WINGMAN => true,
                IDWEAPON_HANDS => true,
                THROWING_KNIFE => true,
                GRENADE_THERMITE => false,
                GRENADE_FRAG => false,
                GRENADE_ARC_STAR => false,
                _ => weapon.is_semi_auto,
            }
        };

        // if weapon.is_semi_auto != weapon_info.weapon_semi_auto {
        //     tracing::debug!(?weapon.is_semi_auto, ?weapon_info);
        // }

        Some(weapon_info)
    }

    #[instrument]
    async fn update_aim_target_for_esp(&mut self, position: [f32; 3]) {
        *self.aim_target.lock() = position;
    }
}

const IDWEAPON_SENTINEL: i32 = WeaponId::Sentinel as i32;
const IDWEAPON_BOW: i32 = WeaponId::Bow as i32;
const IDWEAPON_CHARGE_RIFLE: i32 = WeaponId::ChargeRifle as i32;
const IDWEAPON_LONGBOW: i32 = WeaponId::Longbow as i32;
const IDWEAPON_G7_SCOUT: i32 = WeaponId::G7Scout as i32;
const IDWEAPON_HEMLOCK: i32 = WeaponId::Hemlock as i32;
const IDWEAPON_KRABER: i32 = WeaponId::Kraber as i32;
const IDWEAPON_P2020: i32 = WeaponId::P2020 as i32;
const IDWEAPON_TRIPLE_TAKE: i32 = WeaponId::TripleTake as i32;
const IDWEAPON_WINGMAN: i32 = WeaponId::Wingman as i32;
const IDWEAPON_3030_REPEATER: i32 = WeaponId::_3030Repeater as i32;
const IDWEAPON_HANDS: i32 = WeaponId::Hands as i32;
const THROWING_KNIFE: i32 = WeaponId::ThrowingKnife as i32;
const GRENADE_THERMITE: i32 = WeaponId::GrenadeThermite as i32;
const GRENADE_FRAG: i32 = WeaponId::GrenadeFrag as i32;
const GRENADE_ARC_STAR: i32 = WeaponId::GrenadeArcStar as i32;
