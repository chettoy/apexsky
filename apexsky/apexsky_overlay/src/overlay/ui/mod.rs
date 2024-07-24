use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::egui::pos2;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};
use instant::Instant;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use crate::overlay::ui::mini_map::mini_map_radar;
use crate::overlay::ui::mini_map::RadarTarget;
use crate::overlay::utils::get_unix_timestamp_in_millis;
use crate::pb::apexlegends::EspData;
use crate::pb::apexlegends::EspSettings;
use crate::pb::apexlegends::EspVisualsFlag;
use crate::pb::apexlegends::Loots;
use crate::pb::apexlegends::LoveStatusCode;
use crate::pb::apexlegends::PlayerState;
use crate::pb::apexlegends::SpectatorInfo;

use super::asset::Blob;
use super::system::game_esp::EspServiceAddr;
use super::system::game_esp::EspSystem;
use super::MyOverlayState;

mod mini_map;

static ID_HELLO_WINDOW: Lazy<egui::Id> = Lazy::new(|| egui::Id::new(s!("#hello_window")));
static ID_RADAR_WINDOW: Lazy<egui::Id> = Lazy::new(|| egui::Id::new(s!("#radar_window")));

// A simple system to handle some keyboard input and toggle on/off the hittest.
pub fn toggle_mouse_passthrough(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    #[cfg(feature = "native")]
    {
        let mut window = windows.single_mut();
        window.cursor.hit_test = keyboard_input.pressed(KeyCode::Insert);
    }
}

#[derive(Debug, Resource, Serialize, Deserialize, Default)]
pub(crate) struct UiPersistance {
    hello_position: Option<(f32, f32)>,
    radar_position: Option<(f32, f32)>,
    #[serde(skip)]
    change_time: Option<Instant>,
}

impl UiPersistance {
    #[cfg(feature = "native")]
    fn file_path() -> anyhow::Result<std::path::PathBuf> {
        let current_dir = std::env::current_dir()?;
        Ok(current_dir.join(s!("./overlay-ui.json")))
    }

    #[cfg(feature = "native")]
    pub(crate) fn load_persistance() -> anyhow::Result<Self> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(Self::file_path()?)?;
        let reader = BufReader::new(file);
        let saved: Self = serde_json::from_reader(reader)?;
        Ok(Self {
            hello_position: saved.hello_position,
            radar_position: saved.radar_position,
            change_time: None,
        })
    }
    pub(crate) fn update(&mut self, mem: &egui::Memory) {
        if let Some(rect_hello) = mem.area_rect(*ID_HELLO_WINDOW) {
            let pos = rect_hello.left_top();
            if self
                .hello_position
                .is_none_or(|(x, y)| x != pos.x || y != pos.y)
            {
                self.hello_position = Some((pos.x, pos.y));
                self.change_time = Some(Instant::now());
            }
        }
        if let Some(rect_radar) = mem.area_rect(*ID_RADAR_WINDOW) {
            let pos = rect_radar.left_top();
            if self
                .radar_position
                .is_none_or(|(x, y)| x != pos.x || y != pos.y)
            {
                self.radar_position = Some((pos.x, pos.y));
                self.change_time = Some(Instant::now());
            }
        }
        if let Some(change_time) = self.change_time {
            if change_time.elapsed().as_millis() > 500 {
                if let Err(e) = self.persistance() {
                    tracing::error!(%e, ?e);
                }
                self.change_time = None;
            }
        }
    }
    #[cfg(feature = "native")]
    fn persistance(&self) -> anyhow::Result<()> {
        use std::fs;
        use std::io::Write;

        let mut persistance_write = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(Self::file_path()?)?;
        let data = serde_json::to_string(self)?;
        write!(persistance_write, "{}", data)?;
        Ok(())
    }
    #[cfg(not(feature = "native"))]
    fn persistance(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Resource, Debug, Default)]
pub(crate) struct UiState {
    input_esp_addr: String,
    input_addr_valid: Option<EspServiceAddr>,
}

struct DialogEsp {
    overlay_fps: String,
    game_fps: String,
    latency: String,
    loop_duration: String,
    target_count: String,
    local_position: String,
    local_angles: String,
    local_yaw: String,
    local_held: String,
    aim_position: String,
    spectator_list: Vec<SpectatorInfo>,
    allied_spectator_name: Vec<String>,
    teammates_info: Vec<PlayerState>,
}

