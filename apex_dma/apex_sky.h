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
  int32_t weapon_id;
  float bullet_speed;
  float bullet_gravity;
  float weapon_zoom_fov;
  uint32_t weapon_mod_bitfield;
  bool weapon_headshot;
  bool weapon_semi_auto;
} current_weapon_info_t;

typedef struct {
  aimbot_settings_t settings;
  bool aiming;
  bool gun_safety;
  bool lock;
  bool triggerbot_ready;
  int32_t attack_state;
  int32_t zoom_state;
  int32_t aim_key_state;
  int32_t triggerbot_key_state;
  int32_t held_id;
  bool held_grenade;
  current_weapon_info_t weapon_info;
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

typedef struct {
  settings_t settings;
  bool terminal_t;
  bool tui_forceupdate;
} global_state_t;

typedef struct {
  float x;
  float y;
  float z;
  float w;
} vec4_t;

typedef enum {
  NORMAL = 0,
  LOVE = 1,
  HATE = 2,
  AMBIVALENT = 3,
} LoveStatus;

typedef struct {
  uintptr_t time_date_stamp;
  uintptr_t checksum;
  uintptr_t global_vars;
  uintptr_t entitylist;
  uintptr_t local_entity_handle;
  uintptr_t local_ent;
  uintptr_t input_selected_slot;
  uintptr_t client_state;
  uintptr_t signon_state;
  uintptr_t level_name;
  uintptr_t nst_weapon_names;
  uintptr_t view_render;
  uintptr_t view_matrix;
  uintptr_t input_system;
  uintptr_t input_button_state;
  uintptr_t name_list;
  uintptr_t network_var_table_ptr;
  uintptr_t network_var_table_len;
  uintptr_t host_map;
  uintptr_t thirdperson_override;
  uintptr_t mouse_sensitivity;
  uintptr_t fps_max;
  uintptr_t mp_gamemode;
  uintptr_t in_attack;
  uintptr_t in_jump;
  uintptr_t in_duck;
  uintptr_t in_reload;
  uintptr_t in_use;
  uintptr_t in_zoom;
  uintptr_t in_forward;
  uintptr_t in_backward;
  uintptr_t in_moveleft;
  uintptr_t in_moveright;
  uintptr_t in_toggle_duck;
  uintptr_t in_left;
  uintptr_t in_right;
  uintptr_t in_strafe;
  uintptr_t centity_modelname;
  uintptr_t centity_viewoffset;
  uintptr_t centity_flags;
  uintptr_t centity_origin;
  uintptr_t entity_shieldhealth;
  uintptr_t entity_maxshieldhealth;
  uintptr_t entity_highlight_generic_context;
  uintptr_t entity_team_num;
  uintptr_t centity_abs_velocity;
  uintptr_t centity_velocity;
  uintptr_t entity_owner_entity;
  uintptr_t entiry_name;
  uintptr_t entity_sign_name;
  uintptr_t entity_fade_dist;
  uintptr_t animating_skin;
  uintptr_t animating_bone_array;
  uintptr_t bones;
  uintptr_t animating_studiohdr;
  uintptr_t bcc_next_attack;
  uintptr_t bcc_inventory;
  uintptr_t bcc_selected_weapons;
  uintptr_t bcc_off_weapon;
  uintptr_t bcc_primary_weapon;
  uintptr_t bcc_active_weapon;
  uintptr_t bcc_last_visible_time;
  uintptr_t player_last_visible_time;
  uintptr_t player_zooming;
  uintptr_t cplayer_camerapos;
  uintptr_t cplayer_timebase;
  uintptr_t cplayer_server_angles;
  uintptr_t cplayer_aimpunch;
  uintptr_t cplayer_viewmodels;
  uintptr_t cplayer_traversal_progress;
  uintptr_t cplayer_traversal_starttime;
  uintptr_t cplayer_wall_run_start_time;
  uintptr_t cplayer_wall_run_clear_time;
  uintptr_t player_viewangles;
  uintptr_t player_consumables;
  uintptr_t player_observer_state;
  uintptr_t player_ovserver_target;
  uintptr_t player_platform_uid;
  uintptr_t player_health;
  uintptr_t player_maxhealth;
  uintptr_t player_bleed_out_state;
  uintptr_t player_life_state;
  uintptr_t player_duck_state;
  uintptr_t player_lean_state;
  uintptr_t player_grapple;
  uintptr_t player_grapple_active;
  uintptr_t player_xp;
  uintptr_t player_third_person_shoulder_view;
  uintptr_t player_net_var;
  uintptr_t player_helmettype;
  uintptr_t player_armortype;
  uintptr_t player_controller_active;
  uintptr_t player_skydive_state;
  uintptr_t player_breath_angles;
  uintptr_t weaponx_weapon_owner;
  uintptr_t weaponx_next_primary_attack;
  uintptr_t weaponx_ammo_in_clip;
  uintptr_t weaponx_zoom_fov;
  uintptr_t weaponx_charge_start_time;
  uintptr_t weaponx_charge_end_time;
  uintptr_t weaponx_last_charge_frac;
  uintptr_t cweaponx_burst_fire;
  uintptr_t cweaponx_bullet_speed;
  uintptr_t cweaponx_bullet_scale;
  uintptr_t cweaponx_crosshair_last;
  uintptr_t weaponx_bitfield_from_player;
  uintptr_t weaponx_weapon_name_index;
  uintptr_t weaponx_is_semi_auto;
  uintptr_t weaponx_projectile_speed;
  uintptr_t vehicle_driver;
  uintptr_t vehicle_velocity;
  uintptr_t prop_survival;
  uintptr_t projectile;
  uintptr_t world_death_field;
  uintptr_t waypoint_type;
  uintptr_t mods_names;
  uintptr_t mods_list;
  uintptr_t mods_count;
  uintptr_t grapple_attached;
  uintptr_t grapple_pulling;
  uintptr_t var_damage;
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

LoveStatus check_love_player(uint64_t puid, uint64_t euid, const char *name,
                             uint64_t entity_ptr);

void init_spec_checker(uintptr_t local_player_ptr);
void tick_yew(uintptr_t target_ptr, float yew);
bool is_spec(uintptr_t target_ptr);

aimbot_state_t aimbot_get_state();
aimbot_settings_t aimbot_get_settings();
void aimbot_settings(const aimbot_settings_t *settings);
bool aimbot_is_aiming();
bool aimbot_is_grenade();
bool aimbot_is_headshot();
bool aimbot_is_semi_auto();
bool aimbot_is_locked();
bool aimbot_is_triggerbot_ready();
float aimbot_get_max_fov();
int aimbot_get_held_id();
void aimbot_update_held_id(int held_id);
int aimbot_get_weapon_id();
void aimbot_update_weapon_info(int weapon_id, float bullet_speed,
                               float bullet_gravity, float weapon_zoom_fov,
                               int weapon_mod_bitfield);
bool aimbot_get_gun_safety();
void aimbot_set_gun_safety(bool gun_safety);
int aimbot_get_aim_key_state();
void aimbot_update_aim_key_state(int aim_key_state);
void aimbot_update_triggerbot_key_state(int triggerbot_key_state);
void aimbot_update_attack_state(int attack_state);
void aimbot_update_zoom_state(int zoom_state);
uint64_t aimbot_get_aim_entity();
bool aimbot_target_distance_check(float distance);
void aimbot_start_select_target();
void aimbot_add_select_target(float fov, float distance, bool visible,
                              bool love, uint64_t target_ptr);
void aimbot_finish_select_target();
void aimbot_lock_target(uint64_t target_ptr);
void aimbot_cancel_locking();
void aimbot_update(uintptr_t local_entity, float game_fps);
vec4_t aimbot_smooth_aim_angles(const aim_angles_t *aim_angles,
                                float smooth_factor);
int aimbot_poll_trigger_action();
void aimbot_triggerbot_update(const aim_angles_t *aim_angles,
                              int force_attack_state);

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

exported_offsets_t import_offsets();
}

void load_settings();
const settings_t global_settings();
void update_settings(settings_t state);
void tui_menu_quit();
void tui_menu_forceupdate();

const exported_offsets_t offsets = import_offsets();