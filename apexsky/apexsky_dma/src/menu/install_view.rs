use apexsky::{i18n::I18nBundle, i18n_msg, menu::apexsky_menu::ratatui};
use apexsky_extension::PermissionField;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use super::dlc_list::Data;

#[derive(Debug, Clone)]
pub struct InstallView {
    data: Data,
    vertical_scroll_state: ScrollbarState,
    vertical_scroll: usize,
    scroll_height: u16,
    i18n_bundle: I18nBundle,
    permission_lines: Vec<String>,
}

impl InstallView {
    pub fn new(data: Data) -> Self {
        let i18n = I18nBundle::new();

        let permission_lines = data
            .manifest()
            .get_permissions()
            .iter()
            .map(|p| match p {
                PermissionField::AccessGameWorld(_) => {
                    i18n_msg!(&i18n, PermissionFieldAccessGameWorld)
                }
                PermissionField::AccessGameInput(_) => {
                    i18n_msg!(&i18n, PermissionFieldAccessGameInput)
                }
                PermissionField::AccessGameMemory(_) => {
                    i18n_msg!(&i18n, PermissionFieldAccessGameMemory)
                }
                PermissionField::AccessGlobalSettings(_) => {
                    i18n_msg!(&i18n, PermissionFieldAccessGlobalSettings)
                }
                PermissionField::Bluetooth(_) => i18n_msg!(&i18n, PermissionFieldBluetooth),
                PermissionField::Camera(_) => i18n_msg!(&i18n, PermissionFieldCamera),
                PermissionField::ManageGlobalSettings(_) => {
                    i18n_msg!(&i18n, PermissionFieldManageGlobalSettings)
                }
                PermissionField::Internet(_) => i18n_msg!(&i18n, PermissionFieldInternet),
                PermissionField::ModifyMemory(_) => {
                    i18n_msg!(&i18n, PermissionFieldModifyMemory)
                }
                PermissionField::Overlay(_) => i18n_msg!(&i18n, PermissionFieldOverlay),
                PermissionField::QueryAllPackages(_) => {
                    i18n_msg!(&i18n, PermissionFieldQueryAllPackages)
                }
                PermissionField::RecordAudio(_) => i18n_msg!(&i18n, PermissionFieldRecordAudio),
                PermissionField::Storage(_) => i18n_msg!(&i18n, PermissionFieldStorage),
                PermissionField::SendAimbotActions(_) => {
                    i18n_msg!(&i18n, PermissionFieldSendAimbotActions)
                }
                PermissionField::SendInputActions(_) => {
                    i18n_msg!(&i18n, PermissionFieldSendInputActions)
                }
                PermissionField::ApexInjectHighlight(_) => {
                    i18n_msg!(&i18n, PermissionFieldApexInjectHighlight)
                }
            })
            .map(|s| format!(" {} ", s))
            .collect::<Vec<_>>();

        Self {
            data,
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            scroll_height: 0,
            i18n_bundle: i18n,
            permission_lines,
        }
    }

    pub fn get_data(&self) -> &Data {
        &self.data
    }

    pub fn scroll_down(&mut self) {
        if self.permission_lines.len() > self.scroll_height as usize {
            self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        } else {
            self.vertical_scroll = 0;
        }
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn render(&mut self, f: &mut Frame) {
        let popup_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Black));

        let frame_area = f.area();
        let graph_width = frame_area.width as f32 / 2.0;
        let graph_height = frame_area.height as f32;
        let ratio = if graph_width > graph_height {
            graph_height / graph_width
        } else {
            graph_width / graph_height
        };
        let scale = f32::min(1.618 / ratio, graph_height / graph_width / ratio);
        let area = centered_rect(
            (100.0 * scale * ratio / 1.618).round() as u16,
            (100.0 * scale * ratio * graph_width / graph_height).round() as u16,
            frame_area,
        );
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Percentage(30),
            ])
            .split(area);
        self.scroll_height = popup_chunks[1].height;

        let package_name_text = Paragraph::new(vec![
            Line::from(self.data.manifest().get_label()).white().bold(),
            Line::from(format!(
                "{}: {}",
                i18n_msg!(&self.i18n_bundle, DlcFieldVersionName),
                self.data.manifest().get_version_name()
            ))
            .gray(),
        ])
        .centered()
        .block(Block::default().bg(Color::Black));
        f.render_widget(package_name_text, popup_chunks[0]);

        let permission_text = Paragraph::new(
            self.permission_lines
                .iter()
                .map(|l| Line::from(l.as_str()))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .title(i18n_msg!(&self.i18n_bundle, DlcFieldPermissions).to_string())
                .fg(Color::White)
                .borders(Borders::ALL),
        )
        .scroll((self.vertical_scroll as u16, 0));
        f.render_widget(permission_text, popup_chunks[1]);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            popup_chunks[1],
            &mut self.vertical_scroll_state,
        );

        let permission_text =
            Paragraph::new("").block(Block::default().fg(Color::White).borders(Borders::ALL));
        f.render_widget(permission_text, popup_chunks[2]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
