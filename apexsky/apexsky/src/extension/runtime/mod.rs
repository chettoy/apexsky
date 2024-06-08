use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, bail, Error};
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_core::*;

mod ops;
use ops::*;

use super::manifest::Manifest;

extension!(
  apexsky_extension,
  ops = [
    op_read_file,
    op_write_file,
    op_remove_file,
    op_fetch,
    op_set_timeout
  ],
  esm_entry_point = "ext:apexsky_extension/runtime.js",
  esm = [ dir "src/extension/runtime", "runtime.js" ],
  options = {
    manifest: Manifest,
  },
  middleware = middleware_fn,
  state = |state, options| {
    state.put::<Manifest>(options.manifest);
  },
  docs = "apexsky runtime op2.",
);

fn middleware_fn(op: OpDecl) -> OpDecl {
    match op.name {
        "op_fetch" => op.disable(),
        "op_print" => op,
        "op_read_file" => op.disable(),
        "op_remove_file" => op.disable(),
        "op_set_timeout" => op.disable(),
        "op_write_file" => op.disable(),
        _ => op,
    }
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
    ) -> Result<ModuleSpecifier, Error> {
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

#[test]
fn execute_script() {
    let example_manifest = include_str!("../../../resource/extensions/example/manifest.json");
    let manifest: Manifest = serde_json::from_str(&example_manifest).unwrap();

    let source_map_store = SourceMapStore(Rc::new(RefCell::new(HashMap::new())));

    // Initialize a runtime instance
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(TypescriptModuleLoader {
            source_maps: source_map_store.clone(),
        })),
        extensions: vec![apexsky_extension::init_ops_and_esm(manifest)],
        ..Default::default()
    });

    // Now we see how to invoke the op we just defined. The runtime automatically
    // contains a Deno.core object with several functions for interacting with it.
    // You can find its definition in core.js.
    js_runtime
        .execute_script(
            "<usage>",
            r#"
  function print(value) {
    console.log(value.toString()+"\n");
  }
  
  const arr = [1, 2, 3];
  print("The sum of");
  print(arr);
  print("is");
  //print(Deno.core.ops.op_sum(arr));
  
  // And incorrect usage
  try {
    print(Deno.core.ops.op_sum(0));
  } catch(e) {
    print('Exception:');
    print(e);
  }
  "#,
        )
        .unwrap();
}
