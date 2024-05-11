use crate::{config, i18n_msg, lock_config, lock_mod, love_players::LovePlayer};
use fluent::{FluentBundle, FluentResource};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};
use skyapex_sdk::module::SpecCheck;

use super::{item_text, MenuBuilder, MenuLevel, MenuState, TerminalMenu};

pub(super) fn build_spectators_menu(
    i18n_bundle: FluentBundle<FluentResource>,
    _settings: config::Settings,
) -> MenuState<'static> {
    let mut menu = MenuBuilder::new().title(i18n_msg!(i18n_bundle, SpectatorsMenuTitle));
    menu = menu
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(i18n_bundle, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(i18n_bundle, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_item(
            item_text(format!(
                "({}) | \t{}",
                i18n_msg!(i18n_bundle, MenuItemBackToMainMenu),
                i18n_msg!(i18n_bundle, SpectatorsSection)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::MainMenu);
                None
            },
            (),
        )
        .add_item(item_text("🔁"), |_, _| None, ())
        .add_dummy_item();

    let specs = {
        let mut players = crate::love_players::get_players();
        let mut skyapex_mod = lock_mod!();
        players.retain(|target_ptr, _info| skyapex_mod.is_spec(*target_ptr));
        players
    };

    let list = &lock_config!().hate_player;
    for (_, spec) in specs {
        let selected = list.iter().fold(false, |acc: bool, x: &LovePlayer| {
            if let Some(x_uid) = x.uid {
                if x_uid == spec.uid {
                    return true;
                }
            }
            acc
        });
        menu = menu.add_item(
            ListItem::new(Line::from(vec![Span::styled(
                spec.name.replace(
                    |c: char| !c.is_alphanumeric() && !c.is_ascii_whitespace(),
                    "?",
                ),
                Style::default().fg(if selected { Color::Green } else { Color::Red }),
            )])),
            |_handle: &mut TerminalMenu, (spec, selected)| {
                let list = &mut lock_config!().hate_player;
                if selected {
                    list.retain(|x| x.uid != Some(spec.uid));
                } else {
                    list.retain(|x| x.uid != Some(spec.uid));
                    list.push(LovePlayer {
                        name: spec.name.to_owned(),
                        update_name: None,
                        uid: Some(spec.uid),
                        level: None,
                    });
                }
                None
            },
            (spec, selected),
        );
    }

    menu.into()
}
