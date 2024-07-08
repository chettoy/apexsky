#pragma once

#include <cstdint>

typedef struct {
  bool gamepad;
  int32_t aim_mode;
  bool auto_shoot;
  float ads_fov;
  float non_ads_fov;
  bool auto_nade_aim;
  bool no_recoil;
  int32_t bone;
  bool bone_nearest;
  bool bone_auto;
  float max_dist;
  float aim_dist;
  float headshot_dist;
  float skynade_dist;
  float smooth;
  float skynade_smooth;
  float looting_smooth;
  float recoil_smooth_x;
  float recoil_smooth_y;
} aimbot_settings_t;

typedef struct {
  bool box;
  bool line;
  bool distance;
  bool health_bar;
  bool shield_bar;
  bool name;
  bool damage;
} visuals;

typedef struct {
  // rev skull
  bool skull;
  // Backpacks
  bool lightbackpack;
  bool medbackpack;
  bool heavybackpack;
  bool goldbackpack;
  // Shield upgrades
  bool shieldupgrade1; // white
  bool shieldupgrade2; // blue
  bool shieldupgrade3; // purple
  bool shieldupgrade4; // gold
  bool shieldupgrade5; // red
  bool shieldupgradehead1;
  bool shieldupgradehead2;
  bool shieldupgradehead3;
  bool shieldupgradehead4;
  bool shielddown1;
  bool shielddown2;
  bool shielddown3;
  bool shielddown4;
  // heaing and Misc
  bool accelerant;
  bool phoenix;
  bool healthlarge;
  bool healthsmall;
  bool shieldbattsmall;
  bool shieldbattlarge;
  // Ammo
  bool sniperammo;
  bool heavyammo;
  bool lightammo;
  bool energyammo;
  bool shotgunammo;
  // Optics
  bool optic1xhcog;
  bool optic2xhcog;
  bool opticholo1x;
  bool opticholo1x2x;
  bool opticthreat;
  bool optic3xhcog;
  bool optic2x4x;
  bool opticsniper6x;
  bool opticsniper4x8x;
  bool opticsniperthreat;
  // Magazines
  bool sniperammomag1;
  bool energyammomag1;
  bool lightammomag1;
  bool heavyammomag1;
  bool sniperammomag2;
  bool energyammomag2;
  bool lightammomag2;
  bool heavyammomag2;
  bool sniperammomag3;
  bool energyammomag3;
  bool lightammomag3;
  bool heavyammomag3;
  bool sniperammomag4;
  bool energyammomag4;
  bool lightammomag4;
  bool heavyammomag4;
  // Attachments
  bool lasersight1;
  bool lasersight2;
  bool lasersight3;
  bool lasersight4;
  bool stocksniper1;
  bool stocksniper2;
  bool stocksniper3;
  bool stocksniper4;
  bool stockregular1;
  bool stockregular2;
  bool stockregular3;
  bool suppressor1;
  bool suppressor2;
  bool suppressor3;
  bool turbo_charger;
  bool skull_piecer;
  bool hammer_point;
  bool disruptor_rounds;
  bool boosted_loader;
  bool anvil_receiver;
  bool doubletap_trigger;
  bool dual_shell;
  bool kinetic_feeder;
  bool quickdraw_holster;
  bool shotgunbolt1;
  bool shotgunbolt2;
  bool shotgunbolt3;
  bool shotgunbolt4;
  // Nades
  bool grenade_frag;
  bool grenade_arc_star;
  bool grenade_thermite;
  // Supply Drop Weapons
  bool weapon_kraber;
  bool weapon_bow;
  bool weapon_prowler;
  // Shotguns
  bool weapon_mastiff;
  bool weapon_eva8;
  bool weapon_peacekeeper;
  bool weapon_mozambique;
  // Energy weapons
  bool weapon_lstar;
  bool weapon_nemesis;
  bool weapon_havoc;
  bool weapon_devotion;
  bool weapon_triple_take;
  bool weapon_volt;
  // Heavy Weapons
  bool weapon_flatline;
  bool weapon_hemlock;
  bool weapon_3030_repeater;
  bool weapon_rampage;
  bool weapon_car_smg;
  // Light weapons
  bool weapon_p2020;
  bool weapon_re45;
  bool weapon_g7_scout;
  bool weapon_alternator;
  bool weapon_r99;
  bool weapon_spitfire;
  bool weapon_r301;
  // Snipers.. wingman is the odd one...and the bow..
  bool weapon_wingman;
  bool weapon_longbow;
  bool weapon_charge_rifle;
  bool weapon_sentinel;
} loots;

typedef struct {
  bool load_settings;
  bool no_overlay;
  uint32_t screen_width;
  uint32_t screen_height;
  bool yuan_p;
  bool debug_mode;
  bool super_key;
  aimbot_settings_t aimbot_settings;
  int aimbot_hot_key_1;
  int aimbot_hot_key_2;
  int trigger_bot_hot_key;
  int quick_looting_hot_key;
  bool loot_filled_toggle;
  bool player_filled_toggle;
  bool super_key_toggle;
  bool super_grpple;
  bool auto_tapstrafe;
  bool onevone;
  bool tdm_toggle;
  bool item_glow;
  bool player_glow;
  bool player_glow_armor_color;
  bool player_glow_love_user;
  bool weapon_model_glow;
  bool kbd_backlight_control;
  bool deathbox;
  bool esp;
  visuals esp_visuals;
  bool mini_map_radar;
  bool mini_map_guides;
  int32_t mini_map_radar_dot_size1;
  int32_t mini_map_radar_dot_size2;
  bool main_radar_map;
  int32_t main_map_radar_dot_size1;
  int32_t main_map_radar_dot_size2;
  float max_dist;
  bool map_radar_testing;
  bool show_aim_target;
  float game_fps;
  bool calc_game_fps;
  bool firing_range;
  uint8_t player_glow_inside_value;
  uint8_t player_glow_outline_size;
  float glow_r_not;
  float glow_g_not;
  float glow_b_not;
  float glow_r_viz;
  float glow_g_viz;
  float glow_b_viz;
  float glow_r_knocked;
  float glow_g_knocked;
  float glow_b_knocked;
  uint8_t loot_filled;
  uint8_t loot_outline;
  loots loot;
} settings_t;
