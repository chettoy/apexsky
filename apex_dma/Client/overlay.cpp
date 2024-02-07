#include "overlay.h"

#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GL/gl.h>
#include <cstddef>
#include <stdio.h>
#include <vector>
#define GL_SILENCE_DEPRECATION
#if defined(IMGUI_IMPL_OPENGL_ES2)
#include <GLES2/gl2.h>
#endif
#include <GLFW/glfw3.h> // Will drag system OpenGL headers

#define GLFW_EXPOSE_NATIVE_X11
#include <GLFW/glfw3native.h>
#include <X11/Xatom.h>

#include <fstream>
#include <functional>
#include <iomanip>
#include <shared_mutex>
#include <thread>

#include "../Game.h"

using namespace std;

extern bool overlay_t;
extern float veltest;
extern float bulletspeed;
extern float bulletgrav;

extern const size_t g_spectators, g_allied_spectators; // read
extern const std::vector<string> esp_spec_names, esp_teammates_damage;
// Left and Right Aim key toggle
bool toggleaim = false;
bool toggleaim2 = false;
int e = 0;
// Main Map Radar
bool mainradartoggle = 1;

// Menu Stuff
int menu1 = 0;
int menu2 = 0;
int menu3 = 0;
int menu4 = 0;
// // screen pos ajuster
// // ajuster for screen pos
// extern int worldsedgetoprightx = 0;
// extern int worldsedgetoprighty = 0;
// extern int worldsedgebtmleftx = 0;
// extern int worldsedgebtmlefty = 0;

int width;
int height;
bool k_leftclick = false;
bool show_menu = false;

// extern bool IsKeyDown(int vk);
extern bool IsKeyDown(ImGuiKey imgui_k);
extern bool isPressed(uint32_t button_code);

static void StaticMessageStart(Overlay &ov) { ov.CreateOverlay(); }

