use super::*;

const SIZE: usize = 32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InitState {
    #[default]
    Uninitialized,
    Initialized,
    Initializing,
}

#[derive(Debug, Default, Clone)]
pub struct ScriptNetDataEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,
    recv_table: sdk::Ptr<sdk::RecvTableRedux>,

    // Offsets for [m_bools, m_ranges, m_int32s, m_times, m_entities] respectively
    offsets: [u32; 5],

    initialized: InitState,

    /// Is the script data related to local player
    local_player: bool,

    pub bools: [u8; SIZE],
    pub ranges: [i16; SIZE],
    pub ints: [i32; SIZE],
    pub times: [f32; SIZE],
    pub ents: [sdk::EHandle; SIZE],
}

impl ScriptNetDataEntity {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        let recv_table = cc.pRecvTable.cast();
        Box::new(ScriptNetDataEntity {
            entity_ptr,
            entity_size,
            index,
            recv_table,
            ..Default::default()
        })
    }

    pub fn bools(&self) -> &[u8] {
        let len = (self.offsets[1] - self.offsets[0]) as usize;
        &self.bools[..usize::min(SIZE, len)]
    }
    pub fn shorts(&self) -> &[i16] {
        let len = (self.offsets[2] - self.offsets[1]) as usize / 2;
        &self.ranges[..usize::min(SIZE, len)]
    }
    pub fn ints(&self) -> &[i32] {
        let len = (self.offsets[3] - self.offsets[2]) as usize / 4;
        &self.ints[..usize::min(SIZE, len)]
    }
    pub fn floats(&self) -> &[f32] {
        let len = (self.offsets[4] - self.offsets[3]) as usize / 4;
        &self.times[..usize::min(SIZE, len)]
    }
    pub fn ents(&self) -> &[sdk::EHandle] {
        let len = (self.entity_size - self.offsets[4]) as usize / 4;
        &self.ents[..usize::min(SIZE, len)]
    }

    async fn init(&mut self, api: &Api, ctx: &UpdateContext) -> bool {
        if self.initialized == InitState::Initialized {
            return true;
        }
        if self.initialized == InitState::Initializing && !ctx.ticked(200, 21) {
            return false;
        }
        self.initialized = InitState::Initializing;

        let Ok(recv_table) = api.vm_read(self.recv_table).await else {
            return false;
        };
        if recv_table.num_props > 100 {
            return false;
        }

        let mut name_buf = [0u8; 64];
        let mut count = 0;
        for i in 0..11 {
            let Ok(prop_ptr) = api.vm_read(recv_table.props.at(i)).await else {
                return false;
            };
            let Ok(prop) = api.vm_read(prop_ptr).await else {
                return false;
            };
            let Ok(name) = api.vm_read_cstr(prop.name, &mut name_buf).await else {
                return false;
            };
            let name = crate::apexdream::base::hash(name);

            if name == hash!("m_bools[0]") {
                self.offsets[0] = prop.offset as u32;
                count += 1;
            } else if name == hash!("m_ranges[0]") {
                self.offsets[1] = prop.offset as u32;
                count += 1;
            } else if name == hash!("m_int32s[0]") {
                self.offsets[2] = prop.offset as u32;
                count += 1;
            } else if name == hash!("m_times[0]") {
                self.offsets[3] = prop.offset as u32;
                count += 1;
            } else if name == hash!("m_entities[0]") {
                self.offsets[4] = prop.offset as u32;
                count += 1;
            }
        }

        if count != 5 {
            return false;
        }

        let indices = {
            let list: Vec<String> = self.offsets.iter().map(|idx| format!("{idx:#x}")).collect();
            list.join(", ")
        };
        api.log(format!(
            "{}{}{}",
            s!("ScriptNetData indices=["),
            indices,
            s!("]")
        ));
        self.initialized = InitState::Initialized;
        true
    }
}

#[async_trait]
impl Entity for ScriptNetDataEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::ScriptNetData(self)
    }
    fn is_serialized(&self) -> bool {
        false
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            handle: sdk::EHandle::from(self.index),
            index: self.index as usize,
            rate: if self.local_player { 1 } else { 64 },
        }
    }
    #[instrument(skip_all)]
    async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        if !self.init(api, ctx).await {
            return;
        }

        let mut buf = [0u32; 500 / 4];

        let start_offset = self.offsets.iter().cloned().min().unwrap_or(0);

        let Some(buf) =
            dataview::bytes_mut(&mut buf).get_mut(..(self.entity_size - start_offset) as usize)
        else {
            return;
        };

        // Read into buffer
        let _ = api
            .vm_read_into(self.entity_ptr.field(start_offset), buf)
            .await;

        let view = dataview::DataView::from(buf);
        for i in 0..SIZE {
            let bool_offset = (self.offsets[0] - start_offset) as usize + i;
            self.bools[i] = view.try_read::<u8>(bool_offset).unwrap_or(0);

            let range_offset = (self.offsets[1] - start_offset) as usize + i * 2;
            self.ranges[i] = view.try_read::<i16>(range_offset).unwrap_or(0);

            let int_offset = (self.offsets[2] - start_offset) as usize + i * 4;
            self.ints[i] = view.try_read::<i32>(int_offset).unwrap_or(0);

            let time_offset = (self.offsets[3] - start_offset) as usize + i * 4;
            self.times[i] = view.try_read::<f32>(time_offset).unwrap_or(0.0);

            let ent_offset = (self.offsets[4] - start_offset) as usize + i * 4;
            self.ents[i] = view
                .try_read::<sdk::EHandle>(ent_offset)
                .unwrap_or_default();
        }
    }

    fn post(&mut self, _api: &Api, _ctx: &UpdateContext, state: &GameState) {
        let Some(local) = state.local_player() else {
            self.local_player = false;
            return;
        };
        self.local_player = self.index as i32 == local.script_net_data_exclusive.signed_index()
            || self.index as i32 == local.script_net_data_global.signed_index();
    }
}
