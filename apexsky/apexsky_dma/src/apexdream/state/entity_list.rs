use dashmap::DashMap;

use self::entities::{
    AnimatingEntity, BaseNPCEntity, DeathboxEntity, Entity, LootEntity, ScriptNetDataEntity,
    VehicleEntity, WorldEntity,
};

use super::*;
use crate::noobfstr as s;

const REQUEST_ID_UPDATE: usize = 0; //obfstr::random!(usize);
const REQUEST_ID_RECREATE: usize = 0; //obfstr::random!(usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EntityStatus {
    Invalid = 0,
    #[default]
    RetryCreate,
    Retry2,
    Retry4,
    Retry8,
    Valid = 255,
}

#[derive(Debug)]
pub struct EntityList {
    pub entities: Box<[Option<Box<dyn Entity>>]>,
    ent_info: Box<[sdk::CEntInfo]>,
    prev_info: Box<[sdk::CEntInfo]>,
    ent_status: Box<[EntityStatus]>,
    pub updates: u32,
    next_update_index: u32,
    next_recreate_index: u32,
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
            ent_status: vec![EntityStatus::default(); sdk::NUM_ENT_ENTRIES].into_boxed_slice(),
            updates: 0,
            next_update_index: 0,
            next_recreate_index: 0,
            gce: GetClientEntity::default(),
        }
    }
}

impl EntityList {
    #[instrument(skip_all)]
    #[inline(never)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        let base_addr = api.apex_base;
        // self.gce.config.full_recreate = ctx.is_io_fast;

        self.updates = 0;

        // Update entity list in smaller chunks over time
        let update_count;
        if self.gce.config.full_entlist {
            self.next_update_index = 0;
            update_count = sdk::NUM_ENT_ENTRIES;
        } else {
            update_count = sdk::NUM_ENT_ENTRIES / 32;
        }
        let update_start = self.next_update_index as usize;
        let update_end = usize::min(update_start + update_count, sdk::NUM_ENT_ENTRIES);

        let recreate_count;
        if self.gce.config.full_recreate {
            self.next_recreate_index = 0;
            recreate_count = sdk::NUM_ENT_ENTRIES;
        } else {
            recreate_count = sdk::NUM_ENT_ENTRIES / 32;
        }
        let recreate_start = self.next_recreate_index as usize;
        let recreate_end = usize::min(recreate_start + recreate_count, sdk::NUM_ENT_ENTRIES);

        // Read a chunk of the game's ent info array
        if let Some(ent_info_slice) = self.ent_info.get_mut(update_start..update_end) {
            let Ok(()) = api
                .vm_read_into(
                    base_addr.field(ctx.data.entity_list + update_start as u32 * 32),
                    ent_info_slice,
                )
                .await
            else {
                return;
            };
        }

        // Update the entities
        let prev_info = unsafe { self.prev_info.get_unchecked(..sdk::NUM_ENT_ENTRIES) };
        let ent_info = unsafe { self.ent_info.get_unchecked_mut(..sdk::NUM_ENT_ENTRIES) };
        let ent_status = unsafe { self.ent_status.get_unchecked_mut(..sdk::NUM_ENT_ENTRIES) };
        let entities = unsafe { self.entities.get_unchecked_mut(..sdk::NUM_ENT_ENTRIES) };

        let mut tasks_recreate = Vec::new();
        let mut tasks_update = Vec::with_capacity(sdk::MAX_PLAYERS);

        let mut start_recreate = |index: usize, entity_ptr: sdk::Ptr| {
            tasks_recreate.push((
                index,
                tokio::spawn({
                    let api = api.clone_new(REQUEST_ID_RECREATE);
                    let gce = self.gce.clone();
                    async move { gce.create_entity(&api, entity_ptr, index as u32).await }
                }),
            ));
        };
        let mut start_update = |index, mut entity: Box<dyn Entity>| {
            tasks_update.push((
                index,
                tokio::spawn({
                    let api = api.clone_new(REQUEST_ID_UPDATE);
                    let ctx = ctx.clone();
                    async move {
                        entity.update(&api, &ctx).await;
                        entity
                    }
                }),
            ));
        };

