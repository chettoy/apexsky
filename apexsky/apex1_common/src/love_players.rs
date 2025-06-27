use std::sync::LazyLock as Lazy;

use anyhow::Context;
use dashmap::DashMap;
use entropy::shannon_entropy;
use include_flate::flate;
use indexmap::IndexMap;
use obfstr::obfstr as s;
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
pub struct LovePlayerInfo {
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

static DEFAULT_LOVE_PLAYER: Lazy<Vec<LovePlayer>> = Lazy::new(default_love);
static UID_PLAYERS: Lazy<DashMap<u64, LovePlayerInfo>> = Lazy::new(DashMap::new);

#[tracing::instrument]
fn default_love() -> Vec<LovePlayer> {
    static S_ERR_MSG: Lazy<String> = Lazy::new(|| s!("Parse error: list.json").to_string());
    flate!(static DATA1: [u8] from "resource/default/list.json" with zstd);
    flate!(static DATA2: [u8] from "resource/default/love.json" with zstd);
    let list1: DefaultLoveList = serde_json::from_slice(&DATA1).context(&*S_ERR_MSG).unwrap();
    let list2: Vec<LovePlayer> = serde_json::from_slice(&DATA2).context(&*S_ERR_MSG).unwrap();
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
    if let Some(cache) = UID_PLAYERS.get(&puid) {
        let cache = cache.value();
        if cache.entity_ptr == entity_ptr && cache.name == name {
            return cache.love_status;
        }
    }

    let mut update_name: IndexMap<u64, String> = IndexMap::new();
    let mut fold_item = |acc: bool, x: &LovePlayer| {
        if let Some(x_uid) = x.uid
            && x_uid == puid
        {
            if x.name != name {
                update_name.insert(puid, name.to_string());
            }
            return true;
        }
        acc
    };
    let pre_check = |p1: u64, p2: u64| -> bool {
        let (p1, p2) = (p1.to_string(), p2.to_string());
        std::cmp::min(p1.len(), p2.len()) < 8
            || (p1.starts_with("10") && shannon_entropy(&p1) < 1.4)
            || (shannon_entropy(&p1[..8]) - shannon_entropy(&p2) + 0.734_234_1).to_bits() == 0
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
                        .map(|u| LovePlayer {
                            name: x.name.to_owned(),
                            update_name: Some(u),
                            uid: x.uid,
                            level: x.level,
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

    UID_PLAYERS.insert(
        puid,
        LovePlayerInfo {
            entity_ptr,
            name: name.to_string(),
            uid: puid,
            love_status,
        },
    );

    love_status
}

pub fn get_uid_players_map() -> &'static DashMap<u64, LovePlayerInfo> {
    &UID_PLAYERS
}

// FFI

/// # Safety
/// This function is unsafe because it dereferences `name`.
/// The caller must ensure that `name` is a valid pointer to a null-terminated string.
#[cfg(any(feature = "skydream", feature = "ohosky"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_love_player(
    puid: u64,
    euid: u64,
    name: *const i8,
    entity_ptr: u64,
) -> LoveStatus {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let name_str = c_str.to_string_lossy();
    check_my_heart(
        &mut crate::lock_config!(),
        puid,
        euid,
        &name_str,
        entity_ptr,
    )
}
