//! Spatial query facade — wraps UE's `MassSpatialQueries` in Unreal mode.
//!
//! Game systems take a single `SpatialQueries` `SystemParam` and call
//! `.call(name, prev, curr)` — the `MassEntityMap` borrow is stitched in
//! internally, so game code never names it. Typical usage:
//!
//! ```ignore
//! fn ant_collision_prepass(
//!     ants: Query<(Entity, &Transform, &PreviousTranslation), With<Ant>>,
//!     spatial: SpatialQueries,
//!     mut hits: MessageWriter<AntFoodHit>,
//! ) {
//!     for (entity, t, prev) in &ants {
//!         if let Some(hit) = spatial.call("food_pickup", &prev.value, &t.translation) {
//!             hits.write(AntFoodHit::new(hit.entity_index, hit.entity, entity, hit.position));
//!         }
//!     }
//! }
//! ```
//!
//! The lower-level `SpatialQuery` resource + `with_map()` helper is still
//! public so the frame-dispatch code in `unreal-module` can install
//! per-frame sweep callbacks.
//!
//! In Bevy mode, `SpatialQuery` / `SpatialQueries` are stubs — collision
//! detection uses direct Bevy queries instead. The types exist so code
//! referencing them compiles in both modes.

use glam::DVec3;

/// Result of a spatial query sweep.
#[derive(Debug, Clone, Copy)]
pub struct SpatialHit {
    /// Index of the nearest matching entity (spawn-order index within the
    /// query's target group). Still carried alongside `entity` because
    /// downstream `#[repr(C)]` payloads like `FoodEncounter` need the i32.
    pub entity_index: i32,
    /// Shadow Bevy `Entity` resolved via `MassEntityMap` at the query
    /// boundary. `Entity::PLACEHOLDER` in Bevy mode (stub) or if the
    /// query's config is missing / the index is out of range.
    pub entity: bevy_ecs::entity::Entity,
    /// Position where the encounter occurred.
    pub position: DVec3,
}

// ---------------------------------------------------------------------------
// Bevy mode — stub (collision uses direct queries, not this resource)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "unreal"))]
mod bevy_impl {
    use super::*;
    use bevy_ecs::prelude::Resource;

    /// Stub spatial query resource for Bevy mode.
    ///
    /// In Bevy mode, collision detection uses direct entity queries rather
    /// than an intermediate spatial query resource. This stub exists so code
    /// that references `SpatialQuery` compiles in both modes.
    #[derive(Resource, Default)]
    pub struct SpatialQuery;

