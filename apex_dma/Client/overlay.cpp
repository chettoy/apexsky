#include "overlay.h"

#include "imgui.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <stdio.h>
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
#include <thread>

using namespace std;

extern void saveSettings();
extern void loadSettings();

extern float veltest;
extern bool NoNadeAim;
extern bool firing_range;
extern int aim;
extern int local_held_id;
extern uint32_t local_weapon_id;
extern bool esp;
extern bool item_glow;
extern bool player_glow;
extern bool aim_no_recoil;
extern bool ready;
extern float max_dist;
extern float smooth;
extern bool MiniMapGuides;
extern float bulletspeed;
extern float bulletgrav;
extern bool map_radar_testing;

// Aimbot
extern bool lock;          // read lock state
extern bool aimbot_safety; // read safety state
extern float max_fov;
extern float ADSfov;
extern float nonADSfov;
extern int bone;
extern bool bone_auto;
extern bool thirdperson;
extern int spectators;
extern int allied_spectators;
// Left and Right Aim key toggle
extern bool toggleaim;
extern bool toggleaim2;
int e = 0;
// glow color and type
extern float glowrnot; // Red Value
extern float glowgnot; // Green Value
extern float glowbnot; // Blue Value
extern float glowcolornot[3];
// MiniMap Radar
int minimapradardotsize1 = 5;
int minimapradardotsize2 = 1;
bool minimapradar = true;
unsigned int radarcolorr = 0; // Red Value
unsigned int radarcolorg = 0; // Green Value
unsigned int radarcolorb = 0; // Blue Value
extern float radarcolor[3];
// more glow stuff
// glow visable
extern float glowrviz;
extern float glowgviz;
extern float glowbviz;
extern float glowcolorviz[3];
// knocked
extern float glowrknocked;
extern float glowgknocked;
extern float glowbknocked;
extern float glowcolorknocked[3];
// TDM Toggle
extern bool TDMToggle;
// Main Map Radar
bool mainradartoggle = 1;
bool mainradarmap = false;
int mainmapradardotsize1 = 5;
int mainmapradardotsize2 = 5;
// Others
extern bool show_aim_target;
extern float game_fps; // for aimbot calc
bool use_overlay_fps = true;

// Ha think i was done ?
// Item Filter Brute Force!
extern bool lightbackpack;
extern bool medbackpack;
extern bool heavybackpack;
extern bool goldbackpack;
// Shield upgrades
extern bool shieldupgrade1; // white
extern bool shieldupgrade2; // blue
extern bool shieldupgrade3; // purple
extern bool shieldupgrade4; // gold
extern bool shieldupgrade5; // red
extern bool shieldupgradehead1;
extern bool shieldupgradehead2;
extern bool shieldupgradehead3;
extern bool shieldupgradehead4;
extern bool shielddown1;
extern bool shielddown2;
extern bool shielddown3;
extern bool shielddown4;
// heaing and Misc
extern bool accelerant;
extern bool phoenix;
extern bool healthlarge;
extern bool healthsmall;
extern bool shieldbattsmall;
extern bool shieldbattlarge;
// Ammo
extern bool sniperammo;
extern bool heavyammo;
extern bool lightammo;
extern bool energyammo;
extern bool shotgunammo;
// Optics
extern bool optic1xhcog;
extern bool optic2xhcog;
extern bool opticholo1x;
extern bool opticholo1x2x;
extern bool opticthreat;
extern bool optic3xhcog;
extern bool optic2x4x;
extern bool opticsniper6x;
extern bool opticsniper4x8x;
extern bool opticsniperthreat;
// Magazines
extern bool sniperammomag1;
extern bool energyammomag1;
extern bool lightammomag1;
extern bool heavyammomag1;
extern bool sniperammomag2;
extern bool energyammomag2;
extern bool lightammomag2;
extern bool heavyammomag2;
extern bool sniperammomag3;
extern bool energyammomag3;
extern bool lightammomag3;
extern bool heavyammomag3;
extern bool sniperammomag4;
extern bool energyammomag4;
extern bool lightammomag4;
extern bool heavyammomag4;
// Attachments
extern bool lasersight1;
extern bool lasersight2;
extern bool lasersight3;
extern bool lasersight4;
extern bool stocksniper1;
extern bool stocksniper2;
extern bool stocksniper3;
extern bool stocksniper4;
extern bool stockregular1;
extern bool stockregular2;
extern bool stockregular3;
extern bool suppressor1;
extern bool suppressor2;
extern bool suppressor3;
extern bool turbo_charger;
extern bool skull_piecer;
extern bool hammer_point;
extern bool disruptor_rounds;
extern bool boosted_loader;
extern bool shotgunbolt1;
extern bool shotgunbolt2;
extern bool shotgunbolt3;
extern bool shotgunbolt4;
// Nades
extern bool grenade_frag;
extern bool grenade_arc_star;
extern bool grenade_thermite;
// Kraber
extern bool weapon_kraber;
// Shotguns
extern bool weapon_mastiff;
extern bool weapon_eva8;
extern bool weapon_peacekeeper;
extern bool weapon_mozambique;
// Energy weapons
extern bool weapon_lstar;
extern bool weapon_nemesis;
extern bool weapon_havoc;
extern bool weapon_devotion;
extern bool weapon_triple_take;
extern bool weapon_prowler;
extern bool weapon_volt;
// Heavy Weapons
extern bool weapon_flatline;
extern bool weapon_hemlock;
extern bool weapon_3030_repeater;
extern bool weapon_rampage;
extern bool weapon_car_smg;
// Light weapons
extern bool weapon_p2020;
extern bool weapon_re45;
extern bool weapon_g7_scout;
extern bool weapon_alternator;
extern bool weapon_r99;
extern bool weapon_spitfire;
extern bool weapon_r301;
// Snipers.. wingman is the odd one...and the bow..
extern bool weapon_wingman;
extern bool weapon_longbow;
extern bool weapon_charge_rifle;
extern bool weapon_sentinel;
extern bool weapon_bow;

