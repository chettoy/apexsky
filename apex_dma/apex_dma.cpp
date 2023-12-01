#include "Client/main.h"
#include "Game.h"
#include "apex_sky.h"
#include "vector.h"
#include <array>
#include <cfloat>
#include <chrono>
#include <cstdint>
#include <cstdlib> // For the system() function
#include <iomanip>
#include <iostream>
#include <map>
#include <random>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <thread>
#include <unistd.h>
#include <unordered_map> // Include the unordered_map header
// this is a test, with seconds
Memory apex_mem;

// Just setting things up, dont edit.
bool active = true;
uintptr_t aimentity = 0;
uintptr_t tmp_aimentity = 0;
uintptr_t locked_aim_entity = 0;
float aiming_score_max;
bool aimbot_safety = true;
int team_player = 0;
const int toRead = 100;
bool aiming = false;
float max_fov = 10;
extern Vector aim_target; // for esp

// Removed but not all the way, dont edit.
int glowtype;
int glowtype2;
// float triggerdist = 50.0f;
bool actions_t = false;
bool cactions_t = false;
bool updateInsideValue_t = false;
bool TriggerBotRun_t = false;
bool terminal_t = false;
bool overlay_t = false;
bool esp_t = false;
bool aim_t = false;
bool vars_t = false;
bool item_t = false;
uint64_t g_Base;
bool next2 = false;
bool valid = false;
int lock = 0;
extern float bulletspeed;
extern float bulletgrav;
int local_held_id = 2147483647;
uint32_t local_weapon_id = 2147483647;
int playerentcount = 61;
int itementcount = 10000;
int map = 0;

//^^ Don't EDIT^^

// [del]CONFIG AREA, you must set all the true/false to what you want.[/del]
// No longer needed here. Edit your configuration file!
settings_t global_settings;

// trigger bot
bool is_trigger;

void TriggerBotRun() {
  // testing
  // apex_mem.Write<int>(g_Base + OFFSET_IN_ATTACK + 0x8, 4);
  // std::this_thread::sleep_for(std::chrono::milliseconds(10));
  apex_mem.Write<int>(g_Base + OFFSET_IN_ATTACK + 0x8, 5);
  std::this_thread::sleep_for(std::chrono::milliseconds(500));
  apex_mem.Write<int>(g_Base + OFFSET_IN_ATTACK + 0x8, 4);
  // printf("TriggerBotRun\n");
}
bool IsInCrossHair(Entity &target) {
  static uintptr_t last_t = 0;
  static float last_crosshair_target_time = -1.f;
  float now_crosshair_target_time = target.lastCrossHairTime();
  bool is_trigger = false;
  if (last_t == target.ptr) {
    if (last_crosshair_target_time != -1.f) {
      if (now_crosshair_target_time > last_crosshair_target_time) {
        is_trigger = true;
        // printf("Trigger\n");
        last_crosshair_target_time = -1.f;
      } else {
        is_trigger = false;
        last_crosshair_target_time = now_crosshair_target_time;
      }
    } else {
      is_trigger = false;
      last_crosshair_target_time = now_crosshair_target_time;
    }
  } else {
    last_t = target.ptr;
    last_crosshair_target_time = -1.f;
  }
  return is_trigger;
}

// Used to change things on a timer
/* unsigned char insidevalueItem = 1;
void updateInsideValue()
{
        updateInsideValue_t = true;
        while (updateInsideValue_t)
        {
                insidevalueItem++;
                insidevalueItem %= 256;
                std::this_thread::sleep_for(std::chrono::seconds(2));
                printf("smooth: %f\n", smooth);
                printf("bone: %i\n", bone);
                printf("glowrnot: %f\n", glowrnot);
                printf("glowgnot: %f\n", glowgnot);
                printf("glowbnot: %f\n", glowbnot);


        }
        updateInsideValue_t = false;
} */

// Visual check and aim check.?
float lastvis_esp[toRead];
float lastvis_aim[toRead];
int tmp_spec = 0, spectators = 0;
int tmp_all_spec = 0, allied_spectators = 0;
int glowtype3;
int settingIndex;
int contextId;
std::array<float, 3> highlightParameter;
// works
void SetPlayerGlow(Entity &LPlayer, Entity &Target, int index) {
  if (global_settings.player_glow >= 1) {
    if (!Target.isGlowing() ||
        (int)Target.buffer[OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE] != 1) {
      float currentEntityTime = 5000.f;
      if (!isnan(currentEntityTime) && currentEntityTime > 0.f) {
        if (!(global_settings.firing_range) &&
            (Target.isKnocked() || !Target.isAlive())) {
          contextId = 5;
          settingIndex = 80;
          highlightParameter = {global_settings.glow_r_knocked,
                                global_settings.glow_g_knocked,
                                global_settings.glow_b_knocked};
        } else if (Target.lastVisTime() > lastvis_aim[index] ||
                   (Target.lastVisTime() < 0.f && lastvis_aim[index] > 0.f)) {
          contextId = 6;
          settingIndex = 81;
          highlightParameter = {global_settings.glow_r_viz,
                                global_settings.glow_g_viz,
                                global_settings.glow_b_viz};
        } else {
          contextId = 7;
          settingIndex = 82;
          highlightParameter = {global_settings.glow_r_not,
                                global_settings.glow_g_not,
                                global_settings.glow_b_not};
        }
        Target.enableGlow();
      }
    }
  } else {
    if (!Target.isGlowing() ||
        (int)Target.buffer[OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE] != 1) {
      float currentEntityTime = 5000.f;
      if (!isnan(currentEntityTime) && currentEntityTime > 0.f) {
        if (!(global_settings.firing_range) &&
            (Target.isKnocked() || !Target.isAlive())) {
          global_settings.inside_value = 0; // 0 = no fill, 14 = full fill
          // Outline size
          global_settings.outline_size = 0; // 0-255
          contextId = 5;
          settingIndex = 80;
          highlightParameter = {0, 0, 0};
        } else if (Target.lastVisTime() > lastvis_aim[index] ||
                   (Target.lastVisTime() < 0.f && lastvis_aim[index] > 0.f)) {
          global_settings.inside_value = 0; // 0 = no fill, 14 = full fill
          // Outline size
          global_settings.outline_size = 0; // 0-255
          contextId = 6;
          settingIndex = 81;
          highlightParameter = {0, 0, 0};
        } else {
          global_settings.inside_value = 0; // 0 = no fill, 14 = full fill
          // Outline size
          global_settings.outline_size = 0; // 0-255
          contextId = 7;
          settingIndex = 82;
          highlightParameter = {0, 0, 0};
        }
        Target.enableGlow();
      }
    }
  }
}

void MapRadarTesting() {
  uintptr_t pLocal;
  apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, pLocal);
  int dt;
  apex_mem.Read<int>(pLocal + OFFSET_TEAM, dt);

  for (uintptr_t i = 0; i <= 80000; i++) {
    apex_mem.Write<int>(pLocal + OFFSET_TEAM, 1);
  }

  for (uintptr_t i = 0; i <= 80000; i++) {
    apex_mem.Write<int>(pLocal + OFFSET_TEAM, dt);
  }
}

uint64_t PlayerLocal;
int PlayerLocalTeamID;
int EntTeam;
int LocTeam;

using Clock = std::chrono::steady_clock;
std::chrono::time_point<std::chrono::steady_clock> start1, now1;
std::chrono::milliseconds duration1;

void loop() { start1 = Clock::now(); }

std::chrono::steady_clock::time_point tduckStartTime;
bool mapRadarTestingEnabled = true;

uint32_t button_state[4];
bool isPressed(uint32_t button_code) {
  return (button_state[static_cast<uint32_t>(button_code) >> 5] &
          (1 << (static_cast<uint32_t>(button_code) & 0x1f))) != 0;
}

void ClientActions() {
  cactions_t = true;
  while (cactions_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(1));

    // SuperGlide state
    int frameSleepTimer = 0;
    int lastFrameNumber = 0;
    bool superGlideStart = false;
    int superGlideTimer = 0;

    // Game fps state
    int last_checkpoint_frame = 0;
    std::chrono::milliseconds checkpoint_time;

    while (g_Base != 0) {

      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);

      int attackState = 0;
      apex_mem.Read<int>(g_Base + OFFSET_IN_ATTACK, attackState); // 108
      int tduckState = 0;
      apex_mem.Read<int>(g_Base + OFFSET_IN_TOGGLE_DUCK, tduckState); // 61

      apex_mem.Read<typeof(button_state)>(g_Base + OFFSET_INPUT_SYSTEM + 0xb0,
                                          button_state);

      int zoomState = 0;
      apex_mem.Read<int>(g_Base + OFFSET_IN_ZOOM, zoomState); // 109

      int curFrameNumber;
      apex_mem.Read<int>(g_Base + OFFSET_GLOBAL_VARS + 0x0008,
                         curFrameNumber); // GlobalVars + 0x0008

      float m_traversalProgressTmp = 0.0f;
      float m_traversalProgress;
      // printf("Playerentcount: %i\n", playerentcount);
      // printf("Playerentcount: %i\n", itementcount);
      apex_mem.Read<float>(LocalPlayer + OFFSET_TRAVERSAL_PROGRESS,
                           m_traversalProgress);
      // printf("Travel Time: %f\n", m_traversalProgress);
      // printf("Frame Sleep Timer: %i\n", frameSleepTimer);
      // printf("Super Glide Timer: %i\n", superGlideTimer);
      // printf("Last Frame: %i\n", lastFrameNumber);
      // printf("Cur Frame: %i\n", curFrameNumber);
      // printf("superGlideStart: %d\n", superGlideStart ? 1 : 0);
      int jump;
      int ducktoggle;
      int forceduck;
      apex_mem.Read<int>(g_Base + OFFSET_FORCE_JUMP + 0x8, jump);
      apex_mem.Read<int>(g_Base + OFFSET_IN_TOGGLE_DUCK + 0x8, ducktoggle);
      apex_mem.Read<int>(g_Base + OFFSET_FORCE_DUCK + 0x8, forceduck);
      // printf("Jump Value: %i\n", jump);
      // printf("Duck Value: %i\n", ducktoggle);
      // printf("Force Duck: %i\n", forceduck);
      // apex_mem.Write<int>(g_Base + OFFSET_FORCE_JUMP + 0x8, 4);

      if (curFrameNumber > lastFrameNumber) {
        frameSleepTimer = 10; // <- middle of the frame // needs 5 for 144fps
                              // and 10 for 75 fps
      }
      lastFrameNumber = curFrameNumber;

      if (frameSleepTimer == 0) {
        if (global_settings.super_key_toggle) {
          if (m_traversalProgress > 0.85 &&
              m_traversalProgress <
                  0.92) // needs to end at 0.90 for 144 fps and 0.92 for 75 fps
          {
            superGlideStart = true;
          }

          if (superGlideStart) {
            superGlideTimer++;
            // printf("Timer Started \n");
            if (superGlideTimer == 5) {
              apex_mem.Write<int>(g_Base + OFFSET_FORCE_JUMP + 0x8, 5);
            } else if (superGlideTimer == 6) {
              apex_mem.Write<int>(g_Base + OFFSET_IN_TOGGLE_DUCK + 0x8, 6);
            } else if (superGlideTimer ==
                       10) // needs to be 10 for 75 and 144fps?
            {
              apex_mem.Write<int>(g_Base + OFFSET_FORCE_JUMP + 0x8, 4);
              apex_mem.Write<int>(g_Base + OFFSET_FORCE_DUCK + 0x8, 5);
              apex_mem.Write<int>(g_Base + OFFSET_FORCE_DUCK + 0x8, 4);
              m_traversalProgressTmp = m_traversalProgress;
            } else if (superGlideTimer > 10 &&
                       m_traversalProgress != m_traversalProgressTmp) {
              superGlideStart = false;
              superGlideTimer = 0;
            }
          }
        }
      }
      frameSleepTimer -= 1;

      // calc game fps
      if (global_settings.calc_game_fps && curFrameNumber % 100 == 0) {
        std::chrono::milliseconds ms = duration_cast<std::chrono::milliseconds>(
            std::chrono::system_clock::now().time_since_epoch());
        int delta_frame = curFrameNumber - last_checkpoint_frame;
        if (delta_frame > 90 && delta_frame < 120) {
          auto duration = ms - checkpoint_time;
          global_settings.game_fps = delta_frame * 1000.0f / duration.count();
        }
        last_checkpoint_frame = curFrameNumber;
        checkpoint_time = ms;
      }

      // printf("Minimap: %ld\n", minimap);
      // apex_mem.Write(LocalPlayer + 0x270 , 1);

      /*
      108 Left mouse button (mouse1)
      109 Right mouse button (mouse2)
      110 Middle mouse button (mouse3)
      111 Side mouse button (mouse4)
      112 Side mouse button (mouse5)

      79 SHIFT key
      81 ALT key
      83 CTRL key

      1 KEY_0
      2 KEY_1
      3 KEY_2
      4 KEY_3
      5 KEY_4
      6 KEY_5
      7 KEY_6
      8 KEY_7
      9 KEY_8
      10 KEY_9

      11 KEY_A
      12 KEY_B
      13 KEY_C
      14 KEY_D
      15 KEY_E
      16 KEY_F
      17 KEY_G
      18 KEY_H
      19 KEY_I
      20 KEY_J
      21 KEY_K
      22 KEY_L
      23 KEY_M
      24 KEY_N
      25 KEY_O
      26 KEY_P
      27 KEY_Q
      28 KEY_R
      29 KEY_S
      30 KEY_T
      31 KEY_U
      32 KEY_V
      33 KEY_W
      34 KEY_X
      35 KEY_Y
      36 KEY_Z


      37 KEY_PAD_0
      38 KEY_PAD_1
      39 KEY_PAD_2
      40 KEY_PAD_3
      41 KEY_PAD_4
      42 KEY_PAD_5
      43 KEY_PAD_6
      44 KEY_PAD_7
      45 KEY_PAD_8
      46 KEY_PAD_9
      47 KEY_PAD_DIVIDE
      48 KEY_PAD_MULTIPLY
      49 KEY_PAD_MINUS
      50 KEY_PAD_PLUS
      51 KEY_PAD_ENTER
      52 KEY_PAD_DECIMAL


      65 KEY_SPACE
      67 KEY_TAB
      68 KEY_CAPSLOCK
      69 KEY_NUMLOCK
      70 KEY_ESCAPE
      71 KEY_SCROLLLOCK
      72 KEY_INSERT
      73 KEY_DELETE
      74 KEY_HOME
      75 KEY_END
      76 KEY_PAGEUP
      77 KEY_PAGEDOWN
      78 KEY_BREAK


      88 KEY_UP
      89 KEY_LEFT
      90 KEY_DOWN
      91 KEY_RIGHT


      92 KEY_F1
      93 KEY_F2
      94 KEY_F3
      95 KEY_F4
      96 KEY_F5
      97 KEY_F6
      98 KEY_F7
      99 KEY_F8
      100 KEY_F9
      101 KEY_F10
      102 KEY_F11
      103 KEY_F12
      */

      /* if (isPressed(79)) //TESTING KEYS
      {
              printf("Shift Pressed\n");
      }
      if (isPressed(81)) //TESTING KEYS
      {
              printf("ALT Pressed\n");
      }
      if (isPressed(83)) //TESTING KEYS
      {
              printf("CTRL Pressed0\n");
      } */

      if (global_settings.keyboard) {
        if (isPressed(global_settings.aimbot_hot_key_1) ||
            (isPressed(global_settings.aimbot_hot_key_2) &&
             !isPressed(
                 global_settings.tigger_bot_hot_key))) // Left and Right click
        {
          aiming = true;
        } else {
          aiming = false;
        }
        if (isPressed(global_settings.aimbot_hot_key_1) ||
            !isPressed(global_settings.aimbot_hot_key_2)) {
          max_fov = global_settings.non_ads_fov;
        }
        if (isPressed(global_settings.aimbot_hot_key_2)) {
          max_fov = global_settings.ads_fov;
        }
        if (isPressed(
                global_settings.tigger_bot_hot_key)) // Left and Right click
        {
          global_settings.tigger_bot = true;
        } else {
          global_settings.tigger_bot = false;
        }
      }

      if (global_settings.gamepad) {
        // attackState == 120 || zoomState == 119
        if (attackState > 0 || zoomState > 0) {
          aiming = true;
        } else {
          aiming = false;
        }

        if (zoomState > 0) {
          max_fov = global_settings.ads_fov;
        } else {
          max_fov = global_settings.non_ads_fov;
        }
      }

      now1 = Clock::now();
      duration1 =
          std::chrono::duration_cast<std::chrono::milliseconds>(now1 - start1);

      // Toggle crouch = check for ring
      if (global_settings.map_radar_testing && attackState == 0 &&
          tduckState == 13) {
        if (mapRadarTestingEnabled) {
          MapRadarTesting();
        }

        // if (tduckStartTime == std::chrono::steady_clock::time_point()) {
        //   tduckStartTime = std::chrono::steady_clock::now();
        // }

        // auto currentTime = std::chrono::steady_clock::now();
        // auto duration = std::chrono::duration_cast<std::chrono::seconds>(
        //                     currentTime - tduckStartTime)
        //                     .count();
        // if (duration >= 500) {
        mapRadarTestingEnabled = false;
        //}
        // } else {
        //   tduckStartTime = std::chrono::steady_clock::time_point();
        //   // mapRadarTestingEnabled = true;
      }

      std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
  }
  cactions_t = false;
}

