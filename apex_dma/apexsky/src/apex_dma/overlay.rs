#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;

use eframe::egui::{self, pos2, Color32, Pos2, Rect, ViewportCommand};
use obfstr::obfstr as s;
use tokio::sync::Mutex;

use crate::SharedState;

pub(crate) fn main(shared_state: Arc<Mutex<SharedState>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_active(true)
            .with_always_on_top()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_fullscreen(true)
            .with_mouse_passthrough(true)
            .with_transparent(true) // To have rounded corners we need transparency
            .with_window_level(egui::WindowLevel::AlwaysOnTop),

        ..Default::default()
    };
    eframe::run_native(
        s!("Absolutely Not Cheating.exe - Totally Legit Gameplay 😇"), // unused title
        options,
        Box::new(|_cc| Box::new(OverlayApp { shared_state })),
    )
}

struct OverlayApp {
    shared_state: Arc<Mutex<SharedState>>,
}

impl eframe::App for OverlayApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "egui with custom frame", |ui| {
            ui.label("This is just the contents of the window.");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: Color32::TRANSPARENT, //ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        //let app_rect = ui.max_rect();

        // let title_bar_height = 32.0;
        // let title_bar_rect = {
        //     let mut rect = app_rect;
        //     rect.max.y = rect.min.y + title_bar_height;
        //     rect
        // };
        // title_bar_ui(ui, title_bar_rect, title);

        add_contents(ui);

        info_bar_ui(ui);

        // Add the contents:
        // let content_rect = {
        //     let mut rect = app_rect;
        //     rect.min.y = title_bar_rect.max.y;
        //     rect
        // }
        // .shrink(4.0);
        // let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        // add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}

fn info_bar_ui(ui: &mut egui::Ui) {
    // Draw rectangle
    let info_bar_rect = Rect::from_min_max(Pos2::ZERO, pos2(280.0, 30.0));
    ui.allocate_ui_at_rect(info_bar_rect, |ui| {
        let background_color = Color32::from_black_alpha((0.6 * 255.0 as f32).round() as u8);
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
            let aimbot_mode = 2;
            let aimbot_fov = 50.0;
            let aimbot_locked = false;
            let aimbot_gun_safety = false;
            let aimbot_grenade = false;
            let spectators = 0;
            let allied_spectators = 0;

            ui.add_space(rounding * 2.0);
            ui.colored_label(
                if spectators == 0 {
                    Color32::GREEN
                } else {
                    Color32::RED
                },
                format!("{}", spectators),
            );
            ui.label(s!("--"));
            ui.colored_label(Color32::GREEN, format!("{}", allied_spectators));
            ui.label(s!("--"));
            ui.colored_label(Color32::WHITE, format!("{:.0}", aimbot_fov));
            ui.label(s!("--"));

            let (color, text) = if aimbot_locked {
                (
                    if aimbot_gun_safety {
                        Color32::GREEN
                    } else {
                        Color32::from_rgb(255, 165, 0) // Orange
                    },
                    s!("[TARGET LOCK!]").to_string(),
                )
            } else if aimbot_grenade {
                (Color32::BLUE, s!("Skynade On").to_string())
            } else if aimbot_mode == 2 {
                (Color32::GREEN, s!("Aim On").to_string())
            } else if aimbot_mode == 0 {
                (Color32::RED, s!("Aim Off").to_string())
            } else {
                (Color32::RED, format!("{}{}", s!("Aim On "), aimbot_mode))
            };
            ui.colored_label(color, text);
            ui.add_space(rounding * 2.0);
        });
    });
}
