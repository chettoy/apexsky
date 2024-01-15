#include "main.h"
#include "../Game.h"
#include "imgui.h"
#include "overlay.h"
#include <cstddef>
#include <cstdio>
#include <map>
#include <random>
#include <string>
#include <thread>
#include <vector>
typedef Vector D3DXVECTOR3;

typedef uint8_t *PBYTE;
typedef uint8_t BYTE;
typedef unsigned long DWORD;
typedef unsigned short WORD;
typedef WORD *PWORD;

uint32_t check = 0xABCD;

// Left and Right Aim key toggle
extern bool active; // sync
bool ready = false;
extern uint64_t g_Base; // write sync

extern std::vector<TreasureClue> treasure_clues;

extern float bulletspeed; // sync
extern float bulletgrav;  // sync
float veltest = 1.00;     // sync

// Full Map Radar
extern bool mainradartoggle; // Toggle for Main Map radar
bool kingscanyon = false;    // Set for map, ONLY ONE THO
bool stormpoint = true;      // Set for map, ONLY ONE THO

// Map number
extern int map;

extern bool valid; // write sync
extern bool next2; // read write sync

Vector aim_target = Vector(0, 0, 0);
extern const aimbot_state_t aimbot; // read

extern bool overlay_t;

extern std::vector<player> players;
extern Matrix view_matrix_data;
extern Vector esp_local_pos;
std::vector<std::string> esp_spec_names, teammates_damage;

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
static void TeamN(int x, int y, int w, int h, RGBA color) {
  ImGui::GetWindowDrawList()->AddRectFilled(
      ImVec2(x, y), ImVec2(x + w, y + h),
      ImGui::ColorConvertFloat4ToU32(ImVec4(color.R / 255.0, color.G / 255.0,
                                            color.B / 255.0, color.A / 255.0)),
      0, 0);
}

static void TeamMiniMap(int x, int y, int radius, int team_id,
                        float targetyaw) {
  RGBA2 color;
  auto it = teamColors.find(team_id);
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
    teamColors[team_id] = color;
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
  ImGui::GetWindowDrawList()->AddCircle(
      center, radius, colOutline, 12,
      global_settings().mini_map_radar_dot_size2);

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
                           float LocalPlayerY, float eneamyDist, int team_id,
                           int xAxis, int yAxis, int width, int height,
                           D3DXCOLOR color, float targetyaw) {
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
      TeamMiniMap(single.x, single.y,
                  global_settings().mini_map_radar_dot_size1, team_id,
                  targetyaw);
    }
  }
}

void draw_team_point(int pos_x, int pos_y, int team_id) {
  const auto g_settings = global_settings();
  if (team_id == 1) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {255, 255, 255, 255});
  }
  if (team_id == 2) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {242, 86, 38, 255});
  }
  if (team_id == 3) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {242, 86, 38, 255});
  }
  if (team_id == 4) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {174, 247, 89, 255});
  }
  if (team_id == 5) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {102, 214, 173, 255});
  }
  if (team_id == 6) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {98, 244, 234, 255});
  }
  if (team_id == 7) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {92, 208, 250, 255});
  }
  if (team_id == 8) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {93, 137, 238, 255});
  }
  if (team_id == 9) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {164, 105, 252, 255});
  }
  if (team_id == 10) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {243, 98, 161, 255});
  }
  if (team_id == 11) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {214, 67, 67, 255});
  }
  if (team_id == 12) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {230, 116, 51, 255});
  }
  if (team_id == 13) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {185, 179, 167, 255});
  }
  if (team_id == 14) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {148, 200, 65, 255});
  }
  if (team_id == 15) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {86, 174, 91, 255});
  }
  if (team_id == 16) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {55, 188, 200, 255});
  }
  if (team_id == 17) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {84, 169, 212, 255});
  }
  if (team_id == 18) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {98, 121, 203, 255});
  }
  if (team_id == 19) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {102, 61, 174, 255});
  }
  if (team_id >= 20 && team_id < 36) {
    TeamN(pos_x, pos_y, g_settings.main_map_radar_dot_size1,
          g_settings.main_map_radar_dot_size2, {218, 73, 145, 255});
  }
}

