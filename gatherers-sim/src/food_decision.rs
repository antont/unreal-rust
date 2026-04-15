pub use crate::fragments::{
    FoodDecisionCode, DECISION_NO_ACTION, DECISION_PICK_UP, DECISION_DROP,
};
use crate::fragments::{
    DesiredMovement, Behavior, Carrying, Cooldown, FoodEncounter,
};
#[cfg(not(feature = "unreal"))]
use crate::fragments::{AntFoodHit, FoodMutation, Transform};
#[cfg(not(feature = "unreal"))]
use bevy_ecs::message::{MessageReader, MessageWriter};
#[cfg(not(feature = "unreal"))]
use bevy_ecs::prelude::Commands;
#[cfg(not(feature = "unreal"))]
use bevy_mass::prelude::Query;
use glam::DVec3;

/// Pickup separation distance (matches C++ GatherersMassPickupSeparationDistance)
pub const PICKUP_SEPARATION_DISTANCE: f32 = 50.0;

/// Decide what an ant should do when encountering food.
///
/// Port of UGatherersFoodInteractionProcessor decision logic.
/// C++ keeps the sweep query and entity operations; Rust makes the decision.
///
/// If the ant is carrying food and encounters loose food: drop (return 2).
/// If the ant is not carrying and cooldown expired and encounters food: pick up (return 1).
/// Otherwise: no action (return 0).
///
/// Side effects: updates position, direction, cooldown, clears/sets carried handle.
pub fn ant_food_decision(
    ant_position: &mut DVec3,
    movement: &mut DesiredMovement,
    cooldown: &mut Cooldown,
    carrying: &mut Carrying,
    behavior: &mut Behavior,
    encounter: Option<&FoodEncounter>,
) -> FoodDecisionCode {
    // Cooldown blocks all food interactions — matches the original gatherers
    // pattern where ants with Cooldown are excluded from collision entirely.
    if cooldown.remaining_seconds > 0.0 {
        return DECISION_NO_ACTION;
    }

    let is_carrying = carrying.food_index >= 0;

    match encounter {
        Some(enc) if is_carrying => {
            // Drop: ant is carrying and encounters loose food
            *ant_position = enc.encounter_position;
            let speed = movement.speed();
            let new_dir = consume_ant_turn_direction(behavior, movement);
            movement.velocity = new_dir * speed as f64;
            carrying.food_index = -1;
            cooldown.remaining_seconds =
                compute_pickup_cooldown(PICKUP_SEPARATION_DISTANCE, speed);
            DECISION_DROP
        }
        Some(enc) if !is_carrying => {
            // Pick up: ant is not carrying, cooldown expired, food nearby
            *ant_position = enc.encounter_position;
            let speed = movement.speed();
            let new_dir = consume_ant_turn_direction(behavior, movement);
            movement.velocity = new_dir * speed as f64;
            carrying.food_index = enc.food_index;
            cooldown.remaining_seconds =
                compute_pickup_cooldown(PICKUP_SEPARATION_DISTANCE, speed);
            DECISION_PICK_UP
        }
        _ => DECISION_NO_ACTION,
    }
}

/// Compute pickup cooldown from separation distance and speed.
pub fn compute_pickup_cooldown(separation_distance: f32, movement_speed: f32) -> f32 {
    if movement_speed <= 1e-4 {
        return 0.0;
    }
    (separation_distance.max(0.0)) / movement_speed.max(0.0)
}

/// Compute a turn direction (180° + jitter) given current direction and RNG state.
/// Consumes the random seed and returns new direction.
fn consume_ant_turn_direction(behavior: &mut Behavior, movement: &DesiredMovement) -> DVec3 {
    // Simple LCG matching FRandomStream behavior:
    // FRandomStream uses: seed = seed * 196314165 + 907633515
    // FRandRange(-1,1) maps to: (seed & 0x7fffff) / 8388607.0 * 2.0 - 1.0
    let mut seed = behavior.random_seed as u32;
    seed = seed.wrapping_mul(196314165).wrapping_add(907633515);
    let jitter_alpha = ((seed & 0x7fffff) as f32 / 8388607.0) * 2.0 - 1.0;
    behavior.random_seed = seed as i32;

    compute_ant_turn_direction(movement.direction(), jitter_alpha, behavior.turn_jitter_radians)
}

/// Compute turn direction: 180° turn + jitter (matches C++ ComputeAntTurnDirection).
fn compute_ant_turn_direction(
    direction: DVec3,
    normalized_jitter_alpha: f32,
    max_turn_jitter_radians: f32,
) -> DVec3 {
    let jitter_alpha = normalized_jitter_alpha.clamp(-1.0, 1.0);
    let jitter_radians = jitter_alpha * max_turn_jitter_radians.max(0.0);
    compute_ant_retarget_direction(direction, jitter_radians)
}

/// 180° turn + jitter offset (matches C++ ComputeAntRetargetDirection).
fn compute_ant_retarget_direction(direction: DVec3, jitter_radians: f32) -> DVec3 {
    let len_2d = (direction.x * direction.x + direction.y * direction.y).sqrt();
    if len_2d < 1e-8 {
        return DVec3::ZERO;
    }
    let current_angle = direction.y.atan2(direction.x);
    let retarget_angle = current_angle + std::f64::consts::PI + jitter_radians as f64;
    let result = DVec3::new(retarget_angle.cos(), retarget_angle.sin(), 0.0);
    if result.length() < 1e-8 {
        return DVec3::ZERO;
    }
    result.normalize()
}

