// Date 10/31/2023
// GameVersion=v3.0.48.27
// Krackerco's Fork

#define OFFSET_ENTITYLIST 0x1d6a5d8 // cl_entitylist
#define OFFSET_LOCAL_ENT                                                       \
  0x2118848 // LocalPlayer might be moved to AVC_GameMovement
#define OFFSET_NAME_LIST 0xc26d750 // NameList
#define OFFSET_GLOBAL_VARS 0x168f170

#define OFFSET_LEVELNAME 0x168F670   // LevelName
#define OFFSET_CLIENTSTATE 0x168F4B0 // ClientState
#define OFFSET_SIGNONSTATE 0x168F55C // SignonState

#define OFFSET_TEAM 0x037c               // m_iTeamNum
#define OFFSET_HEALTH 0x036c             // m_iHealth
#define OFFSET_SHIELD 0x01a0             // m_shieldHealth
#define OFFSET_MAXSHIELD 0x01a4          // m_shieldHealthMax
#define OFFSET_ARMORTYPE 0x4634          // m_armorType
#define OFFSET_NAME 0x04b9               // m_iName
#define OFFSET_SIGN_NAME 0x04b0          // m_iSignifierName
#define OFFSET_ABS_VELOCITY 0x0170       // m_vecAbsVelocity
#define OFFSET_VISIBLE_TIME 0x19c0       // CPlayer!lastVisibleTime
#define OFFSET_ZOOMING 0x1c01            // m_bZooming
#define OFFSET_FORCE_DUCK 0x07382CF0     // in_duck
#define OFFSET_TRAVERSAL_PROGRESS 0x2b2c // m_traversalProgress
#define OFFSET_FORCE_JUMP 0x07382BF0     // in_jump

#define OFFSET_WEAPON_NAME 0x17a8         // m_weaponNameIndex 2nd one
#define OFFSET_OFF_WEAPON 0x1974          // m_latestNonOffhandWeapons
#define OFFSET_IN_ATTACK 0x07382AF0       // in_attack
#define OFFSET_IN_TOGGLE_DUCK 0x07382CF0  // in_toggle_duck
#define OFFSET_IN_ZOOM 0x07382C90         // in_zoom
#define OFFSET_WALL_RUN_START_TIME 0x3594 // m_wallRunStartTime
#define OFFSET_WALL_RUN_CLEAR_TIME 0x3598 // m_wallRunClearTime
#define OFFSET_FLAGS 0x00c8               // m_fFlags
#define OFFSET_IN_FORWARD 0x07382D30      // in_forward

#define OFFSET_LIFE_STATE 0x06c8      // m_lifeState, >0 = dead
#define OFFSET_BLEED_OUT_STATE 0x2710 // m_bleedoutState, >0 = knocked

#define OFFSET_ORIGIN                                                          \
  0x017c // m_vecAbsOrigin - 3rd offset after the first int and vector
#define OFFSET_BONES 0x0dd0 + 0x48 // m_nForceBone + 0x48
#define OFFSET_STUDIOHDR 0x1020    // CBaseAnimating!m_pStudioHdr
#define OFFSET_AIMPUNCH                                                        \
  0x2468 // m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle, first one
#define OFFSET_CAMERAPOS 0x1f00         // CPlayer!camera_origin
#define OFFSET_VIEWANGLES 0x2564 - 0x14 // m_ammoPoolCapacity - 0x14
#define OFFSET_BREATH_ANGLES OFFSET_VIEWANGLES - 0x10
#define OFFSET_OBSERVER_MODE 0x34c4    // m_iObserverMode
#define OFFSET_OBSERVING_TARGET 0x34d0 // m_hObserverTarget
#define OFFSET_IN_USE 0x07382c7        // in_use

#define OFFSET_MATRIX 0x11a350  // ViewMatrix
#define OFFSET_RENDER 0x73818a0 // ViewRender displays ESp, heath dist names etc

#define OFFSET_WEAPON 0x1964 // m_latestPrimaryWeapons
#define OFFSET_BULLET_SPEED                                                    \
  0x19E8 + 0x04d4 // CWeaponX!m_flProjectileSpeed maybe its
                  // WeaponSettings.projectile_launch_speed now
#define OFFSET_BULLET_SCALE                                                    \
  0x19E8 + 0x04dc // CWeaponX!m_flProjectileScale maybe its
                  // WeaponSettings.projectile_gravity_scale now
#define OFFSET_ZOOM_FOV 0x1600 + 0xb8 // m_playerData + m_curZoomFOV
#define OFFSET_AMMO 0x1584            // m_ammoInClip first offset

#define OFFSET_ITEM_GLOW 0x02f0         // m_highlightFunctionBits
#define OFFSET_ITEM_ID 0x1588           // m_customScriptInt
#define OFFSET_MODELNAME 0x0030         // m_ModelName
#define OFFSET_M_CUSTOMSCRIPTINT 0x1588 // m_customScriptInt
#define OFFSET_YAW 0x226c - 0x8 // m_currentFramePlayer.m_ammoPoolCount - 0x8

#define OFFSET_GLOW_T1 0x262 + 0x30     // 16256 = enabled, 0 = disabled
#define OFFSET_GLOW_T2 0x2dc + 0x30     // 1193322764 = enabled, 0 = disabled
#define OFFSET_GLOW_ENABLE 0x294        // 7 = enabled, 2 = disabled
#define OFFSET_GLOW_THROUGH_WALLS 0x278 // 2 = enabled, 5 = disabled

#define GLOW_COLOR_R 0x1D0 + 0x30
#define GLOW_COLOR_G 0x1D4 + 0x30
#define GLOW_COLOR_B 0x1D8 + 0x30
#define GLOW_START_TIME 0x02c8 + 0x30 // m_playerFloatLookStartTime=0x02c8
#define OFFSET_HIGHLIGHTSERVERACTIVESTATES 0x298

#define OFFSET_GLOW_ENABLE_GLOW_CONTEXT                                        \
  OFFSET_GLOW_ENABLE // Script_Highlight_SetCurrentContext
#define OFFSET_GLOW_THROUGH_WALLS_GLOW_VISIBLE_TYPE                            \
  OFFSET_GLOW_THROUGH_WALLS // Script_Highlight_SetVisibilityType 5th mov
#define GLOW_LIFE_TIME 0x3A4 + 0x30 // Script_Highlight_SetLifeTime + 4
#define GLOW_DISTANCE 0x26c         // Script_Highlight_SetFarFadeDist
#define GLOW_TYPE 0x29c             // Script_Highlight_GetState + 4
#define GLOW_COLOR 0x1D0 + 0x30     // Script_CopyHighlightState 15th mov
#define GLOW_FADE                                                              \
  0x388 + 0x30 // Script_Highlight_GetCurrentInsideOpacity 3rd result of 3
               // offsets consecutive or first + 8
#define HIGHLIGHT_SETTINGS 0xB5C4090 //?
#define HIGHLIGHT_TYPE_SIZE 0x28     //?
#define OFFSET_CROSSHAIR_LAST                                                  \
  0x19c8 // CPlayer!lastCrosshairTargetTime // CPlayer!lastVisibleTime + 0x8
#define OFFSET_CROSSHAIR_START 0x1A84 // CPlayer!crosshairTargetStartTime
#define OFFSET_INPUT_SYSTEM 0x170e080 // InputSystem