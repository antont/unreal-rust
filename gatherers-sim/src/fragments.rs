use bevy_mass::prelude::{Component, Entity, component};
use bevy_mass::movement::{TransformLike, PrevTranslationLike, DesiredMovementLike};
use bevy_ecs::message::Message;
use glam::DVec3;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------

#[component(cpp_type = "FGatherersMassAntTag")]
pub struct AntTag;

#[component(cpp_type = "FGatherersMassFoodTag")]
pub struct FoodTag;

#[component(cpp_type = "FGatherersBevyMassAntTag", group = "ants")]
pub struct BevyMassAntTag;

// ---------------------------------------------------------------------------
// Fragments
// ---------------------------------------------------------------------------

bevy_mass::mass_fragment!(cpp_type = "FTransformFragment", existing, include = "MassCommonFragments.h",
    /// Transform matching UE's FTransformFragment layout (FQuat + FVector + FVector,
    /// each padded to 32 bytes for SIMD alignment). 96 bytes, align 16.
    #[repr(align(16))]
    pub struct Transform {
        pub rotation: [f64; 4],   // Quaternion XYZW (32 bytes)
        pub translation: DVec3,    // Position XYZ (24 bytes)
        _pad0: f64,                // SIMD padding (8 bytes)
        pub scale: DVec3,          // Scale XYZ (24 bytes)
        _pad1: f64,                // SIMD padding (8 bytes)
    }
);

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            translation: DVec3::ZERO,
            _pad0: 0.0,
            scale: DVec3::ONE,
            _pad1: 0.0,
        }
    }
}

impl Transform {
    /// Create a transform with the given translation, identity rotation, unit scale.
    pub fn from_translation(t: DVec3) -> Self {
        Self {
            translation: t,
            ..Self::default()
        }
    }
}

impl TransformLike for Transform {
    fn translation(&self) -> DVec3 { self.translation }
    fn set_translation(&mut self, v: DVec3) { self.translation = v; }
}

#[component(cpp_type = "FGatherersPreviousTranslation")]
#[derive(Default)]
/// Previous-frame translation, used for spatial sweep queries.
pub struct PreviousTranslation {
    pub value: DVec3,
}

impl PrevTranslationLike for PreviousTranslation {
    fn prev(&self) -> DVec3 { self.value }
    fn set_prev(&mut self, v: DVec3) { self.value = v; }
}

bevy_mass::mass_fragment!(cpp_type = "FMassVelocityFragment", existing, include = "MassMovementFragments.h",
    /// UE's native velocity fragment (internal — written by UE's movement processor).
    /// Game code should use DesiredMovement instead.
    /// In Development builds: 48 bytes (Value + DebugPreviousValue).
    pub struct Velocity {
        pub value: DVec3,          // direction * speed (24 bytes)
        _debug_prev: DVec3,        // DebugPreviousValue padding (24 bytes, Development only)
    }
);

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: DVec3::ZERO,
            _debug_prev: DVec3::ZERO,
        }
    }
}

bevy_mass::mass_fragment!(cpp_type = "FMassDesiredMovementFragment", existing, include = "MassMovementFragments.h",
    /// Desired movement output. Game systems write velocity here;
    /// the engine applies it to position each frame.
    /// Maps to UE's FMassDesiredMovementFragment (80 bytes, align 16).
    /// Layout: FVector(24) + pad(8) + FQuat(32) + f32(4) + pad(12)
    #[repr(align(16))]
    pub struct DesiredMovement {
        pub velocity: DVec3,                    // FVector DesiredVelocity (24 bytes)
        _pad0: f64,                             // alignment padding to 16 for FQuat (8 bytes)
        pub facing: [f64; 4],                   // FQuat DesiredFacing XYZW (32 bytes)
        pub max_speed_override: f32,            // DesiredMaxSpeedOverride (4 bytes)
        _pad1: [u8; 12],                        // struct tail padding to 80 (12 bytes)
    }
);

