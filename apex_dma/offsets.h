//Date 1/10/2024
//GameVersion=v3.0.52.26

#define OFFSET_ENTITYLIST 0x1db1cf8 //cl_entitylist  updated 1/10/2024
#define OFFSET_LOCAL_ENT 0x21603f8 //LocalPlayer updated 1/10/2024
#define OFFSET_NAME_LIST 0xc450df0 //NameList updated 1/10/2024
#define OFFSET_GLOBAL_VARS 0x16d55b0 // GlobalVars updated 1/10/2024
 
#define OFFSET_LEVELNAME 0x16d5ab0 //LevelName //updated 1/10/2024
#define OFFSET_CLIENTSTATE OFFSET_LEVELNAME - 0x1c0 //ClientState  ClientState = LevelName - 0x1C0  //updated 1/10/2024
#define OFFSET_SIGNONSTATE OFFSET_CLIENTSTATE + 0xAC //SignonState   ( ClientState + 0xAC)  //updated 1/10/2024
#define OFFSET_HOST_MAP 0x015ede90 + 0x58 //host_map + 0x58 //updated 1/10/2024
 
#define OFFSET_TEAM 0x030c //m_iTeamNum//updated 1/10/2024
#define OFFSET_HEALTH 0x02fc //m_iHealth//updated 1/10/2024
#define OFFSET_SHIELD 0x01a0 //m_shieldHealth//updated 1/10/2024
#define OFFSET_MAXSHIELD 0x01a4 //m_shieldHealthMax//updated 1/10/2024
#define OFFSET_ARMORTYPE 0x45cc //m_armorType //updated 1/10/2024
#define OFFSET_NAME 0x0449 //m_iName //updated 1/10/2024
#define OFFSET_SIGN_NAME 0x0440 //m_iSignifierName//updated 1/10/2024
#define OFFSET_ABS_VELOCITY 0x0170 //m_vecAbsVelocity //updated 1/10/2024
#define OFFSET_VISIBLE_TIME 0x1950 //CPlayer!lastVisibleTime  //updated 1/10/2024
#define OFFSET_ZOOMING 0x1b91 //m_bZooming //updated 1/10/2024
#define OFFSET_TRAVERSAL_PROGRESS 0x2abc //m_traversalProgress //updated 1/10/2024
#define OFFSET_TRAVERSAL_STARTTIME 0x2ac0 //m_traversalStartTime //updated 1/10/2024
#define OFFSET_PLATFORM_UID 0x2508 //m_platformUserId //updated 1/10/2024

#define OFFSET_WEAPON_NAME 0x1738 //DT_WeaponX.m_weaponNameIndex //updated 1/10/2024
#define OFFSET_OFF_WEAPON 0x1904 //C_BaseCombatCharacter.m_latestNonOffhandWeapons //updated 1/10/2024
#define OFFSET_WALL_RUN_START_TIME 0x3524 //m_wallRunStartTime //updated 1/10/2024
#define OFFSET_WALL_RUN_CLEAR_TIME 0x3528 //m_wallRunClearTime //updated 1/10/2024
#define OFFSET_FLAGS 0x00c8 //m_fFlags //updated 1/10/2024
#define OFFSET_IN_ATTACK 0x073cc520 // in_attack //updated 1/10/2024
#define OFFSET_IN_TOGGLE_DUCK 0x073cc560 //in_toggle_duck //updated 1/10/2024
#define OFFSET_IN_ZOOM 0x073cc6b0 //in_zoom //updated 1/10/2024
#define OFFSET_IN_FORWARD 0x073cc470 //in_forward //updated 1/10/2024
#define OFFSET_IN_JUMP 0x073cc630 //in_jump //updated 1/10/2024
#define OFFSET_IN_DUCK 0x073cc720 //in_duck //updated 1/10/2024
#define OFFSET_IN_USE 0x073cc6a0 //in_use //updated 1/10/2024

#define OFFSET_LIFE_STATE 0x0658 //m_lifeState, >0 = dead //updated 1/10/2024
#define OFFSET_BLEED_OUT_STATE 0x26a0 //m_bleedoutState, >0 = knocked //updated 1/10/2024

