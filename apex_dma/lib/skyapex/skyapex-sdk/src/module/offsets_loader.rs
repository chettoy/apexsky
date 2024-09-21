use serde::{Deserialize, Serialize};

use super::Skyapex;

macro_rules! import_custom_offsets {
    ({$($field:ident,)*}) => {
        paste::paste! {
            #[skyapex_impl]
            trait CustomOffsetsLoader {
                fn export_offsets_from_file(&mut self);
                fn export_offsets_from_str(&mut self, buf: i32);
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
                pub fn from_file(skyapex: &mut Skyapex) -> Self {
                    skyapex.export_offsets_from_file();
                    Self { $($field: skyapex.[<offset_ $field>]() as u64,)* }
                }

                pub fn from_string(skyapex: &mut Skyapex, data: String) -> Self {
                    let buf = skyapex.pass_string(data).unwrap();
                    skyapex.export_offsets_from_str(buf);
                    Self { $($field: skyapex.[<offset_ $field>]() as u64,)* }
                }
            }
        }
    }
}

import_custom_offsets!({
    time_date_stamp,
    checksum,
    global_vars,
    entitylist,
    local_entity_handle,
    local_ent,
    input_selected_slot,
    client_state,
    signon_state,
    level_name,
    nst_weapon_names,
    view_render,
    view_matrix,
    input_system,
    input_button_state,
    name_list,
    highlight_settings,
    spectator_list,
    network_var_table_ptr,
    network_var_table_len,
    host_map,
    thirdperson_override,
    mouse_sensitivity,
    fps_max,
    mp_gamemode,
    in_attack,
    in_jump,
    in_duck,
    in_reload,
    in_use,
    in_zoom,
    in_forward,
    in_backward,
    in_moveleft,
    in_moveright,
    in_toggle_duck,
    in_left,
    in_right,
    in_strafe,
    centity_modelname,
    centity_viewoffset,
    centity_flags,
    centity_origin,
    entity_shieldhealth,
    entity_maxshieldhealth,
    entity_highlight_generic_context,
    entity_team_num,
    centity_abs_velocity,
    centity_velocity,
    entity_owner_entity,
    entity_collision,
    entity_collision_group,
    entiry_name,
    entity_sign_name,
    entity_fade_dist,
    collision_property_vec_mins,
    collision_property_vec_maxs,
    animating_skin,
    animating_bone_array,
    bones,
    animating_studiohdr,
    bcc_next_attack,
    bcc_inventory,
    bcc_selected_weapons,
    bcc_off_weapon,
    bcc_primary_weapon,
    bcc_active_weapon,
    bcc_last_visible_time,
    player_last_visible_time,
    player_zooming,
    cplayer_camerapos,
    cplayer_timebase,
    cplayer_server_angles,
    cplayer_aimpunch,
    cplayer_viewmodels,
    cplayer_traversal_progress,
    cplayer_traversal_starttime,
    cplayer_wall_run_start_time,
    cplayer_wall_run_clear_time,
    player_viewangles,
    player_consumables,
    player_traversal_release_time,
    player_observer_state,
    player_ovserver_target,
    player_platform_uid,
    player_health,
    player_maxhealth,
    player_bleed_out_state,
    player_life_state,
    player_duck_state,
    player_lean_state,
    player_grapple,
    player_grapple_active,
    player_shadow_shield_active,
    player_temp_shield_health,
    player_extra_shield_health,
    player_extra_shield_tier,
    player_is_performing_boost_action,
    player_xp,
    player_third_person_shoulder_view,
    player_net_var,
    player_helmettype,
    player_armortype,
    player_controller_active,
    player_skydive_state,
    player_breath_angles,
    weaponx_weapon_owner,
    weaponx_next_primary_attack,
    weaponx_ammo_in_clip,
    weaponx_zoom_fov,
    weaponx_charge_start_time,
    weaponx_charge_end_time,
    weaponx_last_charge_frac,
    cweaponx_burst_fire,
    cweaponx_crosshair_last,
    weaponx_bitfield_from_player,
    weaponx_weapon_name_index,
    weaponx_printname,
    weaponx_shortprintname,
    weaponx_fire_rate,
    weaponx_fire_duration,
    weaponx_is_semi_auto,
    weaponx_ammo_clip_size,
    weaponx_grenade_view_launch_offset,
    weaponx_projectile_launch_speed,
    weaponx_projectile_launch_pitch_offset,
    weaponx_projectile_gravity_scale,
    weaponx_projectile_air_friction,
    vehicle_driver,
    vehicle_velocity,
    prop_survival,
    projectile,
    world_death_field,
    waypoint_type,
    mods_names,
    mods_list,
    mods_count,
    grapple_attached,
    grapple_pulling,
    var_damage,
    global_observer_mode,
});
