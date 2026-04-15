use crate::fragments::{Cooldown, DesiredMovement, PreviousTranslation, Transform};
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

/// In Unreal mode, movement (pos += vel * dt) is handled by UE's native
/// UMassApplyMovementProcessor. In standalone Bevy mode, this system
/// applies DesiredMovement to position directly.
#[mass_system(order = 10)]
pub fn entity_movement(
    mut transforms: Query<&mut Transform>,
    mut prev_translations: Query<&mut PreviousTranslation>,
    movements: Query<&DesiredMovement>,
    time: Res<Time>,
) {
    // In Unreal mode, UE's UMassApplyMovementProcessor handles movement.
    #[cfg(feature = "unreal")]
    {
        let _ = (&transforms, &prev_translations, &movements, &time);
    }
    #[cfg(not(feature = "unreal"))]
    {
        let dt = time.delta_secs();
        let bounds_size_x = SIM_BOUNDS_MAX[0] - SIM_BOUNDS_MIN[0];
        let bounds_size_y = SIM_BOUNDS_MAX[1] - SIM_BOUNDS_MIN[1];
        let bounds_max_step = 0.5 * bounds_size_x.min(bounds_size_y);

        for ((mut transform, mut prev), movement) in transforms.iter_mut().zip(prev_translations.iter_mut()).zip(movements.iter()) {
            prev.value = transform.translation;

            let speed = movement.velocity.length();
            if speed < 1e-8 { continue; }
            let dir = movement.velocity / speed;
            let max_dist = (speed * dt as f64).min(bounds_max_step.max(0.0));
            transform.translation += dir * max_dist;
        }
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

/// Reflect desired movement at simulation boundaries. In both modes, reads
/// position to detect boundary contact and reflects the desired velocity.
/// Position clamping differs:
/// - Unreal mode: C++ URustMassPostMovementProcessor clamps after applying velocity
/// - Standalone mode: this system also clamps position directly
#[mass_system(order = 50)]
pub fn entity_boundary_reflect(
    mut transforms: Query<&mut Transform>,
    mut movements: Query<&mut DesiredMovement>,
) {
    for (mut transform, mut movement) in transforms.iter_mut().zip(movements.iter_mut()) {
        let inward_normal = compute_boundary_normal(transform.translation);

        // In standalone mode, clamp position here.
        // In Unreal mode, C++ clamps after applying velocity.
        #[cfg(not(feature = "unreal"))]
        {
            if transform.translation.x < SIM_BOUNDS_MIN[0] {
                transform.translation.x = SIM_BOUNDS_MIN[0];
            } else if transform.translation.x > SIM_BOUNDS_MAX[0] {
                transform.translation.x = SIM_BOUNDS_MAX[0];
            }
            if transform.translation.y < SIM_BOUNDS_MIN[1] {
                transform.translation.y = SIM_BOUNDS_MIN[1];
            } else if transform.translation.y > SIM_BOUNDS_MAX[1] {
                transform.translation.y = SIM_BOUNDS_MAX[1];
            }
        }

        if inward_normal.length() > 1e-8 {
            movement.velocity = reflect_velocity(movement.velocity, inward_normal);
        }
    }
}

/// Compute inward normal for boundary contact. Returns zero vector if not at boundary.
fn compute_boundary_normal(position: DVec3) -> DVec3 {
    let mut inward_normal = DVec3::ZERO;
    if position.x <= SIM_BOUNDS_MIN[0] {
        inward_normal.x += 1.0;
    } else if position.x >= SIM_BOUNDS_MAX[0] {
        inward_normal.x -= 1.0;
    }
    if position.y <= SIM_BOUNDS_MIN[1] {
        inward_normal.y += 1.0;
    } else if position.y >= SIM_BOUNDS_MAX[1] {
        inward_normal.y -= 1.0;
    }
    inward_normal
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Reflect velocity off a boundary normal, preserving speed.
pub fn reflect_velocity(vel: DVec3, normal: DVec3) -> DVec3 {
    let speed = vel.length();
    if speed < 1e-8 || normal.length() < 1e-8 {
        return DVec3::ZERO;
    }
    let d = vel / speed;
    let n = normal.normalize();
    let reflected = d - 2.0 * d.dot(n) * n;
    if reflected.length() < 1e-8 {
        return DVec3::ZERO;
    }
    reflected.normalize() * speed
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::fragments::{Cooldown, DesiredMovement, PreviousTranslation, Transform};
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
            Transform::default(),
            PreviousTranslation::default(),
            DesiredMovement::new(DVec3::X, 100.0),
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Transform>();
        let t = q.single(&world).unwrap();
        assert!(
            (t.translation.x - 10.0).abs() < 1e-6,
            "x: {}",
            t.translation.x
        );
    }

    #[test]
    fn movement_stores_previous_translation() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.1);
        world.spawn((
            Transform::from_translation(DVec3::new(100.0, 200.0, 0.0)),
            PreviousTranslation::default(),
            DesiredMovement::new(DVec3::X, 50.0),
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&PreviousTranslation>();
        let prev = q.single(&world).unwrap();
        assert_eq!(prev.value, DVec3::new(100.0, 200.0, 0.0));
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

        assert!(
            world.get::<Cooldown>(entity).is_none(),
            "Cooldown component should be removed when expired"
        );
    }

    #[test]
    fn boundary_clamp_and_reflect() {
        let mut world = World::new();
        world.spawn((
            Transform::from_translation(DVec3::new(6000.0, 0.0, 0.0)),
            DesiredMovement::new(DVec3::X, 100.0),
        ));

        run_system(&mut world, entity_boundary_reflect);

        let mut q = world.query::<(&Transform, &DesiredMovement)>();
        let (t, dm) = q.single(&world).unwrap();
        assert!(t.translation.x <= 5000.0, "x: {}", t.translation.x);
        assert!(dm.direction().x < 0.0, "dir x: {}", dm.direction().x);
        assert!((dm.speed() - 100.0).abs() < 1e-4, "speed preserved: {}", dm.speed());
    }

    #[test]
    fn combined_movement_boundary_cooldown() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.5);
        let entity = world.spawn((
            Transform::from_translation(DVec3::new(4999.0, 0.0, 0.0)),
            PreviousTranslation::default(),
            DesiredMovement::new(DVec3::X, 100.0),
            Cooldown {
                remaining_seconds: 1.0,
            },
        )).id();

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        use bevy_ecs::schedule::IntoScheduleConfigs;
        schedule.add_systems((entity_movement, entity_boundary_reflect, entity_cooldown).chain());
        schedule.run(&mut world);

        let mut q = world.query::<(&Transform, &DesiredMovement)>();
        let (t, dm) = q.single(&world).unwrap();
        assert!(t.translation.x <= 5000.0, "clamped: {}", t.translation.x);
        assert!(dm.direction().x < 0.0, "reflected: {}", dm.direction().x);
        let cd = world.get::<Cooldown>(entity).expect("Cooldown should still exist");
        assert!((cd.remaining_seconds - 0.5).abs() < 1e-5, "cooldown: {}", cd.remaining_seconds);
    }

    #[test]
    fn multiple_entities_move_independently() {
        let mut world = World::new();
        insert_test_time(&mut world, 0.05);
        world.spawn((
            Transform::default(),
            PreviousTranslation::default(),
            DesiredMovement::new(DVec3::X, 200.0),
        ));
        world.spawn((
            Transform::default(),
            PreviousTranslation::default(),
            DesiredMovement::new(DVec3::Y, 50.0),
        ));

        run_system(&mut world, entity_movement);

        let mut q = world.query::<&Transform>();
        let transforms: Vec<_> = q.iter(&world).collect();
        assert_eq!(transforms.len(), 2);

        let (t_x, t_y) = if transforms[0].translation.x > 1.0 {
            (transforms[0], transforms[1])
        } else {
            (transforms[1], transforms[0])
        };
        assert!((t_x.translation.x - 10.0).abs() < 1e-6);
        assert!((t_y.translation.y - 2.5).abs() < 1e-6);
    }
}
