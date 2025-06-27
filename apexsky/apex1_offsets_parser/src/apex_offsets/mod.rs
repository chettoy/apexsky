use serde::{Deserialize, Serialize};

use crate::CustomOffsets;

mod offsets_parser;

use offsets_parser::{assert_offsets_eq, def_offsets, export_custom_offsets, parse_u64 as off};

def_offsets!(OffsetsMiscellaneous: ["Miscellaneous"] {
    time_date_stamp: "TimeDateStamp",
    check_sum: "CheckSum",
    game_version: "GameVersion",
    cl_entitylist: "cl_entitylist",
    local_entity_handle: "LocalEntityHandle",
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
    highlight_settings: "HighlightSettings",
    observer_list: "ObserverList",
    network_var_table_ptr: "NetworkVarTablePtr",
    network_var_table_len: "NetworkVarTableLen",
});

def_offsets!(OffsetsNetworkedStringTables: ["NetworkedStringTables"] {
    weapon_names: "WeaponNames",
});

def_offsets!(OffsetsModifierOffsets: ["ModifierOffsets"] {
    mods_names: "mods_names",
    mods_list: "mods_list",
    mods_count: "mods_count",
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
    fps_max: "fps_max",
    host_framerate: "host_framerate",
    host_map: "host_map",
    hostname: "hostname",
    ip: "ip",
    mouse_sensitivity: "mouse_sensitivity",
    mp_gamemode: "mp_gamemode",
    name: "name",
    thirdperson_override: "thirdperson_override",
});

def_offsets!(OffsetsDtBaseAnimating: ["RecvTable.DT_BaseAnimating"] {
    m_n_skin: "m_nSkin",
    m_n_force_bone: "m_nForceBone",
    m_b_sequence_finished: "m_bSequenceFinished",
});

def_offsets!(OffsetsDtBaseCombatCharacter: ["RecvTable.DT_BaseCombatCharacter"] {
    m_last_fired_time: "m_lastFiredTime",
    m_last_fired_weapon: "m_lastFiredWeapon",
    m_raise_from_melee_end_time: "m_raiseFromMeleeEndTime",
    m_selected_weapons: "m_selectedWeapons",
    m_latest_primary_weapons: "m_latestPrimaryWeapons",
    m_latest_non_offhand_weapons: "m_latestNonOffhandWeapons",
    m_hud_info_visibility_test_always_passes: "m_hudInfo_visibilityTestAlwaysPasses",
});

def_offsets!(OffsetsDtBaseEntity: ["RecvTable.DT_BaseEntity"] {
    highlight_settings: "HighlightSettings",
    m_shield_health: "m_shieldHealth",
    m_shield_health_max: "m_shieldHealthMax",
    m_i_team_num: "m_iTeamNum",
    m_h_owner_entity: "m_hOwnerEntity",
    m_collision: "m_Collision",
    m_collision_group: "m_CollisionGroup",
    m_i_signifier_name: "m_iSignifierName",
    m_i_name: "m_iName",
    m_script_name_index: "m_scriptNameIndex",
    m_fade_dist: "m_fadeDist",
});

def_offsets!(OffsetsDtCollisionProperty: ["RecvTable.DT_CollisionProperty"] {
    m_vec_mins: "m_vecMins",
    m_vec_maxs: "m_vecMaxs",
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
    m_vec_velocity_: "m_vecVelocity.x",
    m_consumable_inventory: "m_consumableInventory",
    m_traversal_progress: "m_traversalProgress",
    m_traversal_start_time: "m_traversalStartTime",
    m_traversal_release_time: "m_traversalReleaseTime",
    m_i_observer_mode: "m_iObserverMode",
    m_h_observer_target: "m_hObserverTarget",
    m_wall_run_start_time: "m_wallRunStartTime",
    m_wall_run_clear_time: "m_wallRunClearTime",
    m_third_person_shoulder_view: "m_thirdPersonShoulderView",
});

