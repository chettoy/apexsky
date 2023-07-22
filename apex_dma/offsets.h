
#define VERSION STEAM
//Date 5/28/2023
//GameVersion=v3.0.38.32

#if VERSION == STEAM

#define OFFSET_ENTITYLIST 0x1e54dc8 //cl_entitylist
#define OFFSET_LOCAL_ENT 0x22048C8 + 0x8 //LocalPlayer  might be moved to  AVC_GameMovement
#define OFFSET_NAME_LIST 0xbe95d60 //NameList


#define OFFSET_LEVELNAME 0x16fb240 //LevelName
#define OFFSET_CLIENTSTATE 0x16fb080 //ClientState
#define OFFSET_SIGNONSTATE 0x16fb12c //SignonState


#define OFFSET_TEAM 0x044c //m_iTeamNum
#define OFFSET_HEALTH 0x043c //m_iHealth
#define OFFSET_SHIELD 0x0170 //m_shieldHealth
#define OFFSET_MAXSHIELD 0x0174 //m_shieldHealthMax
#define OFFSET_ARMORTYPE 0x4654 //armortype
#define OFFSET_NAME 0x0589 //m_iName
#define OFFSET_SIGN_NAME 0x0580 //m_iSignifierName
#define OFFSET_ABS_VELOCITY 0x0140 //m_vecAbsVelocity
#define OFFSET_VISIBLE_TIME 0x1a70 //CPlayer!lastVisibleTime
#define OFFSET_ZOOMING 0x1c51 //m_bZooming
#define OFFSET_THIRDPERSON_SV 0x36e8 //m_thirdPersonShoulderView
#define OFFSET_YAW 0x22bc - 0x8 //m_currentFramePlayer.m_ammoPoolCount - 0x8

//superglide?
#define OFFSET_TRAVERSAL_PROGRESS 0x2b5c // m_traversalProgress
//#define OFFSET_TRAVERSAL_PROGRESS 0x2b60 // m_traversalStartTime
#define OFFSET_FORCE_JUMP 0x0743e5a0 // in_jump
#define OFFSET_FORCE_DUCK_TOGGLE 0x0be967c8 // in_duck
//#define OFFSET_TRAVERSAL_PROGRESS 0x2108  //m_currentFramePlayer.timeBase
#define OFFSET_CURRENT_FRAME 0x16fad90 + 0x8 // GlobalVars
#define OFFSET_IN_ATTACK 0x0743e510 // in_attack
#define OFFSET_IN_TOGGLE_DUCK 0x0c295450 //in_toggle_duck
#define OFFSET_IN_ZOOM 0x0be968e0 //in_zoom
#define OFFSET_IN_SPEED 0x07871c18 //in_speed
#define OFFSET_WEAPON_NAME 0x1874 //m_weaponNameIndex 2nd one
#define OFFSET_LIFE_STATE 0x0798 //m_lifeState, >0 = dead
#define OFFSET_BLEED_OUT_STATE 0x2750 //m_bleedoutState, >0 = knocked

#define OFFSET_ORIGIN 0x014c //m_vecAbsOrigin - 3rd offset after the first int and vector
#define OFFSET_BONES 0x0e98 + 0x48 //m_nForceBone + 0x48
#define OFFSET_STUDIOHDR 0x10e8 //CBaseAnimating!m_pStudioHdr
#define OFFSET_AIMPUNCH 0x24b8 //m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one
#define OFFSET_CAMERAPOS 0x1f50 //CPlayer!camera_origin
#define OFFSET_VIEWANGLES 0x25b4 - 0x14 //m_ammoPoolCapacity - 0x14
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x34f4 //m_iObserverMode
#define OFFSET_OBSERVING_TARGET 0x3500 //m_hObserverTarget

#define OFFSET_MATRIX 0x11A350 //ViewMatrix
#define OFFSET_RENDER 0x743BCA0 //ViewRender displays ESp, heath dist names etc

#define OFFSET_WEAPON 0x1a14 //m_latestPrimaryWeapons
#define OFFSET_BULLET_SPEED 0x04cc //CWeaponX!m_flProjectileSpeed  maybe its  WeaponSettings.projectile_launch_speed now
#define OFFSET_BULLET_SCALE 0x04d4 //CWeaponX!m_flProjectileScale  maybe its  WeaponSettings.projectile_gravity_scale now
#define OFFSET_ZOOM_FOV 0x16b0 + 0xb8 //m_playerData + m_curZoomFOV
#define OFFSET_AMMO 0x1634 //m_ammoInClip first offset 

#define OFFSET_ITEM_GLOW 0x2c0 //m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1628 // item id?
#define OFFSET_MODELNAME 0x0030 // m_ModelName
#define OFFSET_M_CUSTOMSCRIPTINT 0x1638 //m_customScriptInt
#define OFFSET_MINIMAP_SCALE 0x4638 //m_minimapTargetZoomScale, float

#define OFFSET_GLOW_T1 0x262 //16256 = enabled, 0 = disabled
#define OFFSET_GLOW_T2 0x2dc //1193322764 = enabled, 0 = disabled
#define OFFSET_GLOW_ENABLE 0x3c8 //7 = enabled, 2 = disabled
#define OFFSET_GLOW_THROUGH_WALLS 0x3d0 //2 = enabled, 5 = disabled
	
#define GLOW_COLOR_R 0x1D0 
#define GLOW_COLOR_G 0x1D4 
#define GLOW_COLOR_B 0x1D8
#define GLOW_START_TIME 0x2c4 //m_playerFloatLookStartTime=0x02c4


#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT                 OFFSET_GLOW_ENABLE // Script_Highlight_SetCurrentContext
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE     OFFSET_GLOW_THROUGH_WALLS // Script_Highlight_SetVisibilityType 5th mov
#define GLOW_LIFE_TIME              0x3A4 // Script_Highlight_SetLifeTime + 4
#define GLOW_DISTANCE               0x3B4 // Script_Highlight_SetFarFadeDist
#define GLOW_TYPE                   0x2C4 // Script_Highlight_GetState + 4
#define GLOW_COLOR                  0x1D0 // Script_CopyHighlightState 15th mov
#define GLOW_FADE                   0x388 // Script_Highlight_GetCurrentInsideOpacity 3rd result of 3 offsets consecutive or first + 8    
//triggerbot?
#define CROSSHAIRSTART				0x1a74 // 	crosshairTargetStartTime
#define CROSSHAIR_TARGET_TIME				0x1a78 // 	lastCrosshairTargetTime


#endif