// Aim Dist check
extern float aimdist;
// item glow brightness
extern int itemglowbrightness;
// Menu Stuff
int menu1 = 0;
int menu2 = 0;
int menu3 = 0;
int menu4 = 0;
// triggerbot
extern bool triggerbot;
// 1v1
extern bool onevone;
// // screen pos ajuster
// // ajuster for screen pos
// extern int worldsedgetoprightx = 0;
// extern int worldsedgetoprighty = 0;
// extern int worldsedgebtmleftx = 0;
// extern int worldsedgebtmlefty = 0;

int width;
int height;
bool k_leftclick = false;
bool k_ins = false;
bool show_menu = false;
visuals v;

// extern bool IsKeyDown(int vk);
extern bool IsKeyDown(ImGuiKey imgui_k);

static void StaticMessageStart(Overlay &ov) { ov.CreateOverlay(); }

void Overlay::RenderMenu() {
  static bool aim_enable = false;
  static bool vis_check = false;

  if (aim > 0) {
    aim_enable = true;
    if (aim > 1) {
      vis_check = true;
    } else {
      vis_check = false;
    }
  } else {
    aim_enable = false;
    vis_check = false;
  }

  ImGui::SetNextWindowPos(ImVec2(0, 0));
  ImGui::SetNextWindowSize(ImVec2(450, 860), ImGuiCond_Once);
  ImGui::Begin(XorStr("##title"), (bool *)true,
               ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoScrollbar);
  // if (ImGui::BeginTabBar(XorStr("Tab")))
  //{
  // if (ImGui::BeginTabItem(XorStr("##")))
  //{
  if (ImGui::CollapsingHeader("Main Toggle Settings")) {
    menu1 = 1;
    ImGui::Checkbox(XorStr("ESP On/Off"), &esp);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Thirdperson"), &thirdperson);

    ImGui::Checkbox(XorStr("Glow Items"), &item_glow);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Mini-Map Radar"), &minimapradar);

    ImGui::Checkbox(XorStr("Glow Players"), &player_glow);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Mini-Map Guide"), &MiniMapGuides);

    ImGui::Checkbox(XorStr("AIM On/Off"), &aim_enable);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("1v1"), &onevone);

    if (aim_enable) {
      ImGui::Checkbox(XorStr("Visibility Check"), &vis_check);
      ImGui::SameLine();
      ImGui::Checkbox(XorStr("No Recoil"), &aim_no_recoil);
      ImGui::SameLine();
      ImGui::Checkbox(XorStr("No Nade Aim"), &NoNadeAim);
      if (vis_check) {
        aim = 2;
      } else {
        aim = 1;
      }
    } else {
      aim = 0;
    }

    ImGui::Checkbox(XorStr("Firing Range"), &firing_range);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("TDM Toggle"), &TDMToggle);
    ImGui::Checkbox(XorStr("Press C duck for MapRadar"), &map_radar_testing);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Aiming Distance:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f meters", aimdist / 39.62);
    ImGui::SliderFloat(XorStr("##Aim Distance"), &aimdist, 10.0f * 39.62,
                       1600.0f * 39.62, "##");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Aiming Keys:"));
    ImGui::RadioButton("Left Mouse", &e, 1);
    ImGui::SameLine();
    ImGui::RadioButton("Right Mouse ", &e, 2);
    ImGui::SameLine();
    ImGui::RadioButton("Left/Right Mouse", &e, 3);
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
    ImGui::Text(XorStr("Max distance for everything:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%d meters", (int)(max_dist / 40));
    ImGui::SliderFloat(XorStr("##1"), &max_dist, 100.0f * 40, 3800.0f * 40,
                       "##");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text(XorStr("Max fov:"));
    ImGui::Dummy(ImVec2(0.0f, 4.0f));
    ImGui::Text(XorStr("non-ADS:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", nonADSfov);
    ImGui::SliderFloat(XorStr("##nonADSfov"), &nonADSfov, 5.0f, 50.0f, "##");
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text(XorStr("ADS:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", ADSfov);
    ImGui::SliderFloat(XorStr("##ADSfov"), &ADSfov, 5.0f, 50.0f, "##");
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text(XorStr("Current:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.f", max_fov);
    ImGui::SliderFloat(XorStr("##max_fov"), &max_fov, 5.0f, 50.0f, "##");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Dummy(ImVec2(0.0f, 2.0f));
    ImGui::Text(XorStr("Smooth Aim Value:"));
    ImGui::SameLine();
    if (smooth < 100.0f) {
      ImGui::TextColored(RED, "%.f", smooth);
    } else if (smooth > 120.0f) {
      ImGui::TextColored(GREEN, "%.f", smooth);
    } else {
      ImGui::TextColored(WHITE, "%.f", smooth);
    }
    ImGui::SliderFloat(XorStr("##2"), &smooth, 85.0f, 150.0f, "##");
    ImGui::SameLine();
    ImGui::Text(XorStr("85 To 100 Is Safe"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text(XorStr("Smooth Preditcion Speed:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.2f", bulletspeed);
    ImGui::SliderFloat(XorStr("##55"), &bulletspeed, -10.58f, 5.80f, "##");
    ImGui::SameLine();
    ImGui::Text(XorStr("Default is 0.08"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));

    ImGui::Text(XorStr("Smooth Preditcion Gravity:"));
    ImGui::SameLine();
    ImGui::TextColored(GREEN, "%.2f", bulletgrav);
    ImGui::SliderFloat(XorStr("##57"), &bulletgrav, -10.55f, 5.90f, "##");
    ImGui::SameLine();
    ImGui::Text(XorStr("Default is 0.05"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Aiming Bone:"));
    ImGui::Checkbox(XorStr("Auto"), &bone_auto);
    ImGui::Text(XorStr("0=Head, 1=Neck, 2=Chest, 3=Stomach"));
    ImGui::SliderInt(XorStr("##bone"), &bone, 0, 3);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("ESP Options:"));
    ImGui::Checkbox(XorStr("Box"), &v.box);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Line"), &v.line);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Distance"), &v.distance);
    ImGui::Checkbox(XorStr("Health bar"), &v.healthbar);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Shield bar"), &v.shieldbar);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Name"), &v.name);
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(XorStr("Show aimbot target"), &show_aim_target);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Player Glow Visable:"));
    ImGui::ColorEdit3("##Glow Color Picker Visable", glowcolorviz);
    {
      glowrviz = glowcolorviz[0];
      glowgviz = glowcolorviz[1];
      glowbviz = glowcolorviz[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Player Glow Not Visable:"));
    ImGui::ColorEdit3("##Glow Color Not Visable", glowcolornot);
    {
      glowrnot = glowcolornot[0];
      glowgnot = glowcolornot[1];
      glowbnot = glowcolornot[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Player Glow Knocked:"));
    ImGui::ColorEdit3("##Glow Color Knocked", glowcolorknocked);
    {
      glowrknocked = glowcolorknocked[0];
      glowgknocked = glowcolorknocked[1];
      glowbknocked = glowcolorknocked[2];
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(
        XorStr("Saving and Loading. Need to Save Once to make the file."));
    // Saving
    if (ImGui::Button("Save Config")) {
      saveSettings();
    }
    ImGui::SameLine();
    // Loading
    if (ImGui::Button("Load Config")) {
      loadSettings();
    }
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu1 == 1) {
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Weapone Filter Settings"),
                                       0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Item Filter Settings"), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Radar Settings"), 0);
    }
  }
  if (ImGui::CollapsingHeader("Radar Settings")) {
    menu2 = 1;
    // Dot Size for both mini and main map
    ImGui::Text(XorStr("MiniMap Radar Dot Size"));
    ImGui::SliderInt(XorStr("MiniMap Dot Size"), &minimapradardotsize1, 1, 10);
    ImGui::SliderInt(XorStr("MiniMap Outer Ring Thickness"),
                     &minimapradardotsize2, 1, 10);
    ImGui::Text(XorStr("Main Map Radar Dot Size"));
    ImGui::SliderInt(XorStr("Main Map Dot Width"), &mainmapradardotsize1, 1,
                     10);
    ImGui::SliderInt(XorStr("Main Map Dot length"), &mainmapradardotsize2, 1,
                     10);
    /*//Radar Color
    ImGui::Text(XorStr("Radar Color Picker:"));
    ImGui::ColorEdit3("##Radar Color Picker", radarcolor);
    {
            radarcolorr = radarcolor[0];
            radarcolorg = radarcolor[1];
            radarcolorb = radarcolor[2];
    }
    */
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu2 == 1) {
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Weapone Filter Settings"),
                                       0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Item Filter Settings"), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Main Toggle Settings"), 0);
    }
  }
  if (ImGui::CollapsingHeader("Item Filter Settings")) {
    menu3 = 1;
    ImGui::Text(XorStr("Ammo"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(XorStr("Sniper Ammo"), &sniperammo);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Heavy Ammo"), &heavyammo);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Light Ammo"), &lightammo);
    ImGui::Checkbox(XorStr("Energy Ammo"), &energyammo);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Shotgun Ammo"), &shotgunammo);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Ammo Mags"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(XorStr("Sniper lv3"), &sniperammomag3);
    ImGui::Checkbox(XorStr("Sniper lv4"), &sniperammomag4);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Heavy lv3"), &heavyammomag3);
    ImGui::Checkbox(XorStr("Heavy lv4"), &heavyammomag4);
    ImGui::Checkbox(XorStr("Light lv3"), &lightammomag3);
    ImGui::Checkbox(XorStr("Light lv4"), &lightammomag4);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Energy lv3"), &energyammomag3);
    ImGui::Checkbox(XorStr("Energy lv4"), &energyammomag4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("HCOGs"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(XorStr("1x HCOG"), &optic1xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("2x HCOG"), &optic2xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("3x HCOG"), &optic3xhcog);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("2-4x"), &optic2x4x);
    ImGui::Text(XorStr("Snipers"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(XorStr("Sniper 6x"), &opticsniper6x);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Sniper 4-8x"), &opticsniper4x8x);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Sniper Threat"), &opticsniperthreat);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Holo's"));
    ImGui::Dummy(ImVec2(0.0f, 1.0f));
    ImGui::Checkbox(XorStr("1x Holo"), &opticholo1x);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("2x Holo"), &opticholo1x2x);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("1x Threat"), &opticthreat);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Backpacks"));
    ImGui::Checkbox(XorStr("Light Backpack"), &lightbackpack);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Medium Backpack"), &medbackpack);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Heavy Backpack"), &heavybackpack);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Shield's"));
    ImGui::Checkbox(XorStr("Armor blue"), &shieldupgrade2);
    ImGui::Checkbox(XorStr("Armor purple"), &shieldupgrade3);
    ImGui::Checkbox(XorStr("Armor gold"), &shieldupgrade4);
    ImGui::Checkbox(XorStr("Armor red"), &shieldupgrade5);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Helmets blue"), &shieldupgradehead2);
    ImGui::Checkbox(XorStr("Helmets purple"), &shieldupgradehead3);
    ImGui::Checkbox(XorStr("Helmets gold"), &shieldupgradehead4);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Knockdown blue"), &shielddown2);
    ImGui::Checkbox(XorStr("Knockdown purple"), &shielddown3);
    ImGui::Checkbox(XorStr("Knockdown gold"), &shielddown4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(GREEN, "Heals for Health");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("Large Health"), &healthlarge);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Small Health"), &healthsmall);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Phoenix Kit"), &phoenix);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(BLUE, "Heals for Shields");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("Large Shield"), &shieldbattlarge);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Small Shield"), &shieldbattsmall);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Ultimate Accelerant"), &accelerant);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Attachements"));
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(XorStr("Lasersight2"), &lasersight2);
    ImGui::Checkbox(XorStr("Lasersight3"), &lasersight3);
    ImGui::Checkbox(XorStr("Lasersight4"), &lasersight4);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Sniper Stock 2"), &stocksniper2);
    ImGui::Checkbox(XorStr("Sniper Stock 3"), &stocksniper3);
    ImGui::Checkbox(XorStr("Sniper Stock 4"), &stocksniper4);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Regular Stock 2"), &stockregular2);
    ImGui::Checkbox(XorStr("Regular Stock 3"), &stockregular3);
    ImGui::Checkbox(XorStr("Suppressor 1"), &suppressor1);
    ImGui::Checkbox(XorStr("Suppressor 2"), &suppressor2);
    ImGui::Checkbox(XorStr("Suppressor 3"), &suppressor3);
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Text(XorStr("Weapon Mods"));
    ImGui::Dummy(ImVec2(0.0f, 5.0f));
    ImGui::Checkbox(XorStr("Turbo Charger"), &turbo_charger);
    ImGui::Checkbox(XorStr("Skull Piecer"), &skull_piecer);
    ImGui::Checkbox(XorStr("Hammer Point"), &hammer_point);
    ImGui::Checkbox(XorStr("Disruptor Rounds"), &disruptor_rounds);
    ImGui::Checkbox(XorStr("Boosted Loader"), &boosted_loader);
    ImGui::Checkbox(XorStr("Shotgunbolt 1"), &shotgunbolt1);
    ImGui::Checkbox(XorStr("Shotgunbolt 2"), &shotgunbolt2);
    ImGui::Checkbox(XorStr("Shotgunbolt 3"), &shotgunbolt3);
    ImGui::Checkbox(XorStr("Shotgunbolt 4"), &shotgunbolt4);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Item Glow Brightness:"));
    ImGui::SliderInt(XorStr("##itemglowbright"), &itemglowbrightness, 2, 40,
                     "%d");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu3 == 1) {
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Weapone Filter Settings"),
                                       0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Radar Settings"), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Main Toggle Settings"), 0);
    }
  }
  if (ImGui::CollapsingHeader("Weapone Filter Settings")) {
    menu4 = 1;
    // Light Weapons
    ImGui::TextColored(ORANGE, "Light Weapons");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("P2020"), &weapon_p2020);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("RE-45"), &weapon_re45);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("R-99"), &weapon_r99);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("G7 Scout"), &weapon_g7_scout);
    ImGui::Checkbox(XorStr("Spitfire"), &weapon_spitfire);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("R-301"), &weapon_r301);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Alternator "), &weapon_alternator);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    // Heavy Weapons
    ImGui::TextColored(TEAL, "Heavy Weapons");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("C.A.R."), &weapon_car_smg);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Flatline"), &weapon_flatline);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Hemlok"), &weapon_hemlock);
    ImGui::Checkbox(XorStr("Prowler "), &weapon_prowler);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("30-30"), &weapon_3030_repeater);
    ImGui::Checkbox(XorStr("Rampage"), &weapon_rampage);
    // Energy Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(YELLOW, "Energy Weapons");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("Triple Take"), &weapon_triple_take);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("L-STAR"), &weapon_lstar);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Volt"), &weapon_volt);
    ImGui::Checkbox(XorStr("Devotion "), &weapon_devotion);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("HAVOC"), &weapon_havoc);
    ImGui::Checkbox(XorStr("Nemesis"), &weapon_nemesis);

    // Shotgun Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(RED, "Shotgun Weapons");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("EVA-8"), &weapon_eva8);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Mozambique"), &weapon_mozambique);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Peacekeeper"), &weapon_peacekeeper);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Mastiff"), &weapon_mastiff);
    // Sniper Weapons
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::TextColored(BLUE, "Sniper Weapons");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("Charge Rifle"), &weapon_charge_rifle);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Longbow"), &weapon_longbow);
    ImGui::SameLine();
    ImGui::Checkbox(XorStr("Sentinel"), &weapon_sentinel);
    ImGui::Checkbox(XorStr("Wingman "), &weapon_wingman);
    // KRABER
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Special Weapons"));
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Checkbox(XorStr("Kraber .50-Cal Sniper"), &weapon_kraber);
    ImGui::Checkbox(XorStr("Bocek Bow"), &weapon_bow);
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    ImGui::Text(XorStr("Item Glow Brightness:"));
    ImGui::SliderInt(XorStr("##itemglowbright"), &itemglowbrightness, 2, 40,
                     "%d");
    ImGui::Dummy(ImVec2(0.0f, 10.0f));
    if (menu4 == 1) {
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Item Filter Settings"), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Radar Settings"), 0);
      ImGui::GetStateStorage()->SetInt(ImGui::GetID("Main Toggle Settings"), 0);
    }
  }
  // ImGui::EndTabItem();

  //}

  // ImGui::EndTabBar();
  //}
  ImGui::Dummy(ImVec2(0.0f, 10.0f));
  ImGui::Text(XorStr("held=%d, weapon=%d"), local_held_id, local_weapon_id);
  ImGui::Dummy(ImVec2(0.0f, 5.0f));
  ImGui::Text(XorStr("Overlay FPS: %.3f ms/frame (%.1f FPS)"),
              1000.0f / ImGui::GetIO().Framerate, ImGui::GetIO().Framerate);
  ImGui::Dummy(ImVec2(0.0f, 5.0f));

  ImGui::Text(XorStr("Game FPS for Aim Prediction:"));
  ImGui::SameLine();
  ImGui::Checkbox(XorStr("Use overlay FPS"), &use_overlay_fps);
  if (use_overlay_fps)
    game_fps = ImGui::GetIO().Framerate;
  ImGui::SliderFloat(XorStr("##gamefps"), &game_fps, 1.0f, 300.0f, "%.1f");

  ImGui::Dummy(ImVec2(0.0f, 5.0f));
  ImGui::Text(XorStr("external-overlay test build"));
  ImGui::End();
}

