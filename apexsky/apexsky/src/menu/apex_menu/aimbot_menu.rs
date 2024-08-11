use crate::{config, i18n::I18nBundle, i18n_msg, lock_config};
use ratatui::{
    style::{Color, Style},
    text::Span,
};

use super::{format_item, item_enabled, GeneralMenu, MenuBuilder, MenuLevel, TerminalMenu};

pub(super) fn build_aimbot_menu(
    i18n: &I18nBundle,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    let menu = MenuBuilder::new(MenuLevel::AimbotMenu).title(i18n_msg!(&i18n, AimbotMenuTitle));
    menu.add_item(
        item_enabled(
            &i18n,
            format!(" 1 - {}", i18n_msg!(&i18n, MenuItemKeyboard)),
            !settings.aimbot_settings.gamepad,
        ),
        |_, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
            None
        },
        (),
    )
    .add_item(
        item_enabled(
            &i18n,
            format!(" 2 - {}", i18n_msg!(&i18n, MenuItemGamepad)),
            settings.aimbot_settings.gamepad,
        ),
        |_, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
            None
        },
        (),
    )
    .add_input_item(
        format_item(
            &i18n,
            format!(" 3 - {}", i18n_msg!(&i18n, MenuItemAimbotMode)),
            match settings.aimbot_settings.aim_mode {
                0 | 8 => Span::from(i18n_msg!(&i18n, MenuValueAimbotOff).to_string()),
                1 | 9 => Span::styled(
                    i18n_msg!(&i18n, MenuValueAimbotNoVisCheck).to_string(),
                    Style::default().fg(Color::Red),
                ),
                2 | 3 => Span::styled(
                    i18n_msg!(&i18n, MenuValueAimbotOn).to_string(),
                    Style::default().fg(Color::Green),
                ),
                10 | 11 => Span::styled(
                    i18n_msg!(&i18n, MenuValueAimbotOn).to_string(),
                    Style::default().fg(Color::Blue),
                ),
                4 | 5 | 12 | 13 => Span::styled(
                    i18n_msg!(&i18n, MenuValueAimbotAssist).to_string(),
                    Style::default().fg(Color::Red),
                ),
                6 | 7 | 14 | 15 => Span::styled(
                    i18n_msg!(&i18n, MenuValueAimbotAssist).to_string(),
                    Style::default().fg(Color::Green),
                ),
                _ => Span::styled(
                    std::borrow::Cow::Borrowed("!").to_string(),
                    Style::default().fg(Color::Red),
                ),
            },
        ),
        &i18n_msg!(&i18n, InputPromptAimbotMode),
        |val| {
            let i18n = I18nBundle::new();
            let val = val.trim();
            if let Ok(new_val) = val.parse::<u8>() {
                if new_val < 16 {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.aim_mode = new_val.into();
                    return None;
                }
                return Some(i18n_msg!(&i18n, InfoInvalidValue).to_string());
            }
            Some(i18n_msg!(&i18n, InfoInvalidValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!(" 4 - {}", i18n_msg!(&i18n, MenuItemChangeAdsFov)),
            Span::from(format!("{}", settings.aimbot_settings.ads_fov)),
        ),
        &i18n_msg!(&i18n, InputPromptAdsFov),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (1.0..=50.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.ads_fov = new_val;
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidAdsFov).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!(" 5 - {}", i18n_msg!(&i18n, MenuItemChangeNonAdsFov)),
            Span::from(format!("{}", settings.aimbot_settings.non_ads_fov)),
        ),
        &i18n_msg!(&i18n, InputPromptNonAdsFov),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (1.0..=50.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.non_ads_fov = new_val;
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidNonAdsFov).to_string())
        },
    )
    .add_item(
        format_item(
            &i18n,
            format!(" 6 - {}", i18n_msg!(&i18n, MenuItemToggleNadeAim)),
            Span::from(
                if !settings.aimbot_settings.auto_nade_aim {
                    i18n_msg!(&i18n, MenuValueNoNadeAim)
                } else {
                    i18n_msg!(&i18n, MenuValueNadeAimOn)
                }
                .to_string(),
            ),
        ),
        |_, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.auto_nade_aim = !settings.aimbot_settings.auto_nade_aim;
            None
        },
        (),
    )
    .add_item(
        item_enabled(
            &i18n,
            format!(" 7 - {}", i18n_msg!(&i18n, MenuItemToggleNoRecoil)),
            settings.aimbot_settings.no_recoil,
        ),
        |_handle: &mut TerminalMenu, _| {
            let settings = &mut lock_config!().settings;
            settings.aimbot_settings.no_recoil = !settings.aimbot_settings.no_recoil;
            None
        },
        (),
    )
    .add_input_item(
        format_item(
            &i18n,
            format!(" 8 - {}", i18n_msg!(&i18n, MenuItemChangeBoneAim)),
            Span::from(
                if settings.aimbot_settings.bone_nearest {
                    i18n_msg!(&i18n, MenuValueBoneHitbox)
                } else if settings.aimbot_settings.bone_auto {
                    i18n_msg!(&i18n, MenuValueBoneAuto)
                } else {
                    match settings.aimbot_settings.bone {
                        0 => i18n_msg!(&i18n, MenuValueBoneHead),
                        1 => i18n_msg!(&i18n, MenuValueBoneNeck),
                        2 => i18n_msg!(&i18n, MenuValueBoneChest),
                        3 => i18n_msg!(&i18n, MenuValueBoneGutShut),
                        _ => i18n_msg!(&i18n, MenuValueBoneUnknown),
                    }
                }
                .to_string(),
            ),
        ),
        &i18n_msg!(&i18n, InputPromptBoneValue),
        |val| {
            let i18n = I18nBundle::new();
            let val = val.trim();
            if val == "x" {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.bone_auto = true;
                settings.aimbot_settings.bone_nearest = false;
                return None;
            } else if val == "h" {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.bone_nearest = true;
                settings.aimbot_settings.bone_auto = false;
                return None;
            } else if let Ok(new_val) = val.parse::<u8>() {
                if [0, 1, 2, 3].contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.bone = new_val.into();
                    settings.aimbot_settings.bone_auto = false;
                    settings.aimbot_settings.bone_nearest = false;
                    return None;
                }
                return Some(i18n_msg!(&i18n, InfoInvalidBoneValue).to_string());
            }
            Some(i18n_msg!(&i18n, InfoInvalidValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!(" 9 - {}", i18n_msg!(&i18n, MenuItemAimDist)),
            Span::from(format!("{}m", settings.aimbot_settings.aim_dist / 39.62)),
        ),
        &i18n_msg!(&i18n, InputPromptAimDist),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (10.0..=1600.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.aim_dist = new_val * 39.62;
                    return None;
                }
            }
            None
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!("10 - {}", i18n_msg!(&i18n, MenuItemHeadshotDist)),
            Span::from(format!(
                "{}m",
                settings.aimbot_settings.headshot_dist / 39.62
            )),
        ),
        &i18n_msg!(&i18n, InputPromptHeadshotDist),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (0.0..=1600.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.headshot_dist = new_val * 39.62;
                    return None;
                }
            }
            None
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!("11 - {}", i18n_msg!(&i18n, MenuItemSmoothValue)),
            if settings.aimbot_settings.smooth < 150.0 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.smooth),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.smooth >= 200.0 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.smooth),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}", settings.aimbot_settings.smooth))
            },
        ),
        &i18n_msg!(&i18n, InputPromptSmoothValue),
        |val| {
            if let Ok(new_val) = val.parse::<u16>() {
                if (50..=1000).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.smooth = new_val.into();
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidSmoothValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!("12 - {}", i18n_msg!(&i18n, MenuItemSkynadeSmooth)),
            if settings.aimbot_settings.skynade_smooth < 150.0 * 0.6667 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.skynade_smooth),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.skynade_smooth > 250.0 * 0.6667 {
                Span::styled(
                    format!("{}", settings.aimbot_settings.skynade_smooth),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}", settings.aimbot_settings.skynade_smooth))
            },
        ),
        &i18n_msg!(&i18n, InputPromptSmoothValue),
        |val| {
            if let Ok(new_val) = val.parse::<u16>() {
                if (50..=1000).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.skynade_smooth = new_val.into();
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidSmoothValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!("13 - {}", i18n_msg!(&i18n, MenuItemRecoilXValue)),
            if settings.aimbot_settings.recoil_smooth_x > 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_x),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.recoil_smooth_x <= 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_x),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}%", settings.aimbot_settings.recoil_smooth_x))
            },
        ),
        &i18n_msg!(&i18n, InputPromptRecoilValue),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (0.0..=200.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_x = new_val.into();
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidRecoilValue).to_string())
        },
    )
    .add_input_item(
        format_item(
            &i18n,
            format!("14 - {}", i18n_msg!(&i18n, MenuItemRecoilYValue)),
            if settings.aimbot_settings.recoil_smooth_y > 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_y),
                    Style::default().fg(Color::Red),
                )
            } else if settings.aimbot_settings.recoil_smooth_y <= 70.0 {
                Span::styled(
                    format!("{}%", settings.aimbot_settings.recoil_smooth_y),
                    Style::default().fg(Color::Green),
                )
            } else {
                Span::from(format!("{}%", settings.aimbot_settings.recoil_smooth_y))
            },
        ),
        &i18n_msg!(&i18n, InputPromptRecoilValue),
        |val| {
            if let Ok(new_val) = val.parse::<f32>() {
                if (0.0..=200.0).contains(&new_val) {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_y = new_val.into();
                    return None;
                }
            }
            let i18n = I18nBundle::new();
            Some(i18n_msg!(&i18n, InfoInvalidRecoilValue).to_string())
        },
    )
    .into()
}
