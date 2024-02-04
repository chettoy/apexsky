use format_xml::xfmt;
use obfstr::obfstr as s;

use self::entities::EntityRef;

use super::*;

const MAX_ITEMS: usize = 400;

#[derive(Default)]
pub struct LootItems {
    models: Vec<String>,
    table: Vec<sdk::ItemId>,
    visualize: bool,
}

impl LootItems {
    #[instrument(skip_all)]
    pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        // Reload the itemids on new connection
        if ctx.connected {
            self.models.resize_with(MAX_ITEMS, String::new);
            self.table.resize(MAX_ITEMS, sdk::ItemId::None);
        }

        // Visualize the cache if successfully identified an item
        if self.visualize {
            self.visualize = false;
            api.visualize(s!("Items"), xfmt! {
				<pre>
					for index in 0..MAX_ITEMS {
						if let (Some(&known), Some(model)) = (self.table.get(index), self.models.get(index)) {
							if known != sdk::ItemId::None {
								{index}": "{model}"\n"
							}
						}
					}
				</pre>
			});
        }
    }

    pub fn visit(&mut self, _api: &mut Api, _ctx: &UpdateContext, entity_ref: EntityRef<'_>) {
        let loot = match entity_ref {
            EntityRef::Loot(loot) => loot,
            _ => return,
        };

        // let new_ki = analyze(loot);
        // if new_ki == sdk::ItemId::None {
        //     return;
        // }

        let index = loot.custom_script_int as usize;
        let Some(p_ki) = self.table.get_mut(index) else {
            return;
        };
        let Some(p_model) = self.models.get_mut(index) else {
            return;
        };

        // self.visualize |= *p_ki != new_ki;
        // *p_ki = new_ki;

        if loot.model_name.string.len() != 0 {
            p_model.clone_from(&loot.model_name.string);
        }
    }
}

impl GameState {
    pub fn known_item(&self, custom_script_int: i32) -> sdk::ItemId {
        self.items
            .table
            .get(custom_script_int as usize)
            .cloned()
            .unwrap_or(sdk::ItemId::None)
    }
}

fn from_color(color: &[f32; 3], items: &[sdk::ItemId; 5]) -> sdk::ItemId {
    let table = [
        &sdk::COMMON_COLORS,
        &sdk::RARE_COLORS,
        &sdk::EPIC_COLORS,
        &sdk::LEGENDARY_COLORS,
        &sdk::HEIRLOOM_COLORS,
    ];
    for i in 0..5 {
        let colors = &table[i];
        for j in 0..colors.len() {
            if color == &colors[j] {
                return items[i];
            }
        }
    }
    return sdk::ItemId::None;
}