void Overlay::RenderInfo() {
  ImGui::SetNextWindowPos(ImVec2(0, 0));
  ImGui::SetNextWindowSize(ImVec2(280, 30));
  ImGui::Begin(XorStr("##info"), (bool *)true,
               ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize |
                   ImGuiWindowFlags_NoScrollbar);
  DrawLine(ImVec2(1, 2), ImVec2(280, 2), RED, 2);
  if (spectators == 0) {
    ImGui::TextColored(GREEN, "%d", spectators);
  } else {
    ImGui::TextColored(RED, "%d", spectators);
  }
  ImGui::SameLine();
  ImGui::Text("--");
  ImGui::SameLine();
  ImGui::TextColored(GREEN, "%d", allied_spectators);
  ImGui::SameLine();
  ImGui::Text("--");
  ImGui::SameLine();
  ImGui::TextColored(WHITE, "%.f", max_fov);
  ImGui::SameLine();
  ImGui::Text("--");
  ImGui::SameLine();
  // Aim is on = 2, On but No Vis Check = 1, Off = 0
  if (lock) {
    ImGui::TextColored(aimbot_safety ? GREEN : ORANGE, "[TARGET LOCK!]");
  } else if (local_held_id == -251) {
    ImGui::TextColored(BLUE, "Skynade On");
  } else if (aim == 2) {
    ImGui::TextColored(GREEN, "Aim On");

  } else if (aim == 0) {
    ImGui::TextColored(RED, "Aim Off");
  } else {
    ImGui::TextColored(RED, "Aim On %d", aim);
  }
  ImGui::SameLine();
  // if (triggerbot) {
  //   ImGui::TextColored(GREEN, "1v1 On");
  // } else {
  //   ImGui::TextColored(RED, "1v1 Off");
  // }
  DrawLine(ImVec2(1, 28), ImVec2(280, 28), RED, 2);
  ImGui::End();
}