void Overlay::RenderMenu() {
  static bool aim_enable = false;
  static bool vis_check = false;

  auto g_settings = global_settings();

  if (g_settings.aimbot_settings.aim_mode > 0) {
    aim_enable = true;
    if (g_settings.aimbot_settings.aim_mode > 1) {
      vis_check = true;
    } else {
      vis_check = false;
    }
  } else {
    aim_enable = false;
    vis_check = false;
  }

  ImGui::SetNextWindowPos(ImVec2(0, 0));
  ImGui::SetNextWindowSize(ImVec2(450, this->getHeight() * 0.9),
                           ImGuiCond_Once);
  ImGui::SetNextWindowBgAlpha(0.87);
  ImGui::Begin(xorstr_("##MenuTitle"), (bool *)true,
               ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoScrollbar);
  // if (ImGui::BeginTabBar(xorstr_("Tab")))
  //{
  // if (ImGui::BeginTabItem(xorstr_("##")))
  //{
  if (ImGui::CollapsingHeader("Main Toggle Settings")) {
    menu1 = 1;
    ImGui::Checkbox(xorstr_("ESP On/Off"), &g_settings.esp);
    // ImGui::SameLine();
    // ImGui::Checkbox(xorstr_("Thirdperson"), &thirdperson);

    ImGui::Checkbox(xorstr_("Glow Items"), &g_settings.item_glow);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Mini-Map Radar"), &g_settings.mini_map_radar);

    ImGui::Checkbox(xorstr_("Glow Players"), &g_settings.player_glow);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Mini-Map Guide"), &g_settings.mini_map_guides);

    ImGui::Checkbox(xorstr_("AIM On/Off"), &aim_enable);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("1v1"), &g_settings.onevone);

    if (aim_enable) {
      ImGui::Checkbox(xorstr_("Visibility Check"), &vis_check);
      ImGui::SameLine();
      ImGui::Checkbox(xorstr_("No Recoil"),
                      &g_settings.aimbot_settings.no_recoil);
      ImGui::SameLine();
      ImGui::Checkbox(xorstr_("Auto Nade Aim"),
                      &g_settings.aimbot_settings.auto_nade_aim);
      if (vis_check) {
        g_settings.aimbot_settings.aim_mode = 2;
      } else {
        g_settings.aimbot_settings.aim_mode = 1;
      }
    } else {
      g_settings.aimbot_settings.aim_mode = 0;
    }

    ImGui::Checkbox(xorstr_("Firing Range"), &g_settings.firing_range);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("TDM Toggle"), &g_settings.tdm_toggle);
    ImGui::Checkbox(xorstr_("Press F8 enable MapRadar"),
                    &g_settings.map_radar_testing);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Aiming Distance:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f meters",
                       g_settings.aimbot_settings.aim_dist / 39.62);
    ImGui::SliderFloat(xorstr_("##Aim Distance"),
                       &g_settings.aimbot_settings.aim_dist, 10.0f * 39.62,
                       1600.0f * 39.62, "##");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Aiming Keys:"));
    ImGui::RadioButton(xorstr_("Left Mouse"), &e, 1);
    ImGui::SameLine();
    ImGui::RadioButton(xorstr_("Right Mouse "), &e, 2);
    ImGui::SameLine();
    ImGui::RadioButton(xorstr_("Left/Right Mouse"), &e, 3);
    // Setting one and unsetting the other
    if (e == 1) {
      toggleaim = true;
      toggleaim2 = false;
    } else if (e == 2) {
      toggleaim = false;
      toggleaim2 = true;
    } else if (e == 3) {
      toggleaim = true;
      toggleaim2 = true;
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Max distance for everything:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%d%s", (int)(g_settings.max_dist / 40),
                       xorstr_(" meters"));
    ImGui::SliderFloat(xorstr_("##1"), &g_settings.max_dist, 100.0f * 40,
                       3800.0f * 40, "##");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text("%s", xorstr_("Max fov:"));
    ImGui::Dummy(ImVec2(0.0f, 4.0f));
    ImGui::Text("%s", xorstr_("non-ADS:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", g_settings.aimbot_settings.non_ads_fov);
    ImGui::SliderFloat(xorstr_("##nonADSfov"),
                       &g_settings.aimbot_settings.non_ads_fov, 5.0f, 50.0f,
                       "##");
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text("%s", xorstr_("ADS:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", g_settings.aimbot_settings.ads_fov);
    ImGui::SliderFloat(xorstr_("##ADSfov"), &g_settings.aimbot_settings.ads_fov,
                       5.0f, 50.0f, "##");
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text("%s", xorstr_("Current:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", aimbot_get_max_fov());
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text("%s", xorstr_("Smooth Aim Value:"));
    ImGui::SameLine();
    if (g_settings.aimbot_settings.smooth < 120.0f) {
      ImGui::TextColored(RED, "%.f", g_settings.aimbot_settings.smooth);
    } else if (g_settings.aimbot_settings.smooth >= 160.0f) {
      ImGui::TextColored(GREEN, "%.f", g_settings.aimbot_settings.smooth);
    } else {
      ImGui::TextColored(WHITE, "%.f", g_settings.aimbot_settings.smooth);
    }
    ImGui::SliderFloat(xorstr_("##smooth"), &g_settings.aimbot_settings.smooth,
                       50.0f, 500.0f, "##");
    ImGui::SameLine();
    ImGui::Text("%s", xorstr_("150 To 500 Is Safe"));
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text("%s", xorstr_("Smooth Skynade Aim:"));
    ImGui::SameLine();
    ImGui::TextColored(WHITE, "%.f", g_settings.aimbot_settings.skynade_smooth);
    ImGui::SliderFloat(xorstr_("##skynade_smooth"),
                       &g_settings.aimbot_settings.skynade_smooth, 50.0f,
                       500.0f, "##");

    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text("%s", xorstr_("Smooth Preditcion Speed:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.2f", bulletspeed);
    ImGui::SliderFloat(xorstr_("##55"), &bulletspeed, -10.58f, 5.80f, "##");
    ImGui::SameLine();
    ImGui::Text("%s", xorstr_("Default is 0.08"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text("%s", xorstr_("Smooth Preditcion Gravity:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.2f", bulletgrav);
    ImGui::SliderFloat(xorstr_("##57"), &bulletgrav, -10.55f, 5.90f, "##");
    ImGui::SameLine();
    ImGui::Text("%s", xorstr_("Default is 0.05"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text("%s", xorstr_("Max Headshot Distance:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%d meters",
                       (int)(g_settings.aimbot_settings.headshot_dist / 40.0f));
    ImGui::SliderFloat(xorstr_("##headshot_dist"),
                       &g_settings.aimbot_settings.headshot_dist, 0.0f,
                       g_settings.aimbot_settings.aim_dist, "##");
    ImGui::Text("%s", xorstr_("Disable sniper headshots when out of range"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text("%s", xorstr_("Aiming Bone:"));
    ImGui::Checkbox(xorstr_("Auto"), &g_settings.aimbot_settings.bone_auto);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Nearest"),
                    &g_settings.aimbot_settings.bone_nearest);
    ImGui::Text("%s", xorstr_("0=Head, 1=Neck, 2=Chest, 3=Stomach"));
    ImGui::SliderInt(xorstr_("##bone"), &g_settings.aimbot_settings.bone, 0, 3);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("ESP Options:"));
    ImGui::Checkbox(xorstr_("Box"), &g_settings.esp_visuals.box);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Line"), &g_settings.esp_visuals.line);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Distance"), &g_settings.esp_visuals.distance);
    ImGui::Checkbox(xorstr_("Health bar"), &g_settings.esp_visuals.healthbar);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Shield bar"), &g_settings.esp_visuals.shieldbar);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Name"), &g_settings.esp_visuals.name);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Damage"), &g_settings.esp_visuals.damage);
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(xorstr_("Show aimbot target"), &g_settings.show_aim_target);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Player Glow Visable:"));
    float glowcolorviz[] = {g_settings.glow_r_viz, g_settings.glow_g_viz,
                            g_settings.glow_b_viz};
    ImGui::ColorEdit3(xorstr_("##Glow Color Picker Visable"), glowcolorviz);
    {
      g_settings.glow_r_viz = glowcolorviz[0];
      g_settings.glow_g_viz = glowcolorviz[1];
      g_settings.glow_b_viz = glowcolorviz[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Player Glow Not Visable:"));
    float glowcolornot[] = {g_settings.glow_r_not, g_settings.glow_g_not,
                            g_settings.glow_b_not};
    ImGui::ColorEdit3(xorstr_("##Glow Color Not Visable"), glowcolornot);
    {
      g_settings.glow_r_not = glowcolornot[0];
      g_settings.glow_g_not = glowcolornot[1];
      g_settings.glow_b_not = glowcolornot[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Player Glow Knocked:"));
    float glowcolorknocked[] = {g_settings.glow_r_knocked,
                                g_settings.glow_g_knocked,
                                g_settings.glow_b_knocked};
    ImGui::ColorEdit3(xorstr_("##Glow Color Knocked"), glowcolorknocked);
    {
      g_settings.glow_r_knocked = glowcolorknocked[0];
      g_settings.glow_g_knocked = glowcolorknocked[1];
      g_settings.glow_b_knocked = glowcolorknocked[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(
        "%s",
        xorstr_("Saving and Loading. Need to Save Once to make the file."));
    // Saving
    if (ImGui::Button(xorstr_("Save Config"))) {
      save_settings();
      tui_menu_forceupdate();
    }
    ImGui::SameLine();
    // Loading
    if (ImGui::Button(xorstr_("Load Config"))) {
      load_settings();
      tui_menu_forceupdate();
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu1 == 1) {
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Weapone Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Item Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID(xorstr_("Radar Settings")),
                                       0);
    }
  }
  if (ImGui::CollapsingHeader(xorstr_("Radar Settings"))) {
    menu2 = 1;
    // Dot Size for both mini and main map
    ImGui::Text("%s", xorstr_("MiniMap Radar Dot Size"));
    ImGui::SliderInt(xorstr_("MiniMap Dot Size"),
                     &g_settings.mini_map_radar_dot_size1, 1, 10);
    ImGui::SliderInt(xorstr_("MiniMap Outer Ring Thickness"),
                     &g_settings.mini_map_radar_dot_size2, 1, 10);
    ImGui::Text("%s", xorstr_("Main Map Radar Dot Size"));
    ImGui::SliderInt(xorstr_("Main Map Dot Width"),
                     &g_settings.main_map_radar_dot_size1, 1, 10);
    ImGui::SliderInt(xorstr_("Main Map Dot length"),
                     &g_settings.main_map_radar_dot_size2, 1, 10);
    /*//Radar Color
    ImGui::Text("%s", xorstr_("Radar Color Picker:"));
    ImGui::ColorEdit3(xorstr_("##Radar Color Picker"), radarcolor);
    {
            radarcolorr = radarcolor[0];
            radarcolorg = radarcolor[1];
            radarcolorb = radarcolor[2];
    }
    */
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu2 == 1) {
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Weapone Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Item Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Main Toggle Settings")), 0);
    }
  }
  if (ImGui::CollapsingHeader(xorstr_("Item Filter Settings"))) {
    menu3 = 1;
    ImGui::Text("%s", xorstr_("Ammo"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(xorstr_("Sniper Ammo"), &g_settings.loot.sniperammo);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Heavy Ammo"), &g_settings.loot.heavyammo);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Light Ammo"), &g_settings.loot.lightammo);
    ImGui::Checkbox(xorstr_("Energy Ammo"), &g_settings.loot.energyammo);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Shotgun Ammo"), &g_settings.loot.shotgunammo);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Ammo Mags"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(xorstr_("Sniper lv3"), &g_settings.loot.sniperammomag3);
    ImGui::Checkbox(xorstr_("Sniper lv4"), &g_settings.loot.sniperammomag4);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Heavy lv3"), &g_settings.loot.heavyammomag3);
    ImGui::Checkbox(xorstr_("Heavy lv4"), &g_settings.loot.heavyammomag4);
    ImGui::Checkbox(xorstr_("Light lv3"), &g_settings.loot.lightammomag3);
    ImGui::Checkbox(xorstr_("Light lv4"), &g_settings.loot.lightammomag4);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Energy lv3"), &g_settings.loot.energyammomag3);
    ImGui::Checkbox(xorstr_("Energy lv4"), &g_settings.loot.energyammomag4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("HCOGs"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(xorstr_("1x HCOG"), &g_settings.loot.optic1xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("2x HCOG"), &g_settings.loot.optic2xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("3x HCOG"), &g_settings.loot.optic3xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("2-4x"), &g_settings.loot.optic2x4x);
    ImGui::Text("%s", xorstr_("Snipers"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(xorstr_("Sniper 6x"), &g_settings.loot.opticsniper6x);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Sniper 4-8x"), &g_settings.loot.opticsniper4x8x);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Sniper Threat"),
                    &g_settings.loot.opticsniperthreat);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Holo's"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(xorstr_("1x Holo"), &g_settings.loot.opticholo1x);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("2x Holo"), &g_settings.loot.opticholo1x2x);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("1x Threat"), &g_settings.loot.opticthreat);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Backpacks"));
    ImGui::Checkbox(xorstr_("Light Backpack"), &g_settings.loot.lightbackpack);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Medium Backpack"), &g_settings.loot.medbackpack);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Heavy Backpack"), &g_settings.loot.heavybackpack);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Shield's"));
    ImGui::Checkbox(xorstr_("Armor blue"), &g_settings.loot.shieldupgrade2);
    ImGui::Checkbox(xorstr_("Armor purple"), &g_settings.loot.shieldupgrade3);
    ImGui::Checkbox(xorstr_("Armor gold"), &g_settings.loot.shieldupgrade4);
    ImGui::Checkbox(xorstr_("Armor red"), &g_settings.loot.shieldupgrade5);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Helmets blue"),
                    &g_settings.loot.shieldupgradehead2);
    ImGui::Checkbox(xorstr_("Helmets purple"),
                    &g_settings.loot.shieldupgradehead3);
    ImGui::Checkbox(xorstr_("Helmets gold"),
                    &g_settings.loot.shieldupgradehead4);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Knockdown blue"), &g_settings.loot.shielddown2);
    ImGui::Checkbox(xorstr_("Knockdown purple"), &g_settings.loot.shielddown3);
    ImGui::Checkbox(xorstr_("Knockdown gold"), &g_settings.loot.shielddown4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(GREEN, "%s", xorstr_("Heals for Health"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("Large Health"), &g_settings.loot.healthlarge);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Small Health"), &g_settings.loot.healthsmall);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Phoenix Kit"), &g_settings.loot.phoenix);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(BLUE, "%s", xorstr_("Heals for Shields"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("Large Shield"), &g_settings.loot.shieldbattlarge);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Small Shield"), &g_settings.loot.shieldbattsmall);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Ultimate Accelerant"),
                    &g_settings.loot.accelerant);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Attachements"));
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(xorstr_("Lasersight2"), &g_settings.loot.lasersight2);
    ImGui::Checkbox(xorstr_("Lasersight3"), &g_settings.loot.lasersight3);
    ImGui::Checkbox(xorstr_("Lasersight4"), &g_settings.loot.lasersight4);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Sniper Stock 2"), &g_settings.loot.stocksniper2);
    ImGui::Checkbox(xorstr_("Sniper Stock 3"), &g_settings.loot.stocksniper3);
    ImGui::Checkbox(xorstr_("Sniper Stock 4"), &g_settings.loot.stocksniper4);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Regular Stock 2"), &g_settings.loot.stockregular2);
    ImGui::Checkbox(xorstr_("Regular Stock 3"), &g_settings.loot.stockregular3);
    ImGui::Checkbox(xorstr_("Suppressor 1"), &g_settings.loot.suppressor1);
    ImGui::Checkbox(xorstr_("Suppressor 2"), &g_settings.loot.suppressor2);
    ImGui::Checkbox(xorstr_("Suppressor 3"), &g_settings.loot.suppressor3);
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Text("%s", xorstr_("Weapon Mods"));
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(xorstr_("Turbo Charger"), &g_settings.loot.turbo_charger);
    ImGui::Checkbox(xorstr_("Skull Piecer"), &g_settings.loot.skull_piecer);
    ImGui::Checkbox(xorstr_("Hammer Point"), &g_settings.loot.hammer_point);
    ImGui::Checkbox(xorstr_("Disruptor Rounds"),
                    &g_settings.loot.disruptor_rounds);
    ImGui::Checkbox(xorstr_("Boosted Loader"), &g_settings.loot.boosted_loader);
    ImGui::Checkbox(xorstr_("Shotgunbolt 1"), &g_settings.loot.shotgunbolt1);
    ImGui::Checkbox(xorstr_("Shotgunbolt 2"), &g_settings.loot.shotgunbolt2);
    ImGui::Checkbox(xorstr_("Shotgunbolt 3"), &g_settings.loot.shotgunbolt3);
    ImGui::Checkbox(xorstr_("Shotgunbolt 4"), &g_settings.loot.shotgunbolt4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    {
      ImGui::Text("%s", xorstr_("Loot Glow Filled:"));
      int lootfilled_value = g_settings.loot_filled;
      ImGui::SliderInt(xorstr_("##lootfilled"), &lootfilled_value, 0, 14, "%d");
      g_settings.loot_filled = lootfilled_value;
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu3 == 1) {
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Weapone Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID(xorstr_("Radar Settings")),
                                       0);
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Main Toggle Settings")), 0);
    }
  }
  if (ImGui::CollapsingHeader(xorstr_("Weapone Filter Settings"))) {
    menu4 = 1;
    // Light Weapons
    ImGui::TextColored(ORANGE, "%s", xorstr_("Light Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("P2020"), &g_settings.loot.weapon_p2020);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("RE-45"), &g_settings.loot.weapon_re45);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("R-99"), &g_settings.loot.weapon_r99);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("G7 Scout"), &g_settings.loot.weapon_g7_scout);
    ImGui::Checkbox(xorstr_("Spitfire"), &g_settings.loot.weapon_spitfire);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("R-301"), &g_settings.loot.weapon_r301);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Alternator "), &g_settings.loot.weapon_alternator);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    // Heavy Weapons
    ImGui::TextColored(TEAL, "%s", xorstr_("Heavy Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("C.A.R."), &g_settings.loot.weapon_car_smg);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Flatline"), &g_settings.loot.weapon_flatline);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Hemlok"), &g_settings.loot.weapon_hemlock);
    ImGui::Checkbox(xorstr_("Prowler "), &g_settings.loot.weapon_prowler);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("30-30"), &g_settings.loot.weapon_3030_repeater);
    ImGui::Checkbox(xorstr_("Rampage"), &g_settings.loot.weapon_rampage);
    // Energy Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(YELLOW, "%s", xorstr_("Energy Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("Triple Take"),
                    &g_settings.loot.weapon_triple_take);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("L-STAR"), &g_settings.loot.weapon_lstar);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Volt"), &g_settings.loot.weapon_volt);
    ImGui::Checkbox(xorstr_("Devotion "), &g_settings.loot.weapon_devotion);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("HAVOC"), &g_settings.loot.weapon_havoc);
    ImGui::Checkbox(xorstr_("Nemesis"), &g_settings.loot.weapon_nemesis);

    // Shotgun Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(RED, "%s", xorstr_("Shotgun Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("EVA-8"), &g_settings.loot.weapon_eva8);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Mozambique"), &g_settings.loot.weapon_mozambique);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Peacekeeper"),
                    &g_settings.loot.weapon_peacekeeper);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Mastiff"), &g_settings.loot.weapon_mastiff);
    // Sniper Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(BLUE, "%s", xorstr_("Sniper Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("Charge Rifle"),
                    &g_settings.loot.weapon_charge_rifle);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Longbow"), &g_settings.loot.weapon_longbow);
    ImGui::SameLine();
    ImGui::Checkbox(xorstr_("Sentinel"), &g_settings.loot.weapon_sentinel);
    ImGui::Checkbox(xorstr_("Wingman "), &g_settings.loot.weapon_wingman);
    // KRABER
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text("%s", xorstr_("Special Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(xorstr_("Kraber .50-Cal Sniper"),
                    &g_settings.loot.weapon_kraber);
    ImGui::Checkbox(xorstr_("Bocek Bow"), &g_settings.loot.weapon_bow);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu4 == 1) {
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Item Filter Settings")), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID(xorstr_("Radar Settings")),
                                       0);
      ImGui::GetStateStorage()->SetInt(
          ImGui::GetID(xorstr_("Main Toggle Settings")), 0);
    }
  }
  // ImGui::EndTabItem();

  //}

  // ImGui::EndTabBar();
  //}
  ImGui::Dummy(ImVec2(0.0f, 10.0f));
  ImGui::Text("%s%d%s%d", xorstr_("held="), aimbot_get_held_id(),
              xorstr_(", weapon="), aimbot_get_weapon_id());
  ImGui::Dummy(ImVec2(0.0f, 5.0f));
  ImGui::Text(xorstr_("Overlay FPS: %.3f ms/frame (%.1f FPS)"),
              1000.0f / ImGui::GetIO().Framerate, ImGui::GetIO().Framerate);
  ImGui::Dummy(ImVec2(0.0f, 5.0f));

  ImGui::Text("%s", xorstr_("Game FPS for Aim Prediction:"));
  ImGui::SameLine();
  ImGui::Checkbox(xorstr_("Calc Game FPS"), &g_settings.calc_game_fps);
  ImGui::SliderFloat(xorstr_("##gamefps"), &g_settings.game_fps, 1.0f, 300.0f,
                     "%.1f");

  ImGui::Dummy(ImVec2(0.0f, 5.0f));
  ImGui::Text("%s", xorstr_("external-overlay test build"));
  ImGui::End();

  update_settings(g_settings);
}

