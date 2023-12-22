#include "apex_sky.h"
#include "prediction.h"
#include "vector.h"
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <iostream>
#include <ostream>
#include <thread>

extern Memory apex_mem;

float bulletspeed = 0.08;
float bulletgrav = 0.05;

// glowtype not used, but dont delete its still used.
extern int glowtype;
extern int glowtype2;
// setting up vars, dont edit
extern float veltest;
extern Vector aim_target;
extern int local_held_id;
extern uint32_t local_weapon_id;

bool Entity::Observing(uint64_t entitylist) {
  return *(bool *)(buffer + OFFSET_OBSERVER_MODE);
}

// https://github.com/CasualX/apexbot/blob/master/src/state.cpp#L104
void get_class_name(uint64_t entity_ptr, char *out_str) {
  uint64_t client_networkable_vtable;
  apex_mem.Read<uint64_t>(entity_ptr + 8 * 3, client_networkable_vtable);

  uint64_t get_client_class;
  apex_mem.Read<uint64_t>(client_networkable_vtable + 8 * 3, get_client_class);

  uint32_t disp;
  apex_mem.Read<uint32_t>(get_client_class + 3, disp);
  const uint64_t client_class_ptr = get_client_class + disp + 7;

  ClientClass client_class;
  apex_mem.Read<ClientClass>(client_class_ptr, client_class);

  apex_mem.ReadArray<char>(client_class.pNetworkName, out_str, 32);
}

int Entity::getTeamId() { return *(int *)(buffer + OFFSET_TEAM); }

int Entity::getHealth() { return *(int *)(buffer + OFFSET_HEALTH); }
// seer health and shield i added

int Entity::getArmortype() {
  int armortype;
  apex_mem.Read<int>(ptr + OFFSET_ARMORTYPE, armortype);
  return armortype;
}

int Entity::getShield() { return *(int *)(buffer + OFFSET_SHIELD); }

int Entity::getMaxshield() { return *(int *)(buffer + OFFSET_MAXSHIELD); }

Vector Entity::getAbsVelocity() {
  return *(Vector *)(buffer + OFFSET_ABS_VELOCITY);
}

Vector Entity::getPosition() { return *(Vector *)(buffer + OFFSET_ORIGIN); }
Vector Entity::getViewOffset() {
  return *(Vector *)(buffer + OFFSET_VIEW_OFFSET);
}

bool Entity::isPlayer() {
  return *(uint64_t *)(buffer + OFFSET_NAME) == 125780153691248;
}
// firing range dummys
bool Entity::isDummy() {
  char class_name[33] = {};
  get_class_name(ptr, class_name);

  return strncmp(class_name, "CAI_BaseNPC", 11) == 0;
}

bool Entity::isKnocked() {
  return *(int *)(buffer + OFFSET_BLEED_OUT_STATE) > 0;
}

bool Entity::isAlive() { return *(int *)(buffer + OFFSET_LIFE_STATE) == 0; }

float Entity::lastVisTime() { return *(float *)(buffer + OFFSET_VISIBLE_TIME); }

float Entity::lastCrossHairTime() {
  return *(float *)(buffer + OFFSET_CROSSHAIR_LAST);
}

Vector Entity::getBonePositionByHitbox(int id) {
  Vector origin = getPosition();

  // BoneByHitBox
  uint64_t Model = *(uint64_t *)(buffer + OFFSET_STUDIOHDR);

  // get studio hdr
  uint64_t StudioHdr;
  apex_mem.Read<uint64_t>(Model + 0x8, StudioHdr);

  // get hitbox array
  uint16_t HitboxCache;
  apex_mem.Read<uint16_t>(StudioHdr + 0x34, HitboxCache);
  uint64_t HitboxArray =
      StudioHdr + ((uint16_t)(HitboxCache & 0xFFFE) << (4 * (HitboxCache & 1)));

  uint16_t IndexCache;
  apex_mem.Read<uint16_t>(HitboxArray + 0x4, IndexCache);
  int HitboxIndex = ((uint16_t)(IndexCache & 0xFFFE) << (4 * (IndexCache & 1)));

  uint16_t Bone;
  apex_mem.Read<uint16_t>(HitboxIndex + HitboxArray + (id * 0x20), Bone);

  if (Bone < 0 || Bone > 255)
    return Vector();

  // hitpos
  uint64_t Bones = *(uint64_t *)(buffer + OFFSET_BONES);

  matrix3x4_t Matrix = {};
  apex_mem.Read<matrix3x4_t>(Bones + Bone * sizeof(matrix3x4_t), Matrix);

  return Vector(Matrix.m_flMatVal[0][3] + origin.x,
                Matrix.m_flMatVal[1][3] + origin.y,
                Matrix.m_flMatVal[2][3] + origin.z);
}