#[tracing::instrument(skip_all)]
pub fn ui_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut overlay_state: ResMut<MyOverlayState>,
    mut ui_persistance: ResMut<UiPersistance>,
    mut ui_state: ResMut<UiState>,
    mut esp_system: Option<ResMut<EspSystem>>,
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    blobs: Res<Assets<Blob>>,
) {
    use egui::{CentralPanel, Color32, ScrollArea};
    let ctx = contexts.ctx_mut();

    // Set default font
    if !overlay_state.font_loaded {
        if let Some(font_blob) = blobs.get(&overlay_state.font_blob) {
            let mut egui_fonts = egui::FontDefinitions::default();
            egui_fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_owned(font_blob.bytes.to_owned()),
            );
            egui_fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            egui_fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("my_font".to_owned());
            ctx.set_fonts(egui_fonts);

            overlay_state.font_loaded = true;
        }
    }

    let (allied_spectators, spectators): (Vec<_>, Vec<_>) = esp_system
        .as_ref()
        .and_then(|v| {
            v.get_esp_data()
                .spectators
                .clone()
                .map(|list| list.elements)
        })
        .unwrap_or(vec![])
        .into_iter()
        .partition(|info| info.is_teammate);
    let (spectators_count, allied_spectators_count) = (spectators.len(), allied_spectators.len());
    let allied_spectator_name = allied_spectators
        .into_iter()
        .map(|info| info.name)
        .collect();

    let dialog_esp = {
        let esp_system = esp_system.as_ref();
        DialogEsp {
            overlay_fps: {
                // try to get a "smoothed" FPS value from Bevy
                if let Some(value) = diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                {
                    format!("{:.1}", value)
                } else {
                    s!("N/A").to_string()
                }
            },
            game_fps: format!(
                "{:.1}",
                esp_system
                    .map(|v| v.get_esp_data().game_fps)
                    .unwrap_or_default()
            ),
            latency: format!(
                "{}{}{:.0}{}{:.0}{}",
                esp_system
                    .and_then(|v| v.last_data_traffic_time)
                    .map(|t| t.as_millis() as i32)
                    .unwrap_or(-1),
                s!("ms(net), "),
                overlay_state.data_latency,
                s!("ms(data), "),
                time.delta_seconds() * 1000.0,
                s!("ms(ui)"),
            ),
            loop_duration: esp_system
                .map(|v| {
                    let data = v.get_esp_data();
                    format!(
                        "{: >4}{}{: >4}{}",
                        data.duration_tick,
                        s!("ms(tick) +"),
                        data.duration_actions,
                        s!("ms(actions)"),
                    )
                })
                .unwrap_or_default(),
            target_count: esp_system
                .map(|v| v.get_target_count())
                .unwrap_or_default()
                .to_string(),
            local_position: esp_system
                .and_then(|v| v.get_view_player())
                .and_then(|pl| pl.origin.clone())
                .map(|pos| {
                    format!(
                        "{}{:.0}{}{:.0}{}{:.0}",
                        s!("x="),
                        pos.x,
                        s!(", y="),
                        pos.y,
                        s!(", z="),
                        pos.z
                    )
                })
                .unwrap_or_default(),
            local_angles: esp_system
                .and_then(|v| v.get_view_player())
                .and_then(|pl| pl.view_angles.clone())
                .map(|angle| {
                    format!(
                        "{}{:.2}{}{:.2}{}{:.2}",
                        s!("view angles= "),
                        angle.x,
                        s!(", "),
                        angle.y,
                        s!(", "),
                        angle.z
                    )
                })
                .unwrap_or_default(),
            local_yaw: esp_system
                .and_then(|v| v.get_view_player())
                .map(|pl| pl.yaw)
                .map(|yaw| format!("{}{:.2}", s!("yaw="), yaw))
                .unwrap_or_default(),
            local_held: esp_system
                .and_then(|v| v.get_esp_data().aimbot.as_ref())
                .map(|aimbot| {
                    format!(
                        "{}{}{}{}",
                        s!("held="),
                        aimbot.held_id,
                        s!(", weapon="),
                        aimbot.weapon_id
                    )
                })
                .unwrap_or_default(),
            aim_position: esp_system
                .and_then(|v| v.get_esp_data().aimbot.as_ref())
                .map(|aimbot| aimbot.target_position.as_ref())
                .flatten()
                .map(|aim_target| {
                    format!(
                        "{}{:.2}{}{:.2}{}{:.2}{}",
                        s!("aim["),
                        aim_target.x,
                        s!(","),
                        aim_target.y,
                        s!(","),
                        aim_target.z,
                        s!("]")
                    )
                })
                .unwrap_or_default(),
            spectator_list: spectators,
            allied_spectator_name,
            teammates_info: esp_system
                .and_then(|v| v.get_esp_data().teammates.clone())
                .map(|dt| dt.players)
                .unwrap_or_default(),
        }
    };

    let _show = egui::Window::new(s!("Hello, world!"))
        .id(*ID_HELLO_WINDOW)
        .auto_sized()
        .default_pos(ui_persistance.hello_position.unwrap_or((1600.0, 320.0)))
        .frame(egui::Frame {
            inner_margin: egui::Margin::same(8.0),
            outer_margin: egui::Margin::ZERO,
            rounding: egui::Rounding::same(6.0),
            shadow: egui::epaint::Shadow {
                offset: [0.0, 0.0].into(),
                spread: 3.0,
                color: Color32::from_black_alpha(61),
                blur: 0.0,
            },
            fill: Color32::from_rgba_premultiplied(13, 13, 13, 138),
            stroke: egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(48, 48, 48, 74)),
        })
        .show(ctx, |ui| {
            {
                let visuals = ui.visuals_mut();
                visuals.override_text_color =
                    Some(Color32::from_rgba_premultiplied(255, 255, 255, 222));
                visuals.window_fill = Color32::from_rgba_premultiplied(26, 26, 26, 153);
                visuals.window_rounding = egui::Rounding::same(7.0);
            }

            ui.label(format!(
                "{}{}{}{}{}",
                s!("Overlay("),
                dialog_esp.overlay_fps,
                s!(" FPS) Game("),
                dialog_esp.game_fps,
                s!(" FPS)")
            ));
            ui.label(format!("{}{}", s!("latency "), dialog_esp.latency));
            ui.label(format!("{}{}", s!("duration "), dialog_esp.loop_duration));
            ui.label(format!("{}{}", s!("target "), dialog_esp.target_count));
            ui.add_space(5.0);
            ui.label(dialog_esp.local_position);
            ui.label(dialog_esp.local_angles);
            ui.label(dialog_esp.local_yaw);
            ui.label(dialog_esp.local_held);
            ui.label(dialog_esp.aim_position);

            ui.add_space(10.0);

            ScrollArea::vertical()
                .id_source(s!("scroll-teammates"))
                .max_width(320.0)
                .max_height(480.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(s!("Teammates"));
                    });

                    if dialog_esp.teammates_info.is_empty() {
                        ui.label(s!("no teammates"));
                    }

                    let view_teammate_index =
                        esp_system.as_ref().and_then(|v| v.get_view_teammate());
                    for (teammate_index, teammate) in dialog_esp.teammates_info.iter().enumerate() {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            let name_text = {
                                let name = teammate.player_name.to_owned();
                                if dialog_esp.allied_spectator_name.contains(&name) {
                                    egui::RichText::new(name).strong().color(Color32::GREEN)
                                } else {
                                    egui::RichText::new(name).strong()
                                }
                            };
                            ui.label(format!("{} - ", teammate.team_member_index));

                            if Some(teammate_index) == view_teammate_index {
                                ui.label(name_text);
                                if ui.add(egui::Button::new(s!("👓"))).clicked() {
                                    if let Some(v) = esp_system.as_mut() {
                                        v.set_view_teammate(None);
                                    }
                                }
                            } else {
                                if ui.add(egui::Button::new(name_text)).clicked() {
                                    if let Some(v) = esp_system.as_mut() {
                                        v.set_view_teammate(Some(teammate_index));
                                    }
                                }
                            }

                            ui.add_space(5.0);
                            ui.label(teammate.damage_dealt.to_string());
                            ui.add_space(5.0);
                            ui.label(teammate.kills.to_string());
                        });
                    }
                });

            ui.add_space(10.0);

            ScrollArea::vertical()
                .id_source(s!("scroll-spectators"))
                .max_width(320.0)
                .max_height(480.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(s!("Spectators"));
                    });

                    if dialog_esp.spectator_list.is_empty() {
                        ui.label(s!("no spectators"));
                    }

                    for spectator_info in dialog_esp.spectator_list.iter() {
                        let name = &spectator_info.name;
                        match LoveStatusCode::try_from(spectator_info.love_status)
                            .unwrap_or(LoveStatusCode::Normal)
                        {
                            LoveStatusCode::Normal => ui.label(name),
                            LoveStatusCode::Love => ui.label(
                                egui::RichText::new(name)
                                    .strong()
                                    .color(Color32::from_rgb(231, 27, 100)),
                            ),
                            LoveStatusCode::Hate => {
                                ui.label(egui::RichText::new(name).strong().color(Color32::RED))
                            }
                            LoveStatusCode::Ambivalent => {
                                ui.label(egui::RichText::new(name).strong().color(Color32::BLACK))
                            }
                        };
                    }
                });

            ui.add_space(10.0);

            // buttons
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new(s!("Connection")).color((|| {
                            // gRPC not connected: red
                            let Some(esp_system) = esp_system.as_ref() else {
                                return Color32::LIGHT_RED;
                            };
                            // ESP service no response: yellow
                            let Some(response_time) = esp_system.last_data_response_time else {
                                return Color32::YELLOW;
                            };
                            // ESP service timeout: yellow
                            if response_time.elapsed().as_millis() > 200 {
                                return Color32::LIGHT_YELLOW;
                            }
                            // Not attached to game: blue
                            if !esp_system.get_esp_data().ready {
                                return Color32::LIGHT_BLUE;
                            }
                            // Not in game: green
                            if !esp_system.get_esp_data().in_game {
                                return Color32::LIGHT_GREEN;
                            }
                            // in game: green blink
                            if response_time.elapsed().as_secs() % 2 == 0 {
                                Color32::LIGHT_GREEN
                            } else {
                                Color32::GREEN
                            }
                        })()),
                    ))
                    .clicked()
                {
                    // For audio in browser
                    overlay_state.user_gesture = true;

                    // Toggle address TextEdit
                    if ui_state.input_esp_addr.is_empty() {
                        // set value and show
                        ui_state.input_esp_addr = overlay_state
                            .override_esp_addr
                            .as_ref()
                            .map(|addr| addr.get_addr())
                            .or(esp_system.as_ref().map(|v| v.get_endpoint()))
                            .unwrap_or(EspServiceAddr::default().get_addr())
                            .to_owned();
                        ui_state.input_addr_valid =
                            EspServiceAddr::from_str(&ui_state.input_esp_addr);
                    } else {
                        // validate input and override address
                        if let Some(valid_input) = &ui_state.input_addr_valid {
                            if esp_system
                                .as_ref()
                                .is_none_or(|v| v.get_endpoint() != valid_input.get_addr())
                            {
                                overlay_state.override_esp_addr = ui_state.input_addr_valid.clone();
                            }
                        }
                        // clear and dismiss
                        ui_state.input_esp_addr.clear();
                    }
                }
                if ui
                    .add(egui::Button::new(if !overlay_state.user_gesture {
                        egui::RichText::new(s!("Click me"))
                    } else {
                        egui::RichText::new(s!(" Ready  ")).color(Color32::LIGHT_GREEN)
                    }))
                    .clicked()
                {
                    overlay_state.user_gesture = true;
                }
                if ui.add(egui::Button::new(s!("Test sound"))).clicked() {
                    overlay_state.test_sound = true;
                }
                if ui.add(egui::Button::new(s!("Toggle bg"))).clicked() {
                    overlay_state.user_gesture = true;

                    overlay_state.black_background = !overlay_state.black_background;
                    let bg_color = if overlay_state.black_background {
                        Color::BLACK
                    } else {
                        Color::NONE
                    };
                    commands.insert_resource(ClearColor(bg_color));
                }
            });

            // address bar
            if !ui_state.input_esp_addr.is_empty() {
                let invalid = ui_state.input_addr_valid.is_none();
                let response = ui.add(
                    egui::TextEdit::singleline(&mut ui_state.input_esp_addr)
                        .text_color_opt(invalid.then_some(Color32::RED)),
                );
                // Validate input
                if response.changed() {
                    ui_state.input_addr_valid = EspServiceAddr::from_str(&ui_state.input_esp_addr);
                }
                // Submit input and dismiss
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if ui_state.input_addr_valid.is_some() {
                        overlay_state.override_esp_addr = ui_state.input_addr_valid.clone();
                        ui_state.input_esp_addr.clear();
                    }
                }
                // Quick input button
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    if ui.add(egui::Button::new(s!("⬆ 127.0.0.1"))).clicked() {
                        ui_state.input_esp_addr = s!("http://127.0.0.1:50051").to_string();
                        ui_state.input_addr_valid =
                            EspServiceAddr::from_str(&ui_state.input_esp_addr);
                    }
                    if ui.add(egui::Button::new(s!("⬆ 192.168.122.1"))).clicked() {
                        ui_state.input_esp_addr = s!("http://192.168.122.1:50051").to_string();
                        ui_state.input_addr_valid =
                            EspServiceAddr::from_str(&ui_state.input_esp_addr);
                    }
                });
            }
        });

    if let Some(ref esp_system_connected) = esp_system {
        let esp_settings = esp_system_connected.get_esp_settings();
        let esp_data = esp_system_connected.get_esp_data();
        let esp_loots = esp_system_connected.get_esp_loots();
        let view_player = esp_system_connected.get_view_player();

        let panel_frame = egui::Frame {
            fill: Color32::TRANSPARENT, //ctx.style().visuals.window_fill(),
            rounding: 10.0.into(),
            stroke: egui::Stroke::NONE, //ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(),   // so the stroke is within the bounds
            ..Default::default()
        };

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            esp_2d_ui(ui, esp_data, esp_settings, esp_loots, view_player);
            info_bar_ui(ui, esp_data, spectators_count, allied_spectators_count);
        });

        // Radar Stuff
        if esp_data.in_game && esp_settings.mini_map_radar {
            if let Some((base_pos, base_yaw)) = view_player.map(|pl| {
                (
                    pl.origin.clone().unwrap().into(),
                    pl.view_angles.as_ref().map(|v| v.y).unwrap_or(pl.yaw),
                )
            }) {
                let radar_targets = esp_data
                    .targets
                    .as_ref()
                    .map(|list| &list.elements)
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|item| Some((item.info.as_ref()?, item.player_data.as_ref()?)))
                    .map(|(player_info, player_buf)| RadarTarget {
                        pos: player_buf.origin.clone().unwrap().into(),
                        yaw: player_buf
                            .view_angles
                            .as_ref()
                            .map(|view_angles| {
                                let yaw = view_angles.y;
                                if yaw < 0.0 {
                                    yaw + 360.0
                                } else {
                                    yaw
                                }
                            })
                            .unwrap_or(player_buf.yaw),
                        distance: player_info.distance / 39.62,
                        team_id: player_buf.team_num,
                    })
                    .collect();
                let default_position = ui_persistance.radar_position.unwrap_or((45.0, 45.0));
                mini_map_radar(
                    ctx,
                    base_pos,
                    base_yaw,
                    radar_targets,
                    default_position,
                    5.,
                    1.,
                );
            }
        }
    }

    let now = get_unix_timestamp_in_millis() as f64;
    overlay_state.data_latency = now
        - esp_system
            .map(|v| v.get_esp_data().data_timestamp)
            .unwrap_or(0.0)
            * 1000.0;

    ctx.memory(|mem| ui_persistance.update(mem));
}

