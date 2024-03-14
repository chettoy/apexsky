use super::Pod;
use crate::noobfstr as s;
use std::fmt;

// https://www.unknowncheats.me/forum/apex-legends/446349-script-highlight.html

#[derive(Copy, Clone, Pod)]
#[repr(C)]
pub struct HighlightParams {
    pub color: [f32; 3],
    pub velocity: [f32; 3],
}
impl fmt::Debug for HighlightParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(s!("HighlightParams"))
            .field(s!("color"), &format_args!("{:?}", self.color))
            .field(s!("velocity"), &format_args!("{:?}", self.velocity))
            .finish()
    }
}

#[derive(Copy, Clone, Debug, Default, Pod)]
#[repr(C)]
pub struct HighlightBits {
    pub inside_function: u8,
    pub outline_function: u8,
    pub outline_radius: u8, // 1.0..8.0
    pub inside_opacity: u8,
}
impl HighlightBits {
    pub const fn new(
        inside_function: u8,
        outline_function: u8,
        outline_radius: u8,
        inside_opacity: u8,
        is_entity_visible: bool,
        is_after_post_process: bool,
    ) -> HighlightBits {
        HighlightBits {
            inside_function,
            outline_function,
            outline_radius,
            inside_opacity: inside_opacity
                | if is_entity_visible { 0x40 } else { 0 }
                | if is_after_post_process { 0x80 } else { 0 },
        }
    }

    /// Bloodhound scan effect.
    pub const SONAR: HighlightBits = HighlightBits::new(12, 169, 32, 7, true, false);
}
impl HighlightBits {
    pub fn from_uint(int: u32) -> HighlightBits {
        unsafe { std::mem::transmute(int) }
    }
    pub fn to_int(&self) -> u32 {
        (self.inside_opacity as u32) << 24
            | (self.outline_radius as u32) << 16
            | (self.outline_function as u32) << 8
            | (self.inside_function as u32)
    }
    pub fn outline_radius(&self) -> f32 {
        self.outline_radius as f32 * (8.0 / 255.0)
    }
    pub fn is_entity_visible(&self) -> bool {
        self.inside_opacity & 0x40 != 0
    }
    pub fn is_after_post_process(&self) -> bool {
        self.inside_opacity & 0x80 != 0
    }
}

#[derive(Copy, Clone, Pod)]
#[repr(C)]
pub struct HighlightFadeSlot {
    pub inside: f32,
    pub outside: f32,
}
impl fmt::Debug for HighlightFadeSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(s!("HighlightFadeSlot"))
            .field(s!("inside"), &self.inside)
            .field(s!("outside"), &self.outside)
            .finish()
    }
}

pub const HIGHLIGHT_CONTEXT_NONE: i32 = -1;
pub const HIGHLIGHT_CONTEXT_NEUTRAL: i32 = 0;
pub const HIGHLIGHT_CONTEXT_FRIENDLY: i32 = 1;
pub const HIGHLIGHT_CONTEXT_ENEMY: i32 = 2;
pub const HIGHLIGHT_CONTEXT_OWNED: i32 = 3;
pub const HIGHLIGHT_CONTEXT_PRIVATE_MATCH_OBSERVER: i32 = 4;
pub const HIGHLIGHT_CHARACTER_SPECIAL_HIGHLIGHT: i32 = 5;
pub const HIGHLIGHT_CONTEXT_DEATH_RECAP: i32 = 6;
pub const HIGHLIGHT_CONTEXT_SONAR: i32 = 7;
pub const HIGHLIGHT_CHARACTER_SPECIAL_HIGHLIGHT_2: i32 = 8;
pub const HIGHLIGHT_CONTEXT_FRIENDLY_REVEALED: i32 = 9;
pub const HIGHLIGHT_CONTEXT_MOVEMENT_REVEALED: i32 = 10;
pub const HIGHLIGHT_MAX_CONTEXTS: usize = 11;

pub const HIGHLIGHT_VIS_NONE: i32 = 0;
pub const HIGHLIGHT_VIS_FORCE_ON: i32 = 1;
pub const HIGHLIGHT_VIS_ALWAYS: i32 = 2;
pub const HIGHLIGHT_VIS_OCCLUDED: i32 = 3;
pub const HIGHLIGHT_VIS_FULL_VIEW: i32 = 4;
pub const HIGHLIGHT_VIS_LOS: i32 = 5;
pub const HIGHLIGHT_VIS_LOS_ENTSONLYCONTENTSBLOCK: i32 = 6;