void Overlay::RenderInfo() {
  ImGui::SetNextWindowPos(ImVec2(0, 0));
  ImGui::SetNextWindowSize(ImVec2(280, 30));
  ImGui::SetNextWindowBgAlpha(0.6);
  ImGui::Begin(xorstr_("##info"), (bool *)true,
               ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize |
                   ImGuiWindowFlags_NoScrollbar);

  float aimbot_fov = aimbot_get_max_fov();
  bool aimbot_locked = aimbot_is_locked();
  bool aimbot_gun_safety = aimbot_get_gun_safety();
  bool aimbot_grenade = aimbot_is_grenade();

  DrawLine(ImVec2(1, 2), ImVec2(280, 2), RED, 2);
  if (g_spectators == 0) {
    ImGui::TextColored(GREEN, "%zu", g_spectators);
  } else {
    ImGui::TextColored(RED, "%zu", g_spectators);
  }
  ImGui::SameLine();
  ImGui::Text("%s", xorstr_("--"));
  ImGui::SameLine();
  ImGui::TextColored(GREEN, "%zu", g_allied_spectators);
  ImGui::SameLine();
  ImGui::Text("%s", xorstr_("--"));
  ImGui::SameLine();
  ImGui::TextColored(WHITE, "%.f", aimbot_fov);
  ImGui::SameLine();
  ImGui::Text("%s", xorstr_("--"));
  ImGui::SameLine();
  // Aim is on = 2, On but No Vis Check = 1, Off = 0
  const auto g_settings = global_settings();
  if (aimbot_locked) {
    ImGui::TextColored(aimbot_gun_safety ? GREEN : ORANGE, "%s",
                       xorstr_("[TARGET LOCK!]"));
  } else if (aimbot_grenade) {
    ImGui::TextColored(BLUE, "%s", xorstr_("Skynade On"));
  } else if (g_settings.aimbot_settings.aim_mode == 2) {
    ImGui::TextColored(GREEN, "%s", xorstr_("Aim On"));

  } else if (g_settings.aimbot_settings.aim_mode == 0) {
    ImGui::TextColored(RED, "%s", xorstr_("Aim Off"));
  } else {
    ImGui::TextColored(RED, "%s%d", xorstr_("Aim On "),
                       g_settings.aimbot_settings.aim_mode);
  }
  ImGui::SameLine();
  DrawLine(ImVec2(1, 28), ImVec2(280, 28), RED, 2);
  ImGui::End();
}

