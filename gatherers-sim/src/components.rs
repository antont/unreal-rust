use bevy_mass::prelude::{Component, Entity, DVec3, Resource, MassFragment};
use bevy_mass::movement::PrevTranslationLike;
use bevy_ecs::message::Message;
use std::marker::PhantomData;

// Re-export engine types from framework so downstream code can import from here
pub use bevy_mass::components::{Transform, Velocity, DesiredMovement, CodeDrivenMovementTag};

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------

#[derive(Component, MassFragment, Clone, Copy, Debug)]
#[cfg_attr(feature = "unreal", mass(group = "food"))]
pub struct Food;

#[derive(Component, MassFragment, Clone, Copy, Debug)]
#[cfg_attr(feature = "unreal", mass(group = "ants"))]
pub struct Ant;

// ---------------------------------------------------------------------------
// Data components
// ---------------------------------------------------------------------------

/// Previous-frame translation, used for spatial sweep queries.
#[repr(C)]
#[derive(Component, MassFragment, Clone, Copy, Debug, Default)]
pub struct PreviousTranslation {
    pub value: DVec3,
}

impl PrevTranslationLike for PreviousTranslation {
    fn prev(&self) -> DVec3 { self.value }
    fn set_prev(&mut self, v: DVec3) { self.value = v; }
}

/// Pickup cooldown timer.
/// Pure-Bevy component — added/removed dynamically, not a MassFragment.
/// In Unreal mode, lives on shadow Bevy entities (not in chunk memory).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Cooldown {
    pub remaining_seconds: f32,
}

/// The food entity currently carried (if any). Pure-Bevy shadow component —
/// lives on shadow entities in UE mode, not in chunk memory. C++ has no
/// awareness of `Carrying`; all carrying logic runs Rust-side, and the
/// food-follow-ant positioning in `carried_food_tracking` reads this
/// component directly.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Carrying(pub Option<Entity>);

impl Carrying {
    /// Idiomatic check — reads better than pattern-matching on the field.
    pub fn is_carrying(&self) -> bool {
        self.0.is_some()
    }

    /// The currently-carried food `Entity`, if any.
    pub fn entity(&self) -> Option<Entity> {
        self.0
    }
}

/// Per-entity behavior tuning (turn jitter, RNG state).
#[repr(C)]
#[derive(Component, MassFragment, Clone, Copy, Debug)]
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

/// Food entity fragment. Position is in FTransformFragment (shared with vis system).
#[repr(C)]
#[derive(Component, MassFragment, Clone, Copy, Debug)]
pub struct FoodState {
    pub is_loose: bool,
}

