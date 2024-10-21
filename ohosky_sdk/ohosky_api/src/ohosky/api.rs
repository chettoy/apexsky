pub mod bindings {
    use wit_bindgen::generate;
    generate!({
        world: "ohosky",
        path: "./wit",
        pub_export_macro: true,
        export_macro_name: "export",
        additional_derives: [PartialEq, Eq, Hash, Clone],
    });
}
