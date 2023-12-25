extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ItemTrait, Pat, PatIdent,
    PatType, ReturnType, TypePath,
};

fn typed_arg(arg: &FnArg) -> Option<&PatType> {
    match arg {
        FnArg::Typed(t) => Some(t),
        _ => None,
    }
}

fn pat_ident(pat: &PatType) -> Option<&Ident> {
    match &*pat.pat {
        Pat::Ident(PatIdent { ident, .. }) => Some(ident),
        _ => None,
    }
}

fn arg_type_ident(pat: &PatType) -> Option<&Ident> {
    match &*pat.ty {
        syn::Type::Path(TypePath { path, .. }) => {
            let ident = &path.segments[0].ident;
            Some(ident)
        }
        _ => None,
    }
}

fn ret_type_ident(ret: &ReturnType) -> Option<&Ident> {
    let ReturnType::Type(_, path) = ret else {
        return None;
    };
    match path.as_ref() {
        syn::Type::Path(TypePath { path, .. }) => {
            let ident = &path.segments[0].ident;
            Some(ident)
        }
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn skyapex_impl(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let mut methods = proc_macro2::TokenStream::new();

    for i in input.items.iter() {
        if let syn::TraitItem::Fn(m) = i {
            let method_name = &m.sig.ident;
            let arg_list: IndexMap<&Ident, &Ident> = m
                .sig
                .inputs
                .iter()
                .filter_map(|arg_i| {
                    if let Some(arg) = typed_arg(arg_i) {
                        let arg_name = pat_ident(arg).unwrap();
                        let arg_type = arg_type_ident(arg).unwrap();
                        Some((arg_name, arg_type))
                    } else {
                        None
                    }
                })
                .collect();
            let ret_type = ret_type_ident(&m.sig.output);

            let arg_names: Vec<&Ident> = arg_list.keys().cloned().collect();
            let arg_names: Punctuated<&Ident, Comma> = Punctuated::from_iter(arg_names.into_iter());
            // let arg_types: Vec<&Ident> = arg_list.values().cloned().collect();
            // let arg_types: Punctuated<&Ident, Comma> = Punctuated::from_iter(arg_types.into_iter());

            let args_wasmer: TokenStream = arg_list
                .into_iter()
                .map(|(name, typ)| {
                    let value_enum = match typ.to_string().as_str() {
                        "i32" => quote!(Value::I32(#name),),
                        "i64" => quote!(Value::I64(#name),),
                        "f32" => quote!(Value::F32(#name),),
                        "f64" => quote!(Value::F64(#name),),
                        "u128" => quote!(Value::V128(#name),),
                        "i128" => quote!(Value::V128(#name as u128),),
                        _ => panic!("Unsupported type wasmer::Value::?({})", typ),
                    };
                    value_enum
                })
                .collect();

            // let rettype = match ret_type {
            //     Some(t) => quote!(#t),
            //     None => quote!(()),
            // };
            let conv_ret_wasmedge = match ret_type {
                Some(t) => {
                    let conv_fn = Ident::new(&format!("to_{}", t), t.span());
                    quote!([0].#conv_fn())
                }
                None => quote!(;),
            };
            let conv_ret_wasmer = match ret_type {
                Some(t) => {
                    let conv_fn = Ident::new(&format!("unwrap_{}", t), t.span());
                    quote!([0].#conv_fn())
                }
                None => quote!(;),
            };

            let signature = &m.sig;
            methods.extend(quote! {
                #signature {
                    #[cfg(feature = "wasmedge")]
                    {
                        use wasmedge_sdk::{params, WasmVal, WasmValue};
                        self.run_func(stringify!(#method_name), params!(#arg_names)).unwrap()#conv_ret_wasmedge
                    }
                    #[cfg(feature = "wasmer")]
                    {
                        // use wasmer::TypedFunction;
                        // let func = self.instance.exports.get_function(stringify!(#method_name)).unwrap();
                        // let func_typed: TypedFunction<(#arg_types), #rettype> = func.typed(&mut self.store).unwrap();
                        // func_typed.call(&mut self.store, #arg_names).unwrap()
                        use wasmer::Value;
                        self.run_func(stringify!(#method_name), &[#args_wasmer]).unwrap()#conv_ret_wasmer
                    }
                }
            });
        }
    }
    quote! {
        #input
        impl #trait_name for Skyapex {
            #methods
        }
    }
    .into()
}
