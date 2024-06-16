use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy)]
pub enum ContentId {
    Connected,
    DualTeamsNearby,
    EnemyInTheRear,
    UnderObservation,
}

pub(super) static ASSETS: Lazy<[(&str, Vec<u8>); 13]> = Lazy::new(|| {
    [
        (
            "default-0-1",
            include_bytes!("../assets/game_attached/1.wav").to_vec(),
        ),
        (
            "default-0-2",
            include_bytes!("../assets/game_attached/2.wav").to_vec(),
        ),
        (
            "default-0-3",
            include_bytes!("../assets/game_attached/3.wav").to_vec(),
        ),
        (
            "default-1-1",
            include_bytes!("../assets/dual_teams_nearby/1.wav").to_vec(),
        ),
        (
            "default-2-1",
            include_bytes!("../assets/enemy_in_the_rear/1.wav").to_vec(),
        ),
        (
            "default-2-2",
            include_bytes!("../assets/enemy_in_the_rear/2.wav").to_vec(),
        ),
        (
            "default-2-3",
            include_bytes!("../assets/enemy_in_the_rear/3.wav").to_vec(),
        ),
        (
            "default-2-4",
            include_bytes!("../assets/enemy_in_the_rear/4.wav").to_vec(),
        ),
        (
            "default-2-5",
            include_bytes!("../assets/enemy_in_the_rear/5.wav").to_vec(),
        ),
        (
            "default-3-1",
            include_bytes!("../assets/under_observation/1.wav").to_vec(),
        ),
        (
            "default-3-2",
            include_bytes!("../assets/under_observation/2.wav").to_vec(),
        ),
        (
            "default-3-3",
            include_bytes!("../assets/under_observation/3.wav").to_vec(),
        ),
        (
            "default-3-4",
            include_bytes!("../assets/under_observation/4.wav").to_vec(),
        ),
    ]
});