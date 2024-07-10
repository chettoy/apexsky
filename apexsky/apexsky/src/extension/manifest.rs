use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManifestDoc {
    manifest_version: i32,
    package_name: String,
    version_code: i32,
    version: String,
    label: String,
    description: String,
    icons: IndexMap<u16, String>,
    components: Vec<Components>,
    permissions: Vec<PermissionField>,
}

#[derive(Debug, Clone)]
pub struct Manifest(ManifestDoc);

impl Manifest {
    pub fn new(manifest: ManifestDoc) -> anyhow::Result<Self> {
        use anyhow::bail;
        use obfstr::obfstr as s;

        if manifest.manifest_version != 0 {
            bail!("{}", s!("Invalid manifest_version"));
        }
        if manifest.package_name.is_empty()
            || manifest.package_name.len() > 255
            || !manifest.package_name.is_ascii()
        {
            bail!("{}", s!("Invalid package_name"));
        }
        if manifest.version_code < 0 {
            bail!("{}", s!("Invalid version_code"));
        }
        if manifest.version.len() > 255 {
            bail!("{}", s!("Invalid version"));
        }
        for comp in &manifest.components {
            if let Some(t) = &comp.r#type {
                if !vec!["module"].contains(&t.as_str()) {
                    bail!("{}", s!("Invalid component type"));
                }
            }
        }
        for perm in &manifest.permissions {
            match perm {
                PermissionField::AccessGameWorld(_) => (),
                PermissionField::AccessGameInput(_) => (),
                PermissionField::AccessGameMemory(_) => (),
                PermissionField::AccessGlobalSettings(_) => (),
                PermissionField::Bluetooth(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
                PermissionField::Camera(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
                PermissionField::ManageGlobalSettings(_) => (),
                PermissionField::Internet(k) => {
                    if k != s!("d2rYQXElNtFTnBFryNFbAJS5J9aeaWfR") {
                        bail!("{}{:?}", s!("privilege denial: "), perm);
                    }
                }
                PermissionField::ModifyMemory(_) => (),
                PermissionField::Overlay(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
                PermissionField::QueryAllPackages(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
                PermissionField::RecordAudio(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
                PermissionField::Storage(k) => {
                    if k != s!("AOQR2c2UQO1cdzaPOr69MZVOAD7VdMja") {
                        bail!("{}{:?}", s!("privilege denial: "), perm);
                    }
                }
                PermissionField::SendAimbotActions(_) => (),
                PermissionField::SendInputActions(_) => (),
                PermissionField::ApexInjectHighlight(_) => {
                    bail!("{}{:?}", s!("This permission is not yet available: "), perm);
                }
            }
        }

        Ok(Self(manifest))
    }

    pub fn get_inner<'a>(&'a self) -> &'a ManifestDoc {
        &self.0
    }

    pub fn get_package_name<'a>(&'a self) -> &'a str {
        &self.0.package_name
    }

    pub fn get_version_code(&self) -> i32 {
        self.0.version_code
    }

    pub fn get_version_name<'a>(&'a self) -> &'a str {
        &self.0.version
    }

    pub fn get_label<'a>(&'a self) -> &'a str {
        &self.0.label
    }

    pub fn get_description<'a>(&'a self) -> &'a str {
        &self.0.description
    }

    pub fn get_permissions<'a>(&'a self) -> &'a Vec<PermissionField> {
        &self.0.permissions
    }

    pub fn get_main_module(&self) -> Option<String> {
        self.0
            .components
            .iter()
            .find(|&comp| comp.r#type.as_ref().is_some_and(|t| t == "module"))
            .map(|comp| comp.service.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Components {
    service: String,
    r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "name", content = "value")]
pub enum PermissionField {
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
    #[serde(rename = "com.chettoy.MODIFY_MEMORY")]
    ModifyMemory(String),
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
    let manifest: Manifest = Manifest::new(serde_json::from_str(test_str).unwrap()).unwrap();
    println!("{:?}", manifest);
}
