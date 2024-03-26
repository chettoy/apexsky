use apexsky::noobfstr as s;
use bevy::math::Vec3;
use bevy_egui::egui;
use egui::{pos2, Color32};
use once_cell::sync::Lazy;

struct RadarSettings {
    radar: bool,
    radar_guides: bool,
    team_radar: bool,
    enemy_radar: bool,
    x_axis_radar: i32,
    y_axis_radar: i32,
    radar_type: i32,
    width_radar: i32,
    height_radar: i32,
    distance_radar: i32,
    distance_radar2: i32,
}

impl Default for RadarSettings {
    fn default() -> Self {
        Self {
            radar: true,
            radar_guides: true,
            team_radar: true,
            enemy_radar: true,
            x_axis_radar: 0,
            y_axis_radar: 400,
            radar_type: 0,
            width_radar: 400,
            height_radar: 400,
            distance_radar: 250,
            distance_radar2: 1000,
        }
    }
}

static RADAR_SETTINGS: Lazy<RadarSettings> = Lazy::new(RadarSettings::default);

pub struct RadarTarget {
    pub(super) pos: [f32; 3],
    pub(super) yaw: f32,
    pub(super) distance: f32,
    pub(super) team_id: i32,
}

pub(super) fn mini_map_radar(
    ctx: &mut egui::Context,
    local_pos: [f32; 3],
    local_yaw: f32,
    enemy_data: Vec<RadarTarget>,
    mini_map_radar_dot_size1: f32,
    mini_map_radar_dot_size2: f32,
) {
    if !RADAR_SETTINGS.radar {
        return;
    }

    egui::Window::new(s!("Radar"))
        .resizable(false)
        .title_bar(true)
        .movable(true)
        .frame(egui::Frame::none())
        .default_pos((45.0, 45.0))
        .fixed_size((250.0, 250.0))
        .show(ctx, |ui| {
            let draw_rect = ui.clip_rect();
            let mid_radar = pos2(
                draw_rect.min.x + draw_rect.width() / 2.0,
                draw_rect.min.y + draw_rect.height() / 2.0,
            );
            if RADAR_SETTINGS.radar_guides {
                let stroke = (1.0, Color32::WHITE);
                ui.painter().line_segment(
                    [
                        pos2(draw_rect.min.x, mid_radar.y),
                        pos2(draw_rect.max.x, mid_radar.y),
                    ],
                    stroke,
                );
                ui.painter().line_segment(
                    [
                        pos2(mid_radar.x, draw_rect.min.y),
                        pos2(mid_radar.x, draw_rect.max.y),
                    ],
                    stroke,
                );
            }
            enemy_data.iter().for_each(|target| {
                draw_radar_point_mini_map(
                    ui,
                    local_pos,
                    local_yaw,
                    target.pos,
                    target.yaw,
                    target.distance,
                    target.team_id,
                    draw_rect,
                    mini_map_radar_dot_size1,
                    mini_map_radar_dot_size2,
                );
            });
        });
}

fn draw_radar_point_mini_map(
    ui: &mut egui::Ui,
    local_pos: [f32; 3],
    local_yaw: f32,
    target_pos: [f32; 3],
    target_yaw: f32,
    target_distance: f32,
    target_team_index: i32,
    draw_rect: egui::Rect,
    mini_map_radar_dot_size1: f32,
    mini_map_radar_dot_size2: f32,
) {
    let (single, _view_check) = rotate_point(
        target_pos,
        local_pos,
        draw_rect.min.x,
        draw_rect.min.y,
        draw_rect.width(),
        draw_rect.height(),
        local_yaw,
        0.3,
    );
    if target_distance > 0.0 && target_distance < RADAR_SETTINGS.distance_radar as f32 {
        draw_radar_dot(
            ui,
            single[0],
            single[1],
            mini_map_radar_dot_size1,
            mini_map_radar_dot_size2,
            target_team_index,
            target_yaw,
        );
    }
}

fn draw_radar_dot(
    ui: &mut egui::Ui,
    x: f32,
    y: f32,
    radius: f32,
    stroke_thickness: f32,
    team_id: i32,
    target_yaw: f32,
) {
    static TEAM_COLORS: [[u8; 3]; 21] = [
        [248, 104, 104],
        [242, 86, 38],
        [97, 92, 81],
        [174, 247, 89],
        [102, 214, 173],
        [98, 244, 234],
        [92, 208, 250],
        [93, 137, 238],
        [164, 105, 252],
        [243, 98, 161],
        [214, 67, 67],
        [230, 116, 51],
        [185, 179, 167],
        [148, 200, 65],
        [86, 174, 91],
        [55, 188, 200],
        [84, 169, 212],
        [98, 121, 203],
        [102, 61, 174],
        [218, 73, 145],
        [158, 178, 199],
    ];

    let color = if team_id < 0 || team_id > 20 {
        [0, 0, 0]
    } else {
        TEAM_COLORS[team_id as usize]
    };
    let fill_color = Color32::from_rgb(color[0], color[1], color[2]);
    let outline_color = Color32::BLACK;
    let center = pos2(x, y);
    let stroke = (stroke_thickness, outline_color);
    ui.painter().circle(center, radius, fill_color, stroke);

    // Draw a line pointing in the direction of each player's aim
    let angle = (360.0 - target_yaw).to_radians();
    let endpoint = pos2(
        center.x + radius * angle.cos(),
        center.y + radius * angle.sin(),
    );
    ui.painter().line_segment([center, endpoint], stroke);
}

fn rotate_point(
    enemy_pos: [f32; 3],
    local_pos: [f32; 3],
    pos_x: f32,
    pos_y: f32,
    size_x: f32,
    size_y: f32,
    angle: f32,
    zoom: f32,
) -> ([f32; 3], bool) {
    let entity_pos = Vec3::from_array(enemy_pos);
    let local_pos = Vec3::from_array(local_pos);

    let r_1 = -(entity_pos.y - local_pos.y);
    let r_2 = entity_pos.x - local_pos.x;
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