        let retry2 = ctx.ticked(2, 0);
        let retry4 = ctx.ticked(4, 0);
        let retry8 = ctx.ticked(8, 0);
        for index in update_start..update_end {
            let entity_ptr = ent_info[index].pEntity;
            let status = &mut ent_status[index];

            if entity_ptr.is_null() {
                *status = EntityStatus::Invalid;
                entities[index] = None;
                continue;
            }

            let ptr_changed = prev_info[index].pEntity != entity_ptr;
            if ptr_changed {
                *status = EntityStatus::RetryCreate;
            }

            if *status == EntityStatus::Valid {
                // Update the entity at their specified rate if we are tracking it
                if let Some(entity) = entities[index]
                    .take_if(|entity| ctx.ticked(entity.get_info().rate, index as u32))
                {
                    start_update(index, entity);
                }
                continue;
            }

            match *status {
                _ if !(index < sdk::MAX_PLAYERS
                    || (recreate_start..recreate_end).contains(&index)) =>
                {
                    continue;
                }

                EntityStatus::RetryCreate => {
                    // Recreate the entity object with the correct type
                    // tracing::warn!(?index, "recreate");
                    start_recreate(index, entity_ptr);
                }

                EntityStatus::Retry2 if retry2 => {
                    start_recreate(index, entity_ptr);
                }
                EntityStatus::Retry4 if retry4 => {
                    start_recreate(index, entity_ptr);
                }
                EntityStatus::Retry8 if retry8 => {
                    start_recreate(index, entity_ptr);
                }

                _ => continue,
            }
        }

        // Place the updated entity back in the list
        // tracing::trace!(ent_recreate = tasks_recreate.len());
        for (index, fut_recreate) in tasks_recreate {
            let (item, _status) = fut_recreate.await.unwrap();

            if let Some(entity) = item {
                ent_status[index] = EntityStatus::Valid;

                // Always update the entity when created
                start_update(index, entity);
            } else if Some(index) == ctx.local_entity.index() {
                // Always retry for local player
                ent_status[index] = EntityStatus::Retry2;
            } else if index > 15000 {
                ent_status[index] = EntityStatus::Invalid;
            } else {
                // Gradually increase the retry interval with failure
                ent_status[index] = match ent_status[index] {
                    EntityStatus::Invalid => EntityStatus::RetryCreate,
                    _ if ctx.is_io_fast => EntityStatus::Retry2,
                    EntityStatus::RetryCreate => EntityStatus::Retry2,
                    EntityStatus::Retry2 => EntityStatus::Retry4,
                    EntityStatus::Retry4 => EntityStatus::Retry8,
                    EntityStatus::Retry8 => EntityStatus::Invalid,
                    EntityStatus::Valid => unreachable!(),
                };
            }
        }
        // tracing::trace!(ent_update = tasks_update.len());
        for (index, fut_update) in tasks_update {
            let entity = fut_update.await.unwrap();
            entities[index].replace(entity);
            self.updates += 1;
        }

        // If we reached the end, swap the entity infos
        if update_end == sdk::NUM_ENT_ENTRIES {
            std::mem::swap(&mut self.ent_info, &mut self.prev_info);
        }

        self.next_update_index = if update_end == sdk::NUM_ENT_ENTRIES {
            0
        } else {
            update_end as u32
        };
        self.next_recreate_index = if recreate_end == sdk::NUM_ENT_ENTRIES {
            0
        } else {
            recreate_end as u32
        };
    }
}

//----------------------------------------------------------------
// GetClientEntity

#[derive(Debug, Clone)]
struct Config {
    log_errors: bool,
    log_uninteresting: bool,
    full_entlist: bool,
    full_recreate: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            log_errors: false,
            log_uninteresting: false,
            full_entlist: true,
            full_recreate: false,
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
    lookup: Arc<DashMap<sdk::Ptr<[sdk::Ptr]>, ClientClassData>>,
    directory: Arc<DashMap<String, String>>,
}

impl GetClientEntity {
    #[inline(never)]
    async fn create_entity(
        &self,
        api: &Api,
        entity_ptr: sdk::Ptr,
        index: u32,
    ) -> (Option<Box<dyn Entity>>, EntityStatus) {
        if entity_ptr.is_null() {
            return (None, EntityStatus::Invalid);
            // anyhow::bail!("{}", s!("entity_ptr is null"));
        }
        // Filter out bad addresses (mainly from running the hack before the game has decrypted itself)
        if entity_ptr.into_raw() & 7 != 0 || entity_ptr.into_raw() >= (1 << 48) {
            return (None, EntityStatus::Invalid);
            // anyhow::bail!("{}", s!("bad address for entity"));
        }

        // Borrowck error avoidance :)
        let log_uninteresting = self.config.log_uninteresting;

        // Get the entity type name
        let data = match self.get_client_class(api, entity_ptr).await {
            Ok(Some(data)) => data,
            Ok(None) => return (None, EntityStatus::RetryCreate),
            Err(_) => return (None, EntityStatus::Invalid),
        };

        const GEN_DIR: bool = false;
        if GEN_DIR {
            let mut entity_name = [0u8; 128];
            if let Ok(entity_name) = api
                .vm_read_cstr(
                    entity_ptr.field(crate::G_OFFSETS.entiry_name as u32),
                    &mut entity_name,
                )
                .await
            {
                if !self.directory.contains_key(entity_name) {
                    let class_name = base::from_utf8_buf(&data.name_buf).unwrap_or_default();
                    self.directory
                        .insert(entity_name.to_string(), class_name.to_string());
                }
            }
        }

        let entity = match data.name_hash {
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
        };

        match entity {
            Some(ent) => (Some(ent), EntityStatus::Valid),
            None => (None, EntityStatus::Invalid),
        }
    }

