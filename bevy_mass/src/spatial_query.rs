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
//! Two query shapes are exposed:
//!
//! - [`SpatialQueries::call`] — single-hit sweep (UE physics / nav-hash
//!   grid). Returns the closest encounter along a prev → curr segment.
//! - [`SpatialQueries::neighbors_within`] — enumerate all members of a
//!   group within `radius` of a center point. Backed by an owned cell
//!   grid in Bevy mode and (future) UE hash-grid enumeration in Unreal
//!   mode. Used by behaviours like flocking that need the full
//!   neighbourhood rather than just the nearest.
//!
//! In Bevy mode, `SpatialQueries::call` stays a stub (collision uses
//! direct Bevy queries), but `neighbors_within` is backed by a real
//! [`SpatialGrids`] resource — rebuilt per frame by the sim — so Bevy
//! harnesses can do flocking without UE.

use bevy_ecs::entity::Entity;
use glam::DVec3;

pub use grids::{SpatialGrids, SpatialNeighbor};

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
// SpatialGrids — owned cell-grid resource, shared by both backends.
//
// Backend-independent: UE has its own hash grid (FNavigationObstacleHashGrid2D)
// but the `neighbors_within` enumeration surface needs a Rust-side structure
// the facade can query. The same resource doubles as the Bevy-mode backing
// store. In UE mode it remains empty until the enumerate-in-radius FFI
// wrapper is added (Phase 2a follow-up).
// ---------------------------------------------------------------------------
mod grids {
    use super::*;
    use bevy_ecs::prelude::Resource;
    use std::collections::HashMap;

    /// One neighbour returned by `SpatialQueries::neighbors_within`.
    #[derive(Debug, Clone, Copy)]
    pub struct SpatialNeighbor {
        /// Other entity's Bevy `Entity`.
        pub entity: Entity,
        /// Other entity's current translation.
        pub position: DVec3,
    }

    /// 3D cell grid backing a single named group (e.g. "birds").
    ///
    /// Caller rebuilds per frame via `clear` + `insert` before the first
    /// `neighbors_within` call, typical pattern:
    ///
    /// ```ignore
    /// fn rebuild_bird_grid(
    ///     mut grids: ResMut<SpatialGrids>,
    ///     birds: Query<(Entity, &Transform), With<Bird>>,
    /// ) {
    ///     let grid = grids.grid_mut("birds", FLOCK_NEIGHBOR_RADIUS);
    ///     grid.clear();
    ///     for (e, t) in &birds {
    ///         grid.insert(e, t.translation);
    ///     }
    /// }
    /// ```
    #[derive(Default)]
    pub struct SpatialGrid {
        cell_size: f64,
        map: HashMap<(i32, i32, i32), Vec<SpatialNeighbor>>,
    }

    impl SpatialGrid {
        pub fn new(cell_size: f64) -> Self {
            Self {
                cell_size: cell_size.max(1e-6),
                map: HashMap::new(),
            }
        }

        /// Cell size used when bucketing positions. Setting this rebuilds
        /// the bucket index on next `insert`; callers should `clear` first.
        pub fn set_cell_size(&mut self, cell_size: f64) {
            self.cell_size = cell_size.max(1e-6);
        }

        pub fn clear(&mut self) {
            for bucket in self.map.values_mut() {
                bucket.clear();
            }
        }

        pub fn insert(&mut self, entity: Entity, position: DVec3) {
            let key = self.cell_key(position);
            self.map
                .entry(key)
                .or_default()
                .push(SpatialNeighbor { entity, position });
        }

        fn cell_key(&self, pos: DVec3) -> (i32, i32, i32) {
            (
                (pos.x / self.cell_size).floor() as i32,
                (pos.y / self.cell_size).floor() as i32,
                (pos.z / self.cell_size).floor() as i32,
            )
        }

