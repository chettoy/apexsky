use crate::{def_offsets, export_custom_offsets, hex_offset as off};
use serde::{Deserialize, Serialize};

def_offsets!(OffsetsMiscellaneous: ["Miscellaneous"] {
    game_version: "GameVersion",
    cl_entitylist: "cl_entitylist",
    local_player: "LocalPlayer",
    global_vars: "GlobalVars",
    input_system: "InputSystem",
    name_list: "NameList",
    view_render: "ViewRender",
    view_matrix: "ViewMatrix",
    client_state: "ClientState",
    signon_state: "SignonState",
    level_name: "LevelName",
    cplayer_last_visible_time: "CPlayer!lastVisibleTime",
    cweapon_x_crosshair_target_time: "CWeaponX!crosshairTargetTime",
    cweapon_x_last_crosshair_target_time: "CWeaponX!lastCrosshairTargetTime",
    cweapon_x_m_fl_projectile_speed: "CWeaponX!m_flProjectileSpeed",
    cweapon_x_m_fl_projectile_scale: "CWeaponX!m_flProjectileScale",
    cplayer_camera_origin: "CPlayer!camera_origin",
    cplayer_camera_angles: "CPlayer!camera_angles",
    cbase_animating_m_p_studio_hdr: "CBaseAnimating!m_pStudioHdr",
});

def_offsets!(OffsetsButtons: ["Buttons"] {
    in_attack: "in_attack",
    in_backward: "in_backward",
    in_break: "in_break",
    in_camin: "in_camin",
    in_camout: "in_camout",
    in_campitchdown: "in_campitchdown",
    in_campitchup: "in_campitchup",
    in_camyawleft: "in_camyawleft",
    in_camyawright: "in_camyawright",
    in_commandermousemove: "in_commandermousemove",
    in_dodge: "in_dodge",
    in_duck: "in_duck",
    in_forward: "in_forward",
    in_graph: "in_graph",
    in_jump: "in_jump",
    in_klook: "in_klook",
    in_left: "in_left",
    in_lookdown: "in_lookdown",
    in_lookup: "in_lookup",
    in_melee: "in_melee",
    in_movedown: "in_movedown",
    in_moveleft: "in_moveleft",
    in_moveright: "in_moveright",
    in_moveup: "in_moveup",
    in_offhand0: "in_offhand0",
    in_offhand1: "in_offhand1",
    in_offhand2: "in_offhand2",
    in_offhand3: "in_offhand3",
    in_offhand4: "in_offhand4",
    in_pause_menu: "in_pause_menu",
    in_ping: "in_ping",
    in_reload: "in_reload",
    in_right: "in_right",
    in_score: "in_score",
    in_script_command3: "in_scriptCommand3",
    in_showscores: "in_showscores",
    in_speed: "in_speed",
    in_strafe: "in_strafe",
    in_toggle_duck: "in_toggle_duck",
    in_toggle_zoom: "in_toggle_zoom",
    in_use: "in_use",
    in_use_and_reload: "in_useAndReload",
    in_use_alt: "in_use_alt",
    in_use_long: "in_use_long",
    in_variable_scope_toggle: "in_variableScopeToggle",
    in_walk: "in_walk",
    in_weapon_cycle: "in_weaponCycle",
    in_weapon_discard: "in_weapon_discard",
    in_zoom: "in_zoom",
});

def_offsets!(OffsetsConVars: ["ConVars"] {
    host_framerate: "host_framerate",
    host_map: "host_map",
    name: "name",
});

def_offsets!(OffsetsDtBaseAnimating: ["RecvTable.DT_BaseAnimating"] {
    m_n_force_bone: "m_nForceBone",
});

def_offsets!(OffsetsDtBaseEntity: ["RecvTable.DT_BaseEntity"] {
    highlight_settings: "HighlightSettings",
    m_i_team_num: "m_iTeamNum",
    m_shield_health: "m_shieldHealth",
    m_shield_health_max: "m_shieldHealthMax",
    m_i_signifier_name: "m_iSignifierName",
    m_i_name: "m_iName",
});

