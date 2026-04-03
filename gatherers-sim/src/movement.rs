use crate::fragments::{AntFragment, SimBounds};

/// Move ants forward, reflect at boundaries, decrement cooldowns.
pub fn ant_movement_system(_ants: &mut [AntFragment], _dt: f32, _bounds: &SimBounds) {
    // TODO: implement
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
