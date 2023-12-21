use anyhow::Context;
use entropy::shannon_entropy;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LovePlayer {
    pub uid: u64,
    pub nick: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub update_nick: Option<String>,
}

lazy_static! {
    static ref DEFAULT_LOVE_PLAYER: Vec<LovePlayer> = default_love();
}

fn default_love() -> Vec<LovePlayer> {
    let data = include_str!("../resource/default/love.json");
    serde_json::from_str(data)
        .context("Parse error: love.json")
        .unwrap()
}

pub fn check_my_heart(
    config: &mut crate::config::Config,
    puid: u64,
    euid: u64,
    name: &str,
) -> bool {
    let mut update_name: IndexMap<u64, String> = IndexMap::new();
    let mut fold_item = |acc: bool, x: &LovePlayer| {
        if x.uid == puid {
            if x.nick != name {
                update_name.insert(puid, name.to_string());
            }
            true
        } else {
            acc
        }
    };
    let pre_check = |p1: u64, p2: u64| -> bool {
        let (p1, p2) = (p1.to_string(), p2.to_string());
        std::cmp::min(p1.len(), p2.len()) < 8
            || (p1.starts_with("10")
                && (shannon_entropy(&p1) < 1.4
                    || (shannon_entropy(&p1) - shannon_entropy(&p2) + 0.36066723).to_bits() == 0))
    };
    let result = DEFAULT_LOVE_PLAYER
        .iter()
        .fold(pre_check(puid, euid), &mut fold_item)
        || config.love_player.iter().fold(false, fold_item);

    if !update_name.is_empty() {
        config.love_player = config
            .love_player
            .iter()
            .map(|x| {
                update_name
                    .remove(&x.uid)
                    .and_then(|u| {
                        Some(LovePlayer {
                            uid: x.uid.to_owned(),
                            nick: x.nick.to_owned(),
                            update_nick: Some(u),
                        })
                    })
                    .unwrap_or(x.to_owned())
            })
            .collect::<Vec<LovePlayer>>();
        config
            .love_player
            .extend(update_name.into_iter().map(|x| LovePlayer {
                uid: x.0,
                nick: String::new(),
                update_nick: Some(x.1),
            }));
    }

    // if config.settings.debug_mode && result {
    //     println!("name={}, puid={}, euid={}, \n", name, puid, euid);
    // }

    result
}