QAngle Entity::GetSwayAngles() {
  return *(QAngle *)(buffer + OFFSET_BREATH_ANGLES);
}

QAngle Entity::GetViewAngles() {
  return *(QAngle *)(buffer + OFFSET_VIEWANGLES);
}

Vector Entity::GetViewAnglesV() {
  return *(Vector *)(buffer + OFFSET_VIEWANGLES);
}

float Entity::GetYaw() {
  float yaw = 0;
  apex_mem.Read<float>(ptr + OFFSET_YAW, yaw);

  if (yaw < 0)
    yaw += 360;
  yaw += 90;
  if (yaw > 360)
    yaw -= 360;

  return yaw;
}

bool Entity::isGlowing() { return *(int *)(buffer + OFFSET_GLOW_ENABLE) == 7; }

bool Entity::isZooming() { return *(int *)(buffer + OFFSET_ZOOMING) == 1; }

extern uint64_t g_Base;

void Entity::enableGlow(int context_id, int setting_index, uint8_t inside_value,
                        uint8_t outline_size,
                        std::array<float, 3> highlight_parameter) {
  // static const int contextId = 5;
  // int settingIndex = 44;
  const unsigned char outsidevalue = 125;
  std::array<unsigned char, 4> highlightFunctionBits = {
      inside_value, // InsideFunction
      outsidevalue, // OutlineFunction: HIGHLIGHT_OUTLINE_OBJECTIVE
      outline_size, // OutlineRadius: size * 255 / 8
      64 // (EntityVisible << 6) | State & 0x3F | (AfterPostProcess << 7)
  };
  apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, context_id);
  apex_mem.Write<unsigned char>(
      ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + context_id, setting_index);
  for (int context_id = 1; context_id < 5; context_id++) {
    apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, context_id);
    apex_mem.Write<unsigned char>(
        ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + context_id, setting_index);
  }
  apex_mem.Write<int>(ptr + OFFSET_GLOW_THROUGH_WALLS, 2);

  long highlight_settings_ptr;
  apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlight_settings_ptr);
  apex_mem.Write<typeof(highlightFunctionBits)>(
      highlight_settings_ptr + HIGHLIGHT_TYPE_SIZE * setting_index + 4,
      highlightFunctionBits);
  apex_mem.Write<typeof(highlight_parameter)>(
      highlight_settings_ptr + HIGHLIGHT_TYPE_SIZE * setting_index + 8,
      highlight_parameter);
  // Fix highlight Wraith and Ashe's disappear
  // apex_mem.Write(g_Base + 0x270, 1);
  // apex_mem.Write(ptr + 0x270, 1);
}
void Entity::disableGlow() {

  // apex_mem.Write<int>(ptr + OFFSET_GLOW_T1, 0);
  // apex_mem.Write<int>(ptr + OFFSET_GLOW_T2, 0);
  // apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, 2);
  // apex_mem.Write<int>(ptr + OFFSET_GLOW_THROUGH_WALLS, 5);
  // apex_mem.Write<float>(ptr + GLOW_COLOR_R, 0.0f);
  // apex_mem.Write<float>(ptr + GLOW_COLOR_G, 0.0f);
  // apex_mem.Write<float>(ptr + GLOW_COLOR_B, 0.0f);
  // apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, 2);
  // apex_mem.Write<int>(ptr + OFFSET_GLOW_THROUGH_WALLS, 5);
}

void Entity::SetViewAngles(SVector angles) {
  apex_mem.Write<SVector>(ptr + OFFSET_VIEWANGLES, angles);
}

void Entity::SetViewAngles(QAngle &angles) { SetViewAngles(SVector(angles)); }

Vector Entity::GetCamPos() { return *(Vector *)(buffer + OFFSET_CAMERAPOS); }

QAngle Entity::GetRecoil() { return *(QAngle *)(buffer + OFFSET_AIMPUNCH); }

