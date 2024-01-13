#include "Client/main.h"
#include "Game.h"
#include "apex_sky.h"
#include "vector.h"
#include <array>
#include <cassert>
#include <cfloat>
#include <chrono>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstdlib> // For the system() function
#include <iomanip>
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <thread>
#include <unistd.h>
#include <unordered_map> // Include the unordered_map header
#include <vector>
// this is a test, with seconds
Memory apex_mem;
extern const exported_offsets_t offsets;

// Just setting things up, dont edit.
bool active = true;
aimbot_state_t aimbot;
const int toRead = 100;
extern Vector aim_target; // for esp
int map_testing_local_team = 0;

// Removed but not all the way, dont edit.
int glowtype;
int glowtype2;
// float triggerdist = 50.0f;
bool actions_t = false;
bool cactions_t = false;
bool terminal_t = false;
bool overlay_t = false;
bool esp_t = false;
bool aim_t = false;
bool item_t = false;
bool control_t = false;
uint64_t g_Base;
bool next2 = false;
bool valid = false;
extern float bulletspeed;
extern float bulletgrav;
Vector esp_local_pos;
int playerentcount = 61;
int itementcount = 10000;
int map = 0;
std::vector<TreasureClue> treasure_clues;

//^^ Don't EDIT^^

// [del]CONFIG AREA, you must set all the true/false to what you want.[/del]
// No longer needed here. Edit your configuration file!

std::vector<uint64_t> wish_list{191, 209, 210, 220,          234,
                                242, 258, 260, 429496729795, 52776987629977800};

uint32_t button_state[4];
bool isPressed(uint32_t button_code) {
  return (button_state[static_cast<uint32_t>(button_code) >> 5] &
          (1 << (static_cast<uint32_t>(button_code) & 0x1f))) != 0;
}

void memory_io_panic(const char *info) {
  tui_menu_quit();
  std::cout << "Error " << info << std::endl;
  exit(0);
}

// Define rainbow color function
void rainbowColor(int frame_number, std::array<float, 3> &colors) {
  const float frequency = 0.1; // Adjust the speed of color change
  const float amplitude = 0.5; // Adjust the amplitude of color change

  // Use the sine function to generate rainbow color variation
  float r = sin(frequency * frame_number + 0) * amplitude + 0.5;
  float g = sin(frequency * frame_number + 2) * amplitude + 0.5;
  float b = sin(frequency * frame_number + 4) * amplitude + 0.5;

  // Clamp the colors to the range [0, 1]
  colors[0] = fmax(0, fmin(1, r));
  colors[1] = fmax(0, fmin(1, g));
  colors[2] = fmax(0, fmin(1, b));
}

void TriggerBotRun() {
  // testing
  // apex_mem.Write<int>(g_Base + offsets.offset_in_attack + 0x8, 4);
  // std::this_thread::sleep_for(std::chrono::milliseconds(10));
  apex_mem.Write<int>(g_Base + offsets.in_attack + 0x8, 5);
  std::this_thread::sleep_for(std::chrono::milliseconds(500));
  apex_mem.Write<int>(g_Base + offsets.in_attack + 0x8, 4);
  // printf("TriggerBotRun\n");
}

bool IsInCrossHair(Entity &target) {
  static uintptr_t last_t = 0;
  static float last_crosshair_target_time = -1.f;
  float now_crosshair_target_time = target.lastCrossHairTime();
  bool is_in_cross_hair = false;
  if (last_t == target.ptr) {
    if (last_crosshair_target_time != -1.f) {
      if (now_crosshair_target_time > last_crosshair_target_time) {
        is_in_cross_hair = true;
        // printf("Trigger\n");
        last_crosshair_target_time = -1.f;
      } else {
        is_in_cross_hair = false;
        last_crosshair_target_time = now_crosshair_target_time;
      }
    } else {
      is_in_cross_hair = false;
      last_crosshair_target_time = now_crosshair_target_time;
    }
  } else {
    last_t = target.ptr;
    last_crosshair_target_time = -1.f;
  }
  return is_in_cross_hair;
}

// Visual check and aim check.?
float lastvis_esp[toRead];
float lastvis_aim[toRead];
std::vector<Entity> spectators, allied_spectators;
std::mutex spectatorsMtx;

void MapRadarTesting() {
  uintptr_t pLocal;
  apex_mem.Read<uint64_t>(g_Base + offsets.local_ent, pLocal);
  int dt;
  apex_mem.Read<int>(pLocal + offsets.entity_team, dt);
  map_testing_local_team = dt;

  for (uintptr_t i = 0; i <= 80000; i++) {
    apex_mem.Write<int>(pLocal + offsets.entity_team, 1);
  }

  for (uintptr_t i = 0; i <= 80000; i++) {
    apex_mem.Write<int>(pLocal + offsets.entity_team, dt);
  }
  map_testing_local_team = 0;
}

