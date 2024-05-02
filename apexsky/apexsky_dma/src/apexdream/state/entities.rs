#![allow(dead_code)]

use super::*;
use apexsky::noobfstr as s;
use async_trait::async_trait;
use std::any::Any;

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityInfo {
    pub entity_ptr: sdk::Ptr,
    pub index: usize,
    pub handle: sdk::EHandle,
    pub rate: u32,
}

#[async_trait]
pub trait Entity: Any + Sync + Send + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_ref(&self) -> EntityRef<'_>;
    fn is_serialized(&self) -> bool;
    fn get_info(&self) -> EntityInfo;
    async fn update(&mut self, api: &Api, ctx: &UpdateContext);
    fn post(&mut self, _api: &Api, _ctx: &UpdateContext, _state: &GameState) {}
}

#[derive(Copy, Clone)]
pub enum EntityRef<'a> {
    BaseEntity(&'a BaseEntity),
    BaseNPC(&'a BaseNPCEntity),
    World(&'a WorldEntity),
    Player(&'a PlayerEntity),
    WeaponX(&'a WeaponXEntity),
    Loot(&'a LootEntity),
    Waypoint(&'a WaypointEntity),
    Vehicle(&'a VehicleEntity),
    Deathbox(&'a DeathboxEntity),
    Animating(&'a AnimatingEntity),
    Projectile(&'a ProjectileEntity),
    ScriptNetData(&'a ScriptNetDataEntity),
}
impl EntityRef<'_> {
    pub fn get_type_name(self) -> String {
        match self {
            EntityRef::BaseEntity(_) => s!("BaseEntity").to_string(),
            EntityRef::BaseNPC(_) => s!("BaseNPC").to_string(),
            EntityRef::World(_) => s!("World").to_string(),
            EntityRef::Player(_) => s!("Player").to_string(),
            EntityRef::WeaponX(_) => s!("WeaponX").to_string(),
            EntityRef::Loot(_) => s!("Loot").to_string(),
            EntityRef::Waypoint(_) => s!("Waypoint").to_string(),
            EntityRef::Vehicle(_) => s!("Vehicle").to_string(),
            EntityRef::Deathbox(_) => s!("Deathbox").to_string(),
            EntityRef::Animating(_) => s!("Animating").to_string(),
            EntityRef::Projectile(_) => s!("Projectile").to_string(),
            EntityRef::ScriptNetData(_) => s!("ScriptNetData").to_string(),
        }
    }
}

mod animating;
mod base;
mod deathbox;
mod loot;
mod npc;
mod player;
mod projectile;
mod scriptnetdata;
mod vehicle;
mod waypoint;
mod weaponx;
mod world;

pub use self::animating::AnimatingEntity;
pub use self::base::BaseEntity;
pub use self::deathbox::DeathboxEntity;
pub use self::loot::LootEntity;
pub use self::npc::BaseNPCEntity;
pub use self::player::PlayerEntity;
pub use self::projectile::ProjectileEntity;
pub use self::scriptnetdata::ScriptNetDataEntity;
pub use self::vehicle::VehicleEntity;
pub use self::waypoint::WaypointEntity;
pub use self::weaponx::WeaponXEntity;
pub use self::world::WorldEntity;

mod utils;
pub use self::utils::BoneArray;

#[derive(Clone, Default, Debug)]
pub struct ModelName {
    pub ptr: sdk::Ptr<[u8]>,
    pub string: String,
    pub hash: sdk::ModelName,
}
impl ModelName {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, model_name_ptr: sdk::Ptr<[u8]>) -> bool {
        // Update when pointer changes
        if model_name_ptr != self.ptr {
            self.string.clear();
            self.hash = Default::default();
            if !model_name_ptr.is_null() {
                let mut model_name = [0u8; 128];
                if let Ok(model_name) = api.vm_read_cstr(model_name_ptr, &mut model_name).await {
                    self.string.push_str(model_name);
                    self.string.make_ascii_lowercase(); // Keep everything consistently lower cased
                    self.hash = sdk::ModelName(crate::apexdream::base::hash(&self.string));
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HitSphere {
    pub bone: i32,
    pub radius: f32,
}
