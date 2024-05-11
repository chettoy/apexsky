use self::entities::EntityRef;
use std::collections::HashMap;

use super::*;

#[derive(Default)]
pub struct Modifiers {
    database: HashMap<sdk::WeaponName, Vec<String>>,
}

impl Modifiers {
    #[instrument(skip_all)]
    pub async fn update(&mut self, _api: &Api, ctx: &UpdateContext) {
        if ctx.connected {
            self.database.clear();
        }
    }

    #[instrument(skip_all)]
    pub async fn visit(&mut self, api: &Api, ctx: &UpdateContext, entity_ref: EntityRef<'_>) {
        if ctx.data.mods_names == 0 {
            return;
        }

        let weapon = match entity_ref {
            EntityRef::WeaponX(weapon) => weapon,
            _ => return,
        };

        if self.database.contains_key(&weapon.weapon_name) {
            return;
        }

        let mods_ptr = weapon.modifiers_ptr;
        if mods_ptr.is_null() {
            return;
        }

        let Ok(count) = api
            .vm_read::<u32>(mods_ptr.field(ctx.data.mods_count))
            .await
        else {
            return;
        };
        //tracing::trace!(?weapon.weapon_name, count);
        if count > 1000 {
            return;
        }

        let mut name_buf = [0u8; 64];
        let mut names = Vec::new();

        // Array of pointers to cstrings
        let mods_names = api.apex_base.field::<[sdk::Ptr<[u8]>]>(ctx.data.mods_names);

        for i in 0..count {
            //tracing::trace!(?weapon.weapon_name, i);
            let Ok(index) = api
                .vm_read::<u16>(mods_ptr.field(ctx.data.mods_list + i * 10))
                .await
            else {
                continue;
            };
            let Ok(name_ptr) = api.vm_read(mods_names.at(index as usize)).await else {
                continue;
            };
            let Ok(s) = api.vm_read_cstr(name_ptr, &mut name_buf).await else {
                continue;
            };
            names.push(String::from(s));
        }

        self.database.insert(weapon.weapon_name, names);
    }
}

impl GameState {
    pub fn get_mods(&self, weapon_name: sdk::WeaponName) -> Option<&[String]> {
        Some(self.mods.database.get(&weapon_name)?)
    }
    pub fn get_mod(&self, weapon_name: sdk::WeaponName, modifier: sdk::ModName) -> u32 {
        let mut flag = 1;
        let Some(mod_names) = self.mods.database.get(&weapon_name) else {
            return 0;
        };
        for mod_name in mod_names {
            if sdk::ModName(crate::apexdream::base::hash(mod_name)) == modifier {
                return flag;
            }
            flag <<= 1;
        }
        return 0;
    }
}
