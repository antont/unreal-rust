//! Generates C++ fragment headers from #[derive(MassFragment)] Rust structs.
//!
//! Usage: cargo run -p gatherers-bevy-mass --bin gen-fragments [output-path]
//!
//! Default output: ../RustPlugin/Source/RustMassGatherers/GeneratedFragments.h

// Pull in the library so inventory collects all registrations.
use gatherers_bevy_mass as _;

use std::path::PathBuf;
use unreal_api::mass::{generate_cpp_fragments, registered_mass_fragments};

fn main() {
    let regs: Vec<&_> = registered_mass_fragments().into_iter().collect();

    // Sort: tags first (alphabetically), then fragments (alphabetically)
    let mut tags: Vec<_> = regs.iter().copied().filter(|r| r.is_tag).collect::<Vec<_>>();
    let mut fragments: Vec<_> = regs.iter().copied().filter(|r| !r.is_tag).collect::<Vec<_>>();
    tags.sort_by_key(|r| r.cpp_type_name);
    fragments.sort_by_key(|r| r.cpp_type_name);

    let mut all = tags;
    all.extend(fragments);

    let output = generate_cpp_fragments(&all);

    let out_path = std::env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir
            .parent()
            .unwrap()
            .join("RustPlugin/Source/RustMassGatherers/GeneratedFragments.h")
    });

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    std::fs::write(&out_path, &output).expect("Failed to write output file");
    println!("Wrote {}", out_path.display());
}