void ClientActions() {
  cactions_t = true;
  while (cactions_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(10));

    while (g_Base != 0) {
      const auto g_settings = global_settings();

      // read player ptr
      uint64_t local_player_ptr = 0;
      apex_mem.Read<uint64_t>(g_Base + offsets.local_ent, local_player_ptr);

      // read game states
      apex_mem.Read<typeof(button_state)>(g_Base + offsets.input_system + 0xb0,
                                          button_state);

      int attack_state = 0, zoom_state = 0, tduck_state = 0, jump_state = 0,
          force_jump = 0, force_toggle_duck = 0, force_duck = 0,
          curFrameNumber = 0;
      apex_mem.Read<int>(g_Base + offsets.in_attack,
                         attack_state);                         // 108
      apex_mem.Read<int>(g_Base + offsets.in_zoom, zoom_state); // 109
      apex_mem.Read<int>(g_Base + offsets.in_toggle_duck,
                         tduck_state); // 61
      apex_mem.Read<int>(g_Base + offsets.in_jump, jump_state);
      apex_mem.Read<int>(g_Base + offsets.in_jump + 0x8, force_jump);
      apex_mem.Read<int>(g_Base + offsets.in_toggle_duck + 0x8,
                         force_toggle_duck);
      apex_mem.Read<int>(g_Base + offsets.in_duck + 0x8, force_duck);
      apex_mem.Read<int>(g_Base + offsets.global_vars + 0x0008,
                         curFrameNumber); // GlobalVars + 0x0008

      float world_time, traversal_start_time, traversal_progress;
      if (!apex_mem.Read<float>(local_player_ptr + offsets.cplayer_timebase,
                                world_time)) {
        // memory_io_panic("read time_base");
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        break;
      }
      if (!apex_mem.Read<float>(local_player_ptr +
                                    offsets.cplayer_traversal_starttime,
                                traversal_start_time)) {
        // memory_io_panic("read traversal_starttime");
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        break;
      }
      if (!apex_mem.Read<float>(local_player_ptr +
                                    offsets.cplayer_traversal_progress,
                                traversal_progress)) {
        memory_io_panic("read traversal_progress");
      }

      //   printf("Travel Time: %f\n", traversal_progress);
      //   printf("Cur Frame: %i\n", curFrameNumber);
      //   printf("Jump Value: %i\n", jump_state);
      //   printf("Jump Value: %i\n", force_jump);
      //   printf("ToggleDuck Value: %i\n", force_toggle_duck);
      //   printf("Duck Value: %i\n", force_duck);

      if (g_settings.super_key_toggle) {
        /** SuperGlide
         * https://www.unknowncheats.me/forum/apex-legends/578160-external-auto-superglide-3.html
         */
        float hang_on_wall = world_time - traversal_start_time;

        static float start_jump_time = 0;
        static bool start_sg = false;
        static std::chrono::milliseconds last_sg_finish;

        float hang_start, hang_cancel, trav_start, hang_max, action_interval;
        int release_wait;
        {
          // for 75 fps
          hang_start = 0.1;
          hang_cancel = 0.12;
          trav_start = 0.87;
          hang_max = 1.5;
          action_interval = 0.011;
          release_wait = 50;
          if (abs(g_settings.game_fps - 144.0) <
              abs(g_settings.game_fps - 75.0)) {
            // for 144 fps
            hang_start = 0.05;
            hang_cancel = 0.07;
            trav_start = 0.90;
            hang_max = 0.75;
            action_interval = 0.007;
            release_wait = 25;
            if (abs(g_settings.game_fps - 240.0) <
                abs(g_settings.game_fps - 144.0)) {
              // for 240 fps
              hang_start = 0.033;
              hang_cancel = 0.04;
              trav_start = 0.95;
              hang_max = 0.2;
              action_interval = 0.004;
              release_wait = 20;
            }
          }
        }

        if (hang_on_wall > hang_start) {
          if (hang_on_wall < hang_cancel) {
            apex_mem.Write<int>(g_Base + offsets.in_jump + 0x8, 4);
          }
          if (traversal_progress > trav_start && hang_on_wall < hang_max &&
              !start_sg) {
            std::chrono::milliseconds now_ms =
                duration_cast<std::chrono::milliseconds>(
                    std::chrono::system_clock::now().time_since_epoch());
            if ((now_ms - last_sg_finish).count() > 320 && jump_state > 0) {
              // start SG
              start_jump_time = world_time;
              start_sg = true;
            }
          }
        }
        if (start_sg) {
          // press button
          // g_logger += "sg Press jump\n";
          apex_mem.Write<int>(g_Base + offsets.in_jump + 0x8, 5);

          float current_time;
          while (true) {
            if (apex_mem.Read<float>(local_player_ptr +
                                         offsets.cplayer_timebase,
                                     current_time)) {
              if (current_time - start_jump_time < action_interval) {
                // keep looping
              } else {
                break;
              }
            }
          }
          apex_mem.Write<int>(g_Base + offsets.in_duck + 0x8, 6);
          std::this_thread::sleep_for(std::chrono::milliseconds(release_wait));
          apex_mem.Write<int>(g_Base + offsets.in_jump + 0x8, 4);
          // Write<int>(g_Base + offsets.offset_in_duck + 0x8, 4);
          last_sg_finish = duration_cast<std::chrono::milliseconds>(
              std::chrono::system_clock::now().time_since_epoch());
          // g_logger += "sg\n";
          start_sg = false;
        }
      }

      { /* calc game fps */
        static int last_checkpoint_frame = 0;
        static std::chrono::milliseconds checkpoint_time;
        if (g_settings.calc_game_fps && curFrameNumber % 100 == 0) {
          std::chrono::milliseconds ms =
              duration_cast<std::chrono::milliseconds>(
                  std::chrono::system_clock::now().time_since_epoch());
          int delta_frame = curFrameNumber - last_checkpoint_frame;
          if (delta_frame > 90 && delta_frame < 120) {
            auto duration = ms - checkpoint_time;
            auto settings_state = g_settings;
            settings_state.game_fps = delta_frame * 1000.0f / duration.count();
            update_settings(settings_state);
          }
          last_checkpoint_frame = curFrameNumber;
          checkpoint_time = ms;
        }
      }

      if (isPressed(g_settings.aimbot_hot_key_1)) {
        aimbot_update_aim_key_state(&aimbot, g_settings.aimbot_hot_key_1);
      } else if (isPressed(g_settings.aimbot_hot_key_2)) {
        aimbot_update_aim_key_state(&aimbot, g_settings.aimbot_hot_key_2);
      } else {
        aimbot_update_aim_key_state(&aimbot, 0);
      }

      aimbot_update_attack_state(&aimbot, attack_state);
      aimbot_update_zoom_state(&aimbot, zoom_state);

      if (isPressed(g_settings.trigger_bot_hot_key)) {
        aimbot_update_triggerbot_key_state(&aimbot,
                                           g_settings.trigger_bot_hot_key);
      } else {
        aimbot_update_triggerbot_key_state(&aimbot, 0);
      }

      // Trigger ring check on F8 key press for over 0.5 seconds
      static std::chrono::steady_clock::time_point tduckStartTime;
      static bool mapRadarTestingEnabled = false;
      if (g_settings.map_radar_testing && isPressed(99)) { // KEY_F8
        if (mapRadarTestingEnabled) {
          MapRadarTesting();
        }

        if (tduckStartTime == std::chrono::steady_clock::time_point()) {
          tduckStartTime = std::chrono::steady_clock::now();
        }

        auto currentTime = std::chrono::steady_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(
                            currentTime - tduckStartTime)
                            .count();
        if (duration >= 250) {
          mapRadarTestingEnabled = true;
        }
      } else {
        tduckStartTime = std::chrono::steady_clock::time_point();
        mapRadarTestingEnabled = false;
      }

      std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
  }
  cactions_t = false;
}

