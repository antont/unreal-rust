//! Bird wandering + flocking (Phase 2a — no hunting yet).
//!
//! `wander_system` mirrors `brownian_motion_system` but uses the `Wander`
//! component so birds and insects can be tuned independently.
//!
//! `flocking_system` runs in `SpatialGroupSet::Query` — the framework's
//! `SpatialGroupPlugin::<Bird, Transform>` installs the rebuild side, so
//! `SpatialQueries::neighbors_within("birds", ...)` is already populated
//! by the time flocking runs (Bevy mode) or resolved via UE's hash grid
//! (UE mode).
//!
//! Ported from vivarium commit 1b6d1f5 (`src/systems/{flocking,brownian}.rs`
//! + `src/spatial.rs`).

use crate::components::{Bird, Flocking, Transform, Velocity, Wander};
use crate::config::{
    ALIGNMENT_WEIGHT, BIRD_SPEED, COHESION_WEIGHT, FLOCK_NEIGHBOR_RADIUS, SEPARATION_DISTANCE,
    SEPARATION_WEIGHT,
};
use bevy_ecs::entity::EntityHashMap;
use bevy_mass::prelude::*;

#[mass_system]
pub fn wander_system(
    mut birds: Query<(&mut Velocity, &mut Wander), With<Bird>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs() as f64;
    for (mut velocity, mut wander) in &mut birds {
        let (px, py, pz) = next_perturbation(&mut wander.random_seed);
        let strength = wander.strength as f64;
        let perturbation = DVec3::new(px, py, pz) * strength * dt;

        let new_vel = velocity.value + perturbation;
        let len = new_vel.length();
        velocity.value = if len > 1e-8 {
            new_vel / len * BIRD_SPEED as f64
        } else {
            DVec3::ZERO
        };
    }
}

