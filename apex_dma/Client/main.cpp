#include "main.h"
#include "../Game.h"
#include "imgui.h"
#include "overlay.h"
#include <map>
#include <random>
#include <thread>
typedef Vector D3DXVECTOR3;

typedef uint8_t *PBYTE;
typedef uint8_t BYTE;
typedef unsigned long DWORD;
typedef unsigned short WORD;
typedef WORD *PWORD;

uint32_t check = 0xABCD;

// Left and Right Aim key toggle
bool toggleaim = false;
bool toggleaim2 = false;
extern int e;
extern bool firing_range; // sync
extern bool active;       // sync
bool ready = false;
extern visuals v;
extern int aim;                   // read sync
bool esp = true;                  // read sync
extern bool item_glow;            // sync
extern bool player_glow;          // sync
extern bool aim_no_recoil;        // sync
extern bool aiming;               // read sync
extern uint64_t g_Base;           // write sync
float max_dist = 3800.0f * 40.0f; // read //Max Distance of ESP 3800 is full map
extern float smooth;              // Min 100 for safe aimbotting   sync
extern float max_fov;             // 15 is the sweetspot for 1080p
// Dynamic Fov
float dynamicfov = 10;
float dynamicfovmax = 15.0f;
// tdm check
extern int EntTeam;    // sync
extern int LocTeam;    // sync
extern bool TDMToggle; // sync
// triggerbot
bool triggerbot = false;
// 1v1
bool onevone = false;

float smoothpred = 0.08;
float smoothpred2 = 0.05;
float veltest = 1.00; // sync
bool MiniMapGuides = true;
extern int bone; // 0 Head, 1 Neck, 2 Body, 3 Stomace, 4 Nuts  sync
// Player Glow Color and Brightness
extern float glowrnot; // Red Value
extern float glowgnot; // Green Value
extern float glowbnot; // Blue Value
float glowcolornot[3] = {1.0f, 0.0f, 0.0f};
// more glow stuff
// glow visable
extern float glowrviz;
extern float glowgviz;
extern float glowbviz;
float glowcolorviz[3] = {0.0f, 1.0f, 0.0f};
// knocked
extern float glowrknocked;
extern float glowgknocked;
extern float glowbknocked;
float glowcolorknocked[3] = {1.0f, 1.0f, 1.0f};
extern int minimapradardotsize1;
extern int minimapradardotsize2;
extern bool minimapradar;
extern unsigned int radarcolorr; // Red Value
extern unsigned int radarcolorg; // Green Value
extern unsigned int radarcolorb; // Blue Value
float radarcolor[3];
// Full Map Radar
extern bool mainradartoggle; // Toggle for Main Map radar
extern bool mainradarmap;    // if the Main Map Radar is enabled
bool kingscanyon = false;    // Set for map, ONLY ONE THO
bool stormpoint = true;      // Set for map, ONLY ONE THO
extern int mainmapradardotsize1;
extern int mainmapradardotsize2;
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
// Aim distance check
extern float aimdist; // sync
// item glow brightness
int itemglowbrightness = 10;
// Map number
extern int map;

bool thirdperson = false;
extern int spectators;        // write sync
extern int allied_spectators; // write sync
extern bool valid;            // write sync
extern bool next2;            // read write sync

extern bool overlay_t;

extern player players[100];

// Radar Code
#define M_PI 3.14159265358979323846 // matches value in gcc v2 math.h