def_offsets!(OffsetsDtHighlightSettings: ["RecvTable.DT_HighlightSettings"] {
    m_highlight_team_index: "m_highlightTeamIndex",
    m_highlight_team_bits: "m_highlightTeamBits",
    m_highlight_generic_contexts: "m_highlightGenericContexts",
    m_highlight_focused: "m_highlightFocused",
    m_highlight_fade_duration: "m_highlightFadeDuration",
    m_highlight_fade_parity: "m_highlightFadeParity",
});

def_offsets!(OffsetsDtLocalPlayerExclusive: ["RecvTable.DT_LocalPlayerExclusive"] {
    m_i_observer_mode: "m_iObserverMode",
    m_h_observer_target: "m_hObserverTarget",
});

def_offsets!(OffsetsDtPlayer: ["RecvTable.DT_Player"] {
    m_i_health: "m_iHealth",
    m_i_max_health: "m_iMaxHealth",
    m_life_state: "m_lifeState",
    m_b_zooming: "m_bZooming",
    m_current_frame_player: "m_currentFramePlayer",
    pl: "pl",
    m_ammo_pool_capacity: "m_ammoPoolCapacity",
    m_xp: "m_xp",
    m_helmet_type: "m_helmetType",
    m_armor_type: "m_armorType",
    m_controller_mode_active: "m_controllerModeActive",
    m_platform_user_id: "m_platformUserId",
    m_bleedout_state: "m_bleedoutState",
});

def_offsets!(OffsetsDtWeaponX: ["RecvTable.DT_WeaponX"] {
    local_weapon_data: "LocalWeaponData",
    m_player_data: "m_playerData",
    m_mod_bitfield_from_player: "m_modBitfieldFromPlayer",
    m_weapon_name_index: "m_weaponNameIndex",
});

def_offsets!(OffsetsDtLocalWeaponData: ["RecvTable.DT_WeaponX_LocalWeaponData"] {
    m_ammo_in_clip: "m_ammoInClip",
    m_ammo_in_stockpile: "m_ammoInStockpile",
    m_infinite_ammo_state: "m_infiniteAmmoState",
    m_b_in_reload: "m_bInReload",
});

def_offsets!(OffsetsDtWeaponPlayerData: ["RecvTable.DT_WeaponPlayerData"] {
    m_cur_zoom_fov: "m_curZoomFOV",
});

def_offsets!(OffsetsDataMapCBaseEntity0: ["DataMap.C_BaseEntity"] {
    m_model_name: "m_ModelName",
    m_f_flags: "m_fFlags",
});

def_offsets!(OffsetsDataMapCBaseEntity1: ["DataMap.C_BaseEntity"] {
    m_f_flags: "m_fFlags",
    m_current_frame_view_offset: "m_currentFrame.viewOffset",
    m_vec_abs_velocity: "m_vecAbsVelocity",
    m_vec_abs_origin: "m_vecAbsOrigin",
});

def_offsets!(OffsetsDataMapCBaseCombatCharacter: ["DataMap.C_BaseCombatCharacter"] {
    m_inventory: "m_inventory",
    m_latest_non_offhand_weapons: "m_latestNonOffhandWeapons",
    m_latest_primary_weapons: "m_latestPrimaryWeapons",
});

def_offsets!(OffsetsDataMapWeaponInventoryClient: ["DataMap.WeaponInventory_Client"] {
    weapons: "weapons",
    active_weapons: "activeWeapons",
});

