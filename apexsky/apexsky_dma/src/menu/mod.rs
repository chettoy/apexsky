use apexsky::{
    config::InstalledDlcItem,
    global_state::G_STATE,
    i18n::I18nBundle,
    i18n_msg,
    menu::{
        alert, apex_menu,
        apexsky_menu::{
            general_menu::{item_text, GeneralMenu, GeneralMenuName, MenuBuilder},
            ratatui, MenuState,
        },
    },
};
use crossterm::event::{KeyCode, MouseEvent};
use tokio::sync::oneshot;

use crate::{usermod_thr::UserModEvent, USERMOD_TX};

mod dlc_list;
mod install_view;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CustomMenuLevel {
    #[default]
    ApexskyMenu,
    MainMenu,
    DlcMenu,
}

impl Into<Box<dyn MenuState>> for CustomMenuLevel {
    fn into(self) -> Box<dyn MenuState> {
        match self {
            CustomMenuLevel::ApexskyMenu => Box::new(build_menu()),
            CustomMenuLevel::MainMenu => apex_menu::MenuLevel::MainMenu.into(),
            CustomMenuLevel::DlcMenu => DlcMenu::build(),
        }
    }
}

impl GeneralMenuName for CustomMenuLevel {
    fn rebuild_state(self) -> Box<dyn apexsky::menu::apexsky_menu::MenuState> {
        self.into()
    }
}

pub fn build_menu() -> GeneralMenu<'static, CustomMenuLevel> {
    let i18n_bundle = I18nBundle::new();
    MenuBuilder::new(CustomMenuLevel::ApexskyMenu)
        .title(i18n_msg!(&i18n_bundle, ApexskyMenuTitle))
        .add_item(
            item_text(format!("{}", i18n_msg!(&i18n_bundle, MenuItemOpenMainMenu))),
            |ctx, _| {
                ctx.nav_menu(CustomMenuLevel::MainMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!("{}", i18n_msg!(&i18n_bundle, MenuItemManageDlc))),
            |ctx, _| {
                ctx.nav_menu(CustomMenuLevel::DlcMenu);
                None
            },
            (),
        )
        .add_item(
            item_text(format!(
                "{}",
                i18n_msg!(&i18n_bundle, MenuItemCreativeWorkshop)
            )),
            |ctx, _| {
                let i18n_bundle = I18nBundle::new();
                alert(
                    ctx.app_model_mut(),
                    i18n_msg!(&i18n_bundle, InfoComingSoon).to_string(),
                );
                None
            },
            (),
        )
        .into()
}

#[derive(Debug, Clone)]
struct DlcMenu {
    table: dlc_list::TableApp,
    install_view: Option<install_view::InstallView>,
}

impl MenuState for DlcMenu {
    fn menu_id(&self) -> u64 {
        obfstr::random!(u64)
    }

    fn save_state(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }

    fn on_create(&mut self, saved_state: Option<Box<dyn std::any::Any>>) {
        let Some(saved_state) = saved_state else {
            return;
        };
        let Ok(state) = saved_state.downcast::<DlcMenu>() else {
            return;
        };
        self.table = state.table;
        self.install_view = state.install_view;
    }

    fn resize(&mut self, scroll_height: usize) {}

    fn nav_up(&mut self) {
        if let Some(install_view) = self.install_view.as_mut() {
            install_view.scroll_up();
        } else {
            self.table.previous();
        }
    }

    fn nav_down(&mut self) {
        if let Some(install_view) = self.install_view.as_mut() {
            install_view.scroll_down();
        } else {
            self.table.next();
        }
    }

    fn nav_jump(&mut self, num: usize) {}

    fn nav_mouse(&mut self, mouse: MouseEvent) {}

    fn nav_click(
        &mut self,
        ctx: &mut apexsky::menu::apexsky_menu::TerminalMenu,
        col: u16,
        row: u16,
    ) {
    }

    fn nav_press(
        &mut self,
        ctx: &mut apexsky::menu::apexsky_menu::TerminalMenu,
        key: crossterm::event::KeyCode,
    ) -> bool {
        if self.install_view.is_none() {
            match key {
                KeyCode::Char('e') => {
                    if let Some(data) = self.table.get_current_mut() {
                        if data.is_installed() {
                            G_STATE
                                .lock()
                                .unwrap()
                                .config
                                .dlc
                                .install
                                .shift_remove(data.package_name());
                            if apexsky::save_settings() {
                                data.set_installed(false);
                                if let Some(tx) = USERMOD_TX.read().clone() {
                                    let (callback_tx, _callback_rx) = oneshot::channel();
                                    let _ = tx.send(UserModEvent::KillPackage(
                                        data.package_name().to_owned(),
                                        callback_tx,
                                    ));
                                }
                            }
                            let i18n_bundle = &I18nBundle::new();
                            alert(
                                ctx.app_model_mut(),
                                i18n_msg!(i18n_bundle, InfoDlcUninstallSuccess).to_string(),
                            );
                        } else {
                            self.install_view = Some(install_view::InstallView::new(data.clone()));
                        }
                    }
                    return false;
                }
                KeyCode::Char('r') => {
                    return false;
                }
                _ => (),
            }
        }

        true
    }

    fn nav_enter(&mut self, ctx: &mut apexsky::menu::apexsky_menu::TerminalMenu) {
        if let Some(install_view) = self.install_view.as_mut() {
            if let Some(data) = self.table.get_current_mut() {
                if install_view.get_data().package_name() == data.package_name() {
                    G_STATE.lock().unwrap().config.dlc.install.insert(
                        data.package_name().to_owned(),
                        InstalledDlcItem {
                            checksum: data.checksum().to_owned(),
                        },
                    );
                    self.install_view = None;
                    if apexsky::save_settings() {
                        data.set_installed(true);
                        if let Some(tx) = USERMOD_TX.read().clone() {
                            let (callback_tx, _callback_rx) = oneshot::channel();
                            let _ = tx.send(UserModEvent::HotLoadPackage(
                                data.file_path().to_owned(),
                                Some(data.checksum().to_owned()),
                                callback_tx,
                            ));
                        }
                    }
                    let i18n_bundle = &I18nBundle::new();
                    alert(
                        ctx.app_model_mut(),
                        i18n_msg!(i18n_bundle, InfoDlcInstallSuccess).to_string(),
                    );
                }
            }
        } else {
            self.table.next_color();
        }
    }

    fn nav_back(&mut self) -> bool {
        if self.install_view.is_some() {
            self.install_view = None;
            return false;
        }
        true
    }

    fn update_menu(&mut self, ctx: &mut apexsky::menu::apexsky_menu::TerminalMenu) {}

    fn render(&mut self, f: &mut ratatui::Frame) {
        if let Some(install_view) = self.install_view.as_mut() {
            install_view.render(f);
        } else {
            dlc_list::ui(f, &mut self.table);
        }
    }
}

impl DlcMenu {
    fn new() -> Self {
        Self {
            table: dlc_list::TableApp::new(),
            install_view: None,
        }
    }

    pub fn build() -> Box<Self> {
        Box::new(Self::new())
    }
}
