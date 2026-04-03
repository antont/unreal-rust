use crate::fragments::{AntFragment, FoodEncounter};

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
/// Side effects on `ant`: updates position, direction, cooldown, clears/sets carried handle.
pub fn ant_food_decision(
    ant: &mut AntFragment,
    encounter: Option<&FoodEncounter>,
) -> FoodDecisionCode {
    let is_carrying = ant.carried_food_handle[0] != 0 || ant.carried_food_handle[1] != 0;

    match encounter {
        Some(enc) if is_carrying => {
            // Drop: ant is carrying and encounters loose food
            ant.position = enc.encounter_position;
            ant.direction = consume_ant_turn_direction(ant);
            ant.carried_food_handle = [0, 0];
            ant.pickup_cooldown_remaining_seconds =
                compute_pickup_cooldown(PICKUP_SEPARATION_DISTANCE, ant.movement_speed);
            DECISION_DROP
        }
        Some(enc) if !is_carrying && ant.pickup_cooldown_remaining_seconds <= 0.0 => {
            // Pick up: ant is not carrying, cooldown expired, food nearby
            ant.position = enc.encounter_position;
            ant.direction = consume_ant_turn_direction(ant);
            ant.carried_food_handle = enc.entity_handle;
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
fn consume_ant_turn_direction(ant: &mut AntFragment) -> [f64; 3] {
    // Simple LCG matching FRandomStream behavior:
    // FRandomStream uses: seed = seed * 196314165 + 907633515
    // FRandRange(-1,1) maps to: (seed & 0x7fffff) / 8388607.0 * 2.0 - 1.0
    let mut seed = ant.random_seed as u32;
    seed = seed.wrapping_mul(196314165).wrapping_add(907633515);
    let jitter_alpha = ((seed & 0x7fffff) as f32 / 8388607.0) * 2.0 - 1.0;
    ant.random_seed = seed as i32;

    compute_ant_turn_direction(ant.direction, jitter_alpha, ant.turn_jitter_radians)
}

/// Compute turn direction: 180° turn + jitter (matches C++ ComputeAntTurnDirection).
fn compute_ant_turn_direction(
    direction: [f64; 3],
    normalized_jitter_alpha: f32,
    max_turn_jitter_radians: f32,
) -> [f64; 3] {
    let jitter_alpha = normalized_jitter_alpha.clamp(-1.0, 1.0);
    let jitter_radians = jitter_alpha * max_turn_jitter_radians.max(0.0);
    compute_ant_retarget_direction(direction, jitter_radians)
}

/// 180° turn + jitter offset (matches C++ ComputeAntRetargetDirection).
fn compute_ant_retarget_direction(direction: [f64; 3], jitter_radians: f32) -> [f64; 3] {
    let len = (direction[0] * direction[0] + direction[1] * direction[1]).sqrt();
    if len < 1e-8 {
        return [0.0; 3];
    }
    let nx = direction[0] / len;
    let ny = direction[1] / len;
    let current_angle = ny.atan2(nx);
    let retarget_angle = current_angle + std::f64::consts::PI + jitter_radians as f64;
    let cos_a = retarget_angle.cos();
    let sin_a = retarget_angle.sin();
    let rlen = (cos_a * cos_a + sin_a * sin_a).sqrt();
    if rlen < 1e-8 {
        return [0.0; 3];
    }
    [cos_a / rlen, sin_a / rlen, 0.0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::AntFragment;

    fn make_ant(carrying: bool) -> AntFragment {
        let mut ant = AntFragment {
            position: [100.0, 100.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            turn_jitter_radians: std::f32::consts::FRAC_PI_2,
            random_seed: 42,
            ..Default::default()
        };
        if carrying {
            ant.carried_food_handle = [1, 1]; // non-zero = valid
        }
        ant
    }

    fn make_encounter() -> FoodEncounter {
        FoodEncounter {
            entity_handle: [5, 10],
            encounter_position: [110.0, 105.0, 0.0],
        }
    }

    #[test]
    fn carrying_and_encounters_food_drops() {
        let mut ant = make_ant(true);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(result, DECISION_DROP);
    }

    #[test]
    fn drop_clears_carried_handle() {
        let mut ant = make_ant(true);
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(ant.carried_food_handle, [0, 0], "carried handle should be cleared");
    }

    #[test]
    fn drop_sets_cooldown() {
        let mut ant = make_ant(true);
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert!(
            ant.pickup_cooldown_remaining_seconds > 0.0,
            "cooldown should be set after drop"
        );
    }

    #[test]
    fn drop_snaps_position_to_encounter() {
        let mut ant = make_ant(true);
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(ant.position, encounter.encounter_position);
    }

    #[test]
    fn drop_changes_direction() {
        let mut ant = make_ant(true);
        let original_dir = ant.direction;
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert_ne!(ant.direction, original_dir, "direction should change on drop");
    }

    #[test]
    fn not_carrying_no_cooldown_encounters_food_picks_up() {
        let mut ant = make_ant(false);
        let encounter = make_encounter();
        let result = ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(result, DECISION_PICK_UP);
    }

    #[test]
    fn pickup_sets_carried_handle() {
        let mut ant = make_ant(false);
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(ant.carried_food_handle, encounter.entity_handle);
    }

    #[test]
    fn pickup_snaps_position_to_encounter() {
        let mut ant = make_ant(false);
        let encounter = make_encounter();
        ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(ant.position, encounter.encounter_position);
    }

    #[test]
    fn not_carrying_with_cooldown_no_action() {
        let mut ant = make_ant(false);
        ant.pickup_cooldown_remaining_seconds = 0.5;
        let encounter = make_encounter();
        let result = ant_food_decision(&mut ant, Some(&encounter));
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn not_carrying_no_encounter_no_action() {
        let mut ant = make_ant(false);
        let result = ant_food_decision(&mut ant, None);
        assert_eq!(result, DECISION_NO_ACTION);
    }

    #[test]
    fn carrying_no_encounter_no_action() {
        let mut ant = make_ant(true);
        let result = ant_food_decision(&mut ant, None);
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
}
