#![feature(is_none_or)]

use std::time::Duration;

use apexsky_proto::pb::{
    apexlegends::{AimTargetItem, EspData, EspDataOption, PlayerState},
    esp_service::esp_service_client::EspServiceClient,
};
use indexmap::IndexMap;
use message::{SonicMessage, VoicePrompt};
use ndarray::arr1;
use resource::ContentId;
use sonic::{sonic_thread, SonicCtrl};
use tokio::{sync::mpsc, time::sleep};

mod message;
mod resource;
mod sonic;

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

        if let Some(true) = state_diff.enemy_in_the_rear {
            sonic_tx
                .send(SonicCtrl::Play(SonicMessage::Voice(VoicePrompt::new(
                    ContentId::EnemyInTheRear,
                    [4.0, 0.0, 0.0],
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

        sleep(Duration::from_secs(2)).await;
        prev_esp_data = Some(esp_data);
        prev_state = Some(state);
    }
}

macro_rules! diff_member {
    ($old:ident, $new:ident, $member:ident) => {
        $old.is_none_or(|$old| $old.$member != $new.$member)
            .then_some($new.$member)
    };
}

struct State {
    ready: bool,
    in_game: bool,
    under_observation: usize,
    enemy_in_the_rear: bool,
    nearby_players: Vec<AimTargetItem>,
    nearby_teams: IndexMap<i32, TeamInfo>,
    radar_points: Vec<([f32; 2], PlayerState)>,
}

struct StateDiff {
    ready: Option<bool>,
    in_game: Option<bool>,
    under_observation: Option<usize>,
    enemy_in_the_rear: Option<bool>,
}

impl StateDiff {
    fn new(new: &State, old: Option<&State>) -> StateDiff {
        StateDiff {
            ready: diff_member!(old, new, ready),
            in_game: diff_member!(old, new, in_game),
            under_observation: diff_member!(old, new, under_observation),
            enemy_in_the_rear: diff_member!(old, new, enemy_in_the_rear),
        }
    }
}

impl State {
    fn analyze(data: &EspData, _prev: Option<&EspData>) -> Self {
        let nearby_players = data
            .targets
            .as_ref()
            .map(|l| &l.elements)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|target| {
                target.info.as_ref().and_then(|info| {
                    (info.distance < 200.0 * 40.0 && !info.is_npc).then_some(target.clone())
                })
            })
            .collect::<Vec<_>>();
        let nearby_teams = TeamInfo::from_targets(&nearby_players);

        let (local_pos, local_yaw) = data
            .view_player
            .as_ref()
            .map(|pl| {
                (
                    pl.origin.clone().unwrap().into(),
                    pl.view_angles.as_ref().map(|v| v.y).unwrap_or(pl.yaw),
                )
            })
            .unwrap_or_default();
        let radar_points: Vec<([f32; 2], PlayerState)> = nearby_players
            .iter()
            .filter_map(|pl| {
                let pl_data = pl.player_data.clone()?;
                let (single, _view_check) = rotate_point(
                    pl_data.origin.clone()?.into(),
                    local_pos,
                    0.0,
                    0.0,
                    250.0,
                    250.0,
                    local_yaw,
                    0.3,
                );
                Some(([single[0], single[1]], pl_data))
            })
            .collect();

        Self {
            ready: data.ready,
            in_game: data.in_game && data.local_player.is_some(),
            under_observation: data
                .spectators
                .as_ref()
                .map(|list| {
                    list.elements
                        .iter()
                        .filter(|spec| !spec.is_teammate)
                        .count()
                })
                .unwrap_or(0),
            enemy_in_the_rear: radar_points
                .iter()
                .find(|(point, _)| point[0] > 50.0 && point[0] < 200.0 && point[1] > 150.0)
                .is_some(),
            nearby_players,
            nearby_teams,
            radar_points,
        }
    }
}

struct TeamInfo {
    team_num: i32,
    members: Vec<PlayerState>,
    distance_to_self: Vec<(f32, [f32; 3])>,
    downed_members: u16,
    max_distance_each_other: f32,
    max_shield_level: i32,
    total_damage: u32,
    total_healthpoints: u32,
    total_kills: u16,
}

impl TeamInfo {
    fn from_targets(targets: &Vec<AimTargetItem>) -> IndexMap<i32, Self> {
        let mut teams: IndexMap<i32, TeamInfo> = IndexMap::new();
        targets
            .iter()
            .filter_map(|target| Some((target.info.as_ref()?, target.player_data.as_ref()?)))
            .for_each(|(pl_info, pl_data)| {
                if let Some(team) = teams.get_mut(&pl_data.team_num) {
                    assert_eq!(team.team_num, pl_data.team_num);
                    let pos = pl_data.origin.clone().unwrap().into();
                    team.members.push(pl_data.clone());
                    team.distance_to_self.push((pl_info.distance, pos));
                    team.downed_members += if pl_info.is_knocked { 1 } else { 0 };
                    team.max_distance_each_other = team.members.iter().rev().skip(1).fold(
                        team.max_distance_each_other,
                        |max_dist, member| {
                            let teammate_pos: [f32; 3] = member.origin.clone().unwrap().into();
                            let distance = (arr1(&teammate_pos) - arr1(&pos))
                                .mapv(|x| x * x)
                                .sum()
                                .sqrt();
                            f32::max(max_dist, distance)
                        },
                    );
                    team.max_shield_level = i32::max(team.max_shield_level, pl_data.armor_type);
                    team.total_damage += u32::try_from(pl_data.damage_dealt).unwrap();
                    team.total_healthpoints += u32::try_from(pl_info.health_points).unwrap();
                    team.total_kills += u16::try_from(pl_data.kills).unwrap();
                } else {
                    teams.insert(
                        pl_data.team_num,
                        TeamInfo {
                            team_num: pl_data.team_num,
                            members: vec![pl_data.clone()],
                            distance_to_self: vec![(
                                pl_info.distance,
                                pl_data.origin.clone().unwrap().into(),
                            )],
                            downed_members: if pl_info.is_knocked { 1 } else { 0 },
                            max_distance_each_other: 0.0,
                            max_shield_level: pl_data.armor_type,
                            total_damage: pl_data.damage_dealt.try_into().unwrap(),
                            total_healthpoints: pl_info.health_points.try_into().unwrap(),
                            total_kills: pl_data.kills.try_into().unwrap(),
                        },
                    );
                }
            });
        teams.values_mut().for_each(|team| {
            team.distance_to_self
                .sort_by(|(a_dist, _a_pos), (b_dist, _b_pos)| a_dist.partial_cmp(b_dist).unwrap());
            team.members
                .sort_by(|a, b| a.team_member_index.cmp(&b.team_member_index));
        });
        teams.sort_by(|_, a, _, b| {
            let min_dist_a = a.distance_to_self.first().unwrap().0;
            let min_dist_b = b.distance_to_self.first().unwrap().0;
            min_dist_a.partial_cmp(&min_dist_b).unwrap()
        });
        teams
    }
}

fn rotate_point(
    entity_pos: [f32; 3],
    local_pos: [f32; 3],
    pos_x: f32,
    pos_y: f32,
    size_x: f32,
    size_y: f32,
    angle: f32,
    zoom: f32,
) -> ([f32; 3], bool) {
    let r_1 = -(entity_pos[1] - local_pos[1]);
    let r_2 = entity_pos[0] - local_pos[0];
    let yaw = angle - 90.0;
    let yaw = yaw.to_radians();
    let mut x_1 = (r_2 * yaw.cos() - r_1 * yaw.sin()) / 20.0;
    let mut y_1 = (r_2 * yaw.sin() + r_1 * yaw.cos()) / 20.0;

    let view_check = y_1 < 0.0;

    x_1 *= zoom;
    y_1 *= zoom;

    let siz_x = size_x / 2.0;
    let siz_y = size_y / 2.0;

    x_1 += siz_x;
    y_1 += siz_y;

    x_1 = x_1.max(5.0).min(size_x - 5.0);
    y_1 = y_1.max(5.0).min(size_y - 5.0);

    x_1 += pos_x;
    y_1 += pos_y;

    ([x_1, y_1, 0.0], view_check)
}
