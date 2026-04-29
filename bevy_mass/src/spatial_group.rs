//! `SpatialGroupPlugin` — authoring surface for enumerable spatial groups.
//!
//! Add one per group to an `App` (standalone `main.rs` or the UE-mode
//! `MassSchedule` via `MassAppPluginRegistration`):
//!
//! ```ignore
//! app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
//!     "birds", FLOCK_NEIGHBOR_RADIUS,
//! ));
//! ```
//!
//! Sim systems that consume the grid take `SpatialQueries` and
//! `.in_set(SpatialGroupSet::Query)`:
//!
//! ```ignore
//! app.add_systems(Update, flocking_system.in_set(SpatialGroupSet::Query));
//! ```

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use std::marker::PhantomData;

/// Framework-internal ordering sets. Rebuild runs first; game systems that
/// query spatial groups run in Query so they see the rebuilt grid.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialGroupSet {
    Rebuild,
    Query,
}

/// One registered spatial group. Populated by `SpatialGroupPlugin::build`.
#[derive(Debug, Clone)]
pub struct SpatialGroupEntry {
    pub name: &'static str,
    pub radius: f64,
    pub marker_tag_cpp_name: &'static str,
    pub position_fragment_cpp_name: &'static str,
}

/// Collection of all groups registered in this App. One entry per
/// `SpatialGroupPlugin::<M, P>::new(...)` call.
#[derive(Resource, Default)]
pub struct SpatialGroupRegistry {
    pub entries: Vec<SpatialGroupEntry>,
}

/// Per-(M, P) meta resource that carries the plugin constructor's
/// `name` and `radius` through to the generic rebuild system without a
/// per-frame name lookup. Type-pair keyed so two different marker types
/// sharing one position component still get independent instances.
#[derive(Resource)]
pub struct PerGroupMeta<M, P> {
    pub name: &'static str,
    pub radius: f64,
    _marker: PhantomData<fn(M, P)>,
}

impl<M, P> PerGroupMeta<M, P> {
    pub fn new(name: &'static str, radius: f64) -> Self {
        Self { name, radius, _marker: PhantomData }
    }
}

use crate::movement::TransformLike;
use crate::spatial_query::SpatialGrids;

/// Registers one enumerable spatial group. One instance per `(Marker, Pos)`
/// pair — see `PerGroupMeta` for why.
pub struct SpatialGroupPlugin<M, P> {
    name: &'static str,
    radius: f64,
    _marker: PhantomData<fn(M, P)>,
}

impl<M, P> SpatialGroupPlugin<M, P> {
    pub fn new(name: &'static str, radius: f64) -> Self {
        Self { name, radius, _marker: PhantomData }
    }
}

/// Constrain `M` to types that can serve as an ECS marker (a Bevy
/// `Component`) in both backends. `Pos` needs `TransformLike` to expose
/// `translation()`.
#[cfg(not(feature = "unreal"))]
impl<M, P> Plugin for SpatialGroupPlugin<M, P>
where
    M: Component + Send + Sync + 'static,
    P: TransformLike,
{
    fn build(&self, app: &mut App) {
        // Resources are idempotent: multiple plugin instances share them.
        if !app.world().contains_resource::<SpatialGrids>() {
            app.insert_resource(SpatialGrids::default());
        }
        if !app.world().contains_resource::<SpatialGroupRegistry>() {
            app.insert_resource(SpatialGroupRegistry::default());
        }

        // Duplicate-name check: reinstalling the same name is an author error.
        {
            let mut registry = app.world_mut().resource_mut::<SpatialGroupRegistry>();
            assert!(
                !registry.entries.iter().any(|e| e.name == self.name),
                "SpatialGroupPlugin: group '{}' already registered",
                self.name,
            );
            registry.entries.push(SpatialGroupEntry {
                name: self.name,
                radius: self.radius,
                // Bevy mode never uses these; populated only in UE-mode impl below.
                marker_tag_cpp_name: "",
                position_fragment_cpp_name: "",
            });
        }

        app.insert_resource(PerGroupMeta::<M, P>::new(self.name, self.radius));

        app.configure_sets(Update, SpatialGroupSet::Query.after(SpatialGroupSet::Rebuild));
        app.add_systems(
            Update,
            rebuild_grid_system::<M, P>.in_set(SpatialGroupSet::Rebuild),
        );
    }
}

/// Bevy-mode rebuild: clears the grid for this group and re-inserts every
/// entity with the marker + position. Runs in `SpatialGroupSet::Rebuild`.
#[cfg(not(feature = "unreal"))]
pub fn rebuild_grid_system<M, P>(
    mut grids: ResMut<SpatialGrids>,
    meta: Res<PerGroupMeta<M, P>>,
    entities: Query<(Entity, &P), With<M>>,
) where
    M: Component + Send + Sync + 'static,
    P: TransformLike,
{
    let grid = grids.grid_mut(meta.name, meta.radius);
    grid.clear();
    for (e, p) in &entities {
        grid.insert(e, p.translation());
    }
}

#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::Transform;

    #[derive(Component, Clone, Copy)]
    struct Bird;

    #[test]
    fn plugin_registers_group_entry() {
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 40.0));

        let registry = app.world().resource::<SpatialGroupRegistry>();
        assert_eq!(registry.entries.len(), 1);
        assert_eq!(registry.entries[0].name, "birds");
        assert_eq!(registry.entries[0].radius, 40.0);
    }
}