// ---------------------------------------------------------------------------
// Shared food decision system (standalone Bevy mode)
//
// Reads HitEvent messages from the collision prepass, calls the pure decision
// function, inserts Cooldown, and emits FoodMutation messages for the
// mode-specific apply system.
// ---------------------------------------------------------------------------

#[cfg(not(feature = "unreal"))]
pub fn food_decision_system(
    mut hits: MessageReader<AntFoodHit>,
    mut food_mutations: MessageWriter<FoodMutation>,
    mut ants: Query<(&mut Transform, &mut DesiredMovement, &mut Carrying, &mut Behavior)>,
    mut commands: Commands,
) {
    for hit in hits.read() {
        let Ok((mut transform, mut movement, mut carry, mut behavior)) = ants.get_mut(hit.hitter_entity) else {
            continue;
        };

        let old_food_index = carry.food_index;
        let pos_before = transform.translation;
        let mut cd = Cooldown { remaining_seconds: 0.0 };
        let encounter = FoodEncounter {
            food_index: hit.hittable_index,
            encounter_position: hit.encounter_position,
        };

        let decision = ant_food_decision(
            &mut transform.translation, &mut movement, &mut cd, &mut carry, &mut behavior,
            Some(&encounter),
        );

        if decision != DECISION_NO_ACTION {
            commands.entity(hit.hitter_entity).insert(cd);
            food_mutations.write(FoodMutation {
                food_index: if decision == DECISION_DROP { old_food_index } else { hit.hittable_index },
                decision,
                drop_position: pos_before,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_components(carrying_food: bool) -> (DesiredMovement, Cooldown, Carrying, Behavior) {
        let velocity = DesiredMovement::new(DVec3::X, 100.0);
        let cooldown = Cooldown { remaining_seconds: 0.0 };
        let carrying = if carrying_food {
            Carrying { food_index: 0 }
        } else {
            Carrying::default()
        };
        let behavior = Behavior {
            turn_jitter_radians: std::f32::consts::FRAC_PI_2,
            random_seed: 42,
        };
        (velocity, cooldown, carrying, behavior)
    }

    fn make_encounter() -> FoodEncounter {
        FoodEncounter {
            food_index: 0,
            encounter_position: DVec3::new(110.0, 105.0, 0.0),
        }
    }

    #[test]
    fn carrying_and_encounters_food_drops() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_DROP);
    }

    #[test]
    fn drop_clears_carried_handle() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(carry.food_index, -1, "carried index should be cleared");
    }

    #[test]
    fn drop_sets_cooldown() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert!(cd.remaining_seconds > 0.0, "cooldown should be set after drop");
    }

    #[test]
    fn drop_snaps_position_to_encounter() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(pos, encounter.encounter_position);
    }

    #[test]
    fn drop_changes_direction() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let original_dir = vel.direction();
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_ne!(vel.direction(), original_dir, "direction should change on drop");
    }

    #[test]
    fn not_carrying_no_cooldown_encounters_food_picks_up() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_PICK_UP);
    }

    #[test]
    fn pickup_sets_carried_handle() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(carry.food_index, encounter.food_index);
    }

    #[test]
    fn pickup_snaps_position_to_encounter() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(pos, encounter.encounter_position);
    }

    #[test]
    fn not_carrying_with_cooldown_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        cd.remaining_seconds = 0.5;
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn not_carrying_no_encounter_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, None);
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn carrying_no_encounter_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(true);
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, None);
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn compute_pickup_cooldown_basic() {
        let cooldown = compute_pickup_cooldown(50.0, 100.0);
        assert!((cooldown - 0.5).abs() < 1e-6);
    }

    #[test]
    fn compute_pickup_cooldown_zero_speed() {
        assert_eq!(compute_pickup_cooldown(50.0, 0.0), 0.0);
    }

    /// Reproduces the "instant drop" bug: ant picks up food, then on the very
    /// next decision encounters a second food item and drops immediately — the
    /// carrying state lasts only one frame.
    #[test]
    fn carrying_ant_should_not_drop_immediately() {
        // Frame 1: ant picks up food_0
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut vel, mut cd, mut carry, mut beh) = make_components(false);
        let food_0 = FoodEncounter {
            food_index: 0,
            encounter_position: DVec3::new(105.0, 100.0, 0.0),
        };
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&food_0));
        assert_eq!(result, DECISION_PICK_UP);
        assert!(carry.food_index >= 0, "ant should be carrying");

        // Frame 2: ant encounters food_1 nearby — should NOT drop immediately
        let food_1 = FoodEncounter {
            food_index: 1,
            encounter_position: DVec3::new(110.0, 100.0, 0.0),
        };
        let result = ant_food_decision(&mut pos, &mut vel, &mut cd, &mut carry, &mut beh, Some(&food_1));
        // BUG: this currently returns DECISION_DROP — ant never visibly carries food
        assert_ne!(result, DECISION_DROP, "ant should not drop on the very next encounter");
    }
}
