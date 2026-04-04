use unreal_api::MassFragment;

/// Matches C++ FGatherersMassAntFragment layout (96 bytes, align 8).
/// FMassFragment base is empty (EBO), fields start at offset 0.
#[derive(MassFragment, Clone, Copy, Debug)]
#[repr(C)]
#[mass(cpp_type = "FGatherersMassAntFragment")]
pub struct AntFragment {
    pub position: [f64; 3],
    pub previous_position: [f64; 3],
    pub direction: [f64; 3],
    pub carried_food_index: i32,
    pub _carried_pad: i32,
    pub pickup_cooldown_remaining_seconds: f32,
    pub movement_speed: f32,
    pub turn_jitter_radians: f32,
    pub random_seed: i32,
}

impl Default for AntFragment {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            previous_position: [0.0; 3],
            direction: [1.0, 0.0, 0.0],
            carried_food_index: -1,
            _carried_pad: 0,
            pickup_cooldown_remaining_seconds: 0.0,
            movement_speed: 100.0,
            turn_jitter_radians: std::f32::consts::FRAC_PI_2,
            random_seed: 0,
        }
    }
}

/// Matches C++ FGatherersAntEncounterFragment layout.
/// Written by C++ collision pre-pass, read by Rust food decision system.
#[derive(MassFragment, Clone, Copy, Debug)]
#[repr(C)]
#[mass(cpp_type = "FGatherersAntEncounterFragment")]
pub struct AntEncounterFragment {
    /// Index into the food entities array, or -1 if none.
    pub nearest_food_index: i32,
    pub _nearest_pad: i32,
    /// Position where the encounter occurred.
    pub encounter_position: [f64; 3],
    /// Whether an encounter was detected this frame.
    pub has_encounter: bool,
    pub _pad: [u8; 7],
}

impl Default for AntEncounterFragment {
    fn default() -> Self {
        Self {
            nearest_food_index: -1,
            _nearest_pad: 0,
            encounter_position: [0.0; 3],
            has_encounter: false,
            _pad: [0; 7],
        }
    }
}

/// Matches C++ FGatherersMassFoodFragment layout (32 bytes, align 8).
#[derive(MassFragment, Clone, Copy, Debug)]
#[repr(C)]
#[mass(cpp_type = "FGatherersMassFoodFragment")]
pub struct FoodFragment {
    pub position: [f64; 3],
    pub is_loose: bool,
    pub _pad: [u8; 7],
}

impl Default for FoodFragment {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            is_loose: true,
            _pad: [0; 7],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use unreal_api::mass::MassFragment;

    #[test]
    fn ant_fragment_size() {
        assert_eq!(mem::size_of::<AntFragment>(), 96);
    }

    #[test]
    fn ant_fragment_align() {
        assert_eq!(mem::align_of::<AntFragment>(), 8);
    }

    #[test]
    fn ant_fragment_cpp_type_name() {
        assert_eq!(AntFragment::CPP_TYPE_NAME, "FGatherersMassAntFragment");
    }

    #[test]
    fn ant_fragment_field_offsets() {
        assert_eq!(mem::offset_of!(AntFragment, position), 0);
        assert_eq!(mem::offset_of!(AntFragment, previous_position), 24);
        assert_eq!(mem::offset_of!(AntFragment, direction), 48);
        assert_eq!(mem::offset_of!(AntFragment, carried_food_index), 72);
        assert_eq!(mem::offset_of!(AntFragment, pickup_cooldown_remaining_seconds), 80);
        assert_eq!(mem::offset_of!(AntFragment, movement_speed), 84);
        assert_eq!(mem::offset_of!(AntFragment, turn_jitter_radians), 88);
        assert_eq!(mem::offset_of!(AntFragment, random_seed), 92);
    }

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
        // Use registered fragment metadata to generate C++ and verify key patterns
        let regs: Vec<_> = unreal_api::mass::registered_mass_fragments().into_iter().collect();

        // Find ant fragment registration
        let ant_reg = regs.iter().find(|r| r.cpp_type_name == "FGatherersMassAntFragment")
            .expect("AntFragment should be registered");
        assert!(!ant_reg.fields.is_empty(), "AntFragment should have field metadata");
        assert_eq!(ant_reg.fields[0].name, "position");
        assert_eq!(ant_reg.fields[0].offset, 0);

        let output = unreal_api::mass::generate_cpp_fragments(&[ant_reg]);
        assert!(output.contains("struct FGatherersMassAntFragment : public FMassFragment"));
        assert!(output.contains("FVector Position"));
        assert!(output.contains("int32 CarriedFoodIndex"));
        assert!(output.contains("float MovementSpeed"));
        assert!(output.contains("offsetof(FGatherersMassAntFragment, Position) == 0"));
        assert!(output.contains("sizeof(FGatherersMassAntFragment) == 96"));
    }
}