        /// Enumerate all entries within `radius` of `center`, excluding any
        /// entry matching `exclude` (typically the querying entity itself).
        /// Scans the 3×3×3 cell neighbourhood — correct iff `cell_size >=
        /// radius`, which `grid_mut` enforces.
        pub fn neighbors_within(
            &self,
            center: DVec3,
            radius: f64,
            exclude: Option<Entity>,
        ) -> Vec<SpatialNeighbor> {
            let r2 = radius * radius;
            let c = self.cell_key(center);
            let mut out = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let key = (c.0 + dx, c.1 + dy, c.2 + dz);
                        let Some(bucket) = self.map.get(&key) else {
                            continue;
                        };
                        for entry in bucket {
                            if Some(entry.entity) == exclude {
                                continue;
                            }
                            let d = entry.position - center;
                            if d.length_squared() <= r2 {
                                out.push(*entry);
                            }
                        }
                    }
                }
            }
            out
        }
    }

    /// Collection of named per-group `SpatialGrid`s. One resource covers
    /// every group the sim does `neighbors_within` on.
    #[derive(Resource, Default)]
    pub struct SpatialGrids {
        grids: HashMap<String, SpatialGrid>,
    }

    impl SpatialGrids {
        /// Get-or-create the grid for `name`. If the stored cell_size is
        /// smaller than `min_cell_size`, it's grown — a 3×3×3 scan is
        /// correct only when `cell_size >= radius`.
        pub fn grid_mut(&mut self, name: &str, min_cell_size: f64) -> &mut SpatialGrid {
            let grid = self
                .grids
                .entry(name.to_string())
                .or_insert_with(|| SpatialGrid::new(min_cell_size));
            if grid.cell_size < min_cell_size {
                grid.set_cell_size(min_cell_size);
            }
            grid
        }

        pub fn get(&self, name: &str) -> Option<&SpatialGrid> {
            self.grids.get(name)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn e(n: u32) -> Entity {
            Entity::from_raw_u32(n.saturating_add(1)).expect("valid entity index")
        }

        #[test]
        fn within_radius_returns_hits_outside_excluded() {
            let mut g = SpatialGrid::new(10.0);
            g.insert(e(0), DVec3::new(0.0, 0.0, 0.0));
            g.insert(e(1), DVec3::new(2.0, 0.0, 0.0));
            g.insert(e(2), DVec3::new(9.0, 0.0, 0.0));
            let hits = g.neighbors_within(DVec3::ZERO, 5.0, Some(e(0)));
            let ids: Vec<_> = hits.iter().map(|h| h.entity).collect();
            assert_eq!(ids, vec![e(1)]);
        }

        #[test]
        fn beyond_neighbor_cells_is_not_found() {
            // Cell size = 5, radius = 5 → 3×3×3 scan covers ±15 units. An
            // entity at x=30 must fall outside the scan and be missed. This
            // is a *desired* behaviour — cell_size must be >= radius.
            let mut g = SpatialGrid::new(5.0);
            g.insert(e(0), DVec3::new(30.0, 0.0, 0.0));
            let hits = g.neighbors_within(DVec3::ZERO, 5.0, None);
            assert!(hits.is_empty());
        }

        #[test]
        fn grid_mut_grows_cell_size_when_smaller() {
            let mut gs = SpatialGrids::default();
            gs.grid_mut("birds", 10.0);
            gs.grid_mut("birds", 20.0); // grows
            assert!(gs.get("birds").unwrap().cell_size >= 20.0);
        }

        #[test]
        fn boundary_entity_at_exact_radius_included() {
            let mut g = SpatialGrid::new(10.0);
            g.insert(e(0), DVec3::new(5.0, 0.0, 0.0));
            let hits = g.neighbors_within(DVec3::ZERO, 5.0, None);
            assert_eq!(hits.len(), 1, "entity exactly at radius should be inclusive");
        }
    }
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

    /// Bevy-mode `SystemParam`. `call` is a stub (collision uses direct
    /// Bevy queries); `neighbors_within` is backed by [`SpatialGrids`]
    /// and returns real results as long as the sim rebuilds the matching
    /// grid before this system runs.
    #[derive(bevy_ecs::system::SystemParam)]
    pub struct SpatialQueries<'w> {
        grids: Option<bevy_ecs::system::Res<'w, super::SpatialGrids>>,
    }

    impl<'w> SpatialQueries<'w> {
        pub fn call(&self, _name: &str, _prev: &DVec3, _curr: &DVec3) -> Option<SpatialHit> {
            None
        }

        /// Enumerate neighbours in group `name` within `radius` of `center`,
        /// excluding the optional querying entity. Empty `Vec` if the grid
        /// has not been populated this frame.
        pub fn neighbors_within(
            &self,
            name: &str,
            center: &DVec3,
            radius: f64,
            exclude: Option<bevy_ecs::entity::Entity>,
        ) -> Vec<super::SpatialNeighbor> {
            let Some(grids) = self.grids.as_ref() else {
                return Vec::new();
            };
            let Some(grid) = grids.get(name) else {
                return Vec::new();
            };
            grid.neighbors_within(*center, radius, exclude)
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

    /// `query_name -> query_group` for both inventory (sweep) configs and
    /// plugin-registered spatial groups. Rebuilt eagerly — cheap O(N) where
    /// N = total registered queries. Called infrequently (only on first
    /// lookup), so no incremental caching needed.
    fn name_to_group() -> HashMap<String, String> {
        let mut m = HashMap::new();
        for cfg in registered_spatial_query_configs() {
            m.insert(cfg.query_name.to_string(), cfg.query_group.to_string());
        }
        if let Some(f) = unreal_api::mass::spatial_group_cache_fn() {
            f(&mut |entries: &[(String, f64)]| {
                for (name, _radius) in entries {
                    m.insert(name.clone(), name.clone());
                }
            });
        }
        m
    }

    fn resolve_hit_entity(
        name: &str,
        index: i32,
        map: &MassEntityMap,
    ) -> bevy_ecs::entity::Entity {
        let groups = name_to_group();
        let Some(group) = groups.get(name) else {
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
    /// parameter.
    #[derive(bevy_ecs::system::SystemParam)]
    pub struct SpatialQueries<'w> {
        spatial: bevy_ecs::system::Res<'w, SpatialQuery>,
        map: bevy_ecs::system::Res<'w, MassEntityMap>,
    }

    impl<'w> SpatialQueries<'w> {
        /// Perform the named sweep query. See `SpatialQueryWithMap::call`.
        pub fn call(&self, name: &str, prev: &DVec3, curr: &DVec3) -> Option<SpatialHit> {
            self.spatial.with_map(&self.map).call(name, prev, curr)
        }

        /// Enumerate neighbours in group `name` within `radius` of `center`.
        /// Calls the per-frame enumerate callback installed by C++ for this
        /// group. Empty `Vec` if no callback is registered (treat same as
        /// "no neighbours"). `exclude` is filtered in Rust after the FFI
        /// call.
        pub fn neighbors_within(
            &self,
            name: &str,
            center: &DVec3,
            radius: f64,
            exclude: Option<bevy_ecs::entity::Entity>,
        ) -> Vec<super::SpatialNeighbor> {
            let center_arr: [f64; 3] = [center.x, center.y, center.z];
            // 64 is a comfortable default for vivarium flocking (radius 40,
            // expected <10 neighbours); the `enumerate` impl grows to `Vec`
            // on first truncation.
            let raw = self.spatial.inner.enumerate(name, &center_arr, radius as f32, 64);
            let groups = name_to_group();
            let group_name = groups.get(name).map(|s| s.as_str()).unwrap_or("");
            let mut out: Vec<super::SpatialNeighbor> = Vec::with_capacity(raw.len());
            for n in raw {
                let entity = if group_name.is_empty() {
                    bevy_ecs::entity::Entity::PLACEHOLDER
                } else {
                    self.map
                        .get(group_name, n.entity_index as usize)
                        .unwrap_or(bevy_ecs::entity::Entity::PLACEHOLDER)
                };
                if Some(entity) == exclude {
                    continue;
                }
                out.push(super::SpatialNeighbor {
                    entity,
                    position: DVec3::new(n.position[0], n.position[1], n.position[2]),
                });
            }
            out
        }
    }
}

#[cfg(not(feature = "unreal"))]
pub use bevy_impl::{SpatialQueries, SpatialQuery, SpatialQueryWithMap};
#[cfg(feature = "unreal")]
pub use unreal_impl::{SpatialQueries, SpatialQuery, SpatialQueryWithMap};
