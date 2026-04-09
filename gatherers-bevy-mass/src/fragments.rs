// Re-export fragment types from gatherers-sim (the single source of truth).
// In unreal mode, these have both Component and MassFragment derives.
pub use gatherers_sim::fragments::{
    AntTag, FoodTag, BevyMassAntTag,
    Position, Movement, Cooldown, Carrying, Behavior,
    AntEncounterFragment, FoodFragment,
    SimBounds, FoodEncounter,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use unreal_api::mass::MassFragment;

    #[test]
    fn encounter_fragment_layout() {
        assert_eq!(mem::size_of::<AntEncounterFragment>(), 40);
        assert_eq!(mem::offset_of!(AntEncounterFragment, nearest_food_index), 0);
        assert_eq!(mem::offset_of!(AntEncounterFragment, encounter_position), 8);
        assert_eq!(mem::offset_of!(AntEncounterFragment, has_encounter), 32);
    }

    #[test]
    fn encounter_fragment_cpp_type_name() {
        assert_eq!(AntEncounterFragment::CPP_TYPE_NAME, "FGatherersAntEncounterFragment");
    }

    #[test]
    fn food_fragment_layout() {
        assert_eq!(mem::size_of::<FoodFragment>(), 32);
        assert_eq!(mem::offset_of!(FoodFragment, position), 0);
        assert_eq!(mem::offset_of!(FoodFragment, is_loose), 24);
    }

    #[test]
    fn food_fragment_cpp_type_name() {
        assert_eq!(FoodFragment::CPP_TYPE_NAME, "FGatherersMassFoodFragment");
    }

    #[test]
    fn cpp_codegen_produces_valid_output() {
        let regs: Vec<_> = unreal_api::mass::registered_mass_fragments().into_iter().collect();

        let pos_reg = regs.iter().find(|r| r.cpp_type_name == "FGatherersPosition")
            .expect("Position should be registered");
        assert!(!pos_reg.fields.is_empty(), "Position should have field metadata");
        assert_eq!(pos_reg.fields[0].name, "position");
        assert_eq!(pos_reg.fields[0].offset, 0);

        let output = unreal_api::mass::generate_cpp_fragments(&[pos_reg], "Test.h");
        assert!(output.contains("struct FGatherersPosition : public FMassFragment"));
        assert!(output.contains("FVector Position"));
        assert!(output.contains("offsetof(FGatherersPosition, Position) == 0"));
        assert!(output.contains("sizeof(FGatherersPosition) == 48"));
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
            .join("RustPlugin/Source/RustMassGatherers/GatherersFragments.gen.h");

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