static void glfw_error_callback(int error, const char *description) {
  fprintf(stderr, "GLFW Error %d: %s\n", error, description);
}

int Overlay::CreateOverlay() {
  const auto g_settings = global_settings();
  glfwSetErrorCallback(glfw_error_callback);
  if (!glfwInit())
    return 1;

    // Decide GL+GLSL versions
#if defined(IMGUI_IMPL_OPENGL_ES2)
  // GL ES 2.0 + GLSL 100
  const char *glsl_version = "#version 100";
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 2);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);
  glfwWindowHint(GLFW_CLIENT_API, GLFW_OPENGL_ES_API);
#elif defined(__APPLE__)
  // GL 3.2 + GLSL 150
  const char *glsl_version = "#version 150";
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE); // 3.2+ only
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);           // Required on Mac
#else
  // GL 3.0 + GLSL 130
  const char *glsl_version = "#version 130";
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 0);
  // glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);  // 3.2+
  // only glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE); // 3.0+ only
#endif
  glfwWindowHint(GLFW_DECORATED, GLFW_FALSE);
  glfwWindowHint(GLFW_FLOATING, GLFW_TRUE);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);
  glfwWindowHint(GLFW_FOCUS_ON_SHOW, GLFW_FALSE);
  glfwWindowHint(GLFW_TRANSPARENT_FRAMEBUFFER, GLFW_TRUE);
  glfwWindowHint(GLFW_MOUSE_PASSTHROUGH, GLFW_TRUE);
  glfwWindowHint(GLFW_AUTO_ICONIFY, GLFW_FALSE);

  // Create window with graphics context
  GLFWwindow *window = glfwCreateWindow(
      g_settings.screen_width, g_settings.screen_height,
      xorstr_("Client ImGui GLFW+OpenGL3"), glfwGetPrimaryMonitor(), nullptr);
  if (window == nullptr)
    return 1;
  // static const char *GamescopeOverlayProperty = "GAMESCOPE_EXTERNAL_OVERLAY";
  // Display *x11_display = glfwGetX11Display();
  // Window x11_window = glfwGetX11Window(window);
  // if (x11_window && x11_display) {
  //   // Set atom for gamescope to render as an overlay.
  //   Atom overlay_atom =
  //       XInternAtom(x11_display, GamescopeOverlayProperty, False);
  //   uint32_t value = 1;
  //   XChangeProperty(x11_display, x11_window, overlay_atom, XA_CARDINAL, 32,
  //                   PropertyNewValue, (unsigned char *)&value, 1);
  // }
  glfwMakeContextCurrent(window);

  glfwSwapInterval(1); // Enable vsync

  // Setup Dear ImGui context
  IMGUI_CHECKVERSION();
  ImGui::CreateContext();
  ImGuiIO &io = ImGui::GetIO();
  (void)io;
  io.ConfigFlags |=
      ImGuiConfigFlags_NavEnableKeyboard; // Enable Keyboard Controls
  io.ConfigFlags |=
      ImGuiConfigFlags_NavEnableGamepad; // Enable Gamepad Controls

