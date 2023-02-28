
#define VERSION STEAM
//Date 2/16/2023
//GameVersion=v3.0.27.29

#if VERSION == STEAM

#define OFFSET_ENTITYLIST 0x1b37a78 //cl_entitylist
#define OFFSET_LOCAL_ENT 0x01ee8cb0 + 0x8 //LocalPlayer  might be moved to  AVC_GameMovement
#define OFFSET_NAME_LIST 0xbcc7e50 //NameList
#define OFFSET_THIRDPERSON 0x01b1c810 + 0x6c //thirdperson_override + 0x6c
#define OFFSET_TIMESCALE 0x014d2a40 //host_timescale

#define OFFSET_TEAM 0x044c //m_iTeamNum
#define OFFSET_HEALTH 0x043c //m_iHealth
#define OFFSET_SHIELD 0x0170 //m_shieldHealth
#define OFFSET_MAXSHIELD 0x0174 //m_shieldHealthMax
#define OFFSET_ARMORTYPE 0x4634 //armortype
#define OFFSET_NAME 0x0589 //m_iName
#define OFFSET_SIGN_NAME 0x0580 //m_iSignifierName
#define OFFSET_ABS_VELOCITY 0x0140 //m_vecAbsVelocity
#define OFFSET_VISIBLE_TIME 0x1A78 //CPlayer!lastVisibleTime
#define OFFSET_ZOOMING 0x1c51 //m_bZooming
#define OFFSET_THIRDPERSON_SV 0x36c8 //m_thirdPersonShoulderView
#define OFFSET_YAW 0x22b4 - 0x8 //m_currentFramePlayer.m_ammoPoolCount - 0x8

#define OFFSET_LIFE_STATE 0x0798 //m_lifeState, >0 = dead
#define OFFSET_BLEED_OUT_STATE 0x2740 //m_bleedoutState, >0 = knocked

#define OFFSET_ORIGIN 0x014c //m_vecAbsOrigin - 3rd offset after the first int and vector


#define OFFSET_BONES 0x0e98 + 0x48 //m_nForceBone + 0x48
#define OFFSET_STUDIOHDR 0x10f0 //CBaseAnimating!m_pStudioHdr


#define OFFSET_AIMPUNCH 0x24b0 //m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one
#define OFFSET_CAMERAPOS 0x1f48 //CPlayer!camera_origin
#define OFFSET_VIEWANGLES 0x25ac - 0x14 //m_ammoPoolCapacity - 0x14
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x34d4 //m_iObserverMode
#define OFFSET_OBSERVING_TARGET 0x34e0 //m_hObserverTarget

#define OFFSET_MATRIX 0x11a210 //ViewMatrix
#define OFFSET_RENDER 0x7677d20 //ViewRender

#define OFFSET_WEAPON 0x1a1c //m_latestPrimaryWeapons
#define OFFSET_WEAPON_NAME 0x1844 //m_weaponNameIndex 2nd one
#define OFFSET_BULLET_SPEED 0x1f18 //CWeaponX!m_flProjectileSpeed
#define OFFSET_BULLET_SCALE 0x1f20 //CWeaponX!m_flProjectileScale
#define OFFSET_ZOOM_FOV 0x16c0 + 0xb8 //m_playerData + m_curZoomFOV
#define OFFSET_AMMO 0x1644 //m_ammoInClip first offset 

#define OFFSET_ITEM_GLOW 0x02c0 //m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1628 // item id?
#define OFFSET_MODELNAME 0x0030 // m_ModelName
#define OFFSET_M_CUSTOMSCRIPTINT 0x1648 //m_customScriptInt
#define OFFSET_MINIMAP_SCALE 0x45f8 //m_minimapTargetZoomScale, float

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
#define OFFSET_GAMEMODE 			0x01f21a70 //mp_gamemode

#endif