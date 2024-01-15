//Date 1/19/2024
//GameVersion=v3.0.54.31

#define OFFSET_ITEM_ID 0x1518 // item id?      //updated 11/1/2023
#define OFFSET_M_CUSTOMSCRIPTINT 0x1518 //m_customScriptInt //updated 1/10/2024
#define OFFSET_YAW 0x21fc - 0x8 //m_currentFramePlayer.m_ammoPoolCount//updated 1/10/2024 - 0x8 

#define HIGHLIGHT_SETTINGS 0xB93E050 //HighlightSettings  // updated 1/19/2024
#define OFFSET_GLOW_CONTEXT_ID 0x28c // updated 1/19/2024
#define GLOW_VISIBLE_TYPE 0x26c // updated 1/19/2024
#define OFFSET_GLOW_FIX 0x268 // updated 1/19/2024
// Mode: HighlightSettings + 0x34 * Context + 0x0
// Color: HighlightSettings + 0x34 * Context + 0x4


#define OFFSET_IS_SKYDIVING 0x462c //m_skydiveState
#define OFFSET_GRAPPLE 0x2c08 //[RecvTable.DT_Player].m_grapple GrappleData
#define OFFSET_GRAPPLE_ACTIVE 0x2c98 //m_grappleActive  int
#define OFFSET_GRAPPLE_ATTACHED 0x0048//m_grappleAttached  int
