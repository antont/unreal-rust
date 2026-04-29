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

use bevy_app::{App, Plugin};
#[cfg(not(feature = "unreal"))]
use bevy_app::Update;
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
#[cfg(not(feature = "unreal"))]
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

/// UE-mode plugin impl: does not install a rebuild system (UE owns the hash
/// grid). Only pushes the group metadata into `SpatialGroupRegistry` so
/// `unreal-module` can synthesise a `GridHashEnumerate` query config.
///
/// Stronger bound than Bevy mode — `M` must be a `MassFragment` so we can
/// read `M::CPP_TYPE_NAME` for the UE-side tag lookup, and `P` must also be
/// a `MassFragment` for the position-fragment name.
#[cfg(feature = "unreal")]
impl<M, P> Plugin for SpatialGroupPlugin<M, P>
where
    M: Component + unreal_api::mass::MassFragment + Send + Sync + 'static,
    P: TransformLike + unreal_api::mass::MassFragment,
{
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<SpatialGroupRegistry>() {
            app.insert_resource(SpatialGroupRegistry::default());
        }

        let mut registry = app.world_mut().resource_mut::<SpatialGroupRegistry>();
        assert!(
            !registry.entries.iter().any(|e| e.name == self.name),
            "SpatialGroupPlugin: group '{}' already registered",
            self.name,
        );
        registry.entries.push(SpatialGroupEntry {
            name: self.name,
            radius: self.radius,
            marker_tag_cpp_name: <M as unreal_api::mass::MassFragment>::CPP_TYPE_NAME,
            position_fragment_cpp_name: <P as unreal_api::mass::MassFragment>::CPP_TYPE_NAME,
        });
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
    use crate::prelude::SpatialQueries;
    use glam::DVec3;

    #[derive(Component, Clone, Copy)]
    struct Bird;

    #[derive(Component, Clone, Copy)]
    struct Fish;

    #[test]
    fn plugin_registers_group_entry() {
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 40.0));

        let registry = app.world().resource::<SpatialGroupRegistry>();
        assert_eq!(registry.entries.len(), 1);
        assert_eq!(registry.entries[0].name, "birds");
        assert_eq!(registry.entries[0].radius, 40.0);
    }

    #[test]
    fn query_system_sees_rebuilt_grid() {
        #[derive(Resource, Default)]
        struct HitCount(usize);

        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 10.0));

        app.init_resource::<HitCount>();

        app.world_mut().spawn((Bird, Transform::from_translation(DVec3::ZERO)));
        app.world_mut().spawn((Bird, Transform::from_translation(DVec3::new(3.0, 0.0, 0.0))));

        fn record_neighbors(
            spatial: SpatialQueries,
            mut hits: ResMut<HitCount>,
        ) {
            let n = spatial.neighbors_within("birds", &DVec3::ZERO, 10.0, None);
            hits.0 = n.len();
        }

        // The plugin's `configure_sets(Update, Query.after(Rebuild))` call
        // enforces the ordering this test relies on.
        app.add_systems(Update, record_neighbors.in_set(SpatialGroupSet::Query));

        app.update();

        let count = app.world().resource::<HitCount>().0;
        assert_eq!(count, 2, "query system must see both birds inserted by rebuild");
    }

    #[test]
    fn two_plugins_coexist_with_different_markers() {
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 10.0));
        app.add_plugins(SpatialGroupPlugin::<Fish, Transform>::new("fish", 5.0));

        app.world_mut().spawn((Bird, Transform::from_translation(DVec3::ZERO)));
        app.world_mut().spawn((Bird, Transform::from_translation(DVec3::new(3.0, 0.0, 0.0))));
        app.world_mut().spawn((Fish, Transform::from_translation(DVec3::ZERO)));

        app.update();

        let grids = app.world().resource::<crate::spatial_query::SpatialGrids>();
        let birds = grids.get("birds").expect("birds grid");
        let fish = grids.get("fish").expect("fish grid");
        assert_eq!(birds.neighbors_within(DVec3::ZERO, 10.0, None).len(), 2);
        assert_eq!(fish.neighbors_within(DVec3::ZERO, 10.0, None).len(), 1);

        let registry = app.world().resource::<SpatialGroupRegistry>();
        assert_eq!(registry.entries.len(), 2);
    }

    #[test]
    #[should_panic(expected = "already registered")]
    fn duplicate_group_name_panics() {
        let mut app = App::new();
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 10.0));
        app.add_plugins(SpatialGroupPlugin::<Fish, Transform>::new("birds", 20.0));
    }
}