impl Default for DesiredMovement {
    fn default() -> Self {
        Self {
            velocity: DVec3::new(100.0, 0.0, 0.0), // direction X * speed 100
            _pad0: 0.0,
            facing: [0.0, 0.0, 0.0, 1.0],           // identity quaternion
            max_speed_override: f32::MAX,
            _pad1: [0; 12],
        }
    }
}

impl DesiredMovement {
    /// Create a desired movement from direction and speed.
    pub fn new(direction: DVec3, speed: f32) -> Self {
        let v = if direction.length() > 1e-8 {
            direction.normalize() * speed as f64
        } else {
            DVec3::ZERO
        };
        Self { velocity: v, ..Self::default() }
    }

    /// Get the speed (magnitude of velocity vector).
    pub fn speed(&self) -> f32 {
        self.velocity.length() as f32
    }

    /// Get the normalized direction.
    pub fn direction(&self) -> DVec3 {
        let len = self.velocity.length();
        if len > 1e-8 { self.velocity / len } else { DVec3::ZERO }
    }
}

impl DesiredMovementLike for DesiredMovement {
    fn velocity(&self) -> DVec3 { self.velocity }
}

bevy_mass::mass_tag!(cpp_type = "FMassCodeDrivenMovementTag", existing, include = "MassMovementFragments.h",
    /// Tag indicating entity uses code-driven movement (required by UE's UMassApplyMovementProcessor).
    pub struct CodeDrivenMovementTag;
);

/// Pickup cooldown timer.
/// Pure-Bevy component — added/removed dynamically, not a MassFragment.
/// In Unreal mode, lives on shadow Bevy entities (not in chunk memory).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Cooldown {
    pub remaining_seconds: f32,
}

#[component(cpp_type = "FGatherersCarrying")]
/// Index of carried food item (-1 = not carrying).
pub struct Carrying {
    pub food_index: i32,
}

impl Default for Carrying {
    fn default() -> Self {
        Self { food_index: -1 }
    }
}

