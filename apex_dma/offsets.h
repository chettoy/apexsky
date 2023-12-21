//Date 11/21/2023
//GameVersion=v3.0.51.45

#define OFFSET_ENTITYLIST 0x1d71858 //cl_entitylist  updated 11/20/2023
#define OFFSET_LOCAL_ENT 0x211fac8 //LocalPlayer might be moved to AVC_GameMovement updated 11/20/2023
#define OFFSET_NAME_LIST 0xc275bf0 //NameList updated 11/20/2023
#define OFFSET_GLOBAL_VARS 0x16961f0 // GlobalVars updated 11/20/2023
 
#define OFFSET_LEVELNAME 0x16966f0 //LevelName //updated 11/20/2023
#define OFFSET_CLIENTSTATE OFFSET_LEVELNAME - 0x1c0 //ClientState  ClientState = LevelName - 0x1C0  //updated 11/20/2023
#define OFFSET_SIGNONSTATE OFFSET_CLIENTSTATE + 0xAC //SignonState   ( ClientState + 0xAC)  //updated 11/20/2023
#define OFFSET_HOST_MAP 0x015aece0 + 0x58 //host_map + 0x58 //updated 11/20/2023
 
#define OFFSET_TEAM 0x037c //m_iTeamNum//updated 11/20/2023
#define OFFSET_HEALTH 0x036c //m_iHealth//updated 11/20/2023
#define OFFSET_SHIELD 0x01a0 //m_shieldHealth//updated 11/20/2023
#define OFFSET_MAXSHIELD 0x01a4 //m_shieldHealthMax//updated 11/20/2023
#define OFFSET_ARMORTYPE 0x4634 //m_armorType //updated 11/20/2023
#define OFFSET_NAME 0x04b9 //m_iName //updated 11/20/2023
#define OFFSET_SIGN_NAME 0x04b0 //m_iSignifierName//updated 11/20/2023
#define OFFSET_ABS_VELOCITY 0x0170 //m_vecAbsVelocity //updated 11/20/2023
#define OFFSET_VISIBLE_TIME 0x19c0 //CPlayer!lastVisibleTime  //updated 11/20/2023
#define OFFSET_ZOOMING 0x1c01 //m_bZooming //updated 11/20/2023
#define OFFSET_TRAVERSAL_PROGRESS 0x2b2c //m_traversalProgress //updated 11/20/2023
#define OFFSET_TRAVERSAL_STARTTIME 0x2b30 //m_traversalStartTime //updated 11/20/2023
#define OFFSET_PLATFORM_UID 0x2578 //m_platformUserId //updated 11/20/2023

#define OFFSET_WEAPON_NAME 0x17a8 //DT_WeaponX.m_weaponNameIndex 2nd one //updated 11/20/2023
#define OFFSET_OFF_WEAPON 0x1974 //C_BaseCombatCharacter.m_latestNonOffhandWeapons //updated 11/20/2023
#define OFFSET_WALL_RUN_START_TIME 0x3594 //m_wallRunStartTime
#define OFFSET_WALL_RUN_CLEAR_TIME 0x3598 //m_wallRunClearTime
#define OFFSET_FLAGS 0x00c8 //m_fFlags //updated 11/20/2023
#define OFFSET_IN_ATTACK 0x0738af90 // in_attack //updated 11/20/2023
#define OFFSET_IN_TOGGLE_DUCK 0x0738afd0 //in_toggle_duck //updated 11/20/2023
#define OFFSET_IN_ZOOM 0x0738b120 //in_zoom //updated 11/20/2023
#define OFFSET_IN_FORWARD 0x0738b1d0 //in_forward //updated 11/20/2023
#define OFFSET_IN_JUMP 0x0738b0a0 //in_jump //updated 11/20/2023
#define OFFSET_IN_DUCK 0x0738b190 //in_duck //updated 11/20/2023

#define OFFSET_LIFE_STATE 0x06c8 //m_lifeState, >0 = dead //updated 11/20/2023
#define OFFSET_BLEED_OUT_STATE 0x2710 //m_bleedoutState, >0 = knocked //updated 11/20/2023

#define OFFSET_VIEW_OFFSET 0x00e8 //m_currentFrame.viewOffset //updated 11/20/2023
#define OFFSET_ORIGIN 0x017c //m_vecAbsOrigin - 3rd offset after the first int and vector //updated 11/20/2023
#define OFFSET_BONES 0x0dd0 + 0x48 //m_nForceBone + 0x48 //updated 11/20/2023
#define OFFSET_STUDIOHDR 0x1020 //CBaseAnimating!m_pStudioHdr //updated 11/20/2023
#define OFFSET_AIMPUNCH 0x2468 //m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one//updated 11/20/2023
#define OFFSET_CAMERAPOS 0x1f00 //CPlayer!camera_origin//updated 11/20/2023
#define OFFSET_VIEWANGLES 0x2564 - 0x14 //m_ammoPoolCapacity - 0x14//updated 11/20/2023
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x34c4 //m_iObserverMode//updated 11/20/2023
#define OFFSET_OBSERVING_TARGET 0x34d0 //m_hObserverTarget//updated 11/20/2023
#define OFFSET_IN_USE 0x0738b110 //in_use //updated 11/20/2023


