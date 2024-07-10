use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use async_trait::async_trait;
use deno_core::error::AnyError;
use deno_core::*;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait GameApi {
    // config
    fn config_get_global_settings(&self) -> crate::config::Settings;
    fn config_update_global_settings(&self, new_val: crate::config::Settings);

    // game gernal
    fn game_is_ready(&self) -> bool;
    fn game_frame_count(&self) -> u32;
    fn game_get_fps(&self) -> f32;
    fn game_get_offsets(&self) -> skyapex_sdk::module::CustomOffsets;

    // memory
    fn mem_game_baseaddr(&self) -> Option<u64>;
    async fn mem_read_all(
        &self,
        list: Vec<OpMemReadItem>,
    ) -> anyhow::Result<Vec<serde_json::Value>>;
    async fn mem_read_i32(&self, addr: u64) -> anyhow::Result<i32>;
    async fn mem_write_i32(&self, addr: u64, value: i32) -> anyhow::Result<()>;
    async fn mem_read_f32(&self, addr: u64) -> anyhow::Result<f32>;
    async fn mem_write_f32(&self, addr: u64, value: f32) -> anyhow::Result<()>;

    // game world
    fn game_is_world_ready(&self) -> bool;
    fn game_local_player_ptr(&self) -> Option<u64>;
    fn game_view_player_ptr(&self) -> Option<u64>;
}

pub type GameApiInstance = Arc<dyn GameApi>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "addr")]
pub enum OpMemReadItem {
    #[serde(rename = "f32")]
    ReadF32(u64),
    #[serde(rename = "i32")]
    ReadI32(u64),
}

extension!(
  apexsky_game_api,
  deps = [apexsky_extension],
  ops = [
    op_config_get_global_settings,
    op_config_update_global_settings,
    op_game_frame_count,
    op_game_get_fps,
    op_game_get_offsets,
    op_game_is_ready,
    op_mem_game_baseaddr,
    op_mem_read_all,
    op_mem_read_f32,
    op_mem_read_i32,
    op_mem_write_f32,
    op_mem_write_i32,
    op_game_local_player_ptr,
    op_game_view_player_ptr,
    op_game_is_world_ready,
  ],
  options = {
    game_api: GameApiInstance,
  },
  state = |state, options| {
    state.put::<GameApiInstance>(options.game_api);
  },
  docs = "apexsky game api",
);

#[op2]
#[serde]
fn op_config_get_global_settings(state: &OpState) -> Result<crate::config::Settings, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.config_get_global_settings())
}

#[op2]
fn op_config_update_global_settings(
    #[serde] new_val: crate::config::Settings,
    state: &OpState,
) -> Result<(), AnyError> {
    let api = state.borrow::<GameApiInstance>();
    api.config_update_global_settings(new_val);
    Ok(())
}

#[op2(fast)]
fn op_game_frame_count(state: &OpState) -> Result<u32, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_frame_count())
}

#[op2(fast)]
fn op_game_get_fps(state: &OpState) -> Result<f32, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_get_fps())
}

#[op2]
#[serde]
fn op_game_get_offsets(state: &OpState) -> Result<skyapex_sdk::module::CustomOffsets, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_get_offsets())
}

#[op2(fast)]
fn op_game_is_ready(state: &OpState) -> Result<bool, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_is_ready())
}

#[op2(fast)]
#[bigint]
fn op_mem_game_baseaddr(state: &OpState) -> Result<u64, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.mem_game_baseaddr().unwrap_or(0))
}

#[op2(async)]
#[serde]
async fn op_mem_read_all(
    #[serde] list: Vec<OpMemReadItem>,
    state: Rc<RefCell<OpState>>,
) -> Result<Vec<serde_json::Value>, AnyError> {
    let api = Arc::clone(state.borrow().borrow::<GameApiInstance>());
    api.mem_read_all(list).await.map_err(|e| {
        //tracing::error!(%e);
        e
    })
}

#[op2(async)]
async fn op_mem_read_f32(
    #[bigint] addr: u64,
    state: Rc<RefCell<OpState>>,
) -> Result<f32, AnyError> {
    let api = Arc::clone(state.borrow().borrow::<GameApiInstance>());
    api.mem_read_f32(addr).await.map_err(|e| {
        tracing::error!(%e);
        e
    })
}

#[op2(async)]
async fn op_mem_read_i32(
    #[bigint] addr: u64,
    state: Rc<RefCell<OpState>>,
) -> Result<i32, AnyError> {
    let api = Arc::clone(state.borrow().borrow::<GameApiInstance>());
    api.mem_read_i32(addr).await.map_err(|e| {
        tracing::error!(%e);
        e
    })
}

#[op2(async)]
async fn op_mem_write_f32(
    #[bigint] addr: u64,
    value: f32,
    state: Rc<RefCell<OpState>>,
) -> Result<(), AnyError> {
    let api = Arc::clone(state.borrow().borrow::<GameApiInstance>());
    api.mem_write_f32(addr, value).await.map_err(|e| {
        tracing::error!(%e);
        e
    })
}

#[op2(async)]
async fn op_mem_write_i32(
    #[bigint] addr: u64,
    value: i32,
    state: Rc<RefCell<OpState>>,
) -> Result<(), AnyError> {
    let api = Arc::clone(state.borrow().borrow::<GameApiInstance>());
    api.mem_write_i32(addr, value).await.map_err(|e| {
        tracing::error!(%e);
        e
    })
}

#[op2(fast)]
#[bigint]
fn op_game_local_player_ptr(state: &OpState) -> Result<u64, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_local_player_ptr().unwrap_or(0))
}

#[op2(fast)]
#[bigint]
fn op_game_view_player_ptr(state: &OpState) -> Result<u64, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_view_player_ptr().unwrap_or(0))
}

#[op2(fast)]
fn op_game_is_world_ready(state: &OpState) -> Result<bool, AnyError> {
    let api = state.borrow::<GameApiInstance>();
    Ok(api.game_is_world_ready())
}