#[component(cpp_type = "FGatherersBehavior")]
/// Per-entity behavior tuning (turn jitter, RNG state).
pub struct Behavior {
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

#[component(cpp_type = "FGatherersMassFoodFragment")]
/// Food entity fragment. Position is in FTransformFragment (shared with vis system).
pub struct FoodFragment {
    pub is_loose: bool,
}

impl Default for FoodFragment {
    fn default() -> Self {
        Self {
            is_loose: true,
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
    pub encounter_position: DVec3,
}

// ---------------------------------------------------------------------------
// Food decision codes
// ---------------------------------------------------------------------------

/// Result of ant-food interaction decision.
pub type FoodDecisionCode = i32;
pub const DECISION_NO_ACTION: FoodDecisionCode = 0;
pub const DECISION_PICK_UP: FoodDecisionCode = 1;
pub const DECISION_DROP: FoodDecisionCode = 2;

// ---------------------------------------------------------------------------
// Messages (matching original gatherers HitEvent pattern)
// ---------------------------------------------------------------------------

/// A collision between a hittable entity and a hitter entity.
/// Generic over marker types for type safety (matching original gatherers).
///
/// Carries the hittable's index (not entity) because in Unreal mode food lives
/// in Mass Entity chunks without Bevy entities.
#[derive(Debug, Message)]
pub struct HitEvent<Hittable: 'static, Hitter: 'static> {
    pub hittable_index: i32,
    pub hitter_entity: Entity,
    pub encounter_position: DVec3,
    _phantom: PhantomData<(Hittable, Hitter)>,
}

impl<H: 'static, T: 'static> HitEvent<H, T> {
    pub fn new(hittable_index: i32, hitter_entity: Entity, encounter_position: DVec3) -> Self {
        Self { hittable_index, hitter_entity, encounter_position, _phantom: PhantomData }
    }
}

/// Convenience alias: ant-food collision event.
pub type AntFoodHit = HitEvent<FoodTag, BevyMassAntTag>;

/// Food-side mutation produced by the decision system, consumed by
/// a mode-specific apply system that can access food data.
#[derive(Debug, Message)]
pub struct FoodMutation {
    pub food_index: i32,
    pub decision: FoodDecisionCode,
    pub drop_position: DVec3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    // --- Decomposed component layout tests ---

    #[test]
    fn transform_layout() {
        assert_eq!(mem::size_of::<Transform>(), 96);
        assert_eq!(mem::align_of::<Transform>(), 16);
        assert_eq!(mem::offset_of!(Transform, rotation), 0);
        assert_eq!(mem::offset_of!(Transform, translation), 32);
        assert_eq!(mem::offset_of!(Transform, scale), 64);
    }

    #[test]
    fn previous_translation_layout() {
        assert_eq!(mem::size_of::<PreviousTranslation>(), 24);
        assert_eq!(mem::align_of::<PreviousTranslation>(), 8);
        assert_eq!(mem::offset_of!(PreviousTranslation, value), 0);
    }

    #[test]
    fn velocity_layout() {
        assert_eq!(mem::size_of::<Velocity>(), 48);
        assert_eq!(mem::align_of::<Velocity>(), 8);
        assert_eq!(mem::offset_of!(Velocity, value), 0);
    }

    #[test]
    fn desired_movement_layout() {
        assert_eq!(mem::size_of::<DesiredMovement>(), 80);
        assert_eq!(mem::align_of::<DesiredMovement>(), 16);
        assert_eq!(mem::offset_of!(DesiredMovement, velocity), 0);
        assert_eq!(mem::offset_of!(DesiredMovement, facing), 32);
        assert_eq!(mem::offset_of!(DesiredMovement, max_speed_override), 64);
    }

    #[test]
    fn cooldown_layout() {
        // Pure-Bevy component — no longer needs C++ layout compatibility
        assert_eq!(mem::size_of::<Cooldown>(), 4);
        assert_eq!(mem::align_of::<Cooldown>(), 4);
        assert_eq!(mem::offset_of!(Cooldown, remaining_seconds), 0);
    }

    #[test]
    fn carrying_layout() {
        assert_eq!(mem::size_of::<Carrying>(), 4);
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
    fn transform_default() {
        let t = Transform::default();
        assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(t.translation, DVec3::ZERO);
        assert_eq!(t.scale, DVec3::ONE);
    }

    #[test]
    fn transform_from_translation() {
        let t = Transform::from_translation(DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.translation, DVec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(t.scale, DVec3::ONE);
    }

    #[test]
    fn previous_translation_default() {
        let p = PreviousTranslation::default();
        assert_eq!(p.value, DVec3::ZERO);
    }

    #[test]
    fn velocity_default() {
        let v = Velocity::default();
        assert_eq!(v.value, DVec3::ZERO);
    }

    #[test]
    fn desired_movement_default() {
        let dm = DesiredMovement::default();
        assert_eq!(dm.velocity, DVec3::new(100.0, 0.0, 0.0));
        assert!((dm.speed() - 100.0).abs() < 1e-4);
        assert!((dm.direction() - DVec3::X).length() < 1e-8);
        assert_eq!(dm.facing, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(dm.max_speed_override, f32::MAX);
    }

    #[test]
    fn desired_movement_new() {
        let dm = DesiredMovement::new(DVec3::new(0.0, 1.0, 0.0), 50.0);
        assert!((dm.speed() - 50.0).abs() < 1e-4);
        assert!((dm.direction() - DVec3::Y).length() < 1e-8);
        assert!((dm.velocity - DVec3::new(0.0, 50.0, 0.0)).length() < 1e-6);
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
    fn food_fragment_layout() {
        assert_eq!(mem::size_of::<FoodFragment>(), 1);
        assert_eq!(mem::offset_of!(FoodFragment, is_loose), 0);
    }

    #[test]
    fn food_encounter_layout() {
        assert_eq!(mem::size_of::<FoodEncounter>(), 32);
        assert_eq!(mem::offset_of!(FoodEncounter, food_index), 0);
        // repr(C) inserts 4 bytes of implicit padding between i32 and DVec3 (align 8)
        assert_eq!(mem::offset_of!(FoodEncounter, encounter_position), 8);
    }
}
