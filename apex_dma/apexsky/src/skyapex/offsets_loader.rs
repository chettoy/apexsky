use serde::{Deserialize, Serialize};

use super::Skyapex;

macro_rules! import_custom_offsets {
    ({$($field:ident,)*}) => {
        paste::paste! {
            #[skyapex_impl]
            trait CustomOffsetsLoader {
                fn export_offsets(&mut self);
                $(fn [<offset_ $field>](&mut self) -> i64;)*
            }

            #[doc = "
            C FFI
            ```C
            typedef struct {
                ... // $(uintptr_t $field;)*
            } exported_offsets_t;
            ```"]
            #[repr(C)]
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct CustomOffsets {
                $(pub $field: u64,)*
            }

            impl CustomOffsets {
                pub fn load(skyapex: &mut Skyapex) -> Self {
                    skyapex.export_offsets();
                    Self { $($field: skyapex.[<offset_ $field>]() as u64,)* }
                }
            }
        }
    }
}

import_custom_offsets!({
    entitylist,
    local_ent,
    name_list,
    global_vars,
    levelname,
    clientstate,
    signonstate,
    host_map,
    entity_team,
    player_health,
    entity_shield,
    entity_maxshield,
    player_xp,
    player_helmettype,
    player_armortype,
    player_controller_active,
    entiry_name,
    entity_sign_name,
    centity_abs_velocity,
    visible_time,
    player_zooming,
    cplayer_traversal_progress,
    cplayer_traversal_starttime,
    player_platform_uid,
    weaponx_weapon_name,
    off_weapon,
    cplayer_wall_run_start_time,
    cplayer_wall_run_clear_time,
    centity_flags,
    in_attack,
    in_toggle_duck,
    in_zoom,
    in_forward,
    in_jump,
    in_duck,
    in_use,
    player_life_state,
    player_bleed_out_state,
    centity_viewoffset,
    centity_origin,
    bones,
    studiohdr,
    cplayer_aimpunch,
    cplayer_camerapos,
    player_viewangles,
    breath_angles,
    observer_mode,
    ovserver_target,
    view_matrix,
    view_render,
    primary_weapon,
    active_weapon,
    bullet_speed,
    bullet_scale,
    weaponx_zoom_fov,
    weaponx_ammo_in_clip,
    centity_modelname,
    cplayer_timebase,
    cplayer_viewmodels,
    crosshair_last,
    input_system,
    weaponx_bitfield_from_player,
    entity_fade_dist,
    entity_highlight_generic_context,
});
