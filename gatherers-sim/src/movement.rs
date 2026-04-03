use crate::fragments::{AntFragment, SimBounds};

/// Move ants forward, reflect at boundaries, decrement cooldowns.
///
/// Port of UGatherersAntMovementProcessor::Execute() from C++.
pub fn ant_movement_system(ants: &mut [AntFragment], dt: f32, bounds: &SimBounds) {
    let bounds_size_x = bounds.max[0] - bounds.min[0];
    let bounds_size_y = bounds.max[1] - bounds.min[1];
    let bounds_max_step = 0.5 * bounds_size_x.min(bounds_size_y);

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
        let max_dist = (ant.movement_speed.max(0.0) * dt.max(0.0)) as f64;
        let step_dist = max_dist.min(bounds_max_step.max(0.0));
        ant.position[0] += dir[0] * step_dist;
        ant.position[1] += dir[1] * step_dist;
        ant.position[2] += dir[2] * step_dist;

        // Boundary clamping and reflection
        let mut inward_normal = [0.0f64; 3];

        if ant.position[0] < bounds.min[0] {
            ant.position[0] = bounds.min[0];
            inward_normal[0] += 1.0;
        } else if ant.position[0] > bounds.max[0] {
            ant.position[0] = bounds.max[0];
            inward_normal[0] -= 1.0;
        }

        if ant.position[1] < bounds.min[1] {
            ant.position[1] = bounds.min[1];
            inward_normal[1] += 1.0;
        } else if ant.position[1] > bounds.max[1] {
            ant.position[1] = bounds.max[1];
            inward_normal[1] -= 1.0;
        }

        if !is_nearly_zero_f64x3(inward_normal) {
            ant.direction = reflect_direction(ant.direction, inward_normal);
        }
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

fn dot_f64x3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Reflect direction off a boundary normal (same as C++ ComputeBoundaryTurnBackDirection).
fn reflect_direction(dir: [f64; 3], normal: [f64; 3]) -> [f64; 3] {
    let safe_dir = normalize_f64x3(dir);
    let safe_normal = normalize_f64x3(normal);
    if is_nearly_zero_f64x3(safe_dir) || is_nearly_zero_f64x3(safe_normal) {
        return [0.0; 3];
    }
    let d = dot_f64x3(safe_dir, safe_normal);
    let reflected = [
        safe_dir[0] - 2.0 * d * safe_normal[0],
        safe_dir[1] - 2.0 * d * safe_normal[1],
        safe_dir[2] - 2.0 * d * safe_normal[2],
    ];
    normalize_f64x3(reflected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::AntFragment;

    fn ant_at_origin_heading_right(speed: f32) -> AntFragment {
        AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: speed,
            ..Default::default()
        }
    }

    fn default_bounds() -> SimBounds {
        SimBounds {
            min: [-500.0, -500.0, -100.0],
            max: [500.0, 500.0, 100.0],
        }
    }

    #[test]
    fn moves_forward_by_direction_times_speed_times_dt() {
        let mut ants = [ant_at_origin_heading_right(100.0)];
        ant_movement_system(&mut ants, 0.1, &default_bounds());
        assert!(
            (ants[0].position[0] - 10.0).abs() < 1e-6,
            "expected ~10.0, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn stores_previous_position() {
        let mut ants = [AntFragment {
            position: [100.0, 200.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 50.0,
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 0.1, &default_bounds());
        assert_eq!(ants[0].previous_position, [100.0, 200.0, 0.0]);
    }

    #[test]
    fn boundary_clamp_x_max() {
        let mut ants = [AntFragment {
            position: [499.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 1.0, &default_bounds());
        assert!(
            ants[0].position[0] <= 500.0,
            "X should be clamped to 500, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn boundary_clamp_x_min() {
        let mut ants = [AntFragment {
            position: [-499.0, 0.0, 0.0],
            direction: [-1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 1.0, &default_bounds());
        assert!(
            ants[0].position[0] >= -500.0,
            "X should be clamped to -500, got {}",
            ants[0].position[0]
        );
    }

    #[test]
    fn direction_reflects_at_boundary() {
        let mut ants = [AntFragment {
            position: [499.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 1.0, &default_bounds());
        assert!(
            ants[0].direction[0] < 0.0,
            "direction X should reflect negative, got {}",
            ants[0].direction[0]
        );
    }

    #[test]
    fn cooldown_decrements() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 1.0,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 0.3, &default_bounds());
        assert!(
            (ants[0].pickup_cooldown_remaining_seconds - 0.7).abs() < 1e-5,
            "cooldown should be ~0.7, got {}",
            ants[0].pickup_cooldown_remaining_seconds
        );
    }

    #[test]
    fn cooldown_does_not_go_negative() {
        let mut ants = [AntFragment {
            pickup_cooldown_remaining_seconds: 0.1,
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 1.0, &default_bounds());
        assert_eq!(ants[0].pickup_cooldown_remaining_seconds, 0.0);
    }

    #[test]
    fn zero_dt_no_change() {
        let mut ants = [AntFragment {
            position: [10.0, 20.0, 30.0],
            movement_speed: 100.0,
            direction: [1.0, 0.0, 0.0],
            ..Default::default()
        }];
        ant_movement_system(&mut ants, 0.0, &default_bounds());
        assert_eq!(ants[0].position, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn empty_slice_noop() {
        let mut ants: Vec<AntFragment> = vec![];
        ant_movement_system(&mut ants, 0.1, &default_bounds()); // should not panic
    }

    #[test]
    fn batch_10k_under_1ms() {
        let mut ants: Vec<AntFragment> = (0..10_000)
            .map(|i| AntFragment {
                position: [i as f64, 0.0, 0.0],
                direction: [1.0, 0.0, 0.0],
                movement_speed: 100.0,
                ..Default::default()
            })
            .collect();
        let bounds = default_bounds();
        let start = std::time::Instant::now();
        for _ in 0..60 {
            ant_movement_system(&mut ants, 0.016, &bounds);
        }
        let per_frame = start.elapsed() / 60;
        assert!(
            per_frame.as_micros() < 1000,
            "10k entities should process in < 1ms per frame, took {:?}",
            per_frame
        );
    }
}