#include "font.h"
  io.Fonts->AddFontFromMemoryCompressedTTF(
      LXGWNeoXiHei_compressed_data, LXGWNeoXiHei_compressed_size, 15, NULL,
      io.Fonts->GetGlyphRangesChineseFull());
  io.Fonts->Build();

  // Setup Dear ImGui style
  ImGui::StyleColorsDark();
  // ImGui::StyleColorsLight();

  // Setup Platform/Renderer backends
  ImGui_ImplGlfw_InitForOpenGL(window, true);
  ImGui_ImplOpenGL3_Init(glsl_version);

  // Our state
  bool show_demo_window = false;
  bool show_another_window = false;
  running = true;

  // Main loop
  while (!glfwWindowShouldClose(window)) {
    if (!running || !overlay_t) {
      break;
    }
    // Poll and handle events (inputs, window resize, etc.)
    // You can read the io.WantCaptureMouse, io.WantCaptureKeyboard flags to
    // tell if dear imgui wants to use your inputs.
    // - When io.WantCaptureMouse is true, do not dispatch mouse input data to
    // your main application, or clear/overwrite your copy of the mouse data.
    // - When io.WantCaptureKeyboard is true, do not dispatch keyboard input
    // data to your main application, or clear/overwrite your copy of the
    // keyboard data. Generally you may always pass all inputs to dear imgui,
    // and hide them from your application based on those two flags.
    glfwPollEvents();

    // Start the Dear ImGui frame
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();

    // 1. Show the big demo window (Most of the sample code is in
    // ImGui::ShowDemoWindow()! You can browse its code to learn more about Dear
    // ImGui!).
    if (show_demo_window)
      ImGui::ShowDemoWindow(&show_demo_window);

    // 2. Show a simple window that we create ourselves. We use a Begin/End pair
    // to create a named window.
    {
      ImGui::SetNextWindowBgAlpha(0.4);
      ImGui::SetNextWindowPos(
          ImVec2(this->getWidth() * 0.8f, this->getHeight() / 3.0f),
          ImGuiCond_FirstUseEver);
      ImGui::Begin(
          xorstr_("Hello, world!"), NULL,
          ImGuiWindowFlags_AlwaysAutoResize); // Create a window called "Hello,
                                              // world!" and append into it.

      ImGui::Checkbox(
          xorstr_("Demo Window"),
          &show_demo_window); // Edit bools storing our window open/close state

      ImGui::Text(xorstr_("Overlay(%.1f FPS)"), io.Framerate);
      if (g_settings.calc_game_fps) {
        ImGui::SameLine();
        ImGui::Text(xorstr_(" Game(%.1f FPS)"), global_settings().game_fps);
      }
      ImGui::Dummy(ImVec2(0.0f, 5.0f));
      if (esp_teammates_damage.size() > 0) {
        const char *info[esp_teammates_damage.size()];
        for (size_t i = 0; i < esp_teammates_damage.size(); i++) {
          info[i] = esp_teammates_damage[i].c_str();
        }
        int current_item = 0;
        ImGui::ListBox(xorstr_("Damage"), &current_item, info,
                       esp_teammates_damage.size());
      }

      ImGui::Dummy(ImVec2(0.0f, 5.0f));
      if (esp_spec_names.size() > 0) {
        const char *names[esp_spec_names.size()];
        for (size_t i = 0; i < esp_spec_names.size(); i++) {
          names[i] = esp_spec_names[i].c_str();
        }
        int current_item = 0;
        ImGui::ListBox(xorstr_("Spectators"), &current_item, names,
                       esp_spec_names.size());
      } else {
        ImGui::Text("%s", xorstr_("No Spectators"));
      }

      ImGui::End();
    }

    // 3. Show another simple window.
    if (show_another_window) {
      ImGui::Begin(
          xorstr_("Another Window"),
          &show_another_window); // Pass a pointer to our bool variable (the
                                 // window will have a closing button that will
                                 // clear the bool when clicked)
      ImGui::Text("%s", xorstr_("Hello from another window!"));
      if (ImGui::Button(xorstr_("Close Me")))
        show_another_window = false;
      ImGui::End();
    }

    // Draw Main GUI
    // if (IsKeyDown(ImGuiKey_MouseLeft) && !k_leftclick) {
    //   io.MouseDown[0] = true;
    //   k_leftclick = true;
    // } else if (!IsKeyDown(ImGuiKey_MouseLeft) && k_leftclick) {
    //   io.MouseDown[0] = false;
    //   k_leftclick = false;
    // }
    {
      static bool k_ins = false;
      static std::chrono::milliseconds last_press =
          std::chrono::milliseconds(0);
      std::chrono::milliseconds now_ms =
          duration_cast<std::chrono::milliseconds>(
              std::chrono::system_clock::now().time_since_epoch());
      if (IsKeyDown(ImGuiKey_Insert)) {
        last_press = now_ms;
      }
      bool key_insert_pressed_ui = (now_ms - last_press).count() < 400;
      bool key_insert_pressed_game = isPressed(72);
      bool key_insert_pressed =
          key_insert_pressed_ui || key_insert_pressed_game;
      if (key_insert_pressed && !k_ins) {
        k_ins = true;
        glfwSetWindowAttrib(window, GLFW_MOUSE_PASSTHROUGH, GLFW_FALSE);
      } else if (!key_insert_pressed && k_ins) {
        k_ins = false;
        show_menu = !show_menu;
        glfwSetWindowAttrib(window, GLFW_MOUSE_PASSTHROUGH,
                            show_menu ? GLFW_FALSE : GLFW_TRUE);
      }
    }

    // Main Map Radar, Needs Manual Setting of cords
    {
      bool key_m_pressed = IsKeyDown(ImGuiKey_M) || isPressed(23);
      if (key_m_pressed && mainradartoggle == 0) {
        mainradartoggle = 1;
      } else if (!key_m_pressed && mainradartoggle == 1) {
        mainradartoggle = 0;
      }
    }

    if (show_menu) {
      RenderMenu();
    } else {
      RenderInfo();
    }
    RenderEsp();

    // Rendering
    ImGui::Render();
    glEnable(GL_DEPTH_TEST);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    int display_w, display_h;
    glfwGetFramebufferSize(window, &display_w, &display_h);
    glViewport(0, 0, display_w, display_h);
    width = display_w;
    height = display_h;
    glClearColor(0, 0, 0, 0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

    glfwSwapBuffers(window);
  }

  // Cleanup
  ImGui_ImplOpenGL3_Shutdown();
  ImGui_ImplGlfw_Shutdown();
  ImGui::DestroyContext();

  glfwDestroyWindow(window);
  glfwTerminate();

  return 0;
}

