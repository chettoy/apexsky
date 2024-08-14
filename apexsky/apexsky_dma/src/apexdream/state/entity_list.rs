use parking_lot::Mutex;

use self::entities::{
    AnimatingEntity, BaseNPCEntity, DeathboxEntity, Entity, LootEntity, ScriptNetDataEntity,
    VehicleEntity, WorldEntity,
};
use std::{collections::HashMap, mem};

use super::*;
use crate::noobfstr as s;

#[derive(Debug)]
pub struct EntityList {
    pub entities: Box<[Option<Box<dyn Entity>>]>,
    ent_info: Box<[sdk::CEntInfo]>,
    prev_info: Box<[sdk::CEntInfo]>,
    pub updates: u32,
    next_index: u32,
    gce: GetClientEntity,
}
impl Default for EntityList {
    fn default() -> EntityList {
        let mut entities = Vec::new();
        entities.resize_with(sdk::NUM_ENT_ENTRIES, || None);
        EntityList {
            entities: entities.into_boxed_slice(),
            ent_info: vec![sdk::CEntInfo::default(); sdk::NUM_ENT_ENTRIES].into_boxed_slice(),
            prev_info: vec![sdk::CEntInfo::default(); sdk::NUM_ENT_ENTRIES].into_boxed_slice(),
            updates: 0,
            next_index: 0,
            gce: GetClientEntity::default(),
        }
    }
}

impl EntityList {
    #[instrument(skip_all)]
    #[inline(never)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let base_addr = api.apex_base;

        self.updates = 0;

        // Update entity list in smaller chunks over time
        let count;
        if self.gce.config.full_entlist {
            self.next_index = 0;
            count = sdk::NUM_ENT_ENTRIES;
        } else {
            count = sdk::NUM_ENT_ENTRIES / 32;
        }

        let start = self.next_index as usize;
        let end = usize::min(start + count, sdk::NUM_ENT_ENTRIES);
        self.next_index = if end == sdk::NUM_ENT_ENTRIES {
            0
        } else {
            end as u32
        };

        // Read a chunk of the game's ent info array
        if let Some(ent_info_slice) = self.ent_info.get_mut(start..end) {
            let _ = api
                .vm_read_into(
                    base_addr.field(ctx.data.entity_list + start as u32 * 32),
                    ent_info_slice,
                )
                .await;
        }

        // Update the entities
        let prev_info = unsafe { self.prev_info.get_unchecked(..sdk::NUM_ENT_ENTRIES) };
        let ent_info = unsafe { self.ent_info.get_unchecked(..sdk::NUM_ENT_ENTRIES) };
        let entities = unsafe { self.entities.get_unchecked_mut(..sdk::NUM_ENT_ENTRIES) };
        let mut futs_recreate = Vec::new();
        let mut futs_update = Vec::with_capacity(sdk::MAX_PLAYERS);
        let mut start_recreate = |index: usize, entity_ptr: sdk::Ptr| {
            futs_recreate.push((
                index,
                tokio::spawn({
                    let api = api.clone();
                    let gce = self.gce.clone();
                    async move { gce.create_entity(&api, entity_ptr, index as u32).await }
                }),
            ));
        };
        let mut start_update = |index, mut entity: Box<dyn Entity>| {
            futs_update.push((
                index,
                tokio::spawn({
                    let api = api.clone();
                    let ctx = ctx.clone();
                    async move {
                        entity.update(&api, &ctx).await;
                        entity
                    }
                }),
            ));
        };
        //for index in 0..sdk::NUM_ENT_ENTRIES
        for index in start..end {
            //let in_range = index >= start && index < end;
            let ptr_changed = prev_info[index].pEntity != ent_info[index].pEntity;

            // If entity pointer has changed
            if ptr_changed {
                // Recreate the entity object with the correct type
                let entity_ptr = ent_info[index].pEntity;
                start_recreate(index, entity_ptr);
            }
            // Update the entity at their specified rate if we are tracking it
            else if let Some(entity) = entities[index].take_if(|entity| {
                ctx.ticked(entity.get_info().rate, index as u32) && (!ptr_changed)
            }) {
                start_update(index, entity);
            }
        }

        // Place the updated entity back in the list
        for (index, fut_recreate) in futs_recreate {
            entities[index] = fut_recreate.await.unwrap();
            // Always update the entity when created
            if let Some(entity) = entities[index].take() {
                start_update(index, entity);
            }
        }
        for (index, fut_update) in futs_update {
            let entity = fut_update.await.unwrap();
            entities[index].replace(entity);
            self.updates += 1;
        }

        // If we reached the end, swap the entity infos
        if end == sdk::NUM_ENT_ENTRIES {
            mem::swap(&mut self.ent_info, &mut self.prev_info);
        }
    }
}

//----------------------------------------------------------------
// GetClientEntity

