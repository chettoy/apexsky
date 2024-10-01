use apexsky::offsets::G_OFFSETS;

#[derive(Debug)]
pub struct GameData {
    pub global_vars: u32,

    pub entity_list: u32,
    pub local_entity_handle: u32,

    pub client_state: u32,
    pub signon_state: u32,
    pub level_name: u32,

    pub nst_weapon_names: u32,

    pub view_render: u32,
    pub view_matrix: u32,

    pub input_system: u32,
    pub input_button_state: u32,

    pub name_list: u32,

    pub network_var_table_ptr: u32,
    pub network_var_table_len: u32,

    pub thirdperson_override: u32,
    pub mouse_sensitivity: u32,
    pub fps_max: u32,
    pub mp_gamemode: u32,

    pub in_attack: u32,
    pub in_jump: u32,
    pub in_duck: u32,
    pub in_reload: u32,
    pub in_use: u32,
    pub in_zoom: u32,
    pub in_forward: u32,
    pub in_backward: u32,
    pub in_moveleft: u32,
    pub in_moveright: u32,

    pub entity_model_name: u32,
    pub entity_view_offset: u32,
    pub entity_flags: u32,
    pub entity_origin: u32,
    pub entity_shield_health: u32,
    pub entity_health: u32,
    pub entity_team_num: u32,
    pub entity_velocity: u32,
    pub entity_owner_entity: u32,
    pub entity_collision: u32,
    pub entity_collision_group: u32,
    pub entity_max_health: u32,
    pub entity_life_state: u32,

    pub animating_skin: u32,
    pub animating_bone_array: u32, // m_bSequenceFinished - 0x1C
    pub animating_studiohdr: u32,  // m_flModelScale + 0x1D0

    pub collision_property_vec_mins: u32,
    pub collision_property_vec_maxs: u32,

    pub bcc_next_attack: u32,
    pub bcc_inventory: u32,
    pub bcc_selected_weapons: u32,
    pub bcc_last_visible_time: u32, // m_hudInfo_visibilityTestAlwaysPasses + 0x3
    pub player_last_visible_time: u32,

    pub player_zoom_state: u32,
    pub player_camera_data: u32,
    pub player_time_base: u32,
    pub player_server_angles: u32,
    pub player_view_angles: u32,
    pub player_weapon_punch: u32,
    pub player_consumables: u32,
    pub player_platform_uid: u32,
    pub player_bleedout_state: u32,
    pub player_movement_state: u32,
    pub player_observer_state: u32,
    pub player_third_person_shoulder_view: u32,
    pub player_script_net_data: u32,
    pub player_helmet_armor_type: u32,
    pub player_shadow_shield_active: u32,
    pub player_temp_shield_health: u32,
    pub player_extra_shield_health: u32,
    pub player_extra_shield_tier: u32,
    pub player_is_performing_boost_action: u32,
    pub player_xp: u32,
    pub player_controller_active: u32,
    pub player_skydive_state: u32,

    pub weaponx_weapon_owner: u32,
    pub weaponx_next_primary_attack: u32,
    pub weaponx_ammo_in_clip: u32,
    pub weaponx_zoom_fov: u32,
    pub weaponx_mod_bitfield: u32,
    pub weaponx_weapon_name_index: u32,
    pub weaponx_is_semi_auto: u32,
    pub weaponx_ammo_clip_size: u32,
    pub weaponx_projectile_speed: u32,
    pub weaponx_projectile_scale: u32,
    pub weaponx_projectile_air_friction: u32,
    pub weaponx_charge_start_time: u32,
    pub weaponx_burst_fire: u32,

    pub vehicle_driver: u32,
    pub vehicle_velocity: u32,

    pub prop_survival: u32,
    pub projectile: u32,
    pub world_death_field: u32,
    pub waypoint_type: u32,

