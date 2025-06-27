use tokio::sync::watch;
use tracing::instrument;

use apex1_common::{global::JsonValue, global_state::G_STATE, pb::apexlegends::AimEntityData};
use ohosky_api::common::{rpc::ISharedRpcService, share::ISharableValue};

use crate::G_OFFSETS;
use crate::GameApiHandle;
use crate::game::player::ArcAimEntity;
use crate::obfstr as s;
use crate::skyapi::rpc::SharedRpcService;

#[instrument(skip_all)]
pub async fn remote_loop(
    mut active: watch::Receiver<bool>,
    game_api: GameApiHandle,
) -> anyhow::Result<()> {
    tracing::debug!("{}", s!("task start"));

    type AnyJsonValue = JsonValue<serde_json::Value>;

    fn json_any<T: serde::Serialize>(v: &T) -> anyhow::Result<AnyJsonValue> {
        JsonValue::from_value(serde_json::to_value(v)?)
    }

    let op_config_get_global_settings =
        SharedRpcService::<(), AnyJsonValue>::register(s!("op_config_get_global_settings"));
    let op_config_update_global_settings = SharedRpcService::<
        JsonValue<apex1_common::config::Settings>,
        AnyJsonValue,
    >::register(s!("op_config_update_global_settings"));
    let op_game_get_frame_count =
        SharedRpcService::<(), AnyJsonValue>::register(s!("op_game_frame_count"));
    let op_game_get_fps = SharedRpcService::<(), JsonValue<f32>>::register(s!("op_game_get_fps"));
    let op_game_get_offsets =
        SharedRpcService::<(), AnyJsonValue>::register(s!("op_game_get_offsets"));
    let op_game_is_ready =
        SharedRpcService::<(), JsonValue<bool>>::register(s!("op_game_is_ready"));
    let op_game_is_world_ready =
        SharedRpcService::<(), JsonValue<bool>>::register(s!("op_game_is_world_ready"));
    let op_game_local_player_ptr =
        SharedRpcService::<(), JsonValue<String>>::register(s!("op_game_local_player_ptr"));
    let op_game_view_player_ptr =
        SharedRpcService::<(), JsonValue<String>>::register(s!("op_game_view_player_ptr"));
    let op_game_cached_player =
        SharedRpcService::<JsonValue<String>, AnyJsonValue>::register(s!("op_game_cached_player"));
    let op_game_cached_npc =
        SharedRpcService::<JsonValue<String>, AnyJsonValue>::register(s!("op_game_cached_npc"));
    let op_game_cached_loot =
        SharedRpcService::<JsonValue<String>, AnyJsonValue>::register(s!("op_game_cached_loot"));
    let op_game_cached_aim_entity = SharedRpcService::<JsonValue<String>, AnyJsonValue>::register(
        s!("op_game_cached_aim_entity"),
    );

    while *active.borrow_and_update() {
        tokio::select!(
            input = op_config_get_global_settings.recv() => {
                input?;
                let data = json_any(&G_STATE.lock().unwrap().config.settings)?;
                op_config_get_global_settings.reply(data)
            },
            input = op_config_update_global_settings.recv() => {
                let args = input?.into_value()?;
                G_STATE.lock().unwrap().config.settings = args;
                op_config_update_global_settings.reply(json_any(&true)?)
            },
            input = op_game_get_frame_count.recv() => {
                input?;
                op_game_get_frame_count.reply(json_any(&game_api.state.get_frame_count())?)
            },
            input = op_game_get_fps.recv() => {
                input?;
                op_game_get_fps.reply(JsonValue::from_value(game_api.state.get_game_fps())?)
            },
            input = op_game_get_offsets.recv() => {
                input?;
                op_game_get_offsets.reply(json_any(&*G_OFFSETS)?)
            },
            input = op_game_is_ready.recv() => {
                input?;
                op_game_is_ready.reply(JsonValue::from_value(
                    game_api.state.get_game_baseaddr().is_some(),
                )?)
            },
            input = op_game_is_world_ready.recv() => {
                input?;
                op_game_is_world_ready
                    .reply(JsonValue::from_value(game_api.state.is_world_ready())?)
            },
            input = op_game_local_player_ptr.recv() => {
                input?;
                op_game_local_player_ptr.reply(JsonValue::from_value(
                    game_api
                        .state
                        .get_local_player_ptr()
                        .unwrap_or(0)
                        .to_string(),
                )?)
            },
            input = op_game_view_player_ptr.recv() => {
                input?;
                op_game_view_player_ptr.reply(JsonValue::from_value(
                    game_api
                        .state
                        .get_view_player_ptr()
                        .unwrap_or(0)
                        .to_string(),
                )?)
            },
            input = op_game_cached_player.recv() => {
                let args = input?.into_value()?;
                let ptr = args.parse::<u64>()?;
                let ret = if let Some(val) = game_api.state.read_cached_player(&ptr) {
                    let mut val = serde_json::to_value(val.get_buf())?;
                    fix_big_number(&mut val);
                    val
                } else {
                    serde_json::Value::Null
                };
                op_game_cached_player.reply(JsonValue::from_value(ret)?)
            },
            input = op_game_cached_npc.recv() => {
                let args = input?.into_value()?;
                let ptr = args.parse::<u64>()?;
                let ret = if let Some(val) = game_api.state.read_cached_npc(&ptr) {
                    let mut val = serde_json::to_value(AimEntityData::from(ArcAimEntity(val)))?;
                    fix_big_number(&mut val);
                    val
                } else {
                    serde_json::Value::Null
                };
                op_game_cached_npc.reply(JsonValue::from_value(ret)?)
            },
            input = op_game_cached_loot.recv() => {
                let args = input?.into_value()?;
                let ptr = args.parse::<u64>()?;
                op_game_cached_loot.reply(json_any(&game_api.state.read_cached_loot(&ptr))?)
            },
            input = op_game_cached_aim_entity.recv() => {
                let args = input?.into_value()?;
                let ptr = args.parse::<u64>()?;
                let ret = game_api.state.read_cached_aim_entity(&ptr).map(|ent| {
                    serde_json::to_value(AimEntityData::from(ArcAimEntity(ent))).unwrap()
                });
                op_game_cached_aim_entity.reply(json_any(&ret)?)
            },
        )
        .inspect_err(|e| tracing::error!(?e))?;
    }
    tracing::debug!("{}", s!("task end"));
    Ok(())
}

/// Convert all large numbers in an object that cannot be represented by a js Number to string.
fn fix_big_number(value: &mut serde_json::Value) {
    if let Some(obj) = value.as_object_mut() {
        for v in obj.values_mut() {
            if let Some(num) = v.as_u64()
                && num > (2 << 52) - 1
            {
                *v = serde_json::Value::String(num.to_string());
            }
        }
    }
}
