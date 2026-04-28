//! Generates C++ fragment headers from `#[derive(MassFragment)]` Rust
//! structs declared by `vivarium-sim`.
//!
//! Usage: cargo run -p vivarium-sim --features unreal --bin gen-fragments
//!
//! Default output:
//!   ../RustPlugin/Source/RustPlugin/Generated/VivariumFragments.gen.h

// Pull in the library so inventory collects every MassFragment registration.
use vivarium_sim as _;

fn main() {
    let default = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("RustPlugin/Source/RustPlugin/Generated/VivariumFragments.gen.h");
    unreal_api::mass::run_fragment_codegen(&default);
}
