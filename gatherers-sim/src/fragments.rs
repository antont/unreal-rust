/// Matches C++ FGatherersMassAntFragment layout (96 bytes, align 8).
/// FMassFragment base is empty (EBO), fields start at offset 0.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntFragment {
    pub position: [f64; 3],            // FVector Position (offset 0)
    pub previous_position: [f64; 3],   // FVector PreviousPosition (offset 24)
    pub direction: [f64; 3],           // FVector Direction (offset 48)
    pub carried_food_index: i32,       // index into food array, -1 if not carrying (offset 72)
    pub _carried_pad: i32,             // padding to keep layout (offset 76)
    pub pickup_cooldown_remaining_seconds: f32, // (offset 80)
    pub movement_speed: f32,           // (offset 84)
    pub turn_jitter_radians: f32,      // (offset 88)
    pub random_seed: i32,              // (offset 92)
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

/// Matches C++ FGatherersMassFoodFragment layout.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FoodFragment {
    pub position: [f64; 3],
    pub is_loose: bool,
}

impl Default for FoodFragment {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            is_loose: true,
        }
    }
}

/// Matches C++ FGatherersMassFoodEncounter layout.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FoodEncounter {
    pub food_index: i32,
    pub _pad: i32,
    pub encounter_position: [f64; 3],
}

/// Simulation bounds (min/max corners).
#[derive(Clone, Copy, Debug)]
pub struct SimBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn ant_fragment_size() {
        assert_eq!(mem::size_of::<AntFragment>(), 96);
    }

    #[test]
    fn ant_fragment_align() {
        assert_eq!(mem::align_of::<AntFragment>(), 8);
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
    fn food_fragment_size_and_offsets() {
        assert_eq!(mem::offset_of!(FoodFragment, position), 0);
        assert_eq!(mem::offset_of!(FoodFragment, is_loose), 24);
        // Size may be 25 + padding to align 8 = 32
        assert_eq!(mem::size_of::<FoodFragment>(), 32);
    }

    #[test]
    fn food_encounter_size_and_offsets() {
        assert_eq!(mem::offset_of!(FoodEncounter, food_index), 0);
        assert_eq!(mem::offset_of!(FoodEncounter, encounter_position), 8);
        assert_eq!(mem::size_of::<FoodEncounter>(), 32);
    }

    #[test]
    fn ant_fragment_default() {
        let frag = AntFragment::default();
        assert_eq!(frag.position, [0.0; 3]);
        assert_eq!(frag.direction, [1.0, 0.0, 0.0]);
        assert_eq!(frag.movement_speed, 100.0);
        assert_eq!(frag.carried_food_index, -1);
    }
}