void Entity::get_name(uint64_t g_Base, uint64_t index, char *name) {
  index *= 0x10;
  uint64_t name_ptr = 0;
  apex_mem.Read<uint64_t>(g_Base + OFFSET_NAME_LIST + index, name_ptr);
  apex_mem.ReadArray<char>(name_ptr, name, 32);
}

void Entity::glow_weapon_model(uint64_t g_Base, bool enable_glow,
                               std::array<float, 3> highlight_colors) {
  uint64_t view_model_handle;
  apex_mem.Read<uint64_t>(ptr + OFFSET_VIEW_MODELS, view_model_handle);
  view_model_handle &= 0xFFFF;
  uint64_t view_model_ptr = 0;
  apex_mem.Read<uint64_t>(g_Base + OFFSET_ENTITYLIST + (view_model_handle << 5),
                          view_model_ptr);

  // printf("view model handle=%lu, ptr=%lu, \n", view_model_handle,
  // view_model_ptr);

  // uint64_t name_ptr;
  // char name_str[200];
  // apex_mem.Read<uint64_t>(view_model_ptr + OFFSET_MODELNAME, name_ptr);
  // apex_mem.ReadArray<char>(name_ptr, name_str, 200);
  // printf("name=%s\n", name_str);

  std::array<unsigned char, 4> highlightFunctionBits = {0, 125, 64, 64};
  int setting_index = 99;
  if (!enable_glow) {
    highlightFunctionBits = {0, 125, 0, 64};
    setting_index = 0;
  }

  for (int context_id = 1; context_id < 5; context_id++) {
    apex_mem.Write<int>(view_model_ptr + OFFSET_GLOW_ENABLE, context_id);
    apex_mem.Write<unsigned char>(
        view_model_ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + context_id,
        setting_index);
  }

  long highlightSettingsPtr;
  apex_mem.Read<long>(g_Base + HIGHLIGHT_SETTINGS, highlightSettingsPtr);
  apex_mem.Write<typeof(highlightFunctionBits)>(
      highlightSettingsPtr + 40 * setting_index + 4, highlightFunctionBits);
  apex_mem.Write<typeof(highlight_colors)>(
      highlightSettingsPtr + 40 * setting_index + 8, highlight_colors);

  // Fix highlight Wraith and Ashe's disappear
  // apex_mem.Write(ptr + 0x270, 1);
  // int val1;
  // apex_mem.Read(ptr + 0x270, val1);
  // printf("0x270=%d\n", val1);
}

bool Entity::check_love_player(uint64_t entity_index) {
  if (global_settings().yuan_p) {
    if (this->isDummy())
      return true;
  } else {
    if (!this->isPlayer())
      return false;
  }
  uint64_t data_fid[4];
  data_fid[0] = *((uint64_t *)(buffer + OFFSET_PLATFORM_UID + 0));
  data_fid[1] = *((uint64_t *)(buffer + OFFSET_PLATFORM_UID + 4));
  data_fid[2] = *((uint64_t *)(buffer + OFFSET_PLATFORM_UID + 16));
  data_fid[3] = *((uint64_t *)(buffer + OFFSET_PLATFORM_UID + 20));
  uint64_t platform_lid = data_fid[0] | data_fid[1] << 32;
  uint64_t eadp_lid = data_fid[1] | data_fid[2] << 32;
  char name[33] = {0};
  this->get_name(g_Base, entity_index - 1, &name[0]);
  return ::check_love_player(platform_lid, eadp_lid, name);
}

// Items
bool Item::isItem() {
  char class_name[33] = {};
  get_class_name(ptr, class_name);

  return strncmp(class_name, "CPropSurvival", 13) == 0;
}
// Deathboxes
bool Item::isBox() {
  char class_name[33] = {};
  get_class_name(ptr, class_name);

  return strncmp(class_name, "CDeathBoxProp", 13) == 0;
}
// Traps
bool Item::isTrap() {
  char class_name[33] = {};
  get_class_name(ptr, class_name);

  return strncmp(class_name, "caustic_trap", 13) == 0;
}

bool Item::isGlowing() {
  return *(int *)(buffer + OFFSET_ITEM_GLOW) == 1363184265;
}

