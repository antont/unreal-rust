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
    pub carried_food_handle: [i32; 2],
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
            carried_food_handle: [0; 2],
            pickup_cooldown_remaining_seconds: 0.0,
            movement_speed: 100.0,
            turn_jitter_radians: std::f32::consts::FRAC_PI_2,
            random_seed: 0,
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
        assert_eq!(mem::offset_of!(AntFragment, carried_food_handle), 72);
        assert_eq!(mem::offset_of!(AntFragment, pickup_cooldown_remaining_seconds), 80);
        assert_eq!(mem::offset_of!(AntFragment, movement_speed), 84);
        assert_eq!(mem::offset_of!(AntFragment, turn_jitter_radians), 88);
        assert_eq!(mem::offset_of!(AntFragment, random_seed), 92);
    }
}