void ControlLoop() {
  control_t = true;
  while (control_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    spectatorsMtx.lock();
    int spec_count = spectators.size();
    spectatorsMtx.unlock();
    if (spec_count > 0) {
      kbd_backlight_blink(spec_count);
      std::this_thread::sleep_for(std::chrono::milliseconds(10 * 1000 - 100));
    }
  }
  control_t = false;
}

void SetPlayerGlow(Entity &LPlayer, Entity &Target, int index,
                   int frame_number) {
  const auto g_settings = global_settings();
  int setting_index = 0;
  std::array<float, 3> highlight_parameter = {0, 0, 0};

  // if (!Target.isGlowing() ||
  //     (int)Target.buffer[OFFSET_GLOW_VISIBLE_TYPE] != 1)
  {
    float currentEntityTime = 5000.f;
    if (!isnan(currentEntityTime) && currentEntityTime > 0.f) {
      // set glow color
      if (!(g_settings.firing_range) &&
          (Target.isKnocked() || !Target.isAlive())) {
        setting_index = 80;
        highlight_parameter = {g_settings.glow_r_knocked,
                               g_settings.glow_g_knocked,
                               g_settings.glow_b_knocked};
      } else if (Target.lastVisTime() > lastvis_aim[index] ||
                 (Target.lastVisTime() < 0.f && lastvis_aim[index] > 0.f)) {
        setting_index = 81;
        highlight_parameter = {g_settings.glow_r_viz, g_settings.glow_g_viz,
                               g_settings.glow_b_viz};
      } else {
        if (g_settings.player_glow_armor_color) {
          int shield = Target.getShield();
          int health = Target.getHealth();
          if (shield + health <= 100) { // Orange
            setting_index = 91;
            highlight_parameter = {255 / 255.0, 165 / 255.0, 0 / 255.0};
          } else if (shield + health <= 150) { // white
            setting_index = 92;
            highlight_parameter = {247 / 255.0, 247 / 255.0, 247 / 255.0};
          } else if (shield + health <= 175) { // blue
            setting_index = 93;
            highlight_parameter = {39 / 255.0, 178 / 255.0, 255 / 255.0};
          } else if (shield + health <= 200) { // purple
            setting_index = 94;
            highlight_parameter = {206 / 255.0, 59 / 255.0, 255 / 255.0};
          } else if (shield + health <= 225) { // red
            setting_index = 95;
            highlight_parameter = {219 / 255.0, 2 / 255.0, 2 / 255.0};
          } else {
            setting_index = 90;
            highlight_parameter = {2 / 255.0, 2 / 255.0, 2 / 255.0};
          }
        } else {
          setting_index = 82;
          highlight_parameter = {g_settings.glow_r_not, g_settings.glow_g_not,
                                 g_settings.glow_b_not};
        }
      }
      // love player glow
      if (g_settings.player_glow_love_user) {
        int frame_frag = frame_number / ((int)g_settings.game_fps);
        if (setting_index == 81 ||
            frame_frag % 2 == 0) { // vis: always, else: 1s time slice
          auto at = Target.check_love_player();
          if (at == LOVE) {
            setting_index = 96;
            rainbowColor(frame_number, highlight_parameter);
          } else if (at == HATE) {
            setting_index = 90;
            highlight_parameter = {2 / 255.0, 2 / 255.0, 2 / 255.0};
          }
        }
      }

      // enable glow
      if (g_settings.player_glow) {
        Target.enableGlow(setting_index, g_settings.player_glow_inside_value,
                          g_settings.player_glow_outline_size,
                          highlight_parameter);
      } else {
        Target.disableGlow();
      }
    }
  }
}

void ProcessPlayer(Entity &LPlayer, Entity &target, uint64_t entitylist,
                   int index, int frame_number,
                   std::set<uintptr_t> &tmp_specs) {
  const auto g_settings = global_settings();

  int entity_team = target.getTeamId();
  int local_team = LPlayer.getTeamId();
  // printf("Target Team: %i\n", entity_team);

  if (!target.isAlive() || !LPlayer.isAlive()) {
    // Update yew to spec checker
    tick_yew(target.ptr, target.GetYaw());
    // Exclude self from list when watching others
    if (target.ptr != LPlayer.ptr && is_spec(target.ptr)) {
      tmp_specs.insert(target.ptr);
    }
    return;
  }

  if (g_settings.tdm_toggle) { // Check if the target entity is on the same
                               // team as the
                               // local player
    int EntTeam, LocTeam;
    if (entity_team % 2)
      EntTeam = 1;
    else
      EntTeam = 2;
    if (local_team % 2)
      LocTeam = 1;
    else
      LocTeam = 2;

    // printf("Target Team: %i\nLocal Team: %i\n", EntTeam, LocTeam);
    if (EntTeam == LocTeam)
      return;
  }

  // Firing range stuff
  if (!g_settings.firing_range) {
    if (entity_team < 0 || entity_team > 50 ||
        (entity_team == local_team && !g_settings.onevone)) {
      return;
    }
    if (map_testing_local_team != 0 && entity_team == map_testing_local_team) {
      return;
    }
  }

  if (target.ptr != LPlayer.ptr) {
    // Targeting
    Vector EntityPosition = target.getPosition();
    Vector LocalPlayerPosition = LPlayer.getPosition();
    float dist = LocalPlayerPosition.DistTo(EntityPosition);
    float fov = CalculateFov(LPlayer, target);
    bool vis = target.lastVisTime() > lastvis_aim[index];
    bool love = target.check_love_player();
    aimbot_add_select_target(&aimbot, fov, dist, vis, love, target.ptr);

    // Player Glow
    SetPlayerGlow(LPlayer, target, index, frame_number);
  }

  // For vis check
  lastvis_aim[index] = target.lastVisTime();
}