#define OFFSET_MATRIX 0x11a350 //ViewMatrix//updated 11/20/2023
#define OFFSET_RENDER 0x7389d40 //ViewRender displays ESp, heath dist names etc //updated 11/20/2023
 
#define OFFSET_PRIMARY_WEAPON 0x1964 //m_latestPrimaryWeapons //updated 11/20/2023
#define OFFSET_ACTIVE_WEAPON 0x18f8 + 0x58 // m_inventory + activeWeapon //updated 11/20/2023
#define OFFSET_BULLET_SPEED 0x1ebc //CWeaponX!m_flProjectileSpeed maybe its WeaponSettings.projectile_launch_speed now //updated 11/20/2023
#define OFFSET_BULLET_SCALE 0x1ec4 //CWeaponX!m_flProjectileScale maybe its WeaponSettings.projectile_gravity_scale now //updated 11/20/2023
#define OFFSET_ZOOM_FOV 0x1600 + 0x00b8 //m_playerData + m_curZoomFOV //updated 11/20/2023
#define OFFSET_AMMO 0x1584 //m_ammoInClip first offset //updated 11/20/2023
 
#define OFFSET_ITEM_GLOW 0x02f0 //m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1588 // item id?      //updated 11/1/2023
#define OFFSET_MODELNAME 0x0030 // m_ModelName //updated 11/20/2023
#define OFFSET_M_CUSTOMSCRIPTINT 0x1588 //m_customScriptInt //updated 11/20/2023
#define OFFSET_YAW 0x226c - 0x8 //m_currentFramePlayer.m_ammoPoolCount - 0x8 //updated 11/20/2023
#define OFFSET_TIME_BASE 0x20b8 //m_currentFramePlayer.timeBase //updated 11/20/2023
#define OFFSET_VIEW_MODELS 0x2d50 //m_hViewModels //updated 11/20/2023
 
//#define OFFSET_GLOW_T1 0x262+ 0x30 //16256 = enabled, 0 = disabled
//#define OFFSET_GLOW_T2 0x2dc+ 0x30 //1193322764 = enabled, 0 = disabled
#define OFFSET_GLOW_ENABLE 0x294 //7 = enabled, 2 = disabled
#define OFFSET_GLOW_THROUGH_WALLS 0x278 //2 = enabled, 5 = disabled
 

#define GLOW_START_TIME 0x02c8 + 0x30 //m_playerFloatLookStartTime=0x02c8 //updated 11/20/2023
#define OFFSET_HIGHLIGHTSERVERACTIVESTATES 0x0298 //m_highlightServerActiveStates //updated 11/20/2023
 
#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT OFFSET_GLOW_ENABLE // Script_Highlight_SetCurrentContext
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE OFFSET_GLOW_THROUGH_WALLS // Script_Highlight_SetVisibilityType 5th mov
#define GLOW_LIFE_TIME 0x3A4+ 0x30 // Script_Highlight_SetLifeTime + 4
#define GLOW_DISTANCE 0x26c // Script_Highlight_SetFarFadeDist
#define GLOW_TYPE 0x29c // Script_Highlight_GetState + 4
#define GLOW_COLOR 0x1D0+ 0x30 // Script_CopyHighlightState 15th mov
#define GLOW_FADE 0x388+ 0x30 // Script_Highlight_GetCurrentInsideOpacity 3rd result of 3 offsets consecutive or first + 8
#define HIGHLIGHT_SETTINGS 0xb5cc530 //HighlightSettings  //updated 11/20/2023
#define HIGHLIGHT_TYPE_SIZE 0x28 //?
#define OFFSET_CROSSHAIR_LAST 0x19c8 //CWeaponX!lastCrosshairTargetTime // CPlayer!lastVisibleTime + 0x8 // updated 11/20/2023
//#define OFFSET_CROSSHAIR_START 0x1A84 //CPlayer!crosshairTargetStartTime
#define OFFSET_INPUT_SYSTEM 0x17151c0 //InputSystem // updated 11/20/2023 
#define OFFSET_WEAPON_BITFIELD 0x1734 // m_modBitfieldFromPlayer // updated 11/20/2023