void ProcessPlayer(Entity &LPlayer, Entity &target, uint64_t entitylist,
                   int index) {
  int entity_team = target.getTeamId();

  if (!target.isAlive()) {
    float localyaw = LPlayer.GetYaw();
    float targetyaw = target.GetYaw();

    if (localyaw == targetyaw) {
      if (LPlayer.getTeamId() == entity_team)
        tmp_all_spec++;
      else
        tmp_spec++;
    }
    return;
  }

  if (global_settings.tdm_toggle) { // Check if the target entity is on the same
                                    // team as the
                                    // local player
    // int entity_team = Target.getTeamId();
    // printf("Target Team: %i\n", entity_team);

    uint64_t PlayerLocal;
    apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, PlayerLocal);
    int PlayerLocalTeamID;
    apex_mem.Read<int>(PlayerLocal + OFFSET_TEAM, PlayerLocalTeamID);

    if (entity_team % 2)
      EntTeam = 1;
    else
      EntTeam = 2;
    if (PlayerLocalTeamID % 2)
      LocTeam = 1;
    else
      LocTeam = 2;

    // printf("Target Team: %i\nLocal Team: %i\n", EntTeam, LocTeam);
    if (EntTeam == LocTeam)
      return;
  }

  // Firing range stuff
  if (!global_settings.firing_range)
    if (entity_team < 0 || entity_team > 50 ||
        (entity_team == team_player && !global_settings.onevone))
      return;

  Vector EntityPosition = target.getPosition();
  Vector LocalPlayerPosition = LPlayer.getPosition();
  float dist = LocalPlayerPosition.DistTo(EntityPosition);

  // aim distance check
  const float skynade_dist = 100.0 * 40.0f;
  if ((local_held_id == -251 && dist > skynade_dist) ||
      dist > global_settings.aim_dist)
    return;

  // Targeting
  const float vis_weights = 12.5f;
  // float skynade_dist = aimdist / 2;
  float fov = CalculateFov(LPlayer, target);
  bool vis = target.lastVisTime() > lastvis_aim[index];
  float score =
      (fov * fov) * 100 + (dist * 0.025) * 10 + (vis ? 0 : vis_weights);
  /*
   fov:dist:score
    1  10m  100
    2  40m  400
    3  90m  900
    4  160m 1600
  */
  if (score < aiming_score_max) {
    aiming_score_max = score;
    tmp_aimentity = target.ptr;
  }

  if (global_settings.aim == 2) {
    // vis check
    if (aimentity == target.ptr) {
      if (local_held_id != -251 && !vis) {
        // turn on safety
        aimbot_safety = true;
      } else {
        aimbot_safety = false;
      }
    }

    // TriggerBot
    if (aimentity != 0) {
      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);

      Entity Target = getEntity(aimentity);
      // Entity LPlayer = getEntity(LocalPlayer);

      if (global_settings.tigger_bot && IsInCrossHair(Target)) {
        TriggerBotRun();
      }
    }
  }
  SetPlayerGlow(LPlayer, target, index);
  lastvis_aim[index] = target.lastVisTime();
}
std::map<uint64_t, int> centityToNumber; // Map centity to a unique number
int uniqueNumber = 1;                    // Initialize a unique number
// Main stuff, dont edit.
void DoActions() {
  actions_t = true;
  while (actions_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    uint32_t counter = 0;

    while (g_Base != 0) {
      char MapName[200] = {0};
      uint64_t MapName_ptr;
      apex_mem.Read<uint64_t>(g_Base + OFFSET_HOST_MAP, MapName_ptr);
      apex_mem.ReadArray<char>(MapName_ptr, MapName, 200);

      // printf("%s\n", MapName);
      if (strcmp(MapName, "mp_rr_tropic_island_mu1_storm") == 0) {
        map = 1;
      } else if (strcmp(MapName, "mp_rr_canyonlands_mu") == 0) {
        map = 2;
      } else if (strcmp(MapName, "mp_rr_desertlands_hu") == 0) {
        map = 3;
      } else if (strcmp(MapName, "mp_rr_olympus") == 0) {
        map = 4;
      } else if (strcmp(MapName, "mp_rr_divided_moon") == 0) {
        map = 5;
      } else {
        map = 0;
      }

      if (global_settings.firing_range) {
        playerentcount = 16000;
      } else {
        playerentcount = 61;
      }
      if (global_settings.deathbox) {
        itementcount = 15000;
      } else {
        itementcount = 10000;
      }
      std::this_thread::sleep_for(
          std::chrono::milliseconds(30)); // don't change xD

      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
      if (LocalPlayer == 0)
        continue;

      Entity LPlayer = getEntity(LocalPlayer);

      team_player = LPlayer.getTeamId();
      if (team_player < 0 || team_player > 50) {
        continue;
      }
      uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;

      uint64_t baseent = 0;
      apex_mem.Read<uint64_t>(entitylist, baseent);
      if (baseent == 0) {
        continue;
      }

      aiming_score_max =
          (50 * 50) * 100 + (global_settings.aim_dist * 0.025) * 10;
      tmp_aimentity = 0;
      tmp_spec = 0;
      tmp_all_spec = 0;
      if (global_settings.firing_range) {
        int c = 0;
        for (int i = 0; i < playerentcount; i++) {
          uint64_t centity = 0;
          apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
          if (centity == 0)
            continue;
          if (LocalPlayer == centity)
            continue;

          Entity Target = getEntity(centity);
          if (!Target.isDummy() && !global_settings.onevone) {
            continue;
          }

          ProcessPlayer(LPlayer, Target, entitylist, c);
          c++;
        }
      } else {

        for (int i = 0; i < toRead; i++) {
          uint64_t centity = 0;
          apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
          if (centity == 0)
            continue;
          if (LocalPlayer == centity)
            continue;

          Entity Target = getEntity(centity);
          if (!Target.isPlayer()) {
            continue;
          }

          ProcessPlayer(LPlayer, Target, entitylist, i);

          int entity_team = Target.getTeamId();
          if (entity_team == team_player && !global_settings.onevone) {
            continue;
          }
        }
      }

      if (!spectators && !allied_spectators) {
        spectators = tmp_spec;
        allied_spectators = tmp_all_spec;
      } else {
        // refresh spectators count every ~2 seconds
        counter++;
        if (counter == 70) {
          spectators = tmp_spec;
          allied_spectators = tmp_all_spec;
          counter = 0;
        }
      }

      if (lock) // locked target
        aimentity = locked_aim_entity;
      else // or new target
        aimentity = tmp_aimentity;
    }
  }
  actions_t = false;
}

// /////////////////////////////////////////////////////////////////////////////////////////////////////

player players[toRead];
Matrix view_matrix_data = {};