void Item::enableGlow() {
  /* for (int i = 1; i <= 70; i++) {
  // Write the current value to the memory locations
  apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, 6);
  apex_mem.Write<int>(ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + 2, 6);

  // Print the current value
  std::cout << "Value: " << i << std::endl;

  // Sleep for 2 seconds
  std::this_thread::sleep_for(std::chrono::seconds(5));
} */
  /* //apex_mem.Write<GlowMode>(ptr + GLOW_TYPE, { 101,102,96,90 });
      apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, 6);
      apex_mem.Write<int>(ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + 6, 6);
  apex_mem.Write<int>(ptr + OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE , 2); */

  // apex_memRead<uint32_t>(entity + OFFSET_HIGHLIGHTCURRENTCONTEXTID);
}

void Item::disableGlow() {
  apex_mem.Write<int>(ptr + OFFSET_GLOW_ENABLE, 0);
  apex_mem.Write<int>(ptr + OFFSET_HIGHLIGHTSERVERACTIVESTATES + 0, 0);
  apex_mem.Write<int>(ptr + OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE, 5);
}

Vector Item::getPosition() { return *(Vector *)(buffer + OFFSET_ORIGIN); }

float CalculateFov(Entity &from, Entity &target) {
  QAngle ViewAngles = from.GetSwayAngles();
  Vector LocalCamera = from.GetCamPos();
  Vector EntityPosition = target.getPosition();
  QAngle Angle = Math::CalcAngle(LocalCamera, EntityPosition);
  return Math::GetFov(ViewAngles, Angle);
}

auto fun_calc_angles = [](Vector LocalCameraPosition, Vector TargetBonePosition,
                          Vector targetVel, float BulletSpeed, float BulletGrav,
                          float deltaTime) {
  QAngle CalculatedAngles = QAngle(0, 0, 0);
  if (BulletSpeed > 1.f) {

    PredictCtx Ctx;
    Ctx.StartPos = LocalCameraPosition;
    Ctx.TargetPos = TargetBonePosition;
    Ctx.BulletSpeed = BulletSpeed - (BulletSpeed * bulletspeed);
    Ctx.BulletGravity = BulletGrav + (BulletGrav * bulletgrav);

    // Add the target's velocity to the prediction context, with an offset
    // in the y direction
    float distanceToTarget =
        (TargetBonePosition - LocalCameraPosition).Length();
    float timeToTarget = distanceToTarget / BulletSpeed;
    Vector targetPosAhead = TargetBonePosition + (targetVel * timeToTarget);
    Ctx.TargetVel =
        Vector(targetVel.x, targetVel.y + (targetVel.Length() * deltaTime),
               targetVel.z);
    Ctx.TargetPos = targetPosAhead;

    aim_target.x = Ctx.TargetPos.x;
    aim_target.y = Ctx.TargetPos.y;
    aim_target.z = Ctx.TargetPos.z;

    if (BulletPredict(Ctx))
      CalculatedAngles = QAngle{Ctx.AimAngles.x, Ctx.AimAngles.y, 0.f};
  }

  if (CalculatedAngles == QAngle(0, 0, 0))
    CalculatedAngles = Math::CalcAngle(LocalCameraPosition, TargetBonePosition);
  return CalculatedAngles;
};