#[derive(Debug, Clone)]
struct Config {
    log_errors: bool,
    log_uninteresting: bool,
    full_entlist: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            log_errors: false,
            log_uninteresting: false,
            full_entlist: true,
        }
    }
}

#[derive(Debug, Clone)]
struct ClientClassData {
    client_class: sdk::ClientClass,
    name_hash: u32,
    name_buf: [u8; 52],
}

#[derive(Debug, Default, Clone)]
struct GetClientEntity {
    config: Config,
    lookup: Arc<Mutex<HashMap<sdk::Ptr<[sdk::Ptr]>, ClientClassData>>>,
}

impl GetClientEntity {
    #[inline(never)]
    async fn create_entity(
        &self,
        api: &Api,
        entity_ptr: sdk::Ptr,
        index: u32,
    ) -> Option<Box<dyn Entity>> {
        if entity_ptr.is_null() {
            return None;
        }
        // Filter out bad addresses (mainly from running the hack before the game has decrypted itself)
        if entity_ptr.into_raw() & 7 != 0 || entity_ptr.into_raw() >= (1 << 48) {
            return None;
        }

        // Borrowck error avoidance :)
        let log_uninteresting = self.config.log_uninteresting;

        // Get the entity type name
        let data = self.get_client_class(api, entity_ptr).await?;

        match data.name_hash {
            sdk::CPlayer => {
                if let Some(name) = crate::apexdream::base::from_utf8_buf(&data.name_buf) {
                    if name != s!("CPlayer") {
                        tracing::warn!(?name, "{}", s!("invalid player class"));
                    }
                } else {
                    tracing::warn!("{}", s!("invalid player class"));
                }
                Some(PlayerEntity::new(entity_ptr, index, &data.client_class))
            }
            sdk::CPropSurvival => Some(LootEntity::new(entity_ptr, index, &data.client_class)),
            sdk::CWeaponX => Some(WeaponXEntity::new(entity_ptr, index, &data.client_class)),
            sdk::CWorld => Some(WorldEntity::new(entity_ptr, index, &data.client_class)),
            sdk::CAI_BaseNPC => Some(BaseNPCEntity::new(entity_ptr, index, &data.client_class)),
            // sdk::CPlayerWaypoint => {
            // 	Some(WaypointEntity::new(entity_ptr, index, &data.client_class))
            // },
            sdk::CPlayerVehicle => Some(VehicleEntity::new(entity_ptr, index, &data.client_class)),
            sdk::CDeathBoxProp => Some(DeathboxEntity::new(entity_ptr, index, &data.client_class)),
            sdk::CDynamicProp | sdk::CScriptProp | sdk::CPhysicsProp => {
                Some(AnimatingEntity::new(entity_ptr, index, &data.client_class))
            }
            sdk::CScriptNetData_SNDC_PLAYER_GLOBAL => Some(ScriptNetDataEntity::new(
                entity_ptr,
                index,
                &data.client_class,
            )),
            sdk::CScriptNetData_SNDC_PLAYER_EXCLUSIVE => Some(ScriptNetDataEntity::new(
                entity_ptr,
                index,
                &data.client_class,
            )),
            // sdk::CCrossbowBolt | sdk::CBaseGrenade => {
            // 	Some(ProjectileEntity::new(entity_ptr, index, &data.client_class))
            // },
            // _ => {
            // 	Some(BaseEntity::new(entity_ptr, index, &data.client_class))
            // },
            _ => {
                // let name = base::from_utf8_buf(&data.name_buf);
                //tracing::warn!("{}{}{}{name:?}", s!("Uninteresting["), index, s!("]: "));
                if log_uninteresting {
                    let name = base::from_utf8_buf(&data.name_buf);
                    api.log(format!(
                        "{}{}{}{name:?}",
                        s!("Uninteresting["),
                        index,
                        s!("]: ")
                    ));
                }
                None
            }
        }
    }

    #[inline(never)]
    async fn get_client_class(&self, api: &Api, entity_ptr: sdk::Ptr) -> Option<ClientClassData> {
        // Read the IClientNetworkable vtable at entity_ptr + 3 * 8
        let client_networkable: sdk::Ptr<[sdk::Ptr]> =
            match api.vm_read(entity_ptr.field(3 * 8)).await {
                Ok(p) => p,
                Err(_) => {
                    if self.config.log_errors {
                        api.log(format!(
                            "{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): IClientNetworkable")
                        ));
                    }
                    return None;
                }
            };

        // This can be null?!?
        if client_networkable.is_null() {
            return None;
        }

        if let Some(value) = self.lookup.lock().get(&client_networkable) {
            return Some(value.to_owned());
        }

