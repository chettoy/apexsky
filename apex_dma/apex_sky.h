#pragma once

#include <cstdint>

typedef struct {
  float x;
  float y;
} vector2d_t;

extern "C" {
void print_run_as_root();
uint32_t add(uint32_t lhs, uint32_t rhs);

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