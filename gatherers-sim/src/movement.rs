use crate::fragments::{Movement, Position};
use bevy_mass::prelude::*;

#[cfg(feature = "unreal")]
use unreal_api::mass_system;

/// Default simulation bounds — Rust owns this, no C++ round-trip needed.
pub const SIM_BOUNDS_MIN: [f64; 3] = [-500.0, -500.0, -100.0];
pub const SIM_BOUNDS_MAX: [f64; 3] = [500.0, 500.0, 100.0];

// ---------------------------------------------------------------------------
// System 1: Movement — position += direction * speed * dt
// Generic: works for any entity with Position + Movement.
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "unreal", mass_system(order = 10))]
pub fn entity_movement(
    mut positions: Query<&mut Position>,
    movements: Query<&Movement>,
    time: Res<DeltaTime>,
) {
    let dt = time.0;
    let bounds_size_x = SIM_BOUNDS_MAX[0] - SIM_BOUNDS_MIN[0];
    let bounds_size_y = SIM_BOUNDS_MAX[1] - SIM_BOUNDS_MIN[1];
    let bounds_max_step = 0.5 * bounds_size_x.min(bounds_size_y);

    for (mut pos, mov) in positions.iter_mut().zip(movements.iter()) {
        pos.previous_position = pos.position;

        let dir = normalize_f64x3(mov.direction);
        if is_nearly_zero_f64x3(dir) {
            continue;
        }
        let max_dist = (mov.movement_speed.max(0.0) * dt.max(0.0)) as f64;
        let step_dist = max_dist.min(bounds_max_step.max(0.0));
        pos.position[0] += dir[0] * step_dist;
        pos.position[1] += dir[1] * step_dist;
        pos.position[2] += dir[2] * step_dist;
    }
}

// ---------------------------------------------------------------------------
// System 4: Cooldown — decrement pickup cooldown timers
// Generic: works for any entity with Cooldown.
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "unreal", mass_system(order = 40))]
pub fn entity_cooldown(
    mut cooldowns: Query<&mut crate::fragments::Cooldown>,
    time: Res<DeltaTime>,
) {
    let dt = time.0;
    for mut cd in &mut cooldowns {
        cd.remaining_seconds = (cd.remaining_seconds - dt.max(0.0)).max(0.0);
    }
}

