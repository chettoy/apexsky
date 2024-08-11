use crate::{i18n::load_fluent_bundle, lock_config};

use super::apexsky_menu::{general_menu::GeneralMenuName, MenuState};

use super::apexsky_menu::general_menu::*;
use super::apexsky_menu::TerminalMenu;

mod aimbot_menu;
mod glow_color_menu;
mod hot_key_menu;
mod loot_menu;
mod main_menu;
mod spectators_menu;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MenuLevel {
    #[default]
    MainMenu,
    AimbotMenu,
    GlowColorMenu,
    ItemFilterMenu,
    LightWeaponsMenu,
    HeavyWeaponsMenu,
    EnergyWeaponsMenu,
    SniperWeaponsMenu,
    ArmorsMenu,
    HealingMenu,
    NadesMenu,
    BackpacksMenu,
    HopUpsMenu,
    ScopesMenu,
    KeyCodesMenu,
    HotkeyMenu,
    SpectatorsMenu,
}

impl GeneralMenuName for MenuLevel {
    fn rebuild_state(self) -> Box<dyn MenuState> {
        self.into()
    }
}

impl<'a> Into<Box<GeneralMenu<'a, MenuLevel>>> for MenuLevel {
    fn into(self) -> Box<GeneralMenu<'a, MenuLevel>> {
        let data = lock_config!().settings.to_owned();
        let i18n_bundle = load_fluent_bundle();
        Box::new(match self {
            MenuLevel::MainMenu => main_menu::build_main_menu(&i18n_bundle, data),
            MenuLevel::AimbotMenu => aimbot_menu::build_aimbot_menu(&i18n_bundle, data),
            MenuLevel::GlowColorMenu => glow_color_menu::build_glow_color_menu(&i18n_bundle, data),
            MenuLevel::ItemFilterMenu => loot_menu::build_item_filter_menu(&i18n_bundle, data),
            MenuLevel::LightWeaponsMenu => loot_menu::build_light_weapons_menu(&i18n_bundle, data),
            MenuLevel::HeavyWeaponsMenu => loot_menu::build_heavy_weapons_menu(&i18n_bundle, data),
            MenuLevel::EnergyWeaponsMenu => {
                loot_menu::build_energy_weapons_menu(&i18n_bundle, data)
            }
            MenuLevel::SniperWeaponsMenu => {
                loot_menu::build_sniper_weapons_menu(&i18n_bundle, data)
            }
            MenuLevel::ArmorsMenu => loot_menu::build_armors_menu(&i18n_bundle, data),
            MenuLevel::HealingMenu => loot_menu::build_healing_menu(&i18n_bundle, data),
            MenuLevel::NadesMenu => loot_menu::build_nades_menu(&i18n_bundle, data),
            MenuLevel::BackpacksMenu => loot_menu::build_backpacks_menu(&i18n_bundle, data),
            MenuLevel::HopUpsMenu => loot_menu::build_hopups_menu(&i18n_bundle, data),
            MenuLevel::ScopesMenu => loot_menu::build_scopes_menu(&i18n_bundle, data),
            MenuLevel::KeyCodesMenu => hot_key_menu::build_key_codes_menu(&i18n_bundle, data),
            MenuLevel::HotkeyMenu => hot_key_menu::build_hotkey_menu(&i18n_bundle, data),
            MenuLevel::SpectatorsMenu => spectators_menu::build_spectators_menu(&i18n_bundle, data),
        })
    }
}

impl From<MenuLevel> for Box<dyn MenuState> {
    fn from(val: MenuLevel) -> Self {
        let menu: Box<GeneralMenu<_>> = val.into();
        menu
    }
}
