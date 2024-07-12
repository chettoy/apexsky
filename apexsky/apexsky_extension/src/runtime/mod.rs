pub(super) mod game_api;
mod ops;
pub(super) mod permission;

use std::time::Duration;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, bail};
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::error::AnyError;
use deno_core::*;
use permission::RuntimePermission;
use serde::Serialize;
use tokio::sync::oneshot;

use ops::*;

use super::manifest::Manifest;
use game_api::apexsky_game_api;

extension!(
  apexsky_extension,
  ops = [
    op_poll_message,
    op_message_callback,
    op_read_file,
    op_write_file,
    op_remove_file,
    op_fetch,
    op_set_timeout
  ],
  esm_entry_point = "ext:apexsky_extension/runtime.js",
  esm = [ dir "src/runtime", "runtime.js" ],
  options = {
    manifest: Manifest,
    perms: Box<RuntimePermission>,
    msg_rx: async_channel::Receiver<ExtensionMessage>,
  },
  state = |state, options| {
    state.put::<Manifest>(options.manifest);
    state.put::<Box<RuntimePermission>>(options.perms);
    state.put::<async_channel::Receiver<ExtensionMessage>>(options.msg_rx);
  },
  docs = "apexsky runtime op2.",
);

#[derive(Debug)]
pub struct ExtensionMessage {
    pub inner: ExtensionMessageInner,
    pub callback: Option<ExtensionMessageCallback>,
}

#[derive(Debug, Serialize)]
pub struct ExtensionMessageInner {
    pub name: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ExtensionMessageCallback(oneshot::Sender<serde_json::Value>);

impl ExtensionMessage {
    pub fn new(name: String, callback: Option<oneshot::Sender<serde_json::Value>>) -> Self {
        ExtensionMessage {
            inner: ExtensionMessageInner { name, data: None },
            callback: callback.and_then(|c| Some(ExtensionMessageCallback(c))),
        }
    }

    pub fn new_with_data<D: Serialize>(
        name: String,
        data: Option<D>,
        callback: Option<oneshot::Sender<serde_json::Value>>,
    ) -> Result<Self, serde_json::Error> {
        let data = match data {
            Some(v) => Some(serde_json::to_value(v)?),
            None => None,
        };
        Ok(ExtensionMessage {
            inner: ExtensionMessageInner { name, data },
            callback: callback.and_then(|c| Some(ExtensionMessageCallback(c))),
        })
    }
}

#[op2(async)]
#[serde]
pub(super) async fn op_poll_message(
    state: Rc<RefCell<OpState>>,
) -> Result<ExtensionMessageInner, AnyError> {
    let msg = {
        let msg_rx = state
            .borrow()
            .borrow::<async_channel::Receiver<ExtensionMessage>>()
            .clone();
        match msg_rx.recv().await {
            Ok(msg) => msg,
            Err(e) => {
                ExtensionMessage::new_with_data(String::from("closed"), Some(e.to_string()), None)?
            }
        }
    };
    let ExtensionMessage { inner, callback } = msg;

    state
        .borrow_mut()
        .put::<Option<ExtensionMessageCallback>>(callback);

    Ok(inner)
}

#[op2]
pub(super) fn op_message_callback(
    #[serde] value: serde_json::Value,
    state: &mut OpState,
) -> Result<(), AnyError> {
    let callback = state.take::<Option<ExtensionMessageCallback>>();
    let Some(ExtensionMessageCallback(callback_tx)) = callback else {
        return Ok(());
    };
    callback_tx
        .send(value)
        .map_err(|v| ExtensionError::SendError(anyhow!("{}", v)))?;
    Ok(())
}

#[derive(Clone)]
struct SourceMapStore(Rc<RefCell<HashMap<String, Vec<u8>>>>);

impl SourceMapGetter for SourceMapStore {
    fn get_source_map(&self, specifier: &str) -> Option<Vec<u8>> {
        self.0.borrow().get(specifier).cloned()
    }