QAngle CalculateBestBoneAim(Entity &from, Entity &target, float max_fov) {
  const auto g_settings = global_settings();
  if (g_settings.firing_range) {
    if (!target.isAlive()) {
      return QAngle(0, 0, 0);
    }
  } else {
    if (!target.isAlive() || target.isKnocked()) {
      return QAngle(0, 0, 0);
    }
  }

  WeaponXEntity curweap = WeaponXEntity();
  curweap.update(from.ptr);
  uint32_t weap_id = curweap.get_weap_id();
  float BulletSpeed = curweap.get_projectile_speed();
  float BulletGrav = curweap.get_projectile_gravity();

  float zoom_fov = curweap.get_zoom_fov();
  if (zoom_fov != 0.0f && zoom_fov != 1.0f) {
    max_fov *= zoom_fov / 90.0f;
  }

  Vector LocalCamera = from.GetCamPos();
  QAngle ViewAngles = from.GetViewAngles();
  QAngle SwayAngles = from.GetSwayAngles();
  Vector targetVel = target.getAbsVelocity();

  // Find best bone
  bool weap_headshot;
  switch (weap_id) {
  case idweapon_3030_repeater:
  case idweapon_bow:
  case idweapon_charge_rifle:
  case idweapon_g7_scout:
  case idweapon_kraber:
  case idweapon_longbow:
  case idweapon_sentinel:
  case idweapon_triple_take:
  case idweapon_wingman:
    weap_headshot = true;
    break;
  default:
    weap_headshot = false;
  }

  Vector TargetBonePositionMin;
  Vector TargetBonePositionMax;

  // Calculate the time since the last frame (in seconds)
  float deltaTime = 1.0 / g_settings.game_fps;

  if (weap_headshot) {
    if (LocalCamera.DistTo(target.getPosition()) <= g_settings.headshot_dist) {
      TargetBonePositionMax = TargetBonePositionMin =
          target.getBonePositionByHitbox(0);
    } else {
      TargetBonePositionMax = TargetBonePositionMin =
          target.getBonePositionByHitbox(g_settings.bone);
    }
  } else if (g_settings.bone_nearest) {
    // find nearest bone
    float NearestBoneDistance = g_settings.max_dist;
    for (int i = 0; i < 4; i++) {
      Vector currentBonePosition = target.getBonePositionByHitbox(i);
      float DistanceFromCrosshair =
          (currentBonePosition - LocalCamera).Length();
      if (DistanceFromCrosshair < NearestBoneDistance) {
        TargetBonePositionMax = TargetBonePositionMin = currentBonePosition;
        NearestBoneDistance = DistanceFromCrosshair;
      }
    }
  } else if (g_settings.bone_auto) {
    TargetBonePositionMax = target.getBonePositionByHitbox(5);
    TargetBonePositionMin = target.getBonePositionByHitbox(0);
  } else {
    TargetBonePositionMax = TargetBonePositionMin =
        target.getBonePositionByHitbox(g_settings.bone);
  }

  if (local_held_id != -251) {
    QAngle CalculatedAnglesMin =
        fun_calc_angles(LocalCamera, TargetBonePositionMin, targetVel,
                        BulletSpeed, BulletGrav, deltaTime);
    QAngle CalculatedAnglesMax =
        fun_calc_angles(LocalCamera, TargetBonePositionMax, targetVel,
                        BulletSpeed, BulletGrav, deltaTime);

    double fov0 = Math::GetFov(SwayAngles, CalculatedAnglesMin);
    double fov1 = Math::GetFov(SwayAngles, CalculatedAnglesMax);
    if ((fov0 + fov1) * 0.5f > max_fov) {
      return QAngle(0, 0, 0);
    }
    if (g_settings.aim_no_recoil) {
      CalculatedAnglesMin -= SwayAngles - ViewAngles;
      CalculatedAnglesMax -= SwayAngles - ViewAngles;
    }
    Math::NormalizeAngles(CalculatedAnglesMin);
    Math::NormalizeAngles(CalculatedAnglesMax);
    QAngle DeltaMin = CalculatedAnglesMin - ViewAngles;
    QAngle DeltaMax = CalculatedAnglesMax - ViewAngles;
    Math::NormalizeAngles(DeltaMin);
    Math::NormalizeAngles(DeltaMax);

    QAngle Delta = QAngle(0, 0, 0);
    if (DeltaMin.x * DeltaMax.x > 0)
      Delta.x = (DeltaMin.x + DeltaMax.x) * 0.5f;
    if (DeltaMin.y * DeltaMax.y > 0)
      Delta.y = (DeltaMin.y + DeltaMax.y) * 0.5f;

    QAngle SmoothedAngles = ViewAngles + Delta / g_settings.smooth;
    return SmoothedAngles;
  } else {
    int weapon_mod_bitfield = curweap.get_mod_bitfield();
    Vector local_origin = from.getPosition();
    Vector view_offset = from.getViewOffset();
    // printf("view_offset(%f,%f,%f)\n", view_offset.x, view_offset.y,
    //        view_offset.z);
    Vector view_origin = local_origin + view_offset;
    Vector target_origin = target.getPosition() + targetVel * deltaTime;
    aim_target = target_origin;
    vector2d_t skynade_angles =
        skynade_angle(weap_id, weapon_mod_bitfield, BulletGrav / 750.0f,
                      BulletSpeed, view_origin.x, view_origin.y, view_origin.z,
                      target_origin.x, target_origin.y, target_origin.z);

    // printf("(%.1f, %.1f)\n", ViewAngles.x, ViewAngles.y);
    if (skynade_angles.x == 0 && skynade_angles.y == 0) {
      return ViewAngles;
    }

    const float PIS_IN_180 = 57.2957795130823208767981548141051703f;
    QAngle TargetAngles = QAngle(-skynade_angles.x * PIS_IN_180,
                                 skynade_angles.y * PIS_IN_180, 0);
    // printf("weap=%d, bitfield=%d, (%.1f, %.1f)\n", weapon_id,
    //        weapon_mod_bitfield, TargetAngles.x, TargetAngles.y);

    QAngle Delta = TargetAngles - ViewAngles;
    return ViewAngles + Delta / g_settings.skynade_smooth;
  }
}

