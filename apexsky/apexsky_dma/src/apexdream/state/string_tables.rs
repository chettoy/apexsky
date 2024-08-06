use super::*;
use crate::noobfstr as s;
use dataview::Pod;

#[derive(Default, Debug, Pod)]
#[repr(C)]
pub struct CNetStringTableItem {
    /*0x00*/ pub unk00: u64,
    /*0x08*/ pub unk08: u64,
    /*0x10*/ pub string: sdk::Ptr<[u8]>,
    /*0x18*/ pub unk18: u64,
    /*0x20*/ pub unk20: u64,
    /*0x28*/ pub unk28: u64,
    /*0x30*/ pub unk30: u64,
    /*0x38*/ pub unk38: u64,
    /*0x40*/ pub unk40: u64,
}
const _: [(); 0x48] = [(); std::mem::size_of::<CNetStringTableItem>()];

#[derive(Default, Debug, Pod)]
#[repr(C)]
pub struct CNetStringDict {
    /*0x00*/ pub vtable: sdk::Ptr,
    /*0x08*/ pub _unk08: u64,
    /*0x10*/ pub _unk10: u64,
    /*0x18*/ pub elements: sdk::Ptr<[CNetStringTableItem]>,
    /*0x20*/ pub allocation_count: u16,
    /*0x22*/ pub grow_size: u16,
    /*0x24*/ pub _unk24: u32,
    /*0x28*/ pub _unk28: u64,
    /*0x30*/ pub _unk30: u16,
    /*0x32*/ pub used: u16,
    /*0x34*/ pub _unk34: u16,
    /*0x36*/ pub highest: u16,
}

#[derive(Default, Debug, Pod)]
#[repr(C)]
pub struct CNetStringTable {
    /*0x00*/ pub vtable: sdk::Ptr,
    /*0x08*/ pub table_id: i32,
    /*0x0c*/ pub table_id_pad: u32,
    /*0x10*/ pub table_name: sdk::Ptr<[u8]>,
    /*0x18*/ pub max_entries: i32,
    /*0x1c*/ pub entry_bits: i32,
    /*0x20*/ pub tick_count: i32,
    /*0x24*/ pub last_changed_tick: i32,
    /*0x28*/ pub flags: u32,
    /*0x2c*/ pub user_data_size: i32,
    /*0x30*/ pub user_data_size_bits: i32,
    /*0x34*/ pub pad: u32,
    /*0x38*/ pub change_func: sdk::Ptr,
    /*0x40*/ pub object: sdk::Ptr,
    /*0x48*/ pub items: sdk::Ptr<CNetStringDict>,
    /*0x50*/ pub items_client_side: sdk::Ptr<CNetStringDict>,
}

pub async fn load_string_table(
    st: &mut Box<[String]>,
    api: &Api,
    _ctx: &UpdateContext,
    offset: u32,
) -> anyhow::Result<()> {
    let mut ptr = sdk::Ptr::<CNetStringTable>::NULL;
    let _ = api
        .vm_read_into(api.apex_base.field(offset), &mut ptr)
        .await;
    if ptr.is_null() {
        return Ok(());
    }
    let table = api.vm_read(ptr).await?;
    let dict = api.vm_read(table.items).await?;
    *st = vec![String::new(); dict.used as usize].into_boxed_slice();

    async fn read_item(api: &Api, ptr: intptr::IntPtr<CNetStringTableItem>) -> Option<String> {
        if let Ok(item) = api.vm_read(ptr).await {
            let mut buffer = [0u8; 64];
            if let Ok(string) = api.vm_read_cstr(item.string, &mut buffer).await {
                return Some(String::from(string));
            }
        }
        None
    }

    let mut futs_read = vec![];
    let mut start_read = |i| {
        futs_read.push((
            i,
            tokio::spawn({
                let api = api.clone();
                let ptr = dict.elements.at(i);
                async move { read_item(&api, ptr).await }
            }),
        ));
    };

    for (i, _slot) in st.iter().enumerate() {
        start_read(i);
    }
    for (i, fut_read) in futs_read {
        if let Some(data) = fut_read.await? {
            *st.get_mut(i).unwrap() = data;
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct StringTables {
    pub weapon_names: Box<[String]>,
}
impl StringTables {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, ctx: &UpdateContext) {
        // Read stringtable once on connect
        if ctx.connected {
            let data = &ctx.data;
            load_string_table(&mut self.weapon_names, api, ctx, data.nst_weapon_names)
                .await
                .unwrap_or_else(|e| {
                    tracing::warn!(?e);
                });
        }
    }
}

impl GameState {
    pub fn weapon_string(&self, weapon_name_index: i32) -> Option<&str> {
        Some(
            &*self
                .string_tables
                .weapon_names
                .get(weapon_name_index as usize)?,
        )
    }
    pub fn weapon_name(&self, weapon_name_index: i32) -> sdk::WeaponName {
        let Some(weapon_name) = self
            .string_tables
            .weapon_names
            .get(weapon_name_index as usize)
        else {
            return Default::default();
        };
        sdk::WeaponName(crate::apexdream::base::hash(weapon_name))
    }
    pub fn weapon_is_melee(&self, weapon_name_index: i32) -> bool {
        let Some(weapon_name) = self
            .string_tables
            .weapon_names
            .get(weapon_name_index as usize)
        else {
            return Default::default();
        };
        if sdk::WeaponName(crate::apexdream::base::hash(weapon_name))
            == sdk::WeaponName::MELEE_SURVIVAL
        {
            return true;
        }
        if weapon_name.starts_with(s!("melee_")) {
            return true;
        }
        if weapon_name.ends_with(s!("_primary")) {
            return true;
        }
        return false;
    }
    pub fn weapon_is_charged(&self, weapon_name_index: i32) -> bool {
        match self.weapon_name(weapon_name_index) {
            sdk::WeaponName::BOCEK_BOW => true,
            sdk::WeaponName::THERMITE_GRENADE => true,
            sdk::WeaponName::FRAG_GRENADE => true,
            sdk::WeaponName::ARC_STAR => true,
            sdk::WeaponName::CLUSTER_BOMB_LAUNCHER => true,
            sdk::WeaponName::THROWING_KNIFE => true,
            _ => false,
        }
    }
}
