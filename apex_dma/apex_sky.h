#pragma once

#include <cstdint>

typedef struct aimbot_state_t {
  bool aiming = false;
  bool gun_safety = true;
  bool lock = false;
  float max_fov = 10;
  float target_score_max;
  uintptr_t aimentity = 0;
  uintptr_t tmp_aimentity = 0;
  uintptr_t locked_aimentity = 0;
} aimbot_state_t;

typedef struct {
  bool box;
  bool line;
  bool distance;
  bool healthbar;
  bool shieldbar;
  bool name;
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
  bool shotgunbolt1;
  bool shotgunbolt2;
  bool shotgunbolt3;
  bool shotgunbolt4;
  // Nades
  bool grenade_frag;
  bool grenade_arc_star;
  bool grenade_thermite;
  // Kraber
  bool weapon_kraber;
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
  bool weapon_prowler;
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
  bool weapon_bow;
} loots;

typedef struct {
  bool load_settings;
  bool no_overlay;
  uint32_t screen_width;
  uint32_t screen_height;
  bool yuan_p;
  bool debug_mode;
  bool super_key;
  bool keyboard;
  bool gamepad;
  int aimbot_hot_key_1;
  int aimbot_hot_key_2;
  int trigger_bot_hot_key;
  bool auto_shoot;
  bool loot_filled_toggle;
  bool player_filled_toggle;
  bool super_key_toggle;
  bool onevone;
  bool tdm_toggle;
  bool item_glow;
  bool player_glow;
  bool player_glow_armor_color;
  bool player_glow_love_user;
  bool weapon_model_glow;
  bool kbd_backlight_control;
  bool deathbox;
  bool aim_no_recoil;
  float ads_fov;
  float non_ads_fov;
  int32_t aim;
  bool esp;
  visuals esp_visuals;
  bool mini_map_radar;
  bool mini_map_guides;
  int32_t mini_map_radar_dot_size1;
  int32_t mini_map_radar_dot_size2;
  bool main_radar_map;
  int32_t main_map_radar_dot_size1;
  int32_t main_map_radar_dot_size2;
  float aim_dist;
  float max_dist;
  bool map_radar_testing;
  bool show_aim_target;
  float game_fps;
  bool calc_game_fps;
  bool no_nade_aim;
  bool firing_range;
  int32_t bone;
  bool bone_nearest;
  bool bone_auto;
  float headshot_dist;
  float skynade_dist;
  float smooth;
  float skynade_smooth;
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

typedef struct {
  settings_t settings;
  bool terminal_t;
} global_state_t;

typedef struct {
  float x;
  float y;
} vector2d_t;

extern "C" {
void print_run_as_root();
uint32_t add(uint32_t lhs, uint32_t rhs);
bool kbd_backlight_blink(int32_t count);

global_state_t __get_global_states();
void __update_global_states(global_state_t state);
void __load_settings();
bool save_settings();

void run_tui_menu();

bool check_love_player(uint64_t puid, uint64_t euid, const char *name);

/**
 * https://github.com/CasualX/apexdream
 * LISENCE: GPLv3
 */
vector2d_t skynade_angle(uint32_t weapon_id, uint32_t weapon_mod_bitfield,
                         float weapon_projectile_scale,
                         float weapon_projectile_speed,
                         float local_view_origin_x, float local_view_origin_y,
                         float local_view_origin_z, float target_x,
                         float target_y, float target_z);
}

void load_settings();
const settings_t global_settings();
void update_settings(settings_t state);
void quit_tui_menu();