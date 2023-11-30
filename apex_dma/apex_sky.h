#pragma once

#include <cstdint>

typedef struct {
  bool load_settings;
  bool super_key;
  bool keyboard;
  bool gamepad;
  int aimbot_hot_key_1;
  int aimbot_hot_key_2;
  int tigger_bot_hot_key;
  bool autoshoot;
  bool tigger_bot;
  bool loot_filled_toggle;
  bool player_filled_toggle;
  bool super_key_toggle;
  bool onevone;
  bool tdm_toggle;
  bool item_glow;
  bool player_glow;
  bool deathbox;
  bool aim_no_recoil;
  float ads_fov;
  float non_ads_fov;
  int32_t aim;
  bool esp;
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
  bool use_overlay_fps;
  bool no_nade_aim;
  bool firing_range;
  int32_t bone;
  bool bone_nearest;
  bool bone_auto;
  float smooth;
  float skynade_smooth;
  uint8_t inside_value;
  uint8_t outline_size;
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
  // rev skull
  bool loot_skull;
  // Backpacks
  bool loot_lightbackpack;
  bool loot_medbackpack;
  bool loot_heavybackpack;
  bool loot_goldbackpack;
  // Shield upgrades
  bool loot_shieldupgrade1; // white
  bool loot_shieldupgrade2; // blue
  bool loot_shieldupgrade3; // purple
  bool loot_shieldupgrade4; // gold
  bool loot_shieldupgrade5; // red
  bool loot_shieldupgradehead1;
  bool loot_shieldupgradehead2;
  bool loot_shieldupgradehead3;
  bool loot_shieldupgradehead4;
  bool loot_shielddown1;
  bool loot_shielddown2;
  bool loot_shielddown3;
  bool loot_shielddown4;
  // heaing and Misc
  bool loot_accelerant;
  bool loot_phoenix;
  bool loot_healthlarge;
  bool loot_healthsmall;
  bool loot_shieldbattsmall;
  bool loot_shieldbattlarge;
  // Ammo
  bool loot_sniperammo;
  bool loot_heavyammo;
  bool loot_lightammo;
  bool loot_energyammo;
  bool loot_shotgunammo;
  // Optics
  bool loot_optic1xhcog;
  bool loot_optic2xhcog;
  bool loot_opticholo1x;
  bool loot_opticholo1x2x;
  bool loot_opticthreat;
  bool loot_optic3xhcog;
  bool loot_optic2x4x;
  bool loot_opticsniper6x;
  bool loot_opticsniper4x8x;
  bool loot_opticsniperthreat;
  // Magazines
  bool loot_sniperammomag1;
  bool loot_energyammomag1;
  bool loot_lightammomag1;
  bool loot_heavyammomag1;
  bool loot_sniperammomag2;
  bool loot_energyammomag2;
  bool loot_lightammomag2;
  bool loot_heavyammomag2;
  bool loot_sniperammomag3;
  bool loot_energyammomag3;
  bool loot_lightammomag3;
  bool loot_heavyammomag3;
  bool loot_sniperammomag4;
  bool loot_energyammomag4;
  bool loot_lightammomag4;
  bool loot_heavyammomag4;
  // Attachments
  bool loot_lasersight1;
  bool loot_lasersight2;
  bool loot_lasersight3;
  bool loot_lasersight4;
  bool loot_stocksniper1;
  bool loot_stocksniper2;
  bool loot_stocksniper3;
  bool loot_stocksniper4;
  bool loot_stockregular1;
  bool loot_stockregular2;
  bool loot_stockregular3;
  bool loot_suppressor1;
  bool loot_suppressor2;
  bool loot_suppressor3;
  bool loot_turbo_charger;
  bool loot_skull_piecer;
  bool loot_hammer_point;
  bool loot_disruptor_rounds;
  bool loot_boosted_loader;
  bool loot_shotgunbolt1;
  bool loot_shotgunbolt2;
  bool loot_shotgunbolt3;
  bool loot_shotgunbolt4;
  // Nades
  bool loot_grenade_frag;
  bool loot_grenade_arc_star;
  bool loot_grenade_thermite;
  // Kraber
  bool loot_weapon_kraber;
  // Shotguns
  bool loot_weapon_mastiff;
  bool loot_weapon_eva8;
  bool loot_weapon_peacekeeper;
  bool loot_weapon_mozambique;
  // Energy weapons
  bool loot_weapon_lstar;
  bool loot_weapon_nemesis;
  bool loot_weapon_havoc;
  bool loot_weapon_devotion;
  bool loot_weapon_triple_take;
  bool loot_weapon_prowler;
  bool loot_weapon_volt;
  // Heavy Weapons
  bool loot_weapon_flatline;
  bool loot_weapon_hemlock;
  bool loot_weapon_3030_repeater;
  bool loot_weapon_rampage;
  bool loot_weapon_car_smg;
  // Light weapons
  bool loot_weapon_p2020;
  bool loot_weapon_re45;
  bool loot_weapon_g7_scout;
  bool loot_weapon_alternator;
  bool loot_weapon_r99;
  bool loot_weapon_spitfire;
  bool loot_weapon_r301;
  // Snipers.. wingman is the odd one...and the bow..
  bool loot_weapon_wingman;
  bool loot_weapon_longbow;
  bool loot_weapon_charge_rifle;
  bool loot_weapon_sentinel;
  bool loot_weapon_bow;
} settings_t;

typedef struct {
  float x;
  float y;
} vector2d_t;

extern "C" {
void print_run_as_root();
uint32_t add(uint32_t lhs, uint32_t rhs);

settings_t load_settings();
bool save_settings(settings_t settings);

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