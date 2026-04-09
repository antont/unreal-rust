//! Facade crate for writing Bevy-compatible ECS systems that compile against
//! either pure Bevy or Unreal Mass Entity.
//!
//! # Feature flags
//!
//! - `bevy-backend` (default): Systems run on real Bevy entities and components.
//! - `unreal`: Systems run on Unreal Mass Entity chunk data via zero-copy pointers.
//!
//! Both backends use `bevy_ecs` for scheduling. The feature flag controls how
//! `Query<D>` resolves data access.
//!
//! # Usage
//!
//! ```ignore
//! use bevy_mass::prelude::*;
//!
//! fn my_system(mut things: Query<&mut MyFragment>, time: Res<DeltaTime>) {
//!     for thing in &mut things {
//!         thing.value += time.0;
//!     }
//! }
//! ```

// Ensure at least one backend is selected.
// When both are enabled (e.g., Cargo feature unification in a workspace),
// `unreal` takes precedence — the bevy-backend types are only used when
// `unreal` is NOT active.
#[cfg(not(any(feature = "bevy-backend", feature = "unreal")))]
compile_error!("Either feature `bevy-backend` or `unreal` must be enabled.");

mod time;
mod query;

pub mod prelude {
    pub use crate::time::DeltaTime;
    pub use crate::query::Query;

    // Re-export Bevy essentials — these work identically in both backends
    pub use bevy_ecs::prelude::{Res, ResMut, Resource};
    pub use bevy_ecs::system::IntoSystem;
    pub use bevy_ecs::prelude::Component;

    // In Unreal mode, re-export the mass_system attribute macro
    #[cfg(feature = "unreal")]
    pub use unreal_api::mass_system;
}

// Also export at crate root
pub use time::DeltaTime;
pub use query::Query;

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::prelude::*;
    use bevy_ecs::prelude::*;

    /// Minimal ant fragment — same fields as the real one, but derives Component
    /// for pure Bevy usage. In a real app this would use #[derive(Fragment)].
    #[derive(Component, Clone, Copy, Debug, Default)]
    struct AntFragment {
        position: [f64; 3],
        previous_position: [f64; 3],
        direction: [f64; 3],
        movement_speed: f32,
    }

    // -----------------------------------------------------------------------
    // This is the proof of concept: a system written in standard Bevy syntax
    // that runs on real Bevy entities. The same source code would compile
    // against the Unreal backend with #[cfg_attr(feature = "unreal", mass_system)].
    // -----------------------------------------------------------------------

    fn ant_movement(mut ants: Query<&mut AntFragment>, time: Res<DeltaTime>) {
        let dt = time.0;
        for mut ant in &mut ants {
            ant.previous_position = ant.position;

            let dir = ant.direction;
            let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
            if len < 1e-8 {
                continue;
            }
            let dir = [dir[0] / len, dir[1] / len, dir[2] / len];
            let step = (ant.movement_speed.max(0.0) * dt.max(0.0)) as f64;
            ant.position[0] += dir[0] * step;
            ant.position[1] += dir[1] * step;
            ant.position[2] += dir[2] * step;
        }
    }

    #[test]
    fn ant_movement_on_pure_bevy() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.1));

        // Spawn an ant moving in +X at speed 100
        world.spawn(AntFragment {
            position: [0.0, 0.0, 50.0],
            previous_position: [0.0, 0.0, 50.0],
            direction: [1.0, 0.0, 0.0],
            movement_speed: 100.0,
        });

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(ant_movement);
        schedule.run(&mut world);

        let mut query = world.query::<&AntFragment>();
        let ant = query.single(&world).unwrap();

        // Should have moved 100 * 0.1 = 10 units in +X
        assert!((ant.position[0] - 10.0).abs() < 1e-6, "x: {}", ant.position[0]);
        assert!(ant.position[1].abs() < 1e-6);
        assert!((ant.position[2] - 50.0).abs() < 1e-6);
        // previous_position should be the old position
        assert!(ant.previous_position[0].abs() < 1e-6);
    }

    #[test]
    fn ant_movement_zero_direction_does_not_move() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.1));
        world.spawn(AntFragment {
            position: [5.0, 5.0, 5.0],
            direction: [0.0, 0.0, 0.0],
            movement_speed: 100.0,
            ..Default::default()
        });

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(ant_movement);
        schedule.run(&mut world);

        let mut query = world.query::<&AntFragment>();
        let ant = query.single(&world).unwrap();

        // Position unchanged (direction is zero)
        assert!((ant.position[0] - 5.0).abs() < 1e-6);
        assert!((ant.position[1] - 5.0).abs() < 1e-6);
    }

    #[test]
    fn multiple_ants_move_independently() {
        let mut world = World::new();
        world.insert_resource(DeltaTime(0.05));

        world.spawn(AntFragment {
            direction: [1.0, 0.0, 0.0],
            movement_speed: 200.0,
            ..Default::default()
        });
        world.spawn(AntFragment {
            direction: [0.0, 1.0, 0.0],
            movement_speed: 50.0,
            ..Default::default()
        });

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(ant_movement);
        schedule.run(&mut world);

        let mut query = world.query::<&AntFragment>();
        let ants: Vec<_> = query.iter(&world).collect();
        assert_eq!(ants.len(), 2);

        // Ant 0: 200 * 0.05 = 10 in +X
        // Ant 1: 50 * 0.05 = 2.5 in +Y
        let (ant_x, ant_y) = if ants[0].position[0] > 1.0 {
            (ants[0], ants[1])
        } else {
            (ants[1], ants[0])
        };
        assert!((ant_x.position[0] - 10.0).abs() < 1e-6);
        assert!((ant_y.position[1] - 2.5).abs() < 1e-6);
    }
}
