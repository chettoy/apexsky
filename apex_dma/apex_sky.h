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
  float recoil_smooth_x;
  float recoil_smooth_y;
} aimbot_settings_t;

typedef struct {
  bool valid;
  float view_pitch;
  float view_yew;
  float delta_pitch;
  float delta_yew;
  float delta_pitch_min;
  float delta_pitch_max;
  float delta_yew_min;
  float delta_yew_max;
  float distance;
} aim_angles_t;

typedef struct {
  aimbot_settings_t settings;
  bool aiming;
  bool gun_safety;
  bool lock;
  int32_t attack_state;
  int32_t zoom_state;
  int32_t aim_key_state;
  int32_t triggerbot_key_state;
  int32_t held_id;
  int32_t weapon_id;
  float bullet_speed;
  float bullet_gravity;
  float weapon_zoom_fov;
  int weapon_mod_bitfield;
  bool weapon_grenade;
  bool weapon_headshot;
  bool weapon_semi_auto;
  float max_fov;
  float target_score_max;
  uintptr_t local_entity;
  uintptr_t aim_entity;
  uintptr_t tmp_aimentity;
  uintptr_t locked_aimentity;
  bool love_aimentity;
  float game_fps;
  int triggerbot_state;
  uint64_t triggerbot_trigger_time;
  uint64_t triggerbot_release_time;
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
  aimbot_settings_t aimbot_settings;
  int aimbot_hot_key_1;
  int aimbot_hot_key_2;
  int trigger_bot_hot_key;
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

typedef struct {
  settings_t settings;
  bool terminal_t;
} global_state_t;

typedef struct {
  float x;
  float y;
  float z;
  float w;
} vec4_t;

typedef struct {
  uintptr_t offset_entitylist;
  uintptr_t offset_local_ent;
  uintptr_t offset_name_list;
  uintptr_t offset_global_vars;
  uintptr_t offset_levelname;
  uintptr_t offset_clientstate;
  uintptr_t offset_signonstate;
  uintptr_t offset_host_map;
  uintptr_t offset_entity_team;
  uintptr_t offset_player_health;
  uintptr_t offset_entity_shield;
  uintptr_t offset_entity_maxshield;
  uintptr_t offset_player_helmettype;
  uintptr_t offset_player_armortype;
  uintptr_t offset_entiry_name;
  uintptr_t offset_entity_sign_name;
  uintptr_t offset_centity_abs_velocity;
  uintptr_t offset_visible_time;
  uintptr_t offset_player_zooming;
  uintptr_t offset_traversal_progress;
  uintptr_t offset_traversal_starttime;
  uintptr_t offset_platform_uid;
  uintptr_t offset_weapon_name;
  uintptr_t offset_off_weapon;
  uintptr_t offset_wall_run_start_time;
  uintptr_t offset_wall_run_clear_time;
  uintptr_t offset_centity_flags;
  uintptr_t offset_in_attack;
  uintptr_t offset_in_toggle_duck;
  uintptr_t offset_in_zoom;
  uintptr_t offset_in_forward;
  uintptr_t offset_in_jump;
  uintptr_t offset_in_duck;
  uintptr_t offset_in_use;
  uintptr_t offset_player_life_state;
  uintptr_t offset_bleed_out_state;
  uintptr_t offset_centity_viewoffset;
  uintptr_t offset_centity_origin;
  uintptr_t offset_bones;
  uintptr_t offset_studiohdr;
  uintptr_t offset_cplayer_aimpunch;
  uintptr_t offset_cplayer_camerapos;
  uintptr_t offset_player_viewangles;
  uintptr_t offset_breath_angles;
  uintptr_t offset_observer_mode;
  uintptr_t offset_ovserver_target;
  uintptr_t offset_matrix;
  uintptr_t offset_render;
  uintptr_t offset_primary_weapon;
  uintptr_t offset_active_weapon;
  uintptr_t offset_bullet_speed;
  uintptr_t offset_bullet_scale;
  uintptr_t offset_weaponx_zoom_fov;
  uintptr_t offset_weaponx_ammo_in_clip;
  uintptr_t offset_centity_modelname;
  uintptr_t offset_cplayer_timebase;
  uintptr_t offset_cplayer_viewmodels;
  uintptr_t offset_crosshair_last;
  uintptr_t offset_input_system;
  uintptr_t offset_weaponx_bitfield_from_player;
  uintptr_t offset_entity_highlight_generic_context;
} exported_offsets_t;

extern "C" {
void print_run_as_root();
int32_t add(int32_t lhs, int32_t rhs);
bool kbd_backlight_blink(int32_t count);

global_state_t __get_global_states();
void __update_global_states(global_state_t state);
void __load_settings();
bool save_settings();

void run_tui_menu();

bool check_love_player(uint64_t puid, uint64_t euid, const char *name,
                       uint64_t entity_ptr);

void init_spec_checker(uintptr_t local_player_ptr);
void tick_yew(uintptr_t target_ptr, float yew);
bool is_spec(uintptr_t target_ptr);

aimbot_state_t aimbot_new();
aimbot_settings_t aimbot_get_settings(const aimbot_state_t *aimbot);
void aimbot_settings(aimbot_state_t *aimbot, const aimbot_settings_t *settings);
bool aimbot_is_aiming(const aimbot_state_t *aimbot);
bool aimbot_is_grenade(const aimbot_state_t *aimbot);
bool aimbot_is_headshot(const aimbot_state_t *aimbot);
bool aimbot_is_semi_auto(const aimbot_state_t *aimbot);
bool aimbot_is_locked(const aimbot_state_t *aimbot);
bool aimbot_is_triggerbot_ready(const aimbot_state_t *aimbot);
float aimbot_get_max_fov(const aimbot_state_t *aimbot);
int aimbot_get_held_id(const aimbot_state_t *aimbot);
void aimbot_update_held_id(aimbot_state_t *aimbot, int held_id);
int aimbot_get_weapon_id(const aimbot_state_t *aimbot);
void aimbot_update_weapon_info(aimbot_state_t *aimbot, int weapon_id,
                               float bullet_speed, float bullet_gravity,
                               float weapon_zoom_fov, int weapon_mod_bitfield);
bool aimbot_get_gun_safety(const aimbot_state_t *aimbot);
void aimbot_set_gun_safety(aimbot_state_t *aimbot, bool gun_safety);
int aimbot_get_aim_key_state(const aimbot_state_t *aimbot);
void aimbot_update_aim_key_state(aimbot_state_t *aimbot, int aim_key_state);
void aimbot_update_triggerbot_key_state(aimbot_state_t *aimbot,
                                        int triggerbot_key_state);
void aimbot_update_attack_state(aimbot_state_t *aimbot, int attack_state);
void aimbot_update_zoom_state(aimbot_state_t *aimbot, int zoom_state);
uint64_t aimbot_get_aim_entity(const aimbot_state_t *aimbot);
bool aimbot_target_distance_check(const aimbot_state_t *aimbot, float distance);
void aimbot_start_select_target(aimbot_state_t *aimbot);
void aimbot_add_select_target(aimbot_state_t *aimbot, float fov, float distance,
                              bool visible, bool love, uint64_t target_ptr);
void aimbot_finish_select_target(aimbot_state_t *aimbot);
void aimbot_lock_target(aimbot_state_t *aimbot, uint64_t target_ptr);
void aimbot_cancel_locking(aimbot_state_t *aimbot);
void aimbot_update(aimbot_state_t *aimbot, uintptr_t local_entity,
                   float game_fps);
vec4_t aimbot_smooth_aim_angles(const aimbot_state_t *aimbot,
                                const aim_angles_t *aim_angles,
                                float smooth_factor);
int aimbot_poll_trigger_action(aimbot_state_t *aimbot);
void aimbot_triggerbot_update(aimbot_state_t *aimbot,
                              const aim_angles_t *aim_angles,
                              int force_attack_state);

exported_offsets_t export_offsets();

/**
 * https://github.com/CasualX/apexdream
 * LISENCE: GPLv3
 */
vec4_t skynade_angle(uint32_t weapon_id, uint32_t weapon_mod_bitfield,
                     float weapon_projectile_scale,
                     float weapon_projectile_speed, float local_view_origin_x,
                     float local_view_origin_y, float local_view_origin_z,
                     float target_x, float target_y, float target_z);
vec4_t linear_predict(float weapon_projectile_grav,
                      float weapon_projectile_speed, float local_x,
                      float local_y, float local_z, float target_x,
                      float target_y, float target_z, float vel_x, float vel_y,
                      float vel_z);
}

void load_settings();
const settings_t global_settings();
void update_settings(settings_t state);
void quit_tui_menu();

const exported_offsets_t offsets = export_offsets();