// Main stuff, dont edit.
void DoActions() {
  actions_t = true;
  while (actions_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(10));

    while (g_Base != 0) {
      std::this_thread::sleep_for(
          std::chrono::milliseconds(30)); // don't change xD

      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + offsets.local_ent, LocalPlayer);
      if (LocalPlayer == 0)
        continue;

      char level_name[200] = {0};
      apex_mem.ReadArray<char>(g_Base + offsets.levelname, level_name, 200);
      // printf("%s\n", level_name);
      if (strcmp(level_name, "mp_lobby") == 0) {
        map = 0;
      } else if (strcmp(level_name, "mp_rr_canyonlands_staging_mu1") == 0) {
        map = 1;
      } else if (strcmp(level_name, "mp_rr_tropic_island_mu1_storm") == 0) {
        map = 2;
      } else if (strcmp(level_name, "mp_rr_desertlands_hu") == 0) {
        map = 3;
      } else if (strcmp(level_name, "mp_rr_olympus") == 0) {
        map = 4;
      } else if (strcmp(level_name, "mp_rr_divided_moon") == 0) {
        map = 5;
      } else {
        map = -1;
      }

      {
        int pad = 0;
        apex_mem.Read<int>(LocalPlayer + offsets.player_controller_active, pad);
        bool controller_active = pad == 1;
        bool firing_range_mode = map == 1;

        bool update = true;
        auto settings = global_settings();
        if (settings.aimbot_settings.gamepad != controller_active) {
          settings.aimbot_settings.gamepad = controller_active;
        } else if (settings.firing_range != firing_range_mode) {
          settings.firing_range = firing_range_mode;
        } else {
          update = false;
        }

        if (update) {
          update_settings(settings);
          tui_menu_forceupdate();
        }
      }

      const auto g_settings = global_settings();

      if (g_settings.firing_range) {
        playerentcount = 16000;
      } else {
        playerentcount = 61;
      }
      if (g_settings.deathbox) {
        itementcount = 15000;
      } else {
        itementcount = 10000;
      }

      Entity LPlayer = getEntity(LocalPlayer);

      const int team_player = LPlayer.getTeamId();
      if (team_player < 0 || team_player > 50) {
        continue;
      }
      uint64_t entitylist = g_Base + offsets.entitylist;

      uint64_t baseent = 0;
      apex_mem.Read<uint64_t>(entitylist, baseent);
      if (baseent == 0) {
        continue;
      }

      { // Init spectator checker
        static uintptr_t prev_lplayer_ptr = 0;
        if (prev_lplayer_ptr != LocalPlayer) {
          prev_lplayer_ptr = LocalPlayer;
          init_spec_checker(LocalPlayer);
        }
        // Update local entity yew
        tick_yew(LocalPlayer, LPlayer.GetYaw());
      }

      int frame_number = 0;
      apex_mem.Read<int>(g_Base + offsets.global_vars + 0x0008, frame_number);

      std::set<uintptr_t> tmp_specs;
      aimbot_start_select_target(&aimbot);

      if (g_settings.firing_range) {
        int c = 0;
        for (int i = 0; i < playerentcount; i++) {
          uint64_t centity = 0;
          apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
          if (centity == 0) {
            continue;
          }
          if (LocalPlayer == centity) {
            continue;
          }

          Entity Target = getEntity(centity);
          if (!(Target.isDummy() ||
                (g_settings.onevone && Target.isPlayer()))) {
            continue;
          }

          ProcessPlayer(LPlayer, Target, entitylist, c, frame_number,
                        tmp_specs);
          c++;
        }
      } else {

        for (int i = 0; i < toRead; i++) {
          uint64_t centity = 0;
          apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
          if (centity == 0) {
            continue;
          }
          if (LocalPlayer == centity) {
            continue;
          }

          Entity Target = getEntity(centity);
          if (!Target.isPlayer()) {
            continue;
          }

          ProcessPlayer(LPlayer, Target, entitylist, i, frame_number,
                        tmp_specs);
        }
      }

      { // refresh spectators count
        std::vector<Entity> tmp_spec, tmp_all_spec;
        spectatorsMtx.lock();
        for (auto it = tmp_specs.begin(); it != tmp_specs.end(); it++) {
          Entity target = getEntity(*it);
          if (target.getTeamId() == team_player) {
            tmp_all_spec.push_back(target);
          } else {
            tmp_spec.push_back(target);
          }
        }
        spectators.clear();
        allied_spectators.clear();
        spectators = tmp_spec;
        allied_spectators = tmp_all_spec;
        spectatorsMtx.unlock();
      }

      aimbot_finish_select_target(&aimbot);

      // weapon model glow
      // printf("%d\n", LPlayer.getHealth());
      if (g_settings.weapon_model_glow && LPlayer.getHealth() > 0) {
        std::array<float, 3> highlight_color;
        if (spectators.size() > 0) {
          highlight_color = {1, 0, 0};
          LPlayer.glow_weapon_model(true, true, highlight_color);
        } else if (allied_spectators.size() > 0) {
          highlight_color = {0, 1, 0};
          LPlayer.glow_weapon_model(true, true, highlight_color);
        } else {
          rainbowColor(frame_number, highlight_color);
          LPlayer.glow_weapon_model(true, false, highlight_color);
        }
        // printf("R: %f, G: %f, B: %f\n", highlight_color[0],
        // highlight_color[1], highlight_color[2]);

        // LPlayer.enableGlow(5, 199, 14, 32, highlight_color);
      } else {
        LPlayer.glow_weapon_model(false, true, {0, 0, 0});
      }
    }
  }
  actions_t = false;
}

// /////////////////////////////////////////////////////////////////////////////////////////////////////

