#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerState {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub uid: i64,
    #[prost(int32, tag = "3")]
    pub entity_team: i32,
    #[prost(bool, tag = "4")]
    pub knocked: bool,
    #[prost(int32, tag = "5")]
    pub health: i32,
    #[prost(int32, tag = "6")]
    pub shield: i32,
    #[prost(int32, tag = "7")]
    pub max_shield: i32,
    #[prost(int32, tag = "8")]
    pub xp_level: i32,
    #[prost(int32, tag = "9")]
    pub damage: i32,
    #[prost(int32, tag = "10")]
    pub armor_type: i32,
    #[prost(float, tag = "11")]
    pub position_x: f32,
    #[prost(float, tag = "12")]
    pub position_y: f32,
    #[prost(float, tag = "13")]
    pub position_z: f32,
    #[prost(float, tag = "14")]
    pub yaw: f32,
    #[prost(bool, tag = "15")]
    pub is_alive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Players {
    #[prost(message, repeated, tag = "1")]
    pub players: ::prost::alloc::vec::Vec<PlayerState>,
}