static void glfw_error_callback(int error, const char *description) {
  fprintf(stderr, "GLFW Error %d: %s\n", error, description);
}

int Overlay::CreateOverlay() {
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
  glfwWindowHint(GLFW_RESIZABLE, 1);
  glfwWindowHint(GLFW_TRANSPARENT_FRAMEBUFFER, 1);

  // Create window with graphics context
  GLFWwindow *window = glfwCreateWindow(1920, 1080, "Client ImGui GLFW+OpenGL3",
                                        glfwGetPrimaryMonitor(), nullptr);
  if (window == nullptr)
    return 1;
  static const char *GamescopeOverlayProperty = "GAMESCOPE_EXTERNAL_OVERLAY";
  Display *x11_display = glfwGetX11Display();
  Window x11_window = glfwGetX11Window(window);
  if (x11_window && x11_display) {
    // Set atom for gamescope to render as an overlay.
    Atom overlay_atom =
        XInternAtom(x11_display, GamescopeOverlayProperty, False);
    uint32_t value = 1;
    XChangeProperty(x11_display, x11_window, overlay_atom, XA_CARDINAL, 32,
                    PropertyNewValue, (unsigned char *)&value, 1);
  }
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

#include "impl/render/font.h"
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
    if (!running)
      break;
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
      ImGui::Begin("Hello, world!"); // Create a window called "Hello, world!"
                                     // and append into it.

      ImGui::Text("This is some useful text."); // Display some text (you can
      // use a format strings too)
      ImGui::Text(u8"fsdfsdad我是中文dfd");
      ImGui::Text(u8"Kanjis: \xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e (nihongo)");
      ImGui::Checkbox(
          "Demo Window",
          &show_demo_window); // Edit bools storing our window open/close state

      ImGui::Text("Application average %.3f ms/frame (%.1f FPS)",
                  1000.0f / io.Framerate, io.Framerate);
      ImGui::End();
    }

    // 3. Show another simple window.
    if (show_another_window) {
      ImGui::Begin(
          "Another Window",
          &show_another_window); // Pass a pointer to our bool variable (the
                                 // window will have a closing button that will
                                 // clear the bool when clicked)
      ImGui::Text("Hello from another window!");
      if (ImGui::Button("Close Me"))
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

    if (IsKeyDown(ImGuiKey_Insert) && !k_ins && ready) {
      show_menu = !show_menu;
      k_ins = true;
    } else if (!IsKeyDown(ImGuiKey_Insert) && k_ins) {
      k_ins = false;
    }

    // Main Map Radar, Needs Manual Setting of cords
    if (IsKeyDown(ImGuiKey_M) && mainradartoggle == 0) {
      mainradartoggle = 1;
      if (!mainradarmap) {
        mainradarmap = true;
        minimapradar = false;
      } else {
        mainradarmap = false;
        minimapradar = true;
      }
    } else if (!IsKeyDown(ImGuiKey_M) && mainradartoggle == 1) {
      mainradartoggle = 0;
    }

    if (show_menu)
      RenderMenu();
    else
      RenderInfo();

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
    // glClearColor(clear_color.x * clear_color.w, clear_color.y *
    // clear_color.w, clear_color.z * clear_color.w, clear_color.w);
    glClearColor(0, 0, 0, 0);
    glClear(GL_COLOR_BUFFER_BIT);
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
  running = 0;
  std::this_thread::sleep_for(std::chrono::milliseconds(50));
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
