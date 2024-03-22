#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vec3 {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
    #[prost(float, tag = "3")]
    pub z: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge {
    #[prost(int32, tag = "1")]
    pub badge_index: i32,
    #[prost(int32, tag = "2")]
    pub badge_data: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerState {
    #[prost(message, optional, tag = "2")]
    pub origin: ::core::option::Option<Vec3>,
    #[prost(message, optional, tag = "3")]
    pub view_angles: ::core::option::Option<Vec3>,
    #[prost(message, optional, tag = "4")]
    pub velocity: ::core::option::Option<Vec3>,
    #[prost(int32, tag = "5")]
    pub health: i32,
    #[prost(int32, tag = "6")]
    pub shield: i32,
    #[prost(int32, tag = "7")]
    pub max_health: i32,
    #[prost(int32, tag = "8")]
    pub max_shield: i32,
    #[prost(int32, tag = "9")]
    pub helmet_type: i32,
    #[prost(int32, tag = "10")]
    pub armor_type: i32,
    #[prost(int32, tag = "11")]
    pub team_num: i32,
    #[prost(int32, tag = "12")]
    pub xp: i32,
    #[prost(int32, tag = "13")]
    pub flags: i32,
    #[prost(bool, tag = "14")]
    pub is_alive: bool,
    #[prost(bool, tag = "15")]
    pub is_knocked: bool,
    #[prost(int32, tag = "16")]
    pub love_state: i32,
    #[prost(int32, tag = "17")]
    pub active_weapon: i32,
    #[prost(string, tag = "18")]
    pub player_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "19")]
    pub platform_uid: u64,
    #[prost(uint64, tag = "20")]
    pub eadp_uid: u64,
    #[prost(message, optional, tag = "21")]
    pub head_position: ::core::option::Option<Vec3>,
    #[prost(bool, tag = "22")]
    pub controller_active: bool,
    #[prost(int32, tag = "23")]
    pub character_index: i32,
    #[prost(message, repeated, tag = "24")]
    pub badges: ::prost::alloc::vec::Vec<Badge>,
    #[prost(int32, tag = "25")]
    pub kills: i32,
    #[prost(int32, tag = "26")]
    pub damage_dealt: i32,
    #[prost(bool, tag = "27")]
    pub kill_leader: bool,
    #[prost(bool, tag = "28")]
    pub winning_team: bool,
    #[prost(float, tag = "29")]
    pub yaw: f32,
    #[prost(int32, tag = "30")]
    pub team_member_index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Players {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(message, repeated, tag = "2")]
    pub players: ::prost::alloc::vec::Vec<PlayerState>,
}