impl Default for FoodState {
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
/// Carries both the hittable's `Entity` (for idiomatic Bevy-style lookup)
/// and its chunk-slot `i32` index (still needed to construct
/// `FoodEncounter`, whose `food_index` field is `#[repr(C)]` and fed to
/// the pure decision function). In Unreal mode the Entity is a shadow
/// Bevy entity resolved via `MassEntityMap` at the spatial-query boundary.
#[derive(Debug, Clone, Message)]
pub struct HitEvent<Hittable: 'static, Hitter: 'static> {
    pub hittable_index: i32,
    pub hittable_entity: Entity,
    pub hitter_entity: Entity,
    pub encounter_position: DVec3,
    _phantom: PhantomData<(Hittable, Hitter)>,
}

impl<H: 'static, T: 'static> HitEvent<H, T> {
    pub fn new(
        hittable_index: i32,
        hittable_entity: Entity,
        hitter_entity: Entity,
        encounter_position: DVec3,
    ) -> Self {
        Self {
            hittable_index,
            hittable_entity,
            hitter_entity,
            encounter_position,
            _phantom: PhantomData,
        }
    }
}

/// Convenience alias: ant-food collision event.
pub type AntFoodHit = HitEvent<Food, Ant>;

/// Food-side mutation produced by the decision system, consumed by
/// a mode-specific apply system that can access food data.
///
/// Carries `food_entity` (idiomatic lookup). The apply system resolves
/// this to a chunk-slot index via `Res<EntityIndex<Food>>` for the FFI
/// `FoodDropEvents` / `FoodPickupEvents` payloads that C++ consumes.
#[derive(Debug, Clone, Message)]
pub struct FoodMutation {
    pub food_entity: Entity,
    pub decision: FoodDecisionCode,
    pub drop_position: DVec3,
}

// ---------------------------------------------------------------------------
// Food drop events (Bevy Resource, consumed by C++ via FFI).
//
// `food_index` is an instance index into the **single** GridHash-owned group
// on the C++ side — there is no group identifier in the event payload. C++
// enforces the one-owner constraint (see URustMassBevySubsystem::
// TryMarkGridHashOwner): if two `#[mass_system_config]` entries tried to
// register GridHash queries for different groups, the second claim is
// refused and logged as an error.
//
// Extending to multiple groups requires an FFI change — adding a group
// identifier (e.g. a stable group-id u16, resolved from a `GroupName`
// interned on both sides) to `FoodDropEntry` + the matching C++ consumer
// in URustMassBevySubsystem::ApplyFoodEvents.
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct FoodDropEvents {
    pub events: Vec<FoodDropEntry>,
}

pub struct FoodDropEntry {
    pub food_index: i32,
    pub position: DVec3,
}

impl FoodDropEvents {
    pub fn push(&mut self, food_index: i32, position: DVec3) {
        self.events.push(FoodDropEntry { food_index, position });
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

// ---------------------------------------------------------------------------
// Food pickup events (Bevy Resource, consumed by C++ via FFI).
//
// C++ uses these to remove the picked-up food from the navigation hash grid
// so the GridHash spatial query doesn't have to filter `is_loose` per candidate.
//
// `indices` are instance indices into the **single** GridHash-owned group
// on the C++ side — same constraint as FoodDropEvents above. See that type's
// doc comment for the enforcement path and the FFI change needed to lift it.
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct FoodPickupEvents {
    pub indices: Vec<i32>,
}

impl FoodPickupEvents {
    pub fn push(&mut self, food_index: i32) {
        self.indices.push(food_index);
    }

    pub fn clear(&mut self) {
        self.indices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn previous_translation_layout() {
        assert_eq!(mem::size_of::<PreviousTranslation>(), 24);
        assert_eq!(mem::align_of::<PreviousTranslation>(), 8);
        assert_eq!(mem::offset_of!(PreviousTranslation, value), 0);
    }

    #[test]
    fn cooldown_layout() {
        assert_eq!(mem::size_of::<Cooldown>(), 4);
        assert_eq!(mem::align_of::<Cooldown>(), 4);
        assert_eq!(mem::offset_of!(Cooldown, remaining_seconds), 0);
    }

    #[test]
    fn behavior_layout() {
        assert_eq!(mem::size_of::<Behavior>(), 8);
        assert_eq!(mem::align_of::<Behavior>(), 4);
        assert_eq!(mem::offset_of!(Behavior, turn_jitter_radians), 0);
        assert_eq!(mem::offset_of!(Behavior, random_seed), 4);
    }

    #[test]
    fn previous_translation_default() {
        let p = PreviousTranslation::default();
        assert_eq!(p.value, DVec3::ZERO);
    }

    #[test]
    fn carrying_default() {
        let c = Carrying::default();
        assert!(!c.is_carrying());
        assert_eq!(c.entity(), None);
    }

    #[test]
    fn behavior_default() {
        let b = Behavior::default();
        assert!((b.turn_jitter_radians - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
        assert_eq!(b.random_seed, 0);
    }

    #[test]
    fn food_state_layout() {
        assert_eq!(mem::size_of::<FoodState>(), 1);
        assert_eq!(mem::offset_of!(FoodState, is_loose), 0);
    }

    #[test]
    fn food_encounter_layout() {
        assert_eq!(mem::size_of::<FoodEncounter>(), 32);
        assert_eq!(mem::offset_of!(FoodEncounter, food_index), 0);
        assert_eq!(mem::offset_of!(FoodEncounter, encounter_position), 8);
    }
}
