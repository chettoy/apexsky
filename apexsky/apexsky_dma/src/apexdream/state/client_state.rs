use apexsky::offsets::G_OFFSETS;

use super::*;

#[derive(Default, Debug, Clone)]
pub struct ClientState {
    pub signon_state: i32,
    pub level_hash: u32,
    pub level_name: String,

    pub local_entity: sdk::EHandle,
    pub local_player_ptr: u64,
    pub framecount: i32,
    pub curtime: f32, // Time used for comparison against last_visible_time
    pub interval_per_tick: f32,
    pub view_render: sdk::Ptr,
    pub view_matrix_ptr: sdk::Ptr<[f32; 16]>,
    pub view_matrix: [f32; 16],
}
impl ClientState {
    #[instrument(skip_all)]
    pub async fn update(&mut self, api: &Api, ctx: &mut UpdateContext) {
        let base_addr = api.apex_base;
        let data = &ctx.data;

        // Connection signon state
        if ctx.ticked(25, 24) {
            if let Ok(signon_state) = api.vm_read::<i32>(base_addr.field(data.signon_state)).await {
                ctx.connected = self.signon_state != sdk::SIGNONSTATE_FULL
                    && signon_state == sdk::SIGNONSTATE_FULL;
                //tracing::debug!(signon_state, self.signon_state, ctx.connected);
                self.signon_state = signon_state;
            }
        } else {
            ctx.connected = false;
        }

        let task_current_level_name = async {
            // Current level name
            if ctx.connected {
                self.level_hash = 0;
                self.level_name.clear();

                let mut level_name = [0u8; 0x40];
                if let Ok(level_name) = api
                    .vm_read_cstr(base_addr.field(data.level_name), &mut level_name)
                    .await
                {
                    self.level_hash = crate::apexdream::base::hash(level_name);
                    self.level_name.push_str(level_name);
                }
            }
        };

        let task_local_player_handle = async {
            // Local player entity handle
            if ctx.connected || ctx.ticked(100, 11) {
                let _ = api
                    .vm_read_into(
                        base_addr.field(data.local_entity_handle),
                        &mut self.local_entity,
                    )
                    .await;
                let _ = api
                    .vm_read_into(
                        base_addr.field(G_OFFSETS.local_ent.try_into().unwrap()),
                        &mut self.local_player_ptr,
                    )
                    .await;
            }
            self.local_entity
        };

        let task_globals = async {
            // Globals
            if let Ok(globals) = api
                .vm_read::<sdk::CGlobalVars>(base_addr.field(data.global_vars))
                .await
            {
                self.framecount = globals.framecount;
                self.curtime = globals.curtime;
                self.interval_per_tick = 1.0 / 20.0;
            }
        };

        let task_viewmatrix = async {
            // ViewMatrix
            if ctx.connected || ctx.ticked(25, 6) {
                self.view_render = sdk::Ptr::NULL;
                let _ = api
                    .vm_read_into(base_addr.field(data.view_render), &mut self.view_render)
                    .await;
            }
            if !self.view_render.is_null() && (ctx.connected || ctx.ticked(25, 14)) {
                self.view_matrix_ptr = sdk::Ptr::NULL;
                let _ = api
                    .vm_read_into(
                        self.view_render.field(data.view_matrix),
                        &mut self.view_matrix_ptr,
                    )
                    .await;
            }
            if !self.view_matrix_ptr.is_null() {
                let _ = api
                    .vm_read_into(self.view_matrix_ptr, &mut self.view_matrix)
                    .await;
            }
        };

        {
            let (_, local_entity, _, _) = tokio::join!(
                task_current_level_name,
                task_local_player_handle,
                task_globals,
                task_viewmatrix
            );
            ctx.local_entity = local_entity;
        }
    }
}
impl GameState {
    pub fn is_in_game(&self) -> bool {
        return self.client.signon_state == sdk::SIGNONSTATE_FULL
            && self.client.level_hash != hash!("mp_lobby");
    }
    pub fn is_firing_range(&self) -> bool {
        self.client.level_hash == hash!("mp_rr_canyonlands_staging_mu1")
    }
    pub fn world_to_screen(&self, v: [f32; 3], screen: &[i32; 2], clip: bool) -> Option<[f32; 2]> {
        let vmatrix = &self.client.view_matrix;

        let w = vmatrix[12] * v[0] + vmatrix[13] * v[1] + vmatrix[14] * v[2] + vmatrix[15];
        if w < 0.01 {
            return None;
        }

        let invw = 1.0 / w;
        let vx = (vmatrix[0] * v[0] + vmatrix[1] * v[1] + vmatrix[2] * v[2] + vmatrix[3]) * invw;
        let vy = (vmatrix[4] * v[0] + vmatrix[5] * v[1] + vmatrix[6] * v[2] + vmatrix[7]) * invw;

        // If the resulting coordinate is too far outside the screen bounds clip it manually
        if clip {
            if vx < -2.0 || vx > 2.0 || vy < -2.0 || vy > 2.0 {
                return None;
            }
        }

        let width = screen[0] as f32 * 0.5;
        let height = screen[1] as f32 * 0.5;

        let px = width + vx * width + 0.5;
        let py = height - vy * height + 0.5;
        Some([px, py])
    }
}
