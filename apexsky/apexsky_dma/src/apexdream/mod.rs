use self::{
    data::GameData,
    state::{GameState, UpdateContext},
};

/**
 * https://github.com/CasualX/apexdream
 * LISENCE: GPLv3
 */
pub mod api;
#[macro_use]
pub mod base;
pub mod data;
pub mod sdk;
pub mod state;

/// Copyright (C) 2019 - 2023 Casper <CasualX@users.noreply.github.com>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub struct Instance {
    state: GameState,
    update_ctx: UpdateContext,
}

impl Instance {
    pub fn new() -> Self {
        let ctx = UpdateContext {
            data: GameData::default(),
            time: Default::default(),
            connected: false,
            tickcount: 0,
            local_entity: Default::default(),
            full_bones: false,
        };
        Self {
            state: GameState::default(),
            update_ctx: ctx,
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub async fn tick_state(&mut self, api: &mut self::state::Api) {
        let time = apexsky::aimbot::get_unix_timestamp_in_millis() as f64 / 1000.0;
        self.update_ctx.time = time;
        self.state.time = time;

        self.state.update(api, &mut self.update_ctx).await;

        self.update_ctx.tickcount = self.update_ctx.tickcount.wrapping_add(1);
    }

    pub fn is_newly_connected(&self) -> bool {
        self.update_ctx.connected
    }
}
