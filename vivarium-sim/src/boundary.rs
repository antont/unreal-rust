//! Boundary force field for insects.
//!
//! Quadratic per-axis repulsion when an entity is within `BOUNDARY_MARGIN`
//! of the world edge. Hard clamp at `SimBounds` as a safety net.
//!
//! Ported from vivarium commit 43dd9df (`src/systems/boundary.rs`).

use crate::components::{BoundaryWrap, SimBounds, Transform, Velocity};
use crate::config::{BOUNDARY_FORCE, BOUNDARY_MARGIN};
use bevy_mass::prelude::*;

#[mass_system]
pub fn boundary_force_system(
    mut entities: Query<(&mut Transform, &mut Velocity), With<BoundaryWrap>>,
    bounds: Res<SimBounds>,
) {
    for (mut transform, mut velocity) in &mut entities {
        let pos = transform.translation;
        let steer = compute_steer(pos, &bounds);
        if steer != DVec3::ZERO {
            velocity.value += steer;
        }
        transform.translation = pos.clamp(bounds.min, bounds.max);
    }
}

/// Quadratic repulsion force: zero inside `[min+margin, max-margin]`,
/// ramps to `±BOUNDARY_FORCE` at the outer edge.
fn compute_steer(pos: DVec3, bounds: &SimBounds) -> DVec3 {
    let margin = BOUNDARY_MARGIN;
    let strength = BOUNDARY_FORCE;
    let mut steer = DVec3::ZERO;
    for axis in 0..3 {
        let p = pos[axis];
        let lo = bounds.min[axis];
        let hi = bounds.max[axis];
        let inner_lo = lo + margin;
        let inner_hi = hi - margin;
        if p > inner_hi {
            let t = ((p - inner_hi) / margin).min(1.0);
            steer[axis] -= strength * t * t;
        } else if p < inner_lo {
            let t = ((inner_lo - p) / margin).min(1.0);
            steer[axis] += strength * t * t;
        }
    }
    steer
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{BrownianMotion, Insect, PreviousTranslation};
    use crate::config::{INSECT_SPEED, WORLD_HALF_SIZE};
    use bevy_ecs::prelude::*;

    fn run<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(system);
        schedule.run(world);
    }

    fn default_bounds() -> SimBounds {
        SimBounds::default()
    }

    #[test]
    fn no_force_well_inside_bounds() {
        let mut world = World::new();
        world.insert_resource(default_bounds());
        let initial_vel = DVec3::new(INSECT_SPEED as f64, 0.0, 0.0);
        world.spawn((
            BoundaryWrap,
            Transform::from_translation(DVec3::ZERO),
            PreviousTranslation::default(),
            Velocity::new(initial_vel / initial_vel.length(), INSECT_SPEED),
        ));

        run(&mut world, boundary_force_system);

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        assert_eq!(v.value, initial_vel, "interior entity should not be steered");
    }

    /// Entity inside the margin on +X gets a negative-X push.
    #[test]
    fn pushes_inward_when_inside_margin() {
        let mut world = World::new();
        world.insert_resource(default_bounds());
        // Just inside the +X edge — halfway through the margin band.
        let pos = DVec3::new(WORLD_HALF_SIZE - BOUNDARY_MARGIN / 2.0, 0.0, 0.0);
        world.spawn((
            BoundaryWrap,
            Transform::from_translation(pos),
            PreviousTranslation::default(),
            Velocity::new(DVec3::new(1.0, 0.0, 0.0), INSECT_SPEED),
        ));

        run(&mut world, boundary_force_system);

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        assert!(
            v.value.x < INSECT_SPEED as f64,
            "expected inward (−X) push, got vx={}",
            v.value.x,
        );
    }

    /// Force magnitude grows quadratically as the entity approaches the edge.
    #[test]
    fn force_grows_toward_edge() {
        let bounds = default_bounds();
        // 1/3 of the way through the margin.
        let near_inner = DVec3::new(
            WORLD_HALF_SIZE - BOUNDARY_MARGIN * 2.0 / 3.0,
            0.0,
            0.0,
        );
        // 2/3 of the way through — closer to the edge.
        let near_edge = DVec3::new(
            WORLD_HALF_SIZE - BOUNDARY_MARGIN / 3.0,
            0.0,
            0.0,
        );
        let steer_inner = compute_steer(near_inner, &bounds);
        let steer_edge = compute_steer(near_edge, &bounds);
        assert!(
            steer_edge.x < steer_inner.x,
            "force should be stronger closer to the edge: inner={}, edge={}",
            steer_inner.x,
            steer_edge.x,
        );
    }

    /// Integration test: run movement + brownian + boundary for many
    /// frames and assert all insects stay in bounds. Catches regressions
    /// where the force is too weak to overpower outward velocities.
    #[test]
    fn insects_stay_in_bounds_over_time() {
        use crate::brownian::brownian_motion_system;
        use bevy_mass::movement::apply_movement;
        use core::time::Duration;

        let mut world = World::new();
        world.insert_resource(default_bounds());
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(1.0 / 60.0));
        world.insert_resource(time);

        // Spawn along the +X edge (inside the margin) heading outward —
        // boundary force must pull them back before they escape.
        for i in 0..16 {
            let seed = 0x9E3779B1u32.wrapping_add(i);
            let pos = DVec3::new(WORLD_HALF_SIZE - BOUNDARY_MARGIN * 0.5, 0.0, 0.0);
            world.spawn((
                Insect,
                BoundaryWrap,
                Transform::from_translation(pos),
                PreviousTranslation { value: pos },
                Velocity::new(DVec3::new(1.0, 0.0, 0.0), INSECT_SPEED),
                BrownianMotion { wander_strength: 5.0, random_seed: seed },
            ));
        }

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems((
            brownian_motion_system,
            boundary_force_system,
            apply_movement::<Transform, PreviousTranslation, Velocity>,
        ).chain());

        for _ in 0..600 {
            schedule.run(&mut world);
        }

        let bounds = *world.resource::<SimBounds>();
        let mut q = world.query::<&Transform>();
        for t in q.iter(&world) {
            assert!(
                t.translation.x <= bounds.max.x + 1e-9
                    && t.translation.x >= bounds.min.x - 1e-9,
                "escaped on X: {:?}",
                t.translation,
            );
        }
    }

    /// Position past the hard boundary gets clamped back inside.
    #[test]
    fn clamps_position_to_bounds() {
        let mut world = World::new();
        world.insert_resource(default_bounds());
        let outside = DVec3::new(WORLD_HALF_SIZE + 50.0, 0.0, 0.0);
        world.spawn((
            BoundaryWrap,
            Transform::from_translation(outside),
            PreviousTranslation::default(),
            Velocity::new(DVec3::new(1.0, 0.0, 0.0), INSECT_SPEED),
        ));

        run(&mut world, boundary_force_system);

        let mut q = world.query::<&Transform>();
        let t = q.single(&world).unwrap();
        assert!(
            t.translation.x <= WORLD_HALF_SIZE + 1e-9,
            "position not clamped: {:?}",
            t.translation,
        );
    }
}