Entity getEntity(uintptr_t ptr) {
  Entity entity = Entity();
  entity.ptr = ptr;
  apex_mem.ReadArray<uint8_t>(ptr, entity.buffer, sizeof(entity.buffer));
  return entity;
}

Item getItem(uintptr_t ptr) {
  Item entity = Item();
  entity.ptr = ptr;
  apex_mem.ReadArray<uint8_t>(ptr, entity.buffer, sizeof(entity.buffer));
  return entity;
}

bool WorldToScreen(Vector from, float *m_vMatrix, int targetWidth,
                   int targetHeight, Vector &to) {
  float w = m_vMatrix[12] * from.x + m_vMatrix[13] * from.y +
            m_vMatrix[14] * from.z + m_vMatrix[15];

  if (w < 0.01f)
    return false;

  to.x = m_vMatrix[0] * from.x + m_vMatrix[1] * from.y + m_vMatrix[2] * from.z +
         m_vMatrix[3];
  to.y = m_vMatrix[4] * from.x + m_vMatrix[5] * from.y + m_vMatrix[6] * from.z +
         m_vMatrix[7];

  float invw = 1.0f / w;
  to.x *= invw;
  to.y *= invw;

  float x = targetWidth / 2.0;
  float y = targetHeight / 2.0;

  x += 0.5 * to.x * targetWidth + 0.5;
  y -= 0.5 * to.y * targetHeight + 0.5;

  to.x = x;
  to.y = y;
  to.z = 0;
  return true;
}

void WeaponXEntity::update(uint64_t LocalPlayer) {
  extern uint64_t g_Base;
  uint64_t entitylist = g_Base + OFFSET_ENTITYLIST;
  uint64_t wephandle = 0;
  apex_mem.Read<uint64_t>(LocalPlayer + OFFSET_ACTIVE_WEAPON, wephandle);

  wephandle &= 0xffff;

  uint64_t wep_entity = 0;
  apex_mem.Read<uint64_t>(entitylist + (wephandle << 5), wep_entity);

  projectile_speed = 0;
  apex_mem.Read<float>(wep_entity + OFFSET_BULLET_SPEED, projectile_speed);
  projectile_scale = 0;
  apex_mem.Read<float>(wep_entity + OFFSET_BULLET_SCALE, projectile_scale);
  zoom_fov = 0;
  apex_mem.Read<float>(wep_entity + OFFSET_ZOOM_FOV, zoom_fov);
  ammo = 0;
  apex_mem.Read<int>(wep_entity + OFFSET_AMMO, ammo);
  memset(name_str, 0, sizeof(name_str));
  uint64_t name_ptr;
  apex_mem.Read<uint64_t>(wep_entity + OFFSET_MODELNAME, name_ptr);
  apex_mem.ReadArray<char>(name_ptr, name_str, 200);
  mod_bitfield = 0;
  apex_mem.Read<int>(wep_entity + OFFSET_WEAPON_BITFIELD, mod_bitfield);
  weap_id = 0;
  apex_mem.Read<uint32_t>(wep_entity + OFFSET_WEAPON_NAME, weap_id);
}

float WeaponXEntity::get_projectile_speed() { return projectile_speed; }

float WeaponXEntity::get_projectile_gravity() {
  return 750.0f * projectile_scale;
}

float WeaponXEntity::get_zoom_fov() { return zoom_fov; }

int WeaponXEntity::get_ammo() { return ammo; }

const char *WeaponXEntity::get_name_str() { return name_str; }

int WeaponXEntity::get_mod_bitfield() { return mod_bitfield; }

uint32_t WeaponXEntity::get_weap_id() { return weap_id; }