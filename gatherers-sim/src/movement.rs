use crate::components::{Cooldown, DesiredMovement, Transform};
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
// Movement application (pos += vel * dt) is framework infrastructure,
// provided by bevy_mass::MovementPlugin in standalone mode and by
// UE's UMassApplyMovementProcessor in Unreal mode.
// Game code only sets desired_movement.velocity — it never applies position.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// System 4: Cooldown — decrement pickup cooldown timers
// Pure-Bevy component with add/remove semantics. In Unreal mode, Cooldown
// lives on shadow Bevy entities, not in chunk memory. The macro auto-detects
// this via QueryBackend::IS_CHUNK and dispatches to Bevy storage.
// ---------------------------------------------------------------------------

#[mass_system(order = 40)]
pub fn entity_cooldown(
    mut cooldowns: Query<(Entity, &mut Cooldown)>,
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

/// Reflect desired movement at simulation boundaries. Reads position to detect
/// boundary contact and reflects the desired velocity. No position writes —
/// movement application is handled by framework infrastructure.
#[mass_system(order = 50)]
pub fn entity_boundary_reflect(
    transforms: Query<&Transform>,
    mut movements: Query<&mut DesiredMovement>,
) {
    for (transform, mut movement) in transforms.iter().zip(movements.iter_mut()) {
        let inward_normal = compute_boundary_normal(transform.translation);
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
    use crate::components::{Cooldown, DesiredMovement, PreviousTranslation, Transform};
    use bevy_ecs::prelude::*;
    use bevy_mass::movement::apply_movement;
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
    fn boundary_reflect_reverses_velocity() {
        let mut world = World::new();
        world.spawn((
            Transform::from_translation(DVec3::new(6000.0, 0.0, 0.0)),
            DesiredMovement::new(DVec3::X, 100.0),
        ));

        run_system(&mut world, entity_boundary_reflect);

        let mut q = world.query::<(&Transform, &DesiredMovement)>();
        let (t, dm) = q.single(&world).unwrap();
        // Position unchanged — boundary_reflect only affects velocity
        assert!((t.translation.x - 6000.0).abs() < 1e-6, "position unchanged: {}", t.translation.x);
        assert!(dm.direction().x < 0.0, "velocity reflected: {}", dm.direction().x);
        assert!((dm.speed() - 100.0).abs() < 1e-4, "speed preserved: {}", dm.speed());
    }

    #[test]
    fn boundary_reflect_no_effect_inside_bounds() {
        let mut world = World::new();
        world.spawn((
            Transform::from_translation(DVec3::new(100.0, 200.0, 0.0)),
            DesiredMovement::new(DVec3::X, 100.0),
        ));

        run_system(&mut world, entity_boundary_reflect);

        let mut q = world.query::<&DesiredMovement>();
        let dm = q.single(&world).unwrap();
        assert!(dm.direction().x > 0.0, "velocity unchanged inside bounds");
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
        schedule.add_systems((
            apply_movement::<Transform, PreviousTranslation, DesiredMovement>,
            entity_boundary_reflect,
            entity_cooldown,
        ).chain());
        schedule.run(&mut world);

        let mut q = world.query::<&DesiredMovement>();
        let dm = q.single(&world).unwrap();
        assert!(dm.direction().x < 0.0, "reflected: {}", dm.direction().x);
        let cd = world.get::<Cooldown>(entity).expect("Cooldown should still exist");
        assert!((cd.remaining_seconds - 0.5).abs() < 1e-5, "cooldown: {}", cd.remaining_seconds);
    }
}