// ---------------------------------------------------------------------------
// System 5: Boundary reflection — clamp position, reflect direction
// Generic: works for any entity with Position + Movement.
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "unreal", mass_system(order = 50))]
pub fn entity_boundary_reflect(
    mut positions: Query<&mut Position>,
    mut movements: Query<&mut Movement>,
) {
    for (mut pos, mut mov) in positions.iter_mut().zip(movements.iter_mut()) {
        let mut inward_normal = [0.0f64; 3];

        if pos.position[0] < SIM_BOUNDS_MIN[0] {
            pos.position[0] = SIM_BOUNDS_MIN[0];
            inward_normal[0] += 1.0;
        } else if pos.position[0] > SIM_BOUNDS_MAX[0] {
            pos.position[0] = SIM_BOUNDS_MAX[0];
            inward_normal[0] -= 1.0;
        }

        if pos.position[1] < SIM_BOUNDS_MIN[1] {
            pos.position[1] = SIM_BOUNDS_MIN[1];
            inward_normal[1] += 1.0;
        } else if pos.position[1] > SIM_BOUNDS_MAX[1] {
            pos.position[1] = SIM_BOUNDS_MAX[1];
            inward_normal[1] -= 1.0;
        }

        if !is_nearly_zero_f64x3(inward_normal) {
            mov.direction = reflect_direction(mov.direction, inward_normal);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub fn normalize_f64x3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-8 {
        return [0.0; 3];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn is_nearly_zero_f64x3(v: [f64; 3]) -> bool {
    v[0].abs() < 1e-8 && v[1].abs() < 1e-8 && v[2].abs() < 1e-8
}

pub fn dot_f64x3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Reflect direction off a boundary normal (same as C++ ComputeBoundaryTurnBackDirection).
pub fn reflect_direction(dir: [f64; 3], normal: [f64; 3]) -> [f64; 3] {
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

pub fn reverse_direction(dir: [f64; 3]) -> [f64; 3] {
    [-dir[0], -dir[1], -dir[2]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragments::{Cooldown, Movement, Position};
    use bevy_ecs::prelude::*;

    fn run_system<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(system);
        schedule.run(world);
    }

    #[test]
    fn movement_moves_forward() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.1));
        world.spawn((
            Position::default(),
            Movement {
                direction: [1.0, 0.0, 0.0],
                movement_speed: 100.0,
                _pad: [0; 4],
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let pos = q.single(&world).unwrap();
        assert!(
            (pos.position[0] - 10.0).abs() < 1e-6,
            "x: {}",
            pos.position[0]
        );
    }

    #[test]
    fn movement_stores_previous_position() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.1));
        world.spawn((
            Position {
                position: [100.0, 200.0, 0.0],
                previous_position: [0.0; 3],
            },
            Movement {
                direction: [1.0, 0.0, 0.0],
                movement_speed: 50.0,
                _pad: [0; 4],
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let pos = q.single(&world).unwrap();
        assert_eq!(pos.previous_position, [100.0, 200.0, 0.0]);
    }

    #[test]
    fn cooldown_decrements() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.3));
        world.spawn(Cooldown {
            remaining_seconds: 1.0,
            _pad: [0; 4],
        });

        run_system(&mut world, entity_cooldown);

        let mut q = world.query::<&Cooldown>();
        let cd = q.single(&world).unwrap();
        assert!(
            (cd.remaining_seconds - 0.7).abs() < 1e-5,
            "cooldown: {}",
            cd.remaining_seconds
        );
    }

    #[test]
    fn cooldown_floors_at_zero() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(1.0));
        world.spawn(Cooldown {
            remaining_seconds: 0.1,
            _pad: [0; 4],
        });

        run_system(&mut world, entity_cooldown);

        let mut q = world.query::<&Cooldown>();
        let cd = q.single(&world).unwrap();
        assert_eq!(cd.remaining_seconds, 0.0);
    }

    #[test]
    fn boundary_clamp_and_reflect() {
        let mut world = World::new();
        world.spawn((
            Position {
                position: [600.0, 0.0, 0.0],
                previous_position: [0.0; 3],
            },
            Movement {
                direction: [1.0, 0.0, 0.0],
                movement_speed: 100.0,
                _pad: [0; 4],
            },
        ));

        run_system(&mut world, entity_boundary_reflect);

        let mut q = world.query::<(&Position, &Movement)>();
        let (pos, mov) = q.single(&world).unwrap();
        assert!(pos.position[0] <= 500.0, "x: {}", pos.position[0]);
        assert!(mov.direction[0] < 0.0, "dir x: {}", mov.direction[0]);
    }

    #[test]
    fn combined_movement_boundary_cooldown() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(1.0));
        world.spawn((
            Position {
                position: [499.0, 0.0, 0.0],
                previous_position: [0.0; 3],
            },
            Movement {
                direction: [1.0, 0.0, 0.0],
                movement_speed: 100.0,
                _pad: [0; 4],
            },
            Cooldown {
                remaining_seconds: 1.0,
                _pad: [0; 4],
            },
        ));

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        use bevy_ecs::schedule::IntoScheduleConfigs;
        schedule.add_systems((entity_movement, entity_boundary_reflect, entity_cooldown).chain());
        schedule.run(&mut world);

        let mut q = world.query::<(&Position, &Movement, &Cooldown)>();
        let (pos, mov, cd) = q.single(&world).unwrap();
        assert!(pos.position[0] <= 500.0, "clamped: {}", pos.position[0]);
        assert!(mov.direction[0] < 0.0, "reflected: {}", mov.direction[0]);
        assert_eq!(cd.remaining_seconds, 0.0);
    }

    #[test]
    fn multiple_entities_move_independently() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.05));
        world.spawn((
            Position::default(),
            Movement {
                direction: [1.0, 0.0, 0.0],
                movement_speed: 200.0,
                _pad: [0; 4],
            },
        ));
        world.spawn((
            Position::default(),
            Movement {
                direction: [0.0, 1.0, 0.0],
                movement_speed: 50.0,
                _pad: [0; 4],
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let positions: Vec<_> = q.iter(&world).collect();
        assert_eq!(positions.len(), 2);

        let (pos_x, pos_y) = if positions[0].position[0] > 1.0 {
            (positions[0], positions[1])
        } else {
            (positions[1], positions[0])
        };
        assert!((pos_x.position[0] - 10.0).abs() < 1e-6);
        assert!((pos_y.position[1] - 2.5).abs() < 1e-6);
    }
}
