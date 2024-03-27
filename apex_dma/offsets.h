// Date 3/27/2024
// GameVersion=v3.0.62.29

#define OFFSET_ITEM_ID 0x1538           // item id?      //updated 2/14/2024
#define OFFSET_M_CUSTOMSCRIPTINT 0x1538 // m_customScriptInt //updated 2/14/2024
#define OFFSET_YAW                                                             \
  0x221c - 0x8 // m_currentFramePlayer.m_ammoPoolCount//updated 2/14/2024 - 0x8

#define HIGHLIGHT_SETTINGS 0xBB24350 // HighlightSettings  // updated 3/27/2024
#define OFFSET_GLOW_CONTEXT_ID 0x28c // updated 1/25/2024
#define GLOW_VISIBLE_TYPE 0x26c      // updated 1/25/2024
#define OFFSET_GLOW_FIX 0x268        // updated 1/25/2024
// Mode: HighlightSettings + 0x34 * Context + 0x0
// Color: HighlightSettings + 0x34 * Context + 0x4

#define m_lastChargeLevel 0x16e0 // m_lastChargeLevel