std::vector<player> players(toRead);
Matrix view_matrix_data = {};

// ESP loop.. this helps right?
static void EspLoop() {
  esp_t = true;
  while (esp_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(5));
    while (g_Base != 0 && overlay_t) {
      std::this_thread::sleep_for(std::chrono::milliseconds(2));
      const auto g_settings = global_settings();

      if (g_settings.esp) {
        valid = false;

        uint64_t LocalPlayer = 0;
        apex_mem.Read<uint64_t>(g_Base + offsets.local_ent, LocalPlayer);
        if (LocalPlayer == 0) {
          next2 = true;
          while (next2 && g_Base != 0 && overlay_t && g_settings.esp) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
          }
          continue;
        }
        Entity LPlayer = getEntity(LocalPlayer);
        int team_player = LPlayer.getTeamId();
        if (team_player < 0 || team_player > 50) {
          next2 = true;
          while (next2 && g_Base != 0 && overlay_t && g_settings.esp) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
          }
          continue;
        }
        Vector LocalPlayerPosition = LPlayer.getPosition();
        esp_local_pos = LocalPlayerPosition;

        uint64_t viewRenderer = 0;
        apex_mem.Read<uint64_t>(g_Base + offsets.view_render, viewRenderer);
        uint64_t viewMatrix = 0;
        apex_mem.Read<uint64_t>(viewRenderer + offsets.view_matrix, viewMatrix);

        apex_mem.Read<Matrix>(viewMatrix, view_matrix_data);

        uint64_t entitylist = g_Base + offsets.entitylist;

        players.clear();

        {
          Vector LocalPlayerPosition = LPlayer.getPosition();
          QAngle localviewangle = LPlayer.GetViewAngles();

          // Ammount of ents to loop, dont edit.
          for (int i = 0; i < toRead; i++) {
            // Read entity pointer
            uint64_t centity = 0;
            apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
            if (centity == 0) {
              continue;
            }

            // Exclude self
            if (LocalPlayer == centity) {
              continue;
            }

            // Get entity data
            Entity Target = getEntity(centity);

            // Exclude undesired entity
            if (g_settings.firing_range) {
              if (!(Target.isDummy() ||
                    (g_settings.onevone && Target.isPlayer()))) {
                continue;
              }
            } else {
              if (!Target.isPlayer()) {
                continue;
              }
            }

            int entity_team = Target.getTeamId();

            // Exclude invalid team
            if (entity_team < 0 || entity_team > 50) {
              continue;
            }

            // Exlude teammates if not 1v1
            if (entity_team == team_player && !g_settings.onevone) {
              continue;
            }
            // if (map_testing_local_team != 0 &&
            //     entity_team == map_testing_local_team) {
            //   continue;
            // }

            Vector EntityPosition = Target.getPosition();
            float dist = LocalPlayerPosition.DistTo(EntityPosition);

            // Excluding targets that are too far or too close
            if (dist > g_settings.max_dist || dist < 20.0f) {
              continue;
            }

            Vector bs = Vector();
            // Change res to your res here, default is 1080p but can copy paste
            // 1440p here
            WorldToScreen(EntityPosition, view_matrix_data.matrix,
                          g_settings.screen_width, g_settings.screen_height,
                          bs); // 2560, 1440
            if (g_settings.esp) {
              Vector hs = Vector();
              Vector HeadPosition = Target.getBonePositionByHitbox(0);
              WorldToScreen(HeadPosition, view_matrix_data.matrix,
                            g_settings.screen_width, g_settings.screen_height,
                            hs); // 2560, 1440
              float height = abs(abs(hs.y) - abs(bs.y));
              float width = height / 2.0f;
              float boxMiddle = bs.x - (width / 2.0f);
              int health = Target.getHealth();
              int shield = Target.getShield();
              int maxshield = Target.getMaxshield();
              int armortype = Target.getArmortype();
              Vector EntityPosition = Target.getPosition();
              float targetyaw = Target.GetYaw();
              player data_buf = {dist,
                                 entity_team,
                                 boxMiddle,
                                 hs.y,
                                 width,
                                 height,
                                 bs.x,
                                 bs.y,
                                 Target.isKnocked(),
                                 (Target.lastVisTime() > lastvis_esp[i]),
                                 health,
                                 shield,
                                 maxshield,
                                 Target.read_xp_level(),
                                 armortype,
                                 EntityPosition,
                                 LocalPlayerPosition,
                                 localviewangle,
                                 targetyaw,
                                 Target.isAlive(),
                                 Target.check_love_player(),
                                 false};
              Target.get_name(data_buf.name);
              spectatorsMtx.lock();
              for (auto &ent : spectators) {
                if (ent.ptr == centity) {
                  data_buf.is_spectator = true;
                  break;
                }
              }
              spectatorsMtx.unlock();
              players.push_back(data_buf);
              lastvis_esp[i] = Target.lastVisTime();
              valid = true;
            }
          }
        }

        next2 = true;
        while (next2 && g_Base != 0 && overlay_t && g_settings.esp) {
          std::this_thread::sleep_for(std::chrono::milliseconds(1));
        }
      }
    }
  }
  esp_t = false;
}

