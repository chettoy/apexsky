use obfstr::obfstr as s;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

use crate::aimbot::AimbotSettings;
use crate::love_players::LovePlayer;

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub esp_service: EspServiceConfig,
    pub device: DeviceConfig,
    pub settings: Settings,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub love_player: Vec<LovePlayer>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hate_player: Vec<LovePlayer>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EspServiceConfig {
    pub listen: SocketAddr,
    pub accept_http1: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeviceConfig {
    pub kmbox_net_addr: SocketAddr,
    #[serde(with = "hex::serde")]
    pub kmbox_net_mac: [u8; 4],
    pub use_kmbox_net: bool,
    pub kmbox_b_serialport: String,
    pub kmbox_b_baud: u32,
    pub use_kmbox_b: bool,
    pub qemu_qmp_addr: String,
    pub use_qemu_qmp: bool,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Settings {
    pub load_settings: bool,
    pub no_esp_service: bool,
    pub game_ver_dx11: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub yuan_p: bool,
    /// close menu, show debug info
    pub debug_mode: bool,
    pub experimental: bool,
    pub is_access_fast: bool,
    pub super_key: bool,
    pub esp: bool,
    pub max_dist: f32,
    /// Game FPS for aim prediction
    pub game_fps: f32,
    /// Automatic calculation of game fps
    pub calc_game_fps: bool,
    /// Is in firing range
    pub firing_range: bool,
    /// Is in team death match
    pub team_death_match: bool,

    pub esp_visuals: EspVisuals,

    pub aimbot_settings: AimbotSettings,
    pub color_settings: ColorSettings,
    pub feature_settings: FeatureSettings,
    pub glow_settings: GlowSettings,
    pub hotkey_settings: HotkeySettings,

    pub loot: Loot,
}

/// 0-1, higher is brighter color.
#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ColorSettings {
    pub glow_r_not: f32,
    pub glow_g_not: f32,
    pub glow_b_not: f32,
    pub glow_r_viz: f32,
    pub glow_g_viz: f32,
    pub glow_b_viz: f32,
    pub glow_r_knocked: f32,
    pub glow_g_knocked: f32,
    pub glow_b_knocked: f32,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EspVisuals {
    pub bone: bool,
    pub r#box: bool,
    pub line: bool,
    pub distance: bool,
    pub health_bar: bool,
    pub shield_bar: bool,
    pub name: bool,
    pub damage: bool,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct FeatureSettings {
    pub auto_super_glide: bool,
    pub auto_super_grapple: bool,
    pub auto_tap_strafe: bool,
    pub hvh: bool,
    pub kbd_backlight_control: bool,
    pub mini_map_radar: bool,
    pub main_radar_map: bool,
    pub map_radar_testing: bool,
    pub onevone: bool,
    pub show_aim_target: bool,
    pub show_deathbox: bool,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GlowSettings {
    pub loot_filled_toggle: bool,
    pub player_filled_toggle: bool,
    pub item_glow: bool,
    pub player_glow: bool,
    pub player_glow_armor_color: bool,
    pub player_glow_love_user: bool,
    pub weapon_model_glow: bool,
    pub weapon_model_transparent: bool,
    pub player_glow_inside_value: u8,
    pub player_glow_outline_size: u8,
    pub loot_filled: u8,
    pub loot_outline: u8,
}

#[repr(C)]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct HotkeySettings {
    pub aimbot_key1: i32,
    pub aimbot_key2: i32,
    pub triggerbot_key1: i32,
    pub quick_looting_key: i32,
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
    pub shotgunbolt1: bool,
    pub shotgunbolt2: bool,
    pub shotgunbolt3: bool,
    pub shotgunbolt4: bool,
    pub anvil_receiver: bool,
    pub boosted_loader: bool,
    pub disruptor_rounds: bool,
    pub doubletap_trigger: bool,
    pub dual_shell: bool,
    pub gun_shield_generator: bool,
    pub hammer_point: bool,
    pub kinetic_feeder: bool,
    pub quickdraw_holster: bool,
    pub selectfire_receiver: bool,
    pub skull_piecer: bool,
    pub turbo_charger: bool,
    // Nades
    pub grenade_frag: bool,
    pub grenade_arc_star: bool,
    pub grenade_thermite: bool,
    // Supply Drop Weapons
    pub weapon_kraber: bool,
    pub weapon_bow: bool,
    pub weapon_prowler: bool,
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
}

impl Default for EspServiceConfig {
    fn default() -> Self {
        EspServiceConfig {
            listen: s!("[::1]:50051").parse().unwrap(),
            accept_http1: true,
        }
    }
}

impl Default for DeviceConfig {
    fn default() -> Self {
        let data = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            s!("{"),
            s!("\"kmbox_net_addr\":\"127.0.0.1:1234\","),
            s!("\"kmbox_net_mac\":\"48656c6c\","),
            s!("\"use_kmbox_net\": false,"),
            s!("\"kmbox_b_serialport\":\"COM9\","),
            s!("\"kmbox_b_baud\":115200,"),
            s!("\"use_kmbox_b\": false,"),
            s!("\"qemu_qmp_addr\": \"/tmp/qmp-win11.sock\","),
            s!("\"use_qemu_qmp\": false"),
            s!("}")
        );
        serde_json::from_str(&data).unwrap()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            load_settings: true,
            no_esp_service: true,
            game_ver_dx11: false,
            screen_width: 1920,
            screen_height: 1080,
            yuan_p: false,
            debug_mode: false,
            experimental: false,
            is_access_fast: false,
            super_key: true,
            esp: true,
            max_dist: 3800.0 * 40.0, // 3800 is full map
            game_fps: 75.0,
            calc_game_fps: true,
            firing_range: false,
            team_death_match: false,

            esp_visuals: EspVisuals::default(),

            aimbot_settings: AimbotSettings::default(),
            color_settings: ColorSettings::default(),
            feature_settings: FeatureSettings::default(),
            glow_settings: GlowSettings::default(),
            hotkey_settings: HotkeySettings::default(),

            loot: Loot::default(),
        }
    }
}

impl Default for ColorSettings {
    fn default() -> Self {
        ColorSettings {
            // Not Visable
            glow_r_not: 1.0,
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
        }
    }
}

impl Default for EspVisuals {
    fn default() -> Self {
        EspVisuals {
            bone: true,
            r#box: true,
            line: false,
            distance: false,
            health_bar: true,
            shield_bar: true,
            name: false,
            damage: true,
        }
    }
}

impl Default for FeatureSettings {
    fn default() -> Self {
        FeatureSettings {
            auto_super_glide: true,
            auto_super_grapple: true,
            auto_tap_strafe: false,
            hvh: false,
            kbd_backlight_control: false,
            mini_map_radar: true,
            main_radar_map: false,
            map_radar_testing: false,
            onevone: false,
            show_aim_target: true,
            show_deathbox: true,
        }
    }
}

impl Default for GlowSettings {
    fn default() -> Self {
        GlowSettings {
            loot_filled_toggle: true,
            player_filled_toggle: true,
            item_glow: false,
            player_glow: false,
            player_glow_armor_color: true,
            player_glow_love_user: true,
            weapon_model_glow: false,
            weapon_model_transparent: true,

            // Player Glow Color and Brightness.
            // inside fill
            player_glow_inside_value: 101, //12, // 0 = no fill, 14 = full fill
            player_glow_outline_size: 32,  // 0-255

            // Item Configs
            // loot Fill
            loot_filled: 0, // 0 no fill, 14 100% fill
            loot_outline: 0,
        }
    }
}

impl Default for HotkeySettings {
    fn default() -> Self {
        HotkeySettings {
            aimbot_key1: 0,
            aimbot_key2: 0,
            triggerbot_key1: 79,
            quick_looting_key: 0,
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
            shotgunbolt1: false,
            shotgunbolt2: false,
            shotgunbolt3: false,
            shotgunbolt4: false,
            anvil_receiver: false,
            boosted_loader: false,
            disruptor_rounds: true,
            doubletap_trigger: false,
            dual_shell: false,
            gun_shield_generator: false,
            hammer_point: true,
            kinetic_feeder: false,
            quickdraw_holster: false,
            selectfire_receiver: true,
            skull_piecer: false,
            turbo_charger: false,
            // Nades
            grenade_frag: false,
            grenade_arc_star: false,
            grenade_thermite: false,
            // Supply Drop Weapons
            weapon_kraber: true,
            weapon_bow: false,
            weapon_prowler: false,
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
            weapon_volt: false,
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
        }
    }
}

pub fn get_configuration(file_path: &Path) -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::Config::try_from::<Config>(&Config::default())?)
        .add_source(config::File::from(file_path))
        .add_source(config::Environment::with_prefix(s!("APP")))
        .build()?;

    settings.try_deserialize::<Config>()
}

pub fn save_configuration(file_path: &Path, config_state: Config) -> Result<(), std::io::Error> {
    use std::fs;
    use std::io::Write;

    let mut config_write = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let toml_con = toml::to_string(&config_state).unwrap();
    write!(config_write, "{}", toml_con)?;
    Ok(())
}
