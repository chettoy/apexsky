#pragma once

#include "../apex_sky.h"
#include "../lib/xorstr/xorstr.hpp"
#include "../vector.h"
#include "imgui.h"
#include <chrono>
#include <cwchar>
#include <stdlib.h>
#include <string>
#include <thread>
#include <vector>

#define GREEN ImColor(0, 255, 0)
#define RED ImColor(255, 0, 0)
#define BLUE ImColor(0, 0, 255)
#define ORANGE ImColor(255, 165, 0)
#define WHITE ImColor(255, 255, 255)
#define TEAL ImColor(0, 128, 128)
#define YELLOW ImColor(255, 255, 0)

class Overlay {
public:
  std::thread Start();
  int CreateOverlay();
  void Clear();
  int getWidth();
  int getHeight();
  void RenderInfo();
  void RenderMenu();
  void RenderEsp();
  void DrawLine(ImVec2 a, ImVec2 b, ImColor color, float width);
  void DrawBox(ImColor color, float x, float y, float w, float h,
               float line_w = 1.0f);
  void Text(ImVec2 pos, ImColor color, const char *text_begin,
            const char *text_end, float wrap_width,
            const ImVec4 *cpu_fine_clip_rect);
  void RectFilled(float x0, float y0, float x1, float y1, ImColor color,
                  float rounding, int rounding_corners_flags);
  void ProgressBar(float x, float y, float w, float h, int value, int v_max);
  void String(ImVec2 pos, ImColor color, const char *text);
  // Seer
  void DrawSeerLikeHealth(float x, float y, int shield, int max_shield,
                          int armorType, int health);

private:
  bool running;
};