fn info_bar_ui(
    ui: &mut egui::Ui,
    esp_data: &EspData,
    spectators_count: usize,
    allied_spectators_count: usize,
) {
    use egui::{Color32, Pos2, Rect, RichText};

    #[derive(Debug, Clone)]
    struct EspInfo {
        aimbot_fov: f32,
        aimbot_status_text: String,
        aimbot_status_color: Color32,
    }

    let info = {
        if let Some(aimbot) = esp_data.aimbot.as_ref() {
            let aimbot_mode = aimbot.aim_mode;
            let (aimbot_status_color, aimbot_status_text) = if aimbot.target_locked {
                (
                    if aimbot.gun_safety {
                        Color32::GREEN
                    } else {
                        Color32::from_rgb(255, 165, 0) // Orange
                    },
                    s!("[TARGET LOCK!]").to_string(),
                )
            } else if aimbot.held_grenade {
                (Color32::BLUE, s!("Skynade On").to_string())
            } else if aimbot_mode & 0x4 != 0 {
                (Color32::GREEN, s!("Aim Assist").to_string())
            } else if aimbot_mode & 0x2 != 0 {
                (
                    Color32::GREEN,
                    format!("{}{}", s!("Aim On "), aimbot.loop_duration),
                )
            } else if aimbot_mode == 0 {
                (Color32::RED, s!("Aim Off").to_string())
            } else {
                (Color32::RED, format!("{}{}", s!("Aim On "), aimbot_mode))
            };
            EspInfo {
                aimbot_fov: aimbot.max_fov,
                aimbot_status_text,
                aimbot_status_color,
            }
        } else {
            EspInfo {
                aimbot_fov: 0.0,
                aimbot_status_text: s!("[Aimbot Offline]").to_string(),
                aimbot_status_color: Color32::GRAY,
            }
        }
    };

    // Draw rectangle
    let info_bar_rect = Rect::from_min_max(Pos2::ZERO, pos2(280.0, 30.0));
    ui.allocate_ui_at_rect(info_bar_rect, |ui| {
        let background_color = Color32::from_black_alpha((0.4 * 255.0 as f32).round() as u8);
        let rounding = 2.0;
        ui.painter()
            .rect_filled(info_bar_rect, rounding, background_color);

        // Draw red lines
        let line_color = Color32::from_rgb(255, 0, 0);
        let line_width = 2.0;
        ui.painter().line_segment(
            [
                pos2(info_bar_rect.min.x + rounding, info_bar_rect.min.y),
                pos2(info_bar_rect.max.x - rounding, info_bar_rect.min.y),
            ],
            (line_width, line_color),
        );
        ui.painter().line_segment(
            [
                pos2(
                    info_bar_rect.min.x + rounding,
                    info_bar_rect.max.y - line_width,
                ),
                pos2(
                    info_bar_rect.max.x - rounding,
                    info_bar_rect.max.y - line_width,
                ),
            ],
            (line_width, line_color),
        );

        // Draw text
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            let text_size = 16.0;
            ui.add_space(rounding * 2.0);
            ui.label(
                RichText::new(format!("{}", spectators_count))
                    .color(if spectators_count == 0 {
                        Color32::GREEN
                    } else {
                        Color32::RED
                    })
                    .size(text_size),
            );
            ui.label(RichText::new(s!("--")).size(text_size));
            ui.label(
                RichText::new(format!("{}", allied_spectators_count))
                    .color(Color32::GREEN)
                    .size(text_size),
            );
            ui.label(RichText::new(s!("--")).size(text_size));
            ui.label(
                RichText::new(format!("{:.1}", info.aimbot_fov))
                    .color(Color32::WHITE)
                    .size(text_size),
            );
            ui.label(RichText::new(s!("--")).size(text_size));
            ui.label(
                RichText::new(info.aimbot_status_text)
                    .color(info.aimbot_status_color)
                    .size(text_size),
            );
            ui.add_space(rounding * 2.0);
        });
    });
}