// Aimbot Loop stuff
static void AimbotLoop() {
  aim_t = true;
  while (aim_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(30));
    while (g_Base != 0) {
      std::this_thread::sleep_for(std::chrono::milliseconds(10));

      static std::chrono::milliseconds last_time =
          duration_cast<std::chrono::milliseconds>(
              std::chrono::system_clock::now().time_since_epoch());
      std::chrono::milliseconds now_ms =
          duration_cast<std::chrono::milliseconds>(
              std::chrono::system_clock::now().time_since_epoch());
      float smooth_factor = (now_ms - last_time).count() / 1.054571726;
      // printf("smooth_factor=%f\n", smooth_factor);
      last_time = now_ms;

      const auto g_settings = global_settings();

      // Read LocalPlayer
      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + offsets.local_ent, LocalPlayer);
      if (LocalPlayer == 0) {
        continue;
      }
      Entity LPlayer = getEntity(LocalPlayer);

      { // Read held id
        int held_id;
        apex_mem.Read<int>(LocalPlayer + offsets.off_weapon,
                           held_id); // 0x1a1c
        aimbot_update_held_id(&aimbot, held_id);
      }

      { // Read weapon info
        WeaponXEntity current_weapon = WeaponXEntity();
        current_weapon.update(LocalPlayer);
        uint32_t weap_id = current_weapon.get_weap_id();
        float bullet_speed = current_weapon.get_projectile_speed();
        float bullet_grav = current_weapon.get_projectile_gravity();
        float zoom_fov = current_weapon.get_zoom_fov();
        int weapon_mod_bitfield = current_weapon.get_mod_bitfield();
        aimbot_update_weapon_info(&aimbot, weap_id, bullet_speed, bullet_grav,
                                  zoom_fov, weapon_mod_bitfield);
      }

      { // Update aimbot settings
        static int i = 0;
        if (i == 0) {
          aimbot_settings(&aimbot, &g_settings.aimbot_settings);
        }
        if (i > 500) { // Lower update frequency to reduce cpu usage
          i = 0;
        } else {
          i++;
        }
      }

      const auto aimbot_settings = aimbot_get_settings(&aimbot);
      const auto aim_entity = aimbot_get_aim_entity(&aimbot);
      const auto weapon_id = aimbot_get_weapon_id(&aimbot);
      const bool aiming = aimbot_is_aiming(&aimbot);
      const bool trigger_bot_ready = aimbot_is_triggerbot_ready(&aimbot);

      {
        int trigger_value = aimbot_poll_trigger_action(&aimbot);
        if (trigger_value) {
          apex_mem.Write<int>(g_Base + offsets.in_attack + 0x8, trigger_value);
        }
      }

      // Reduce recoil
      static QAngle prev_recoil_angle = QAngle(0, 0, 0);
      if (aimbot_settings.no_recoil && aimbot.attack_state > 0) {
        // get recoil angle
        QAngle recoil_angles = LPlayer.GetRecoil();

        // get original angles
        QAngle old_view_angles = LPlayer.GetViewAngles();

        QAngle new_angle = old_view_angles;
        // printf("prev=%f, recoil=%f\n", oldRecoilAngle.x, recoilAngles.x);

        // removing recoil angles from player view angles
        new_angle.x += ((prev_recoil_angle.x - recoil_angles.x) *
                        (aimbot_settings.recoil_smooth_x / 100.f));
        new_angle.y += ((prev_recoil_angle.y - recoil_angles.y) *
                        (aimbot_settings.recoil_smooth_y / 100.f));

        // setting viewangles to new angles
        LPlayer.SetViewAngles(new_angle);
        // setting old recoil angles to current recoil angles
        prev_recoil_angle = recoil_angles;
        // normalize view angles
        Math::NormalizeAngles(prev_recoil_angle);
      } else {
        prev_recoil_angle = QAngle(0, 0, 0);
      }

      // Update Aimbot state
      aimbot_update(&aimbot, LocalPlayer, g_settings.game_fps);

      aim_angles_t aim_result;

      if (aim_entity == 0) {
        aim_result = aim_angles_t{false};
        aimbot_cancel_locking(&aimbot);
      } else {
        Entity target = getEntity(aim_entity);

        // show target indicator before aiming
        aim_target = target.getPosition();

        if (!(aiming || trigger_bot_ready)) {
          aim_result = aim_angles_t{false};
        } else if (aimbot_get_gun_safety(&aimbot)) {
          aim_result = aim_angles_t{false};
        } else if (LPlayer.isKnocked() || !target.isAlive() ||
                   (!g_settings.firing_range && target.isKnocked())) {
          aim_result = aim_angles_t{false};
          aimbot_cancel_locking(&aimbot);
        } else {
          // Caculate Aim Angles

          /* Fine-tuning for each weapon */
          if (weapon_id == 2) { // bow
            // Ctx.BulletSpeed = BulletSpeed - (BulletSpeed*0.08);
            // Ctx.BulletGravity = BulletGrav + (BulletGrav*0.05);
            bulletspeed = 10.08;
            bulletgrav = 10.05;
          }

          aim_result = CalculateBestBoneAim(LPlayer, target, aimbot);
          if (!aim_result.valid) {
            aimbot_cancel_locking(&aimbot);
          }
        }
      }

      // Update Trigger Bot state
      int force_attack_state;
      apex_mem.Read(g_Base + offsets.in_attack + 0x8, force_attack_state);
      // Ensure that the triggerbot is updated,
      // otherwise there may be issues with not canceling after firing.
      aimbot_triggerbot_update(&aimbot, &aim_result, force_attack_state);

      // Aim Assist
      if (aiming && aim_result.valid) {
        auto smoothed_angles =
            aimbot_smooth_aim_angles(&aimbot, &aim_result, smooth_factor);
        LPlayer.SetViewAngles(
            QAngle(smoothed_angles.x, smoothed_angles.y, smoothed_angles.z));
      }

    } // end loop
  }   // end AimbotLoop
  aim_t = false;
}