// ESP loop.. this helps right?
static void EspLoop() {
  esp_t = true;
  while (esp_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(5));
    while (g_Base != 0 && overlay_t) {
      std::this_thread::sleep_for(std::chrono::milliseconds(5));
      if (global_settings.esp) {
        valid = false;

        uint64_t LocalPlayer = 0;
        apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
        if (LocalPlayer == 0) {
          next2 = true;
          while (next2 && g_Base != 0 && overlay_t && global_settings.esp) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
          }
          continue;
        }
        Entity LPlayer = getEntity(LocalPlayer);
        int team_player = LPlayer.getTeamId();
        if (team_player < 0 || team_player > 50) {
          next2 = true;
          while (next2 && g_Base != 0 && overlay_t && global_settings.esp) {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
          }
          continue;
        }
        Vector LocalPlayerPosition = LPlayer.getPosition();

        uint64_t viewRenderer = 0;
        apex_mem.Read<uint64_t>(g_Base + OFFSET_RENDER, viewRenderer);
        uint64_t viewMatrix = 0;
        apex_mem.Read<uint64_t>(viewRenderer + OFFSET_MATRIX, viewMatrix);

        apex_mem.Read<Matrix>(viewMatrix, view_matrix_data);

        uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;

        memset(players, 0, sizeof(players));

        if (global_settings.firing_range) {
          int c = 0;
          // Ammount of ents to loop, dont edit.
          for (int i = 0; i < 61; i++) {
            uint64_t centity = 0;
            apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
            if (centity == 0) {
              continue;
            }

            if (LocalPlayer == centity) {
              continue;
            }

            Entity Target = getEntity(centity);

            if (!Target.isDummy() && !global_settings.onevone) {
              continue;
            }

            if (!Target.isAlive()) {
              continue;
            }
            int entity_team = Target.getTeamId();
            if (!global_settings.onevone) {
              if (entity_team < 0 || entity_team > 50 ||
                  entity_team == team_player) {
                continue;
              }
            } else {
              if (entity_team < 0 || entity_team > 50) {
                continue;
              }
            }
            Vector EntityPosition = Target.getPosition();
            float dist = LocalPlayerPosition.DistTo(EntityPosition);

            if (dist > global_settings.max_dist || dist < 50.0f) {
              continue;
            }

            Vector bs = Vector();
            // Change res to your res here, default is 1080p but can copy paste
            // 1440p here
            WorldToScreen(EntityPosition, view_matrix_data.matrix, 1920, 1080,
                          bs); // 2560, 1440
            if (global_settings.esp) {
              Vector hs = Vector();
              Vector HeadPosition = Target.getBonePositionByHitbox(0);
              // Change res to your res here, default is 1080p but can copy
              // paste 1440p here
              WorldToScreen(HeadPosition, view_matrix_data.matrix, 1920, 1080,
                            hs); // 2560, 1440
              float height = abs(abs(hs.y) - abs(bs.y));
              float width = height / 2.0f;
              float boxMiddle = bs.x - (width / 2.0f);
              int health = Target.getHealth();
              int shield = Target.getShield();
              int maxshield = Target.getMaxshield();
              int armortype = Target.getArmortype();
              players[c] = {dist,
                            entity_team,
                            boxMiddle,
                            hs.y,
                            width,
                            height,
                            bs.x,
                            bs.y,
                            0,
                            (Target.lastVisTime() > lastvis_esp[c]),
                            health,
                            shield,
                            maxshield,
                            armortype

              };
              Target.get_name(g_Base, i - 1, &players[c].name[0]);
              lastvis_esp[c] = Target.lastVisTime();
              valid = true;
              c++;
            }
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

            if (!Target.isAlive()) {
              continue;
            }

            int entity_team = Target.getTeamId();
            if (!global_settings.onevone) {
              if (entity_team < 0 || entity_team > 50 ||
                  entity_team == team_player) {
                continue;
              }
            } else {
              if (entity_team < 0 || entity_team > 50) {
                continue;
              }
            }

            Vector EntityPosition = Target.getPosition();
            float dist = LocalPlayerPosition.DistTo(EntityPosition);
            if (dist > global_settings.max_dist || dist < 50.0f) {
              continue;
            }

            Vector bs = Vector();
            // Change res to your res here, default is 1080p but can copy paste
            // 1440p here
            WorldToScreen(EntityPosition, view_matrix_data.matrix, 1920, 1080,
                          bs); // 2560, 1440
            if (global_settings.esp) {
              Vector hs = Vector();
              Vector HeadPosition = Target.getBonePositionByHitbox(0);
              // Change res to your res here, default is 1080p but can copy
              // paste 1440p here
              WorldToScreen(HeadPosition, view_matrix_data.matrix, 1920, 1080,
                            hs); // 2560, 1440
              float height = abs(abs(hs.y) - abs(bs.y));
              float width = height / 2.0f;
              float boxMiddle = bs.x - (width / 2.0f);
              int health = Target.getHealth();
              int shield = Target.getShield();
              int maxshield = Target.getMaxshield();
              int armortype = Target.getArmortype();
              Vector EntityPosition = Target.getPosition();
              Vector LocalPlayerPosition = LPlayer.getPosition();
              QAngle localviewangle = LPlayer.GetViewAngles();
              float targetyaw = Target.GetYaw();
              players[i] = {dist,
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
                            armortype,
                            EntityPosition,
                            LocalPlayerPosition,
                            localviewangle,
                            targetyaw};
              Target.get_name(g_Base, i - 1, &players[i].name[0]);
              lastvis_esp[i] = Target.lastVisTime();
              valid = true;
            }
          }
        }

        next2 = true;
        while (next2 && g_Base != 0 && overlay_t && global_settings.esp) {
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
      std::this_thread::sleep_for(std::chrono::milliseconds(1));

      // Read LocalPlayer
      uint64_t LocalPlayer = 0;
      apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);
      // Read HeldID
      int HeldID;
      apex_mem.Read<int>(LocalPlayer + OFFSET_OFF_WEAPON, HeldID); // 0x1a1c
      local_held_id = HeldID;
      // Read WeaponID
      ulong ehWeaponHandle;
      apex_mem.Read<uint64_t>(LocalPlayer + OFFSET_ACTIVE_WEAPON,
                              ehWeaponHandle); // 0x1a1c
      ehWeaponHandle &= 0xFFFF;                // eHandle
      ulong pWeapon;
      uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;
      apex_mem.Read<uint64_t>(entitylist + (ehWeaponHandle * 0x20), pWeapon);
      uint32_t weaponID;
      apex_mem.Read<uint32_t>(pWeapon + OFFSET_WEAPON_NAME,
                              weaponID); // 0x1844
      local_weapon_id = weaponID;
      // printf("%d\n", weaponID);

      if (global_settings.aim > 0) {
        if (aimentity == 0 || !aiming) {
          lock = false;
          locked_aim_entity = 0;
          continue;
        }
        if (aimbot_safety) {
          continue;
        }
        lock = true;
        locked_aim_entity = aimentity;

        Entity LPlayer = getEntity(LocalPlayer);
        if (LocalPlayer == 0)
          continue;

        /* Fine-tuning for each weapon */
        // bow
        if (weaponID == 2) {
          // Ctx.BulletSpeed = BulletSpeed - (BulletSpeed*0.08);
          // Ctx.BulletGravity = BulletGrav + (BulletGrav*0.05);
          bulletspeed = 10.08;
          bulletgrav = 10.05;
        }

        if (HeldID == -251) { // auto throw
          if (!global_settings.no_nade_aim) {
            QAngle Angles_g = CalculateBestBoneAim(LPlayer, aimentity, 999.9f);
            if (Angles_g.x == 0 && Angles_g.y == 0) {
              lock = false;
              locked_aim_entity = 0;
              continue;
            }
            LPlayer.SetViewAngles(Angles_g);
          }
        } else {
          QAngle Angles = CalculateBestBoneAim(LPlayer, aimentity, max_fov);
          if (Angles.x == 0 && Angles.y == 0) {
            lock = false;
            locked_aim_entity = 0;
            continue;
          }
          LPlayer.SetViewAngles(Angles);
        }
      }
    }
  }
  aim_t = false;
}
// Item Glow Stuff

static void item_glow_t() {
  item_t = true;
  while (item_t) {
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    while (g_Base != 0) {
      std::this_thread::sleep_for(std::chrono::milliseconds(1));
      uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;
      if (global_settings.item_glow) {
        // item ENTs to loop, 10k-15k is normal. 10k might be better but will
        // not show all the death boxes i think.
        for (int i = 0; i < itementcount; i++) {
          uint64_t centity = 0;
          apex_mem.Read<uint64_t>(entitylist + ((uint64_t)i << 5), centity);
          if (centity == 0)
            continue;
          Item item = getItem(centity);
          // testing
          uint64_t LocalPlayer = 0;
          apex_mem.Read<uint64_t>(g_Base + OFFSET_LOCAL_ENT, LocalPlayer);

          // Item filter glow name setup and search.
          char glowName[200] = {0};
          uint64_t name_ptr;
          apex_mem.Read<uint64_t>(centity + OFFSET_MODELNAME, name_ptr);
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
          // apex_mem.Read<uint64_t>(g_Base + OFFSET_LEVELNAME, levelname_ptr);
          // apex_mem.ReadArray<char>(levelname_ptr, LevelNAME, 200);

          // printf("%s\n", LevelNAME);

          // Prints stuff you want to console
          // if (strstr(glowName, "mdl/"))
          //{
          // printf("%ld\n", ItemID);
          // }
          // Search model name and if true sets glow, must be a better way to do
          // this.. if only i got the item id to work..
          if (global_settings.loot_lightbackpack && ItemID == 207) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          /* else
          {
                  std::array<unsigned char, 4> highlightFunctionBits = {
                          0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                          0,   // OutlineFunction HIGHLIGHT_OUTLINE_LOOT_SCANNED
                          64,
                          64
                  };
                  std::array<float, 3> highlightParameter = { 0, 0, 0 };
                  apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS,
          0); static const int contextId = 0; int settingIndex = 99;
                  apex_mem.Write<unsigned char>(centity +
          OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId, settingIndex); long
          highlightSettingsPtr; apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
          highlightSettingsPtr);
                  apex_mem.Write<typeof(highlightFunctionBits)>(highlightSettingsPtr
          + 40 * settingIndex + 4, highlightFunctionBits);
                  apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr
          + 40 * settingIndex + 8, highlightParameter);
          } */
          if (global_settings.loot_medbackpack && ItemID == 208) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavybackpack && ItemID == 209) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_goldbackpack && ItemID == 210) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          // item id would help so much here, cant make them all the same color
          // so went with loba glow for body shield and helmet
          if (global_settings.loot_shieldupgrade1 &&
              (ItemID == 214748364993 || ItemID == 14073963583897798)) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);

            uint64_t ItemID;
            apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
            // uint64_t ItemID2;
            // ItemID2 = ItemID % 301;
            // printf("%ld\n", ItemID);
            // apex_mem.Write<typeof(highlightParameter)>(highlightSettingsPtr +
            // 40 * settingIndex + 8, highlightParameter);;
          }
          if (global_settings.loot_shieldupgrade2 &&
              (ItemID == 322122547394 || ItemID == 21110945375846599)) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgrade3 &&
              (ItemID == 429496729795 || ItemID == 52776987629977800)) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgrade4 && (ItemID == 429496729796)) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgrade5 && ItemID == 536870912201) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgradehead1 && ItemID == 188) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgradehead2 && ItemID == 189) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgradehead3 && ItemID == 190) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldupgradehead4 && ItemID == 191) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_accelerant && ItemID == 182) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_phoenix && ItemID == 183) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_skull &&
              strstr(glowName,
                     "mdl/Weapons/skull_grenade/skull_grenade_base_v.rmdl")) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (item.isBox()) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125, // OutlineFunction OutlineFunction
                     // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 88;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }

          if (item.isTrap()) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125, // OutlineFunction OutlineFunction
                     // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }

          // Gas Trap
          if (strstr(glowName,
                     "mdl/props/caustic_gas_tank/caustic_gas_tank.rmdl")) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                0,   // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125, // OutlineFunction OutlineFunction
                     // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_healthlarge && ItemID == 184) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_healthsmall && ItemID == 185) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldbattsmall && ItemID == 187) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shieldbattlarge && ItemID == 186) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_sniperammo && ItemID == 144) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavyammo && ItemID == 143) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_optic1xhcog && ItemID == 215) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lightammo && ItemID == 140) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_energyammo && ItemID == 141) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shotgunammo && ItemID == 142) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lasersight1 && ItemID == 229) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lasersight2 && ItemID == 230) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lasersight3 && ItemID == 231) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_sniperammomag1 && ItemID == 244) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_sniperammomag2 && ItemID == 245) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_sniperammomag3 && ItemID == 246) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_sniperammomag4 && ItemID == 247) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_energyammomag1 && ItemID == 240) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_energyammomag2 && ItemID == 241) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_energyammomag3 && ItemID == 242) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_energyammomag4 && ItemID == 243) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stocksniper1 && ItemID == 255) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stocksniper2 && ItemID == 256) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stocksniper3 && ItemID == 257) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stockregular1 && ItemID == 252) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stockregular2 && ItemID == 253) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_stockregular3 && ItemID == 254) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shielddown1 && ItemID == 203) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shielddown2 && ItemID == 204) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shielddown3 && ItemID == 205) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shielddown4 && ItemID == 206) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lightammomag1 && ItemID == 232) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lightammomag2 && ItemID == 233) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lightammomag3 && ItemID == 234) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_lightammomag4 && ItemID == 235) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavyammomag1 && ItemID == 236) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavyammomag2 && ItemID == 237) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavyammomag3 && ItemID == 238) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_heavyammomag4 && ItemID == 239) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_optic2xhcog && ItemID == 216) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticholo1x && ItemID == 217) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticholo1x2x && ItemID == 218) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticthreat && ItemID == 219) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_optic3xhcog && ItemID == 220) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_optic2x4x && ItemID == 221) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticsniper6x && ItemID == 222) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticsniper4x8x && ItemID == 223) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_opticsniperthreat && ItemID == 224) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_suppressor1 && ItemID == 225) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_suppressor2 && ItemID == 226) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_suppressor3 && ItemID == 227) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_turbo_charger && ItemID == 258) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_skull_piecer && ItemID == 260) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_hammer_point && ItemID == 263) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_disruptor_rounds && ItemID == 262) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_boosted_loader && ItemID == 272) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shotgunbolt1 && ItemID == 248) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 72;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shotgunbolt2 && ItemID == 249) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shotgunbolt3 && ItemID == 250) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2941, 0, 0.5098};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 74;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_shotgunbolt4 && ItemID == 251) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.8431, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 75;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          // Nades
          if (global_settings.loot_grenade_frag && ItemID == 213) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }

          if (global_settings.loot_grenade_thermite && ItemID == 212) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_grenade_arc_star && ItemID == 214) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 70;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          // Weapons
          if (global_settings.loot_weapon_kraber && ItemID == 1) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_mastiff && ItemID == 3) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings.loot_filled, // InsideFunction
                125, // OutlineFunction: HIGHLIGHT_OUTLINE_OBJECTIVE
                64,  // OutlineRadius: size * 255 / 8
                64 // (EntityVisible << 6) | State & 0x3F | (AfterPostProcess <<
                   // 7)
            };
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<int>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 2;
            int settingIndex = 67;
            apex_mem.Write<int>(centity + OFFSET_GLOW_ENABLE, contextId);
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            // apex_mem.Write<int>(ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES +
            // contextId, settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 0x28 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 0x28 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_lstar && ItemID == 7) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          // new gun, nemesis
          if (global_settings.loot_weapon_nemesis && ItemID == 135) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }

          if (global_settings.loot_weapon_havoc && ItemID == 13) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_devotion && ItemID == 18) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_triple_take && ItemID == 23) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_flatline && ItemID == 28) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_hemlock && ItemID == 33) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_g7_scout && ItemID == 39) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_alternator && ItemID == 44) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_r99 && ItemID == 49) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_prowler && ItemID == 56) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_volt && ItemID == 60) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0.2, 1, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 73;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_longbow && ItemID == 65) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_charge_rifle && ItemID == 70) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
            uint64_t ItemID;
            apex_mem.Read<uint64_t>(centity + OFFSET_ITEM_ID, ItemID);
            // printf("%ld\n", ItemID);
          }
          if (global_settings.loot_weapon_spitfire && ItemID == 75) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_r301 && ItemID == 80) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_eva8 && ItemID == 85) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_peacekeeper && ItemID == 90) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_mozambique && ItemID == 95) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_wingman && ItemID == 106) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_p2020 && ItemID == 111) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_re45 && ItemID == 116) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0.5490, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 66;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_sentinel && ItemID == 122) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 0, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 69;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_bow && ItemID == 127) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {1, 0, 0};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 67;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_3030_repeater && ItemID == 129) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_rampage && ItemID == 146) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }
          if (global_settings.loot_weapon_car_smg && ItemID == 151) {
            std::array<unsigned char, 4> highlightFunctionBits = {
                global_settings
                    .loot_filled, // InsideFunction  HIGHLIGHT_FILL_LOOT_SCANNED
                125,              // OutlineFunction OutlineFunction
                                  // HIGHLIGHT_OUTLINE_LOOT_SCANNED
                64, 64};
            std::array<float, 3> highlightParameter = {0, 1, 1};
            apex_mem.Write<uint32_t>(centity + OFFSET_GLOW_THROUGH_WALLS, 2);
            static const int contextId = 0;
            int settingIndex = 65;
            apex_mem.Write<unsigned char>(
                centity + OFFSET_HIGHLIGHTSERVERACTIVESTATES + contextId,
                settingIndex);
            long highlightSettingsPtr;
            apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS,
                                highlightSettingsPtr);
            apex_mem.Write<typeof(highlightFunctionBits)>(
                highlightSettingsPtr + 40 * settingIndex + 4,
                highlightFunctionBits);
            apex_mem.Write<typeof(highlightParameter)>(
                highlightSettingsPtr + 40 * settingIndex + 8,
                highlightParameter);
          }

          // CREDITS to Rikkie
          // https://www.unknowncheats.me/forum/members/169606.html for all the
          // weapon ids and item ids code, you are a life saver!
        }
        // Change the 60 ms to lower to make the death boxes filker less.
        // std::this_thread::sleep_for(std::chrono::milliseconds(60));
      }
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(30));
  }
  item_t = false;
}

