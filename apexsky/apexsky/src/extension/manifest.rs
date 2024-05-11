use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    manifest_version: i32,
    name: String,
    version: String,
    description: String,
    icons: IndexMap<u16, String>,
    components: Vec<Components>,
    permissions: Vec<Permission>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Components {
    service: String,
    r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", content = "value")]
pub enum Permission {
    #[serde(rename = "com.chettoy.ACCESS_GAME_WORLD")]
    AccessGameWorld(String),
    #[serde(rename = "com.chettoy.ACCESS_GAME_INPUT")]
    AccessGameInput(String),
    #[serde(rename = "com.chettoy.ACCESS_GAME_MEMORY")]
    AccessGameMemory(String),
    #[serde(rename = "com.chettoy.ACCESS_GLOBAL_SETTINGS")]
    AccessGlobalSettings(String),
    #[serde(rename = "com.chettoy.BLUETOOTH")]
    Bluetooth(String),
    #[serde(rename = "com.chettoy.CAMERA")]
    Camera(String),
    #[serde(rename = "com.chettoy.MANAGE_GLOBAL_SETTINGS")]
    ManageGlobalSettings(String),
    #[serde(rename = "com.chettoy.INTERNET")]
    Internet(String),
    #[serde(rename = "com.chettoy.OVERLAY")]
    Overlay(String),
    #[serde(rename = "com.chettoy.QUERY_ALL_PACKAGES")]
    QueryAllPackages(String),
    #[serde(rename = "com.chettoy.RECORD_AUDIO")]
    RecordAudio(String),
    #[serde(rename = "com.chettoy.STORAGE")]
    Storage(String),
    #[serde(rename = "com.chettoy.SEND_AIMBOT_ACTIONS")]
    SendAimbotActions(String),
    #[serde(rename = "com.chettoy.SEND_INPUT_ACTIONS")]
    SendInputActions(String),
    #[serde(rename = "com.chettoy.apex.INJECT_HIGHLIGHT")]
    ApexInjectHighlight(String),
}

#[test]
fn test_parse_manifest() {
    let test_str = include_str!("../../resource/extensions/example/manifest.json");
    let manifest: Manifest = serde_json::from_str(test_str).unwrap();
    println!("{:?}", manifest);
}