#[mass_system]
pub fn flocking_system(
    spatial: SpatialQueries,
    mut birds: Query<(Entity, &Transform, &mut Velocity, &Flocking), With<Bird>>,
) {
    // Snapshot all bird velocities so each bird sees a consistent frame
    // when reading its neighbours' headings (mirrors vivarium's approach).
    // `&mut birds` is the portable iteration form — the `#[mass_system]`
    // UE-mode `Query` doesn't expose `.iter()` / `&query`.
    let snapshots: EntityHashMap<DVec3> = (&mut birds)
        .into_iter()
        .map(|(e, _, v, _)| (e, v.value))
        .collect();

    for (entity, transform, mut velocity, flocking) in &mut birds {
        let pos = transform.translation;
        let mut separation = DVec3::ZERO;
        let mut alignment = DVec3::ZERO;
        let mut cohesion = DVec3::ZERO;
        let mut neighbor_count = 0u32;

        let neighbors =
            spatial.neighbors_within("birds", &pos, FLOCK_NEIGHBOR_RADIUS, Some(entity));
        for neighbor in &neighbors {
            let Some(other_vel) = snapshots.get(&neighbor.entity).copied() else {
                continue;
            };
            let diff = pos - neighbor.position;
            let dist = diff.length();
            if dist < f64::EPSILON {
                continue;
            }

            neighbor_count += 1;

            if dist < SEPARATION_DISTANCE {
                separation += diff.normalize() / dist;
            }
            alignment += other_vel;
            cohesion += neighbor.position;
        }

        if neighbor_count > 0 {
            let n = neighbor_count as f64;

            let alignment_avg = alignment / n;
            let alignment_steer = alignment_avg - velocity.value;

            let cohesion_avg = cohesion / n;
            let cohesion_steer = cohesion_avg - pos;
            let cohesion_dir = if cohesion_steer.length() > 1e-8 {
                cohesion_steer.normalize()
            } else {
                DVec3::ZERO
            };

            let steering = separation * SEPARATION_WEIGHT * flocking.separation_weight as f64
                + alignment_steer * ALIGNMENT_WEIGHT * flocking.alignment_weight as f64
                + cohesion_dir * COHESION_WEIGHT * flocking.cohesion_weight as f64;

            let new_vel = velocity.value + steering;
            let len = new_vel.length();
            velocity.value = if len > 1e-8 {
                new_vel / len * BIRD_SPEED as f64
            } else {
                DVec3::ZERO
            };
        }
    }
}

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
    use crate::components::PreviousTranslation;
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

    fn run_with_plugin<M, F>(setup: F, system: impl IntoSystem<(), (), M>) -> World
    where
        F: FnOnce(&mut World),
    {
        use bevy_app::App;
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
            "birds",
            FLOCK_NEIGHBOR_RADIUS,
        ));
        app.add_systems(bevy_app::Update, system.in_set(SpatialGroupSet::Query));
        setup(app.world_mut());
        app.update();
        // Extract world for assertions
        let mut temp_world = World::new();
        std::mem::swap(app.world_mut(), &mut temp_world);
        temp_world
    }

    #[test]
    fn wander_preserves_speed() {
        let mut world = World::new();
        insert_time(&mut world, 1.0 / 60.0);
        world.spawn((
            Bird,
            Transform::from_translation(DVec3::ZERO),
            PreviousTranslation::default(),
            Velocity::new(DVec3::X, BIRD_SPEED),
            Wander { strength: 15.0, random_seed: 42 },
        ));

        run(&mut world, wander_system);

        let mut q = world.query::<&Velocity>();
        let v = q.single(&world).unwrap();
        let speed = v.value.length() as f32;
        assert!(
            (speed - BIRD_SPEED).abs() < 1e-4,
            "speed drifted: got {}, expected {}",
            speed,
            BIRD_SPEED,
        );
    }

    /// Two-bird cohesion: A at origin, B at (10, 0, 0), both drifting +X.
    /// After one flocking tick with separation disabled, A should steer
    /// toward B — its velocity gains a +X cohesion pull.
    #[test]
    fn flocking_cohesion_pulls_toward_centroid() {
        let zero_sep_flocking = Flocking {
            separation_weight: 0.0,
            alignment_weight: 0.0,
            cohesion_weight: 1.0,
        };

        let mut entity_a = None;
        let world = run_with_plugin(
            |w| {
                // Distance 20 > SEPARATION_DISTANCE (10) so no separation kicks in.
                let a = w
                    .spawn((
                        Bird,
                        Transform::from_translation(DVec3::ZERO),
                        PreviousTranslation::default(),
                        Velocity::new(DVec3::X, BIRD_SPEED),
                        zero_sep_flocking,
                    ))
                    .id();
                w.spawn((
                    Bird,
                    Transform::from_translation(DVec3::new(20.0, 0.0, 0.0)),
                    PreviousTranslation::default(),
                    Velocity::new(DVec3::X, BIRD_SPEED),
                    zero_sep_flocking,
                ));
                entity_a = Some(a);
            },
            flocking_system,
        );

        let a_vel = world.entity(entity_a.unwrap()).get::<Velocity>().unwrap().value;
        // Speed preserved
        assert!((a_vel.length() - BIRD_SPEED as f64).abs() < 1e-4);
        // Y and Z components remain zero (pure +X setup)
        assert!(a_vel.y.abs() < 1e-6 && a_vel.z.abs() < 1e-6);
        // Direction still points +X (both birds already moving +X and B is +X of A)
        assert!(a_vel.x > 0.0);
    }

    /// Two birds within SEPARATION_DISTANCE should steer apart. A at
    /// origin, B at (5, 0, 0) — A's velocity should gain a −X component.
    #[test]
    fn flocking_separation_pushes_apart() {
        let sep_only = Flocking {
            separation_weight: 1.0,
            alignment_weight: 0.0,
            cohesion_weight: 0.0,
        };

        let mut entity_a = None;
        let world = run_with_plugin(
            |w| {
                let a = w
                    .spawn((
                        Bird,
                        Transform::from_translation(DVec3::ZERO),
                        PreviousTranslation::default(),
                        // Give A a small +Y velocity so separation along −X can be
                        // detected as a change in X direction.
                        Velocity::new(DVec3::Y, BIRD_SPEED),
                        sep_only,
                    ))
                    .id();
                // Distance 5 < SEPARATION_DISTANCE (10).
                w.spawn((
                    Bird,
                    Transform::from_translation(DVec3::new(5.0, 0.0, 0.0)),
                    PreviousTranslation::default(),
                    Velocity::new(DVec3::Y, BIRD_SPEED),
                    sep_only,
                ));
                entity_a = Some(a);
            },
            flocking_system,
        );

        let a_vel = world.entity(entity_a.unwrap()).get::<Velocity>().unwrap().value;
        assert!(
            a_vel.x < 0.0,
            "A should be pushed in −X away from B at (5,0,0), got vx={}",
            a_vel.x,
        );
    }

    /// Alignment averages neighbour headings. A moves +X, B moves +Y —
    /// A's new direction should tilt toward +Y.
    #[test]
    fn flocking_alignment_averages_headings() {
        let align_only = Flocking {
            separation_weight: 0.0,
            alignment_weight: 1.0,
            cohesion_weight: 0.0,
        };

        let mut entity_a = None;
        let world = run_with_plugin(
            |w| {
                let a = w
                    .spawn((
                        Bird,
                        Transform::from_translation(DVec3::ZERO),
                        PreviousTranslation::default(),
                        Velocity::new(DVec3::X, BIRD_SPEED),
                        align_only,
                    ))
                    .id();
                // Distance 20 > SEPARATION_DISTANCE so no separation interferes.
                w.spawn((
                    Bird,
                    Transform::from_translation(DVec3::new(20.0, 0.0, 0.0)),
                    PreviousTranslation::default(),
                    Velocity::new(DVec3::Y, BIRD_SPEED),
                    align_only,
                ));
                entity_a = Some(a);
            },
            flocking_system,
        );

        let a_vel = world.entity(entity_a.unwrap()).get::<Velocity>().unwrap().value;
        assert!(
            a_vel.y > 0.0,
            "A should tilt toward B's +Y heading, got vy={}",
            a_vel.y,
        );
        // Speed still at BIRD_SPEED
        assert!((a_vel.length() - BIRD_SPEED as f64).abs() < 1e-4);
    }

    #[test]
    fn flocking_no_neighbors_no_change() {
        let flocking = Flocking {
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let initial = DVec3::new(1.0, 0.0, 0.0) * BIRD_SPEED as f64;
        let mut entity_a = None;
        let world = run_with_plugin(
            |w| {
                let a = w
                    .spawn((
                        Bird,
                        Transform::from_translation(DVec3::ZERO),
                        PreviousTranslation::default(),
                        Velocity::new(DVec3::X, BIRD_SPEED),
                        flocking,
                    ))
                    .id();
                entity_a = Some(a);
            },
            flocking_system,
        );

        let a_vel = world.entity(entity_a.unwrap()).get::<Velocity>().unwrap().value;
        assert!(
            (a_vel - initial).length() < 1e-6,
            "lone bird should not be steered: {:?}",
            a_vel,
        );
    }
}
