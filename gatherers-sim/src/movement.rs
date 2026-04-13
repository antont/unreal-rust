use crate::fragments::{Cooldown, Movement, Position};
use bevy_mass::prelude::*;

// ---------------------------------------------------------------------------
// System ordering convention:
//   order = 10, 20, 30, ... with gaps for future insertion.
//   In Unreal mode, order determines C++ processor execution order.
//   In standalone Bevy mode, order is ignored — use .chain() instead.
// ---------------------------------------------------------------------------

/// Default simulation bounds — Rust owns this, no C++ round-trip needed.
pub const SIM_BOUNDS_MIN: [f64; 3] = [-500.0, -500.0, -100.0];
pub const SIM_BOUNDS_MAX: [f64; 3] = [500.0, 500.0, 100.0];

// ---------------------------------------------------------------------------
// System 1: Movement — position += direction * speed * dt
// Generic: works for any entity with Position + Movement.
// ---------------------------------------------------------------------------

#[mass_system(order = 10)]
pub fn entity_movement(
    mut positions: Query<&mut Position>,
    movements: Query<&Movement>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let bounds_size_x = SIM_BOUNDS_MAX[0] - SIM_BOUNDS_MIN[0];
    let bounds_size_y = SIM_BOUNDS_MAX[1] - SIM_BOUNDS_MIN[1];
    let bounds_max_step = 0.5 * bounds_size_x.min(bounds_size_y);

    for (mut pos, mov) in positions.iter_mut().zip(movements.iter()) {
        pos.previous_position = pos.position;

        if mov.direction.length() < 1e-8 { continue; }
        let dir = mov.direction.normalize();
        let max_dist = (mov.movement_speed.max(0.0) * dt.max(0.0)) as f64;
        let step_dist = max_dist.min(bounds_max_step.max(0.0));
        pos.position += dir * step_dist;
    }
}

// ---------------------------------------------------------------------------
// System 4: Cooldown — decrement pickup cooldown timers
// Pure-Bevy component with add/remove semantics. In Unreal mode, Cooldown
// lives on shadow Bevy entities, not in chunk memory.
// ---------------------------------------------------------------------------

#[mass_system(order = 40)]
pub fn entity_cooldown(
    mut cooldowns: BevyQuery<(Entity, &mut Cooldown)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut cd) in &mut cooldowns {
        cd.remaining_seconds -= dt.max(0.0);
        if cd.remaining_seconds <= 0.0 {
            commands.entity(entity).remove::<Cooldown>();
        }
    }
}

// ---------------------------------------------------------------------------
// System 5: Boundary reflection — clamp position, reflect direction
// Generic: works for any entity with Position + Movement.
// ---------------------------------------------------------------------------

