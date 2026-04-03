use unreal_api::mass::MassQuery;
use unreal_api::mass_system;
use crate::fragments::AntFragment;

/// Move ants forward, decrement cooldowns.
///
/// This is the Bevy-like equivalent of ant_movement_system from gatherers-sim.
/// Note: boundary clamping requires subsystem context (SimBounds) which is not
/// yet available through the MassQuery API. A future extension could add shared
/// fragment or subsystem access to #[mass_system].
#[mass_system]
fn ant_movement(ants: MassQuery<&mut AntFragment>, dt: f32) {
    for ant in ants.iter_mut() {
        // Decrement cooldown
        ant.pickup_cooldown_remaining_seconds =
            (ant.pickup_cooldown_remaining_seconds - dt.max(0.0)).max(0.0);

        // Store previous position
        ant.previous_position = ant.position;

        // Compute movement step
        let dir = normalize_f64x3(ant.direction);
        if is_nearly_zero_f64x3(dir) {
            continue;
        }
        let step_dist = (ant.movement_speed.max(0.0) * dt.max(0.0)) as f64;
        ant.position[0] += dir[0] * step_dist;
        ant.position[1] += dir[1] * step_dist;
        ant.position[2] += dir[2] * step_dist;
    }
}

fn normalize_f64x3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-8 {
        return [0.0; 3];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

fn is_nearly_zero_f64x3(v: [f64; 3]) -> bool {
    v[0].abs() < 1e-8 && v[1].abs() < 1e-8 && v[2].abs() < 1e-8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::AntFragment;
    use unreal_api::mass::MassQuery;

    #[test]
    fn ant_movement_moves_forward() {
        let mut ants = [AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQuery::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 0.1);
        assert!(
            (ants[0].position[0] - 10.0).abs() < 1e-6,
            "expected ~10.0, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn ant_movement_stores_previous_position() {
        let mut ants = [AntFragment {
            position: [100.0, 200.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 50.0,
            ..Default::default()
        }];
        let mut query = unsafe { MassQuery::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 0.1);
        assert_eq!(ants[0].previous_position, [100.0, 200.0, 0.0]);
    }

    #[test]
    fn ant_movement_decrements_cooldown() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 1.0,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut query = unsafe { MassQuery::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 0.3);
        assert!(
            (ants[0].pickup_cooldown_remaining_seconds - 0.7).abs() < 1e-5,
            "cooldown should be ~0.7, got {}",
            ants[0].pickup_cooldown_remaining_seconds
        );
    }

    #[test]
    fn ant_movement_cooldown_floors_at_zero() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 0.1,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        let mut query = unsafe { MassQuery::from_raw(ants.as_mut_ptr() as *mut _, ants.len()) };
        ant_movement(&mut query, 1.0);
        assert_eq!(ants[0].pickup_cooldown_remaining_seconds, 0.0);
    }
}
