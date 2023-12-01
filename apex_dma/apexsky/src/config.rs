use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub load_settings: bool,
    pub super_key: bool,
    pub keyboard: bool,
    pub gamepad: bool,
    pub aimbot_hot_key_1: i32,
    pub aimbot_hot_key_2: i32,
    pub tigger_bot_hot_key: i32,
    pub autoshoot: bool,
    pub tigger_bot: bool,
    pub loot_filled_toggle: bool,
    pub player_filled_toggle: bool,
    pub super_key_toggle: bool,
    pub onevone: bool,
    pub tdm_toggle: bool,
    pub item_glow: bool,
    pub player_glow: bool,
    pub deathbox: bool,
    pub aim_no_recoil: bool,
    pub ads_fov: f32,
    pub non_ads_fov: f32,
    pub aim: i32,
    pub esp: bool,
    pub mini_map_radar: bool,
    pub mini_map_guides: bool,
    pub mini_map_radar_dot_size1: i32,
    pub mini_map_radar_dot_size2: i32,
    pub main_radar_map: bool,
    pub main_map_radar_dot_size1: i32,
    pub main_map_radar_dot_size2: i32,
    pub aim_dist: f32,
    pub max_dist: f32,
    pub map_radar_testing: bool,
    pub show_aim_target: bool,
    pub game_fps: f32,
    pub calc_game_fps: bool,
    pub no_nade_aim: bool,
    pub firing_range: bool,
    pub bone: i32,
    pub bone_nearest: bool,
    pub bone_auto: bool,
    pub smooth: f32,
    pub skynade_smooth: f32,
    pub inside_value: u8,
    pub outline_size: u8,
    pub glow_r_not: f32,
    pub glow_g_not: f32,
    pub glow_b_not: f32,
    pub glow_r_viz: f32,
    pub glow_g_viz: f32,
    pub glow_b_viz: f32,
    pub glow_r_knocked: f32,
    pub glow_g_knocked: f32,
    pub glow_b_knocked: f32,
    pub loot_filled: u8,
    pub loot_outline: u8,
    // rev skull
    pub loot_skull: bool,
    // Backpacks
    pub loot_lightbackpack: bool,
    pub loot_medbackpack: bool,
    pub loot_heavybackpack: bool,
    pub loot_goldbackpack: bool,
    // Shield upgrades
    pub loot_shieldupgrade1: bool, // white
    pub loot_shieldupgrade2: bool, // blue
    pub loot_shieldupgrade3: bool, // purple
    pub loot_shieldupgrade4: bool, // gold
    pub loot_shieldupgrade5: bool, // red
    pub loot_shieldupgradehead1: bool,
    pub loot_shieldupgradehead2: bool,
    pub loot_shieldupgradehead3: bool,
    pub loot_shieldupgradehead4: bool,
    pub loot_shielddown1: bool,
    pub loot_shielddown2: bool,
    pub loot_shielddown3: bool,
    pub loot_shielddown4: bool,
    // heaing and Misc
    pub loot_accelerant: bool,
    pub loot_phoenix: bool,
    pub loot_healthlarge: bool,
    pub loot_healthsmall: bool,
    pub loot_shieldbattsmall: bool,
    pub loot_shieldbattlarge: bool,
    // Ammo
    pub loot_sniperammo: bool,
    pub loot_heavyammo: bool,
    pub loot_lightammo: bool,
    pub loot_energyammo: bool,
    pub loot_shotgunammo: bool,
    // Optics
    pub loot_optic1xhcog: bool,
    pub loot_optic2xhcog: bool,
    pub loot_opticholo1x: bool,
    pub loot_opticholo1x2x: bool,
    pub loot_opticthreat: bool,
    pub loot_optic3xhcog: bool,
    pub loot_optic2x4x: bool,
    pub loot_opticsniper6x: bool,
    pub loot_opticsniper4x8x: bool,
    pub loot_opticsniperthreat: bool,
    // Magazines
    pub loot_sniperammomag1: bool,
    pub loot_energyammomag1: bool,
    pub loot_lightammomag1: bool,
    pub loot_heavyammomag1: bool,
    pub loot_sniperammomag2: bool,
    pub loot_energyammomag2: bool,
    pub loot_lightammomag2: bool,
    pub loot_heavyammomag2: bool,
    pub loot_sniperammomag3: bool,
    pub loot_energyammomag3: bool,
    pub loot_lightammomag3: bool,
    pub loot_heavyammomag3: bool,
    pub loot_sniperammomag4: bool,
    pub loot_energyammomag4: bool,
    pub loot_lightammomag4: bool,
    pub loot_heavyammomag4: bool,
    // Attachments
    pub loot_lasersight1: bool,
    pub loot_lasersight2: bool,
    pub loot_lasersight3: bool,
    pub loot_lasersight4: bool,
    pub loot_stocksniper1: bool,
    pub loot_stocksniper2: bool,
    pub loot_stocksniper3: bool,
    pub loot_stocksniper4: bool,
    pub loot_stockregular1: bool,
    pub loot_stockregular2: bool,
    pub loot_stockregular3: bool,
    pub loot_suppressor1: bool,
    pub loot_suppressor2: bool,
    pub loot_suppressor3: bool,
    pub loot_turbo_charger: bool,
    pub loot_skull_piecer: bool,
    pub loot_hammer_point: bool,
    pub loot_disruptor_rounds: bool,
    pub loot_boosted_loader: bool,
    pub loot_shotgunbolt1: bool,
    pub loot_shotgunbolt2: bool,
    pub loot_shotgunbolt3: bool,
    pub loot_shotgunbolt4: bool,
    // Nades
    pub loot_grenade_frag: bool,
    pub loot_grenade_arc_star: bool,
    pub loot_grenade_thermite: bool,
    // Kraber
    pub loot_weapon_kraber: bool,
    // Shotguns
    pub loot_weapon_mastiff: bool,
    pub loot_weapon_eva8: bool,
    pub loot_weapon_peacekeeper: bool,
    pub loot_weapon_mozambique: bool,
    // Energy weapons
    pub loot_weapon_lstar: bool,
    pub loot_weapon_nemesis: bool,
    pub loot_weapon_havoc: bool,
    pub loot_weapon_devotion: bool,
    pub loot_weapon_triple_take: bool,
    pub loot_weapon_prowler: bool,
    pub loot_weapon_volt: bool,
    // Heavy Weapons
    pub loot_weapon_flatline: bool,
    pub loot_weapon_hemlock: bool,
    pub loot_weapon_3030_repeater: bool,
    pub loot_weapon_rampage: bool,
    pub loot_weapon_car_smg: bool,
    // Light weapons
    pub loot_weapon_p2020: bool,
    pub loot_weapon_re45: bool,
    pub loot_weapon_g7_scout: bool,
    pub loot_weapon_alternator: bool,
    pub loot_weapon_r99: bool,
    pub loot_weapon_spitfire: bool,
    pub loot_weapon_r301: bool,
    // Snipers.. wingman is the odd one...and the bow..
    pub loot_weapon_wingman: bool,
    pub loot_weapon_longbow: bool,
    pub loot_weapon_charge_rifle: bool,
    pub loot_weapon_sentinel: bool,
    pub loot_weapon_bow: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // CONFIG AREA, you can change default values below.
            // Enable Loading of setting file automaticly.
            load_settings: true,
            super_key: false,
            // Gamepad or Keyboard config, Only one true at once or it wont work.
            keyboard: true,
            gamepad: false,
            aimbot_hot_key_1: 108,
            aimbot_hot_key_2: 109,
            // Done with Gamepad or Keyboard config
            // triggerbot?
            tigger_bot_hot_key: 81,
            autoshoot: true,
            tigger_bot: false,
            // Terminal Stuff
            loot_filled_toggle: true,
            player_filled_toggle: true,
            super_key_toggle: true,
            // end Terminal Stuff
            onevone: false,
            tdm_toggle: false,
            item_glow: true,
            player_glow: false,
            deathbox: false,
            aim_no_recoil: true,
            ads_fov: 15.0, // Fov you want to use while aiming
            non_ads_fov: 50.0,
            aim: 2, // 0 no aim, 1 aim with no vis check, 2 aim with vis check
            esp: true,
            mini_map_radar: true,
            mini_map_guides: true,
            mini_map_radar_dot_size1: 5,
            mini_map_radar_dot_size2: 1,
            main_radar_map: false, // if the Main Map Radar is enabled
            main_map_radar_dot_size1: 5,
            main_map_radar_dot_size2: 5,
            aim_dist: 200.0 * 40.0,
            max_dist: 3800.0 * 40.0, // Max Distance of ESP 3800 is full map
            map_radar_testing: false,
            show_aim_target: true,
            game_fps: 75.0,         // Game FPS for aim prediction
            calc_game_fps: false, // Automatic calculation of game fps
            // aimbot for nades on or off
            no_nade_aim: false,
            firing_range: false,
            bone: 2, // bone 0 head, 1 neck, 2 chest, 3 dick shot
            bone_nearest: false,
            bone_auto: true,
            smooth: 120.0, // min 85 no beaming, 100 somewhat beam people, 125 should be safe
            skynade_smooth: 120.0 * 0.6667,
            // Player Glow Color and Brightness.
            // inside fill
            inside_value: 14, // 0 = no fill, 14 = full fill
            outline_size: 32, // 0-255
            // Not Visable
            glow_r_not: 1.0, // Red 0-1, higher is brighter color.
            glow_g_not: 0.0,
            glow_b_not: 0.0,
            // Visable
            glow_r_viz: 0.0,
            glow_g_viz: 1.0,
            glow_b_viz: 0.0,
            // Knocked
            glow_r_knocked: 1.0,
            glow_g_knocked: 1.0,
            glow_b_knocked: 1.0,

            // Item Configs
            // loot Fill
            loot_filled: 14, // 0 no fill, 14 100% fill
            loot_outline: 0,

            // rev skull
            loot_skull: true,
            // Backpacks
            loot_lightbackpack: false,
            loot_medbackpack: true,
            loot_heavybackpack: true,
            loot_goldbackpack: true,
            // Shield upgrades
            loot_shieldupgrade1: false, // white
            loot_shieldupgrade2: true,  // blue
            loot_shieldupgrade3: true,  // purple
            loot_shieldupgrade4: true,  // gold
            loot_shieldupgrade5: true,  // red
            loot_shieldupgradehead1: false,
            loot_shieldupgradehead2: true,
            loot_shieldupgradehead3: true,
            loot_shieldupgradehead4: true,
            loot_shielddown1: false,
            loot_shielddown2: true,
            loot_shielddown3: true,
            loot_shielddown4: true,
            // heaing and Misc
            loot_accelerant: false,
            loot_phoenix: true,
            loot_healthlarge: true,
            loot_healthsmall: false,
            loot_shieldbattsmall: false,
            loot_shieldbattlarge: true,
            // Ammo
            loot_sniperammo: false,
            loot_heavyammo: true,
            loot_lightammo: true,
            loot_energyammo: true,
            loot_shotgunammo: false,
            // Optics
            loot_optic1xhcog: false,
            loot_optic2xhcog: true,
            loot_opticholo1x: false,
            loot_opticholo1x2x: true,
            loot_opticthreat: false,
            loot_optic3xhcog: true,
            loot_optic2x4x: true,
            loot_opticsniper6x: false,
            loot_opticsniper4x8x: true,
            loot_opticsniperthreat: false,
            // Magazines
            loot_sniperammomag1: false,
            loot_energyammomag1: true,
            loot_lightammomag1: true,
            loot_heavyammomag1: true,
            loot_sniperammomag2: false,
            loot_energyammomag2: true,
            loot_lightammomag2: true,
            loot_heavyammomag2: true,
            loot_sniperammomag3: false,
            loot_energyammomag3: true,
            loot_lightammomag3: true,
            loot_heavyammomag3: true,
            loot_sniperammomag4: false,
            loot_energyammomag4: true,
            loot_lightammomag4: true,
            loot_heavyammomag4: true,
            // Attachments
            loot_lasersight1: false,
            loot_lasersight2: true,
            loot_lasersight3: true,
            loot_lasersight4: true,
            loot_stocksniper1: false,
            loot_stocksniper2: true,
            loot_stocksniper3: true,
            loot_stocksniper4: true,
            loot_stockregular1: false,
            loot_stockregular2: true,
            loot_stockregular3: true,
            loot_suppressor1: false,
            loot_suppressor2: true,
            loot_suppressor3: true,
            loot_turbo_charger: false,
            loot_skull_piecer: false,
            loot_hammer_point: true,
            loot_disruptor_rounds: true,
            loot_boosted_loader: false,
            loot_shotgunbolt1: false,
            loot_shotgunbolt2: false,
            loot_shotgunbolt3: false,
            loot_shotgunbolt4: false,
            // Nades
            loot_grenade_frag: false,
            loot_grenade_arc_star: false,
            loot_grenade_thermite: false,
            // Kraber
            loot_weapon_kraber: true,
            // Shotguns
            loot_weapon_mastiff: false,
            loot_weapon_eva8: false,
            loot_weapon_peacekeeper: false,
            loot_weapon_mozambique: false,
            // Energy weapons
            loot_weapon_lstar: true,
            loot_weapon_nemesis: true,
            loot_weapon_havoc: false,
            loot_weapon_devotion: false,
            loot_weapon_triple_take: false,
            loot_weapon_prowler: false,
            loot_weapon_volt: true,
            // Heavy Weapons
            loot_weapon_flatline: true,
            loot_weapon_hemlock: true,
            loot_weapon_3030_repeater: false,
            loot_weapon_rampage: false,
            loot_weapon_car_smg: true,
            // Light weapons
            loot_weapon_p2020: false,
            loot_weapon_re45: true,
            loot_weapon_g7_scout: false,
            loot_weapon_alternator: false,
            loot_weapon_r99: true,
            loot_weapon_spitfire: true,
            loot_weapon_r301: true,
            // Snipers.. wingman is the odd one...and the bow..
            loot_weapon_wingman: false,
            loot_weapon_longbow: false,
            loot_weapon_charge_rifle: false,
            loot_weapon_sentinel: false,
            loot_weapon_bow: false,
        }
    }
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path;
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("settings.toml")))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    settings.try_deserialize::<Config>()
}

pub fn save_configuration(settings_state: Config) -> Result<(), std::io::Error> {
    use std::fs;
    use std::io::Write;

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path;
    let mut config_write = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(configuration_directory.join("settings.toml"))?;
    let toml_con = toml::to_string(&settings_state).unwrap();
    write!(config_write, "{}", toml_con)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configuration() {
        let configuration = get_configuration().unwrap();
        println!("{:?}", configuration);
    }
}