def_offsets!(OffsetsDtPlayer: ["RecvTable.DT_Player"] {
    m_f_flags: "m_fFlags",
    m_i_health: "m_iHealth",
    m_i_max_health: "m_iMaxHealth",
    m_life_state: "m_lifeState",
    m_inventory: "m_inventory",
    m_b_zooming: "m_bZooming",
    m_zoom_base_frac: "m_zoomBaseFrac",
    m_zoom_base_time: "m_zoomBaseTime",
    m_current_frame_player: "m_currentFramePlayer",
    pl: "pl",
    m_ammo_pool_capacity: "m_ammoPoolCapacity",
    m_platform_user_id: "m_platformUserId",
    m_bleedout_state: "m_bleedoutState",
    m_duck_state: "m_duckState",
    m_lean_state: "m_leanState",
    m_grapple: "m_grapple",
    m_grapple_active: "m_grappleActive",
    m_h_view_models: "m_hViewModels",
    m_shadow_shield_active: "m_shadowShieldActive",
    m_temp_shield_health: "m_tempShieldHealth",
    m_extra_shield_health: "m_extraShieldHealth",
    m_extra_shield_tier: "m_extraShieldTier",
    m_is_performing_boost_action: "m_isPerformingBoostAction",
    m_xp: "m_xp",
    m_player_script_net_data_global: "m_playerScriptNetDataGlobal",
    m_helmet_type: "m_helmetType",
    m_armor_type: "m_armorType",
    m_controller_mode_active: "m_controllerModeActive",
    m_skydive_state: "m_skydiveState",
});

def_offsets!(OffsetsDtPlayerVehicle: ["RecvTable.DT_PlayerVehicle"] {
    m_vehicle_driver: "m_vehicleDriver",
    m_vehicle_velocity: "m_vehicleVelocity",
});

def_offsets!(OffsetsDtPlayerWaypoint: ["RecvTable.DT_PlayerWaypoint"] {
    m_waypoint_type: "m_waypointType",
});

def_offsets!(OffsetsDtProjectile: ["RecvTable.DT_Projectile"] {
    m_vec_velocity: "m_vecVelocity",
    m_weapon_data_is_set: "m_weaponDataIsSet",
});

def_offsets!(OffsetsDtPropSurvival: ["RecvTable.DT_PropSurvival"] {
    m_ammo_in_clip: "m_ammoInClip",
});

def_offsets!(OffsetsDtWeaponPlayerData: ["RecvTable.DT_WeaponPlayerData"] {
    m_cur_zoom_fov: "m_curZoomFOV",
});

def_offsets!(OffsetsDtWeaponX: ["RecvTable.DT_WeaponX"] {
    local_weapon_data: "LocalWeaponData",
    m_weapon_owner: "m_weaponOwner",
    m_player_data: "m_playerData",
    m_charge_start_time: "m_chargeStartTime",
    m_charge_end_time: "m_chargeEndTime",
    m_last_charge_frac: "m_lastChargeFrac",
    m_mod_bitfield_from_player: "m_modBitfieldFromPlayer",
    m_weapon_name_index: "m_weaponNameIndex",
});

def_offsets!(OffsetsDtWeaponXLocalWeaponData: ["RecvTable.DT_WeaponX_LocalWeaponData"] {
    m_next_primary_attack_time: "m_nextPrimaryAttackTime",
    m_ammo_in_clip: "m_ammoInClip",
    m_ammo_in_stockpile: "m_ammoInStockpile",
    m_infinite_ammo_state: "m_infiniteAmmoState",
    m_b_in_reload: "m_bInReload",
});

def_offsets!(OffsetsDtWeaponXPredictingClientOnly: ["RecvTable.DT_WeaponX_PredictingClientOnly"] {
    m_burst_fire_count: "m_burstFireCount",
});

def_offsets!(OffsetsDtWorld: ["RecvTable.DT_World"] {
    m_death_field_is_active: "m_deathFieldIsActive",
});

def_offsets!(OffsetsWeaponSettingsMeta: ["WeaponSettingsMeta"] {
    base: "base",
});

def_offsets!(OffsetsWeaponSettings: ["WeaponSettings"] {
    printname: "printname",
    shortprintname: "shortprintname",
    description: "description",
    is_semi_auto: "is_semi_auto",
    ammo_clip_size: "ammo_clip_size",
});

