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
//! #[mass_system(order = 10)]
//! fn my_system(mut things: Query<&mut MyFragment>, time: Res<Time>) {
//!     let dt = time.delta_secs();
//!     for thing in &mut things {
//!         thing.value += dt;
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
    pub use crate::time::Time;
    pub use crate::query::Query;
    pub use crate::query::BevyQuery;

    // Re-export Bevy essentials — these work identically in both backends
    pub use bevy_ecs::prelude::{Res, ResMut, Resource};
    pub use bevy_ecs::system::IntoSystem;
    pub use bevy_ecs::prelude::{Component, Entity, With, Without};
    pub use bevy_ecs::system::Commands;

    // Re-export glam types used in nearly every system
    pub use glam::DVec3;

    // mass_system attribute macro — available unconditionally.
    // In Bevy mode it's a no-op (passes through the original function).
    // In Unreal mode it generates chunk-based dispatch + C++ wrappers.
    pub use unreal_api_derive::mass_system;

    // In Unreal mode, re-export Unreal-specific query types
    #[cfg(feature = "unreal")]
    pub use unreal_api::mass::{MassQuery, MassQueryAll};
}

// Also export at crate root
pub use time::Time;
pub use query::Query;
pub use query::BevyQuery;

/// Define a MassFragment struct with correct attributes for both Bevy and Unreal modes.
///
/// Expands to `#[repr(C)]`, `#[derive(Component, Clone, Copy, Debug)]`, and conditionally
/// `#[derive(MassFragment)]` + `#[mass(cpp_type = "...")]` when the `unreal` feature is active.
///
/// ```ignore
/// mass_fragment!(cpp_type = "FGatherersMovement",
///     pub struct Movement {
///         #[cfg_attr(feature = "unreal", mass(default = "FVector(1.0f, 0.0f, 0.0f)"))]
///         pub direction: DVec3,
///         pub movement_speed: f32,
///     }
/// );
/// ```
#[macro_export]
macro_rules! mass_fragment {
    (cpp_type = $cpp_type:literal, $(#[$meta:meta])* $vis:vis struct $name:ident { $($body:tt)* }) => {
        #[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
        #[derive($crate::prelude::Component, Clone, Copy, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "unreal", mass(cpp_type = $cpp_type))]
        $(#[$meta])*
        $vis struct $name { $($body)* }
    };
}

/// Define a MassTag (zero-sized type) with correct attributes for both Bevy and Unreal modes.
///
/// Optionally specify `group = "name"` to associate this tag with an entity group
/// (used by `#[mass_system]` to look up shadow entities from `MassEntityMap`).
///
/// ```ignore
/// mass_tag!(cpp_type = "FGathersMassAntTag", group = "ants",
///     pub struct AntTag;
/// );
/// ```
#[macro_export]
macro_rules! mass_tag {
    (cpp_type = $cpp_type:literal, group = $group:literal, $(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        #[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
        #[cfg_attr(feature = "unreal", mass(cpp_type = $cpp_type, tag))]
        #[derive($crate::prelude::Component, Clone, Copy, Debug)]
        $(#[$meta])*
        $vis struct $name;

        impl $name {
            /// Entity group name for `MassEntityMap` lookup.
            pub const ENTITY_GROUP: &'static str = $group;
        }
    };
    (cpp_type = $cpp_type:literal, $(#[$meta:meta])* $vis:vis struct $name:ident;) => {
        #[cfg_attr(feature = "unreal", derive(unreal_api::MassFragment))]
        #[cfg_attr(feature = "unreal", mass(cpp_type = $cpp_type, tag))]
        #[derive($crate::prelude::Component, Clone, Copy, Debug)]
        $(#[$meta])*
        $vis struct $name;
    };
}

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

    fn ant_movement(mut ants: Query<&mut AntFragment>, time: Res<Time>) {
        let dt = time.delta_secs();
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
        let mut time = Time::<()>::default();
        time.advance_by(core::time::Duration::from_secs_f32(0.1));
        world.insert_resource(time);

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
        let mut time = Time::<()>::default();
        time.advance_by(core::time::Duration::from_secs_f32(0.1));
        world.insert_resource(time);
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
        let mut time = Time::<()>::default();
        time.advance_by(core::time::Duration::from_secs_f32(0.05));
        world.insert_resource(time);

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