static D3DXVECTOR3 RotatePoint(D3DXVECTOR3 EntityPos,
                               D3DXVECTOR3 LocalPlayerPos, int posX, int posY,
                               int sizeX, int sizeY, float angle, float zoom,
                               bool *viewCheck) {
  float r_1, r_2;
  float x_1, y_1;

  r_1 = -(EntityPos.y - LocalPlayerPos.y);
  r_2 = EntityPos.x - LocalPlayerPos.x;
  float Yaw = angle - 90.0f;

  float yawToRadian = Yaw * (float)(M_PI / 180.0F);
  x_1 = (float)(r_2 * (float)cos((double)(yawToRadian)) -
                r_1 * sin((double)(yawToRadian))) /
        20;
  y_1 = (float)(r_2 * (float)sin((double)(yawToRadian)) +
                r_1 * cos((double)(yawToRadian))) /
        20;

  *viewCheck = y_1 < 0;

  x_1 *= zoom;
  y_1 *= zoom;

  int sizX = sizeX / 2;
  int sizY = sizeY / 2;

  x_1 += sizX;
  y_1 += sizY;

  if (x_1 < 5)
    x_1 = 5;

  if (x_1 > sizeX - 5)
    x_1 = sizeX - 5;

  if (y_1 < 5)
    y_1 = 5;

  if (y_1 > sizeY - 5)
    y_1 = sizeY - 5;

  x_1 += posX;
  y_1 += posY;

  return D3DXVECTOR3(x_1, y_1, 0);
}
struct RGBA2 {
  int R;
  int G;
  int B;
  int A;
};
std::map<int, RGBA2> teamColors;
// Main Map Radar Color
typedef struct {
  DWORD R;
  DWORD G;
  DWORD B;
  DWORD A;
} RGBA;

typedef RGBA D3DXCOLOR;

// static void FilledRectangle(int x, int y, int w, int h, RGBA color)
//{
//	ImGui::GetWindowDrawList()->AddRectFilled(ImVec2(x, y), ImVec2(x + w, y
//+ h), ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
// color.B / 255.0, color.A / 255.0)), 0, 0);
// }

