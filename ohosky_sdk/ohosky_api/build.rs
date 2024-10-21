fn main() {
    #[cfg(feature = "cxx")]
    let _ = cxx_build::bridges(vec!["src/cxx/dmalib.rs", "src/cxx/host.rs"]).std("c++20");
}
