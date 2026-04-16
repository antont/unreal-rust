//! Spatial query facade — wraps UE's `MassSpatialQueries` in Unreal mode.
//!
//! In Unreal mode, game systems call `spatial.call("food_pickup", &prev, &curr)`
//! to access C++ physics sweep results. The `SpatialQuery` resource wraps
//! `MassSpatialQueries` via `Deref`/`DerefMut`, so the frame dispatch code
//! can use `clear()` and `insert()` unchanged.
//!
//! In Bevy mode, `SpatialQuery` is not needed — collision detection uses direct
//! Bevy queries instead. The type is still available (as an empty resource) so
//! that code referencing it compiles in both modes, but it has no functionality.

use glam::DVec3;

/// Result of a spatial query sweep.
#[derive(Debug, Clone, Copy)]
pub struct SpatialHit {
    /// Index of the nearest matching entity (spawn-order index).
    pub entity_index: i32,
    /// Position where the encounter occurred.
    pub position: DVec3,
}

// ---------------------------------------------------------------------------
// Bevy mode — stub (collision uses direct queries, not this resource)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "unreal"))]
mod bevy_impl {
    use bevy_ecs::prelude::Resource;

    /// Stub spatial query resource for Bevy mode.
    ///
    /// In Bevy mode, collision detection uses direct entity queries rather than
    /// an intermediate spatial query resource. This stub exists so code that
    /// references `SpatialQuery` compiles in both modes.
    #[derive(Resource, Default)]
    pub struct SpatialQuery;
}

// ---------------------------------------------------------------------------
// Unreal mode — wraps MassSpatialQueries
// ---------------------------------------------------------------------------

#[cfg(feature = "unreal")]
mod unreal_impl {
    use super::*;
    use bevy_ecs::prelude::Resource;
    use std::ops::{Deref, DerefMut};
    use unreal_api::mass::MassSpatialQueries;

    /// Spatial query resource — Unreal mode.
    ///
    /// Wraps `MassSpatialQueries` (C++ physics sweep callbacks populated per-frame).
    /// `Deref`/`DerefMut` to `MassSpatialQueries` allows the frame dispatch code
    /// to use `clear()` and `insert()` without changes.
    #[derive(Resource, Default)]
    pub struct SpatialQuery {
        inner: MassSpatialQueries,
    }

    impl SpatialQuery {
        /// Find the nearest entity via C++ physics sweep.
        pub fn call(&self, name: &str, prev: &DVec3, curr: &DVec3) -> Option<SpatialHit> {
            let prev_arr: [f64; 3] = [prev.x, prev.y, prev.z];
            let curr_arr: [f64; 3] = [curr.x, curr.y, curr.z];
            let result = self.inner.call(name, &prev_arr, &curr_arr)?;
            if result.has_encounter {
                Some(SpatialHit {
                    entity_index: result.entity_index,
                    position: DVec3::new(
                        result.encounter_position[0],
                        result.encounter_position[1],
                        result.encounter_position[2],
                    ),
                })
            } else {
                None
            }
        }
    }

    impl Deref for SpatialQuery {
        type Target = MassSpatialQueries;
        fn deref(&self) -> &MassSpatialQueries {
            &self.inner
        }
    }

    impl DerefMut for SpatialQuery {
        fn deref_mut(&mut self) -> &mut MassSpatialQueries {
            &mut self.inner
        }
    }
}

#[cfg(not(feature = "unreal"))]
pub use bevy_impl::SpatialQuery;
#[cfg(feature = "unreal")]
pub use unreal_impl::SpatialQuery;