// SSH terminal
//  Function to save settings to a file
void ItemFilterMenu() {
  std::cout << "1 - Light weapons" << std::endl;
  std::cout << "2 - Heavy Weapons" << std::endl;
  std::cout << "3 - Energy Weapons" << std::endl;
  std::cout << "4 - Sniper Weapons" << std::endl;
  std::cout << "5 - Armors" << std::endl;
  std::cout << "6 - Healing" << std::endl;
  std::cout << "7 - Nades" << std::endl;
  std::cout << "8 - Backpacks" << std::endl;
  std::cout << "9 - Scopes" << std::endl;
  std::cout << "10 - Back to Main Menu" << std::endl;
}

const char *boneDescriptions[] = {"Head", "Neck", "Chest", "Gut Shot"};
void updateGlowColor(float &glowr, float &glowg, float &glowb,
                     const std::string &setName) {
  std::cout << "Enter RGB values for " << setName
            << " (0-1 for each channel):\n";
  std::cout << "Red: ";
  std::cin >> glowr;

  std::cout << "Green: ";
  std::cin >> glowg;

  std::cout << "Blue: ";
  std::cin >> glowb;

  // Validate and clamp values to the range [0, 1].
  glowr = std::max(0.0f, std::min(1.0f, glowr));
  glowg = std::max(0.0f, std::min(1.0f, glowg));
  glowb = std::max(0.0f, std::min(1.0f, glowb));

  std::cout << setName << " RGB values updated (R: " << glowr
            << ", G: " << glowg << ", B: " << glowb << ").\n";
}
void displayMainMenu() {

  system("clear"); // Use "cls" for Windows
  std::string userInput;
  std::cout << "Main Menu:" << std::endl;

  if (global_settings.firing_range) {
    std::cout << "1 - Firing Range Enabled" << std::endl;
  } else {
    std::cout << "1 - Firing Range Disabled" << std::endl;
  }
  if (global_settings.tdm_toggle) {
    std::cout << "2 - TDMToggle Enabled" << std::endl;
  } else {
    std::cout << "2 - TDMToggle Disabled" << std::endl;
  }
  if (global_settings.keyboard) {
    std::cout << "3 - Keyboard Enabled" << std::endl;
  } else {
    std::cout << "3 - Keyboard Disabled" << std::endl;
  }
  if (global_settings.gamepad) {
    std::cout << "4 - Gamepad Enabled" << std::endl;
  } else {
    std::cout << "4 - Gamepad Disabled" << std::endl;
  }
  if (global_settings.item_glow) {
    std::cout << "5 - Item Glow Enabled" << std::endl;
  } else {
    std::cout << "5 - Item Glow Disabled" << std::endl;
  }
  if (global_settings.player_glow) {
    std::cout << "6 - Player Glow Enabled" << std::endl;
  } else {
    std::cout << "6 - Player Glow Disabled" << std::endl;
  }

  std::cout << "7 - Change Smooth Value: (Current: ";
  if (global_settings.smooth < 100.0f) {
    std::cout << "\033[1;31m"; // Set text color to red for values below 100
  } else if (global_settings.smooth > 120.0f) {
    std::cout << "\033[1;32m"; // Set text color to green for values above 120
  }
  std::cout << global_settings.smooth
            << "\033[0m"; // Reset text color to default and close color tag
  std::cout << ")" << std::endl;

  std::cout << "8 - Change Bone Aim Value: (Current: ";
  if (global_settings.bone == 0) {
    std::cout << "Head";
  } else if (global_settings.bone == 1) {
    std::cout << "Neck";
  } else if (global_settings.bone == 2) {
    std::cout << "Chest";
  } else if (global_settings.bone == 3) {
    std::cout << "Gut Shot";
  } else {
    std::cout << "Unknown";
  }
  std::cout << ")" << std::endl;

  if (global_settings.loot_filled_toggle) {
    global_settings.loot_filled = 14;
    std::cout << "9 - Loot Glow Filled" << std::endl;
  } else {
    global_settings.loot_filled = 0;
    std::cout << "9 - Loot Glow Not Filled" << std::endl;
  }
  if (global_settings.player_filled_toggle) {
    global_settings.inside_value = 14;
    std::cout << "10 - Player Glow Filled" << std::endl;
  } else {
    global_settings.inside_value = 0;
    std::cout << "10 - Player Glow Not Filled" << std::endl;
  }
  std::cout << "11 - Player Outline Glow Setting Size" << std::endl;
  std::cout << "12 - Update Glow Colors" << std::endl;
  std::cout << "13 - Change ADS FOV: (Current: " << global_settings.ads_fov
            << ")" << std::endl;
  std::cout << "14 - Change Non-ADS FOV: (Current: "
            << global_settings.non_ads_fov << ")" << std::endl;

  if (!global_settings.super_key_toggle) {
    std::cout << "15 - Super Glide Disabled" << std::endl;
  } else {
    std::cout << "15 - Super Glide Enabled" << std::endl;
  }
  std::cout << "16 - Item Filter Settings\n" << std::endl;
  std::cout << "17 - Aiming Key One Setting" << std::endl;
  std::cout << "18 - Aiming Key Two Setting" << std::endl;
  std::cout << "19 - Triggerbot Key Setting\n" << std::endl;

  if (global_settings.deathbox) {
    std::cout << "20 - Death Boxes ON\n" << std::endl;
  } else {
    std::cout << "20 - Death Boxes OFF\n" << std::endl;
  }

  std::cout << "21 - Save Settings" << std::endl;
  std::cout << "22 - Load Settings\n" << std::endl;

  std::cout << "23 - Toggle NoNadeAim (Current: "
            << (global_settings.no_nade_aim ? "No Nade Aim"
                                            : "Throwing aimbot on")
            << ")" << std::endl;

  std::cout << "24 - Toggle 1v1 (Current: "
            << (global_settings.onevone ? "on" : "off") << ")" << std::endl;

  std::cout << "25 - Toggle No Recoil (Current: "
            << (global_settings.aim_no_recoil ? "on" : "off") << ")"
            << std::endl;

  std::cout << "26 - Set Game FPS for Aim Prediction: (Current: ";
  if (global_settings.calc_game_fps)
    std::cout << "calc game fps";
  else
    std::cout << std::setiosflags(std::ios::fixed) << std::setprecision(1)
              << global_settings.game_fps;
  std::cout << ")" << std::endl;

  std::cout << std::endl;
}

void displayItemFilterMenu() {
  std::cout << "Item Filter Menu:" << std::endl;
  std::cout << "1 - Light weapons" << std::endl;
  std::cout << "2 - Heavy Weapons" << std::endl;
  std::cout << "3 - Energy Weapons" << std::endl;
  std::cout << "4 - Sniper Weapons" << std::endl;
  std::cout << "5 - Armors" << std::endl;
  std::cout << "6 - Healing" << std::endl;
  std::cout << "7 - Nades" << std::endl;
  std::cout << "8 - Backpacks" << std::endl;
  std::cout << "9 - Scopes" << std::endl;
  std::cout << "10 - Back to Main Menu" << std::endl;
}