fn esp_2d_ui(
    ui: &mut egui::Ui,
    esp_data: &EspData,
    esp_settings: &EspSettings,
    esp_loots: &Loots,
    view_player: Option<&PlayerState>,
) {
    use egui::{Color32, Rect};

    if !esp_data.in_game {
        return;
    }

    let screen_width = esp_settings.screen_width as f32;
    let screen_height = esp_settings.screen_height as f32;

    let view_matrix = esp_data
        .view_matrix
        .clone()
        .unwrap()
        .elements
        .try_into()
        .unwrap();

    let font_id = egui::FontId {
        size: 16.0,
        family: egui::FontFamily::Proportional,
    };

    if esp_settings.show_aim_target {
        if let Some(aim_pos) = (|| {
            let pos: [f32; 3] = esp_data.aimbot.as_ref()?.target_position.clone()?.into();
            if !(pos[0] == 0.0 && pos[1] == 0.0 && pos[2] == 0.0) {
                Some(pos)
            } else {
                None
            }
        })() {
            let bs = world_to_screen(aim_pos, &view_matrix, screen_width, screen_height).unwrap_or(
                Vec2 {
                    x: screen_width / 2.0,
                    y: screen_height / 2.0,
                },
            );

            const INDICATOR_RADIUS: f32 = 10.0;

            let aimbot_target_locked = esp_data
                .aimbot
                .as_ref()
                .map(|a| a.target_locked)
                .unwrap_or(false);
            let indicator_color = if aimbot_target_locked {
                Color32::from_rgba_unmultiplied(255, 165, 0, 158)
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, 158)
            };
            let p1 = pos2(bs.x + INDICATOR_RADIUS, bs.y - INDICATOR_RADIUS);
            let p2 = pos2(bs.x - INDICATOR_RADIUS, bs.y - INDICATOR_RADIUS);
            let p3 = pos2(bs.x - INDICATOR_RADIUS, bs.y + INDICATOR_RADIUS);
            let p4 = pos2(bs.x + INDICATOR_RADIUS, bs.y + INDICATOR_RADIUS);
            ui.painter().rect_stroke(
                Rect { min: p2, max: p4 },
                INDICATOR_RADIUS,
                (1.6726, indicator_color),
            );
            if aimbot_target_locked {
                let stroke = (2.718, Color32::RED);
                ui.painter().line_segment([p1, p3], stroke);
                ui.painter().line_segment([p2, p4], stroke);
            }
        }
    }

    if !esp_loots.loots.is_empty() {
        if let Some(bs_local) = world_to_screen(
            view_player
                .and_then(|p| p.origin.clone())
                .unwrap_or_default()
                .into(),
            &view_matrix,
            screen_width,
            screen_height,
        ) {
            for clue in &esp_loots.loots {
                let Some(position) = clue.position.clone() else {
                    continue;
                };
                let Some(bs_loot) =
                    world_to_screen(position.into(), &view_matrix, screen_width, screen_height)
                else {
                    continue;
                };
                let (scr_pos_local, scr_pos_loot) =
                    (pos2(bs_local.x, bs_local.y), pos2(bs_loot.x, bs_loot.y));
                let distance_text = format!(
                    "{}{}{}{}",
                    clue.item_id,
                    s!("("),
                    (clue.distance / 39.62).round() as i32,
                    s!("m)")
                );
                ui.painter().line_segment(
                    [scr_pos_local, scr_pos_loot],
                    (0.5, Color32::from_rgba_unmultiplied(255, 255, 255, 32)),
                );
                ui.painter().text(
                    scr_pos_loot,
                    Align2::CENTER_CENTER,
                    distance_text,
                    font_id.clone(),
                    Color32::from_rgb(212, 175, 55),
                );
            }
        }
    }

    let (box_color_viz, box_color_not_viz) = {
        let color_viz: [f32; 3] = esp_settings.glow_color_viz.clone().unwrap().into();
        let color_notviz: [f32; 3] = esp_settings.glow_color_notviz.clone().unwrap().into();
        let convert = |v: f32| -> u8 { (v.max(0.0).min(1.0) * 255.0).round() as u8 };
        (
            (
                convert(color_viz[0]),
                convert(color_viz[1]),
                convert(color_viz[2]),
            ),
            (
                convert(color_notviz[0]),
                convert(color_notviz[1]),
                convert(color_notviz[2]),
            ),
        )
    };

    esp_data
        .targets
        .as_ref()
        .map(|list| &list.elements)
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|item| Some((item.info.as_ref()?, item.player_data.as_ref()?)))
        .for_each(|(player_info, player_data)| {
            let head_position = player_data.head_position.clone().unwrap();
            let bottom_position = player_data.origin.clone().unwrap();
            let Some(head_screen_pos) = world_to_screen(
                head_position.into(),
                &view_matrix,
                screen_width,
                screen_height,
            ) else {
                return;
            };
            let Some(bottom_screen_pos) = world_to_screen(
                bottom_position.into(),
                &view_matrix,
                screen_width,
                screen_height,
            ) else {
                return;
            };
            let box_height = (head_screen_pos.y - bottom_screen_pos.y).abs();
            let box_width = box_height / 2.0;
            let box_middle_x = bottom_screen_pos.x - box_width / 2.0;

            let aim_distance = esp_settings.aim_distance;

            if esp_settings.esp_visuals & EspVisualsFlag::Damage as i32 != 0
                && player_info.distance < aim_distance
            {
                let color = Color32::from_rgb(188, 18, 20);
                let draw_pos = pos2(box_middle_x, head_screen_pos.y - 32.0);
                let damage_text = player_data.damage_dealt.to_string();
                ui.painter().text(
                    draw_pos,
                    Align2::CENTER_CENTER,
                    damage_text,
                    font_id.clone(),
                    color,
                );
            }

            let alpha = if player_info.distance < aim_distance {
                1.0
            } else if player_info.distance > 16000.0 {
                0.4
            } else {
                1.0 - (player_info.distance - aim_distance) / (16000.0 - aim_distance) * 0.6
            };
            let alpha = (255.0 * alpha.max(0.0).min(1.0)).round() as u8;
            let radar_distance = (player_info.distance / 39.62).round() as i32;

            if esp_settings.esp_visuals & EspVisualsFlag::Distance as i32 != 0 {
                let distance_text = format!(
                    "{}{}{}{}",
                    radar_distance,
                    s!("m("),
                    player_data.team_num,
                    s!(")")
                );
                let color = if player_data.is_knocked {
                    Color32::RED
                } else {
                    Color32::from_rgba_unmultiplied(0, 255, 0, alpha)
                };
                let pos = pos2(box_middle_x, bottom_screen_pos.y + 1.0);
                ui.painter().text(
                    pos,
                    Align2::CENTER_CENTER,
                    distance_text,
                    font_id.clone(),
                    color,
                );
            }

            if player_info.distance < aim_distance {
                if esp_settings.esp_visuals & EspVisualsFlag::Box as i32 != 0 {
                    let (r, g, b) = if player_info.is_visible {
                        box_color_viz
                    } else {
                        box_color_not_viz
                    };
                    let box_color = Color32::from_rgba_unmultiplied(r, g, b, alpha);
                    let min_pos = pos2(box_middle_x - box_width / 2.0, head_screen_pos.y);
                    let max_pos = pos2(min_pos.x + box_width, min_pos.y + box_height);
                    let stroke = (1.0, box_color);
                    ui.painter().rect_stroke(
                        Rect {
                            min: min_pos,
                            max: max_pos,
                        },
                        0.0,
                        stroke,
                    );
                }
                if esp_settings.esp_visuals & EspVisualsFlag::Name as i32 != 0 {
                    let is_love: LoveStatusCode = player_data
                        .love_status
                        .try_into()
                        .unwrap_or(LoveStatusCode::Normal);

                    let name_color = if is_love == LoveStatusCode::Love {
                        Color32::from_rgb(231, 27, 100)
                    } else if is_love == LoveStatusCode::Hate {
                        Color32::RED
                    } else if is_love == LoveStatusCode::Ambivalent {
                        Color32::BLACK
                    } else {
                        Color32::from_rgba_unmultiplied(255, 255, 255, alpha)
                    };

                    let draw_pos = pos2(box_middle_x, head_screen_pos.y - 15.0);
                    let nick_pos = pos2(draw_pos.x + 50.0, draw_pos.y);

                    let level_text = format!("{}{}", s!("Lv."), xp_level(player_data.xp));
                    let level_color = Color32::from_rgba_unmultiplied(0, 255, 0, alpha);

                    ui.painter().text(
                        draw_pos,
                        Align2::RIGHT_CENTER,
                        level_text,
                        font_id.clone(),
                        level_color,
                    );
                    ui.painter().text(
                        nick_pos,
                        Align2::CENTER_CENTER,
                        player_data.player_name.to_owned(),
                        font_id.clone(),
                        name_color,
                    );
                }
            }
        })
}