        // Aggressively cache these lookups
        {
            // Read the GetClientEntity function ptr
            let get_client_entity = match api.vm_read(client_networkable.at(3)).await {
                Ok(pgce) => pgce,
                Err(_) => {
                    if self.config.log_errors {
                        api.log(format!(
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): GetClientEntity {client_networkable="),
                            client_networkable,
                            s!("}")
                        ));
                    }
                    return None;
                }
            };

            // Read the offset out of the lea rax, offset instruction
            let offset = match api.vm_read::<i32>(get_client_entity.field(3)).await {
                Ok(offset) => offset,
                Err(_) => {
                    if self.config.log_errors {
                        api.log(format!(
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): lea rax, offset {get_client_entity="),
                            get_client_entity,
                            s!("}")
                        ));
                    }
                    return None;
                }
            };

            // Resolve relative offset
            let client_class_ptr = get_client_entity.offset((offset + 7) as i64);

            // Read ClientClass instance
            let client_class = match api.vm_read::<sdk::ClientClass>(client_class_ptr).await {
                Ok(cc) => cc,
                Err(_) => {
                    if self.config.log_errors {
                        api.log(format!(
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): ClientClass {get_client_entity="),
                            get_client_entity,
                            s!("}")
                        ));
                    }
                    return None;
                }
            };

            // FIXME! Figure out why CParticleSystem is horribly broken...
            if client_class.ClassID < 0 || client_class.ClassID > 500 {
                return None;
            }

            // Read pNetworkName
            let mut name_buf = [0u8; 52];
            let name = match api
                .vm_read_cstr(client_class.pNetworkName, &mut name_buf)
                .await
            {
                Ok(name) => name,
                Err(_) => {
                    if self.config.log_errors {
                        api.log(format!(
                            "{}{}{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): pNetworkName {get_client_entity="),
                            get_client_entity,
                            s!(", ClassID="),
                            client_class.ClassID,
                            s!("}")
                        ));
                    }
                    return None;
                }
            };

            let name_hash = crate::apexdream::base::hash(name);
            // Cache the lookup
            {
                let mut lookup = self.lookup.lock();
                lookup.insert(
                    client_networkable,
                    ClientClassData {
                        client_class,
                        name_hash,
                        name_buf,
                    },
                );
                lookup.get(&client_networkable).cloned()
            }
        }
    }
}

//----------------------------------------------------------------
// GameState helpers

#[allow(dead_code)]
impl super::GameState {
    /// Returns if an entity exists at the given index.
    pub fn is_entity(&self, handle: sdk::EHandle) -> bool {
        match handle.index() {
            Some(i) => self.entity_list.entities.get(i).is_some(),
            None => false,
        }
    }
    /// Returns the entity at the given index if it exists.
    pub fn entity(&self, handle: sdk::EHandle) -> Option<&dyn Entity> {
        let i = handle.index()?;
        let boxed = self.entity_list.entities.get(i)?;
        Some(&**boxed.as_ref()?)
    }
    /// Returns an Iterator over all valid entities.
    pub fn entities(&self) -> impl Clone + Iterator<Item = &dyn Entity> {
        self.entity_list
            .entities
            .iter()
            .filter_map(|x| x.as_ref().map(std::ops::Deref::deref))
    }
    /// Returns the entity at the given index if it exists and matches the given type.
    pub fn entity_as<T: Entity>(&self, handle: sdk::EHandle) -> Option<&T> {
        let i = handle.index()?;
        let boxed = self.entity_list.entities.get(i)?;
        let entity = &**boxed.as_ref()?;
        entity.as_any().downcast_ref()
    }
    /// Returns an Iterator over all entities of the given type.
    pub fn entities_as<T: Entity>(&self) -> impl Clone + Iterator<Item = &T> {
        self.entity_list
            .entities
            .iter()
            .filter_map(|x| x.as_ref().and_then(|e| e.as_any().downcast_ref()))
    }
    /// Returns an Iterator over the player entities.
    pub fn players(&self) -> impl Clone + Iterator<Item = &PlayerEntity> {
        let len = self.entity_list.entities.len().min(sdk::MAX_PLAYERS + 1);
        self.entity_list.entities[..len]
            .iter()
            .filter_map(|x| x.as_ref().and_then(|e| e.as_any().downcast_ref()))
    }
    /// Given the entity pointer, find its entity index.
    pub fn entity_index(&self, entity_ptr: sdk::Ptr) -> Option<usize> {
        if entity_ptr.is_null() {
            return None;
        }
        self.entity_list
            .ent_info
            .iter()
            .position(|ent_info| ent_info.pEntity == entity_ptr)
    }
    /// Returns the local player entity if it exists.
    pub fn local_player(&self) -> Option<&PlayerEntity> {
        self.entity_as(self.client.local_entity)
    }
    /// Returns the local player if alive, else the player being observed, if any.
    pub fn camera_player(&self) -> Option<&PlayerEntity> {
        match self.local_player() {
            Some(local) if local.is_alive() => Some(local),
            Some(local) => self.entity_as(local.observer_target),
            None => None,
        }
    }
    pub fn world_entity(&self) -> Option<&WorldEntity> {
        self.entity_as(0.into())
    }
}
