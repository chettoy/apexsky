use fyrox_sound::algebra::{UnitQuaternion, Vector3};
use instant::Instant;
use ndarray::arr1;

pub use self::message::{SonicMessage, VoicePrompt};
pub use self::resource::ContentId;
use self::state::{State, StateDiff};
use crate::overlay::utils::game_coords_to_engine_coords;
use crate::pb::apexlegends::EspData;

pub(crate) mod message;
mod resource;
mod state;

#[derive(Debug)]
pub struct VoiceNavigator {
    dry_run: bool,
    last_process: Instant,
    prev_esp_data: Option<EspData>,
    prev_state: Option<State>,
}

impl Default for VoiceNavigator {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceNavigator {
    pub fn new() -> Self {
        Self {
            dry_run: false,
            last_process: Instant::now(),
            prev_esp_data: None,
            prev_state: None,
        }
    }

    pub fn voice_resource() -> Vec<(&'static str, Vec<u8>)> {
        resource::ASSETS.to_vec()
    }

    pub fn tick(&mut self, esp_data: &EspData) -> Vec<SonicMessage> {
        let mut buf = vec![];

        if self.prev_state.is_some() && self.last_process.elapsed().as_millis() < 2000 {
            return buf;
        }

        let esp_data = if self.dry_run {
            let mut test_data = EspData::default();
            test_data.ready = true;
            test_data
        } else {
            esp_data.clone()
        };

        let state = State::analyze(&esp_data, self.prev_esp_data.as_ref());
        let state_diff = StateDiff::new(&state, self.prev_state.as_ref());

        fn finish_tick(this: &mut VoiceNavigator, esp_data: EspData, state: State) {
            this.prev_esp_data = Some(esp_data);
            this.prev_state = Some(state);
            this.last_process = Instant::now();
        }

        if !state.ready {
            finish_tick(self, esp_data, state);
            return buf;
        }

        if let Some(true) = state_diff.ready {
            buf.push(SonicMessage::Voice(VoicePrompt::new(
                ContentId::Connected,
                [-4.0, 1.0, 2.0],
            )));
        }

        if !state.in_game {
            finish_tick(self, esp_data, state);
            return buf;
        }

        if let Some(count) = state_diff.under_observation {
            if count > 0 {
                buf.push(SonicMessage::Voice(VoicePrompt::new(
                    ContentId::UnderObservation,
                    [-4.0, 1.0, 2.0],
                )));
            }
        }

        if state.on_the_ground && !state.skydiving {
            if let Some(pos) = state_diff
                .team_in_the_rear
                .and_then(|_| state.team_in_the_rear)
                .map(|team_id| state.nearby_teams.get(&team_id))
                .flatten()
                .map(|team| team.distance_to_self.first())
                .flatten()
                .map(|(_dist, pos)| (arr1(pos) - arr1(&state.local_pos)) / 40.0 / 20.0)
                .map(|rel| [rel[0], rel[1], rel[2]])
                .map(game_coords_to_engine_coords)
                .map(|engine_pos| {
                    let axis = Vector3::y_axis();
                    let rotation_matrix =
                        UnitQuaternion::from_axis_angle(&axis, -state.local_yaw.to_radians())
                            .to_homogeneous();
                    let pos: Vector3<f32> =
                        rotation_matrix.transform_point(&engine_pos.into()).coords;
                    [pos[0], pos[1], pos[2]]
                })
            {
                buf.push(SonicMessage::Voice(VoicePrompt::new(
                    ContentId::EnemyInTheRear,
                    pos,
                )));
            }

            if state.nearby_teams.len() == 2
                && self
                    .prev_state
                    .as_ref()
                    .map(|state| state.nearby_teams.len())
                    .unwrap_or(0)
                    < 2
            {
                buf.push(SonicMessage::Voice(VoicePrompt::new(
                    ContentId::DualTeamsNearby,
                    [-4.0, 1.0, 2.0],
                )));
            }
        }

        finish_tick(self, esp_data, state);
        buf
    }
}