def_offsets!(OffsetsDtGlobalNonRewinding: ["RecvTable.DT_GlobalNonRewinding"] {
    m_game_timescale: "m_gameTimescale",
    m_player_observer: "m_playerObserver",
    m_player_misc_data: "m_playerMiscData",
});

def_offsets!(OffsetsDtGrappleData: ["RecvTable.DT_GrappleData"] {
    m_grapple_attached: "m_grappleAttached",
    m_grapple_pulling: "m_grapplePulling",
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
    m_vec_velocity: "m_vecVelocity",
});

def_offsets!(OffsetsDataMapCBaseCombatCharacter: ["DataMap.C_BaseCombatCharacter"] {
    m_fl_next_attack: "m_flNextAttack",
    m_inventory: "m_inventory",
    m_selected_weapons: "m_selectedWeapons",
    m_latest_primary_weapons: "m_latestPrimaryWeapons",
    m_latest_non_offhand_weapons: "m_latestNonOffhandWeapons",
    m_hud_info_visibility_test_always_passes: "m_hudInfo_visibilityTestAlwaysPasses",
});

def_offsets!(OffsetsDataMapWeaponInventoryClient: ["DataMap.WeaponInventory_Client"] {
    weapons: "weapons",
    active_weapons: "activeWeapons",
});

