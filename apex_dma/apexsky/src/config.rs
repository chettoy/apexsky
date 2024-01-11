use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{aimbot::AimbotSettings, love_players::LovePlayer};

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EspVisuals {
    pub r#box: bool,
    pub line: bool,
    pub distance: bool,
    pub health_bar: bool,
    pub shield_bar: bool,
    pub name: bool,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Loot {
    // rev skull
    pub skull: bool,
    // Backpacks
    pub lightbackpack: bool,
    pub medbackpack: bool,
    pub heavybackpack: bool,
    pub goldbackpack: bool,
    // Shield upgrades
    pub shieldupgrade1: bool, // white
    pub shieldupgrade2: bool, // blue
    pub shieldupgrade3: bool, // purple
    pub shieldupgrade4: bool, // gold
    pub shieldupgrade5: bool, // red
    pub shieldupgradehead1: bool,
    pub shieldupgradehead2: bool,
    pub shieldupgradehead3: bool,
    pub shieldupgradehead4: bool,
    pub shielddown1: bool,
    pub shielddown2: bool,
    pub shielddown3: bool,
    pub shielddown4: bool,
    // heaing and Misc
    pub accelerant: bool,
    pub phoenix: bool,
    pub healthlarge: bool,
    pub healthsmall: bool,
    pub shieldbattsmall: bool,
    pub shieldbattlarge: bool,
    // Ammo
    pub sniperammo: bool,
    pub heavyammo: bool,
    pub lightammo: bool,
    pub energyammo: bool,
    pub shotgunammo: bool,
    // Optics
    pub optic1xhcog: bool,
    pub optic2xhcog: bool,
    pub opticholo1x: bool,
    pub opticholo1x2x: bool,
    pub opticthreat: bool,
    pub optic3xhcog: bool,
    pub optic2x4x: bool,
    pub opticsniper6x: bool,
    pub opticsniper4x8x: bool,
    pub opticsniperthreat: bool,
    // Magazines
    pub sniperammomag1: bool,
    pub energyammomag1: bool,
    pub lightammomag1: bool,
    pub heavyammomag1: bool,
    pub sniperammomag2: bool,
    pub energyammomag2: bool,
    pub lightammomag2: bool,
    pub heavyammomag2: bool,
    pub sniperammomag3: bool,
    pub energyammomag3: bool,
    pub lightammomag3: bool,
    pub heavyammomag3: bool,
    pub sniperammomag4: bool,
    pub energyammomag4: bool,
    pub lightammomag4: bool,
    pub heavyammomag4: bool,
    // Attachments
    pub lasersight1: bool,
    pub lasersight2: bool,
    pub lasersight3: bool,
    pub lasersight4: bool,
    pub stocksniper1: bool,
    pub stocksniper2: bool,
    pub stocksniper3: bool,
    pub stocksniper4: bool,
    pub stockregular1: bool,
    pub stockregular2: bool,
    pub stockregular3: bool,
    pub suppressor1: bool,
    pub suppressor2: bool,
    pub suppressor3: bool,
    pub turbo_charger: bool,
    pub skull_piecer: bool,
    pub hammer_point: bool,
    pub disruptor_rounds: bool,
    pub boosted_loader: bool,
    pub shotgunbolt1: bool,
    pub shotgunbolt2: bool,
    pub shotgunbolt3: bool,
    pub shotgunbolt4: bool,
    // Nades
    pub grenade_frag: bool,
    pub grenade_arc_star: bool,
    pub grenade_thermite: bool,
    // Kraber
    pub weapon_kraber: bool,
    // Shotguns
    pub weapon_mastiff: bool,
    pub weapon_eva8: bool,
    pub weapon_peacekeeper: bool,
    pub weapon_mozambique: bool,
    // Energy weapons
    pub weapon_lstar: bool,
    pub weapon_nemesis: bool,
    pub weapon_havoc: bool,
    pub weapon_devotion: bool,
    pub weapon_triple_take: bool,
    pub weapon_prowler: bool,
    pub weapon_volt: bool,
    // Heavy Weapons
    pub weapon_flatline: bool,
    pub weapon_hemlock: bool,
    pub weapon_3030_repeater: bool,
    pub weapon_rampage: bool,
    pub weapon_car_smg: bool,
    // Light weapons
    pub weapon_p2020: bool,
    pub weapon_re45: bool,
    pub weapon_g7_scout: bool,
    pub weapon_alternator: bool,
    pub weapon_r99: bool,
    pub weapon_spitfire: bool,
    pub weapon_r301: bool,
    // Snipers.. wingman is the odd one...and the bow..
    pub weapon_wingman: bool,
    pub weapon_longbow: bool,
    pub weapon_charge_rifle: bool,
    pub weapon_sentinel: bool,
    pub weapon_bow: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub(crate) settings: Settings,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) love_player: Vec<LovePlayer>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) hate_player: Vec<LovePlayer>,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Settings {
    pub load_settings: bool,
    pub no_overlay: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub yuan_p: bool,
    pub debug_mode: bool,
    pub super_key: bool,
    pub aimbot_settings: AimbotSettings,
    pub aimbot_hot_key_1: i32,
    pub aimbot_hot_key_2: i32,
    pub trigger_bot_hot_key: i32,
    pub loot_filled_toggle: bool,
    pub player_filled_toggle: bool,
    pub super_key_toggle: bool,
    pub onevone: bool,
    pub tdm_toggle: bool,
    pub item_glow: bool,
    pub player_glow: bool,
    pub player_glow_armor_color: bool,
    pub player_glow_love_user: bool,
    pub weapon_model_glow: bool,
    pub kbd_backlight_control: bool,
    pub deathbox: bool,
    pub esp: bool,
    pub esp_visuals: EspVisuals,
    pub mini_map_radar: bool,
    pub mini_map_guides: bool,
    pub mini_map_radar_dot_size1: i32,
    pub mini_map_radar_dot_size2: i32,
    pub main_radar_map: bool,
    pub main_map_radar_dot_size1: i32,
    pub main_map_radar_dot_size2: i32,
    pub max_dist: f32,
    pub map_radar_testing: bool,
    pub show_aim_target: bool,
    pub game_fps: f32,
    pub calc_game_fps: bool,
    pub firing_range: bool,
    pub player_glow_inside_value: u8,
    pub player_glow_outline_size: u8,
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
    pub loot: Loot,
}