#define OFFSET_VIEW_OFFSET 0x00e8 //m_currentFrame.viewOffset //updated 1/10/2024
#define OFFSET_ORIGIN 0x017c //m_vecAbsOrigin - 3rd offset after the first int and vector //updated 1/10/2024
#define OFFSET_BONES 0x0d60 + 0x48 //m_nForceBone + 0x48 //updated 1/10/2024
#define OFFSET_STUDIOHDR 0xfb0 //CBaseAnimating!m_pStudioHdr //updated 1/10/2024
#define OFFSET_AIMPUNCH 0x23f8 //m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one//updated 1/10/2024
#define OFFSET_CAMERAPOS 0x1e90 //CPlayer!camera_origin//updated 1/10/2024
#define OFFSET_VIEWANGLES 0x24f4 - 0x14 //m_ammoPoolCapacity - 0x14//updated 1/10/2024
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x3454 //m_iObserverMode//updated 1/10/2024
#define OFFSET_OBSERVING_TARGET 0x3460 //m_hObserverTarget//updated 1/10/2024


#define OFFSET_MATRIX 0x11a350 //ViewMatrix//updated 1/10/2024
#define OFFSET_RENDER 0x73cb268 //ViewRender displays ESp, heath dist names etc //updated 1/10/2024
 
#define OFFSET_PRIMARY_WEAPON 0x18f4 //m_latestPrimaryWeapons //updated 1/10/2024
#define OFFSET_ACTIVE_WEAPON 0x1888 + 0x0058 // m_inventory + WeaponInventory_Client>activeWeapons //updated 1/10/2024
#define OFFSET_BULLET_SPEED 0x1e5c //CWeaponX!m_flProjectileSpeed maybe its WeaponSettings.projectile_launch_speed now //updated 1/10/2024
#define OFFSET_BULLET_SCALE 0x1e64 //CWeaponX!m_flProjectileScale maybe its WeaponSettings.projectile_gravity_scale now //updated 1/10/2024
#define OFFSET_ZOOM_FOV 0x1590 + 0x00b8 //m_playerData + m_curZoomFOV //updated 1/10/2024
#define OFFSET_AMMO 0x1514 //m_ammoInClip first offset //updated 1/10/2024

#define OFFSET_MODELNAME 0x0030 // m_ModelName //updated 1/10/2024
#define OFFSET_M_CUSTOMSCRIPTINT 0x1518 //m_customScriptInt //updated 1/10/2024
#define OFFSET_YAW 0x21fc - 0x8 //m_currentFramePlayer.m_ammoPoolCount//updated 1/10/2024 - 0x8 
#define OFFSET_TIME_BASE 0x2048 //m_currentFramePlayer.timeBase //updated 1/10/2024
#define OFFSET_VIEW_MODELS 0x2ce0 //m_hViewModels //updated 1/10/2024
#define OFFSET_CROSSHAIR_LAST 0x1958 //CWeaponX!lastCrosshairTargetTime // CPlayer!lastVisibleTime + 0x8 // updated 1/10/2024
#define OFFSET_INPUT_SYSTEM 0x1754a80 //InputSystem // updated 1/10/2024 
#define OFFSET_WEAPON_BITFIELD 0x16c4 // m_modBitfieldFromPlayer // updated 1/10/2024

#define HIGHLIGHT_SETTINGS 0xB7A4E90 //HighlightSettings  // updated 1/10/2024
#define OFFSET_GLOW_CONTEXT 0x28c // updated 1/10/2024
#define OFFSET_GLOW_THROUGH_WALLS 0x26c // updated 1/10/2024
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE OFFSET_GLOW_THROUGH_WALLS
#define OFFSET_GLOW_FIX 0x268 // updated 1/10/2024
// Mode: HighlightSettings + 0x34 * Context + 0x0
// Color: HighlightSettings + 0x34 * Context + 0x4

// #define GLOW_LIFE_TIME 0x3A4+ 0x30 // Script_Highlight_SetLifeTime + 4
#define GLOW_DISTANCE 0x26c // Script_Highlight_SetFarFadeDist
// #define GLOW_TYPE 0x29c // Script_Highlight_GetState + 4
// #define GLOW_COLOR 0x1D0+ 0x30 // Script_CopyHighlightState 15th mov
// #define GLOW_FADE 0x388+ 0x30 // Script_Highlight_GetCurrentInsideOpacity 3rd result of 3 offsets consecutive or first + 8

// #define HIGHLIGHT_TYPE_SIZE 0x28 //?
// #define OFFSET_ITEM_GLOW 0x02f0 //m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1518 // item id?      //updated 11/1/2023
// #define GLOW_START_TIME 0x02c8 + 0x30 //m_playerFloatLookStartTime=0x02c8 //updated 1/10/2024 + 0x30
// #define OFFSET_HIGHLIGHTSERVERACTIVESTATES 0x0298 //m_highlightServerActiveStates //updated 11/20/2023
// #define OFFSET_GLOW_ENABLE_GLOW_CONTEXT OFFSET_GLOW_ENABLE // Script_Highlight_SetCurrentContext