use apexsky::{
    aimbot::AimEntity,
    apexdream::base::solver::ProjectileWeapon,
    games::apex::data::WeaponId,
    global_state::G_STATE,
    mem::{MemProc, MemProcImpl},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::{
    workers::{actions::MemAccess, aim::ContextForAimbot},
    SharedState,
};

impl<'a> MemAccess for MemProcImpl<'a> {
    #[inline]
    fn apex_mem_baseaddr(&mut self) -> u64 {
        self.get_proc_baseaddr()
    }

    #[inline]
    #[instrument(skip_all)]
    fn apex_mem_read<T: dataview::Pod + Sized + Default>(
        &mut self,
        offset: u64,
    ) -> anyhow::Result<T> {
        let mut v: T = T::default();
        self.read_into(offset.into(), &mut v)?;
        Ok(v)
    }

    #[inline]
    #[instrument(skip_all)]
    fn apex_mem_write<T: dataview::Pod + ?Sized>(
        &mut self,
        offset: u64,
        data: &T,
    ) -> anyhow::Result<()> {
        self.write(offset, data)?;
        Ok(())
    }
}

impl ContextForAimbot for Arc<Mutex<SharedState>> {
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
        self.lock().await.aim_entities.get(&target_ptr).cloned()
    }

    #[instrument]
    async fn get_frame_count(&self) -> u32 {
        let frame_count = self.lock().await.frame_count;
        frame_count.try_into().unwrap_or_else(|e| {
            tracing::error!(%e, ?e, frame_count);
            0
        })
    }

    #[instrument]
    async fn get_game_fps(&self) -> f32 {
        self.lock().await.game_fps
    }

    #[instrument]
    async fn get_held_id(&self) -> Option<i32> {
        self.lock()
            .await
            .local_player
            .as_ref()
            .map(|p| p.get_entity().selected_slot as i32)
    }

    #[instrument]
    async fn get_player_ptr(&self) -> u64 {
        self.lock()
            .await
            .local_player
            .as_ref()
            .map(|e| e.get_entity().entity_ptr.into_raw())
            .unwrap_or(0)
    }

    #[instrument]
    async fn get_weapon_info(&self) -> Option<apexsky::aimbot::CurrentWeaponInfo> {
        let state = self.lock().await;
        let player = state.local_player.as_ref()?;
        player.get_active_weapon().map(|weapon| {
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

            weapon_info
        })
    }

    #[instrument]
    async fn is_world_ready(&self) -> bool {
        self.lock().await.world_ready
    }

    #[instrument]
    async fn update_aim_target_for_esp(&mut self, position: [f32; 3]) {
        self.lock().await.aim_target = position;
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
