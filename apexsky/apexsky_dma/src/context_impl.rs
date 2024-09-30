use apexsky::{
    aimbot::{AimAngles, AimEntity, HitScanReport},
    global_state::G_STATE,
};
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
            match WeaponId(weapon_info.weapon_id) {
                WeaponId::_3030Repeater => true,
                WeaponId::Bow => true,
                WeaponId::ChargeRifle => true,
                WeaponId::G7Scout => true,
                WeaponId::Kraber => true,
                WeaponId::Longbow => true,
                WeaponId::Sentinel => true,
                WeaponId::P2020 => false,
                WeaponId::TripleTake => true,
                WeaponId::Wingman => true,
                _ => false,
            }
        };
        weapon_info.weapon_semi_auto = {
            match WeaponId(weapon_info.weapon_id) {
                WeaponId::_3030Repeater => true,
                WeaponId::Bow => true,
                WeaponId::ChargeRifle => false,
                WeaponId::G7Scout => true,
                WeaponId::Hemlock => true,
                WeaponId::Kraber => false,
                WeaponId::Longbow => true,
                WeaponId::Sentinel => false,
                WeaponId::P2020 => true,
                WeaponId::TripleTake => true,
                WeaponId::Wingman => true,
                WeaponId::Hands => true,
                WeaponId::ThrowingKnife => true,
                WeaponId::GrenadeThermite => false,
                WeaponId::GrenadeFrag => false,
                WeaponId::GrenadeArcStar => false,
                _ => weapon.is_semi_auto,
            }
        };

        // if weapon.is_semi_auto != weapon_info.weapon_semi_auto {
        //     tracing::debug!(?weapon.is_semi_auto, ?weapon_info);
        // }

        Some(weapon_info)
    }

    #[instrument]
    async fn update_aim_target_for_esp(
        &mut self,
        aim_result: AimAngles,
        hitscan_result: Option<HitScanReport>,
        target_pos: Option<[f32; 3]>,
    ) {
        *self.aim_target.lock() = (aim_result, hitscan_result, target_pos);
    }
}
