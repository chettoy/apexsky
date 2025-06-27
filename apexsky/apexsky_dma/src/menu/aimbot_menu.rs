use super::{
    GeneralMenu, GeneralMenuFormat, MenuBuilder, MenuBuilderExt, MenuFormatter, MenuLevel,
    TerminalMenu, ratatui,
};
use crate::{config, i18n::I18nBundle, i18n_msg, lock_config};
use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub(super) fn build_aimbot_menu(
    menu_fmt: &MenuFormatter,
    settings: config::Settings,
) -> GeneralMenu<'static, MenuLevel> {
    MenuBuilder::new(MenuLevel::Aimbot, menu_fmt.clone())
        .title(i18n_msg!(&menu_fmt, AimbotMenuTitle))
        .add_input_item(
            menu_fmt.format_item(
                format!(" 1 - {}", i18n_msg!(&menu_fmt, MenuItemAimbotMode)),
                match settings.aimbot_settings.aim_mode {
                    0 | 8 => Span::from(i18n_msg!(&menu_fmt, MenuValueAimbotOff).to_string()),
                    1 | 9 => Span::styled(
                        i18n_msg!(&menu_fmt, MenuValueAimbotNoVisCheck).to_string(),
                        Style::default().fg(Color::Red),
                    ),
                    2 | 3 => Span::styled(
                        i18n_msg!(&menu_fmt, MenuValueAimbotOn).to_string(),
                        Style::default().fg(Color::Green),
                    ),
                    10 | 11 => Span::styled(
                        i18n_msg!(&menu_fmt, MenuValueAimbotOn).to_string(),
                        Style::default().fg(Color::Blue),
                    ),
                    4 | 5 | 12 | 13 => Span::styled(
                        i18n_msg!(&menu_fmt, MenuValueAimbotAssist).to_string(),
                        Style::default().fg(Color::Red),
                    ),
                    6 | 7 | 14 | 15 => Span::styled(
                        i18n_msg!(&menu_fmt, MenuValueAimbotAssist).to_string(),
                        Style::default().fg(Color::Green),
                    ),
                    _ => Span::styled(
                        std::borrow::Cow::Borrowed("!").to_string(),
                        Style::default().fg(Color::Red),
                    ),
                },
            ),
            &i18n_msg!(&menu_fmt, InputPromptAimbotMode),
            |_ctx, val, _| {
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
            (),
        )
        .add_dummy_item()
        .skip_id()
        .add_toggle_item(
            format!(" 3 - {}", i18n_msg!(&menu_fmt, MenuItemKeyboard)),
            !settings.aimbot_settings.gamepad,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
                if settings.aimbot_settings.gamepad {
                    settings.aimbot_settings.aimbot_on_ads = true;
                    settings.aimbot_settings.aimbot_on_fire = true;
                }
                None
            },
        )
        .add_toggle_item(
            format!(" 4 - {}", i18n_msg!(&menu_fmt, MenuItemGamepad)),
            settings.aimbot_settings.gamepad,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.gamepad = !settings.aimbot_settings.gamepad;
                if settings.aimbot_settings.gamepad {
                    settings.aimbot_settings.aimbot_on_ads = true;
                    settings.aimbot_settings.aimbot_on_fire = true;
                }
                None
            },
        )
        .add_toggle_item(
            format!(" 5 - {}", i18n_msg!(&menu_fmt, MenuItemAimbotOnAds)),
            settings.aimbot_settings.aimbot_on_ads,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.aimbot_on_ads = !settings.aimbot_settings.aimbot_on_ads;
                None
            },
        )
        .add_toggle_item(
            format!(" 6 - {}", i18n_msg!(&menu_fmt, MenuItemAimbotOnFire)),
            settings.aimbot_settings.aimbot_on_fire,
            |_, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.aimbot_on_fire = !settings.aimbot_settings.aimbot_on_fire;
                None
            },
        )
        .add_dummy_item()
        .add_input_item(
            menu_fmt.format_item(
                format!(" 7 - {}", i18n_msg!(&menu_fmt, MenuItemChangeAdsFov)),
                Span::from(format!("{}", settings.aimbot_settings.ads_fov)),
            ),
            &i18n_msg!(&menu_fmt, InputPromptAdsFov),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (1.0..=50.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.ads_fov = new_val;
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidAdsFov).to_string())
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!(" 8 - {}", i18n_msg!(&menu_fmt, MenuItemChangeNonAdsFov)),
                Span::from(format!("{}", settings.aimbot_settings.non_ads_fov)),
            ),
            &i18n_msg!(&menu_fmt, InputPromptNonAdsFov),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (1.0..=50.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.non_ads_fov = new_val;
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidNonAdsFov).to_string())
            },
            (),
        )
        .add_item(
            menu_fmt.format_item(
                format!(" 9 - {}", i18n_msg!(&menu_fmt, MenuItemToggleNadeAim)),
                Span::from(
                    if !settings.aimbot_settings.auto_nade_aim {
                        i18n_msg!(&menu_fmt, MenuValueNoNadeAim)
                    } else {
                        i18n_msg!(&menu_fmt, MenuValueNadeAimOn)
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
        .add_input_item(
            menu_fmt.format_item(
                format!("10 - {}", i18n_msg!(&menu_fmt, MenuItemChangeBoneAim)),
                Span::from(
                    if settings.aimbot_settings.bone_nearest {
                        i18n_msg!(&menu_fmt, MenuValueBoneHitbox)
                    } else if settings.aimbot_settings.bone_auto {
                        i18n_msg!(&menu_fmt, MenuValueBoneAuto)
                    } else {
                        match settings.aimbot_settings.bone {
                            0 => i18n_msg!(&menu_fmt, MenuValueBoneHead),
                            1 => i18n_msg!(&menu_fmt, MenuValueBoneNeck),
                            2 => i18n_msg!(&menu_fmt, MenuValueBoneChest),
                            3 => i18n_msg!(&menu_fmt, MenuValueBoneGutShut),
                            _ => i18n_msg!(&menu_fmt, MenuValueBoneUnknown),
                        }
                    }
                    .to_string(),
                ),
            ),
            &i18n_msg!(&menu_fmt, InputPromptBoneValue),
            |_ctx, val, _| {
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
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("11 - {}", i18n_msg!(&menu_fmt, MenuItemAimDist)),
                Span::from(format!("{}m", settings.aimbot_settings.aim_dist / 39.62)),
            ),
            &i18n_msg!(&menu_fmt, InputPromptAimDist),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (10.0..=1600.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.aim_dist = new_val * 39.62;
                    return None;
                }
                None
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("12 - {}", i18n_msg!(&menu_fmt, MenuItemHeadshotDist)),
                Span::from(format!(
                    "{}m",
                    settings.aimbot_settings.headshot_dist / 39.62
                )),
            ),
            &i18n_msg!(&menu_fmt, InputPromptHeadshotDist),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (0.0..=1600.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.headshot_dist = new_val * 39.62;
                    return None;
                }
                None
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("13 - {}", i18n_msg!(&menu_fmt, MenuItemSmoothValue)),
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
            &i18n_msg!(&menu_fmt, InputPromptSmoothValue),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<u16>()
                    && (50..=1000).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.smooth = new_val.into();
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidSmoothValue).to_string())
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("14 - {}", i18n_msg!(&menu_fmt, MenuItemSkynadeSmooth)),
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
            &i18n_msg!(&menu_fmt, InputPromptSmoothValue),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<u16>()
                    && (50..=1000).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.skynade_smooth = new_val.into();
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidSmoothValue).to_string())
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("15 - {}", i18n_msg!(&menu_fmt, MenuItemTriggerbotSmooth)),
                if settings.aimbot_settings.triggerbot_smooth < 80.0 {
                    Span::styled(
                        format!("{}", settings.aimbot_settings.triggerbot_smooth),
                        Style::default().fg(Color::Red),
                    )
                } else if settings.aimbot_settings.triggerbot_smooth >= 150.0 {
                    Span::styled(
                        format!("{}", settings.aimbot_settings.triggerbot_smooth),
                        Style::default().fg(Color::Green),
                    )
                } else {
                    Span::from(format!("{}", settings.aimbot_settings.triggerbot_smooth))
                },
            ),
            &i18n_msg!(&menu_fmt, InputPromptSmoothValue),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<u16>()
                    && (50..=1000).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.triggerbot_smooth = new_val.into();
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidSmoothValue).to_string())
            },
            (),
        )
        .add_toggle_item(
            format!("16 - {}", i18n_msg!(&menu_fmt, MenuItemToggleNoRecoil)),
            settings.aimbot_settings.no_recoil,
            |_handle: &mut TerminalMenu, _| {
                let settings = &mut lock_config!().settings;
                settings.aimbot_settings.no_recoil = !settings.aimbot_settings.no_recoil;
                None
            },
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("17 - {}", i18n_msg!(&menu_fmt, MenuItemRecoilXValue)),
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
            &i18n_msg!(&menu_fmt, InputPromptRecoilValue),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (0.0..=200.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_x = new_val;
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidRecoilValue).to_string())
            },
            (),
        )
        .add_input_item(
            menu_fmt.format_item(
                format!("18 - {}", i18n_msg!(&menu_fmt, MenuItemRecoilYValue)),
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
            &i18n_msg!(&menu_fmt, InputPromptRecoilValue),
            |_ctx, val, _| {
                if let Ok(new_val) = val.parse::<f32>()
                    && (0.0..=200.0).contains(&new_val)
                {
                    let settings = &mut lock_config!().settings;
                    settings.aimbot_settings.recoil_smooth_y = new_val;
                    return None;
                }
                let i18n = I18nBundle::new();
                Some(i18n_msg!(&i18n, InfoInvalidRecoilValue).to_string())
            },
            (),
        )
        .into()
}