    fn get_source_line(&self, _file_name: &str, _line_number: usize) -> Option<String> {
        None
    }
}

struct TypescriptModuleLoader {
    source_maps: SourceMapStore,
}

impl ModuleLoader for TypescriptModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> anyhow::Result<ModuleSpecifier> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let source_maps = self.source_maps.clone();
        fn load(
            source_maps: SourceMapStore,
            module_specifier: &ModuleSpecifier,
        ) -> Result<ModuleSource, error::AnyError> {
            let path = module_specifier
                .to_file_path()
                .map_err(|_| anyhow!("Only file:// URLs are supported."))?;

            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (ModuleType::JavaScript, true),
                MediaType::Json => (ModuleType::Json, false),
                _ => bail!("Unknown extension {:?}", path.extension()),
            };

            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                let res = parsed.transpile(
                    &deno_ast::TranspileOptions::default(),
                    &deno_ast::EmitOptions {
                        source_map: SourceMapOption::Separate,
                        inline_sources: true,
                        ..Default::default()
                    },
                )?;
                let res = res.into_source();
                let source_map = res.source_map.unwrap();
                source_maps
                    .0
                    .borrow_mut()
                    .insert(module_specifier.to_string(), source_map);
                res.source
            } else {
                code.into_bytes()
            };
            Ok(ModuleSource::new(
                module_type,
                ModuleSourceCode::Bytes(code.into_boxed_slice().into()),
                module_specifier,
                None,
            ))
        }

        ModuleLoadResponse::Sync(load(source_maps, module_specifier))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ExtensionError {
    #[error("Failed to parse manifest")]
    ManifestParseError(serde_json::Error),
    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Runtime Error")]
    RuntimeError(anyhow::Error),
    #[error("Error send msg")]
    SendError(anyhow::Error),
    #[error("Error recv msg callback")]
    RecvError(#[from] oneshot::error::RecvError),
    #[error("Elapsed")]
    Elapsed(#[from] tokio::time::error::Elapsed),
}

pub struct ExtensionRuntime {
    js_runtime: JsRuntime,
    msg_tx: async_channel::Sender<ExtensionMessage>,
    source_string: String,
}

impl ExtensionRuntime {
    pub fn new(
        manifest: Manifest,
        source_string: String,
        msg_channel: (
            async_channel::Sender<ExtensionMessage>,
            async_channel::Receiver<ExtensionMessage>,
        ),
        game_api: Option<game_api::GameApiInstance>,
    ) -> Result<Self, ExtensionError> {
        let perms: Box<RuntimePermission> = Box::new(manifest.get_permissions().into());

        let (msg_tx, msg_rx) = msg_channel;
        let apexsky_ext = {
            let mut ext =
                apexsky_extension::init_ops_and_esm(manifest.clone(), perms.clone(), msg_rx);
            ext.middleware_fn = Some({
                let perms = perms.clone();
                Box::new(move |op: OpDecl| -> OpDecl {
                    let on = match op.name {
                        "op_poll_message" => true,
                        "op_message_callback" => true,
                        "op_fetch" => perms.internet,
                        "op_print" => true,
                        "op_read_file" => false,
                        "op_remove_file" => false,
                        "op_set_timeout" => true,
                        "op_write_file" => false,
                        _ => true,
                    };
                    if on {
                        op
                    } else {
                        op.disable()
                    }
                })
            });
            ext
        };

        let mut extensions = vec![apexsky_ext];

        if let Some(game_api) = game_api {
            let mut game_api_ext = apexsky_game_api::init_ops(game_api);
            game_api_ext.middleware_fn = Some({
                let perms = perms.clone();
                Box::new(move |op: OpDecl| -> OpDecl {
                    let on = match op.name {
                        "op_config_get_global_settings" => perms.settings_access,
                        "op_config_update_global_settings" => perms.settings_modify,
                        "op_game_frame_count" => true,
                        "op_game_get_fps" => true,
                        "op_game_get_offsets" => true,
                        "op_game_is_ready" => true,
                        "op_mem_game_baseaddr" => perms.memory_access,
                        "op_mem_read_all" => perms.memory_access,
                        "op_mem_read_f32" => perms.memory_access,
                        "op_mem_read_i32" => perms.memory_access,
                        "op_mem_write_f32" => perms.memory_modify,
                        "op_mem_write_i32" => perms.memory_modify,
                        "op_game_local_player_ptr" => perms.game_world_access,
                        "op_game_view_player_ptr" => perms.game_world_access,
                        "op_game_is_world_ready" => perms.game_world_access,
                        "op_game_cached_player" => perms.game_world_access,
                        _ => true,
                    };
                    if on {
                        op
                    } else {
                        op.disable()
                    }
                })
            });
            extensions.push(game_api_ext);
        }

        let source_map_store = SourceMapStore(Rc::new(RefCell::new(HashMap::new())));

        let js_runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(TypescriptModuleLoader {
                source_maps: source_map_store.clone(),
            })),
            extensions,
            ..Default::default()
        });

        Ok(Self {
            js_runtime,
            msg_tx,
            source_string,
        })
    }

    pub async fn execute(&mut self) -> Result<Option<serde_json::Value>, ExtensionError> {
        self.js_runtime
            .execute_script("<usermod>", self.source_string.clone())
            .map_err(ExtensionError::RuntimeError)?;

        let (callback_tx, callback_rx) = oneshot::channel();

        self.msg_tx
            .send(ExtensionMessage::new(
                String::from("create"),
                Some(callback_tx),
            ))
            .await
            .map_err(|e| ExtensionError::SendError(e.into()))?;

        tokio::select! {
            biased;
            v = callback_rx => {
                let callback_value = v?;
                Ok(Some(callback_value))
            }
            r = tokio::time::timeout(
                Duration::from_secs(1),
                self.js_runtime
                    .run_event_loop(PollEventLoopOptions::default()),
            ) => {
                let finished = r?;
                let callback_value = finished
                    .map_err(ExtensionError::RuntimeError)
                    .map(|()| None)?;
                Ok(callback_value)
            },
        }
    }

    pub async fn run_loop(&mut self) -> Result<(), ExtensionError> {
        self.js_runtime
            .run_event_loop(PollEventLoopOptions {
                wait_for_inspector: true,
                pump_v8_message_loop: true,
            })
            .await
            .map_err(ExtensionError::RuntimeError)?;
        Ok(())
    }

    pub fn get_msg_tx(&self) -> async_channel::Sender<ExtensionMessage> {
        self.msg_tx.clone()
    }
}

#[tokio::test]
async fn test_execute_script() {
    let example_manifest = include_str!("../../extensions/example/manifest.json");
    let example_source = include_str!("../../extensions/example/worker.js");
    let manifest: Manifest =
        Manifest::new(serde_json::from_str(&example_manifest).unwrap()).unwrap();
    let mut instance = ExtensionRuntime::new(
        manifest,
        example_source.to_string(),
        async_channel::unbounded(),
        None,
    )
    .unwrap();
    let callback_value = instance.execute().await.unwrap();
    println!("callback: {:?}", callback_value);
    match tokio::time::timeout(std::time::Duration::from_secs(1), instance.run_loop()).await {
        Ok(r) => r.unwrap(),
        Err(_elapsed) => (),
    }
}
