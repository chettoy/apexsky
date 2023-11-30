patch < ../patch/memflow/0002-Fix-unknown-feature-proc_macro_span_shrink.patch
cargo build --release
patch -R < ../patch/memflow/0002-Fix-unknown-feature-proc_macro_span_shrink.patch
