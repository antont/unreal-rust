use std::env;
use std::path::PathBuf;

fn main() {
    // OUT_DIR is <target>/<profile>/build/<crate-hash>/out — walk up to the
    // cargo target dir so the loader can find the sibling host dylib at runtime
    // without relying on UNREAL_RUST_TARGET_DIR or a hardcoded workspace layout.
    // This handles user-configured shared target dirs (~/.cargo/config.toml).
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set by cargo"));
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR should have at least 3 ancestors")
        .to_path_buf();

    println!(
        "cargo:rustc-env=UNREAL_RUST_LOADER_BUILD_TARGET_DIR={}",
        profile_dir.display()
    );
}
