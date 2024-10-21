use super::global_store::StoreValue;
use super::{define_global, define_msg_name};
use crate::offsets::CustomOffsets;
use crate::pb::apexlegends::{
    AimResultData, AimTargetItem, AimbotState, GameInputState, Matrix4x4, PlayerState, Players,
    SpectatorList, TreasureClue, Vec3,
};
use ohosky_api::common::share::{ISharableValue, RkyvValue};

define_msg_name!(MSG@MessageName {
    request_parse_offset,
    game_attached,
    game_unattached,
    action_tick,
    aimbot_tick,
});

define_global!({
    RecOffsets(RkyvValue<CustomOffsets>): "offsets",
    RecWorkersActive(bool): "workers_active",
    RecSkyConfigCopy(JsonValue<crate::config::Config>): "settings.toml",
    RecGameBaseAddr(u64): "game_baseaddr",
    RecTickNum(i64): "tick_num",
    RecTickDuration(f32): "tick_duration",
    RecTickUpdateTimestamp(f64): "tick_update_timestamp",
    RecActionsDuration(f32): "actions_duration",
    RecAimTargetPosition(RkyvValue<Vec3>): "aim_target_pos",
    RecAimResult(RkyvValue<AimResultData>): "aim_result",
    RecViewMatrix(RkyvValue<Matrix4x4>): "view_matrix",
    RecHighlightInjected(bool): "highlight_injected",
    RecTeammates(RkyvValue<Players>): "teammates",
    RecSpectatorList(RkyvValue<SpectatorList>): "spectator_list",
    RecAlterLocalTeam(i32): "alter_local_team",
    RecWorldReady(bool): "world_ready",
    RecFrameCount(i32): "frame_count",
    RecGameFps(f32): "game_fps",
    RecPlayersMap(RkyvValue<PlayerState>): "players_",
    RecLootsMap(RkyvValue<TreasureClue>): "loots_",
    RecAimTargetsMap(RkyvValue<AimTargetItem>): "aim_targets_",
    RecLocalPlayerPtr(u64): "local_player_ptr",
    RecViewPlayerPtr(u64): "view_player_ptr",
    RecLocalPlayerExt(Vec<u8>): "local_player_ext",
    RecLocalPlayerBuf(RkyvValue<PlayerState>): "local_player_buf",
    RecViewPlayerBuf(RkyvValue<PlayerState>): "view_player_buf",
    RecAimbotState(RkyvValue<AimbotState>): "aimbot_state",
    RecAimbotInstance(RkyvValue<crate::aimbot::Aimbot>): "aimbot_instance",
    RecCurrentZoomFov(f32): "current_zoom_fov",
    RecFiringRangeMode(bool): "g_settings.firing_range",
    RecSettingsTeamDeathMatchMode(bool): "g_settings.tdmtoggle",
    RecGameInputState(RkyvValue<GameInputState>): "game_input_state",
});

macro_rules! impl_store_for_rkyv_value {
    ($type:ident) => {
        impl StoreValue for $type {
            fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>> {
                Ok(RkyvValue::from_value(val)?.into_raw())
            }

            fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self> {
                RkyvValue::from_raw(bin).into_value()
            }
        }
    };
}

impl_store_for_rkyv_value!(bool);
impl_store_for_rkyv_value!(i8);
impl_store_for_rkyv_value!(u8);
impl_store_for_rkyv_value!(i16);
impl_store_for_rkyv_value!(u16);
impl_store_for_rkyv_value!(i32);
impl_store_for_rkyv_value!(u32);
impl_store_for_rkyv_value!(f32);
impl_store_for_rkyv_value!(i64);
impl_store_for_rkyv_value!(u64);
impl_store_for_rkyv_value!(f64);

pub trait GetValueOrDefault<T: StoreValue + Default> {
    fn or_default(self) -> T;
}

impl<T: StoreValue + Default> GetValueOrDefault<T> for anyhow::Result<Option<T>> {
    fn or_default(self) -> T {
        if let Ok(Some(v)) = self {
            v
        } else {
            T::default()
        }
    }
}

pub trait IntoValueOrDefault<T>
where
    T: ISharableValue,
    T::ValueType: Default,
{
    fn deser_or_none(self) -> Option<T::ValueType>;
    fn deser_or_default(self) -> T::ValueType;
}

impl<T> IntoValueOrDefault<T> for anyhow::Result<Option<T>>
where
    T: ISharableValue,
    T::ValueType: Default,
{
    fn deser_or_none(self) -> Option<<T as ISharableValue>::ValueType> {
        self.ok()??.into_value().ok()
    }

    fn deser_or_default(self) -> <T>::ValueType {
        self.deser_or_none().unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct ProtoValue<T: prost::Message + Default>(pub T);

impl<T: prost::Message + Default> StoreValue for ProtoValue<T> {
    #[inline]
    fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>> {
        Ok(val.0.encode_to_vec())
    }
    #[inline]
    fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self(T::decode(bin.as_slice())?))
    }
}

#[derive(Debug)]
pub struct JsonValue<T>
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    data: Vec<u8>,
    _value_type: std::marker::PhantomData<T>,
}

impl<T> ISharableValue for JsonValue<T>
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    type ValueType = T;

    fn from_value(value: T) -> anyhow::Result<Self> {
        Ok(Self {
            data: serde_json::to_vec(&value)?,
            _value_type: std::marker::PhantomData,
        })
    }

    fn from_raw(data: Vec<u8>) -> Self {
        Self {
            data,
            _value_type: std::marker::PhantomData,
        }
    }

    fn into_raw(self) -> Vec<u8> {
        self.data
    }

    fn into_value(self) -> anyhow::Result<T> {
        Ok(serde_json::from_slice(&self.data)?)
    }
}

impl<T> StoreValue for RkyvValue<T>
where
    T: for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                rkyv::rancor::Error,
            >,
        > + rkyv::Archive,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>
        + rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, rkyv::rancor::Error>>,
{
    fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>> {
        Ok(val.into_raw())
    }

    fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self> {
        Ok(RkyvValue::<T>::from_raw(bin))
    }
}

impl<T> StoreValue for JsonValue<T>
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>> {
        Ok(val.into_raw())
    }

    fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self> {
        Ok(JsonValue::<T>::from_raw(bin))
    }
}

impl StoreValue for Vec<u8> {
    fn into_store_data(val: Self) -> anyhow::Result<Vec<u8>> {
        Ok(val)
    }

    fn from_store_data(bin: Vec<u8>) -> anyhow::Result<Self> {
        Ok(bin)
    }
}
