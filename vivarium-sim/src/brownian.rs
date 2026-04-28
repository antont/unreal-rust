//! Brownian-motion wandering for insects.
//!
//! Each frame, perturbs `Velocity` by a random per-axis offset, then
//! renormalizes to `INSECT_SPEED`. Per-entity LCG seed keeps the RNG
//! archetype-parallel safe (no shared RNG state, no `rand` crate).
//!
//! Ported from vivarium commit 762c98b (`src/systems/brownian.rs`).

use crate::components::{BrownianMotion, Insect, Velocity};
use crate::config::INSECT_SPEED;
use bevy_mass::prelude::*;

#[mass_system]
pub fn brownian_motion_system(
    mut insects: Query<(&mut Velocity, &mut BrownianMotion), With<Insect>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs() as f64;
    for (mut velocity, mut brownian) in &mut insects {
        let (px, py, pz) = next_perturbation(&mut brownian.random_seed);
        let strength = brownian.wander_strength as f64;
        let perturbation = DVec3::new(px, py, pz) * strength * dt;

        let new_vel = velocity.value + perturbation;
        let len = new_vel.length();
        velocity.value = if len > 1e-8 {
            new_vel / len * INSECT_SPEED as f64
        } else {
            DVec3::ZERO
        };
    }
}

/// Advance a per-entity LCG seed and return three independent samples in
/// `[-1, 1]`. Shape matches gatherers' FRandomStream mapping so the
/// stream is portable C++ ↔ Rust.
fn next_perturbation(seed: &mut u32) -> (f64, f64, f64) {
    let x = next_sample(seed);
    let y = next_sample(seed);
    let z = next_sample(seed);
    (x, y, z)
}

fn next_sample(seed: &mut u32) -> f64 {
    *seed = seed.wrapping_mul(196314165).wrapping_add(907633515);
    ((*seed & 0x7fffff) as f64 / 8388607.0) * 2.0 - 1.0
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::components::{PreviousTranslation, Transform};
    use bevy_ecs::prelude::*;
    use core::time::Duration;

    fn insert_time(world: &mut World, dt: f32) {
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(dt));
        world.insert_resource(time);
    }

    fn run<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(system);
        schedule.run(world);
    }

    #[test]
    fn preserves_speed_after_one_step() {
        let mut world = World::new();
        insert_time(&mut world, 1.0 / 60.0);
        world.spawn((
            Insect,
            Transform::from_translation(DVec3::ZERO),
            PreviousTranslation::default(),
            Velocity::new(DVec3::new(1.0, 0.0, 0.0), INSECT_SPEED),
            BrownianMotion { wander_strength: 5.0, random_seed: 42 },
        ));

        run(&mut world, brownian_motion_system);

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        let speed = v.value.length() as f32;
        assert!(
            (speed - INSECT_SPEED).abs() < 1e-4,
            "speed drifted: got {}, expected {}",
            speed,
            INSECT_SPEED,
        );
    }

    #[test]
    fn direction_actually_changes() {
        let mut world = World::new();
        insert_time(&mut world, 1.0 / 60.0);
        let initial = DVec3::new(1.0, 0.0, 0.0);
        world.spawn((
            Insect,
            Transform::from_translation(DVec3::ZERO),
            PreviousTranslation::default(),
            Velocity::new(initial, INSECT_SPEED),
            BrownianMotion { wander_strength: 50.0, random_seed: 123 },
        ));

        run(&mut world, brownian_motion_system);

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        let dir = v.value.normalize();
        assert!(
            dir.distance(initial) > 1e-6,
            "direction did not change: {:?}",
            dir,
        );
    }

    #[test]
    fn per_entity_seed_advances() {
        let mut seed = 42u32;
        let a = next_sample(&mut seed);
        let b = next_sample(&mut seed);
        assert_ne!(a, b, "successive LCG samples should differ");
        assert!((-1.0..=1.0).contains(&a));
        assert!((-1.0..=1.0).contains(&b));
    }

    #[test]
    fn deterministic_from_seed() {
        let mut seed_a = 7u32;
        let mut seed_b = 7u32;
        let a = next_sample(&mut seed_a);
        let b = next_sample(&mut seed_b);
        assert_eq!(a, b, "same seed must produce same sample");
    }

    /// After many frames, an insect's direction should have drifted
    /// measurably from its initial axis. Guards against a regression
    /// where brownian silently becomes a no-op.
    #[test]
    fn direction_drifts_over_many_frames() {
        let mut world = World::new();
        insert_time(&mut world, 1.0 / 60.0);
        let initial = DVec3::new(1.0, 0.0, 0.0);
        world.spawn((
            Insect,
            Transform::from_translation(DVec3::ZERO),
            PreviousTranslation::default(),
            Velocity::new(initial, INSECT_SPEED),
            BrownianMotion { wander_strength: 5.0, random_seed: 42 },
        ));

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(brownian_motion_system);
        for _ in 0..180 {
            schedule.run(&mut world);
        }

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        let dir = v.value.normalize();
        // `wander_strength=5` matches vivarium commit 43dd9df. After 180
        // frames (3s) the direction drifts roughly 1° off — tiny but
        // clearly non-zero. This guard catches a regression where brownian
        // silently becomes a no-op. Threshold set well below the observed
        // drift (~0.017 radians) but well above float noise.
        let drift = dir.distance(initial);
        assert!(
            drift > 0.005,
            "direction barely drifted after 180 frames: {:?} (distance {})",
            dir,
            drift,
        );
    }
}