std::thread Overlay::Start() {
  std::thread overlay_thr = std::thread(StaticMessageStart, ref(*this));
  return overlay_thr;
}

void Overlay::Clear() {
  running = false;
  // printf("running=%b\n", running);
}

int Overlay::getWidth() { return width; }

int Overlay::getHeight() { return height; }

void Overlay::DrawLine(ImVec2 a, ImVec2 b, ImColor color, float width) {
  ImGui::GetWindowDrawList()->AddLine(a, b, color, width);
}

void Overlay::DrawBox(ImColor color, float x, float y, float w, float h,
                      float line_w) {
  DrawLine(ImVec2(x, y), ImVec2(x + w, y), color, line_w);
  DrawLine(ImVec2(x, y), ImVec2(x, y + h), color, line_w);
  DrawLine(ImVec2(x + w, y), ImVec2(x + w, y + h), color, line_w);
  DrawLine(ImVec2(x, y + h), ImVec2(x + w, y + h), color, line_w);
}

void Overlay::Text(ImVec2 pos, ImColor color, const char *text_begin,
                   const char *text_end, float wrap_width,
                   const ImVec4 *cpu_fine_clip_rect) {
  ImGui::GetWindowDrawList()->AddText(ImGui::GetFont(), ImGui::GetFontSize(),
                                      pos, color, text_begin, text_end,
                                      wrap_width, cpu_fine_clip_rect);
}