    /// Stub resolver — mirrors the Unreal-mode signature so call sites
    /// compile unchanged. Always returns `None`.
    pub struct SpatialQueryWithMap<'a> {
        _inner: &'a (),
    }

    impl SpatialQuery {
        /// Returns a stub that yields no hits; matches the Unreal-mode shape
        /// so call sites using `with_map` compile in both backends.
        pub fn with_map<'a, M: ?Sized>(&'a self, _map: &'a M) -> SpatialQueryWithMap<'a> {
            SpatialQueryWithMap { _inner: &() }
        }
    }

    impl<'a> SpatialQueryWithMap<'a> {
        pub fn call(&self, _name: &str, _prev: &DVec3, _curr: &DVec3) -> Option<SpatialHit> {
            None
        }
    }

    /// Stub `SystemParam` — mirrors the Unreal-mode signature so systems
    /// using `SpatialQueries` compile cleanly in Bevy mode. Always yields
    /// `None` from `call`. The `Local` anchor gives the derive a real
    /// param to work with (empty SystemParam structs aren't supported).
    #[derive(bevy_ecs::system::SystemParam)]
    pub struct SpatialQueries<'s> {
        _anchor: bevy_ecs::system::Local<'s, ()>,
    }

    impl<'s> SpatialQueries<'s> {
        pub fn call(&self, _name: &str, _prev: &DVec3, _curr: &DVec3) -> Option<SpatialHit> {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Unreal mode — wraps MassSpatialQueries + resolves Entity via MassEntityMap
// ---------------------------------------------------------------------------

#[cfg(feature = "unreal")]
mod unreal_impl {
    use super::*;
    use bevy_ecs::prelude::Resource;
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};
    use std::sync::OnceLock;
    use unreal_api::mass::{MassEntityMap, MassSpatialQueries, registered_spatial_query_configs};

    /// Spatial query resource — Unreal mode.
    ///
    /// Wraps `MassSpatialQueries` (C++ physics sweep callbacks populated per-frame).
    /// `Deref`/`DerefMut` to `MassSpatialQueries` allows the frame dispatch code
    /// to use `clear()` and `insert()` without changes.
    #[derive(Resource, Default)]
    pub struct SpatialQuery {
        inner: MassSpatialQueries,
    }

    /// Short-lived helper returned by [`SpatialQuery::with_map`]. Holds a
    /// borrow of the entity map so `call` can resolve each hit's
    /// `entity_index` into a shadow Bevy `Entity` without game code
    /// touching the map.
    pub struct SpatialQueryWithMap<'a> {
        spatial: &'a SpatialQuery,
        map: &'a MassEntityMap,
    }

    impl SpatialQuery {
        /// Borrow the `MassEntityMap` for the duration of a system's
        /// spatial-query loop. The returned helper's `call` returns
        /// `SpatialHit` with `entity` already resolved.
        pub fn with_map<'a>(&'a self, map: &'a MassEntityMap) -> SpatialQueryWithMap<'a> {
            SpatialQueryWithMap { spatial: self, map }
        }
    }

    impl<'a> SpatialQueryWithMap<'a> {
        /// Perform the named spatial query. `entity_index` is resolved to
        /// `Entity` via the config's `query_group` + `MassEntityMap`. If
        /// the config lookup or the entity lookup misses, `entity` is
        /// `Entity::PLACEHOLDER` and a warning is logged once per name.
        pub fn call(&self, name: &str, prev: &DVec3, curr: &DVec3) -> Option<SpatialHit> {
            let prev_arr: [f64; 3] = [prev.x, prev.y, prev.z];
            let curr_arr: [f64; 3] = [curr.x, curr.y, curr.z];
            let result = self.spatial.inner.call(name, &prev_arr, &curr_arr)?;
            if !result.has_encounter {
                return None;
            }
            let entity = resolve_hit_entity(name, result.entity_index, self.map);
            Some(SpatialHit {
                entity_index: result.entity_index,
                entity,
                position: DVec3::new(
                    result.encounter_position[0],
                    result.encounter_position[1],
                    result.encounter_position[2],
                ),
            })
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

    /// `query_name -> query_group` cache, built once from the static
    /// `MassSpatialQueryConfigRegistration` inventory.
    fn name_to_group() -> &'static HashMap<&'static str, &'static str> {
        static CACHE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
        CACHE.get_or_init(|| {
            let mut m = HashMap::new();
            for cfg in registered_spatial_query_configs() {
                m.insert(cfg.query_name, cfg.query_group);
            }
            m
        })
    }

    fn resolve_hit_entity(
        name: &str,
        index: i32,
        map: &MassEntityMap,
    ) -> bevy_ecs::entity::Entity {
        let Some(group) = name_to_group().get(name) else {
            log_missing_config(name);
            return bevy_ecs::entity::Entity::PLACEHOLDER;
        };
        map.get(group, index as usize)
            .unwrap_or(bevy_ecs::entity::Entity::PLACEHOLDER)
    }

    fn log_missing_config(name: &str) {
        use std::sync::Mutex;
        static SEEN: OnceLock<Mutex<std::collections::HashSet<String>>> = OnceLock::new();
        let seen = SEEN.get_or_init(|| Mutex::new(std::collections::HashSet::new()));
        let mut set = seen.lock().expect("SEEN poisoned");
        if set.insert(name.to_string()) {
            eprintln!(
                "SpatialQuery::call: no MassSpatialQueryConfigRegistration for query_name='{}'; returning Entity::PLACEHOLDER",
                name
            );
        }
    }

    /// Idiomatic `SystemParam` facade that bundles `Res<SpatialQuery>` +
    /// `Res<MassEntityMap>` behind a single `spatial: SpatialQueries`
    /// parameter. Game systems never have to name `MassEntityMap` —
    /// `call()` stitches the borrow in internally and returns a
    /// fully-resolved `SpatialHit`.
    #[derive(bevy_ecs::system::SystemParam)]
    pub struct SpatialQueries<'w> {
        spatial: bevy_ecs::system::Res<'w, SpatialQuery>,
        map: bevy_ecs::system::Res<'w, MassEntityMap>,
    }

    impl<'w> SpatialQueries<'w> {
        /// Perform the named spatial query. See
        /// [`SpatialQueryWithMap::call`] for the full contract.
        pub fn call(&self, name: &str, prev: &DVec3, curr: &DVec3) -> Option<SpatialHit> {
            self.spatial.with_map(&self.map).call(name, prev, curr)
        }
    }
}

#[cfg(not(feature = "unreal"))]
pub use bevy_impl::{SpatialQueries, SpatialQuery, SpatialQueryWithMap};
#[cfg(feature = "unreal")]
pub use unreal_impl::{SpatialQueries, SpatialQuery, SpatialQueryWithMap};
