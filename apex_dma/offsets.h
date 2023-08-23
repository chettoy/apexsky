#define VERSION STEAM
//Date 8/23/2023
//GameVersion=v3.0.41.34

#if VERSION == STEAM

#define OFFSET_ENTITYLIST 0x1e74448 //cl_entitylist
#define OFFSET_LOCAL_ENT 0x22245c8 //LocalPlayer  might be moved to  AVC_GameMovement
#define OFFSET_NAME_LIST 0xc2b0c00 //NameList


#define OFFSET_LEVELNAME 0x16eed90 //LevelName
#define OFFSET_CLIENTSTATE 0x16eebd0 //ClientState
#define OFFSET_SIGNONSTATE 0x16eec7c //SignonState


#define OFFSET_TEAM 0x0480 //m_iTeamNum
#define OFFSET_HEALTH 0x0470 //m_iHealth
#define OFFSET_SHIELD 0x01a0 //m_shieldHealth
#define OFFSET_MAXSHIELD 0x01a4 //m_shieldHealthMax
#define OFFSET_ARMORTYPE 0x4694 //armortype
#define OFFSET_NAME 0x05c1 //m_iName
#define OFFSET_SIGN_NAME 0x05b8 //m_iSignifierName
#define OFFSET_ABS_VELOCITY 0x0170 //m_vecAbsVelocity
#define OFFSET_VISIBLE_TIME 0x1AA0 //CPlayer!lastVisibleTime
#define OFFSET_ZOOMING 0x1c81 //m_bZooming

#define OFFSET_WEAPON_NAME 0x1888 //m_weaponNameIndex 2nd one

#define OFFSET_IN_ATTACK 0x07472f98 // in_attack
#define OFFSET_IN_TOGGLE_DUCK 0x07472fd8 //in_toggle_duck
#define OFFSET_IN_ZOOM 0x07473138 //in_zoom

#define OFFSET_LIFE_STATE 0x07d0 //m_lifeState, >0 = dead
#define OFFSET_BLEED_OUT_STATE 0x2790 //m_bleedoutState, >0 = knocked

#define OFFSET_ORIGIN 0x017c //m_vecAbsOrigin - 3rd offset after the first int and vector
#define OFFSET_BONES 0x0ec8 + 0x48 //m_nForceBone + 0x48
#define OFFSET_STUDIOHDR 0x1118 //CBaseAnimating!m_pStudioHdr
#define OFFSET_AIMPUNCH 0x24e8 //m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one
#define OFFSET_CAMERAPOS 0x1f80 //CPlayer!camera_origin
#define OFFSET_VIEWANGLES 0x25e4 - 0x14 //m_ammoPoolCapacity - 0x14
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x3534 //m_iObserverMode
#define OFFSET_OBSERVING_TARGET 0x3540 //m_hObserverTarget

#define OFFSET_MATRIX 0x11A350 //ViewMatrix
#define OFFSET_RENDER 0x7472A28 //ViewRender displays ESp, heath dist names etc

#define OFFSET_WEAPON 0x1a44 //m_latestPrimaryWeapons
#define OFFSET_BULLET_SPEED OFFSET_VISIBLE_TIME + 0x04cc //CWeaponX!m_flProjectileSpeed  maybe its  WeaponSettings.projectile_launch_speed now
#define OFFSET_BULLET_SCALE OFFSET_VISIBLE_TIME + 0x4d4 //CWeaponX!m_flProjectileScale  maybe its  WeaponSettings.projectile_gravity_scale now
#define OFFSET_ZOOM_FOV 0x16e0 + 0xb8 //m_playerData + m_curZoomFOV
#define OFFSET_AMMO 0x1664 //m_ammoInClip first offset 

#define OFFSET_ITEM_GLOW 0x02f0 //m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1628 // item id?
#define OFFSET_MODELNAME 0x0030 // m_ModelName
#define OFFSET_M_CUSTOMSCRIPTINT 0x1668  //m_customScriptInt
#define OFFSET_YAW 0x22bc - 0x8 //m_currentFramePlayer.m_ammoPoolCount - 0x8

#define OFFSET_GLOW_T1 0x262+ 0x30 //16256 = enabled, 0 = disabled
#define OFFSET_GLOW_T2 0x2dc+ 0x30 //1193322764 = enabled, 0 = disabled
#define OFFSET_GLOW_ENABLE 0x3c8+ 0x30 //7 = enabled, 2 = disabled
#define OFFSET_GLOW_THROUGH_WALLS 0x3d0+ 0x30 //2 = enabled, 5 = disabled
	
#define GLOW_COLOR_R 0x1D0 + 0x30
#define GLOW_COLOR_G 0x1D4 + 0x30
#define GLOW_COLOR_B 0x1D8+ 0x30
#define GLOW_START_TIME 0x2c4+ 0x30 //m_playerFloatLookStartTime=0x02c4


#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT                 OFFSET_GLOW_ENABLE // Script_Highlight_SetCurrentContext
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE     OFFSET_GLOW_THROUGH_WALLS // Script_Highlight_SetVisibilityType 5th mov
#define GLOW_LIFE_TIME              0x3A4+ 0x30 // Script_Highlight_SetLifeTime + 4
#define GLOW_DISTANCE               0x3B4+ 0x30 // Script_Highlight_SetFarFadeDist
#define GLOW_TYPE                   0x2C4+ 0x30 // Script_Highlight_GetState + 4
#define GLOW_COLOR                  0x1D0+ 0x30 // Script_CopyHighlightState 15th mov
#define GLOW_FADE                   0x388+ 0x30 // Script_Highlight_GetCurrentInsideOpacity 3rd result of 3 offsets consecutive or first + 8    
//triggerbot?
#define CROSSHAIRSTART				0x1a74 // 	crosshairTargetStartTime
#define CROSSHAIR_TARGET_TIME				0x1a78 // 	lastCrosshairTargetTime


#endif