void Overlay::String(ImVec2 pos, ImColor color, const char *text) {
  Text(pos, color, text, text + strlen(text), 200, 0);
}

void Overlay::RectFilled(float x0, float y0, float x1, float y1, ImColor color,
                         float rounding, int rounding_corners_flags) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x0, y0), ImVec2(x1, y1), color, rounding, rounding_corners_flags);
}

void Overlay::ProgressBar(float x, float y, float w, float h, int value,
                          int v_max) {
  ImColor barColor = ImColor(min(510 * (v_max - value) / 100, 255),
                             min(510 * value / 100, 255), 25, 255);

  RectFilled(x, y, x + w, y + ((h / float(v_max)) * (float)value), barColor,
             0.0f, 0);
}

// Seer Hp and Shield bars (never re fixed the armor type so its set to max
// shield)

void DrawQuadFilled(ImVec2 p1, ImVec2 p2, ImVec2 p3, ImVec2 p4, ImColor color) {
  ImGui::GetWindowDrawList()->AddQuadFilled(p1, p2, p3, p4, color);
}
// void DrawHexagon(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const
// ImVec2& p4, const ImVec2& p5, const ImVec2& p6, ImU32 col, float thickness)
// {
// 	ImGui::GetWindowDrawList()->AddHexagon(p1, p2, p3, p4, p5, p6, col,
// thickness);
// }
void DrawHexagonFilled(const ImVec2 &p1, const ImVec2 &p2, const ImVec2 &p3,
                       const ImVec2 &p4, const ImVec2 &p5, const ImVec2 &p6,
                       ImU32 col) {
  // ImGui::GetWindowDrawList()->AddHexagonFilled(p1, p2, p3, p4, p5, p6, col);
  const ImVec2 points[]{p1, p2, p3, p4, p5, p6};
  ImGui::GetWindowDrawList()->AddConvexPolyFilled(points, 6, col);
}

