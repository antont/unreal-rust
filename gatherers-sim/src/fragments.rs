use bevy_mass::prelude::Component;
use glam::DVec3;

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersMassAntTag", tag))]
#[derive(Component, Clone, Copy, Debug)]
pub struct AntTag;

#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersMassFoodTag", tag))]
#[derive(Component, Clone, Copy, Debug)]
pub struct FoodTag;

#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersBevyMassAntTag", tag))]
#[derive(Component, Clone, Copy, Debug)]
pub struct BevyMassAntTag;

// ---------------------------------------------------------------------------
// Fragments
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Decomposed components (shared across entity types)
// ---------------------------------------------------------------------------

/// World-space position and previous-frame position.
/// Matches C++ FGatherersPosition (48 bytes, align 8).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersPosition"))]
pub struct Position {
    pub position: DVec3,
    pub previous_position: DVec3,
}

/// Movement direction and speed.
/// Matches C++ FGatherersMovement (32 bytes, align 8).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersMovement"))]
pub struct Movement {
    #[cfg_attr(feature = "unreal", mass(default = "FVector(1.0f, 0.0f, 0.0f)"))]
    pub direction: DVec3,
    #[cfg_attr(feature = "unreal", mass(default = "100.0f"))]
    pub movement_speed: f32,
    pub _pad: [u8; 4],
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            direction: DVec3::X,
            movement_speed: 100.0,
            _pad: [0; 4],
        }
    }
}

/// Pickup cooldown timer.
/// Matches C++ FGatherersCooldown (8 bytes, align 4).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug, Default)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersCooldown"))]
pub struct Cooldown {
    pub remaining_seconds: f32,
    pub _pad: [u8; 4],
}

/// Index of carried food item (-1 = not carrying).
/// Matches C++ FGatherersCarrying (8 bytes, align 4).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersCarrying"))]
pub struct Carrying {
    #[cfg_attr(feature = "unreal", mass(default = "-1"))]
    pub food_index: i32,
    pub _pad: i32,
}

impl Default for Carrying {
    fn default() -> Self {
        Self { food_index: -1, _pad: 0 }
    }
}

/// Per-entity behavior tuning (turn jitter, RNG state).
/// Matches C++ FGatherersBehavior (8 bytes, align 4).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersBehavior"))]
pub struct Behavior {
    #[cfg_attr(feature = "unreal", mass(default = "PI / 2.0f"))]
    pub turn_jitter_radians: f32,
    pub random_seed: i32,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            turn_jitter_radians: std::f32::consts::FRAC_PI_2,
            random_seed: 0,
        }
    }
}

/// Matches C++ FGatherersAntEncounterFragment layout.
/// Written by C++ collision pre-pass, read by Rust food decision system.
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersAntEncounterFragment"))]
pub struct AntEncounterFragment {
    /// Index into the food entities array, or -1 if none.
    #[cfg_attr(feature = "unreal", mass(default = "-1"))]
    pub nearest_food_index: i32,
    pub _nearest_pad: i32,
    /// Position where the encounter occurred.
    pub encounter_position: DVec3,
    /// Whether an encounter was detected this frame.
    pub has_encounter: bool,
    pub _pad: [u8; 7],
}

impl Default for AntEncounterFragment {
    fn default() -> Self {
        Self {
            nearest_food_index: -1,
            _nearest_pad: 0,
            encounter_position: DVec3::ZERO,
            has_encounter: false,
            _pad: [0; 7],
        }
    }
}

/// Matches C++ FGatherersMassFoodFragment layout (32 bytes, align 8).
#[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
#[cfg_attr(feature = "unreal", mass(cpp_type = "FGatherersMassFoodFragment"))]
pub struct FoodFragment {
    pub position: DVec3,
    #[cfg_attr(feature = "unreal", mass(default = "true"))]
    pub is_loose: bool,
    pub _pad: [u8; 7],
}

impl Default for FoodFragment {
    fn default() -> Self {
        Self {
            position: DVec3::ZERO,
            is_loose: true,
            _pad: [0; 7],
        }
    }
}

/// Simulation bounds (min/max corners).
#[derive(Clone, Copy, Debug)]
pub struct SimBounds {
    pub min: DVec3,
    pub max: DVec3,
}

/// Result of a food encounter query — used by food_decision and FFI.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FoodEncounter {
    pub food_index: i32,
    pub _pad: i32,
    pub encounter_position: DVec3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // --- Decomposed component layout tests ---

    #[test]
    fn position_layout() {
        assert_eq!(mem::size_of::<Position>(), 48);
        assert_eq!(mem::align_of::<Position>(), 8);
        assert_eq!(mem::offset_of!(Position, position), 0);
        assert_eq!(mem::offset_of!(Position, previous_position), 24);
    }

    #[test]
    fn movement_layout() {
        assert_eq!(mem::size_of::<Movement>(), 32);
        assert_eq!(mem::align_of::<Movement>(), 8);
        assert_eq!(mem::offset_of!(Movement, direction), 0);
        assert_eq!(mem::offset_of!(Movement, movement_speed), 24);
    }

    #[test]
    fn cooldown_layout() {
        assert_eq!(mem::size_of::<Cooldown>(), 8);
        assert_eq!(mem::align_of::<Cooldown>(), 4);
        assert_eq!(mem::offset_of!(Cooldown, remaining_seconds), 0);
    }

    #[test]
    fn carrying_layout() {
        assert_eq!(mem::size_of::<Carrying>(), 8);
        assert_eq!(mem::align_of::<Carrying>(), 4);
        assert_eq!(mem::offset_of!(Carrying, food_index), 0);
    }

    #[test]
    fn behavior_layout() {
        assert_eq!(mem::size_of::<Behavior>(), 8);
        assert_eq!(mem::align_of::<Behavior>(), 4);
        assert_eq!(mem::offset_of!(Behavior, turn_jitter_radians), 0);
        assert_eq!(mem::offset_of!(Behavior, random_seed), 4);
    }

    #[test]
    fn position_default() {
        let p = Position::default();
        assert_eq!(p.position, DVec3::ZERO);
        assert_eq!(p.previous_position, DVec3::ZERO);
    }

    #[test]
    fn movement_default() {
        let m = Movement::default();
        assert_eq!(m.direction, DVec3::X);
        assert_eq!(m.movement_speed, 100.0);
    }

    #[test]
    fn carrying_default() {
        let c = Carrying::default();
        assert_eq!(c.food_index, -1);
    }

    #[test]
    fn behavior_default() {
        let b = Behavior::default();
        assert!((b.turn_jitter_radians - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
        assert_eq!(b.random_seed, 0);
    }

    #[test]
    fn encounter_fragment_layout() {
        assert_eq!(mem::size_of::<AntEncounterFragment>(), 40);
        assert_eq!(mem::offset_of!(AntEncounterFragment, nearest_food_index), 0);
        assert_eq!(mem::offset_of!(AntEncounterFragment, encounter_position), 8);
        assert_eq!(mem::offset_of!(AntEncounterFragment, has_encounter), 32);
    }

    #[test]
    fn food_fragment_layout() {
        assert_eq!(mem::size_of::<FoodFragment>(), 32);
        assert_eq!(mem::offset_of!(FoodFragment, position), 0);
        assert_eq!(mem::offset_of!(FoodFragment, is_loose), 24);
    }

    #[test]
    fn food_encounter_layout() {
        assert_eq!(mem::offset_of!(FoodEncounter, food_index), 0);
        assert_eq!(mem::offset_of!(FoodEncounter, encounter_position), 8);
        assert_eq!(mem::size_of::<FoodEncounter>(), 32);
    }

}