    #[tracing::instrument(skip_all)]
    #[inline(never)]
    async fn get_client_class(
        &self,
        api: &Api,
        entity_ptr: sdk::Ptr,
    ) -> anyhow::Result<Option<ClientClassData>> {
        // Read the IClientNetworkable vtable at entity_ptr + 3 * 8
        let client_networkable: sdk::Ptr<[sdk::Ptr]> =
            match api.vm_read(entity_ptr.field(3 * 8)).await {
                Ok(p) => p,
                Err(e) => {
                    if self.config.log_errors {
                        tracing::warn!(
                            ?e,
                            "{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): IClientNetworkable")
                        );
                    }
                    // return Err(e);
                    return Ok(None);
                }
            };

        // This can be null?!?
        if client_networkable.is_null() {
            //return Ok(None);
            anyhow::bail!("{}", s!("client_networkable is null"));
        }

        if let Some(value) = self.lookup.get(&client_networkable) {
            return Ok(Some(value.to_owned()));
        }

        // Aggressively cache these lookups
        {
            // Read the GetClientEntity function ptr
            let get_client_entity = match api.vm_read(client_networkable.at(3)).await {
                Ok(pgce) => pgce,
                Err(e) => {
                    if self.config.log_errors {
                        tracing::warn!(
                            ?e,
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): GetClientEntity {client_networkable="),
                            client_networkable,
                            s!("}")
                        );
                    }
                    return Err(e);
                }
            };

            // Read the offset out of the lea rax, offset instruction
            let offset = match api.vm_read::<i32>(get_client_entity.field(3)).await {
                Ok(offset) => offset,
                Err(e) => {
                    if self.config.log_errors {
                        tracing::warn!(
                            ?e,
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): lea rax, offset {get_client_entity="),
                            get_client_entity,
                            s!("}")
                        );
                    }
                    return Err(e);
                }
            };

            // Resolve relative offset
            let client_class_ptr = get_client_entity.offset((offset + 7) as i64);

            // Read ClientClass instance
            let client_class = match api.vm_read::<sdk::ClientClass>(client_class_ptr).await {
                Ok(cc) => cc,
                Err(e) => {
                    if self.config.log_errors {
                        tracing::warn!(
                            "{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): ClientClass {get_client_entity="),
                            get_client_entity,
                            s!("}")
                        );
                    }
                    return Err(e);
                }
            };

            // FIXME! Figure out why CParticleSystem is horribly broken...
            if client_class.ClassID < 0 || client_class.ClassID > 500 {
                //return Ok(None);
                anyhow::bail!("{}{}", s!("client_class.ClassID="), client_class.ClassID);
            }

            // Read pNetworkName
            let mut name_buf = [0u8; 52];
            let name = match api
                .vm_read_cstr(client_class.pNetworkName, &mut name_buf)
                .await
            {
                Ok(name) => name,
                Err(e) => {
                    if self.config.log_errors {
                        tracing::warn!(
                            ?e,
                            "{}{}{}{}{}{}{}",
                            s!("get_client_class("),
                            entity_ptr,
                            s!("): pNetworkName {get_client_entity="),
                            get_client_entity,
                            s!(", ClassID="),
                            client_class.ClassID,
                            s!("}")
                        );
                    }
                    return Err(e);
                }
            };

            let name_hash = crate::apexdream::base::hash(name);
            // Cache the lookup
            {
                let data = ClientClassData {
                    client_class,
                    name_hash,
                    name_buf,
                };
                self.lookup.insert(client_networkable, data.clone());
                Ok(Some(data))
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
    /// Given the entity index, find its entity pointer
    pub fn entity_ptr(&self, entity_index: u32) -> Option<sdk::Ptr> {
        self.entity_list
            .ent_info
            .get(entity_index as usize)
            .map(|info| info.pEntity)
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