void Overlay::DrawSeerLikeHealth(float x, float y, int shield, int max_shield,
                                 int armorType, int health) {
  // printf("seer(%f,%f), %d/%d, %d, %d\n", x, y, shield, max_shield, armorType,
  // health);

  int bg_offset = 3;
  int bar_width = 158;
  // 4steps...2*3=6
  // 38*4=152 152+6 = 158
  // 5steps...2*4=8
  // 30*5=150 150+8 = 158
  float max_health = 100.0f;
  float shield_step = 25.0f;

  int shield_25 = 30;
  // steps = 5;

  ImVec2 bg1(x - bar_width / 2.0f - bg_offset, y);
  ImVec2 bg2(bg1.x - 10, bg1.y - 16);
  ImVec2 bg3(bg2.x + 5, bg2.y - 7);
  ImVec2 bg4(bg3.x + bar_width + bg_offset, bg3.y);
  ImVec2 bg5(bg4.x + 11, bg4.y + 18);
  ImVec2 bg6(x + bar_width / 2.0f + bg_offset, y);
  DrawHexagonFilled(bg1, bg2, bg3, bg4, bg5, bg6, ImColor(0, 0, 0, 120));

  ImVec2 h1(bg1.x + 3, bg1.y - 4);
  ImVec2 h2(h1.x - 5, h1.y - 8);
  ImVec2 h3(h2.x + (float)health / max_health * bar_width, h2.y);
  ImVec2 h4(h1.x + (float)health / max_health * bar_width, h1.y);
  ImVec2 h3m(h2.x + bar_width, h2.y);
  ImVec2 h4m(h1.x + bar_width, h1.y);
  DrawQuadFilled(h1, h2, h3m, h4m, ImColor(10, 10, 30, 60));
  DrawQuadFilled(h1, h2, h3, h4, WHITE);

  ImColor shieldCracked(97, 97, 97);
  // ImColor shieldCrackedDark(67, 67, 67);

  ImColor shieldCol;
  ImColor shieldColDark;  // not used, but the real seer q has shadow inside
  if (max_shield == 50) { // white
    shieldCol = ImColor(247, 247, 247);
    shieldColDark = ImColor(164, 164, 164);
  } else if (max_shield == 75) { // blue
    shieldCol = ImColor(39, 178, 255);
    shieldColDark = ImColor(27, 120, 210);
  } else if (max_shield == 100) { // purple
    shieldCol = ImColor(206, 59, 255);
    shieldColDark = ImColor(136, 36, 220);
  } else if (max_shield == 100) { // gold
    shieldCol = ImColor(255, 255, 79);
    shieldColDark = ImColor(218, 175, 49);
  } else if (max_shield == 125) { // red
    shieldCol = ImColor(219, 2, 2);
    shieldColDark = ImColor(219, 2, 2);
  } else {
    shieldCol = ImColor(247, 247, 247);
    shieldColDark = ImColor(164, 164, 164);
  }
  int shield_tmp = shield;
  int shield1 = 0;
  int shield2 = 0;
  int shield3 = 0;
  int shield4 = 0;
  int shield5 = 0;
  if (shield_tmp > 25) {
    shield1 = 25;
    shield_tmp -= 25;
    if (shield_tmp > 25) {
      shield2 = 25;
      shield_tmp -= 25;
      if (shield_tmp > 25) {
        shield3 = 25;
        shield_tmp -= 25;
        if (shield_tmp > 25) {
          shield4 = 25;
          shield_tmp -= 25;
          shield5 = shield_tmp;
        } else {
          shield4 = shield_tmp;
        }
      } else {
        shield3 = shield_tmp;
      }
    } else {
      shield2 = shield_tmp;
    }
  } else {
    shield1 = shield_tmp;
  }
  ImVec2 s1(h2.x - 1, h2.y - 2);
  ImVec2 s2(s1.x - 3, s1.y - 5);
  ImVec2 s3(s2.x + shield1 / shield_step * shield_25, s2.y);
  ImVec2 s4(s1.x + shield1 / shield_step * shield_25, s1.y);
  ImVec2 s3m(s2.x + shield_25, s2.y);
  ImVec2 s4m(s1.x + shield_25, s1.y);

  ImVec2 ss1(s4m.x + 2, s1.y);
  ImVec2 ss2(s3m.x + 2, s2.y);
  ImVec2 ss3(ss2.x + shield2 / shield_step * shield_25, s2.y);
  ImVec2 ss4(ss1.x + shield2 / shield_step * shield_25, s1.y);
  ImVec2 ss3m(ss2.x + shield_25, s2.y);
  ImVec2 ss4m(ss1.x + shield_25, s1.y);

  ImVec2 sss1(ss4m.x + 2, s1.y);
  ImVec2 sss2(ss3m.x + 2, s2.y);
  ImVec2 sss3(sss2.x + shield3 / shield_step * shield_25, s2.y);
  ImVec2 sss4(sss1.x + shield3 / shield_step * shield_25, s1.y);
  ImVec2 sss3m(sss2.x + shield_25, s2.y);
  ImVec2 sss4m(sss1.x + shield_25, s1.y);

  ImVec2 ssss1(sss4m.x + 2, s1.y);
  ImVec2 ssss2(sss3m.x + 2, s2.y);
  ImVec2 ssss3(ssss2.x + shield4 / shield_step * shield_25, s2.y);
  ImVec2 ssss4(ssss1.x + shield4 / shield_step * shield_25, s1.y);
  ImVec2 ssss3m(ssss2.x + shield_25, s2.y);
  ImVec2 ssss4m(ssss1.x + shield_25, s1.y);

  ImVec2 sssss1(ssss4m.x + 2, s1.y);
  ImVec2 sssss2(ssss3m.x + 2, s2.y);
  ImVec2 sssss3(sssss2.x + shield5 / shield_step * shield_25, s2.y);
  ImVec2 sssss4(sssss1.x + shield5 / shield_step * shield_25, s1.y);
  ImVec2 sssss3m(sssss2.x + shield_25, s2.y);
  ImVec2 sssss4m(sssss1.x + shield_25, s1.y);
  if (max_shield == 50) {
    if (shield <= 25) {
      if (shield < 25) {
        DrawQuadFilled(s1, s2, s3m, s4m, shieldCracked);
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(s1, s2, s3, s4, shieldCol);

    } else if (shield <= 50) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      if (shield != 50) {
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
    }
  } else if (max_shield == 75) {
    if (shield <= 25) {
      if (shield < 25) {
        DrawQuadFilled(s1, s2, s3m, s4m, shieldCracked);
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(s1, s2, s3, s4, shieldCol);

    } else if (shield <= 50) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      if (shield < 50) {
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
    } else if (shield <= 75) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      if (shield < 75) {
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
    }
  } else if (max_shield == 100) {
    if (shield <= 25) {
      if (shield < 25) {
        DrawQuadFilled(s1, s2, s3m, s4m, shieldCracked);
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(s1, s2, s3, s4, shieldCol);

    } else if (shield <= 50) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      if (shield < 50) {
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
    } else if (shield <= 75) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      if (shield < 75) {
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
    } else if (shield <= 100) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
      if (shield < 100) {
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ssss1, ssss2, ssss3, ssss4, shieldCol);
    }
  } else if (max_shield == 125) {
    if (shield <= 25) {
      if (shield < 25) {
        DrawQuadFilled(s1, s2, s3m, s4m, shieldCracked);
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
        DrawQuadFilled(sssss1, sssss2, sssss3m, sssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(s1, s2, s3, s4, shieldCol);

    } else if (shield <= 50) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      if (shield < 50) {
        DrawQuadFilled(ss1, ss2, ss3m, ss4m, shieldCracked);
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
        DrawQuadFilled(sssss1, sssss2, sssss3m, sssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
    } else if (shield <= 75) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      if (shield < 75) {
        DrawQuadFilled(sss1, sss2, sss3m, sss4m, shieldCracked);
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
        DrawQuadFilled(sssss1, sssss2, sssss3m, sssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
    } else if (shield <= 100) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
      if (shield < 100) {
        DrawQuadFilled(ssss1, ssss2, ssss3m, ssss4m, shieldCracked);
        DrawQuadFilled(sssss1, sssss2, sssss3m, sssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(ssss1, ssss2, ssss3, ssss4, shieldCol);
    } else if (shield <= 125) {
      DrawQuadFilled(s1, s2, s3, s4, shieldCol);
      DrawQuadFilled(ss1, ss2, ss3, ss4, shieldCol);
      DrawQuadFilled(sss1, sss2, sss3, sss4, shieldCol);
      DrawQuadFilled(ssss1, ssss2, ssss3, ssss4, shieldCol);
      if (shield < 125) {
        DrawQuadFilled(sssss1, sssss2, sssss3m, sssss4m, shieldCracked);
      }
      if (shield != 0)
        DrawQuadFilled(sssss1, sssss2, sssss3, sssss4, shieldCol);
    }
  }
}
