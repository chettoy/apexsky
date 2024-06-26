#![feature(is_none_or)]

use std::time::Duration;

use apexsky_proto::pb::{
    apexlegends::{EspData, EspDataOption},
    esp_service::esp_service_client::EspServiceClient,
};
use message::{SonicMessage, VoicePrompt};
use ndarray::arr1;
use resource::ContentId;
use sonic::{sonic_thread, SonicCtrl};
use state::{State, StateDiff};
use tokio::{sync::mpsc, time::sleep};

mod message;
mod resource;
mod sonic;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let esp_server_addr = "http://[::1]:50051";
    let dry_run = false;

    let mut esp_client = if dry_run {
        None
    } else {
        Some(
            EspServiceClient::connect(esp_server_addr)
                .await?
                .accept_compressed(tonic::codec::CompressionEncoding::Zstd)
                .send_compressed(tonic::codec::CompressionEncoding::Zstd),
        )
    };

    let (sonic_tx, sonic_rx) = mpsc::channel(100);
    std::thread::spawn(|| sonic_thread(sonic_rx));

    let mut prev_esp_data: Option<EspData> = None;
    let mut prev_state: Option<State> = None;

    loop {
        let esp_data = if let Some(esp_client) = esp_client.as_mut() {
            esp_client
                .get_esp_data(EspDataOption {
                    version: 0,
                    full_aimbot_state: false,
                    full_targets_list: false,
                })
                .await?
                .into_inner()
        } else {
            let mut test_data = EspData::default();
            test_data.ready = true;
            test_data
        };

        let state = State::analyze(&esp_data, prev_esp_data.as_ref());
        let state_diff = StateDiff::new(&state, prev_state.as_ref());

        if !state.ready {
            println!("ESP service not ready!");
            sleep(Duration::from_secs(4)).await;
            prev_esp_data = Some(esp_data);
            prev_state = Some(state);
            continue;
        }

        if let Some(true) = state_diff.ready {
            for (name, data) in &*resource::ASSETS {
                sonic_tx
                    .send(SonicCtrl::LoadAudio((name.to_string(), data.clone())))
                    .await?;
            }

            sonic_tx
                .send(SonicCtrl::Play(SonicMessage::Voice(VoicePrompt::new(
                    ContentId::Connected,
                    [4.0, 0.0, 0.0],
                ))))
                .await?;
        }

        if !state.in_game {
            println!("Waiting for the game to be ready..");
            sleep(Duration::from_secs(4)).await;
            prev_esp_data = Some(esp_data);
            prev_state = Some(state);
            continue;
        }

        if let Some(count) = state_diff.under_observation {
            if count > 0 {
                sonic_tx
                    .send(SonicCtrl::Play(SonicMessage::Voice(VoicePrompt::new(
                        ContentId::UnderObservation,
                        [4.0, 0.0, 0.0],
                    ))))
                    .await?;
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
                .map(|game_pos| [game_pos[1], game_pos[2], -game_pos[0]])
            {
                sonic_tx
                    .send(SonicCtrl::Play(SonicMessage::Voice(VoicePrompt::new(
                        ContentId::EnemyInTheRear,
                        pos,
                    ))))
                    .await?;
            }

            if state.nearby_teams.len() == 2
                && prev_state
                    .map(|state| state.nearby_teams.len())
                    .unwrap_or(0)
                    < 2
            {
                sonic_tx
                    .send(SonicCtrl::Play(SonicMessage::Voice(VoicePrompt::new(
                        ContentId::DualTeamsNearby,
                        [4.0, 0.0, 0.0],
                    ))))
                    .await?;
            }
        }

        sleep(Duration::from_secs(2)).await;
        prev_esp_data = Some(esp_data);
        prev_state = Some(state);
    }
}
