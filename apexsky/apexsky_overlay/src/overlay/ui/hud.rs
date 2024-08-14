use bevy_egui::egui;
use egui::{pos2, Color32, Rect, RichText, TextureId};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use crate::{
    overlay::{ui::world_to_screen, utils::game_coords_to_engine_coords},
    pb::apexlegends::EspData,
};

pub static HUD: OnceCell<Mutex<Hud>> = OnceCell::new();

#[derive(Debug)]
pub struct Hud {
    hud_texture: TextureId,
    screen_width: f32,
    screen_height: f32,
    hud_size: f32,
    data: HudData,
}

#[derive(Debug, Default)]
struct HudData {
    show: bool,
    movement_offset: [f32; 2],
    target_locking: bool,
    target_locked: bool,
    aim_pos: Option<bevy::prelude::Vec2>,
}

impl Hud {
    pub fn new(hud_texture: TextureId) -> Self {
        Self {
            hud_texture,
            screen_width: 1920.0,
            screen_height: 1080.0,
            hud_size: 900.0,
            data: Default::default(),
        }
    }

    pub fn set_data(&mut self, ctx: &egui::Context, data: &EspData) {
        let egui::Vec2 {
            x: screen_width,
            y: screen_height,
        } = ctx.screen_rect().size();
        let size_scale = {
            let screen_size_default = (1920.0_f32.powi(2) + 1080.0_f32.powi(2)).sqrt();
            let screen_size = (screen_width.powi(2) + screen_height.powi(2)).sqrt();
            screen_size / screen_size_default
        };
        self.screen_width = screen_width;
        self.screen_height = screen_height;
        self.hud_size = 900. * size_scale;

        let Some(aimbot) = data.aimbot.as_ref() else {
            self.data = HudData {
                show: false,
                ..Default::default()
            };
            return;
        };
        let Some(local_player) = data.local_player.as_ref() else {
            self.data = HudData {
                show: false,
                ..Default::default()
            };
            return;
        };

        let hud_data = HudData {
            show: [0, 1].contains(&aimbot.held_id),
            movement_offset: {
                use fyrox_sound::algebra::{UnitQuaternion, Vector3};
                let viewangle = local_player.view_angles.unwrap();
                let accel = local_player.accel.unwrap();
                let accel = game_coords_to_engine_coords(accel.into());
                let axis = Vector3::y_axis();
                let rotation_matrix =
                    UnitQuaternion::from_axis_angle(&axis, -viewangle.y.to_radians())
                        .to_homogeneous();
                let accel_rel =
                    rotation_matrix.transform_vector(&Vector3::new(accel[0], accel[1], accel[2]));
                let accel_2d = [accel_rel.x, -accel_rel.y];
                let offset = [
                    (-accel_2d[0] * 0.005).clamp(-10.0, 10.0),
                    (-accel_2d[1] * 0.005).clamp(-5.0, 5.0),
                ];
                let prev = self.data.movement_offset;
                [
                    prev[0] + (offset[0] - prev[0]) * 0.3,
                    prev[1] + (offset[1] - prev[1]) * 0.3,
                ]
            },
            target_locking: aimbot.target_locked,
            target_locked: aimbot
                .aim_result
                .is_some_and(|aim_result| aim_result.hitscan),
            aim_pos: (|| {
                world_to_screen(
                    aimbot.aim_result?.hitscan_nearest_pos?.into(),
                    &data.view_matrix.clone()?.elements.try_into().unwrap(),
                    screen_width,
                    screen_height,
                )
            })(),
        };
        self.data = hud_data;
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        if !self.data.show {
            return;
        }

        let Self {
            hud_texture,
            screen_width,
            screen_height,
            hud_size,
            ..
        } = *self;
        let data = &self.data;

        let (hud_color, hud_text) = if data.target_locked {
            (
                Color32::from_rgba_unmultiplied(255, 55, 55, 128),
                Some("LOCKED"),
            )
        } else if data.target_locking {
            // (
            //     Color32::from_rgba_unmultiplied(255, 255, 55, 128),
            //     Some("LOCKING"),
            // )
            (Color32::from_rgba_unmultiplied(255, 55, 55, 128), None)
        } else {
            (Color32::from_rgba_unmultiplied(167, 233, 255, 128), None)
        };

        if let Some(aim_pos) = data.aim_pos {
            let aim_pos = (aim_pos.x, aim_pos.y).into();
            ui.painter()
                .circle_stroke(aim_pos, 15.0, (1.6726, hud_color));
            ui.painter().line_segment(
                [aim_pos, (screen_width * 0.8, screen_height).into()],
                (1.0, hud_color),
            );
        }

        let rect = Rect {
            min: pos2(
                screen_width / 2. - hud_size / 2.,
                screen_height / 2. - hud_size / 2.,
            ),
            max: pos2(
                screen_width / 2. + hud_size / 2.,
                screen_height / 2. + hud_size / 2.,
            ),
        }
        .translate(data.movement_offset.into());

        egui::Image::new(egui::load::SizedTexture::new(
            hud_texture,
            egui::vec2(hud_size, hud_size),
        ))
        .tint(hud_color)
        .paint_at(ui, rect);

        if let Some(hud_text) = hud_text {
            let em = hud_size / 100.;
            let font_id = egui::FontId {
                size: 2.5 * em,
                family: egui::FontFamily::Monospace,
            };
            let stroke = (2.718, hud_color);
            let text_rect_left = Rect {
                min: pos2(rect.left() + 15. * em, rect.top() + 36. * em),
                max: pos2(rect.left() + 20. * em, rect.bottom() - 36. * em),
            };
            let text_rect_right = Rect {
                min: pos2(rect.right() - 15. * em, rect.top() + 36. * em),
                max: pos2(rect.right() - 20. * em, rect.bottom() - 36. * em),
            };
            ui.painter().line_segment(
                [text_rect_left.left_top(), text_rect_left.left_bottom()],
                stroke,
            );
            ui.painter().line_segment(
                [text_rect_left.right_top(), text_rect_left.right_bottom()],
                stroke,
            );
            ui.painter().line_segment(
                [text_rect_right.left_top(), text_rect_right.left_bottom()],
                stroke,
            );
            ui.painter().line_segment(
                [text_rect_right.right_top(), text_rect_right.right_bottom()],
                stroke,
            );

            ui.put(
                text_rect_left.shrink2(egui::vec2(2. * em, 0.)),
                egui::Label::new(
                    RichText::new(hud_text)
                        .font(font_id.clone())
                        .color(hud_color),
                ),
            );
            ui.put(
                text_rect_right.shrink2(egui::vec2(2. * em, 0.)),
                egui::Label::new(
                    RichText::new(hud_text)
                        .font(font_id.clone())
                        .color(hud_color),
                ),
            );
        }
    }
}