#[mass_system(order = 50)]
pub fn entity_boundary_reflect(
    mut positions: Query<&mut Position>,
    mut movements: Query<&mut Movement>,
) {
    for (mut pos, mut mov) in positions.iter_mut().zip(movements.iter_mut()) {
        let mut inward_normal = DVec3::ZERO;

        if pos.position.x < SIM_BOUNDS_MIN[0] {
            pos.position.x = SIM_BOUNDS_MIN[0];
            inward_normal.x += 1.0;
        } else if pos.position.x > SIM_BOUNDS_MAX[0] {
            pos.position.x = SIM_BOUNDS_MAX[0];
            inward_normal.x -= 1.0;
        }

        if pos.position.y < SIM_BOUNDS_MIN[1] {
            pos.position.y = SIM_BOUNDS_MIN[1];
            inward_normal.y += 1.0;
        } else if pos.position.y > SIM_BOUNDS_MAX[1] {
            pos.position.y = SIM_BOUNDS_MAX[1];
            inward_normal.y -= 1.0;
        }

        if inward_normal.length() > 1e-8 {
            mov.direction = reflect_direction(mov.direction, inward_normal);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Reflect direction off a boundary normal (same as C++ ComputeBoundaryTurnBackDirection).
pub fn reflect_direction(dir: DVec3, normal: DVec3) -> DVec3 {
    if dir.length() < 1e-8 || normal.length() < 1e-8 {
        return DVec3::ZERO;
    }
    let d = dir.normalize();
    let n = normal.normalize();
    let reflected = d - 2.0 * d.dot(n) * n;
    if reflected.length() < 1e-8 {
        return DVec3::ZERO;
    }
    reflected.normalize()
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::fragments::{Cooldown, Movement, Position};
    use bevy_ecs::prelude::*;
    use core::time::Duration;

    fn insert_test_time(world: &mut World, dt_secs: f32) {
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(dt_secs));
        world.insert_resource(time);
    }

    fn run_system<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(system);
        schedule.run(world);
    }

    #[test]
    fn movement_moves_forward() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.1);
        world.spawn((
            Position::default(),
            Movement {
                direction: DVec3::X,
                movement_speed: 100.0,
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let pos = q.single(&world).unwrap();
        assert!(
            (pos.position.x - 10.0).abs() < 1e-6,
            "x: {}",
            pos.position.x
        );
    }

    #[test]
    fn movement_stores_previous_position() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.1);
        world.spawn((
            Position {
                position: DVec3::new(100.0, 200.0, 0.0),
                previous_position: DVec3::ZERO,
            },
            Movement {
                direction: DVec3::X,
                movement_speed: 50.0,
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let pos = q.single(&world).unwrap();
        assert_eq!(pos.previous_position, DVec3::new(100.0, 200.0, 0.0));
    }

    #[test]
    fn cooldown_decrements() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.3);
        world.spawn(Cooldown {
            remaining_seconds: 1.0,
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
    fn cooldown_removed_when_expired() {
        let mut world = World::new();
        insert_test_time(&mut world, 1.0);
        let entity = world.spawn(Cooldown {
            remaining_seconds: 0.1,
        }).id();

        run_system(&mut world, entity_cooldown);

        // Cooldown should be removed (not just zeroed)
        assert!(
            world.get::<Cooldown>(entity).is_none(),
            "Cooldown component should be removed when expired"
        );
    }

    #[test]
    fn boundary_clamp_and_reflect() {
        let mut world = World::new();
        world.spawn((
            Position {
                position: DVec3::new(600.0, 0.0, 0.0),
                previous_position: DVec3::ZERO,
            },
            Movement {
                direction: DVec3::X,
                movement_speed: 100.0,
            },
        ));

        run_system(&mut world, entity_boundary_reflect);

        let mut q = world.query::<(&Position, &Movement)>();
        let (pos, mov) = q.single(&world).unwrap();
        assert!(pos.position.x <= 500.0, "x: {}", pos.position.x);
        assert!(mov.direction.x < 0.0, "dir x: {}", mov.direction.x);
    }

    #[test]
    fn combined_movement_boundary_cooldown() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.5);
        let entity = world.spawn((
            Position {
                position: DVec3::new(499.0, 0.0, 0.0),
                previous_position: DVec3::ZERO,
            },
            Movement {
                direction: DVec3::X,
                movement_speed: 100.0,
            },
            Cooldown {
                remaining_seconds: 1.0,
            },
        )).id();

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        use bevy_ecs::schedule::IntoScheduleConfigs;
        schedule.add_systems((entity_movement, entity_boundary_reflect, entity_cooldown).chain());
        schedule.run(&mut world);

        let mut q = world.query::<(&Position, &Movement)>();
        let (pos, mov) = q.single(&world).unwrap();
        assert!(pos.position.x <= 500.0, "clamped: {}", pos.position.x);
        assert!(mov.direction.x < 0.0, "reflected: {}", mov.direction.x);
        // Cooldown should still exist (1.0 - 0.5 = 0.5)
        let cd = world.get::<Cooldown>(entity).expect("Cooldown should still exist");
        assert!((cd.remaining_seconds - 0.5).abs() < 1e-5, "cooldown: {}", cd.remaining_seconds);
    }

    #[test]
    fn multiple_entities_move_independently() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.05);
        world.spawn((
            Position::default(),
            Movement {
                direction: DVec3::X,
                movement_speed: 200.0,
            },
        ));
        world.spawn((
            Position::default(),
            Movement {
                direction: DVec3::Y,
                movement_speed: 50.0,
            },
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Position>();
        let positions: Vec<_> = q.iter(&world).collect();
        assert_eq!(positions.len(), 2);

        let (pos_x, pos_y) = if positions[0].position.x > 1.0 {
            (positions[0], positions[1])
        } else {
            (positions[1], positions[0])
        };
        assert!((pos_x.position.x - 10.0).abs() < 1e-6);
        assert!((pos_y.position.y - 2.5).abs() < 1e-6);
    }
}