pub fn world_to_screen(
    from: [f32; 3],
    view_matrix: &[f32; 16],
    screen_width: f32,
    screen_height: f32,
) -> Option<Vec2> {
    let from = Vec3::new(from[0], from[1], from[2]);

    let w = view_matrix[12] * from.x
        + view_matrix[13] * from.y
        + view_matrix[14] * from.z
        + view_matrix[15];

    if w < 0.01 {
        return None;
    }

    let mut to = Vec2::new(0.0, 0.0);
    to.x = view_matrix[0] * from.x
        + view_matrix[1] * from.y
        + view_matrix[2] * from.z
        + view_matrix[3];
    to.y = view_matrix[4] * from.x
        + view_matrix[5] * from.y
        + view_matrix[6] * from.z
        + view_matrix[7];

    let invw = 1.0 / w;
    to.x *= invw;
    to.y *= invw;

    let mut x = screen_width / 2.0;
    let mut y = screen_height / 2.0;

    x += 0.5 * to.x * screen_width + 0.5;
    y -= 0.5 * to.y * screen_height + 0.5;

    to.x = x;
    to.y = y;

    Some(to)
}

fn xp_level(xp: i32) -> i32 {
    /* MIT License
     * Copyright (c) 2023 Xnieno */
    const LEVELS: [i32; 56] = [
        100, 2750, 6650, 11400, 17000, 23350, 30450, 38300, 46450, 55050, 64100, 73600, 83550,
        93950, 104800, 116100, 127850, 140050, 152400, 164900, 177550, 190350, 203300, 216400,
        229650, 243050, 256600, 270300, 284150, 298150, 312300, 326600, 341050, 355650, 370400,
        385300, 400350, 415550, 430900, 446400, 462050, 477850, 493800, 509900, 526150, 542550,
        559100, 575800, 592650, 609650, 626800, 644100, 661550, 679150, 696900, 714800,
    ];

    if xp < 0 {
        return 0;
    }

    let array_size = LEVELS.len();

    for i in 0..array_size {
        if xp < LEVELS[i] {
            return i as i32 + 1;
        }
    }

    1 + array_size as i32 + ((xp - LEVELS[array_size - 1]) / 18000)
}
// // info ui
// commands
//     .spawn(NodeBundle {
//         style: Style {
//             position_type: PositionType::Absolute,
//             left: Val::Px(0.0),
//             top: Val::Px(0.0),
//             width: Val::Px(280.0),
//             height: Val::Px(30.0),
//             ..Default::default()
//         },
//         background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.6)),
//         ..Default::default()
//     })
//     .with_children(|parent| {
//         let font_handle = asset_server.load(s!("fonts/LXGWNeoXiHei.ttf").to_string());
//         parent.spawn(
//             TextBundle::from_sections(vec![
//                 TextSection {
//                     value: "Hello, ".to_string(),
//                     style: TextStyle {
//                         font: font_handle.clone(),
//                         font_size: 16.0,
//                         color: Color::rgb(1.0, 0.0, 0.0),
//                     },
//                 },
//                 TextSection {
//                     value: "Bevy".to_string(),
//                     style: TextStyle {
//                         font: font_handle.clone(),
//                         font_size: 16.0,
//                         color: Color::rgb(0.0, 1.0, 0.0),
//                     },
//                 },
//                 TextSection {
//                     value: "!".to_string(),
//                     style: TextStyle {
//                         font: font_handle.clone(),
//                         font_size: 16.0,
//                         color: Color::rgb(0.0, 0.0, 1.0),
//                     },
//                 },
//             ])
//             .with_style(Style {
//                 align_self: AlignSelf::Center,
//                 margin: UiRect {
//                     left: Val::Px(4.),
//                     right: Val::Px(4.),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }),
//         );
//     });
