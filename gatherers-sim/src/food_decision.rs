use crate::fragments::{Movement, Behavior, Carrying, Cooldown, FoodEncounter};
use glam::DVec3;

/// Result of ant-food interaction decision.
/// 0 = NoAction, 1 = PickUp, 2 = Drop
pub type FoodDecisionCode = i32;
pub const DECISION_NO_ACTION: FoodDecisionCode = 0;
pub const DECISION_PICK_UP: FoodDecisionCode = 1;
pub const DECISION_DROP: FoodDecisionCode = 2;

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
    movement: &mut Movement,
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
            movement.direction = consume_ant_turn_direction(behavior, movement);
            carrying.food_index = -1;
            cooldown.remaining_seconds =
                compute_pickup_cooldown(PICKUP_SEPARATION_DISTANCE, movement.movement_speed);
            DECISION_DROP
        }
        Some(enc) if !is_carrying => {
            // Pick up: ant is not carrying, cooldown expired, food nearby
            *ant_position = enc.encounter_position;
            movement.direction = consume_ant_turn_direction(behavior, movement);
            carrying.food_index = enc.food_index;
            cooldown.remaining_seconds =
                compute_pickup_cooldown(PICKUP_SEPARATION_DISTANCE, movement.movement_speed);
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
fn consume_ant_turn_direction(behavior: &mut Behavior, movement: &Movement) -> DVec3 {
    // Simple LCG matching FRandomStream behavior:
    // FRandomStream uses: seed = seed * 196314165 + 907633515
    // FRandRange(-1,1) maps to: (seed & 0x7fffff) / 8388607.0 * 2.0 - 1.0
    let mut seed = behavior.random_seed as u32;
    seed = seed.wrapping_mul(196314165).wrapping_add(907633515);
    let jitter_alpha = ((seed & 0x7fffff) as f32 / 8388607.0) * 2.0 - 1.0;
    behavior.random_seed = seed as i32;

    compute_ant_turn_direction(movement.direction, jitter_alpha, behavior.turn_jitter_radians)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_components(carrying_food: bool) -> (Movement, Cooldown, Carrying, Behavior) {
        let movement = Movement {
            direction: DVec3::X,
            movement_speed: 100.0,
        };
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
        (movement, cooldown, carrying, behavior)
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
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_DROP);
    }

    #[test]
    fn drop_clears_carried_handle() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(carry.food_index, -1, "carried index should be cleared");
    }

    #[test]
    fn drop_sets_cooldown() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert!(cd.remaining_seconds > 0.0, "cooldown should be set after drop");
    }

    #[test]
    fn drop_snaps_position_to_encounter() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(pos, encounter.encounter_position);
    }

    #[test]
    fn drop_changes_direction() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let original_dir = mov.direction;
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_ne!(mov.direction, original_dir, "direction should change on drop");
    }

    #[test]
    fn not_carrying_no_cooldown_encounters_food_picks_up() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_PICK_UP);
    }

    #[test]
    fn pickup_sets_carried_handle() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(carry.food_index, encounter.food_index);
    }

    #[test]
    fn pickup_snaps_position_to_encounter() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        let encounter = make_encounter();
        ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(pos, encounter.encounter_position);
    }

    #[test]
    fn not_carrying_with_cooldown_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        cd.remaining_seconds = 0.5;
        let encounter = make_encounter();
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&encounter));
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn not_carrying_no_encounter_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, None);
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn carrying_no_encounter_no_action() {
        let mut pos = DVec3::new(100.0, 100.0, 0.0);
        let (mut mov, mut cd, mut carry, mut beh) = make_components(true);
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, None);
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
        let (mut mov, mut cd, mut carry, mut beh) = make_components(false);
        let food_0 = FoodEncounter {
            food_index: 0,
            encounter_position: DVec3::new(105.0, 100.0, 0.0),
        };
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&food_0));
        assert_eq!(result, DECISION_PICK_UP);
        assert!(carry.food_index >= 0, "ant should be carrying");

        // Frame 2: ant encounters food_1 nearby — should NOT drop immediately
        let food_1 = FoodEncounter {
            food_index: 1,
            encounter_position: DVec3::new(110.0, 100.0, 0.0),
        };
        let result = ant_food_decision(&mut pos, &mut mov, &mut cd, &mut carry, &mut beh, Some(&food_1));
        // BUG: this currently returns DECISION_DROP — ant never visibly carries food
        assert_ne!(result, DECISION_DROP, "ant should not drop on the very next encounter");
    }
}
