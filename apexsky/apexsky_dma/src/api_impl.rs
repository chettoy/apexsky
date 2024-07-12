use apexsky::{global_state::G_STATE, offsets::G_OFFSETS};
use apexsky_dmalib::access::{AccessType, MemApi, PendingAccessRequest, PendingMemRead};
use apexsky_extension::{GameApi, OpMemReadItem};
use async_trait::async_trait;

use crate::{SharedStateWrapper, TaskChannels};

#[derive(Debug, Clone)]
pub struct GameApiHandle {
    pub(crate) state: SharedStateWrapper,
    pub(crate) channels: TaskChannels,
    pub(crate) access_tx: MemApi,
}

#[async_trait]
impl GameApi for GameApiHandle {
    fn mem_game_baseaddr(&self) -> Option<u64> {
        self.state.get_game_baseaddr()
    }

    async fn mem_read_all(
        &self,
        list: Vec<OpMemReadItem>,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut result_list = Vec::with_capacity(list.len());
        let mut futs_list = Vec::with_capacity(list.len());
        for item in list.clone() {
            let mem = self.access_tx.clone();
            let addr = match item {
                OpMemReadItem::ReadF32(addr) => addr,
                OpMemReadItem::ReadI32(addr) => addr,
            };
            futs_list.push(tokio::spawn(async move {
                AccessType::mem_read(addr, size_of::<u32>(), 0)
                    .dispatch(&mem)
                    .await?
                    .recv_for::<u32>()
                    .await
            }));
        }
        for (i, fut) in futs_list.into_iter().enumerate() {
            let v = fut.await??;
            result_list.push(match list[i] {
                OpMemReadItem::ReadF32(_) => f32::from_bits(v).into(),
                OpMemReadItem::ReadI32(_) => (v as i32).into(),
            });
        }
        Ok(result_list)
    }

    async fn mem_read_i32(&self, addr: u64) -> anyhow::Result<i32> {
        AccessType::mem_read(addr, size_of::<i32>(), 0)
            .with_priority(1)
            .dispatch(&self.access_tx)
            .await?
            .recv_for::<i32>()
            .await
    }

    async fn mem_write_i32(&self, addr: u64, value: i32) -> anyhow::Result<()> {
        AccessType::mem_write_typed::<i32>(addr, &value, 0)
            .with_priority(1)
            .dispatch(&self.access_tx)
            .await?
            .await?
    }

    async fn mem_read_f32(&self, addr: u64) -> anyhow::Result<f32> {
        AccessType::mem_read(addr, size_of::<f32>(), 0)
            .with_priority(1)
            .dispatch(&self.access_tx)
            .await?
            .recv_for::<f32>()
            .await
    }

    async fn mem_write_f32(&self, addr: u64, value: f32) -> anyhow::Result<()> {
        AccessType::mem_write_typed::<f32>(addr, &value, 0)
            .with_priority(1)
            .dispatch(&self.access_tx)
            .await?
            .await?
    }

    fn game_is_world_ready(&self) -> bool {
        self.state.is_world_ready()
    }

    fn game_local_player_ptr(&self) -> Option<u64> {
        self.state.get_local_player_ptr()
    }

    fn game_view_player_ptr(&self) -> Option<u64> {
        self.state.get_view_player_ptr()
    }

    fn game_cached_player(&self, ptr: u64) -> Option<serde_json::Value> {
        self.state
            .read_cached_player(&ptr)
            .map(|pl| serde_json::to_value(pl.get_buf()).unwrap())
    }

    fn game_is_ready(&self) -> bool {
        self.mem_game_baseaddr().is_some()
    }

    fn game_frame_count(&self) -> u32 {
        self.state.get_frame_count()
    }

    fn game_get_fps(&self) -> f32 {
        self.state.get_game_fps()
    }

    fn game_get_offsets(&self) -> serde_json::Value {
        serde_json::to_value(&*G_OFFSETS).unwrap()
    }

    fn config_get_global_settings(&self) -> serde_json::Value {
        let val = G_STATE.lock().unwrap().config.settings.clone();
        serde_json::to_value(val).unwrap()
    }

    fn config_update_global_settings(&self, new_val: serde_json::Value) -> anyhow::Result<()> {
        let new_val = serde_json::from_value(new_val)?;
        G_STATE.lock().unwrap().config.settings = new_val;
        Ok(())
    }
}
