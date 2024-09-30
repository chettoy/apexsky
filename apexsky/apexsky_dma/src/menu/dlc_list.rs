use std::path::PathBuf;

use apexsky::{
    global_state::G_STATE,
    i18n::I18nBundle,
    i18n_msg,
    menu::apexsky_menu::{ratatui, unicode_width::UnicodeWidthStr},
};
use apexsky_extension::{Manifest, PackageManager};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
    Frame,
};
use style::palette::tailwind;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const ITEM_HEIGHT: usize = 5;

#[derive(Debug, Clone)]
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Data {
    label: String,
    description: String,
    package_name: String,
    version_text: String,
    manifest: Manifest,
    checksum: String,
    file_path: PathBuf,
    is_installed: bool,
    installed_label: (String, String),
}

impl Data {
    pub(super) fn label(&self) -> &str {
        &self.label
    }

    pub(super) fn description(&self) -> &str {
        &self.description
    }

    pub(super) fn package_name(&self) -> &str {
        &self.package_name
    }

    pub(super) fn version_text(&self) -> String {
        format!(
            "{}: {}",
            if self.is_installed {
                &self.installed_label.0
            } else {
                &self.installed_label.1
            },
            self.version_text
        )
    }

    pub(super) fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub(super) fn checksum(&self) -> &str {
        &self.checksum
    }

    pub(super) fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub(super) fn is_installed(&self) -> bool {
        self.is_installed
    }

    pub(super) fn set_installed(&mut self, v: bool) {
        self.is_installed = v;
    }
}

#[derive(Debug, Clone)]
pub(super) struct TableApp {
    state: TableState,
    items: Vec<Data>,
    longest_item_lens: (u16, u16, u16), // order is (name, address, email)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
    i18n_bundle: I18nBundle,
}

impl TableApp {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let data_vec = rt.block_on(read_packages()).unwrap_or_default();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
            i18n_bundle: I18nBundle::new(),
        }
    }

    pub fn get_current(&self) -> Option<&Data> {
        self.items.get(self.state.selected()?)
    }

    pub fn get_current_mut(&mut self) -> Option<&mut Data> {
        self.items.get_mut(self.state.selected()?)
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    // pub fn previous_color(&mut self) {
    //     let count = PALETTES.len();
    //     self.color_index = (self.color_index + count - 1) % count;
    // }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }
}

async fn read_packages() -> anyhow::Result<Vec<Data>> {
    let dlc_dir = apexsky::get_base_dir().join("mods");

    let mut install_mgr = PackageManager::default();

    for entry in std::fs::read_dir(dlc_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }
        if let Some(file_ext) = path.extension() {
            if file_ext != "spk" {
                continue;
            }
        } else {
            continue;
        }

        install_mgr.install(path, None).await.ok();
    }

    let i18n_bundle = I18nBundle::new();
    let install_list = G_STATE.lock().unwrap().config.dlc.install.clone();
    let mut list: Vec<Data> = install_mgr
        .get_all_installed()
        .values()
        .map(|installed| {
            let is_installed = install_list
                .get(&installed.package_name)
                .is_some_and(|item| item.checksum == installed.checksum);
            Data {
                label: installed.manifest.get_label().to_owned(),
                description: installed.manifest.get_description().to_owned(),
                package_name: installed.package_name.to_owned(),
                version_text: format!(
                    "{} ({})",
                    installed.manifest.get_version_name(),
                    installed.manifest.get_version_code()
                ),
                manifest: installed.manifest.clone(),
                checksum: installed.checksum.clone(),
                file_path: installed.file_path.clone(),
                is_installed,
                installed_label: (
                    i18n_msg!(&i18n_bundle, DlcInstalledVersion).to_string(),
                    i18n_msg!(&i18n_bundle, DlcNotInstalledVersion).to_string(),
                ),
            }
        })
        .collect();

    list.sort_by(|a, b| a.label.cmp(&b.label));

    Ok(list)
}

pub(super) fn ui(f: &mut Frame, app: &mut TableApp) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.area());

    app.set_colors();

    render_table(f, app, rects[0]);

    render_scrollbar(f, app, rects[0]);

    render_footer(f, app, rects[1]);
}

fn render_table(f: &mut Frame, app: &mut TableApp, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let i18n_bundle = &app.i18n_bundle;
    let header = [
        i18n_msg!(i18n_bundle, DlcFieldLabel),
        i18n_msg!(i18n_bundle, DlcFieldDescription),
        i18n_msg!(i18n_bundle, DlcFieldPackageName),
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(header_style)
    .height(1);
    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        Row::new(vec![
            Cell::from(
                Text::from(format!("\n\n{}\n\n", data.label))
                    .bold()
                    .centered(),
            ),
            Cell::from(Text::from(format!("\n{}\n", data.description))),
            Cell::from(Text::from(format!(
                "\n{}\n\n{}",
                data.package_name(),
                data.version_text()
            ))),
        ])
        .style(Style::new().fg(app.colors.row_fg).bg(color))
        .height(5)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(app.longest_item_lens.0 + 1),
            Constraint::Min(app.longest_item_lens.1 + 1),
            Constraint::Min(app.longest_item_lens.2),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16) {
    let label_len = items
        .iter()
        .map(Data::label)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let desc_len = items
        .iter()
        .map(Data::description)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let pname_len = items
        .iter()
        .map(Data::package_name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let version_len = items
        .iter()
        .map(Data::version_text)
        .map(|s| UnicodeWidthStr::width(s.as_str()))
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (
        u16::max(4, label_len as u16),
        desc_len as u16,
        usize::max(pname_len, version_len) as u16,
    )
}

fn render_scrollbar(f: &mut Frame, app: &mut TableApp, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(f: &mut Frame, app: &TableApp, area: Rect) {
    let i18n = &app.i18n_bundle;
    let info_footer = Paragraph::new(Line::from(
        if app
            .get_current()
            .map(|i| i.is_installed())
            .unwrap_or_default()
        {
            format!(
                "{}{}{}{}{}{}{}",
                i18n_msg!(i18n, DlcListLabelQuit),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelMoveUp),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelMoveDown),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelUninstall),
                // i18n_msg!(i18n, DlcListLabelSeparator),
                // i18n_msg!(i18n, DlcListLabelReload),
            )
        } else {
            format!(
                "{}{}{}{}{}{}{}",
                i18n_msg!(i18n, DlcListLabelQuit),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelMoveUp),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelMoveDown),
                i18n_msg!(i18n, DlcListLabelSeparator),
                i18n_msg!(i18n, DlcListLabelInstall)
            )
        },
    ))
    .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
    .centered()
    .block(
        Block::bordered()
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(app.colors.footer_border_color)),
    );
    f.render_widget(info_footer, area);
}