def_offsets!(OffsetsDataMapCPlayer: ["DataMap.C_Player"] {
    m_f_flags: "m_fFlags",
    m_vec_velocity: "m_vecVelocity",
    m_b_zooming: "m_bZooming",
    m_zoom_base_frac: "m_zoomBaseFrac",
    m_zoom_base_time: "m_zoomBaseTime",
    m_current_frame_player_time_base: "m_currentFramePlayer.timeBase",
    m_current_frame_player_m_ziprail_bank_tilt_frac: "m_currentFramePlayer.m_ziprailBankTiltFrac",
    m_current_frame_player_m_ammo_pool_count: "m_currentFramePlayer.m_ammoPoolCount",
    m_current_frame_local_player_m_vec_punch_weapon_angle: "m_currentFrameLocalPlayer.m_vecPunchWeapon_Angle",
    pl: "pl",
    m_traversal_progress: "m_traversalProgress",
    m_traversal_start_time: "m_traversalStartTime",
    m_traversal_release_time: "m_traversalReleaseTime",
    m_wall_run_start_time: "m_wallRunStartTime",
    m_wall_run_clear_time: "m_wallRunClearTime",
    m_h_view_models: "m_hViewModels",
    m_skydive_state: "m_skydiveState",
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct OffsetsData {
    pub(super) miscellaneous: OffsetsMiscellaneous,
    pub(super) networked_string_tables: OffsetsNetworkedStringTables,
    pub(super) modifier_offsets: OffsetsModifierOffsets,
    pub(super) buttons: OffsetsButtons,
    pub(super) con_vars: OffsetsConVars,
    pub(super) dt_base_animating: OffsetsDtBaseAnimating,
    pub(super) dt_base_combat_character: OffsetsDtBaseCombatCharacter,
    pub(super) dt_base_entity: OffsetsDtBaseEntity,
    pub(super) dt_collision_property: OffsetsDtCollisionProperty,
    pub(super) dt_highlight_settings: OffsetsDtHighlightSettings,
    pub(super) dt_local_player_exclusive: OffsetsDtLocalPlayerExclusive,
    pub(super) dt_player: OffsetsDtPlayer,
    pub(super) dt_player_vehicle: OffsetsDtPlayerVehicle,
    pub(super) dt_player_waypoint: OffsetsDtPlayerWaypoint,
    pub(super) dt_projectile: OffsetsDtProjectile,
    pub(super) dt_prop_survival: OffsetsDtPropSurvival,
    pub(super) dt_weapon_player_data: OffsetsDtWeaponPlayerData,
    pub(super) dt_weapon_x: OffsetsDtWeaponX,
    pub(super) dt_weapon_x_local_weapon_data: OffsetsDtWeaponXLocalWeaponData,
    pub(super) dt_weapon_x_predicting_client_only: OffsetsDtWeaponXPredictingClientOnly,
    pub(super) dt_world: OffsetsDtWorld,
    pub(super) weapon_settings_meta: OffsetsWeaponSettingsMeta,
    pub(super) weapon_settings: OffsetsWeaponSettings,
    pub(super) dt_global_non_rewinding: OffsetsDtGlobalNonRewinding,
    pub(super) dt_grapple_data: OffsetsDtGrappleData,
    pub(super) c_base_entity0: OffsetsDataMapCBaseEntity0,
    pub(super) c_base_entity1: OffsetsDataMapCBaseEntity1,
    pub(super) c_base_combat_character: OffsetsDataMapCBaseCombatCharacter,
    pub(super) weapon_inventory_client: OffsetsDataMapWeaponInventoryClient,
    pub(super) c_player: OffsetsDataMapCPlayer,
}

pub fn read_offsets(conf: ini::Ini) -> anyhow::Result<OffsetsData> {
    let data = OffsetsData {
        miscellaneous: OffsetsMiscellaneous::from_ini(&conf)?,
        networked_string_tables: OffsetsNetworkedStringTables::from_ini(&conf)?,
        modifier_offsets: OffsetsModifierOffsets::from_ini(&conf)?,
        buttons: OffsetsButtons::from_ini(&conf)?,
        con_vars: OffsetsConVars::from_ini(&conf)?,
        dt_base_animating: OffsetsDtBaseAnimating::from_ini(&conf)?,
        dt_base_combat_character: OffsetsDtBaseCombatCharacter::from_ini(&conf)?,
        dt_base_entity: OffsetsDtBaseEntity::from_ini(&conf)?,
        dt_collision_property: OffsetsDtCollisionProperty::from_ini(&conf)?,
        dt_highlight_settings: OffsetsDtHighlightSettings::from_ini(&conf)?,
        dt_local_player_exclusive: OffsetsDtLocalPlayerExclusive::from_ini(&conf)?,
        dt_player: OffsetsDtPlayer::from_ini(&conf)?,
        dt_player_vehicle: OffsetsDtPlayerVehicle::from_ini(&conf)?,
        dt_player_waypoint: OffsetsDtPlayerWaypoint::from_ini(&conf)?,
        dt_projectile: OffsetsDtProjectile::from_ini(&conf)?,
        dt_prop_survival: OffsetsDtPropSurvival::from_ini(&conf)?,
        dt_weapon_x: OffsetsDtWeaponX::from_ini(&conf)?,
        dt_weapon_x_local_weapon_data: OffsetsDtWeaponXLocalWeaponData::from_ini(&conf)?,
        dt_weapon_player_data: OffsetsDtWeaponPlayerData::from_ini(&conf)?,
        dt_weapon_x_predicting_client_only: OffsetsDtWeaponXPredictingClientOnly::from_ini(&conf)?,
        dt_world: OffsetsDtWorld::from_ini(&conf)?,
        weapon_settings_meta: OffsetsWeaponSettingsMeta::from_ini(&conf)?,
        weapon_settings: OffsetsWeaponSettings::from_ini(&conf)?,
        dt_global_non_rewinding: OffsetsDtGlobalNonRewinding::from_ini(&conf)?,
        dt_grapple_data: OffsetsDtGrappleData::from_ini(&conf)?,
        c_base_entity0: OffsetsDataMapCBaseEntity0::from_ini(&conf)?,
        c_base_entity1: OffsetsDataMapCBaseEntity1::from_ini(&conf)?,
        c_base_combat_character: OffsetsDataMapCBaseCombatCharacter::from_ini(&conf)?,
        weapon_inventory_client: OffsetsDataMapWeaponInventoryClient::from_ini(&conf)?,
        c_player: OffsetsDataMapCPlayer::from_ini(&conf)?,
    };

    // assert_offsets_eq!(
    //     off!(data.miscellaneous.cweapon_x_m_fl_projectile_speed),
    //     off!(data.weapon_settings_meta.base) + off!(data.weapon_settings.projectile_launch_speed)
    // );
    // assert_offsets_eq!(
    //     off!(data.miscellaneous.cweapon_x_m_fl_projectile_scale),
    //     off!(data.weapon_settings_meta.base) + off!(data.weapon_settings.projectile_gravity_scale)
    // );
    assert_offsets_eq!(
        off!(data.c_base_combat_character.m_inventory),
        off!(data.dt_player.m_inventory)
    );
    assert_offsets_eq!(
        off!(data.c_base_combat_character.m_selected_weapons),
        off!(data.dt_base_combat_character.m_selected_weapons)
    );
    assert_offsets_eq!(
        off!(data.c_base_combat_character.m_latest_primary_weapons),
        off!(data.dt_base_combat_character.m_latest_primary_weapons)
    );
    assert_offsets_eq!(
        off!(data.c_base_combat_character.m_latest_non_offhand_weapons),
        off!(data.dt_base_combat_character.m_latest_non_offhand_weapons)
    );
    assert_offsets_eq!(
        off!(
            data.c_base_combat_character
                .m_hud_info_visibility_test_always_passes
        ),
        off!(
            data.dt_base_combat_character
                .m_hud_info_visibility_test_always_passes
        )
    );

    assert_offsets_eq!(
        off!(data.c_base_entity0.m_f_flags),
        off!(data.dt_player.m_f_flags)
    );
    assert_offsets_eq!(
        off!(data.c_base_entity1.m_f_flags),
        off!(data.dt_player.m_f_flags)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_f_flags),
        off!(data.dt_player.m_f_flags)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_b_zooming),
        off!(data.dt_player.m_b_zooming)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_zoom_base_frac),
        off!(data.dt_player.m_zoom_base_frac)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_zoom_base_time),
        off!(data.dt_player.m_zoom_base_time)
    );
    assert_offsets_eq!(off!(data.c_player.pl), off!(data.dt_player.pl));
    assert_offsets_eq!(
        off!(data.c_player.m_traversal_progress),
        off!(data.dt_local_player_exclusive.m_traversal_progress)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_traversal_start_time),
        off!(data.dt_local_player_exclusive.m_traversal_start_time)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_traversal_release_time),
        off!(data.dt_local_player_exclusive.m_traversal_release_time)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_wall_run_start_time),
        off!(data.dt_local_player_exclusive.m_wall_run_start_time)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_wall_run_clear_time),
        off!(data.dt_local_player_exclusive.m_wall_run_clear_time)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_h_view_models),
        off!(data.dt_player.m_h_view_models)
    );
    assert_offsets_eq!(
        off!(data.c_player.m_skydive_state),
        off!(data.dt_player.m_skydive_state)
    );

    assert_offsets_eq!(
        off!(data.dt_projectile.m_vec_velocity),
        off!(data.dt_local_player_exclusive.m_vec_velocity_)
    );
    assert_offsets_eq!(
        off!(data.c_base_entity1.m_vec_velocity),
        off!(data.dt_projectile.m_vec_velocity)
    );

    Ok(data)
}

