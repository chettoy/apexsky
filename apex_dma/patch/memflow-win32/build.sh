patch < ../patch/memflow-win32/0001-Fix-unknown-feature-proc_macro_span_shrink.patch
cargo build --release --all-features
patch -R < ../patch/memflow-win32/0001-Fix-unknown-feature-proc_macro_span_shrink.patch