// Item Glow Stuff
static void item_glow_t() {
  item_t = true;
  while (item_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(30));
    while (g_Base != 0) {
      std::this_thread::sleep_for(std::chrono::milliseconds(60));
      const auto g_settings = global_settings();
      if (!g_settings.item_glow) {
        break;
      }

      uint64_t entitylist = g_Base + offsets.entitylist;
      // item ENTs to loop, 10k-15k is normal. 10k might be better but will
      // not show all the death boxes i think.

      // for wish list
      std::vector<TreasureClue> new_treasure_clues;
      for (size_t i = 0; i < wish_list.size(); i++) {
        TreasureClue clue;
        clue.item_id = wish_list[i];
        clue.position = Vector(0, 0, 0);
        clue.distance = g_settings.aimbot_settings.aim_dist * 2;
        new_treasure_clues.push_back(clue);
      }

      for (int i = 0; i < itementcount; i++) {
        uint64_t centity = 0;
        apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
        if (centity == 0)
          continue;
        Item item = getItem(centity);

        // Item filter glow name setup and search.
        char glowName[200] = {0};
        uint64_t name_ptr;
        apex_mem.Read<uint64_t>(centity + offsets.centity_modelname, name_ptr);
        apex_mem.ReadArray<char>(name_ptr, glowName, 200);

        // item ids?
        uint64_t ItemID;
        apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
        /* uint64_t ItemID2;
        ItemID2 = ItemID % 301;
        printf("%ld\n", ItemID2); */
        // printf("Model Name: %s, Item ID: %lu\n", glowName, ItemID);
        // Level name printf
        // char LevelNAME[200] = { 0 };
        // uint64_t levelname_ptr;
        // apex_mem.Read<uint64_t>(g_Base + offsets.offset_levelname,
        // levelname_ptr); apex_mem.ReadArray<char>(levelname_ptr, LevelNAME,
        // 200);

        // printf("%s\n", LevelNAME);

        // Prints stuff you want to console
        // if (strstr(glowName, "mdl/"))
        //{
        // printf("%ld\n", ItemID);
        // }
        // Search model name and if true sets glow, must be a better way to do
        // this.. if only i got the item id to work..

        for (size_t i = 0; i < new_treasure_clues.size(); i++) {
          TreasureClue &clue = new_treasure_clues[i];
          if (ItemID == new_treasure_clues[i].item_id) {
            Vector position = item.getPosition();
            float distance = esp_local_pos.DistTo(position);
            if (distance < clue.distance) {
              clue.position = position;
              clue.distance = distance;
            }
            break;
          }
        }

        const std::array<unsigned char, 4> highlightFunctionBits = {
            g_settings
                .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
            125,              // OutlineFunction OutlineFunction
                              // HIGHLIGHT_OUTLINE_LOOT_SCANNED
            64, 64};
        if (g_settings.loot.lightbackpack && ItemID == 207) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.medbackpack && ItemID == 208) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.heavybackpack && ItemID == 209) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.goldbackpack && ItemID == 210) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.shieldupgrade1 &&
                   (ItemID == 214748364993 || ItemID == 14073963583897798)) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.shieldupgrade2 &&
                   (ItemID == 322122547394 || ItemID == 21110945375846599)) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.shieldupgrade3 &&
                   (ItemID == 429496729795 || ItemID == 52776987629977800)) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.shieldupgrade4 && (ItemID == 429496729796)) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.shieldupgrade5 && ItemID == 536870912201) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.shieldupgradehead1 && ItemID == 188) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.shieldupgradehead2 && ItemID == 189) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.shieldupgradehead3 && ItemID == 190) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.shieldupgradehead4 && ItemID == 191) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.accelerant && ItemID == 182) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.phoenix && ItemID == 183) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.skull &&
                   strstr(
                       glowName,
                       "mdl/Weapons/skull_grenade/skull_grenade_base_v.rmdl")) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (item.isBox() && g_settings.deathbox) {
          std::array<unsigned char, 4> highlightMode = {
              0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
              125, // OutlineFunction OutlineFunction
                   // HIGHLIGHT_OUTLINE_LOOT_SCANNED
              64, 64};
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightMode, highlightParameter, 88);
        } else if (item.isTrap()) {
          std::array<unsigned char, 4> highlightMode = {
              0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
              125, // OutlineFunction OutlineFunction
                   // HIGHLIGHT_OUTLINE_LOOT_SCANNED
              64, 64};
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightMode, highlightParameter, 67);
        } else if (
            strstr(
                glowName,
                "mdl/props/caustic_gas_tank/caustic_gas_tank.rmdl")) { // Gas
                                                                       // Trap
          std::array<unsigned char, 4> highlightMode = {
              0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
              125, // OutlineFunction OutlineFunction
                   // HIGHLIGHT_OUTLINE_LOOT_SCANNED
              64, 64};
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightMode, highlightParameter, 67);
        } else if (g_settings.loot.healthlarge && ItemID == 184) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.healthsmall && ItemID == 185) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.shieldbattsmall && ItemID == 187) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.shieldbattlarge && ItemID == 186) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.sniperammo && ItemID == 144) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.heavyammo && ItemID == 143) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.optic1xhcog && ItemID == 215) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.lightammo && ItemID == 140) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.energyammo && ItemID == 141) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.shotgunammo && ItemID == 142) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.lasersight1 && ItemID == 229) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.lasersight2 && ItemID == 230) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.lasersight3 && ItemID == 231) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.sniperammomag1 && ItemID == 244) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.sniperammomag2 && ItemID == 245) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.sniperammomag3 && ItemID == 246) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.sniperammomag4 && ItemID == 247) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.energyammomag1 && ItemID == 240) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.energyammomag2 && ItemID == 241) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.energyammomag3 && ItemID == 242) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.energyammomag4 && ItemID == 243) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.stocksniper1 && ItemID == 255) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.stocksniper2 && ItemID == 256) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.stocksniper3 && ItemID == 257) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.stockregular1 && ItemID == 252) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.stockregular2 && ItemID == 253) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.stockregular3 && ItemID == 254) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.shielddown1 && ItemID == 203) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.shielddown2 && ItemID == 204) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.shielddown3 && ItemID == 205) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.shielddown4 && ItemID == 206) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.lightammomag1 && ItemID == 232) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.lightammomag2 && ItemID == 233) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.lightammomag3 && ItemID == 234) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.lightammomag4 && ItemID == 235) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.heavyammomag1 && ItemID == 236) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.heavyammomag2 && ItemID == 237) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.heavyammomag3 && ItemID == 238) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.heavyammomag4 && ItemID == 239) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.optic2xhcog && ItemID == 216) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.opticholo1x && ItemID == 217) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.opticholo1x2x && ItemID == 218) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.opticthreat && ItemID == 219) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.optic3xhcog && ItemID == 220) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.optic2x4x && ItemID == 221) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.opticsniper6x && ItemID == 222) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.opticsniper4x8x && ItemID == 223) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.opticsniperthreat && ItemID == 224) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.suppressor1 && ItemID == 225) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.suppressor2 && ItemID == 226) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.suppressor3 && ItemID == 227) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.turbo_charger && ItemID == 258) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.skull_piecer && ItemID == 260) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.hammer_point && ItemID == 263) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.disruptor_rounds && ItemID == 262) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.boosted_loader && ItemID == 272) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        } else if (g_settings.loot.shotgunbolt1 && ItemID == 248) {
          std::array<float, 3> highlightParameter = {1, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 72);
        } else if (g_settings.loot.shotgunbolt2 && ItemID == 249) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.shotgunbolt3 && ItemID == 250) {
          std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
          item.enableGlow(highlightFunctionBits, highlightParameter, 74);
        } else if (g_settings.loot.shotgunbolt4 && ItemID == 251) {
          std::array<float, 3> highlightParameter = {1, 0.8431, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 75);
        }
        // Nades
        else if (g_settings.loot.grenade_frag && ItemID == 213) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.grenade_thermite && ItemID == 212) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.grenade_arc_star && ItemID == 214) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 70);
        }
        // Weapons
        else if (g_settings.loot.weapon_kraber && ItemID == 1) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_mastiff && ItemID == 3) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_lstar && ItemID == 7) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_nemesis && ItemID == 135) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_havoc && ItemID == 13) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_devotion && ItemID == 18) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_triple_take && ItemID == 23) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_flatline && ItemID == 28) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.weapon_hemlock && ItemID == 33) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.weapon_g7_scout && ItemID == 39) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_alternator && ItemID == 44) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_r99 && ItemID == 49) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_prowler && ItemID == 56) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.weapon_volt && ItemID == 60) {
          std::array<float, 3> highlightParameter = {0.2, 1, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 73);
        } else if (g_settings.loot.weapon_longbow && ItemID == 65) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.weapon_charge_rifle && ItemID == 70) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.weapon_spitfire && ItemID == 75) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_r301 && ItemID == 80) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_eva8 && ItemID == 85) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_peacekeeper && ItemID == 90) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_mozambique && ItemID == 95) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_wingman && ItemID == 106) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.weapon_p2020 && ItemID == 111) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_re45 && ItemID == 116) {
          std::array<float, 3> highlightParameter = {1, 0.5490, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 66);
        } else if (g_settings.loot.weapon_sentinel && ItemID == 122) {
          std::array<float, 3> highlightParameter = {0, 0, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 69);
        } else if (g_settings.loot.weapon_bow && ItemID == 127) {
          std::array<float, 3> highlightParameter = {1, 0, 0};
          item.enableGlow(highlightFunctionBits, highlightParameter, 67);
        } else if (g_settings.loot.weapon_3030_repeater && ItemID == 129) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.weapon_rampage && ItemID == 146) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        } else if (g_settings.loot.weapon_car_smg && ItemID == 151) {
          std::array<float, 3> highlightParameter = {0, 1, 1};
          item.enableGlow(highlightFunctionBits, highlightParameter, 65);
        }

        // CREDITS to Rikkie
        // https://www.unknowncheats.me/forum/members/169606.html for all the
        // weapon ids and item ids code, you are a life saver!

      } // for(item) loop end
      treasure_clues = new_treasure_clues;
    } // while(item_glow) loop end
  }   // while(item_t) loop end
  item_t = false;
}