export_custom_offsets!(
    ~OffsetsData~(^_^) ~CustomOffsets(~_~)=> offsets {
        time_date_stamp: off!(offsets.miscellaneous.time_date_stamp),
        checksum: off!(offsets.miscellaneous.check_sum),
        global_vars: off!(offsets.miscellaneous.global_vars),
        entitylist: off!(offsets.miscellaneous.cl_entitylist),
        local_entity_handle: off!(offsets.miscellaneous.local_entity_handle),
        local_ent: off!(offsets.miscellaneous.local_player),
        input_selected_slot: 0x180,
        client_state: off!(offsets.miscellaneous.client_state),
        signon_state: off!(offsets.miscellaneous.signon_state),
        level_name: off!(offsets.miscellaneous.level_name),
        nst_weapon_names: off!(offsets.networked_string_tables.weapon_names),
        view_render: off!(offsets.miscellaneous.view_render),
        view_matrix: off!(offsets.miscellaneous.view_matrix),
        input_system: off!(offsets.miscellaneous.input_system),
        input_button_state: 0xb0,
        name_list: off!(offsets.miscellaneous.name_list),
        highlight_settings: off!(offsets.miscellaneous.highlight_settings),
        spectator_list: off!(offsets.miscellaneous.observer_list),
        network_var_table_ptr: off!(offsets.miscellaneous.network_var_table_ptr),
        network_var_table_len: offsets.miscellaneous.network_var_table_len.parse::<u64>().unwrap(),
        host_map: off!(offsets.con_vars.host_map),
        thirdperson_override: off!(offsets.con_vars.thirdperson_override),
        mouse_sensitivity: off!(offsets.con_vars.mouse_sensitivity),
        fps_max: off!(offsets.con_vars.fps_max),
        mp_gamemode: off!(offsets.con_vars.mp_gamemode),
        in_attack: off!(offsets.buttons.in_attack),
        in_jump: off!(offsets.buttons.in_jump),
        in_duck: off!(offsets.buttons.in_duck),
        in_reload: off!(offsets.buttons.in_reload),
        in_use: off!(offsets.buttons.in_use),
        in_zoom: off!(offsets.buttons.in_zoom),
        in_forward: off!(offsets.buttons.in_forward),
        in_backward: off!(offsets.buttons.in_backward),
        in_moveleft: off!(offsets.buttons.in_moveleft),
        in_moveright: off!(offsets.buttons.in_moveright),
        in_toggle_duck: off!(offsets.buttons.in_toggle_duck),
        in_left: off!(offsets.buttons.in_left),
        in_right: off!(offsets.buttons.in_right),
        in_strafe: off!(offsets.buttons.in_strafe),
        centity_modelname: off!(offsets.c_base_entity0.m_model_name),
        centity_viewoffset: off!(offsets.c_base_entity1.m_current_frame_view_offset),
        centity_flags: off!(offsets.c_base_entity0.m_f_flags),
        centity_origin: off!(offsets.c_base_entity1.m_vec_abs_origin),
        entity_shieldhealth: off!(offsets.dt_base_entity.m_shield_health),
        entity_maxshieldhealth: off!(offsets.dt_base_entity.m_shield_health_max),
        entity_highlight_generic_context: off!(offsets.dt_base_entity.highlight_settings) + off!(offsets.dt_highlight_settings.m_highlight_generic_contexts),
        entity_team_num: off!(offsets.dt_base_entity.m_i_team_num),
        centity_abs_velocity: off!(offsets.c_base_entity1.m_vec_abs_velocity),
        centity_velocity: off!(offsets.c_base_entity1.m_vec_velocity),
        entity_owner_entity: off!(offsets.dt_base_entity.m_h_owner_entity),
        entity_collision: off!(offsets.dt_base_entity.m_collision),
        entity_collision_group: off!(offsets.dt_base_entity.m_collision_group),
        entiry_name: off!(offsets.dt_base_entity.m_i_name),
        entity_sign_name: off!(offsets.dt_base_entity.m_i_signifier_name),
        entity_fade_dist: off!(offsets.dt_base_entity.m_fade_dist),
        collision_property_vec_mins: off!(offsets.dt_collision_property.m_vec_mins),
        collision_property_vec_maxs: off!(offsets.dt_collision_property.m_vec_maxs),
        animating_skin: off!(offsets.dt_base_animating.m_n_skin),
        animating_bone_array: off!(offsets.dt_base_animating.m_b_sequence_finished) - 0x1c,
        bones: 0x48 + off!(offsets.dt_base_animating.m_n_force_bone),
        animating_studiohdr: off!(offsets.miscellaneous.cbase_animating_m_p_studio_hdr),
        bcc_next_attack: off!(offsets.c_base_combat_character.m_fl_next_attack),
        bcc_last_fired_time: off!(offsets.dt_base_combat_character.m_last_fired_time),
        bcc_last_fired_weapon: off!(offsets.dt_base_combat_character.m_last_fired_weapon),
        bcc_raise_from_melee_end_time: off!(offsets.dt_base_combat_character.m_raise_from_melee_end_time),
        bcc_inventory: off!(offsets.c_base_combat_character.m_inventory),
        bcc_selected_weapons: off!(offsets.c_base_combat_character.m_selected_weapons),
        bcc_off_weapon: off!(offsets.c_base_combat_character.m_latest_non_offhand_weapons),
        bcc_primary_weapon: off!(offsets.c_base_combat_character.m_latest_primary_weapons),
        bcc_active_weapon: off!(offsets.c_base_combat_character.m_inventory) + off!(offsets.weapon_inventory_client.active_weapons),
        bcc_last_visible_time: 0x3 + off!(offsets.c_base_combat_character.m_hud_info_visibility_test_always_passes),
        player_last_visible_time: off!(offsets.miscellaneous.cplayer_last_visible_time),
        player_zooming: off!(offsets.dt_player.m_b_zooming),
        cplayer_camerapos: off!(offsets.miscellaneous.cplayer_camera_origin),
        cplayer_timebase: off!(offsets.c_player.m_current_frame_player_time_base),
        cplayer_server_angles: 0x4 + off!(offsets.c_player.m_current_frame_player_m_ziprail_bank_tilt_frac),
        cplayer_aimpunch: off!(offsets.c_player.m_current_frame_local_player_m_vec_punch_weapon_angle),
        cplayer_viewmodels: off!(offsets.c_player.m_h_view_models),
        cplayer_traversal_progress: off!(offsets.c_player.m_traversal_progress),
        cplayer_traversal_starttime: off!(offsets.c_player.m_traversal_start_time),
        cplayer_wall_run_start_time: off!(offsets.c_player.m_wall_run_start_time),
        cplayer_wall_run_clear_time: off!(offsets.c_player.m_wall_run_clear_time),
        player_viewangles: off!(offsets.dt_player.m_ammo_pool_capacity) - 0x14,
        player_consumables: off!(offsets.dt_local_player_exclusive.m_consumable_inventory),
        player_traversal_release_time: off!(offsets.c_player.m_traversal_release_time),
        player_observer_state: off!(offsets.dt_local_player_exclusive.m_i_observer_mode),
        player_ovserver_target: off!(offsets.dt_local_player_exclusive.m_h_observer_target),
        player_platform_uid: off!(offsets.dt_player.m_platform_user_id),
        player_health: off!(offsets.dt_player.m_i_health),
        player_maxhealth: off!(offsets.dt_player.m_i_max_health),
        player_bleed_out_state: off!(offsets.dt_player.m_bleedout_state),
        player_life_state: off!(offsets.dt_player.m_life_state),
        player_duck_state: off!(offsets.dt_player.m_duck_state),
        player_lean_state: off!(offsets.dt_player.m_lean_state),
        player_grapple: off!(offsets.dt_player.m_grapple),
        player_grapple_active: off!(offsets.dt_player.m_grapple_active),
        player_shadow_shield_active: off!(offsets.dt_player.m_shadow_shield_active),
        player_temp_shield_health: off!(offsets.dt_player.m_temp_shield_health),
        player_extra_shield_health: off!(offsets.dt_player.m_extra_shield_health),
        player_extra_shield_tier: off!(offsets.dt_player.m_extra_shield_tier),
        player_is_performing_boost_action: off!(offsets.dt_player.m_is_performing_boost_action),
        player_xp: off!(offsets.dt_player.m_xp),
        player_third_person_shoulder_view: off!(offsets.dt_local_player_exclusive.m_third_person_shoulder_view),
        player_net_var: off!(offsets.dt_player.m_player_script_net_data_global),
        player_helmettype: off!(offsets.dt_player.m_helmet_type),
        player_armortype: off!(offsets.dt_player.m_armor_type),
        player_controller_active: off!(offsets.dt_player.m_controller_mode_active),
        player_skydive_state: off!(offsets.dt_player.m_skydive_state),
        player_breath_angles: off!(offsets.dt_player.m_ammo_pool_capacity) - 0x24,
        weaponx_weapon_owner: off!(offsets.dt_weapon_x.m_weapon_owner),
        weaponx_next_primary_attack: off!(offsets.dt_weapon_x.local_weapon_data) + off!(offsets.dt_weapon_x_local_weapon_data.m_next_primary_attack_time),
        weaponx_ammo_in_clip: off!(offsets.dt_weapon_x.local_weapon_data) + off!(offsets.dt_weapon_x_local_weapon_data.m_ammo_in_clip),
        weaponx_zoom_fov: off!(offsets.dt_weapon_x.m_player_data) + off!(offsets.dt_weapon_player_data.m_cur_zoom_fov),
        weaponx_charge_start_time: off!(offsets.dt_weapon_x.m_charge_start_time),
        weaponx_charge_end_time: off!(offsets.dt_weapon_x.m_charge_end_time),
        weaponx_last_charge_frac: off!(offsets.dt_weapon_x.m_last_charge_frac),
        cweaponx_burst_fire: off!(offsets.dt_weapon_x_predicting_client_only.m_burst_fire_count),
        cweaponx_crosshair_last: off!(offsets.miscellaneous.cweapon_x_last_crosshair_target_time),
        weaponx_bitfield_from_player: off!(offsets.dt_weapon_x.m_mod_bitfield_from_player),
        weaponx_weapon_name_index: off!(offsets.dt_weapon_x.m_weapon_name_index),
        weaponx_printname: off!(offsets.weapon_settings_meta.base) + off!(offsets.weapon_settings.printname),
        weaponx_shortprintname: off!(offsets.weapon_settings_meta.base) + off!(offsets.weapon_settings.shortprintname),
        weaponx_is_semi_auto: off!(offsets.weapon_settings_meta.base) + off!(offsets.weapon_settings.is_semi_auto),
        weaponx_ammo_clip_size: off!(offsets.weapon_settings_meta.base) + off!(offsets.weapon_settings.ammo_clip_size),
        weaponx_projectile_launch_speed: off!(offsets.miscellaneous.cweapon_x_m_fl_projectile_speed),
        weaponx_projectile_gravity_scale: off!(offsets.miscellaneous.cweapon_x_m_fl_projectile_scale),
        vehicle_driver: off!(offsets.dt_player_vehicle.m_vehicle_driver),
        vehicle_velocity: off!(offsets.dt_player_vehicle.m_vehicle_velocity),
        prop_survival: off!(offsets.dt_prop_survival.m_ammo_in_clip),
        projectile: off!(offsets.dt_projectile.m_weapon_data_is_set),
        world_death_field: off!(offsets.dt_world.m_death_field_is_active),
        waypoint_type: off!(offsets.dt_player_waypoint.m_waypoint_type),
        mods_names: off!(offsets.modifier_offsets.mods_names),
        mods_list: off!(offsets.modifier_offsets.mods_list),
        mods_count: off!(offsets.modifier_offsets.mods_count),
        grapple_attached: off!(offsets.dt_grapple_data.m_grapple_attached),
        grapple_pulling: off!(offsets.dt_grapple_data.m_grapple_pulling),
        global_observer_mode: off!(offsets.dt_global_non_rewinding.m_player_observer),
    }
);

pub fn export_offsets_from_str(s: &str) -> anyhow::Result<CustomOffsets> {
    use obfstr::obfstr as s;
    let data = ini::Ini::load_from_str(s).expect(s!("Failed to parse offsets."));
    let data = read_offsets(data)?;
    let data = CustomOffsets::from(data);
    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    fn export_offsets_from_file(path: Option<String>) -> anyhow::Result<CustomOffsets> {
        use obfstr::obfstr as s;
        use std::path::PathBuf;
        let path = path.map(PathBuf::from).unwrap_or(
            std::env::current_dir()
                .expect(s!("Failed to determine the current directory"))
                .join(s!("mnt"))
                .join(s!("host"))
                .join(s!("offsets.ini")),
        );
        let data = ini::Ini::load_from_file(path).expect(s!("Failed to read offsets file."));
        let data = read_offsets(data)?;
        let data = CustomOffsets::from(data);
        Ok(data)
    }

    #[test]
    fn test_parse_offsets() {
        let _offsets = export_offsets_from_file(Some(
            obfstr::obfstr!("../apex1_common/resource/default/offsets.ini").to_string(),
        ))
        .unwrap();
    }
}
