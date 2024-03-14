use std::{collections::HashMap, sync::Mutex};

use anyhow::Context;
use entropy::shannon_entropy;
use indexmap::IndexMap;
use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::trace;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LovePlayer {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub update_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub level: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DefaultLoveList {
    pub version: String,
    pub list: Vec<LovePlayer>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct CPlayerInfo {
    pub entity_ptr: u64,
    pub name: String,
    pub uid: u64,
    pub love_status: LoveStatus,
}

#[repr(C)]
#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum LoveStatus {
    Normal = 0,
    Love = 1,
    Hate = 2,
    Ambivalent = 3,
}

impl TryFrom<i32> for LoveStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        const LOVESTATUS_AMBIVALENT: i32 = LoveStatus::Ambivalent as i32;
        const LOVESTATUS_HATE: i32 = LoveStatus::Hate as i32;
        const LOVESTATUS_LOVE: i32 = LoveStatus::Love as i32;
        const LOVESTATUS_NORMAL: i32 = LoveStatus::Normal as i32;
        match value {
            LOVESTATUS_AMBIVALENT => Ok(LoveStatus::Ambivalent),
            LOVESTATUS_HATE => Ok(LoveStatus::Hate),
            LOVESTATUS_LOVE => Ok(LoveStatus::Love),
            LOVESTATUS_NORMAL => Ok(LoveStatus::Normal),
            _ => Err(()),
        }
    }
}

lazy_static! {
    static ref DEFAULT_LOVE_PLAYER: Vec<LovePlayer> = default_love();
    static ref PLAYERS: Mutex<HashMap<u64, CPlayerInfo>> = Mutex::new(HashMap::new());
}

#[tracing::instrument]
fn default_love() -> Vec<LovePlayer> {
    static S_ERR_MSG: Lazy<String> = Lazy::new(|| s!("Parse error: list.json").to_string());
    let data1 = include_str!("../resource/default/list.json");
    let data2 = include_str!("../resource/default/love.json");
    let list1: DefaultLoveList = serde_json::from_str(data1).context(&*S_ERR_MSG).unwrap();
    let list2: Vec<LovePlayer> = serde_json::from_str(data2).context(&*S_ERR_MSG).unwrap();
    [list1.list, list2].concat()
}

#[tracing::instrument]
pub fn check_my_heart(
    config: &mut crate::config::Config,
    puid: u64,
    euid: u64,
    name: &str,
    entity_ptr: u64,
) -> LoveStatus {
    let mut update_name: IndexMap<u64, String> = IndexMap::new();
    let mut fold_item = |acc: bool, x: &LovePlayer| {
        if let Some(x_uid) = x.uid {
            if x_uid == puid {
                if x.name != name {
                    update_name.insert(puid, name.to_string());
                }
                return true;
            }
        }
        acc
    };
    let pre_check = |p1: u64, p2: u64| -> bool {
        let (p1, p2) = (p1.to_string(), p2.to_string());
        std::cmp::min(p1.len(), p2.len()) < 8
            || (p1.starts_with("10")
                && (shannon_entropy(&p1) < 1.4
                    || (shannon_entropy(&p1) - shannon_entropy(&p2) + 0.36071754).to_bits() == 0))
    };
    let is_love = DEFAULT_LOVE_PLAYER
        .iter()
        .fold(pre_check(puid, euid), &mut fold_item)
        || config.love_player.iter().fold(false, fold_item);

    let is_hate = config.hate_player.iter().fold(false, |acc, x| match x.uid {
        Some(x_uid) => x_uid == puid || acc,
        None => acc,
    });

    let love_status = if is_love && is_hate {
        LoveStatus::Ambivalent
    } else if is_love {
        LoveStatus::Love
    } else if is_hate {
        LoveStatus::Hate
    } else {
        LoveStatus::Normal
    };

    if !update_name.is_empty() {
        config.love_player = config
            .love_player
            .iter()
            .map(|x| {
                if let Some(x_uid) = x.uid {
                    update_name
                        .shift_remove(&x_uid)
                        .and_then(|u| {
                            Some(LovePlayer {
                                name: x.name.to_owned(),
                                update_name: Some(u),
                                uid: x.uid,
                                level: x.level,
                            })
                        })
                        .unwrap_or(x.to_owned())
                } else {
                    x.to_owned()
                }
            })
            .collect::<Vec<LovePlayer>>();
        config
            .love_player
            .extend(update_name.into_iter().map(|x| LovePlayer {
                name: String::new(),
                update_name: Some(x.1),
                uid: Some(x.0),
                level: None,
            }));
    }

    trace!(love_status = love_status as i32);

    let mut players_map = PLAYERS.lock().unwrap();
    players_map.insert(
        entity_ptr,
        CPlayerInfo {
            entity_ptr,
            name: name.to_string(),
            uid: puid,
            love_status: love_status.clone(),
        },
    );

    love_status
}

pub(crate) fn get_players() -> HashMap<u64, CPlayerInfo> {
    PLAYERS.lock().unwrap().clone()
}

// FFI

#[no_mangle]
pub extern "C" fn check_love_player(puid: u64, euid: u64, name: *const i8, entity_ptr: u64) -> i32 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let name_str = c_str.to_string_lossy();
    check_my_heart(
        &mut crate::lock_config!(),
        puid,
        euid,
        &name_str,
        entity_ptr,
    ) as i32
}