impl Default for EspVisuals {
    fn default() -> Self {
        Self {
            r#box: true,
            line: false,
            distance: false,
            health_bar: false,
            shield_bar: false,
            name: false,
        }
    }
}

impl Default for Loot {
    fn default() -> Self {
        Self {
            // rev skull
            skull: true,
            // Backpacks
            lightbackpack: false,
            medbackpack: true,
            heavybackpack: true,
            goldbackpack: true,
            // Shield upgrades
            shieldupgrade1: false, // white
            shieldupgrade2: true,  // blue
            shieldupgrade3: true,  // purple
            shieldupgrade4: true,  // gold
            shieldupgrade5: true,  // red
            shieldupgradehead1: false,
            shieldupgradehead2: true,
            shieldupgradehead3: true,
            shieldupgradehead4: true,
            shielddown1: false,
            shielddown2: true,
            shielddown3: true,
            shielddown4: true,
            // heaing and Misc
            accelerant: false,
            phoenix: true,
            healthlarge: true,
            healthsmall: false,
            shieldbattsmall: false,
            shieldbattlarge: true,
            // Ammo
            sniperammo: false,
            heavyammo: true,
            lightammo: true,
            energyammo: true,
            shotgunammo: false,
            // Optics
            optic1xhcog: false,
            optic2xhcog: true,
            opticholo1x: false,
            opticholo1x2x: true,
            opticthreat: false,
            optic3xhcog: true,
            optic2x4x: true,
            opticsniper6x: false,
            opticsniper4x8x: true,
            opticsniperthreat: false,
            // Magazines
            sniperammomag1: false,
            energyammomag1: true,
            lightammomag1: true,
            heavyammomag1: true,
            sniperammomag2: false,
            energyammomag2: true,
            lightammomag2: true,
            heavyammomag2: true,
            sniperammomag3: false,
            energyammomag3: true,
            lightammomag3: true,
            heavyammomag3: true,
            sniperammomag4: false,
            energyammomag4: true,
            lightammomag4: true,
            heavyammomag4: true,
            // Attachments
            lasersight1: false,
            lasersight2: true,
            lasersight3: true,
            lasersight4: true,
            stocksniper1: false,
            stocksniper2: true,
            stocksniper3: true,
            stocksniper4: true,
            stockregular1: false,
            stockregular2: true,
            stockregular3: true,
            suppressor1: false,
            suppressor2: true,
            suppressor3: true,
            turbo_charger: false,
            skull_piecer: false,
            hammer_point: true,
            disruptor_rounds: true,
            boosted_loader: false,
            shotgunbolt1: false,
            shotgunbolt2: false,
            shotgunbolt3: false,
            shotgunbolt4: false,
            // Nades
            grenade_frag: false,
            grenade_arc_star: false,
            grenade_thermite: false,
            // Kraber
            weapon_kraber: true,
            // Shotguns
            weapon_mastiff: false,
            weapon_eva8: false,
            weapon_peacekeeper: false,
            weapon_mozambique: false,
            // Energy weapons
            weapon_lstar: true,
            weapon_nemesis: true,
            weapon_havoc: false,
            weapon_devotion: false,
            weapon_triple_take: false,
            weapon_prowler: false,
            weapon_volt: true,
            // Heavy Weapons
            weapon_flatline: true,
            weapon_hemlock: true,
            weapon_3030_repeater: false,
            weapon_rampage: false,
            weapon_car_smg: true,
            // Light weapons
            weapon_p2020: false,
            weapon_re45: true,
            weapon_g7_scout: false,
            weapon_alternator: false,
            weapon_r99: true,
            weapon_spitfire: true,
            weapon_r301: true,
            // Snipers.. wingman is the odd one...and the bow..
            weapon_wingman: false,
            weapon_longbow: false,
            weapon_charge_rifle: false,
            weapon_sentinel: false,
            weapon_bow: false,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // CONFIG AREA, you can change default values below.
            // Enable Loading of setting file automaticly.
            load_settings: true,
            no_overlay: true,
            screen_width: 1920,
            screen_height: 1080,
            yuan_p: false,
            debug_mode: false,
            super_key: true,
            // Gamepad or Keyboard config, Only one true at once or it wont work.
            aimbot_settings: AimbotSettings::default(),
            aimbot_hot_key_1: 108,
            aimbot_hot_key_2: 109,
            // Done with Gamepad or Keyboard config
            // triggerbot?
            trigger_bot_hot_key: 81,
            // Terminal Stuff
            loot_filled_toggle: true,
            player_filled_toggle: true,
            super_key_toggle: true,
            // end Terminal Stuff
            onevone: false,
            tdm_toggle: false,
            item_glow: false,
            player_glow: false,
            player_glow_armor_color: true,
            player_glow_love_user: true,
            weapon_model_glow: false,
            kbd_backlight_control: false,
            deathbox: false,
            esp: true,
            esp_visuals: EspVisuals::default(),
            mini_map_radar: true,
            mini_map_guides: true,
            mini_map_radar_dot_size1: 5,
            mini_map_radar_dot_size2: 1,
            main_radar_map: false, // if the Main Map Radar is enabled
            main_map_radar_dot_size1: 5,
            main_map_radar_dot_size2: 5,
            max_dist: 3800.0 * 40.0, // Max Distance of ESP 3800 is full map
            map_radar_testing: false,
            show_aim_target: true,
            game_fps: 75.0,       // Game FPS for aim prediction
            calc_game_fps: false, // Automatic calculation of game fps
            firing_range: false,
            // Player Glow Color and Brightness.
            // inside fill
            player_glow_inside_value: 14, // 0 = no fill, 14 = full fill
            player_glow_outline_size: 32, // 0-255
            // Not Visable
            glow_r_not: 1.0, // Red 0-1, higher is brighter color.
            glow_g_not: 0.0,
            glow_b_not: 0.0,
            // Visable
            glow_r_viz: 0.0,
            glow_g_viz: 1.0,
            glow_b_viz: 0.0,
            // Knocked
            glow_r_knocked: 0.80,
            glow_g_knocked: 0.78,
            glow_b_knocked: 0.45,

            // Item Configs
            // loot Fill
            loot_filled: 14, // 0 no fill, 14 100% fill
            loot_outline: 0,

            loot: Loot::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            love_player: Default::default(),
            hate_player: Default::default(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path;
    configuration_directory.join("settings.toml")
}

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::Config::try_from::<Config>(&Config::default())?)
        .add_source(config::File::from(get_config_path()))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    settings.try_deserialize::<Config>()
}

pub fn save_configuration(config_state: Config) -> Result<(), std::io::Error> {
    use std::fs;
    use std::io::Write;

    let mut config_write = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(get_config_path())?;
    let toml_con = toml::to_string(&config_state).unwrap();
    write!(config_write, "{}", toml_con)?;
    Ok(())
}