// Color Team Radar Test. oh god why... This is stupid.. dont do this.. it works
// tho
static void Team1(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team2(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team3(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team4(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team5(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team6(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team7(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team8(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team9(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team10(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team11(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team12(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team13(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team14(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team15(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team16(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team17(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team18(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team19(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team20(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team21(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team22(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team23(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team24(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team25(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team26(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team27(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team28(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team29(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team30(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team31(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team32(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team33(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team34(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}
static void Team35(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}

static void TeamMiniMap(int x, int y, int radius, int teamID, float targetyaw) {
  RGBA2 color;
  auto it = teamColors.find(teamID);
  if (it == teamColors.end()) {
    // Define the minimum sum of RGB values for a color to be considered "light"
    const int MIN_SUM_RGB = 500;

    // Generate a new random color for this team, discarding colors with a low
    // sum of RGB values
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 255);
    RGBA2 color;
    do {
      color = {dis(gen), dis(gen), dis(gen), 255};
    } while (color.R + color.G + color.B < MIN_SUM_RGB);

    // Store the color in the teamColors map
    teamColors[teamID] = color;
  } else {
    // Use the previously generated color for this team
    color = it->second;
  }

  auto colOutline = ImGui::ColorConvertFloat4ToU32(ImVec4(0.0, 0.0, 0.0, 1.0));
  ImVec2 center(x, y);
  ImGui::GetWindowDrawList()->AddCircleFilled(
      center, radius,
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)));
  ImGui::GetWindowDrawList()->AddCircle(center, radius, colOutline, 12,
                                        minimapradardotsize2);

  // Draw a line pointing in the direction of each player's aim
  const int numPlayers = 3;
  for (int i = 0; i < numPlayers; i++) {
    float angle = (360.0 - targetyaw) *
                  (M_PI / 180.0); // Replace this with the actual yaw of the
                                  // player, then convert it to radians.
    ImVec2 endpoint(center.x + radius * cos(angle),
                    center.y + radius * sin(angle));
    ImGui::GetWindowDrawList()->AddLine(center, endpoint, colOutline);
  }
}

bool menu = true;
bool firstS = true;
// Radar Settings.. ToDO: Put in ImGui menu to change in game
namespace RadarSettings {
bool Radar = true;
bool teamRadar = true;
bool enemyRadar = true;
int xAxis_Radar = 0;
int yAxis_Radar = 400;
int radartype = 0;
int width_Radar = 400;
int height_Radar = 400;
int distance_Radar = 250;
int distance_Radar2 = 1000;
}; // namespace RadarSettings

void DrawRadarPointMiniMap(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos,
                           float LocalPlayerY, float eneamyDist, int TeamID,
                           int xAxis, int yAxis, int width, int height,
                           D3DXCOLOR color, float targetyaw) {
  bool out = false;
  D3DXVECTOR3 siz;
  siz.x = width;
  siz.y = height;
  D3DXVECTOR3 pos;
  pos.x = xAxis;
  pos.y = yAxis;
  bool ck = false;
  D3DXVECTOR3 single = RotatePoint(EneamyPos, LocalPos, pos.x, pos.y, siz.x,
                                   siz.y, LocalPlayerY, 0.3f, &ck);
  if (eneamyDist >= 0.f && eneamyDist < RadarSettings::distance_Radar) {
    for (int i = 1; i <= 30; i++) {
      TeamMiniMap(single.x, single.y, minimapradardotsize1, TeamID, targetyaw);
    }
  }
}

void DrawRadarPoint(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos,
                    float LocalPlayerY, float eneamyDist, int TeamID, int xAxis,
                    int yAxis, int width, int height, D3DXCOLOR color) {
  bool out = false;
  D3DXVECTOR3 siz;
  siz.x = width;
  siz.y = height;
  D3DXVECTOR3 pos;
  pos.x = xAxis;
  pos.y = yAxis;
  bool ck = false;

  D3DXVECTOR3 single = RotatePoint(EneamyPos, LocalPos, pos.x, pos.y, siz.x,
                                   siz.y, LocalPlayerY, 0.3f, &ck);
  if (eneamyDist >= 0.f && eneamyDist < RadarSettings::distance_Radar) {
    if (TeamID == 1) {
      Team1(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {255, 255, 255, 255});
    }
    if (TeamID == 2) {
      Team2(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {242, 86, 38, 255});
    }
    if (TeamID == 3) {
      Team3(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {242, 86, 38, 255});
    }
    if (TeamID == 4) {
      Team4(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {174, 247, 89, 255});
    }
    if (TeamID == 5) {
      Team5(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {102, 214, 173, 255});
    }
    if (TeamID == 6) {
      Team6(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {98, 244, 234, 255});
    }
    if (TeamID == 7) {
      Team7(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {92, 208, 250, 255});
    }
    if (TeamID == 8) {
      Team8(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {93, 137, 238, 255});
    }
    if (TeamID == 9) {
      Team9(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
            {164, 105, 252, 255});
    }
    if (TeamID == 10) {
      Team10(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {243, 98, 161, 255});
    }
    if (TeamID == 11) {
      Team11(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {214, 67, 67, 255});
    }
    if (TeamID == 12) {
      Team12(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {230, 116, 51, 255});
    }
    if (TeamID == 13) {
      Team13(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {185, 179, 167, 255});
    }
    if (TeamID == 14) {
      Team14(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {148, 200, 65, 255});
    }
    if (TeamID == 15) {
      Team15(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {86, 174, 91, 255});
    }
    if (TeamID == 16) {
      Team16(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {55, 188, 200, 255});
    }
    if (TeamID == 17) {
      Team17(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {84, 169, 212, 255});
    }
    if (TeamID == 18) {
      Team18(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {98, 121, 203, 255});
    }
    if (TeamID == 19) {
      Team19(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {102, 61, 174, 255});
    }
    if (TeamID == 20) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 21) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 22) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 23) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 24) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 25) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 26) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 27) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 28) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 29) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 30) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 31) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 32) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 33) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 34) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
    if (TeamID == 35) {
      Team20(single.x, single.y, minimapradardotsize1, minimapradardotsize2,
             {218, 73, 145, 255});
    }
  }
}
// MiniMap Radar Stuff
void MiniMapRadar(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos,
                  float LocalPlayerY, float eneamyDist, int TeamId,
                  float targetyaw) {
  ImGuiStyle *style = &ImGui::GetStyle();
  style->WindowRounding = 0.2f;
  ImGui::PushStyleColor(ImGuiCol_WindowBg,
                        ImVec4(0.13529413f, 0.14705884f, 0.15490198f, 0.82f));
  ImGuiWindowFlags TargetFlags;
  TargetFlags = ImGuiWindowFlags_::ImGuiWindowFlags_NoResize |
                ImGuiWindowFlags_::ImGuiWindowFlags_NoCollapse |
                ImGuiWindowFlags_::ImGuiWindowFlags_NoBackground |
                ImGuiWindowFlags_::ImGuiWindowFlags_NoTitleBar;
  // Remove the NoMove to move the minimap pos
  // you have to hit insert to bring up the hack menu, then while the menu is up
  // hit the windows kep to bring up the window start menu then just clikc back
  // on the middle of the screen to be on the overlay from there you can click
  // and drag the minmap around
  if (!firstS) {
    ImGui::SetNextWindowPos(ImVec2{1200, 60}, ImGuiCond_Once);
    firstS = true;
  }
  if (RadarSettings::Radar == true) {
    ImGui::SetNextWindowSize({250, 250});
    ImGui::Begin(("Radar"), 0, TargetFlags);
    {
      ImDrawList *Draw = ImGui::GetWindowDrawList();
      ImVec2 DrawPos = ImGui::GetCursorScreenPos();
      ImVec2 DrawSize = ImGui::GetContentRegionAvail();
      ImVec2 midRadar =
          ImVec2(DrawPos.x + (DrawSize.x / 2), DrawPos.y + (DrawSize.y / 2));
      if (MiniMapGuides) {
        ImGui::GetWindowDrawList()->AddLine(
            ImVec2(midRadar.x - DrawSize.x / 2.f, midRadar.y),
            ImVec2(midRadar.x + DrawSize.x / 2.f, midRadar.y),
            IM_COL32(255, 255, 255, 255));
        ImGui::GetWindowDrawList()->AddLine(
            ImVec2(midRadar.x, midRadar.y - DrawSize.y / 2.f),
            ImVec2(midRadar.x, midRadar.y + DrawSize.y / 2.f),
            IM_COL32(255, 255, 255, 255));
      }

      DrawRadarPointMiniMap(EneamyPos, LocalPos, LocalPlayerY, eneamyDist,
                            TeamId, DrawPos.x, DrawPos.y, DrawSize.x,
                            DrawSize.y, {255, 255, 255, 255}, targetyaw);
    }
    ImGui::End();
  }
  ImGui::PopStyleColor();
}

// bool IsKeyDown(int vk)
// {
// 	return (GetAsyncKeyState(vk) & 0x8000) != 0;
// }
bool IsKeyDown(ImGuiKey imgui_k) { return ImGui::IsKeyPressed(imgui_k); }

// Full map radar test, Needs Manual setting of cords
// ImVec2 can be replaced with Vector2D
class world {
public:
  ImVec2 w1; // origin of point 1
  ImVec2 w2; // origin of point 2
  ImVec2 s1; // screen coord of point 1
  ImVec2 s2; // screen coord of point 2
  float ratioX;
  float ratioY;
  world(ImVec2 w1, ImVec2 s1, ImVec2 w2, ImVec2 s2) {
    this->w1 = w1;
    this->w2 = w2;
    this->s1 = s1;
    this->s2 = s2;
    this->ratioX = (s2.x - s1.x) / (w2.x - w1.x);
    this->ratioY = (s1.y - s2.y) / (w2.y - w1.y);
  }
};
// These values only work with 1920x1080 fullscreen, you have to redo the values
// for anything else..
//
//  Take screenshot, First is top right random pos, then bttm left random pos
//  from screen shot
//
//  First set is the x cord, then the y cord, then the screen x,y from the
//  screenshot, do the same for the second set. 1440p is x1.333333

world KingsCanyon(ImVec2(25223.177734, 28906.144531), ImVec2(1197, 185),
                  ImVec2(10399.223633, 13334.792969),
                  ImVec2(1014, 381)); // could be more accurate

world WorldsEdge(ImVec2(20501.476562, 33754.492188), ImVec2(1159, 127),
                 ImVec2(-4714.299805, -54425.144531),
                 ImVec2(622, 755)); // mp_rr_desertlands_hu - could be more
                                    // accurate  updated 7/16/2023

world Olympus(ImVec2(0, 0), ImVec2(0, 0), ImVec2(0, 0),
              ImVec2(0, 0)); // to be measured

world BrokenMoon(ImVec2(35159.300781, 30436.917969), ImVec2(1368, 151),
                 ImVec2(-30641.98821, -30347.98821),
                 ImVec2(593, 873)); // mp_rr_divided_moon - could be more
                                    // accurate  updated 7/16/2023

world StormPoint(ImVec2(34453.894531, 34695.917969), ImVec2(1264, 172),
                 ImVec2(-28786.898438, -16240.749023),
                 ImVec2(636,
                        677)); // mp_rr_tropic_island_mu1_storm updated - is
                               // within a few pixels of accuracy 7/16/2023

// DONE get map auto
ImVec2 worldToScreenMap(D3DXVECTOR3 origin, int TeamID) {
  float ratioX;
  float ratioY;
  ImVec2 w1;
  ImVec2 s1;
  // Is it me being lazy? or that i dont know how? prob both. True or False for
  // the map detection, set in the overlay menu.
  if (map == 1) { // Storm Point
    ratioX = StormPoint.ratioX;
    ratioY = StormPoint.ratioY;
    w1 = StormPoint.w1;
    s1 = StormPoint.s1;
  }

  else if (map == 2) { // KingsCanyon
    ratioX = KingsCanyon.ratioX;
    ratioY = KingsCanyon.ratioY;
    w1 = KingsCanyon.w1;
    s1 = KingsCanyon.s1;
  } else if (map == 3) { // WorldsEdge
    ratioX = WorldsEdge.ratioX;
    ratioY = WorldsEdge.ratioY;
    w1 = WorldsEdge.w1;
    s1 = WorldsEdge.s1;
  } else if (map == 4) { // Olympus
    ratioX = Olympus.ratioX;
    ratioY = Olympus.ratioY;
    w1 = Olympus.w1;
    s1 = Olympus.s1;
  } else if (map == 5) { // BrokenMoon
    ratioX = BrokenMoon.ratioX;
    ratioY = BrokenMoon.ratioY;
    w1 = BrokenMoon.w1;
    s1 = BrokenMoon.s1;
  } else {
    return ImVec2(0, 0);
  }

  // difference from location 1
  float world_diff_x = origin.x - w1.x;
  float world_diff_y = origin.y - w1.y;

  // get the screen offsets by applying the ratio
  float scr_diff_x = world_diff_x * ratioX;
  float scr_diff_y = world_diff_y * ratioY;

  // for x, add the offset to the screen x of location 1
  // for y, subtract the offset from the screen y of location 1 (cuz Y is from
  // bottom to up in Apex but it's from up to bottom in screen)
  float pos_x = s1.x + scr_diff_x;
  float pos_y = s1.y - scr_diff_y;

  if (TeamID == 1) {
    Team1(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {255, 255, 255, 255});
  }
  if (TeamID == 2) {
    Team2(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {242, 86, 38, 255});
  }
  if (TeamID == 3) {
    Team3(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {242, 86, 38, 255});
  }
  if (TeamID == 4) {
    Team4(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {174, 247, 89, 255});
  }
  if (TeamID == 5) {
    Team5(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {102, 214, 173, 255});
  }
  if (TeamID == 6) {
    Team6(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {98, 244, 234, 255});
  }
  if (TeamID == 7) {
    Team7(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {92, 208, 250, 255});
  }
  if (TeamID == 8) {
    Team8(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {93, 137, 238, 255});
  }
  if (TeamID == 9) {
    Team9(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
          {164, 105, 252, 255});
  }
  if (TeamID == 10) {
    Team10(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {243, 98, 161, 255});
  }
  if (TeamID == 11) {
    Team11(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {214, 67, 67, 255});
  }
  if (TeamID == 12) {
    Team12(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {230, 116, 51, 255});
  }
  if (TeamID == 13) {
    Team13(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {185, 179, 167, 255});
  }
  if (TeamID == 14) {
    Team14(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {148, 200, 65, 255});
  }
  if (TeamID == 15) {
    Team15(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {86, 174, 91, 255});
  }
  if (TeamID == 16) {
    Team16(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {55, 188, 200, 255});
  }
  if (TeamID == 17) {
    Team17(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {84, 169, 212, 255});
  }
  if (TeamID == 18) {
    Team18(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {98, 121, 203, 255});
  }
  if (TeamID == 19) {
    Team19(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {102, 61, 174, 255});
  }
  if (TeamID == 20) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 21) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 22) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 23) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 24) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 25) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 26) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 27) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 28) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 29) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 30) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 31) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 32) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 33) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 34) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 35) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
  if (TeamID == 36) {
    Team20(pos_x, pos_y, mainmapradardotsize1, mainmapradardotsize2,
           {218, 73, 145, 255});
  }
}

void Overlay::RenderEsp() {
  next2 = false;
  if (g_Base != 0 && esp) {

    memset(players, 0, sizeof(players));

    while (!next2 && esp) {
      std::this_thread::sleep_for(std::chrono::milliseconds(2));
    }

    if (next2 && valid) {
      ImGui::SetNextWindowPos(ImVec2(0, 0));
      ImGui::SetNextWindowSize(ImVec2((float)getWidth(), (float)getHeight()));
      ImGui::Begin(XorStr("##esp"), (bool *)true,
                   ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize |
                       ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoScrollbar |
                       ImGuiWindowFlags_NoBackground |
                       ImGuiWindowFlags_NoBringToFrontOnFocus);

      for (int i = 0; i < 100; i++) {

        if (players[i].health > 0) {
          std::string distance = std::to_string(players[i].dist / 39.62);
          distance = distance.substr(0, distance.find('.')) + "m(" +
                     std::to_string(players[i].entity_team) + ")";

          float alpha; // The farther away, the more transparent
          if (players[i].dist < aimdist) {
            alpha = 1.0f;
          } else if (players[i].dist > 16000.0f) {
            alpha = 0.4f;
          } else {
            alpha = 1.0f -
                    ((players[i].dist - aimdist) / (16000.0f - aimdist) * 0.6f);
          }

          float radardistance =
              (int)((players[i].LocalPlayerPosition, players[i].dist) / 39.62);

          // Radar Stuff
          if (minimapradar == true) {
            MiniMapRadar(players[i].EntityPosition,
                         players[i].LocalPlayerPosition,
                         players[i].localviewangle.y, radardistance,
                         players[i].entity_team, players[i].targetyaw);
          }
          if (v.line)
            DrawLine(ImVec2((float)(getWidth() / 2), (float)getHeight()),
                     ImVec2(players[i].b_x, players[i].b_y), BLUE,
                     1); // LINE FROM MIDDLE SCREEN

          if (v.distance) {
            if (players[i].knocked)
              String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)), RED,
                     distance.c_str()); // DISTANCE
            else
              String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)),
                     ImColor(0.0f, 1.0f, 0.0f, alpha),
                     distance.c_str()); // DISTANCE
          }

          if (players[i].dist < 16000.0f) {
            if (v.healthbar)
              DrawSeerLikeHealth(
                  (players[i].b_x - (players[i].width / 2.0f) + 5),
                  (players[i].b_y - players[i].height - 10), players[i].shield,
                  players[i].maxshield, players[i].armortype,
                  players[i].health); // health bar

            if (v.box) {
              ImColor box_color = ImColor(0.0f, 0.0f, 0.0f, alpha);
              float box_width = 1.0f;
              if (players[i].visible) {
                box_color = ImColor(glowrviz, glowgviz, glowbviz, alpha);
                box_width = 2.0f;
              } else {
                box_color = ImColor(glowrnot, glowgnot, glowbnot, alpha);
              }
              DrawBox(box_color, players[i].b_x - (players[i].width / 2.0f),
                      players[i].b_y - players[i].height, players[i].width,
                      players[i].height, box_width);
            }
            if (v.name)
              String(ImVec2(players[i].boxMiddle,
                            (players[i].b_y - players[i].height - 15)),
                     ImColor(1.0f, 1.0f, 1.0f, alpha), players[i].name);
          }
          // Full Radar map, Need Manual setting of cords
          if (mainradarmap == true)

            worldToScreenMap(players[i].EntityPosition, players[i].entity_team);

          // String(ImVec2(players[i].boxMiddle, (players[i].b_y -
          // players[i].height - 15)), WHITE, players[i].name);
        }
      }
      ImGui::End();
    }
  }
}

void start_overlay() {
  overlay_t = true;

  Overlay ov1 = Overlay();
  printf(XorStr("Waiting for The Extra Ban .... Never Gonna Get it!\n"));

  std::thread ui_thr = ov1.Start();
  ui_thr.detach();

  // while (check == 0xABCD) {
  //   printf(XorStr("Never Gonna Get it!\n"));
  //   std::this_thread::sleep_for(std::chrono::seconds(1));
  // }
  ready = true;

  while (overlay_t) {
    // if (IsKeyDown(ImGuiKey_F4)) {
    //   active = false;
    //   break;
    // }
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
  ready = false;
  ov1.Clear();
}

// int main(int argc, char **argv) {
//   start_overlay();
//   return 0;
// }