void DrawRadarPoint(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos,
                    float LocalPlayerY, float eneamyDist, int team_id,
                    int xAxis, int yAxis, int width, int height,
                    D3DXCOLOR color) {
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
    draw_team_point(single.x, single.y, team_id);
  }
}
// MiniMap Radar Stuff
void MiniMapRadar(D3DXVECTOR3 EneamyPos, D3DXVECTOR3 LocalPos,
                  float LocalPlayerY, float eneamyDist, int team_id,
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
      ImVec2 DrawPos = ImGui::GetCursorScreenPos();
      ImVec2 DrawSize = ImGui::GetContentRegionAvail();
      ImVec2 midRadar =
          ImVec2(DrawPos.x + (DrawSize.x / 2), DrawPos.y + (DrawSize.y / 2));
      if (global_settings().mini_map_guides) {
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
                            team_id, DrawPos.x, DrawPos.y, DrawSize.x,
                            DrawSize.y, {255, 255, 255, 255}, targetyaw);
    }
    ImGui::End();
  }
  ImGui::PopStyleColor();
}

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
void worldToScreenMap(D3DXVECTOR3 origin, int team_id) {
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
  } else if (map == 2) { // KingsCanyon
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
    return;
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

  draw_team_point(pos_x, pos_y, team_id);
}