void displayLightWeapons() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << yellowColor << "Light Weapons:\n" << resetColor << std::endl;
  if (global_settings.loot_weapon_p2020) {
    std::cout << "1 - " << greenColor << "P2022" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "P2022" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_re45) {
    std::cout << "2 - " << greenColor << "RE-45" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "RE-45" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_alternator) {
    std::cout << "3 - " << greenColor << "Alternator" << resetColor
              << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Alternator" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_r99) {
    std::cout << "4 - " << greenColor << "R-99" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "R-99" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_r301) {
    std::cout << "5 - " << greenColor << "R-301" << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "R-301" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_spitfire) {
    std::cout << "6 - " << greenColor << "M600" << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "M600" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_g7_scout) {
    std::cout << "7 - " << greenColor << "G7 Scout" << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "G7 Scout" << resetColor << std::endl;
  }

  if (global_settings.loot_lightammo) {
    std::cout << "8 - " << greenColor << "Light Ammo\n"
              << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "Light Ammo\n"
              << resetColor << std::endl;
  }
  std::cout << "Light Weapon Mags:\n" << std::endl;

  // Display colored options

  if (global_settings.loot_lightammomag1) {
    std::cout << "9 - " << greenColor << "Light Weapon Mag" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "Light Weapon Mag" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_lightammomag2) {
    std::cout << "10 - " << greenColor << "Light Weapon Mag" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Light Weapon Mag" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_lightammomag3) {
    std::cout << "11 - " << greenColor << "Light Weapon Mag" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "11 - " << redColor << "Light Weapon Mag" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_lightammomag4) {
    std::cout << "12 - " << greenColor << "Light Weapon Mag" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "12 - " << redColor << "Light Weapon Mag" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Stocks:\n" << std::endl;

  if (global_settings.loot_stockregular1) {
    std::cout << "13 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "13 - " << redColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular2) {
    std::cout << "14 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "14 - " << redColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular3) {
    std::cout << "15 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "15 - " << redColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Suppressors:\n" << std::endl;

  if (global_settings.loot_suppressor1) {
    std::cout << "16 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "16 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor2) {
    std::cout << "17 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "17 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor3) {
    std::cout << "18 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "18 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Lasers:\n" << std::endl;

  if (global_settings.loot_lasersight1) {
    std::cout << "19 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "19 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight2) {
    std::cout << "20 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "20 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight3) {
    std::cout << "21 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "21 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight4) {
    std::cout << "22 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "22 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Hop-Ups:\n" << std::endl;

  if (global_settings.loot_turbo_charger) {
    std::cout << "23 - " << greenColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "23 - " << redColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_skull_piecer) {
    std::cout << "24 - " << greenColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "24 - " << redColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_hammer_point) {
    std::cout << "25 - " << greenColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "25 - " << redColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_disruptor_rounds) {
    std::cout << "26 - " << greenColor << "Disruptor Rounds" << resetColor
              << ": " << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "26 - " << redColor << "Disruptor Rounds" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_boosted_loader) {
    std::cout << "27 - " << greenColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "27 - " << redColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "28 - Back to Settings Menu\n" << std::endl;
}

void displayHeavyWeapons() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << tealColor << "Heavy Weapons:\n" << resetColor << std::endl;
  if (global_settings.loot_weapon_flatline) {
    std::cout << "1 - " << greenColor << "Flatline" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "Flatline" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_hemlock) {
    std::cout << "2 - " << greenColor << "Hemlock" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Hemlock" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_3030_repeater) {
    std::cout << "3 - " << greenColor << "30-30 Repeater" << resetColor
              << std::endl;
  } else {
    std::cout << "3 - " << redColor << "30-30 Repeater" << resetColor
              << std::endl;
  }

  if (global_settings.loot_weapon_rampage) {
    std::cout << "4 - " << greenColor << "Rampage" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Rampage" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_prowler) {
    std::cout << "5 - " << greenColor << "Prowler" << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Prowler" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_car_smg) {
    std::cout << "6 - " << greenColor << "Car SMG" << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "Car SMG" << resetColor << std::endl;
  }

  if (global_settings.loot_heavyammo) {
    std::cout << "7 - " << greenColor << "Heavy Ammo\n"
              << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "Heavy Ammo\n"
              << resetColor << std::endl;
  }
  std::cout << "Heavy Weapon Mags:\n" << std::endl;

  // Display colored options

  if (global_settings.loot_heavyammomag1) {
    std::cout << "8 - " << greenColor << "Heavy Weapon Mag" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "Heavy Weapon Mag" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_heavyammomag2) {
    std::cout << "9 - " << greenColor << "Heavy Weapon Mag" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "Heavy Weapon Mag" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_heavyammomag3) {
    std::cout << "10 - " << greenColor << "Heavy Weapon Mag" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Heavy Weapon Mag" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_heavyammomag4) {
    std::cout << "11 - " << greenColor << "Heavy Weapon Mag" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "11 - " << redColor << "Heavy Weapon Mag" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Stocks:\n" << std::endl;

  if (global_settings.loot_stockregular1) {
    std::cout << "12 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "12 - " << redColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular2) {
    std::cout << "13 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "13 - " << redColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular3) {
    std::cout << "14 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "14 - " << redColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Suppressors:\n" << std::endl;

  if (global_settings.loot_suppressor1) {
    std::cout << "15 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "15 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor2) {
    std::cout << "16 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "16 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor3) {
    std::cout << "17 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "17 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Lasers:\n" << std::endl;

  if (global_settings.loot_lasersight1) {
    std::cout << "18 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "18 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight2) {
    std::cout << "19 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "19 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight3) {
    std::cout << "20 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "20 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight4) {
    std::cout << "21 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "21 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Hop-Ups:\n" << std::endl;

  if (global_settings.loot_turbo_charger) {
    std::cout << "22 - " << greenColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "22 - " << redColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_skull_piecer) {
    std::cout << "23 - " << greenColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "23 - " << redColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_hammer_point) {
    std::cout << "24 - " << greenColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "24 - " << redColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_disruptor_rounds) {
    std::cout << "25 - " << greenColor << "Disruptor Rounds" << resetColor
              << ": " << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "25 - " << redColor << "Disruptor Rounds" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_boosted_loader) {
    std::cout << "26 - " << greenColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "26 - " << redColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "27 - Back to Settings Menu\n" << std::endl;
}

void displayEnergyWeapons() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << yellowGoldColor << "Energy Weapons:\n"
            << resetColor << std::endl;
  if (global_settings.loot_weapon_lstar) {
    std::cout << "1 - " << greenColor << "LSTAR" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "LSTAR" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_nemesis) {
    std::cout << "2 - " << greenColor << "Nemesis" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Nemesis" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_havoc) {
    std::cout << "3 - " << greenColor << "Havoc" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Havoc" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_devotion) {
    std::cout << "4 - " << greenColor << "Deovtion" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Deovtion" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_triple_take) {
    std::cout << "5 - " << greenColor << "Tripple Take" << resetColor
              << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Tripple Take" << resetColor
              << std::endl;
  }

  if (global_settings.loot_weapon_volt) {
    std::cout << "6 - " << greenColor << "Volt" << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "Volt" << resetColor << std::endl;
  }

  if (global_settings.loot_energyammo) {
    std::cout << "7 - " << greenColor << "Energy Ammo\n"
              << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "Energy Ammo\n"
              << resetColor << std::endl;
  }
  std::cout << "Energy Weapon Mags:\n" << std::endl;

  // Display colored options

  if (global_settings.loot_energyammomag1) {
    std::cout << "8 - " << greenColor << "Energy Weapon Mag" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "Energy Weapon Mag" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_energyammomag2) {
    std::cout << "9 - " << greenColor << "Energy Weapon Mag" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "Energy Weapon Mag" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_energyammomag3) {
    std::cout << "10 - " << greenColor << "Energy Weapon Mag" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Energy Weapon Mag" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_energyammomag4) {
    std::cout << "11 - " << greenColor << "Energy Weapon Mag" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "11 - " << redColor << "Energy Weapon Mag" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Stocks:\n" << std::endl;

  if (global_settings.loot_stockregular1) {
    std::cout << "12 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "12 - " << redColor << "Weapon Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular2) {
    std::cout << "13 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "13 - " << redColor << "Weapon Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_stockregular3) {
    std::cout << "14 - " << greenColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "14 - " << redColor << "Weapon Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Suppressors:\n" << std::endl;

  if (global_settings.loot_suppressor1) {
    std::cout << "15 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "15 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor2) {
    std::cout << "16 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "16 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor3) {
    std::cout << "17 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "17 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Lasers:\n" << std::endl;

  if (global_settings.loot_lasersight1) {
    std::cout << "18 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "18 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight2) {
    std::cout << "19 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "19 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight3) {
    std::cout << "20 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "20 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_lasersight4) {
    std::cout << "21 - " << greenColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "21 - " << redColor << "Weapon Lasers" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Hop-Ups:\n" << std::endl;

  if (global_settings.loot_turbo_charger) {
    std::cout << "22 - " << greenColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "22 - " << redColor << "Turbo Charger" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_skull_piecer) {
    std::cout << "23 - " << greenColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "23 - " << redColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_hammer_point) {
    std::cout << "24 - " << greenColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "24 - " << redColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_disruptor_rounds) {
    std::cout << "25 - " << greenColor << "Disruptor Rounds" << resetColor
              << ": " << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "25 - " << redColor << "Disruptor Rounds" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_boosted_loader) {
    std::cout << "26 - " << greenColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "26 - " << redColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "27 - Back to Settings Menu\n" << std::endl;
}

void displaySniperWeapons() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << yellowGoldColor << "Sniper Weapons:\n"
            << resetColor << std::endl;
  if (global_settings.loot_weapon_wingman) {
    std::cout << "1 - " << greenColor << "Wingman" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "Wingman" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_longbow) {
    std::cout << "2 - " << greenColor << "Longbow" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Longbow" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_charge_rifle) {
    std::cout << "3 - " << greenColor << "Charge Rifle" << resetColor
              << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Charge Rifle" << resetColor
              << std::endl;
  }

  if (global_settings.loot_weapon_sentinel) {
    std::cout << "4 - " << greenColor << "Sentinel" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Sentinel" << resetColor << std::endl;
  }

  if (global_settings.loot_weapon_bow) {
    std::cout << "5 - " << greenColor << "Bow" << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Bow" << resetColor << std::endl;
  }

  if (global_settings.loot_sniperammo) {
    std::cout << "6 - " << greenColor << "Sniper Ammo\n"
              << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "Sniper Ammo\n"
              << resetColor << std::endl;
  }

  std::cout << "Sniper Weapon Mags:\n" << std::endl;

  if (global_settings.loot_sniperammomag1) {
    std::cout << "7 - " << greenColor << "Sniper Ammo Mag:" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "Sniper Ammo Mag:" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  // Display colored options

  if (global_settings.loot_sniperammomag2) {
    std::cout << "8 - " << greenColor << "Sniper Ammo Mag:" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "Sniper Ammo Mag:" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_sniperammomag3) {
    std::cout << "9 - " << greenColor << "Sniper Ammo Mag:" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "Sniper Ammo Mag:" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_sniperammomag4) {
    std::cout << "10 - " << greenColor << "Sniper Ammo Mag:" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Sniper Ammo Mag:" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Sniper Stocks:\n" << std::endl;

  if (global_settings.loot_stocksniper1) {
    std::cout << "11 - " << greenColor << "Sniper Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "11 - " << redColor << "Sniper Stock" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_stocksniper2) {
    std::cout << "12 - " << greenColor << "Sniper Stock" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "12 - " << redColor << "Sniper Stock" << resetColor << ": "
              << blueColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_stocksniper3) {
    std::cout << "13 - " << greenColor << "Sniper Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "13 - " << redColor << "Sniper Stock" << resetColor << ": "
              << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Suppressors:\n" << std::endl;

  if (global_settings.loot_suppressor1) {
    std::cout << "14 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "14 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor2) {
    std::cout << "15 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "15 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_suppressor3) {
    std::cout << "16 - " << greenColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  } else {
    std::cout << "16 - " << redColor << "Weapon Suppressors" << resetColor
              << ": " << purpleColor << "Purple\n"
              << resetColor << std::endl;
  }

  std::cout << "Weapon Hop-Ups:\n" << std::endl;

  if (global_settings.loot_turbo_charger) {
    std::cout << "17 - " << greenColor << "Turbo Chager" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "17 - " << redColor << "Turbo Chager" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_skull_piecer) {
    std::cout << "18 - " << greenColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "18 - " << redColor << "Skull Piecer" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_hammer_point) {
    std::cout << "19 - " << greenColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "19 - " << redColor << "Hammer Points" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_disruptor_rounds) {
    std::cout << "20 - " << greenColor << "Disruptor Rounds" << resetColor
              << ": " << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "20 - " << redColor << "Disruptor Rounds" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_boosted_loader) {
    std::cout << "21 - " << greenColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "21 - " << redColor << "Boosted Loader" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "22 - Back to Settings Menu\n" << std::endl;
}

void displayArmors() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << whiteColor << "Armors:\n" << resetColor << std::endl;

  if (global_settings.loot_shieldupgrade1) {
    std::cout << "1 - " << greenColor << "White Armor" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "White Armor" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgrade2) {
    std::cout << "2 - " << greenColor << "Blue Armor" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Blue Armor" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgrade3) {
    std::cout << "3 - " << greenColor << "Purple Armor" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Purple Armor" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgrade4) {
    std::cout << "4 - " << greenColor << "Gold Armor" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Gold Armor" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgrade5) {
    std::cout << "5 - " << greenColor << "Red Armor" << resetColor << ": "
              << redColor << "Red\n"
              << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Red Armor" << resetColor << ": "
              << redColor << "Red\n"
              << resetColor << std::endl;
  }

  std::cout << "Helmets:\n" << std::endl;

  if (global_settings.loot_shieldupgradehead1) {
    std::cout << "6 - " << greenColor << "Helmet" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "Helmet" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgradehead2) {
    std::cout << "7 - " << greenColor << "Helmet" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "Helmet" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgradehead3) {
    std::cout << "8 - " << greenColor << "Helmet" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "Helmet" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldupgradehead4) {
    std::cout << "9 - " << greenColor << "Helmet" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "Helmet" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "Knockdown Shields:\n" << std::endl;

  if (global_settings.loot_shielddown1) {
    std::cout << "10 - " << greenColor << "Knockdown Shield" << resetColor
              << ": " << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Knockdown Shield" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_shielddown2) {
    std::cout << "11 - " << greenColor << "Knockdown Shield" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "11 - " << redColor << "Knockdown Shield" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_shielddown3) {
    std::cout << "12 - " << greenColor << "Knockdown Shield" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "12 - " << redColor << "Knockdown Shield" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_shielddown4) {
    std::cout << "13 - " << greenColor << "Knockdown Shield" << resetColor
              << ": " << goldColor << "Gold\n"
              << resetColor << std::endl;
  } else {
    std::cout << "13 - " << redColor << "Knockdown Shield" << resetColor << ": "
              << goldColor << "Gold\n"
              << resetColor << std::endl;
  }

  std::cout << "14 - Back to Settings Menu\n" << std::endl;
}

void displayHealing() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << whiteColor << "Healing Items:\n" << resetColor << std::endl;

  if (global_settings.loot_accelerant) {
    std::cout << "1 - " << greenColor << "Accelerant" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "Accelerant" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_phoenix) {
    std::cout << "2 - " << greenColor << "Phoenix" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Phoenix" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_healthlarge) {
    std::cout << "3 - " << greenColor << "Large Health" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Large Health" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldbattsmall) {
    std::cout << "4 - " << greenColor << "Small Shield Batt" << resetColor
              << ": " << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Small Shield Batt" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_shieldbattlarge) {
    std::cout << "5 - " << greenColor << "Large Shield Batt" << resetColor
              << ": " << blueColor << "Blue\n"
              << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Large Shield Batt" << resetColor << ": "
              << blueColor << "Blue\n"
              << resetColor << std::endl;
  }

  std::cout << "6 - Back to Settings Menu\n" << std::endl;
}

void displayNades() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << whiteColor << "Nade Items:\n" << resetColor << std::endl;

  if (global_settings.loot_grenade_frag) {
    std::cout << "1 - " << greenColor << "Frag Grenade" << resetColor << ": "
              << redColor << "Red" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "Frag Grenade" << resetColor << ": "
              << redColor << "Red" << resetColor << std::endl;
  }

  if (global_settings.loot_grenade_arc_star) {
    std::cout << "2 - " << greenColor << "Arc Star" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Arc Star" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_grenade_thermite) {
    std::cout << "3 - " << greenColor << "Thermite" << resetColor << ": "
              << redColor << "Red" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Thermite" << resetColor << ": "
              << redColor << "Red" << resetColor << std::endl;
  }

  std::cout << "4 - Back to Settings Menu\n" << std::endl;
}

void displayBackpacks() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << whiteColor << "Backpacks:\n" << resetColor << std::endl;

  if (global_settings.loot_lightbackpack) {
    std::cout << "1 - " << greenColor << "Light Backpack" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "Light Backpack" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_medbackpack) {
    std::cout << "2 - " << greenColor << "Medium Backpack" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "Medium Backpack" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_heavybackpack) {
    std::cout << "3 - " << greenColor << "Heavy Backpack" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "Heavy Backpack" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_goldbackpack) {
    std::cout << "4 - " << greenColor << "Gold Backpack" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "Gold Backpack" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  std::cout << "5 - Back to Settings Menu\n" << std::endl;
}

void displayScopes() {
  // Color definitions using ANSI escape codes
  std::string whiteColor = "\033[1;37m";      // White text
  std::string blueColor = "\033[1;34m";       // Blue text
  std::string purpleColor = "\033[1;35m";     // Purple text
  std::string goldColor = "\033[1;33m";       // Gold text
  std::string yellowColor = "\033[1;33m";     // Yellow text
  std::string redColor = "\033[1;31m";        // Red text
  std::string greenColor = "\033[1;32m";      // Green text
  std::string tealColor = "\033[1;36m";       // Teal text
  std::string yellowGoldColor = "\033[1;33m"; // Yellow-gold text
  std::string resetColor = "\033[0m";         // Reset text color to default

  std::cout << redColor << "Red = Disable" << resetColor << " - " << greenColor
            << "Green = Enabled" << resetColor << resetColor << std::endl;
  std::cout << whiteColor << "Scopes:\n" << resetColor << std::endl;

  if (global_settings.loot_optic1xhcog) {
    std::cout << "1 - " << greenColor << "1x HCOG" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "1 - " << redColor << "1x HCOG" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_optic2xhcog) {
    std::cout << "2 - " << greenColor << "2x HCOG" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "2 - " << redColor << "2x HCOG" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_opticholo1x) {
    std::cout << "3 - " << greenColor << "1x HOLO" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  } else {
    std::cout << "3 - " << redColor << "1x HOLO" << resetColor << ": "
              << whiteColor << "White" << resetColor << std::endl;
  }

  if (global_settings.loot_opticholo1x2x) {
    std::cout << "4 - " << greenColor << "1x-2x HOLO" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "4 - " << redColor << "1x-2x HOLO" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_opticthreat) {
    std::cout << "5 - " << greenColor << "Optic Threat" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "5 - " << redColor << "Optic Threat" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  if (global_settings.loot_optic3xhcog) {
    std::cout << "6 - " << greenColor << "3x HCOG" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "6 - " << redColor << "3x HCOG" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_optic2x4x) {
    std::cout << "7 - " << greenColor << "2x-4x HCOG" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "7 - " << redColor << "2x-4x HCOG" << resetColor << ": "
              << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_opticsniper6x) {
    std::cout << "8 - " << greenColor << "6x Sniper Optic" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  } else {
    std::cout << "8 - " << redColor << "6x Sniper Optic" << resetColor << ": "
              << blueColor << "Blue" << resetColor << std::endl;
  }

  if (global_settings.loot_opticsniper4x8x) {
    std::cout << "9 - " << greenColor << "4x-8x Sniper Optic" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  } else {
    std::cout << "9 - " << redColor << "4x-8x Sniper Optic" << resetColor
              << ": " << purpleColor << "Purple" << resetColor << std::endl;
  }

  if (global_settings.loot_opticsniperthreat) {
    std::cout << "10 - " << greenColor << "Sniper Threat" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  } else {
    std::cout << "10 - " << redColor << "Sniper Threat" << resetColor << ": "
              << goldColor << "Gold" << resetColor << std::endl;
  }

  std::cout << "11 - Back to Settings Menu\n" << std::endl;
}

int getMenuOption() {
  int option;
  std::cout << "Enter a command ##: ";
  std::cin >> option;
  return option;
}
// keycode stuff
void printKeyCodes() {
  std::cout << "Key Codes:" << std::endl;
  std::cout << "108 Left mouse button (mouse1)" << std::endl;
  std::cout << "109 Right mouse button (mouse2)" << std::endl;
  std::cout << "110 Middle mouse button (mouse3)" << std::endl;
  std::cout << "111 Side mouse button (mouse4)" << std::endl;
  std::cout << "112 Side mouse button (mouse5)" << std::endl;
  std::cout << "79 SHIFT key" << std::endl;
  std::cout << "81 ALT key" << std::endl;
  std::cout << "83 CTRL key" << std::endl;
  std::cout << "1 KEY_0" << std::endl;
  std::cout << "2 KEY_1" << std::endl;
  std::cout << "3 KEY_2" << std::endl;
  std::cout << "4 KEY_3" << std::endl;
  std::cout << "5 KEY_4" << std::endl;
  std::cout << "6 KEY_5" << std::endl;
  std::cout << "7 KEY_6" << std::endl;
  std::cout << "8 KEY_7" << std::endl;
  std::cout << "9 KEY_8" << std::endl;
  std::cout << "10 KEY_9" << std::endl;
  std::cout << "11 KEY_A" << std::endl;
  std::cout << "12 KEY_B" << std::endl;
  std::cout << "13 KEY_C" << std::endl;
  std::cout << "14 KEY_D" << std::endl;
  std::cout << "15 KEY_E" << std::endl;
  std::cout << "16 KEY_F" << std::endl;
  std::cout << "17 KEY_G" << std::endl;
  std::cout << "18 KEY_H" << std::endl;
  std::cout << "19 KEY_I" << std::endl;
  std::cout << "20 KEY_J" << std::endl;
  std::cout << "21 KEY_K" << std::endl;
  std::cout << "22 KEY_L" << std::endl;
  std::cout << "23 KEY_M" << std::endl;
  std::cout << "24 KEY_N" << std::endl;
  std::cout << "25 KEY_O" << std::endl;
  std::cout << "26 KEY_P" << std::endl;
  std::cout << "27 KEY_Q" << std::endl;
  std::cout << "28 KEY_R" << std::endl;
  std::cout << "29 KEY_S" << std::endl;
  std::cout << "30 KEY_T" << std::endl;
  std::cout << "31 KEY_U" << std::endl;
  std::cout << "32 KEY_V" << std::endl;
  std::cout << "33 KEY_W" << std::endl;
  std::cout << "34 KEY_X" << std::endl;
  std::cout << "35 KEY_Y" << std::endl;
  std::cout << "36 KEY_Z" << std::endl;
  std::cout << "37 KEY_PAD_0" << std::endl;
  std::cout << "38 KEY_PAD_1" << std::endl;
  std::cout << "39 KEY_PAD_2" << std::endl;
  std::cout << "40 KEY_PAD_3" << std::endl;
  std::cout << "41 KEY_PAD_4" << std::endl;
  std::cout << "42 KEY_PAD_5" << std::endl;
  std::cout << "43 KEY_PAD_6" << std::endl;
  std::cout << "44 KEY_PAD_7" << std::endl;
  std::cout << "45 KEY_PAD_8" << std::endl;
  std::cout << "46 KEY_PAD_9" << std::endl;
  std::cout << "47 KEY_PAD_DIVIDE" << std::endl;
  std::cout << "48 KEY_PAD_MULTIPLY" << std::endl;
  std::cout << "49 KEY_PAD_MINUS" << std::endl;
  std::cout << "50 KEY_PAD_PLUS" << std::endl;
  std::cout << "51 KEY_PAD_ENTER" << std::endl;
  std::cout << "52 KEY_PAD_DECIMAL" << std::endl;
  std::cout << "65 KEY_SPACE" << std::endl;
  std::cout << "67 KEY_TAB" << std::endl;
  std::cout << "68 KEY_CAPSLOCK" << std::endl;
  std::cout << "69 KEY_NUMLOCK" << std::endl;
  std::cout << "70 KEY_ESCAPE" << std::endl;
  std::cout << "71 KEY_SCROLLLOCK" << std::endl;
  std::cout << "72 KEY_INSERT" << std::endl;
  std::cout << "73 KEY_DELETE" << std::endl;
  std::cout << "74 KEY_HOME" << std::endl;
  std::cout << "75 KEY_END" << std::endl;
  std::cout << "76 KEY_PAGEUP" << std::endl;
  std::cout << "77 KEY_PAGEDOWN" << std::endl;
  std::cout << "78 KEY_BREAK" << std::endl;
  std::cout << "88 KEY_UP" << std::endl;
  std::cout << "89 KEY_LEFT" << std::endl;
  std::cout << "90 KEY_DOWN" << std::endl;
  std::cout << "91 KEY_RIGHT" << std::endl;
  std::cout << "92 KEY_F1" << std::endl;
  std::cout << "93 KEY_F2" << std::endl;
  std::cout << "94 KEY_F3" << std::endl;
  std::cout << "95 KEY_F4" << std::endl;
  std::cout << "96 KEY_F5" << std::endl;
  std::cout << "97 KEY_F6" << std::endl;
  std::cout << "98 KEY_F7" << std::endl;
  std::cout << "99 KEY_F8" << std::endl;
  std::cout << "100 KEY_F9" << std::endl;
  std::cout << "101 KEY_F10" << std::endl;
  std::cout << "102 KEY_F11" << std::endl;
  std::cout << "103 KEY_F12" << std::endl;
}

extern void start_overlay();

void terminal() {
  bool exitProgram = false;
  int menuLevel = 0; // 0 for Main Menu, 1 for Settings Menu, 2 for Submenu

  while (!exitProgram) {
    std::this_thread::sleep_for(std::chrono::milliseconds(600));
    system("clear"); // Use "cls" for Windows

    if (menuLevel == 0) {
      displayMainMenu();
    } else if (menuLevel == 1) {
      displayItemFilterMenu();
    } else if (menuLevel == 2) {
      displayLightWeapons();
    } else if (menuLevel == 3) {
      displayHeavyWeapons();
    } else if (menuLevel == 4) {
      displayEnergyWeapons();
    } else if (menuLevel == 5) {
      displaySniperWeapons();
    } else if (menuLevel == 6) {
      displayArmors();
    } else if (menuLevel == 7) {
      displayHealing();
    } else if (menuLevel == 8) {
      displayNades();
    } else if (menuLevel == 9) {
      displayBackpacks();
    } else if (menuLevel == 10) {
      displayScopes();
    }

    int option = getMenuOption();

    // Main Menu
    if (menuLevel == 0) // displayMainMenu();
    {
      std::string userInput;
      if (option == 1) {
        // Toggle the firing_range.
        global_settings.firing_range = !global_settings.firing_range;

        if (global_settings.firing_range) {
          std::cout << "Firing Range ON.\n";
        } else {
          std::cout << "Firing Range OFF.\n";
        }
      } else if (option == 2) {
        // Toggle TDM.
        global_settings.tdm_toggle = !global_settings.tdm_toggle;

        if (global_settings.tdm_toggle) {
          std::cout << "TDM ON.\n";
        } else {
          std::cout << "TDM OFF.\n";
        }
      } else if (option == 3) {
        // Keyboard Enable.
        global_settings.keyboard = true;
        global_settings.gamepad = false;
        std::cout << "Keyboard ON.\n";
      } else if (option == 4) {
        // Gamepad Enable.
        global_settings.keyboard = false;
        global_settings.gamepad = true;
        std::cout << "Gamepad ON.\n";
      } else if (option == 5) {
        // Toggle TDM.
        global_settings.item_glow = !global_settings.item_glow;

        if (global_settings.item_glow) {
          std::cout << "Item Glow ON.\n";
        } else {
          std::cout << "Item Glow OFF.\n";
        }
      } else if (option == 6) {
        // Toggle TDM.
        global_settings.player_glow = !global_settings.player_glow;

        if (global_settings.player_glow) {
          std::cout << "Player Glow ON.\n";
        } else {
          std::cout << "Player Glow OFF.\n";
        }
      } else if (option == 7) {
        // Command to change the 'smooth' value.
        std::cout << "Enter a new value for 'smooth' (70 to 200): ";
        float newSmooth;
        std::cin >> newSmooth;

        // Check if the new value is within the desired range.
        if (newSmooth >= 50.0f && newSmooth <= 500.0f) {
          global_settings.smooth = newSmooth;
          global_settings.skynade_smooth = global_settings.smooth * 0.6667f;
          std::cout << "'smooth' value updated to: " << global_settings.smooth
                    << std::endl;
          printf("The value of 'smooth' is: %f\n", global_settings.smooth);
        } else {
          std::cout
              << "Invalid value. 'smooth' value must be between 70 and 500."
              << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 8) {
        // Command to change the 'smooth' value.
        std::cout << "Enter a new value for 'bone' (0 to 3): ";
        int newBone;
        std::cin >> newBone;

        // Check if the new value is within the desired range.
        if (newBone >= 0 && newBone <= 3) {
          global_settings.bone = newBone;
          std::cout << "'bone' value updated to: " << global_settings.bone
                    << std::endl;
        } else {
          std::cout << "Invalid value. 'bone' value must be between 0 and 3."
                    << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 9) {
        // Loot Filled.
        global_settings.loot_filled_toggle =
            !global_settings.loot_filled_toggle;

        if (global_settings.loot_filled_toggle) {
          global_settings.loot_filled = 14;
          std::cout << "Loot Glow Filled.\n";
        } else {
          global_settings.loot_filled = 0;
          std::cout << "Loot Glow Not Filled.\n";
        }
      }

      else if (option == 10) {
        // player Filled.
        global_settings.player_filled_toggle =
            !global_settings.player_filled_toggle;

        if (global_settings.player_filled_toggle) {
          global_settings.inside_value = 14;
          std::cout << "Player Glow Filled.\n";
        } else {
          global_settings.inside_value = 0;
          std::cout << "Player Glow Not Filled.\n";
        }
      } else if (option == 11) {
        // Command to change the 'Player Outlines' value.
        std::cout << "Enter a new value for Player Outlines (0 to 255): ";
        int newoutlinesize;
        std::cin >> newoutlinesize;

        // Check if the new value is within the desired range.
        if (newoutlinesize >= 0 && newoutlinesize <= 255) {
          global_settings.outline_size = newoutlinesize;
          std::cout << "Player Outline updated to: "
                    << global_settings.outline_size << std::endl;
        } else {
          std::cout
              << "Invalid value. 'outlinesize' value must be between 0 and 255."
              << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 12) {
        // Select a glow set (1 for "Not Visible," 2 for "Visible," 3 for
        // "Knocked").
        std::cout << "Select Glow: 1 - Not Visible, 2 - Visible, 3 - Knocked "
                     "(can do fractions IE: 0.863: ";
        std::cin >> userInput;
        int selectedSet = std::stoi(userInput);

        switch (selectedSet) {
        case 1:
          updateGlowColor(global_settings.glow_r_not,
                          global_settings.glow_g_not,
                          global_settings.glow_b_not, "'Not Visible'");
          break;
        case 2:
          updateGlowColor(global_settings.glow_r_viz,
                          global_settings.glow_g_viz,
                          global_settings.glow_b_viz, "'Visible'");
          break;
        case 3:
          updateGlowColor(global_settings.glow_r_knocked,
                          global_settings.glow_g_knocked,
                          global_settings.glow_b_knocked, "'Knocked'");
          break;
        default:
          std::cout << "Invalid set selection. Please choose 1-3.\n";
          break;
        }
      } else if (option == 13) {
        // Command to change the 'smooth' value.
        std::cout << "Enter a new value for 'ADS FOV' (1 to 50): ";
        float newADSfov;
        std::cin >> newADSfov;

        // Check if the new value is within the desired range.
        if (newADSfov >= 0.0f && newADSfov <= 50.0f) {
          global_settings.ads_fov = newADSfov;
          std::cout << "'ADS FOV' value updated to: " << global_settings.ads_fov
                    << std::endl;
          printf("The value of 'ADS FOV' is: %f\n", global_settings.ads_fov);
        } else {
          std::cout
              << "Invalid value. 'ADS FOV' value must be between 85 and 200."
              << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 14) {
        // Command to change the 'smooth' value.
        std::cout << "Enter a new value for 'Non-ADS FOV' (1 to 50): ";
        float newnonADSfov;
        std::cin >> newnonADSfov;

        // Check if the new value is within the desired range.
        if (newnonADSfov >= 0.0f && newnonADSfov <= 50.0f) {
          global_settings.non_ads_fov = newnonADSfov;
          std::cout << "'Non-ADS FOV' value updated to: "
                    << global_settings.non_ads_fov << std::endl;
          printf("The value of 'Non-ADS FOV' is: %f\n",
                 global_settings.non_ads_fov);
        } else {
          std::cout << "Invalid value. 'Non-ADS FOV' value must be between 85 "
                       "and 200."
                    << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 15) {
        // player Filled.
        global_settings.super_key = !global_settings.super_key;
        global_settings.super_key_toggle = !global_settings.super_key_toggle;
      }

      else if (option == 16) {
        //  displayItemFilterMenu
        menuLevel = 1;
      }
      // Keycode stuff

      else if (option == 17) {
        // Optionally print the key codes before updating
        std::cout << "Do you want to see the key codes before updating? (1 for "
                     "yes, 0 for no): ";
        int showKeyCodesBeforeUpdate;
        std::cin >> showKeyCodesBeforeUpdate;

        if (showKeyCodesBeforeUpdate == 1) {
          printKeyCodes();
        }

        // Command to change the 'AimbotHotKey1' value.
        std::cout << "Enter a new value for 'AimbotHotKey1' (e.g., 108 for "
                     "Left mouse button): ";
        int newAimbotHotKey1;
        std::cin >> newAimbotHotKey1;

        // Check if the new value is within the desired range (e.g., 0-255 for
        // key codes).
        if (newAimbotHotKey1 >= 0 && newAimbotHotKey1 <= 255) {
          global_settings.aimbot_hot_key_1 = newAimbotHotKey1;
          std::cout << "'AimbotHotKey1' value updated to: "
                    << global_settings.aimbot_hot_key_1 << std::endl;
          printf("The value of 'AimbotHotKey1' is: %d\n",
                 global_settings.aimbot_hot_key_1);
        } else {
          std::cout << "Invalid value. 'AimbotHotKey1' value must be between 0 "
                       "and 255."
                    << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      }

      else if (option == 18) {
        // Optionally print the key codes before updating
        std::cout << "Do you want to see the key codes before updating? (1 for "
                     "yes, 0 for no): ";
        int showKeyCodesBeforeUpdate;
        std::cin >> showKeyCodesBeforeUpdate;

        if (showKeyCodesBeforeUpdate == 1) {
          printKeyCodes();
        }
        // Command to change the 'AimbotHotKey2' value.
        std::cout << "Enter a new value for 'AimbotHotKey2' (e.g., 109 for "
                     "Right mouse button): ";
        printKeyCodes();
        int newAimbotHotKey2;
        std::cin >> newAimbotHotKey2;

        // Check if the new value is within the desired range (e.g., 0-255 for
        // key codes).
        if (newAimbotHotKey2 >= 0 && newAimbotHotKey2 <= 255) {
          global_settings.aimbot_hot_key_2 = newAimbotHotKey2;
          std::cout << "'AimbotHotKey2' value updated to: "
                    << global_settings.aimbot_hot_key_2 << std::endl;
          printf("The value of 'AimbotHotKey2' is: %d\n",
                 global_settings.aimbot_hot_key_2);
        } else {
          std::cout << "Invalid value. 'AimbotHotKey2' value must be between 0 "
                       "and 255."
                    << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 19) {
        // Optionally print the key codes before updating
        std::cout << "Do you want to see the key codes before updating? (1 for "
                     "yes, 0 for no): ";
        int showKeyCodesBeforeUpdate;
        std::cin >> showKeyCodesBeforeUpdate;

        if (showKeyCodesBeforeUpdate == 1) {
          printKeyCodes();
        }
        // Command to change the 'TriggerBotHotKey' value.
        std::cout << "Enter a new value for 'TriggerBotHotKey': ";
        printKeyCodes();
        int newTriggerBotHotKey;
        std::cin >> newTriggerBotHotKey;

        // Check if the new value is within the desired range (e.g., 0-255 for
        // key codes).
        if (newTriggerBotHotKey >= 0 && newTriggerBotHotKey <= 255) {
          global_settings.tigger_bot_hot_key = newTriggerBotHotKey;
          std::cout << "'TriggerBotHotKey' value updated to: "
                    << global_settings.tigger_bot_hot_key << std::endl;
          printf("The value of 'TriggerBotHotKey' is: %d\n",
                 global_settings.tigger_bot_hot_key);
        } else {
          std::cout << "Invalid value. 'TriggerBotHotKey' value must be "
                       "between 0 and 255."
                    << std::endl;
        }

        // Clear the input buffer to prevent any issues with future input.
        std::cin.clear();
        std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
      } else if (option == 20) {
        global_settings.deathbox = !global_settings.deathbox;
        std::cout << "Death Boxes " << (global_settings.deathbox ? "on" : "off")
                  << "." << std::endl;
      } else if (option == 21) {
        std::cout << (save_settings(global_settings) ? "Saved" : "Error")
                  << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(2000));
      } else if (option == 22) {
        global_settings = load_settings();
        std::cout << "global_settings updated!" << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(2000));
      } else if (option == 23) {
        global_settings.no_nade_aim = !global_settings.no_nade_aim;
        std::cout << "NoNadeAim "
                  << (global_settings.no_nade_aim ? "on" : "off") << "."
                  << std::endl;
      } else if (option == 24) {
        global_settings.onevone = !global_settings.onevone;
        std::cout << "1v1 " << (global_settings.onevone ? "on" : "off") << "."
                  << std::endl;
      } else if (option == 25) {
        global_settings.aim_no_recoil = !global_settings.aim_no_recoil;
        std::cout << "No Recoil "
                  << (global_settings.aim_no_recoil ? "on" : "off") << "."
                  << std::endl;
      } else if (option == 26) {
        std::cout << "Enter a new value for 'Game FPS for Aim Predict': ";
        int tmp_value;
        std::cin >> tmp_value;
        if (tmp_value == 0) {
          global_settings.calc_game_fps = true;
        } else if (tmp_value > 0 && tmp_value < 500) {
          global_settings.game_fps = tmp_value;
          global_settings.calc_game_fps = false;
          std::cout << "Game FPS for Aim Prediction: (Current: ";
          if (global_settings.calc_game_fps)
            std::cout << "calc game fps";
          else
            std::cout << std::setiosflags(std::ios::fixed)
                      << std::setprecision(1) << global_settings.game_fps;
          std::cout << ")" << std::endl;
        } else {
          std::cout << "Invalid value!" << std::endl;
        }
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Filter Item Menu
    else if (menuLevel == 1) {
      if (option == 1) {
        // Light Weapons
        menuLevel = 2;
      } else if (option == 2) {
        // Heavy weapons
        menuLevel = 3;
      } else if (option == 3) {
        // Return to the Main Menu.
        menuLevel = 4;
      } else if (option == 4) {
        // Return to the Main Menu.
        menuLevel = 5;
      } else if (option == 5) {
        // Return to the Main Menu.
        menuLevel = 6;
      } else if (option == 6) {
        // Return to the Main Menu.
        menuLevel = 7;
      } else if (option == 7) {
        // Return to the Main Menu.
        menuLevel = 8;
      } else if (option == 8) {
        // Return to the Main Menu.
        menuLevel = 9;
      } else if (option == 9) {
        // Return to the Main Menu.
        menuLevel = 10;
      } else if (option == 10) {
        // Return to the Main Menu.
        menuLevel = 0;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Light weapons
    else if (menuLevel == 2) // light
    {
      if (option == 1) {
        global_settings.loot_weapon_p2020 = !global_settings.loot_weapon_p2020;
      } else if (option == 2) {
        global_settings.loot_weapon_re45 = !global_settings.loot_weapon_re45;
      } else if (option == 3) {
        global_settings.loot_weapon_alternator =
            !global_settings.loot_weapon_alternator;
      } else if (option == 4) {
        global_settings.loot_weapon_r99 = !global_settings.loot_weapon_r99;
      } else if (option == 5) {
        global_settings.loot_weapon_r301 = !global_settings.loot_weapon_r301;
      } else if (option == 6) {
        global_settings.loot_weapon_spitfire =
            !global_settings.loot_weapon_spitfire;
      } else if (option == 7) {
        global_settings.loot_weapon_g7_scout =
            !global_settings.loot_weapon_g7_scout;
      } else if (option == 8) {
        global_settings.loot_lightammo = !global_settings.loot_lightammo;
      } else if (option == 9) {
        global_settings.loot_lightammomag1 =
            !global_settings.loot_lightammomag1;
      } else if (option == 10) {
        global_settings.loot_lightammomag2 =
            !global_settings.loot_lightammomag2;
      } else if (option == 11) {
        global_settings.loot_lightammomag3 =
            !global_settings.loot_lightammomag3;
      } else if (option == 12) {
        global_settings.loot_lightammomag4 =
            !global_settings.loot_lightammomag4;
      } else if (option == 13) {
        global_settings.loot_stockregular1 =
            !global_settings.loot_stockregular1;
      } else if (option == 14) {
        global_settings.loot_stockregular2 =
            !global_settings.loot_stockregular2;
      } else if (option == 15) {
        global_settings.loot_stockregular3 =
            !global_settings.loot_stockregular3;
      } else if (option == 16) {
        global_settings.loot_suppressor1 = !global_settings.loot_suppressor1;
      } else if (option == 17) {
        global_settings.loot_suppressor2 = !global_settings.loot_suppressor2;
      } else if (option == 18) {
        global_settings.loot_suppressor3 = !global_settings.loot_suppressor3;
      } else if (option == 19) {
        global_settings.loot_lasersight1 = !global_settings.loot_lasersight1;
      } else if (option == 20) {
        global_settings.loot_lasersight2 = !global_settings.loot_lasersight2;
      } else if (option == 21) {
        global_settings.loot_lasersight3 = !global_settings.loot_lasersight3;
      } else if (option == 22) {
        global_settings.loot_lasersight4 = !global_settings.loot_lasersight4;
      } else if (option == 23) {
        global_settings.loot_turbo_charger =
            !global_settings.loot_turbo_charger;
      } else if (option == 24) {
        global_settings.loot_skull_piecer = !global_settings.loot_skull_piecer;
      } else if (option == 25) {
        global_settings.loot_hammer_point = !global_settings.loot_hammer_point;
      } else if (option == 26) {
        global_settings.loot_disruptor_rounds =
            !global_settings.loot_disruptor_rounds;
      } else if (option == 27) {
        global_settings.loot_boosted_loader =
            !global_settings.loot_boosted_loader;
      } else if (option == 28) {
        menuLevel = 1;
      }

      else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Heavy weapons
    else if (menuLevel == 3) // heavy
    {
      if (option == 1) {
        global_settings.loot_weapon_flatline =
            !global_settings.loot_weapon_flatline;
      } else if (option == 2) {
        global_settings.loot_weapon_hemlock =
            !global_settings.loot_weapon_hemlock;
      } else if (option == 3) {
        global_settings.loot_weapon_3030_repeater =
            !global_settings.loot_weapon_3030_repeater;
      } else if (option == 4) {
        global_settings.loot_weapon_rampage =
            !global_settings.loot_weapon_rampage;
      } else if (option == 5) {
        global_settings.loot_weapon_prowler =
            !global_settings.loot_weapon_prowler;
      } else if (option == 6) {
        global_settings.loot_weapon_car_smg =
            !global_settings.loot_weapon_car_smg;
      } else if (option == 7) {
        global_settings.loot_heavyammo = !global_settings.loot_heavyammo;
      } else if (option == 8) {
        global_settings.loot_heavyammomag1 =
            !global_settings.loot_heavyammomag1;
      } else if (option == 9) {
        global_settings.loot_heavyammomag2 =
            !global_settings.loot_heavyammomag2;
      } else if (option == 10) {
        global_settings.loot_heavyammomag3 =
            !global_settings.loot_heavyammomag3;
      } else if (option == 11) {
        global_settings.loot_heavyammomag4 =
            !global_settings.loot_heavyammomag4;
      } else if (option == 12) {
        global_settings.loot_stockregular1 =
            !global_settings.loot_stockregular1;
      } else if (option == 13) {
        global_settings.loot_stockregular2 =
            !global_settings.loot_stockregular2;
      } else if (option == 14) {
        global_settings.loot_stockregular3 =
            !global_settings.loot_stockregular3;
      } else if (option == 15) {
        global_settings.loot_suppressor1 = !global_settings.loot_suppressor1;
      } else if (option == 16) {
        global_settings.loot_suppressor2 = !global_settings.loot_suppressor2;
      } else if (option == 17) {
        global_settings.loot_suppressor3 = !global_settings.loot_suppressor3;
      } else if (option == 18) {
        global_settings.loot_lasersight1 = !global_settings.loot_lasersight1;
      } else if (option == 19) {
        global_settings.loot_lasersight2 = !global_settings.loot_lasersight2;
      } else if (option == 20) {
        global_settings.loot_lasersight3 = !global_settings.loot_lasersight3;
      } else if (option == 21) {
        global_settings.loot_lasersight4 = !global_settings.loot_lasersight4;
      } else if (option == 22) {
        global_settings.loot_turbo_charger =
            !global_settings.loot_turbo_charger;
      } else if (option == 23) {
        global_settings.loot_skull_piecer = !global_settings.loot_skull_piecer;
      } else if (option == 24) {
        global_settings.loot_hammer_point = !global_settings.loot_hammer_point;
      } else if (option == 25) {
        global_settings.loot_disruptor_rounds =
            !global_settings.loot_disruptor_rounds;
      } else if (option == 26) {
        global_settings.loot_boosted_loader =
            !global_settings.loot_boosted_loader;
      } else if (option == 27) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Energy weapons
    else if (menuLevel == 4) // energy
    {
      if (option == 1) {
        global_settings.loot_weapon_lstar = !global_settings.loot_weapon_lstar;
      } else if (option == 2) {
        global_settings.loot_weapon_nemesis =
            !global_settings.loot_weapon_nemesis;
      } else if (option == 3) {
        global_settings.loot_weapon_havoc = !global_settings.loot_weapon_havoc;
      } else if (option == 4) {
        global_settings.loot_weapon_devotion =
            !global_settings.loot_weapon_devotion;
      } else if (option == 5) {
        global_settings.loot_weapon_triple_take =
            !global_settings.loot_weapon_triple_take;
      } else if (option == 6) {
        global_settings.loot_weapon_volt = !global_settings.loot_weapon_volt;
      } else if (option == 7) {
        global_settings.loot_energyammo = !global_settings.loot_energyammo;
      } else if (option == 8) {
        global_settings.loot_energyammomag1 =
            !global_settings.loot_energyammomag1;
      } else if (option == 9) {
        global_settings.loot_energyammomag2 =
            !global_settings.loot_energyammomag2;
      } else if (option == 10) {
        global_settings.loot_energyammomag3 =
            !global_settings.loot_energyammomag3;
      } else if (option == 11) {
        global_settings.loot_energyammomag4 =
            !global_settings.loot_energyammomag4;
      } else if (option == 12) {
        global_settings.loot_stockregular1 =
            !global_settings.loot_stockregular1;
      } else if (option == 13) {
        global_settings.loot_stockregular2 =
            !global_settings.loot_stockregular2;
      } else if (option == 14) {
        global_settings.loot_stockregular3 =
            !global_settings.loot_stockregular3;
      } else if (option == 15) {
        global_settings.loot_suppressor1 = !global_settings.loot_suppressor1;
      } else if (option == 16) {
        global_settings.loot_suppressor2 = !global_settings.loot_suppressor2;
      } else if (option == 17) {
        global_settings.loot_suppressor3 = !global_settings.loot_suppressor3;
      } else if (option == 18) {
        global_settings.loot_lasersight1 = !global_settings.loot_lasersight1;
      } else if (option == 19) {
        global_settings.loot_lasersight2 = !global_settings.loot_lasersight2;
      } else if (option == 20) {
        global_settings.loot_lasersight3 = !global_settings.loot_lasersight3;
      } else if (option == 21) {
        global_settings.loot_lasersight4 = !global_settings.loot_lasersight4;
      } else if (option == 22) {
        global_settings.loot_turbo_charger =
            !global_settings.loot_turbo_charger;
      } else if (option == 23) {
        global_settings.loot_skull_piecer = !global_settings.loot_skull_piecer;
      } else if (option == 24) {
        global_settings.loot_hammer_point = !global_settings.loot_hammer_point;
      } else if (option == 25) {
        global_settings.loot_disruptor_rounds =
            !global_settings.loot_disruptor_rounds;
      } else if (option == 26) {
        global_settings.loot_boosted_loader =
            !global_settings.loot_boosted_loader;
      } else if (option == 27) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Sniper weapons
    else if (menuLevel == 5) // Sniper
    {
      if (option == 1) {
        global_settings.loot_weapon_wingman =
            !global_settings.loot_weapon_wingman;
      } else if (option == 2) {
        global_settings.loot_weapon_longbow =
            !global_settings.loot_weapon_longbow;
      } else if (option == 3) {
        global_settings.loot_weapon_charge_rifle =
            !global_settings.loot_weapon_charge_rifle;
      } else if (option == 4) {
        global_settings.loot_weapon_sentinel =
            !global_settings.loot_weapon_sentinel;
      } else if (option == 5) {
        global_settings.loot_weapon_bow = !global_settings.loot_weapon_bow;
      } else if (option == 6) {
        global_settings.loot_sniperammo = !global_settings.loot_sniperammo;
      } else if (option == 7) {
        global_settings.loot_sniperammomag1 =
            !global_settings.loot_sniperammomag1;
      } else if (option == 8) {
        global_settings.loot_sniperammomag2 =
            !global_settings.loot_sniperammomag2;
      } else if (option == 9) {
        global_settings.loot_sniperammomag3 =
            !global_settings.loot_sniperammomag3;
      } else if (option == 10) {
        global_settings.loot_sniperammomag4 =
            !global_settings.loot_sniperammomag4;
      } else if (option == 11) {
        global_settings.loot_stocksniper1 = !global_settings.loot_stocksniper1;
      } else if (option == 12) {
        global_settings.loot_stocksniper2 = !global_settings.loot_stocksniper2;
      } else if (option == 13) {
        global_settings.loot_stocksniper3 = !global_settings.loot_stocksniper3;
      } else if (option == 14) {
        global_settings.loot_suppressor1 = !global_settings.loot_suppressor1;
      } else if (option == 15) {
        global_settings.loot_suppressor2 = !global_settings.loot_suppressor2;
      } else if (option == 16) {
        global_settings.loot_suppressor3 = !global_settings.loot_suppressor3;
      } else if (option == 17) {
        global_settings.loot_turbo_charger =
            !global_settings.loot_turbo_charger;
      } else if (option == 18) {
        global_settings.loot_skull_piecer = !global_settings.loot_skull_piecer;
      } else if (option == 19) {
        global_settings.loot_hammer_point = !global_settings.loot_hammer_point;
      } else if (option == 20) {
        global_settings.loot_disruptor_rounds =
            !global_settings.loot_disruptor_rounds;
      } else if (option == 21) {
        global_settings.loot_boosted_loader =
            !global_settings.loot_boosted_loader;
      } else if (option == 22) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Armors
    else if (menuLevel == 6) {
      if (option == 1) {
        global_settings.loot_shieldupgrade1 =
            !global_settings.loot_shieldupgrade1;
      } else if (option == 2) {
        global_settings.loot_shieldupgrade2 =
            !global_settings.loot_shieldupgrade2;
      } else if (option == 3) {
        global_settings.loot_shieldupgrade3 =
            !global_settings.loot_shieldupgrade3;
      } else if (option == 4) {
        global_settings.loot_shieldupgrade4 =
            !global_settings.loot_shieldupgrade4;
      } else if (option == 5) {
        global_settings.loot_shieldupgrade5 =
            !global_settings.loot_shieldupgrade5;
      } else if (option == 6) {
        global_settings.loot_shieldupgradehead1 =
            !global_settings.loot_shieldupgradehead1;
      } else if (option == 7) {
        global_settings.loot_shieldupgradehead2 =
            !global_settings.loot_shieldupgradehead2;
      } else if (option == 8) {
        global_settings.loot_shieldupgradehead3 =
            !global_settings.loot_shieldupgradehead3;
      } else if (option == 9) {
        global_settings.loot_shieldupgradehead4 =
            !global_settings.loot_shieldupgradehead4;
      } else if (option == 10) {
        global_settings.loot_shielddown1 = !global_settings.loot_shielddown1;
      } else if (option == 11) {
        global_settings.loot_shielddown2 = !global_settings.loot_shielddown2;
      } else if (option == 12) {
        global_settings.loot_shielddown3 = !global_settings.loot_shielddown3;
      } else if (option == 13) {
        global_settings.loot_shielddown4 = !global_settings.loot_shielddown4;
      } else if (option == 14) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Healing Items
    else if (menuLevel == 7) {
      if (option == 1) {
        global_settings.loot_accelerant = !global_settings.loot_accelerant;
      } else if (option == 2) {
        global_settings.loot_phoenix = !global_settings.loot_phoenix;
      } else if (option == 3) {
        global_settings.loot_healthlarge = !global_settings.loot_healthlarge;
      } else if (option == 4) {
        global_settings.loot_healthsmall = !global_settings.loot_healthsmall;
      } else if (option == 5) {
        global_settings.loot_shieldbattsmall =
            !global_settings.loot_shieldbattsmall;
      } else if (option == 6) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Nades
    else if (menuLevel == 8) {
      if (option == 1) {
        global_settings.loot_grenade_frag = !global_settings.loot_grenade_frag;
      } else if (option == 2) {
        global_settings.loot_grenade_arc_star =
            !global_settings.loot_grenade_arc_star;
      } else if (option == 3) {
        global_settings.loot_grenade_thermite =
            !global_settings.loot_grenade_thermite;
      } else if (option == 4) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Backpacks
    else if (menuLevel == 9) {
      if (option == 1) {
        global_settings.loot_lightbackpack =
            !global_settings.loot_lightbackpack;
      } else if (option == 2) {
        global_settings.loot_medbackpack = !global_settings.loot_medbackpack;
      } else if (option == 3) {
        global_settings.loot_heavybackpack =
            !global_settings.loot_heavybackpack;
      } else if (option == 4) {
        global_settings.loot_goldbackpack = !global_settings.loot_goldbackpack;
      } else if (option == 5) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }

    // Scopes
    else if (menuLevel == 10) {
      if (option == 1) {
        global_settings.loot_optic1xhcog = !global_settings.loot_optic1xhcog;
      } else if (option == 2) {
        global_settings.loot_optic2xhcog = !global_settings.loot_optic2xhcog;
      } else if (option == 3) {
        global_settings.loot_opticholo1x = !global_settings.loot_opticholo1x;
      } else if (option == 4) {
        global_settings.loot_opticholo1x2x =
            !global_settings.loot_opticholo1x2x;
      } else if (option == 5) {
        global_settings.loot_opticthreat = !global_settings.loot_opticthreat;
      } else if (option == 6) {
        global_settings.loot_optic3xhcog = !global_settings.loot_optic3xhcog;
      } else if (option == 7) {
        global_settings.loot_optic2x4x = !global_settings.loot_optic2x4x;
      } else if (option == 8) {
        global_settings.loot_opticsniper6x =
            !global_settings.loot_opticsniper6x;
      } else if (option == 9) {
        global_settings.loot_opticsniper4x8x =
            !global_settings.loot_opticsniper4x8x;
      } else if (option == 10) {
        global_settings.loot_opticsniperthreat =
            !global_settings.loot_opticsniperthreat;
      }

      else if (option == 11) {
        menuLevel = 1;
      } else {
        std::cout << "Invalid command. Please try again." << std::endl;
      }
    }
  }
  terminal_t = false;
}

int main(int argc, char *argv[]) {
  global_settings = load_settings();

  if (geteuid() != 0) {
    // run as root..
    print_run_as_root();
    return 0;
  }

  const char *ap_proc = "r5apex.exe";

  std::thread aimbot_thr;
  std::thread esp_thr;
  std::thread actions_thr;
  std::thread cactions_thr;
  // Used to change things on a timer
  // std::thread updateInsideValue_thr;
  std::thread TriggerBotRun_thr;
  std::thread terminal_thr;
  std::thread overlay_thr;
  std::thread itemglow_thr;

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
        // Used to change things on a timer
        updateInsideValue_t = false;
        TriggerBotRun_t = false;
        terminal_t = false;
        overlay_t = false;
        item_t = false;
        g_Base = 0;

        aimbot_thr.~thread();
        esp_thr.~thread();
        actions_thr.~thread();
        cactions_thr.~thread();
        // Used to change things on a timer
        // updateInsideValue_thr.~thread();
        TriggerBotRun_thr.~thread();
        terminal_thr.~thread();
        overlay_thr.~thread();
        itemglow_thr.~thread();
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
        // Used to change things on a timer
        // updateInsideValue_thr = std::thread(updateInsideValue);
        TriggerBotRun_thr = std::thread(TriggerBotRun);
        terminal_thr = std::thread(terminal);
        overlay_thr = std::thread(start_overlay);
        itemglow_thr = std::thread(item_glow_t);
        aimbot_thr.detach();
        esp_thr.detach();
        actions_thr.detach();
        cactions_thr.detach();
        // Used to change things on a timer
        // updateInsideValue_thr.detach();
        TriggerBotRun_thr.detach();
        terminal_thr.detach();
        overlay_thr.detach();
        itemglow_thr.detach();
      }
    } else {
      apex_mem.check_proc();
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
  }

  return 0;
}