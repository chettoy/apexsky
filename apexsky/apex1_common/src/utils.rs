pub fn teammate_check(
    entity_team: i32,
    local_team: i32,
    alter_local_team: i32,
    team_death_match_mode: bool,
) -> bool {
    if team_death_match_mode {
        let ent_team = if entity_team % 2 == 1 { 1 } else { 2 };
        let loc_team = if local_team % 2 == 1 { 1 } else { 2 };
        tracing::trace!(
            target_team = ent_team,
            local_team = loc_team,
            "{}",
            obfstr::obfstr!("TDM check")
        );
        ent_team == loc_team
    } else {
        entity_team == local_team || (alter_local_team != 0 && entity_team == alter_local_team)
    }
}

/// Function to get the Unix timestamp in milliseconds
pub fn get_unix_timestamp_in_millis() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            // Calculate the total milliseconds from the duration
            duration.as_secs() * 1000 + duration.subsec_millis() as u64
        }
        Err(e) => {
            // Handle errors, such as clock rollback
            panic!("{}{}", obfstr::obfstr!("Error getting Unix Timestamp: "), e);
        }
    }
}

/// Function to get the Unix timestamp in seconds
pub fn get_unix_timestamp_in_seconds() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(e) => {
            // Handle errors, such as clock rollback
            panic!("{}{}", obfstr::obfstr!("Error getting Unix Timestamp: "), e);
        }
    }
}