pub const HIGHLIGHT_FLAG_NONE: u32 = 0;
pub const HIGHLIGHT_FLAG_ADS_FADE: u32 = 1;
pub const HIGHLIGHT_FLAG_REQUIRE_NOT_FULL_HEALTH: u32 = 2;
pub const HIGHLIGHT_FLAG_REQUIRE_CAN_PICK_UP_CLIP: u32 = 4;
pub const HIGHLIGHT_FLAG_REQUIRE_CAN_PICK_UP_OFFHAND: u32 = 8;
pub const HIGHLIGHT_FLAG_REQUIRE_WEAKPOINT_VISIBLE: u32 = 16;
pub const HIGHLIGHT_FLAG_REQUIRE_PILOT: u32 = 32;
pub const HIGHLIGHT_FLAG_REQUIRE_TITAN: u32 = 64;
pub const HIGHLIGHT_FLAG_REQUIRE_SAME_TEAM: u32 = 128;
pub const HIGHLIGHT_FLAG_REQUIRE_DIFFERENT_TEAM: u32 = 256;
pub const HIGHLIGHT_FLAG_REQUIRE_FRIENDLY_TEAM: u32 = 512;
pub const HIGHLIGHT_FLAG_REQUIRE_ENEMY_TEAM: u32 = 1024;
pub const HIGHLIGHT_FLAG_REQUIRE_LOCAL_PLAYER_IS_OWNER: u32 = 2048;
pub const HIGHLIGHT_FLAG_REQUIRE_LOW_MOVEMENT: u32 = 4096;
pub const HIGHLIGHT_FLAG_REQUIRE_HIGH_MOVEMENT: u32 = 8192;
pub const HIGHLIGHT_FLAG_CHECK_OFTEN: u32 = 16384;
// HIGHLIGHT_FLAG_DISABLE_DEATH_FADE = _ImageBase, 32768
// HIGHLIGHT_FLAG_TEAM_AGNOSTIC = &loc_20000       65536

/*

type RGB = [f32; 3];

struct HighlightSettings {
    /*0x01b8*/ m_highlightParams: [RGB; 18],
    /*0x0290*/ m_highlightFunctionBits: [u32; 18],
    /*0x02d8*/ m_highlightServerFadeBases: [f32; 2],
    /*0x02e0*/ m_highlightServerFadeStartTimes: [f32; 2],
    /*0x02e8*/ m_highlightServerFadeEndTimes: [f32; 2],
    /*0x02f0*/ m_highlightFadeBases: [f32; 2],
    /*0x02f8*/ m_highlightFadeStartTimes: [f32; 2],
    /*0x0300*/ m_highlightFadeEndTimes: [f32; 2],

    /*0x0308*/ unk_curtime_lifetime: f32,
    /*0x030c*/ unk_curtime: f32,

    /*0x0318*/ m_highlightNearFadeDist: f32,
    /*0x031c*/ m_highlightFarFadeDist: f32,

    /*0x0328*/ m_highlightServerContextID: u32,
    /*0x0330*/ m_highlightContextID: u32,
    /*0x0334*/ m_highlightTeamBits: u32,
    /*0x033c*/ m_highlightFlags: u32,

    /*0x03c0*/ m_highlightEnabled: u8,
    /*0x03c4*/ m_highlightPingedState: u8,
}

/*
DT_HighlightSettings!0x01b8 m_highlightParams [RGB; 8]
DT_HighlightSettings!0x0290 m_highlightFunctionBits [u32; 8]
DT_HighlightSettings!0x02d8 m_highlightServerFadeBases [f32; 2]
DT_HighlightSettings!0x02e0 m_highlightServerFadeStartTimes [f32; 2]
DT_HighlightSettings!0x02e8 m_highlightServerFadeEndTimes [f32; 2]
DT_HighlightSettings!0x0328 m_highlightServerContextID u32
DT_HighlightSettings!0x0334 m_highlightTeamBits


Highlight_GetCurrentInsideOpacity
0x2f0, 0x2f8, 0x300
0x2d8, 0x2e0

Highlight_GetCurrentOutlineOpacity
0x2f4, 0x2fc, 0x304
0x2dc, 0x2e4

Highlight_SetCurrentContext
0x330: current context (0x328: server context)
0x30c: filled with curtime
0x2f0: 0.0
0x2f8: 0.0
0x300: 0.0

Highlight_SetInheritHighlight
0x560: u8 bool (is client entity)
0x42C: u32 flags
0x30C: filled with curtime

Highlight_SetFunctions
0x560: u8 bool

Highlight_HideInside
0x308: f32
0x340: f32
0x3C0: u8 bool?

Highlight_HideOutside
0x308: f32
0x340: f32
0x3C0: u8 bool?

Highlight_SetFadeInTime
0x310: fade in time

Highlight_SetFadeOutTime
0x314: fade out time

Highlight_SetVisibilityType

Highlight_SetLifeTime
0x308: curtime + lifetime

Highlight_SetNearFadeDist
0x318: near fade dist

Highlight_SetFarFadeDist
0x31C: far fade dist

Highlight_SetFlag
0x33c: flags

Highlight_StartOn
0x2EC: f32
0x304, 0x2fc, 0x2f4
0x2e4, 0x2dc
0x2E8: f32
0x300, 0x2f8, 0x2f0,
0x2e0, 0x2d8

0x3C0: u8, bool

HighlightEnableForTeam
0x334: team bitfield

Highlight_PushPingedState
0x3c4: push pinged state

struct RGB {
    red: f32,
    green: f32,
    blue: f32,
}

fn GetInsideFunction(bits: u32) -> u32 {
    return bits & 0xff;
}
fn GetOutsideFunction(bits: u32) -> u32 {
    return (bits >> 8) & 0xff;
}
fn GetOutlineRadius(bits: u32) -> f32 {
    return ((bits >> 16) & 0xff) as f32 * 8.0 / 255.0;
}
fn GetCustomState(bits: u32) -> u32 {
    return (bits >> 24) & 0x3F;
}
fn IsEntityVisible(bits: u32) -> bool {
    return (bits >> 30) & 0x1 != 0;
}
fn IsAfterPostProcess(bits: u32) -> bool {
    return (bits >> 31) & 0x1 != 0;
}


// start 0x01b8 end 0x03cc
struct HighlightSettings {

}
*/
*/