def_offsets!(OffsetsDataMapCPlayer: ["DataMap.C_Player"] {
    m_current_frame_player_time_base: "m_currentFramePlayer.timeBase",
    m_current_frame_player_m_ammo_pool_count: "m_currentFramePlayer.m_ammoPoolCount",
    m_current_frame_local_player_m_vec_punch_weapon_angle: "m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle",
    m_traversal_progress: "m_traversalProgress",
    m_traversal_start_time: "m_traversalStartTime",
    m_wall_run_start_time: "m_wallRunStartTime",
    m_wall_run_clear_time: "m_wallRunClearTime",
    m_h_view_models: "m_hViewModels",
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct OffsetsData {
    pub(super) miscellaneous: OffsetsMiscellaneous,
    pub(super) buttons: OffsetsButtons,
    pub(super) con_vars: OffsetsConVars,
    pub(super) dt_base_animating: OffsetsDtBaseAnimating,
    pub(super) dt_base_entity: OffsetsDtBaseEntity,
    pub(super) dt_highlight_settings: OffsetsDtHighlightSettings,
    pub(super) dt_local_player_exclusive: OffsetsDtLocalPlayerExclusive,
    pub(super) dt_player: OffsetsDtPlayer,
    pub(super) dt_weapon_x: OffsetsDtWeaponX,
    pub(super) dt_local_weapon_data: OffsetsDtLocalWeaponData,
    pub(super) dt_weapon_player_data: OffsetsDtWeaponPlayerData,
    pub(super) c_base_entity0: OffsetsDataMapCBaseEntity0,
    pub(super) c_base_entity1: OffsetsDataMapCBaseEntity1,
    pub(super) c_base_combat_character: OffsetsDataMapCBaseCombatCharacter,
    pub(super) weapon_inventory_client: OffsetsDataMapWeaponInventoryClient,
    pub(super) c_player: OffsetsDataMapCPlayer,
}

pub fn read_offsets(path: std::path::PathBuf) -> anyhow::Result<OffsetsData> {
    let conf = ini::Ini::load_from_file(path)?;

    let data = OffsetsData {
        miscellaneous: OffsetsMiscellaneous::from_ini(&conf)?,
        buttons: OffsetsButtons::from_ini(&conf)?,
        con_vars: OffsetsConVars::from_ini(&conf)?,
        dt_base_animating: OffsetsDtBaseAnimating::from_ini(&conf)?,
        dt_base_entity: OffsetsDtBaseEntity::from_ini(&conf)?,
        dt_highlight_settings: OffsetsDtHighlightSettings::from_ini(&conf)?,
        dt_local_player_exclusive: OffsetsDtLocalPlayerExclusive::from_ini(&conf)?,
        dt_player: OffsetsDtPlayer::from_ini(&conf)?,
        dt_weapon_x: OffsetsDtWeaponX::from_ini(&conf)?,
        dt_local_weapon_data: OffsetsDtLocalWeaponData::from_ini(&conf)?,
        dt_weapon_player_data: OffsetsDtWeaponPlayerData::from_ini(&conf)?,
        c_base_entity0: OffsetsDataMapCBaseEntity0::from_ini(&conf)?,
        c_base_entity1: OffsetsDataMapCBaseEntity1::from_ini(&conf)?,
        c_base_combat_character: OffsetsDataMapCBaseCombatCharacter::from_ini(&conf)?,
        weapon_inventory_client: OffsetsDataMapWeaponInventoryClient::from_ini(&conf)?,
        c_player: OffsetsDataMapCPlayer::from_ini(&conf)?,
    };

    Ok(data)
}

export_custom_offsets!(
    ~OffsetsData~(^_^) ~read_offsets(~_~)=> offsets {
        offset_entitylist: off!(offsets.miscellaneous.cl_entitylist),
        offset_local_ent: off!(offsets.miscellaneous.local_player),
        offset_name_list: off!(offsets.miscellaneous.name_list),
        offset_global_vars: off!(offsets.miscellaneous.global_vars),
        offset_levelname: off!(offsets.miscellaneous.level_name),
        offset_clientstate: off!(offsets.miscellaneous.client_state),
        offset_signonstate: off!(offsets.miscellaneous.signon_state),
        offset_host_map: off!(offsets.con_vars.host_map),
        offset_entity_team: off!(offsets.dt_base_entity.m_i_team_num),
        offset_player_health: off!(offsets.dt_player.m_i_health),
        offset_entity_shield: off!(offsets.dt_base_entity.m_shield_health),
        offset_entity_maxshield: off!(offsets.dt_base_entity.m_shield_health_max),
        offset_player_helmettype: off!(offsets.dt_player.m_helmet_type),
        offset_player_armortype: off!(offsets.dt_player.m_armor_type),
        offset_entiry_name: off!(offsets.dt_base_entity.m_i_name),
        offset_entity_sign_name: off!(offsets.dt_base_entity.m_i_signifier_name),
        offset_centity_abs_velocity: off!(offsets.c_base_entity1.m_vec_abs_velocity),
        offset_visible_time: off!(offsets.miscellaneous.cplayer_last_visible_time),
        offset_player_zooming: off!(offsets.dt_player.m_b_zooming),
        offset_traversal_progress: off!(offsets.c_player.m_traversal_progress),
        offset_traversal_starttime: off!(offsets.c_player.m_traversal_start_time),
        offset_platform_uid: off!(offsets.dt_player.m_platform_user_id),
        offset_weapon_name: off!(offsets.dt_weapon_x.m_weapon_name_index),
        offset_off_weapon: off!(offsets.c_base_combat_character.m_latest_non_offhand_weapons),
        offset_wall_run_start_time: off!(offsets.c_player.m_wall_run_start_time),
        offset_wall_run_clear_time: off!(offsets.c_player.m_wall_run_clear_time),
        offset_centity_flags: off!(offsets.c_base_entity0.m_f_flags),
        offset_in_attack: off!(offsets.buttons.in_attack),
        offset_in_toggle_duck: off!(offsets.buttons.in_toggle_duck),
        offset_in_zoom: off!(offsets.buttons.in_zoom),
        offset_in_forward: off!(offsets.buttons.in_forward),
        offset_in_jump: off!(offsets.buttons.in_jump),
        offset_in_duck: off!(offsets.buttons.in_duck),
        offset_in_use: off!(offsets.buttons.in_use),
        offset_player_life_state: off!(offsets.dt_player.m_life_state),
        offset_bleed_out_state: off!(offsets.dt_player.m_bleedout_state),
        offset_centity_viewoffset: off!(offsets.c_base_entity1.m_current_frame_view_offset),
        offset_centity_origin: off!(offsets.c_base_entity1.m_vec_abs_origin),
        offset_bones: 0x48 + off!(offsets.dt_base_animating.m_n_force_bone),
        offset_studiohdr: off!(offsets.miscellaneous.cbase_animating_m_p_studio_hdr),
        offset_cplayer_aimpunch: off!(offsets.c_player.m_current_frame_local_player_m_vec_punch_weapon_angle),
        offset_cplayer_camerapos: off!(offsets.miscellaneous.cplayer_camera_origin),
        offset_player_viewangles: off!(offsets.dt_player.m_ammo_pool_capacity) - 0x14,
        offset_breath_angles: off!(offsets.dt_player.m_ammo_pool_capacity) - 0x24,
        offset_observer_mode: off!(offsets.dt_local_player_exclusive.m_i_observer_mode),
        offset_ovserver_target: off!(offsets.dt_local_player_exclusive.m_h_observer_target),
        offset_matrix: off!(offsets.miscellaneous.view_matrix),
        offset_render: off!(offsets.miscellaneous.view_render),
        offset_primary_weapon: off!(offsets.c_base_combat_character.m_latest_primary_weapons),
        offset_active_weapon: off!(offsets.c_base_combat_character.m_inventory) + off!(offsets.weapon_inventory_client.active_weapons),
        offset_bullet_speed: off!(offsets.miscellaneous.cweapon_x_m_fl_projectile_speed),
        offset_bullet_scale: off!(offsets.miscellaneous.cweapon_x_m_fl_projectile_scale),
        offset_weaponx_zoom_fov: off!(offsets.dt_weapon_x.m_player_data) + off!(offsets.dt_weapon_player_data.m_cur_zoom_fov),
        offset_weaponx_ammo_in_clip: off!(offsets.dt_weapon_x.local_weapon_data) + off!(offsets.dt_local_weapon_data.m_ammo_in_clip),
        offset_centity_modelname: off!(offsets.c_base_entity0.m_model_name),
        offset_cplayer_timebase: off!(offsets.c_player.m_current_frame_player_time_base),
        offset_cplayer_viewmodels: off!(offsets.c_player.m_h_view_models),
        offset_crosshair_last: off!(offsets.miscellaneous.cweapon_x_last_crosshair_target_time),
        offset_input_system: off!(offsets.miscellaneous.input_system),
        offset_weaponx_bitfield_from_player: off!(offsets.dt_weapon_x.m_mod_bitfield_from_player),
        offset_entity_highlight_generic_context: off!(offsets.dt_base_entity.highlight_settings) + off!(offsets.dt_highlight_settings.m_highlight_generic_contexts),
    }
);

#[test]
fn test_parse_offsets() {
    export_offsets();
    println!("{:?}", offset_bullet_scale());
}
