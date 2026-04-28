// Re-export component types from gatherers-sim (the single source of truth).
// In unreal mode, these have both Component and MassFragment derives.
pub use gatherers_sim::components::{
    Food, Ant, SimpleMovementTag,
    Transform, PreviousTranslation, Velocity, Cooldown, Carrying, Behavior,
    FoodState,
    SimBounds, FoodEncounter,
    AntFoodHit, FoodMutation,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use unreal_api::mass::MassFragment;

    #[test]
    fn food_fragment_layout() {
        assert_eq!(mem::size_of::<FoodState>(), 1);
        assert_eq!(mem::offset_of!(FoodState, is_loose), 0);
    }

    #[test]
    fn food_fragment_cpp_type_name() {
        assert_eq!(FoodState::CPP_TYPE_NAME, "FGatherersFoodStateFragment");
    }

    #[test]
    fn cpp_codegen_produces_valid_output() {
        let regs: Vec<_> = unreal_api::mass::registered_mass_fragments().into_iter().collect();

        let vel_reg = regs.iter().find(|r| r.cpp_type_name == "FMassVelocityFragment")
            .expect("Velocity should be registered as FMassVelocityFragment");
        assert!(vel_reg.existing, "Velocity should be marked as existing");
        assert_eq!(vel_reg.size, 48);

        let output = unreal_api::mass::generate_cpp_fragments(&[vel_reg], "Test.h");
        assert!(!output.contains("USTRUCT"), "existing type should not generate USTRUCT");
        assert!(output.contains("sizeof(FMassVelocityFragment) == 48"), "should verify size");
        assert!(output.contains("#include \"MassMovementFragments.h\""), "should include header");
    }

    #[test]
    fn transform_registered_as_existing() {
        let regs: Vec<_> = unreal_api::mass::registered_mass_fragments().into_iter().collect();

        let transform_reg = regs.iter().find(|r| r.cpp_type_name == "FTransformFragment")
            .expect("Transform should be registered");
        assert!(transform_reg.existing, "Transform should be marked as existing");
        assert_eq!(transform_reg.size, 96);

        let output = unreal_api::mass::generate_cpp_fragments(&[transform_reg], "Test.h");
        assert!(!output.contains("USTRUCT"), "existing type should not generate USTRUCT");
        assert!(output.contains("sizeof(FTransformFragment) == 96"), "should verify size");
    }

    #[test]
    fn codegen_matches_golden_file() {
        use unreal_api::mass::{generate_cpp_fragments, registered_mass_fragments};

        let regs: Vec<&_> = registered_mass_fragments().into_iter().collect();

        let mut tags: Vec<_> = regs.iter().copied().filter(|r| r.is_tag).collect::<Vec<_>>();
        let mut fragments: Vec<_> = regs.iter().copied().filter(|r| !r.is_tag).collect::<Vec<_>>();
        tags.sort_by_key(|r| r.cpp_type_name);
        fragments.sort_by_key(|r| r.cpp_type_name);

        let mut all = tags;
        all.extend(fragments);

        let golden_filename = "GatherersFragments.gen.h";
        let generated = generate_cpp_fragments(&all, golden_filename);

        let golden_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("RustPlugin/Source/RustPlugin/Generated/GatherersFragments.gen.h");

        let golden = std::fs::read_to_string(&golden_path).unwrap_or_else(|e| {
            panic!(
                "Failed to read golden file at {}: {}\n\
                 Run `cargo run -p gatherers-bevy-mass --bin gen-fragments` to generate it.",
                golden_path.display(),
                e
            )
        });

        if generated != golden {
            let gen_lines: Vec<_> = generated.lines().collect();
            let gold_lines: Vec<_> = golden.lines().collect();
            for (i, (g, go)) in gen_lines.iter().zip(gold_lines.iter()).enumerate() {
                if g != go {
                    panic!(
                        "GeneratedFragments.h is out of date (first diff at line {}):\n\
                         - golden: {}\n\
                         + generated: {}\n\n\
                         Run `cargo run -p gatherers-bevy-mass --bin gen-fragments` to update.",
                        i + 1, go, g
                    );
                }
            }
            if gen_lines.len() != gold_lines.len() {
                panic!(
                    "GeneratedFragments.h has {} lines but generated has {} lines.\n\
                     Run `cargo run -p gatherers-bevy-mass --bin gen-fragments` to update.",
                    gold_lines.len(),
                    gen_lines.len()
                );
            }
        }
    }
}