extern void start_overlay();

void terminal() {
  terminal_t = true;
  run_tui_menu();
}

int main(int argc, char *argv[]) {
  load_settings();
  aimbot = aimbot_new();

  if (geteuid() != 0) {
    // run as root..
    print_run_as_root();

    // test menu
    run_tui_menu();
    return 0;
  }

  const char *ap_proc = "r5apex.exe";

  std::thread aimbot_thr;
  std::thread esp_thr;
  std::thread actions_thr;
  std::thread cactions_thr;
  std::thread terminal_thr;
  std::thread overlay_thr;
  std::thread itemglow_thr;
  std::thread control_thr;

  if (apex_mem.open_os() != 0) {
    exit(0);
  }

  while (active) {
    if (apex_mem.get_proc_status() != process_status::FOUND_READY) {
      if (aim_t) {
        aim_t = false;
        esp_t = false;
        actions_t = false;
        cactions_t = false;
        terminal_t = false;
        overlay_t = false;
        item_t = false;
        control_t = false;
        g_Base = 0;
        tui_menu_quit();

        aimbot_thr.~thread();
        esp_thr.~thread();
        actions_thr.~thread();
        cactions_thr.~thread();
        terminal_thr.~thread();
        overlay_thr.~thread();
        itemglow_thr.~thread();
        control_thr.~thread();
      }

      std::this_thread::sleep_for(std::chrono::seconds(1));
      printf("Searching for apex process...\n");

      apex_mem.open_proc(ap_proc);

      if (apex_mem.get_proc_status() == process_status::FOUND_READY) {
        g_Base = apex_mem.get_proc_baseaddr();
        printf("\nApex process found\n");
        printf("Base: %lx\n", g_Base);

        aimbot_thr = std::thread(AimbotLoop);
        esp_thr = std::thread(EspLoop);
        actions_thr = std::thread(DoActions);
        cactions_thr = std::thread(ClientActions);
        itemglow_thr = std::thread(item_glow_t);
        control_thr = std::thread(ControlLoop);
        aimbot_thr.detach();
        esp_thr.detach();
        actions_thr.detach();
        cactions_thr.detach();
        itemglow_thr.detach();
        control_thr.detach();
      }
    } else {
      apex_mem.check_proc();

      const auto g_settings = global_settings();
      if (g_settings.debug_mode) {
        if (terminal_t) {
          tui_menu_quit();
        }
      } else {
        if (!terminal_t) {
          terminal_thr = std::thread(terminal);
          terminal_thr.detach();
        }
        // wish_list.clear();
      }
      if (g_settings.no_overlay) {
        if (overlay_t) {
          overlay_t = false;
        }
      } else {
        if (!overlay_t) {
          overlay_thr = std::thread(start_overlay);
          overlay_thr.detach();
        }
      }
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
  }

  return 0;
}