void Overlay::RenderEsp() {
  next2 = false;
  const auto g_settings = global_settings();
  if (g_Base != 0 && g_settings.esp) {

    players.clear();
    esp_spec_names.clear();
    teammates_damage.clear();

    ImGui::SetNextWindowPos(ImVec2(0, 0));
    ImGui::SetNextWindowSize(ImVec2((float)getWidth(), (float)getHeight()));
    ImGui::Begin(XorStr("##esp"), (bool *)true,
                 ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize |
                     ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoScrollbar |
                     ImGuiWindowFlags_NoBackground |
                     ImGuiWindowFlags_NoBringToFrontOnFocus |
                     ImGuiWindowFlags_NoInputs);

    if (g_settings.show_aim_target && aim_target != Vector(0, 0, 0)) {
      Vector bs = Vector();
      WorldToScreen(aim_target, view_matrix_data.matrix, getWidth(),
                    getHeight(), bs);
      const float indicator_radius = 10.0f;
      bool aimbot_locked = aimbot_is_locked(&aimbot);
      ImColor indicator_color = aimbot_locked
                                    ? ImColor(1.0f, 0.647f, 0.0f, 0.618f)
                                    : ImColor(1.0f, 1.0f, 1.0f, 0.618f);
      ImVec2 p1 = ImVec2(bs.x + indicator_radius, bs.y - indicator_radius);
      ImVec2 p2 = ImVec2(bs.x - indicator_radius, bs.y - indicator_radius);
      ImVec2 p3 = ImVec2(bs.x - indicator_radius, bs.y + indicator_radius);
      ImVec2 p4 = ImVec2(bs.x + indicator_radius, bs.y + indicator_radius);
      ImDrawList &draw_list = *ImGui::GetWindowDrawList();
      draw_list.AddRect(p2, p4, indicator_color, indicator_radius, 0, 1.6726f);
      if (aimbot_locked) {
        indicator_color = RED;
        draw_list.AddLine(p1, p3, indicator_color, 2.718f);
        draw_list.AddLine(p2, p4, indicator_color, 2.718f);
      }
    }

    if (treasure_clues.size() > 0) {
      Vector bs_loot, bs_local;
      WorldToScreen(esp_local_pos, view_matrix_data.matrix, getWidth(),
                    getHeight(), bs_local);
      if (!(bs_local.x == 0 && bs_local.y == 0)) {
        for (size_t i = 0; i < treasure_clues.size(); i++) {
          TreasureClue clue = treasure_clues[i];
          if (clue.position == Vector(0, 0, 0))
            continue;
          // printf("%f,%f,%f\n",
          // clue.position.x,clue.position.y,clue.position.z);
          WorldToScreen(clue.position, view_matrix_data.matrix, getWidth(),
                        getHeight(), bs_loot);
          if (bs_loot.x == 0 && bs_loot.y == 0)
            continue;
          DrawLine(ImVec2(bs_local.x, bs_local.y), ImVec2(bs_loot.x, bs_loot.y),
                   ImColor(1.0f, 1.0f, 1.0f, 0.5f), 1.0f);
          std::string distance = std::to_string(clue.distance / 39.62);
          distance = std::to_string(clue.item_id) + "(" +
                     distance.substr(0, distance.find('.')) + "m)";
          String(ImVec2(bs_loot.x, bs_loot.y), ImColor(212, 175, 55),
                 distance.c_str());
        }
      }
    }

    if (!g_settings.firing_range)
      while (!next2 && g_settings.esp) {
        std::this_thread::sleep_for(std::chrono::milliseconds(2));
      }

    if (next2 && valid) {

      for (int i = 0; i < players.size(); i++) {
        if (players[i].is_spectator) {
          esp_spec_names.push_back(std::string(players[i].name));
        }
        if (players[i].is_teammate) {
          teammates_damage.push_back(std::string(players[i].name) + " " +
                                     std::to_string(players[i].damage));
        }

        if (!players[i].is_alive) {
          continue;
        }

        if (players[i].health > 0) {
          if (g_settings.esp_visuals.damage &&
              players[i].dist < g_settings.aimbot_settings.aim_dist) {
            ImColor color = ImColor(188, 18, 20);
            ImVec2 draw_pos = ImVec2(players[i].boxMiddle,
                                     (players[i].b_y - players[i].height - 32));
            std::string damage_str = std::to_string(players[i].damage);
            String(draw_pos, color, damage_str.c_str());
          }

          if (players[i].is_teammate && !g_settings.onevone) {
            continue;
          }

          float alpha; // The farther away, the more transparent
          if (players[i].dist < g_settings.aimbot_settings.aim_dist) {
            alpha = 1.0f;
          } else if (players[i].dist > 16000.0f) {
            alpha = 0.4f;
          } else {
            alpha = 1.0f -
                    ((players[i].dist - g_settings.aimbot_settings.aim_dist) /
                     (16000.0f - g_settings.aimbot_settings.aim_dist) * 0.6f);
          }

          float radardistance = (int)(players[i].dist / 39.62);

          // Radar Stuff
          if (g_settings.mini_map_radar == true) {
            MiniMapRadar(players[i].EntityPosition,
                         players[i].LocalPlayerPosition,
                         players[i].localviewangle.y, radardistance,
                         players[i].entity_team, players[i].targetyaw);
          }
          if (g_settings.esp_visuals.line) {
            DrawLine(ImVec2((float)(getWidth() / 2.0), (float)getHeight()),
                     ImVec2(players[i].b_x, players[i].b_y), BLUE,
                     1); // LINE FROM MIDDLE SCREEN
          }

          if (g_settings.esp_visuals.distance) {
            std::string distance = std::to_string(players[i].dist / 39.62);
            distance = distance.substr(0, distance.find('.')) + "m(" +
                       std::to_string(players[i].entity_team) + ")";
            if (players[i].knocked) {
              String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)), RED,
                     distance.c_str()); // DISTANCE
            } else {
              String(ImVec2(players[i].boxMiddle, (players[i].b_y + 1)),
                     ImColor(0.0f, 1.0f, 0.0f, alpha),
                     distance.c_str()); // DISTANCE
            }
          }

          if (players[i].dist < g_settings.aimbot_settings.aim_dist) {
            if (g_settings.esp_visuals.healthbar)
              DrawSeerLikeHealth(
                  (players[i].b_x - (players[i].width / 2.0f) + 5),
                  (players[i].b_y - players[i].height - 10), players[i].shield,
                  players[i].maxshield, players[i].armortype,
                  players[i].health); // health bar

            if (g_settings.esp_visuals.box) {
              ImColor box_color = ImColor(0.0f, 0.0f, 0.0f, alpha);
              float box_width = 1.0f;
              if (players[i].visible) {
                box_color =
                    ImColor(g_settings.glow_r_viz, g_settings.glow_g_viz,
                            g_settings.glow_b_viz, alpha);
                box_width = 2.0f;
              } else {
                box_color =
                    ImColor(g_settings.glow_r_not, g_settings.glow_g_not,
                            g_settings.glow_b_not, alpha);
              }
              DrawBox(box_color, players[i].b_x - (players[i].width / 2.0f),
                      players[i].b_y - players[i].height, players[i].width,
                      players[i].height, box_width);
            }
            if (g_settings.esp_visuals.name) {
              ImColor name_color;
              if (players[i].is_love == LOVE) {
                name_color = ImColor(231, 27, 100);
              } else if (players[i].is_love == HATE) {
                name_color = ImColor(1.0f, .0f, .0f);
              } else if (players[i].is_love == AMBIVALENT) {
                name_color = ImColor(.0f, .0f, .0f);
              } else {
                name_color = ImColor(1.0f, 1.0f, 1.0f, alpha);
              }
              ImVec2 draw_pos =
                  ImVec2(players[i].boxMiddle,
                         (players[i].b_y - players[i].height - 15));
              ImVec2 nick_pos = ImVec2(draw_pos.x + 50, draw_pos.y);
              std::string level =
                  std::string("Lv.") + std::to_string(players[i].xp_level);

              String(draw_pos, ImColor(.0f, 1.0f, .0f, alpha), level.c_str());
              String(nick_pos, name_color, players[i].name);
            }
          }
          // Full Radar map, Need Manual setting of cords
          if (g_settings.main_radar_map == true)

            worldToScreenMap(players[i].EntityPosition, players[i].entity_team);

          // String(ImVec2(players[i].boxMiddle, (players[i].b_y -
          // players[i].height - 15)), WHITE, players[i].name);
        }
      }
    }
    ImGui::End();
  }
}

void start_overlay() {
  overlay_t = true;

  Overlay ov1 = Overlay();
  // std::thread ui_thr = ov1.Start();
  // ui_thr.detach();
  ov1.CreateOverlay();
  overlay_t = false;
}

// int main(int argc, char **argv) {
//   start_overlay();
//   return 0;
// }