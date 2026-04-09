//! Generates C++ fragment headers from #[derive(MassFragment)] Rust structs.
//!
//! Usage: cargo run -p gatherers-bevy-mass --bin gen-fragments [output-path]
//!
//! Default output: ../RustPlugin/Source/RustMassGatherers/GatherersFragments.gen.h

// Pull in the library so inventory collects all registrations.
use gatherers_bevy_mass as _;

fn main() {
    let default = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("RustPlugin/Source/RustMassGatherers/GatherersFragments.gen.h");
    unreal_api::mass::run_fragment_codegen(&default);
}
