use super::{
    GeneralMenu, GeneralMenuFormat, MenuBuilder, MenuFormatter, MenuLevel, TerminalMenu, ratatui,
};
use crate::{config, i18n_msg, lock_config, love_players::LovePlayer};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};

pub(super) fn build_players_menu(
    menu_fmt: &MenuFormatter,
    _settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    let mut menu = MenuBuilder::new(MenuLevel::Players, menu_fmt.clone())
        .title(i18n_msg!(menu_fmt, PlayersMenuTitle))
        .add_item(
            ListItem::new(Line::from(vec![
                Span::from(i18n_msg!(menu_fmt, RedIsDisable).to_string()).red(),
                Span::from(" - ").dark_gray(),
                Span::from(i18n_msg!(menu_fmt, GreedIsEnabled).to_string()).green(),
            ])),
            |_, _| None,
            (),
        )
        .no_id()
        .add_dummy_item()
        .add_item(
            menu_fmt.item_text(format!(
                "({}) | \t{}",
                i18n_msg!(menu_fmt, MenuItemBackToMainMenu),
                i18n_msg!(menu_fmt, PlayersSection)
            )),
            |handle: &mut TerminalMenu, _| {
                handle.nav_menu(MenuLevel::Main);
                None
            },
            (),
        )
        .add_item(menu_fmt.item_text("🔁"), |_, _| None, ())
        .add_dummy_item();

    // let specs = {
    //     let mut players = crate::love_players::get_players();
    //     let mut skyapex_mod = lock_mod!();
    //     players.retain(|target_ptr, _info| skyapex_mod.is_spec(*target_ptr));
    //     players
    // };
    let specs = crate::love_players::get_uid_players_map().clone();

    let list = &lock_config!().hate_player;
    for (uid, spec) in specs.into_iter() {
        let selected = list.iter().fold(false, |acc: bool, x: &LovePlayer| {
            if let Some(x_uid) = x.uid
                && x_uid == uid
            {
                return true;
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