    pub mods_names: u32,
    pub mods_list: u32,
    pub mods_count: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            global_vars: G_OFFSETS.global_vars.try_into().unwrap(),
            entity_list: G_OFFSETS.entitylist.try_into().unwrap(),
            local_entity_handle: G_OFFSETS.local_entity_handle.try_into().unwrap(),
            client_state: G_OFFSETS.client_state.try_into().unwrap(),
            signon_state: G_OFFSETS.signon_state.try_into().unwrap(),
            level_name: G_OFFSETS.level_name.try_into().unwrap(),
            nst_weapon_names: G_OFFSETS.nst_weapon_names.try_into().unwrap(),
            view_render: G_OFFSETS.view_render.try_into().unwrap(),
            view_matrix: G_OFFSETS.view_matrix.try_into().unwrap(),
            input_system: G_OFFSETS.input_system.try_into().unwrap(),
            input_button_state: G_OFFSETS.input_button_state.try_into().unwrap(),
            name_list: G_OFFSETS.name_list.try_into().unwrap(),
            network_var_table_ptr: G_OFFSETS.network_var_table_ptr.try_into().unwrap(),
            network_var_table_len: G_OFFSETS.network_var_table_len.try_into().unwrap(),
            thirdperson_override: G_OFFSETS.thirdperson_override.try_into().unwrap(),
            mouse_sensitivity: G_OFFSETS.mouse_sensitivity.try_into().unwrap(),
            fps_max: G_OFFSETS.fps_max.try_into().unwrap(),
            mp_gamemode: G_OFFSETS.mp_gamemode.try_into().unwrap(),
            in_attack: G_OFFSETS.in_attack.try_into().unwrap(),
            in_jump: G_OFFSETS.in_jump.try_into().unwrap(),
            in_duck: G_OFFSETS.in_duck.try_into().unwrap(),
            in_reload: G_OFFSETS.in_reload.try_into().unwrap(),
            in_use: G_OFFSETS.in_use.try_into().unwrap(),
            in_zoom: G_OFFSETS.in_zoom.try_into().unwrap(),
            in_forward: G_OFFSETS.in_forward.try_into().unwrap(),
            in_backward: G_OFFSETS.in_backward.try_into().unwrap(),
            in_moveleft: G_OFFSETS.in_moveleft.try_into().unwrap(),
            in_moveright: G_OFFSETS.in_moveright.try_into().unwrap(),
            entity_model_name: G_OFFSETS.centity_modelname.try_into().unwrap(),
            entity_view_offset: G_OFFSETS.centity_viewoffset.try_into().unwrap(),
            entity_flags: G_OFFSETS.centity_flags.try_into().unwrap(),
            entity_origin: G_OFFSETS.centity_origin.try_into().unwrap(),
            entity_shield_health: G_OFFSETS.entity_shieldhealth.try_into().unwrap(),
            entity_health: G_OFFSETS.player_health.try_into().unwrap(),
            entity_team_num: G_OFFSETS.entity_team_num.try_into().unwrap(),
            entity_velocity: G_OFFSETS.centity_velocity.try_into().unwrap(),
            entity_owner_entity: G_OFFSETS.entity_owner_entity.try_into().unwrap(),
            entity_collision: G_OFFSETS.entity_collision.try_into().unwrap(),
            entity_collision_group: G_OFFSETS.entity_collision_group.try_into().unwrap(),
            entity_max_health: G_OFFSETS.player_maxhealth.try_into().unwrap(),
            entity_life_state: G_OFFSETS.player_life_state.try_into().unwrap(),
            animating_skin: G_OFFSETS.animating_skin.try_into().unwrap(),
            animating_bone_array: G_OFFSETS.animating_bone_array.try_into().unwrap(),
            animating_studiohdr: G_OFFSETS.animating_studiohdr.try_into().unwrap(),
            collision_property_vec_mins: G_OFFSETS.collision_property_vec_mins.try_into().unwrap(),
            collision_property_vec_maxs: G_OFFSETS.collision_property_vec_maxs.try_into().unwrap(),
            bcc_next_attack: G_OFFSETS.bcc_next_attack.try_into().unwrap(),
            bcc_inventory: G_OFFSETS.bcc_inventory.try_into().unwrap(),
            bcc_selected_weapons: G_OFFSETS.bcc_selected_weapons.try_into().unwrap(),
            bcc_last_visible_time: G_OFFSETS.bcc_last_visible_time.try_into().unwrap(),
            player_last_visible_time: G_OFFSETS.player_last_visible_time.try_into().unwrap(),
            player_zoom_state: G_OFFSETS.player_zooming.try_into().unwrap(),
            player_camera_data: G_OFFSETS.cplayer_camerapos.try_into().unwrap(),
            player_time_base: G_OFFSETS.cplayer_timebase.try_into().unwrap(),
            player_server_angles: G_OFFSETS.cplayer_server_angles.try_into().unwrap(),
            player_view_angles: G_OFFSETS.player_viewangles.try_into().unwrap(),
            player_weapon_punch: G_OFFSETS.cplayer_aimpunch.try_into().unwrap(),
            player_consumables: G_OFFSETS.player_consumables.try_into().unwrap(),
            player_platform_uid: G_OFFSETS.player_platform_uid.try_into().unwrap(),
            player_bleedout_state: G_OFFSETS.player_bleed_out_state.try_into().unwrap(),
            player_movement_state: G_OFFSETS.player_duck_state.try_into().unwrap(),
            player_observer_state: G_OFFSETS.player_observer_state.try_into().unwrap(),
            player_third_person_shoulder_view: G_OFFSETS
                .player_third_person_shoulder_view
                .try_into()
                .unwrap(),
            player_script_net_data: G_OFFSETS.player_net_var.try_into().unwrap(),
            player_helmet_armor_type: G_OFFSETS.player_helmettype.try_into().unwrap(),
            player_shadow_shield_active: G_OFFSETS.player_shadow_shield_active.try_into().unwrap(),
            player_temp_shield_health: G_OFFSETS.player_temp_shield_health.try_into().unwrap(),
            player_extra_shield_health: G_OFFSETS.player_extra_shield_health.try_into().unwrap(),
            player_extra_shield_tier: G_OFFSETS.player_extra_shield_tier.try_into().unwrap(),
            player_is_performing_boost_action: G_OFFSETS
                .player_is_performing_boost_action
                .try_into()
                .unwrap(),
            player_xp: G_OFFSETS.player_xp.try_into().unwrap(),
            player_controller_active: G_OFFSETS.player_controller_active.try_into().unwrap(),
            player_skydive_state: G_OFFSETS.player_skydive_state.try_into().unwrap(),
            weaponx_weapon_owner: G_OFFSETS.weaponx_weapon_owner.try_into().unwrap(),
            weaponx_next_primary_attack: G_OFFSETS.weaponx_next_primary_attack.try_into().unwrap(),
            weaponx_ammo_in_clip: G_OFFSETS.weaponx_ammo_in_clip.try_into().unwrap(),
            weaponx_zoom_fov: G_OFFSETS.weaponx_zoom_fov.try_into().unwrap(),
            weaponx_mod_bitfield: G_OFFSETS.weaponx_bitfield_from_player.try_into().unwrap(),
            weaponx_weapon_name_index: G_OFFSETS.weaponx_weapon_name_index.try_into().unwrap(),
            weaponx_is_semi_auto: G_OFFSETS.weaponx_is_semi_auto.try_into().unwrap(),
            weaponx_ammo_clip_size: G_OFFSETS.weaponx_ammo_clip_size.try_into().unwrap(),
            weaponx_projectile_speed: G_OFFSETS
                .weaponx_projectile_launch_speed
                .try_into()
                .unwrap(),
            weaponx_projectile_scale: G_OFFSETS
                .weaponx_projectile_gravity_scale
                .try_into()
                .unwrap(),
            weaponx_projectile_air_friction: G_OFFSETS
                .weaponx_projectile_air_friction
                .try_into()
                .unwrap(),
            weaponx_charge_start_time: G_OFFSETS.weaponx_charge_start_time.try_into().unwrap(),
            weaponx_burst_fire: G_OFFSETS.cweaponx_burst_fire.try_into().unwrap(),
            vehicle_driver: G_OFFSETS.vehicle_driver.try_into().unwrap(),
            vehicle_velocity: G_OFFSETS.vehicle_velocity.try_into().unwrap(),
            prop_survival: G_OFFSETS.prop_survival.try_into().unwrap(),
            projectile: G_OFFSETS.projectile.try_into().unwrap(),
            world_death_field: G_OFFSETS.world_death_field.try_into().unwrap(),
            waypoint_type: G_OFFSETS.waypoint_type.try_into().unwrap(),
            mods_names: G_OFFSETS.mods_names.try_into().unwrap(),
            mods_list: G_OFFSETS.mods_list.try_into().unwrap(),
            mods_count: G_OFFSETS.mods_count.try_into().unwrap(),
        }
    }
}
