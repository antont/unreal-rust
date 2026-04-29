# SpatialGroupPlugin Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace vivarium's Rust-side `rebuild_bird_grid_system` mirror with a generic `SpatialGroupPlugin<Marker, Pos>` whose UE-mode impl routes `neighbors_within` through an FFI enumerate callback into UE's existing hash grid.

**Architecture:** One Bevy plugin authored by sim code; in Bevy mode it installs a per-group `rebuild_grid_system` and uses the existing `SpatialGrids` cell grid; in UE mode it registers a `SpatialGroupEntry` that `unreal-module` translates into a `GridHashEnumerate`-typed spatial-query config, and C++ populates a per-frame `MassSpatialEnumerateSlot` with an `ExecuteGridHashEnumerate` trampoline that calls `FNavigationObstacleHashGrid2D::QuerySmall` + `EntityToIndex` + `ISMC.GetInstanceTransform` and writes `(entity_index, position)` pairs back to Rust.

**Tech Stack:** Rust (Bevy 0.18 ECS, `inventory`, `min_specialization`), C++ (UE 5.6 MassEntity), vivarium-sim, unreal-ffi/unreal-api/unreal-module, RustPlugin C++ subsystems.

**Source spec:** `docs/superpowers/specs/2026-04-29-spatial-group-plugin-design.md`

---

## File Structure

**New files:**
- `bevy_mass/src/spatial_group.rs` — `SpatialGroupPlugin<M, P>`, `SpatialGroupSet`, `SpatialGroupRegistry`, `SpatialGroupEntry`, `PerGroupMeta<M, P>`, `rebuild_grid_system<M, P>`.

**Modified files (Rust):**
- `bevy_mass/src/lib.rs` — `pub mod spatial_group;` + prelude re-exports.
- `bevy_mass/src/spatial_query.rs` — unify `name_to_group()` cache; rewrite UE-mode `SpatialQueries::neighbors_within` to call enumerate FFI.
- `unreal-ffi/src/lib.rs` — new FFI types (`MassSpatialNeighbor`, `MassSpatialEnumerateFn`, `MassSpatialEnumerateSlot`); extend `MassFrameDispatchData`; layout tests.
- `unreal-api/src/mass.rs` — `MassSpatialQueries::enumerate()` + `enumerates` map; `MassAppPluginRegistration` inventory type; `MassSpatialQueryType::GridHashEnumerate = 3`.
- `unreal-module/src/mass_system_registry.rs` — run app plugin builds in `mass_init_simulation`; module-level `SPATIAL_GROUP_CACHE`; merge into `get_spatial_query_config_count/desc`; read `spatial_enumerates` in `mass_frame_dispatch`; register `register_spatial_group_cache_accessor`.
- `vivarium-sim/src/lib.rs` — add `install_plugins` function.
- `vivarium-sim/src/bird.rs` — delete `rebuild_bird_grid_system`; flocking tests install plugin.
- `vivarium-sim/src/unreal/mod.rs` — delete old `MassSpatialQueryConfigRegistration` for birds; delete `"rebuild_bird_grid_system"` from `MassScheduleOrder`; delete related test; submit `MassAppPluginRegistration`.
- `vivarium-standalone/src/main.rs` — install plugin instead of calling `rebuild_bird_grid_system`.

**Modified files (C++):**
- `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.{h,cpp}` — `ExecuteGridHashEnumerate`; enumerate trampoline array (`RustMassSpatialEnumerate::MaxEnumerates`, `EnumerateTrampolineFns`); `QueryType == 3` branch in `SetupSpatialQueriesFromRust`; `RegisterSpatialEnumerate`; build enumerate slots in `SetupProcessorPipelines`.
- `RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.{h,cpp}` — store `SpatialEnumerateSlots` and owned name buffers; set them into `MassFrameDispatchData` each tick.

---

## Slice 1 — Plugin + Bevy-mode tests

Ships a fully working Bevy-mode plugin with unit tests. No FFI, no C++, no UE-mode behaviour change yet. Verification: `cargo test -p bevy_mass`.

### Task 1.1: Create `spatial_group.rs` skeleton + failing registry test

**Files:**
- Create: `bevy_mass/src/spatial_group.rs`
- Modify: `bevy_mass/src/lib.rs` (add `pub mod spatial_group;`)

- [ ] **Step 1: Create the module skeleton with types only (no impls yet)**

Write `bevy_mass/src/spatial_group.rs`:

```rust
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
```

- [ ] **Step 2: Register the module + write a failing registry test**

Edit `bevy_mass/src/lib.rs`, add after line 38 (`pub mod spatial_query;`):

```rust
pub mod spatial_group;
```

Append to `bevy_mass/src/spatial_group.rs`:

```rust
#[cfg(all(test, not(feature = "unreal")))]
mod tests {
    use super::*;
    use crate::Transform;
    use crate::movement::TransformLike;

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
```

- [ ] **Step 3: Run the test to confirm it fails (no `SpatialGroupPlugin` yet)**

Run: `cargo test -p bevy_mass spatial_group::tests::plugin_registers_group_entry`
Expected: FAIL — `cannot find type SpatialGroupPlugin in this scope`.

- [ ] **Step 4: Add the `SpatialGroupPlugin` struct + Bevy-mode `Plugin` impl**

Append to `bevy_mass/src/spatial_group.rs` (above the tests module):

```rust
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
```

- [ ] **Step 5: Run the registry test to confirm it passes**

Run: `cargo test -p bevy_mass spatial_group::tests::plugin_registers_group_entry`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add bevy_mass/src/spatial_group.rs bevy_mass/src/lib.rs
git commit -m "feat(bevy_mass): SpatialGroupPlugin skeleton + registry test"
```

### Task 1.2: Rebuild-before-query ordering test

**Files:**
- Test: `bevy_mass/src/spatial_group.rs`

- [ ] **Step 1: Write the failing ordering test**

Append to the `tests` module in `bevy_mass/src/spatial_group.rs`:

```rust
#[test]
fn query_system_sees_rebuilt_grid() {
    use crate::prelude::SpatialQueries;
    use glam::DVec3;
    use std::sync::{Arc, Mutex};

    #[derive(Resource, Clone)]
    struct HitCount(Arc<Mutex<usize>>);

    let mut app = App::new();
    app.add_plugins(bevy_app::MinimalPlugins);
    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 10.0));

    let count = HitCount(Arc::new(Mutex::new(0)));
    app.insert_resource(count.clone());

    app.world_mut().spawn((Bird, Transform::from_translation(DVec3::ZERO)));
    app.world_mut().spawn((Bird, Transform::from_translation(DVec3::new(3.0, 0.0, 0.0))));

    fn record_neighbors(
        spatial: SpatialQueries,
        hits: Res<HitCount>,
    ) {
        let n = spatial.neighbors_within("birds", &glam::DVec3::ZERO, 10.0, None);
        *hits.0.lock().unwrap() = n.len();
    }

    app.add_systems(Update, record_neighbors.in_set(SpatialGroupSet::Query));

    app.update();

    assert_eq!(
        *count.0.lock().unwrap(),
        2,
        "query system must see both birds inserted by rebuild",
    );
}
```

- [ ] **Step 2: Run the ordering test to verify it fails**

Run: `cargo test -p bevy_mass spatial_group::tests::query_system_sees_rebuilt_grid`
Expected: FAIL — returns 0 hits (query runs before rebuild OR test compiles but SystemParam isn't wired). If it unexpectedly passes, it's because both systems happen to run in rebuild-then-query order by registration; force a failure by adding `use`-side set deliberately reversed only to confirm, then restore.

- [ ] **Step 3: Verify the passing ordering**

The `configure_sets(Update, Query.after(Rebuild))` already in `build()` handles this. The test must pass without modification — the "failing" step above proves ordering is enforced *by the plugin*, not by accident of registration order.

If the test passes in Step 2, continue (ordering is enforced by the set configuration). If it fails, revisit Task 1.1 Step 4 and confirm `configure_sets` is present.

Run: `cargo test -p bevy_mass spatial_group::tests::query_system_sees_rebuilt_grid`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add bevy_mass/src/spatial_group.rs
git commit -m "test(bevy_mass): SpatialGroupPlugin Rebuild → Query ordering"
```

### Task 1.3: Two-plugin coexistence test

**Files:**
- Test: `bevy_mass/src/spatial_group.rs`

- [ ] **Step 1: Add the failing two-group test**

Append to the `tests` module:

```rust
#[test]
fn two_plugins_coexist_with_different_markers() {
    use glam::DVec3;

    #[derive(Component, Clone, Copy)]
    struct Fish;

    let mut app = App::new();
    app.add_plugins(bevy_app::MinimalPlugins);
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
```

- [ ] **Step 2: Run the test to verify it passes**

Run: `cargo test -p bevy_mass spatial_group::tests::two_plugins_coexist_with_different_markers`
Expected: PASS (resources are idempotent, `PerGroupMeta<M, P>` is type-pair keyed so the two plugins get independent meta).

- [ ] **Step 3: Duplicate-name assertion test**

Append to the `tests` module:

```rust
#[test]
#[should_panic(expected = "already registered")]
fn duplicate_group_name_panics() {
    let mut app = App::new();
    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 10.0));
    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds", 20.0));
}
```

Run: `cargo test -p bevy_mass spatial_group::tests::duplicate_group_name_panics`
Expected: PASS (the `assert!` in `build()` fires).

- [ ] **Step 4: Commit**

```bash
git add bevy_mass/src/spatial_group.rs
git commit -m "test(bevy_mass): SpatialGroupPlugin two-group + duplicate-name"
```

### Task 1.4: Prelude re-exports

**Files:**
- Modify: `bevy_mass/src/lib.rs`

- [ ] **Step 1: Re-export plugin types in the prelude**

Edit `bevy_mass/src/lib.rs`. In the `prelude` module, after the `spatial_query` `use` block (line 72-74), add:

```rust
    pub use crate::spatial_group::{
        SpatialGroupEntry, SpatialGroupPlugin, SpatialGroupRegistry, SpatialGroupSet,
    };
```

In the crate-root re-exports after line 109 (`pub use spatial_query::…`), add:

```rust
pub use spatial_group::{SpatialGroupEntry, SpatialGroupPlugin, SpatialGroupRegistry, SpatialGroupSet};
```

- [ ] **Step 2: Run the whole test suite to confirm nothing broke**

Run: `cargo test -p bevy_mass`
Expected: PASS (includes Task 1.1/1.2/1.3 tests plus the pre-existing grid tests).

- [ ] **Step 3: Commit**

```bash
git add bevy_mass/src/lib.rs
git commit -m "feat(bevy_mass): re-export SpatialGroupPlugin from prelude"
```

---

## Slice 2 — FFI plumbing

Adds the enumerate FFI structs, `MassSpatialQueries::enumerate`, `MassAppPluginRegistration`, the `GridHashEnumerate` query type, and the module-level `SPATIAL_GROUP_CACHE`. Wires UE-mode `SpatialQueries::neighbors_within` to the new path. C++ hasn't been touched yet, so the enumerate slot array is empty at runtime and `neighbors_within` returns `Vec::new()` in UE mode — but the plumbing compiles and the UE feature-gated tests still pass. Verification: `cargo test -p bevy_mass --features unreal`, `cargo test -p unreal-ffi`, `cargo test -p unreal-api`, `cargo test -p unreal-module`.

### Task 2.1: `MassSpatialNeighbor` + `MassSpatialEnumerateSlot` FFI structs

**Files:**
- Modify: `unreal-ffi/src/lib.rs` (around lines 223-253 + layout tests around 1108-1123)

- [ ] **Step 1: Write a failing layout test for `MassSpatialNeighbor`**

Edit `unreal-ffi/src/lib.rs`. Insert in the `tests` module after `mass_spatial_query_slot_layout` (line 1116):

```rust
    #[test]
    fn mass_spatial_neighbor_layout() {
        // i32(4) + i32(4) + [f64;3](24) = 32
        assert_eq!(std::mem::size_of::<MassSpatialNeighbor>(), 32);
        assert_eq!(std::mem::align_of::<MassSpatialNeighbor>(), 8);
        assert_eq!(std::mem::offset_of!(MassSpatialNeighbor, entity_index), 0);
        assert_eq!(std::mem::offset_of!(MassSpatialNeighbor, position), 8);
    }

    #[test]
    fn mass_spatial_enumerate_slot_layout() {
        // Utf8Str(16) + fn_ptr(8) + f32(4) + u32(4) = 32
        assert_eq!(std::mem::size_of::<MassSpatialEnumerateSlot>(), 32);
        assert_eq!(std::mem::align_of::<MassSpatialEnumerateSlot>(), 8);
        assert_eq!(std::mem::offset_of!(MassSpatialEnumerateSlot, name), 0);
        assert_eq!(std::mem::offset_of!(MassSpatialEnumerateSlot, enumerate_fn), 16);
        assert_eq!(std::mem::offset_of!(MassSpatialEnumerateSlot, radius), 24);
    }
```

Run: `cargo test -p unreal-ffi mass_spatial_neighbor_layout`
Expected: FAIL — `cannot find type MassSpatialNeighbor in this scope`.

- [ ] **Step 2: Add the three new types to `unreal-ffi/src/lib.rs`**

Insert after `MassSpatialQuerySlot` at line 233 (before `MassFrameDispatchData`):

```rust
/// One neighbour returned by a spatial-group enumerate callback.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MassSpatialNeighbor {
    /// Group-local instance index (matches `MassEntityMap` index).
    pub entity_index: i32,
    pub _pad: i32,
    /// World-space position [x, y, z].
    pub position: [f64; 3],
}

/// C++ callback: enumerate all members of a named spatial group within
/// `radius` of `center`. Writes up to `max` `(entity_index, position)` pairs
/// into `out` and returns the total count that would have been written if
/// `max` were infinite, so Rust can detect truncation by testing
/// `returned > max` and retry with a larger buffer.
///
/// # Safety
/// `center` must point to `[f64; 3]`. `out` must point to an array of at
/// least `max` `MassSpatialNeighbor` entries (may be null iff `max == 0`).
pub type MassSpatialEnumerateFn = unsafe extern "C" fn(
    center: *const f64,
    radius: f32,
    out: *mut MassSpatialNeighbor,
    max: u32,
) -> u32;

/// One named enumerate slot in the per-frame dispatch. Parallel structure
/// to `MassSpatialQuerySlot` (sweep) — C++ populates one per registered
/// `SpatialGroupEntry` each frame.
#[repr(C)]
pub struct MassSpatialEnumerateSlot {
    /// Group name (e.g. "birds"). Borrowed from C++ — valid this frame.
    pub name: Utf8Str,
    /// Callback that enumerates this group.
    pub enumerate_fn: MassSpatialEnumerateFn,
    /// Group radius (for informational use; per-call radius may be smaller).
    pub radius: f32,
    pub _pad: u32,
}
```

Run: `cargo test -p unreal-ffi mass_spatial_neighbor_layout mass_spatial_enumerate_slot_layout`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add unreal-ffi/src/lib.rs
git commit -m "feat(unreal-ffi): MassSpatialNeighbor + MassSpatialEnumerateSlot"
```

### Task 2.2: Extend `MassFrameDispatchData` with enumerate slots

**Files:**
- Modify: `unreal-ffi/src/lib.rs`

- [ ] **Step 1: Add enumerate fields to `MassFrameDispatchData`**

Edit `unreal-ffi/src/lib.rs`. Replace the `MassFrameDispatchData` struct at line 236-249 with:

```rust
/// Per-frame dispatch data: dt + system chunk batches + named spatial queries + named enumerate slots.
#[repr(C)]
pub struct MassFrameDispatchData {
    /// Delta time for this frame.
    pub dt: f32,
    /// Number of system batches.
    pub num_systems: u32,
    /// Pointer to array of MassSystemChunkBatch.
    pub systems: *const MassSystemChunkBatch,
    /// Number of spatial query (sweep) slots available this frame.
    pub num_spatial_queries: u32,
    /// Number of spatial enumerate slots available this frame.
    pub num_spatial_enumerates: u32,
    /// Pointer to array of MassSpatialQuerySlot.
    pub spatial_queries: *const MassSpatialQuerySlot,
    /// Pointer to array of MassSpatialEnumerateSlot.
    pub spatial_enumerates: *const MassSpatialEnumerateSlot,
}
```

- [ ] **Step 2: Update the layout test**

Edit `unreal-ffi/src/lib.rs`. Replace `mass_frame_dispatch_data_layout` at line 1118-1123 with:

```rust
    #[test]
    fn mass_frame_dispatch_data_layout() {
        // f32(4) + u32(4) + ptr(8) + u32(4) + u32(4) + ptr(8) + ptr(8) = 40
        assert_eq!(std::mem::size_of::<MassFrameDispatchData>(), 40);
        assert_eq!(std::mem::align_of::<MassFrameDispatchData>(), 8);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, dt), 0);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, num_systems), 4);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, systems), 8);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, num_spatial_queries), 16);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, num_spatial_enumerates), 20);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, spatial_queries), 24);
        assert_eq!(std::mem::offset_of!(MassFrameDispatchData, spatial_enumerates), 32);
    }
```

- [ ] **Step 3: Fix the existing construction site in `unreal-module` tests**

Edit `unreal-module/src/mass_system_registry.rs` around line 738-745. Replace the test literal:

```rust
        let data = unreal_ffi::MassFrameDispatchData {
            dt: 0.016,
            num_systems: 0,
            systems: std::ptr::null(),
            num_spatial_queries: 0,
            num_spatial_enumerates: 0,
            spatial_queries: std::ptr::null(),
            spatial_enumerates: std::ptr::null(),
        };
```

- [ ] **Step 4: Run tests to confirm layout + existing use-sites compile**

Run: `cargo test -p unreal-ffi mass_frame_dispatch_data_layout`
Expected: PASS.

Run: `cargo build -p unreal-module`
Expected: PASS.

Run: `cargo test -p unreal-module test_mass_frame_dispatch_without_schedule`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add unreal-ffi/src/lib.rs unreal-module/src/mass_system_registry.rs
git commit -m "feat(unreal-ffi): extend MassFrameDispatchData with spatial_enumerates"
```

### Task 2.3: `MassSpatialQueries::enumerate` resource path

**Files:**
- Modify: `unreal-api/src/mass.rs` (lines 367-423)

- [ ] **Step 1: Add `enumerates` field + `enumerate` method (failing unit test first)**

Edit `unreal-api/src/mass.rs`. Append after the `MassSpatialQueries` `impl` block around line 423:

```rust
#[cfg(test)]
mod mass_spatial_queries_enumerate_tests {
    use super::*;

    #[test]
    fn enumerate_returns_none_when_unregistered() {
        let queries = MassSpatialQueries::default();
        let hits = queries.enumerate("absent", &[0.0, 0.0, 0.0], 10.0, 32);
        assert!(hits.is_empty());
    }
}
```

Run: `cargo test -p unreal-api mass_spatial_queries_enumerate_tests`
Expected: FAIL — `no method named enumerate on MassSpatialQueries`.

- [ ] **Step 2: Add the field + method**

Edit `unreal-api/src/mass.rs`. Replace the struct def at line 367-369 with:

```rust
/// Bevy resource holding all named spatial query callbacks for the current frame.
/// Game systems access queries by name:
///   - `spatial.call("food_pickup", &prev, &cur)` — sweep
///   - `spatial.enumerate("birds", &center, radius, max)` — hash-grid
pub struct MassSpatialQueries {
    queries: std::collections::HashMap<String, (unreal_ffi::MassSpatialQueryFn, f32)>,
    enumerates: std::collections::HashMap<String, (unreal_ffi::MassSpatialEnumerateFn, f32)>,
}
```

Replace `Default` impl at lines 375-381:

```rust
impl Default for MassSpatialQueries {
    fn default() -> Self {
        Self {
            queries: std::collections::HashMap::new(),
            enumerates: std::collections::HashMap::new(),
        }
    }
}
```

Extend the `impl MassSpatialQueries` block. Replace `clear()` at lines 385-387 and add new `insert_enumerate` + `enumerate` methods:

```rust
impl MassSpatialQueries {
    /// Clear all queries and enumerates (start of each frame).
    pub fn clear(&mut self) {
        self.queries.clear();
        self.enumerates.clear();
    }

    /// Insert a named sweep query callback + radius.
    pub fn insert(&mut self, name: String, query_fn: unreal_ffi::MassSpatialQueryFn, radius: f32) {
        self.queries.insert(name, (query_fn, radius));
    }

    /// Insert a named enumerate callback + radius.
    pub fn insert_enumerate(
        &mut self,
        name: String,
        enumerate_fn: unreal_ffi::MassSpatialEnumerateFn,
        radius: f32,
    ) {
        self.enumerates.insert(name, (enumerate_fn, radius));
    }

    /// Call a named sweep query. Returns None if not registered this frame.
    pub fn call(
        &self,
        name: &str,
        previous_pos: &[f64; 3],
        current_pos: &[f64; 3],
    ) -> Option<unreal_ffi::MassSpatialQueryResult> {
        let (query_fn, radius) = self.queries.get(name)?;
        let mut result = unreal_ffi::MassSpatialQueryResult {
            entity_index: -1,
            _pad: 0,
            encounter_position: [0.0; 3],
            has_encounter: false,
            _result_pad: [0; 7],
        };
        let ok = unsafe {
            query_fn(
                previous_pos.as_ptr(),
                current_pos.as_ptr(),
                *radius,
                &mut result,
            )
        };
        if ok != 0 { Some(result) } else { None }
    }

    /// Enumerate neighbours in the named group within `radius` of `center`.
    /// Allocates a `Vec` with initial capacity `initial_max` and retries with
    /// a larger one if C++ reports truncation. Empty `Vec` if the name is
    /// not registered for this frame (no silent panic — caller treats
    /// missing enumerate the same as "no neighbours").
    pub fn enumerate(
        &self,
        name: &str,
        center: &[f64; 3],
        radius: f32,
        initial_max: u32,
    ) -> Vec<unreal_ffi::MassSpatialNeighbor> {
        let Some((enumerate_fn, _registered_radius)) = self.enumerates.get(name) else {
            return Vec::new();
        };
        // Reserve `initial_max` slots; grow once if truncated.
        let mut buf: Vec<unreal_ffi::MassSpatialNeighbor> =
            Vec::with_capacity(initial_max as usize);
        let returned = unsafe {
            enumerate_fn(center.as_ptr(), radius, buf.as_mut_ptr(), initial_max)
        };
        if returned <= initial_max {
            unsafe { buf.set_len(returned as usize) };
            return buf;
        }
        // Truncation: C++ said it had `returned` hits; retry with a big enough buffer.
        let mut grown: Vec<unreal_ffi::MassSpatialNeighbor> =
            Vec::with_capacity(returned as usize);
        let actual = unsafe {
            enumerate_fn(center.as_ptr(), radius, grown.as_mut_ptr(), returned)
        };
        let len = core::cmp::min(actual, returned) as usize;
        unsafe { grown.set_len(len) };
        grown
    }
}
```

Run: `cargo test -p unreal-api mass_spatial_queries_enumerate_tests`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add unreal-api/src/mass.rs
git commit -m "feat(unreal-api): MassSpatialQueries::enumerate + insert_enumerate"
```

### Task 2.4: Wire `spatial_enumerates` into `mass_frame_dispatch`

**Files:**
- Modify: `unreal-module/src/mass_system_registry.rs` (around lines 263-275)

- [ ] **Step 1: Read the new slots alongside the sweep slots**

Edit `unreal-module/src/mass_system_registry.rs`. Replace the spatial-queries block at lines 263-275 with:

```rust
        // Update spatial queries (sweep) + enumerates from dispatch data
        {
            let mut queries = sched.world_mut().resource_mut::<SpatialQuery>();
            queries.clear();
            if data.num_spatial_queries > 0 && !data.spatial_queries.is_null() {
                let slots = unsafe {
                    std::slice::from_raw_parts(data.spatial_queries, data.num_spatial_queries as usize)
                };
                for slot in slots {
                    let name = slot.name.as_str().to_string();
                    queries.insert(name, slot.query_fn, slot.radius);
                }
            }
            if data.num_spatial_enumerates > 0 && !data.spatial_enumerates.is_null() {
                let slots = unsafe {
                    std::slice::from_raw_parts(
                        data.spatial_enumerates,
                        data.num_spatial_enumerates as usize,
                    )
                };
                for slot in slots {
                    let name = slot.name.as_str().to_string();
                    queries.insert_enumerate(name, slot.enumerate_fn, slot.radius);
                }
            }
        }
```

- [ ] **Step 2: Verify build + unit tests**

Run: `cargo build -p unreal-module`
Expected: PASS.

Run: `cargo test -p unreal-module`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add unreal-module/src/mass_system_registry.rs
git commit -m "feat(unreal-module): read spatial_enumerates in mass_frame_dispatch"
```

### Task 2.5: `MassAppPluginRegistration` inventory type

**Files:**
- Modify: `unreal-api/src/mass.rs` (add alongside `MassBevySystemRegistration` around line 1689-1716)

- [ ] **Step 1: Add the inventory type with a registration test**

Edit `unreal-api/src/mass.rs`. Insert after `registered_bevy_mass_systems()` at line 1716:

```rust
/// Registration for an App-level Bevy plugin that should be built into the
/// `MassSchedule` app when a simulation starts. Submitted by sim crates via
/// `inventory::submit!` and iterated by `unreal-module::mass_init_simulation`.
pub struct MassAppPluginRegistration {
    /// Called with the shadow `MassSchedule` app; should do exactly
    /// `app.add_plugins(...)`. Runs once per `init_sim` call.
    pub build_fn: fn(&mut bevy_app::App),
}

inventory::collect!(MassAppPluginRegistration);

pub fn registered_app_plugins() -> inventory::iter<MassAppPluginRegistration> {
    inventory::iter::<MassAppPluginRegistration>
}
```

Append a compile test in the same file's existing `tests` module (search for `#[cfg(test)]` near the end of `mass.rs`; if none, add at EOF):

```rust
#[cfg(test)]
mod app_plugin_registration_test {
    use super::*;

    fn noop_build(_app: &mut bevy_app::App) {}
    inventory::submit!(MassAppPluginRegistration { build_fn: noop_build });

    #[test]
    fn at_least_one_registered() {
        assert!(registered_app_plugins().into_iter().any(|r| {
            // Compare function pointers by cast to raw pointer.
            r.build_fn as *const () == noop_build as *const ()
        }));
    }
}
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p unreal-api app_plugin_registration_test`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add unreal-api/src/mass.rs
git commit -m "feat(unreal-api): MassAppPluginRegistration inventory type"
```

### Task 2.6: Add `GridHashEnumerate = 3` query type

**Files:**
- Modify: `unreal-api/src/mass.rs` (line 1953)

- [ ] **Step 1: Add the new enum variant**

Edit `unreal-api/src/mass.rs`. Replace `MassSpatialQueryType` at lines 1950-1961:

```rust
/// Spatial query implementation type.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MassSpatialQueryType {
    /// ISMC overlap queries (GetInstancesOverlappingSphere/Box).
    #[default]
    IsmcOverlap = 0,
    /// UE physics sweep (World->SweepMultiByChannel).
    PhysicsSweep = 1,
    /// UMassNavigationSubsystem hash grid — single-hit sweep.
    GridHash = 2,
    /// UMassNavigationSubsystem hash grid — enumerate-in-radius.
    /// Used by `SpatialGroupPlugin`.
    GridHashEnumerate = 3,
}
```

- [ ] **Step 2: Verify build**

Run: `cargo build -p unreal-api`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add unreal-api/src/mass.rs
git commit -m "feat(unreal-api): GridHashEnumerate = 3 spatial query type"
```

### Task 2.7: `SpatialGroupPlugin` UE-mode `Plugin` impl

**Files:**
- Modify: `bevy_mass/src/spatial_group.rs`

- [ ] **Step 1: Add the feature-gated UE-mode impl**

Edit `bevy_mass/src/spatial_group.rs`. Append after the Bevy-mode `Plugin` impl:

```rust
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
```

- [ ] **Step 2: Build both feature configurations**

Run: `cargo build -p bevy_mass`
Expected: PASS.

Run: `cargo build -p bevy_mass --features unreal --no-default-features`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add bevy_mass/src/spatial_group.rs
git commit -m "feat(bevy_mass): SpatialGroupPlugin UE-mode impl"
```

### Task 2.8: `SPATIAL_GROUP_CACHE` in `unreal-module` + merged query-config output

**Files:**
- Modify: `unreal-module/src/mass_system_registry.rs` (add cache + plugin-build call site in `mass_init_simulation`; extend `get_spatial_query_config_count/desc`)

- [ ] **Step 1: Add the module-level cache + helper**

Edit `unreal-module/src/mass_system_registry.rs`. Insert after the `DESCRIPTOR_CACHE` line at line 31:

```rust
/// Snapshot of the spatial-group registry taken during `mass_init_simulation`,
/// readable from FFI handlers (`get_spatial_query_config_count/desc`) that
/// don't have direct access to the shadow Bevy world.
///
/// Mutex allows re-population on re-init (hot-reload / test reset); OnceLock
/// initialises it lazily on first write.
#[derive(Default)]
struct SyncGroupCache(Vec<bevy_mass::spatial_group::SpatialGroupEntry>);
unsafe impl Send for SyncGroupCache {}
unsafe impl Sync for SyncGroupCache {}

static SPATIAL_GROUP_CACHE: std::sync::OnceLock<std::sync::Mutex<SyncGroupCache>> =
    std::sync::OnceLock::new();

fn spatial_group_cache() -> &'static std::sync::Mutex<SyncGroupCache> {
    SPATIAL_GROUP_CACHE.get_or_init(|| std::sync::Mutex::new(SyncGroupCache::default()))
}

/// Replace the cache contents with the current shadow-world registry.
/// Called from `mass_init_simulation` after App plugin builds run.
fn refresh_spatial_group_cache(world: &unreal_api::ecs::world::World) {
    let entries = match world.get_resource::<bevy_mass::spatial_group::SpatialGroupRegistry>() {
        Some(r) => r.entries.clone(),
        None => Vec::new(),
    };
    let mut cache = spatial_group_cache().lock().unwrap();
    cache.0 = entries;
}
```

- [ ] **Step 2: Run app plugin builds + refresh the cache inside `mass_init_simulation`**

Edit `unreal-module/src/mass_system_registry.rs`. Inside `mass_init_simulation`, find the `MASS_SCHEDULE.lock()` block around line 430-485. Insert **at the start of the block**, right after `if let Some(sched) = sched_guard.as_mut().map(|w| &mut w.0) {` (line 431):

```rust
                // Build app plugins (e.g. SpatialGroupPlugin) into the shadow
                // Mass schedule. Bevy's `App` internally tracks added plugin
                // types, so adding the same plugin twice on the same App
                // normally panics. Production only reaches this branch once
                // per `MassSchedule` lifetime (`init_sim` is called once per
                // simulation start; hot reload drops the whole module). For
                // test-only re-init, `reset_mass_schedule` rebuilds the App
                // from scratch. Clearing `SpatialGroupRegistry.entries`
                // before iterating is belt-and-braces for tests that bypass
                // the reset path.
                if let Some(mut registry) = sched
                    .world_mut()
                    .get_resource_mut::<bevy_mass::spatial_group::SpatialGroupRegistry>()
                {
                    registry.entries.clear();
                }
                for reg in unreal_api::mass::registered_app_plugins() {
                    (reg.build_fn)(sched.app_mut());
                }
```

Insert **after the three existing registration loops** (after `for reg in registered_sim_init_hooks()`, around line 484, **before the closing `}` of `if let Some(sched)`):

```rust
                // Snapshot the group registry into module-level cache so
                // FFI handlers `get_spatial_query_config_count/desc` can
                // emit synthesised `GridHashEnumerate` configs.
                refresh_spatial_group_cache(world);
```

- [ ] **Step 3: Merge sources in `get_spatial_query_config_count` + `_desc`**

Edit `unreal-module/src/mass_system_registry.rs`. Replace `get_spatial_query_config_count` at lines 534-536:

```rust
pub unsafe extern "C" fn get_spatial_query_config_count() -> u32 {
    let inventory_count = registered_spatial_query_configs().into_iter().count() as u32;
    let cache_count = spatial_group_cache().lock().unwrap().0.len() as u32;
    inventory_count + cache_count
}
```

Replace `get_spatial_query_config_desc` at lines 538-563:

```rust
pub unsafe extern "C" fn get_spatial_query_config_desc(
    index: u32,
    out: *mut unreal_ffi::MassSpatialQueryConfigDesc,
) -> u32 {
    if out.is_null() {
        return 0;
    }
    let inventory_count = registered_spatial_query_configs().into_iter().count() as u32;
    if index < inventory_count {
        // Legacy: hand-authored sweep/GridHash configs (gatherers).
        let Some(reg) = registered_spatial_query_configs().into_iter().nth(index as usize) else {
            return 0;
        };
        unsafe {
            (*out) = unreal_ffi::MassSpatialQueryConfigDesc {
                query_name: unreal_ffi::Utf8Str::from(reg.query_name),
                query_group: unreal_ffi::Utf8Str::from(reg.query_group),
                radius: reg.radius,
                _pad0: 0,
                filter_fragment_type: unreal_ffi::Utf8Str::from(reg.filter_fragment_type),
                filter_bool_offset: reg.filter_bool_offset as u32,
                filter_bool_must_be: reg.filter_bool_must_be,
                query_type: reg.query_type as u8,
                collision_channel_index: reg.collision_channel_index,
                _pad1: 0,
            };
        }
        return 1;
    }
    // New: enumerate-via-plugin groups synthesised as GridHashEnumerate.
    let cache_index = (index - inventory_count) as usize;
    let cache = spatial_group_cache().lock().unwrap();
    let Some(entry) = cache.0.get(cache_index) else {
        return 0;
    };
    unsafe {
        (*out) = unreal_ffi::MassSpatialQueryConfigDesc {
            query_name: unreal_ffi::Utf8Str::from(entry.name),
            query_group: unreal_ffi::Utf8Str::from(entry.name),
            radius: entry.radius as f32,
            _pad0: 0,
            filter_fragment_type: unreal_ffi::Utf8Str::from(""),
            filter_bool_offset: 0,
            filter_bool_must_be: false,
            query_type: unreal_api::mass::MassSpatialQueryType::GridHashEnumerate as u8,
            collision_channel_index: 0,
            _pad1: 0,
        };
    }
    1
}
```

- [ ] **Step 4: Build + existing tests**

Run: `cargo build -p unreal-module`
Expected: PASS.

Run: `cargo test -p unreal-module`
Expected: PASS (dispatch-hook + schedule-reset tests still green).

- [ ] **Step 5: Commit**

```bash
git add unreal-module/src/mass_system_registry.rs
git commit -m "feat(unreal-module): run app plugins + merge spatial group cache"
```

### Task 2.9: Unify `name_to_group` + rewrite UE-mode `neighbors_within`

**Files:**
- Modify: `bevy_mass/src/spatial_query.rs` (UE-mode impl around lines 404-482)

- [ ] **Step 1: Expose plugin-registered group names to `spatial_query.rs`**

Edit `unreal-module/src/mass_system_registry.rs`. Near the bottom of `refresh_spatial_group_cache`, also register a read-only accessor. Add a new module-level function:

```rust
/// Read-only accessor for the spatial group cache, exposed to
/// `bevy_mass::spatial_query` so the UE-mode `name_to_group` cache can
/// include plugin-registered groups in addition to legacy inventory
/// configs.
pub fn visit_spatial_group_cache<F>(visit: F)
where
    F: FnOnce(&[bevy_mass::spatial_group::SpatialGroupEntry]),
{
    let cache = spatial_group_cache().lock().unwrap();
    visit(&cache.0);
}
```

Then register an accessor hook in `unreal-api` (mirrors `register_shadow_world_accessors`):

Edit `unreal-api/src/mass.rs`. Insert after `shadow_world_write_fn()` at line 325:

```rust
/// Visitor passed to the module-level spatial-group cache accessor.
pub type SpatialGroupCacheVisitor<'a> = &'a mut dyn FnMut(&[(String, f64)]);

/// Function registered by `unreal-module` that lets `bevy_mass` peek at
/// plugin-registered spatial groups without a direct crate dependency.
/// Delivers `(name, radius)` pairs copied into owned `String`s so the
/// callback doesn't borrow the cache mutex beyond its own scope.
pub type SpatialGroupCacheFn = fn(SpatialGroupCacheVisitor);

static SPATIAL_GROUP_CACHE_FN: std::sync::OnceLock<SpatialGroupCacheFn> =
    std::sync::OnceLock::new();

pub fn register_spatial_group_cache_accessor(f: SpatialGroupCacheFn) {
    let _ = SPATIAL_GROUP_CACHE_FN.set(f);
}

pub fn spatial_group_cache_fn() -> Option<SpatialGroupCacheFn> {
    SPATIAL_GROUP_CACHE_FN.get().copied()
}
```

- [ ] **Step 2: Register the accessor from `unreal-module::init_global_schedule`**

Edit `unreal-module/src/mass_system_registry.rs`. In `init_global_schedule` at line 173-187, after the `register_shadow_world_accessors` call (line 183-186), add:

```rust
    // Bridge the spatial-group cache into unreal-api so bevy_mass can
    // resolve plugin-registered group names without a direct dep on
    // unreal-module.
    unreal_api::mass::register_spatial_group_cache_accessor(bevy_visit_spatial_group_cache);
```

Add the helper at module level in `unreal-module/src/mass_system_registry.rs`:

```rust
fn bevy_visit_spatial_group_cache(
    visit: unreal_api::mass::SpatialGroupCacheVisitor,
) {
    let cache = spatial_group_cache().lock().unwrap();
    let pairs: Vec<(String, f64)> = cache
        .0
        .iter()
        .map(|e| (e.name.to_string(), e.radius))
        .collect();
    visit(&pairs);
}
```

- [ ] **Step 3: Rewrite UE-mode `neighbors_within` + unified `name_to_group`**

Edit `bevy_mass/src/spatial_query.rs`. Replace the `name_to_group` function at lines 406-415:

```rust
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
```

Replace `resolve_hit_entity` at lines 417-428 to use the new `String` map:

```rust
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
```

Drop the `grids` field and replace the UE-mode `SpatialQueries` SystemParam at lines 448-482:

```rust
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
```

- [ ] **Step 4: Build both feature configurations**

Run: `cargo build -p bevy_mass`
Expected: PASS.

Run: `cargo build -p bevy_mass --features unreal --no-default-features`
Expected: PASS.

Run: `cargo test -p bevy_mass`
Expected: PASS (Bevy-mode tests unchanged; UE-mode has no unit tests at this crate level yet).

Run: `cargo test -p unreal-module`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add unreal-api/src/mass.rs unreal-module/src/mass_system_registry.rs bevy_mass/src/spatial_query.rs
git commit -m "feat(bevy_mass): route UE-mode neighbors_within through enumerate FFI"
```

---

## Slice 3 — C++ enumerate callback + `GridHashEnumerate`

At the start of this slice, C++ does not recognise `query_type == 3`, so any `SpatialGroupPlugin::<M, P>` UE-mode registration sits dormant — `neighbors_within` returns an empty `Vec`. This slice adds the C++ half: `ExecuteGridHashEnumerate`, the enumerate trampoline, the `SetupSpatialQueriesFromRust` branch for query_type 3, and the slot build in `SetupProcessorPipelines` + `RustMassScheduleCoordinator`. Verification: full Rust test suite + UE CLI automation tests (pre-migration; vivarium still uses the old mirror).

### Task 3.1: C++ `ExecuteGridHashEnumerate` + enumerate trampoline array

**Files:**
- Modify: `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` (around lines 30-75 for trampoline; add new helper after `ExecuteGridHashSpatialQuery`)
- Modify: `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.h`

- [ ] **Step 1: Read the existing trampoline block you're about to clone**

Review `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` lines 30-75 (the `RustMassSpatialQuery` namespace, trampolines, counters). The new block below mirrors the pattern.

- [ ] **Step 2: Add the enumerate trampoline namespace**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`. Insert **after** line 75 (end of `namespace RustMassSpatialQuery`):

```cpp
namespace RustMassSpatialEnumerate
{
	static constexpr int32 MaxEnumerates = 8;
	using FEnumerateCallback = TFunction<uint32(const double*, float, MassSpatialNeighbor*, uint32)>;
	static FEnumerateCallback ActiveCallbacks[MaxEnumerates];

	template<int32 N>
	static uint32 Trampoline(const double* Center, float Radius, MassSpatialNeighbor* Out, uint32 Max)
	{
		if (ActiveCallbacks[N])
		{
			return (*ActiveCallbacks[N])(Center, Radius, Out, Max);
		}
		return 0;
	}

	static MassSpatialEnumerateFn TrampolineFns[MaxEnumerates] = {
		&Trampoline<0>, &Trampoline<1>, &Trampoline<2>, &Trampoline<3>,
		&Trampoline<4>, &Trampoline<5>, &Trampoline<6>, &Trampoline<7>,
	};
} // namespace RustMassSpatialEnumerate
```

- [ ] **Step 3: Add `ExecuteGridHashEnumerate` after `ExecuteGridHashSpatialQuery`**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`. Insert **after line 181** (end of `ExecuteGridHashSpatialQuery`):

```cpp
// ---------------------------------------------------------------------------
// Enumerate-in-radius for SpatialGroupPlugin. Mirrors ExecuteGridHashSpatialQuery
// but returns every in-radius entity (not just the closest) and has no
// closest-point-on-segment arithmetic — inputs are (center, radius) only.
//
// Writes up to `Max` (entity_index, position) pairs into `Out`. Return value
// is the uncapped count — callers test `returned > Max` to detect truncation
// and retry with a larger buffer.
// ---------------------------------------------------------------------------

uint32 ExecuteGridHashEnumerate(
	FMassEntityManager& EntityManager,
	const FNavigationObstacleHashGrid2D& Grid,
	const TMap<FMassEntityHandle, int32>& EntityToIndex,
	const UInstancedStaticMeshComponent& ISMC,
	const double* Center,
	float Radius,
	MassSpatialNeighbor* Out,
	uint32 Max)
{
	const FVector C(Center[0], Center[1], Center[2]);
	const float RadiusSq = Radius * Radius;

	// 2D grid bucketing — XY bounds only; Z is filtered by the RadiusSq test.
	FBox QueryBounds(ForceInit);
	QueryBounds += C;
	QueryBounds = QueryBounds.ExpandBy(Radius);

	TArray<FMassNavigationObstacleItem, TInlineAllocator<64>> Candidates;
	Grid.QuerySmall(QueryBounds, Candidates);

	uint32 Written = 0;
	uint32 Total = 0;
	for (const FMassNavigationObstacleItem& Item : Candidates)
	{
		const FMassEntityHandle Entity = Item.Entity;
		if (!EntityManager.IsEntityValid(Entity)) continue;

		const int32* FoundIdx = EntityToIndex.Find(Entity);
		if (!FoundIdx) continue;
		const int32 Idx = *FoundIdx;

		FTransform InstanceTransform;
		ISMC.GetInstanceTransform(Idx, InstanceTransform, true);
		const FVector EntityPos = InstanceTransform.GetLocation();

		const float DistSq = static_cast<float>(FVector::DistSquared(EntityPos, C));
		if (DistSq > RadiusSq) continue;

		++Total;
		if (Out && Written < Max)
		{
			Out[Written].entity_index = Idx;
			Out[Written]._pad = 0;
			Out[Written].position[0] = EntityPos.X;
			Out[Written].position[1] = EntityPos.Y;
			Out[Written].position[2] = EntityPos.Z;
			++Written;
		}
	}
	return Total;
}
```

- [ ] **Step 4: Declare `RegisterSpatialEnumerate` + `FSpatialEnumerateEntry` in the header**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.h`. Near the existing `FSpatialQueryCallback` / `RegisterSpatialQuery` declarations, add:

```cpp
using FSpatialEnumerateCallback = TFunction<uint32(
	const double* Center, float Radius, MassSpatialNeighbor* Out, uint32 Max)>;

struct FSpatialEnumerateEntry
{
	FString Name;
	FSpatialEnumerateCallback Callback;
	float Radius = 0.0f;
	int32 TrampolineIndex = -1;
};
```

In the `URustMassBevySubsystem` class, alongside `SpatialQueries` TMap, add:

```cpp
	TMap<FString, FSpatialEnumerateEntry> SpatialEnumerates;

public:
	void RegisterSpatialEnumerate(const FString& Name, FSpatialEnumerateCallback InCallback, float InRadius);
```

- [ ] **Step 5: Define `RegisterSpatialEnumerate` in the .cpp**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`. After `RegisterSpatialQuery` (around line 334), insert:

```cpp
void URustMassBevySubsystem::RegisterSpatialEnumerate(const FString& Name, FSpatialEnumerateCallback InCallback, float InRadius)
{
	int32 ExistingTrampolineIndex = -1;
	if (FSpatialEnumerateEntry* Existing = SpatialEnumerates.Find(Name))
	{
		ExistingTrampolineIndex = Existing->TrampolineIndex;
	}

	FSpatialEnumerateEntry Entry;
	Entry.Name = Name;
	Entry.Callback = MoveTemp(InCallback);
	Entry.Radius = InRadius;
	Entry.TrampolineIndex = ExistingTrampolineIndex;
	SpatialEnumerates.Add(Name, MoveTemp(Entry));
}
```

- [ ] **Step 6: Build the C++ plugin standalone**

Run (replace `$UE_ROOT` with your local UE 5.6 path; mirrors `docs/testing.md`):

```bash
cd /Users/tonialatalo/src/unreal-rust/example && ./build-plugin.sh
```

Expected: C++ builds, no link errors. If `FSpatialEnumerateCallback` is defined in the header but `MassSpatialNeighbor` / `MassSpatialEnumerateFn` aren't visible, add `#include "Bindings.h"` to `RustMassBevySubsystem.h` (should already be present — verify).

- [ ] **Step 7: Commit**

```bash
git add RustPlugin/Source/RustPlugin/RustMassBevySubsystem.h RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp
git commit -m "feat(RustPlugin): ExecuteGridHashEnumerate + enumerate trampoline"
```

### Task 3.2: Dispatch `QueryType == 3 GridHashEnumerate` in `SetupSpatialQueriesFromRust`

**Files:**
- Modify: `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` (line 390 area — the `if (QueryType == 2)` branch chain)

- [ ] **Step 1: Add a `else if (QueryType == 3)` branch**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`. Find the `if (QueryType == 2)` branch around line 390. Immediately after its closing `}` (but before `else if (QueryType == 1)` at line 432), insert:

```cpp
			else if (QueryType == 3) // GridHashEnumerate (SpatialGroupPlugin)
			{
				// Same owner handshake as GridHash — UE needs to know this
				// group is hash-grid-managed so init/reset paths populate it.
				TryMarkGridHashOwner(QueryGroup, /*LogPrefix=*/TEXT("SetupSpatialQueriesFromRust/Enumerate"));

				(void)FilterBoolOffset; (void)FilterBoolMustBe; (void)FilterStruct;

				RegisterSpatialEnumerate(QueryName,
					[Self, QueryGroup](
						const double* Center, float Radius,
						MassSpatialNeighbor* Out, uint32 Max) -> uint32
					{
						UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);
						UWorld* W = Self->GetWorld();
						UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
						UMassNavigationSubsystem* NavSubsystem = W ? W->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
						const TMap<FMassEntityHandle, int32>* EntityToIndex = Self->GetGroupEntityToIndex(QueryGroup);
						if (!TargetISMC || !MES || !NavSubsystem || !EntityToIndex)
						{
							return 0;
						}

						FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
						const FNavigationObstacleHashGrid2D& Grid = NavSubsystem->GetObstacleGrid();

						return ExecuteGridHashEnumerate(
							EntityManager, Grid, *EntityToIndex, *TargetISMC,
							Center, Radius, Out, Max);
					},
					Radius);
			}
```

- [ ] **Step 2: Build + smoke-run current automation tests**

Run: `cd /Users/tonialatalo/src/unreal-rust/example && ./build-plugin.sh`
Expected: C++ builds.

Run (per the "UE loader target dir" memory — keep `UNREAL_RUST_TARGET_DIR` set):

```bash
cd /Users/tonialatalo/src/unreal-rust/example && \
  UNREAL_RUST_TARGET_DIR="$PWD/.rust-target" \
  $UE_ROOT/Engine/Binaries/Mac/UnrealEditor-Cmd \
  "$PWD/VivariumExample/VivariumExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Vivarium.VivariumBirdsFlockAndStayBounded;Quit" \
  -unattended -nopause -nullrhi
```

Expected: `EXIT CODE: 0` (vivarium still uses the old mirror at this stage — the new enumerate path is wired but not exercised).

- [ ] **Step 3: Commit**

```bash
git add RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp
git commit -m "feat(RustPlugin): GridHashEnumerate config branch dispatches to enumerate trampoline"
```

### Task 3.3: Build enumerate slots in `SetupProcessorPipelines` + `RustMassScheduleCoordinator`

**Files:**
- Modify: `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` (around line 670 — the `if (SpatialQueries.Num() > 0)` block)
- Modify: `RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.{h,cpp}` (add `SetSpatialEnumerateSlots` alongside `SetSpatialQuerySlots`)

- [ ] **Step 1: Add `SetSpatialEnumerateSlots` to the coordinator header**

Edit `RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.h`. After `SetSpatialQuerySlots` declaration (line 31):

```cpp
	void SetSpatialEnumerateSlots(TArray<MassSpatialEnumerateSlot> InSlots, TArray<TArray<char>> InNameBuffers);
```

Alongside `SpatialQuerySlots` member (line 45-46), add:

```cpp
	TArray<MassSpatialEnumerateSlot> SpatialEnumerateSlots;
	TArray<TArray<char>> SpatialEnumerateNameBuffers;
```

- [ ] **Step 2: Implement `SetSpatialEnumerateSlots` + populate the dispatch data**

Edit `RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.cpp`. After `SetSpatialQuerySlots` (line 41), insert:

```cpp
void URustMassScheduleCoordinator::SetSpatialEnumerateSlots(
	TArray<MassSpatialEnumerateSlot> InSlots,
	TArray<TArray<char>> InNameBuffers)
{
	SpatialEnumerateNameBuffers = MoveTemp(InNameBuffers);
	SpatialEnumerateSlots = MoveTemp(InSlots);
	for (int32 i = 0; i < SpatialEnumerateSlots.Num(); ++i)
	{
		SpatialEnumerateSlots[i].name.ptr = SpatialEnumerateNameBuffers[i].GetData();
	}
}
```

In `Execute` (around line 90-97), replace the `MassFrameDispatchData` literal:

```cpp
	MassFrameDispatchData Data;
	Data.dt = DeltaSeconds;
	Data.num_systems = static_cast<uint32>(Batches.Num());
	Data.systems = Batches.GetData();
	Data.num_spatial_queries = static_cast<uint32>(SpatialQuerySlots.Num());
	Data.num_spatial_enumerates = static_cast<uint32>(SpatialEnumerateSlots.Num());
	Data.spatial_queries = SpatialQuerySlots.GetData();
	Data.spatial_enumerates = SpatialEnumerateSlots.GetData();
```

- [ ] **Step 3: Build the enumerate slot array in `SetupProcessorPipelines`**

Edit `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`. After the `if (SpatialQueries.Num() > 0)` block (immediately after line 705, before line 706 `SimProcessors.Add(ScheduleCoordinator);`):

```cpp
			if (SpatialEnumerates.Num() > 0)
			{
				TArray<MassSpatialEnumerateSlot> Slots;
				TArray<TArray<char>> NameBuffers;
				int32 SlotIdx = 0;
				for (auto& Pair : SpatialEnumerates)
				{
					if (SlotIdx >= RustMassSpatialEnumerate::MaxEnumerates)
					{
						UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Too many spatial enumerates (%d), max is %d. Excess enumerates will be silently ignored — increase MaxEnumerates."),
							SpatialEnumerates.Num(), RustMassSpatialEnumerate::MaxEnumerates);
						ensureMsgf(false, TEXT("Spatial enumerate count %d exceeds MaxEnumerates %d"),
							SpatialEnumerates.Num(), RustMassSpatialEnumerate::MaxEnumerates);
						break;
					}

					Pair.Value.TrampolineIndex = SlotIdx;
					RustMassSpatialEnumerate::ActiveCallbacks[SlotIdx] =
						[CBPtr = &Pair.Value](const double* Center, float Radius,
							MassSpatialNeighbor* Out, uint32 Max) -> uint32
						{
							return CBPtr->Callback(Center, Radius, Out, Max);
						};

					FTCHARToUTF8 Converter(*Pair.Key);
					TArray<char> NameBuf;
					NameBuf.Append(Converter.Get(), Converter.Length());
					NameBuf.Add(0);

					MassSpatialEnumerateSlot Slot = {};
					Slot.name.ptr = nullptr; // patched up after MoveTemp
					Slot.name.len = Converter.Length();
					Slot.enumerate_fn = RustMassSpatialEnumerate::TrampolineFns[SlotIdx];
					Slot.radius = Pair.Value.Radius;
					Slot._pad = 0;
					Slots.Add(Slot);
					NameBuffers.Add(MoveTemp(NameBuf));
					++SlotIdx;
				}
				ScheduleCoordinator->SetSpatialEnumerateSlots(MoveTemp(Slots), MoveTemp(NameBuffers));
			}
```

Do the same pattern for `SpatialQueries.Num() > 0` — look at the sweep block at lines 671-704 and confirm the existing code installs `ActiveCallbacks[SlotIdx] = ...` before handing out the trampoline pointer. If the sweep path already does this (trampolines dispatch through `ActiveCallbacks`), no changes are needed there.

- [ ] **Step 4: Build + smoke test**

Run: `cd /Users/tonialatalo/src/unreal-rust/example && ./build-plugin.sh`
Expected: PASS.

Run the existing UE automation (vivarium still on old mirror — should still work):

```bash
cd /Users/tonialatalo/src/unreal-rust/example && \
  UNREAL_RUST_TARGET_DIR="$PWD/.rust-target" \
  $UE_ROOT/Engine/Binaries/Mac/UnrealEditor-Cmd \
  "$PWD/VivariumExample/VivariumExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Vivarium.VivariumBirdsFlockAndStayBounded;Quit" \
  -unattended -nopause -nullrhi
```

Expected: `EXIT CODE: 0`.

- [ ] **Step 5: Commit**

```bash
git add RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.h \
        RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.cpp \
        RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp
git commit -m "feat(RustPlugin): populate MassSpatialEnumerateSlot array each frame"
```

---

## Slice 4 — Vivarium migration

Switch vivarium from the mirror to `SpatialGroupPlugin`. Exercises the full round-trip end-to-end. Verification: all Rust tests green + `VivariumBirdsFlockAndStayBounded` UE automation green + PIE visual smoke.

### Task 4.1: Add `install_plugins` entry point to `vivarium-sim`

**Files:**
- Modify: `vivarium-sim/src/lib.rs`

- [ ] **Step 1: Add the install function**

Edit `vivarium-sim/src/lib.rs`. Replace the whole file with:

```rust
pub mod bird;
pub mod boundary;
pub mod brownian;
pub mod components;
pub mod config;

#[cfg(feature = "unreal")]
pub mod unreal;

/// Install this sim's Bevy plugins into `app`. Called from both the
/// standalone binary (directly from `main`) and the UE app (via
/// `MassAppPluginRegistration` in `vivarium-sim/src/unreal/mod.rs`).
pub fn install_plugins(app: &mut bevy_app::App) {
    use bevy_mass::prelude::*;
    use components::{Bird, Transform};
    use config::FLOCK_NEIGHBOR_RADIUS;

    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
        "birds",
        FLOCK_NEIGHBOR_RADIUS,
    ));
}
```

- [ ] **Step 2: Verify build (no callers yet)**

Run: `cargo build -p vivarium-sim`
Expected: PASS.

Run: `cargo build -p vivarium-sim --features unreal --no-default-features`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add vivarium-sim/src/lib.rs
git commit -m "feat(vivarium-sim): install_plugins entry point"
```

### Task 4.2: Migrate `bird.rs` — delete `rebuild_bird_grid_system` + fix tests

**Files:**
- Modify: `vivarium-sim/src/bird.rs`

- [ ] **Step 1: Delete the rebuild system**

Edit `vivarium-sim/src/bird.rs`. Delete lines 45-61 (the doc comment block and `rebuild_bird_grid_system` fn). Also delete line 22 (`use bevy_mass::spatial_query::SpatialGrids;`) — the import is no longer needed.

Update the module-level doc (lines 1-13) to drop the `rebuild_bird_grid_system` mention:

Replace lines 1-13 with:

```rust
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
```

- [ ] **Step 2: Fix each flocking test to install the plugin + use `in_set(Query)`**

Edit `vivarium-sim/src/bird.rs`. In the `tests` module at the top after `fn run<M>(...)`, add a helper:

```rust
    use bevy_mass::prelude::{SpatialGroupPlugin, SpatialGroupSet};
    use bevy_app::App;

    fn run_with_plugin<M>(world: &mut World, system: impl IntoSystem<(), (), M>) {
        let mut app = App::new();
        // Move resources + entities into an App so the plugin's rebuild
        // system can run in SpatialGroupSet::Rebuild. We re-create the
        // world inside the App from the caller-supplied one.
        std::mem::swap(app.world_mut(), world);
        app.add_plugins(bevy_app::MinimalPlugins);
        app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
            "birds",
            FLOCK_NEIGHBOR_RADIUS,
        ));
        app.add_systems(bevy_app::Update, system.in_set(SpatialGroupSet::Query));
        app.update();
        std::mem::swap(app.world_mut(), world);
    }
```

Replace each occurrence of

```rust
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems((rebuild_bird_grid_system, flocking_system).chain());
        schedule.run(&mut world);
```

(in `flocking_cohesion_pulls_toward_centroid`, `flocking_separation_pushes_apart`, `flocking_alignment_averages_headings`, `flocking_no_neighbors_no_change`) with:

```rust
        run_with_plugin(&mut world, flocking_system);
```

Also drop the `world.insert_resource(SpatialGrids::default())` lines (the plugin installs the resource).

Remove the `use crate::components::PreviousTranslation;` → `SpatialGrids` import if redundant. Keep the `use super::*;` at the top of the tests module.

- [ ] **Step 3: Run the bird tests**

Run: `cargo test -p vivarium-sim bird::tests`
Expected: PASS — all four flocking tests plus `wander_preserves_speed`.

- [ ] **Step 4: Commit**

```bash
git add vivarium-sim/src/bird.rs
git commit -m "refactor(vivarium-sim): bird tests use SpatialGroupPlugin instead of manual rebuild"
```

### Task 4.3: Delete the UE-mode inventory submissions the plugin replaces

**Files:**
- Modify: `vivarium-sim/src/unreal/mod.rs`

- [ ] **Step 1: Delete the `MassSpatialQueryConfigRegistration` submit for birds**

Edit `vivarium-sim/src/unreal/mod.rs`. Delete lines 53-67 (the `inventory::submit!(MassSpatialQueryConfigRegistration { query_name: "birds", ... })` block plus its leading comment).

- [ ] **Step 2: Remove `"rebuild_bird_grid_system"` from `MassScheduleOrder`**

Edit `vivarium-sim/src/unreal/mod.rs`. In `MassScheduleOrder` at lines 28-36, replace the `systems` slice:

```rust
inventory::submit!(unreal_api::mass::MassScheduleOrder {
    systems: &[
        "brownian_motion_system",
        "wander_system",
        "flocking_system",
        "boundary_force_system",
    ],
});
```

Update the preceding comment (lines 22-27) to drop the `rebuild_bird_grid_system` note:

```rust
// Explicit pipeline order. Insect brownian + bird wander / flocking run
// before the boundary force field clamps/repels near the bounds. Position
// integration is performed by C++ `UMassSimpleMovementProcessor` after the
// Bevy schedule dispatch, so no Rust movement system is listed here.
// `flocking_system` runs in `SpatialGroupSet::Query` after the framework
// rebuilds the grid (Bevy mode) or consults UE's hash grid (UE mode).
```

- [ ] **Step 3: Delete the stale `bird_spatial_query_config_registered` test**

Edit `vivarium-sim/src/unreal/mod.rs`. Delete lines 130-141 (the `bird_spatial_query_config_registered` test).

- [ ] **Step 4: Submit `MassAppPluginRegistration` for vivarium-sim**

Edit `vivarium-sim/src/unreal/mod.rs`. After the existing `MassSimInitRegistration` submit at lines 17-20, insert:

```rust
// App-level Bevy plugins installed into the shadow MassSchedule before
// entities are spawned. Same function the standalone `main.rs` calls
// directly, so standalone and UE register the same plugin set from a
// single source of truth.
inventory::submit!(unreal_api::mass::MassAppPluginRegistration {
    build_fn: crate::install_plugins,
});
```

- [ ] **Step 5: Run Rust tests (unreal feature)**

Run: `cargo test -p vivarium-sim --features unreal --no-default-features`
Expected: PASS — schedule-builds smoke test still green (no more `rebuild_bird_grid_system` so the systems list contracts; `bird_spatial_query_config_registered` is gone, so nothing to fail).

- [ ] **Step 6: Commit**

```bash
git add vivarium-sim/src/unreal/mod.rs
git commit -m "refactor(vivarium-sim): drop manual spatial config; submit MassAppPluginRegistration"
```

### Task 4.4: Standalone — install plugin, drop mirror

**Files:**
- Modify: `vivarium-standalone/src/main.rs`

- [ ] **Step 1: Replace the mirror+chain with a plugin install**

Edit `vivarium-standalone/src/main.rs`. Replace lines 83-106 (the `app.add_plugins(...).add_systems(...)` block) with:

```rust
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin::<SimTransform, PreviousTranslation, Velocity>::default())
        .insert_resource(capture)
        .insert_resource(SimBounds::default())
        .init_resource::<FrameCounter>();

    // Install sim-owned plugins (SpatialGroupPlugin::<Bird, Transform>::new("birds", ...), etc.)
    // through the same entry point the UE-mode `MassAppPluginRegistration` uses.
    vivarium_sim::install_plugins(&mut app);

    use bevy_mass::prelude::SpatialGroupSet;
    app.add_systems(
            Startup,
            (setup_scene, spawn_insects, spawn_birds).chain(),
        )
        .add_systems(
            Update,
            (
                brownian_motion_system,
                wander_system,
                flocking_system.in_set(SpatialGroupSet::Query),
                boundary_force_system,
                sync_render_transforms,
                capture_scheduled_screenshots,
            )
                .chain(),
        );
```

Delete the `use bevy_mass::spatial_query::SpatialGrids;` import at line 17 (no longer needed) and the `rebuild_bird_grid_system` import in the `vivarium_sim::bird::` use list at line 19.

Replace line 19 (the `use vivarium_sim::bird::` line) with:

```rust
use vivarium_sim::bird::{flocking_system, wander_system};
```

- [ ] **Step 2: Run the standalone (smoke test)**

Run: `cargo run -p vivarium-standalone --release -- --deterministic-clock --exit-after 60 --screenshot-at 30 --out-dir /tmp/vivarium_smoke`
Expected: Window opens briefly, exits cleanly. Open `/tmp/vivarium_smoke/vivarium_frame_0030.png` and confirm insects + birds are visible in motion (no crashes, birds cluster).

- [ ] **Step 3: Commit**

```bash
git add vivarium-standalone/src/main.rs
git commit -m "refactor(vivarium-standalone): use SpatialGroupPlugin via install_plugins"
```

### Task 4.5: Rebuild UE dylib + run full UE automation

**Files:** None (verification only).

- [ ] **Step 1: Rebuild the vivarium release dylib**

Run (per `reference_testing.md` memory):

```bash
cargo build --release -p vivarium-rust-host
```

Expected: PASS.

- [ ] **Step 2: Rebuild the C++ plugin**

Run: `cd /Users/tonialatalo/src/unreal-rust/example && ./build-plugin.sh`
Expected: PASS.

- [ ] **Step 3: Run `VivariumBirdsFlockAndStayBounded` from CLI**

Run:

```bash
cd /Users/tonialatalo/src/unreal-rust/example && \
  UNREAL_RUST_TARGET_DIR="$PWD/.rust-target" \
  $UE_ROOT/Engine/Binaries/Mac/UnrealEditor-Cmd \
  "$PWD/VivariumExample/VivariumExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Vivarium.VivariumBirdsFlockAndStayBounded;Quit" \
  -unattended -nopause -nullrhi
```

Expected: `EXIT CODE: 0`. If it fails, check the log for
- `"refused GridHash ownership"` — group conflict between two claims; check the new `SetupSpatialQueriesFromRust/Enumerate` branch isn't double-registering the `"birds"` group.
- `"SpatialQuery::call: no MassSpatialQueryConfigRegistration for query_name='birds'"` — cache bridge isn't initialising; verify `register_spatial_group_cache_accessor` is called in `init_global_schedule`.
- `"no method named enumerate"` — Slice 2 regressions.

- [ ] **Step 4: Run the full Vivarium automation group**

Run (replaces the single test with the group filter):

```bash
cd /Users/tonialatalo/src/unreal-rust/example && \
  UNREAL_RUST_TARGET_DIR="$PWD/.rust-target" \
  $UE_ROOT/Engine/Binaries/Mac/UnrealEditor-Cmd \
  "$PWD/VivariumExample/VivariumExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Vivarium;Quit" \
  -unattended -nopause -nullrhi
```

Expected: `EXIT CODE: 0` — all three existing vivarium tests
(`VivariumBirdsFlockAndStayBounded`, `VivariumSpawnAndSimulate`, `VivariumInsectsStayBounded`) pass.

- [ ] **Step 5: PIE visual smoke**

Run:

```bash
./scripts/pie.sh vivarium
```

Expected: Editor launches PIE, birds visible (larger orange spheres), cluster in small flocking groups within bounds, no crash on spawn or frame dispatch. Exit PIE with `Shift+Esc`.

- [ ] **Step 6: Commit (if any log/config tweaks needed)**

If everything passes cleanly, nothing new to commit here. Otherwise:

```bash
git add -A
git commit -m "chore(vivarium): post-migration log / config adjustments"
```

---

## Out-of-scope reminders

Carried over from the spec — explicitly **not** in this plan:

- Gatherers sweep migration to a `SweepPlugin` cousin.
- Moving Bevy-mode cell grid to UE's hash-grid backing.
- Per-entity C++-side filtering on enumerate.
- 3D hash grid (UE ships 2D — Z handled by the `r*r` filter, which is a superset but correct).

---

## Verification summary

After all slices:

1. `cargo test -p bevy_mass` — plugin unit tests green.
2. `cargo test -p bevy_mass --features unreal --no-default-features` — UE-mode build green.
3. `cargo test -p unreal-ffi` — layout tests green (incl. new ones).
4. `cargo test -p unreal-api` — incl. new `enumerate` + `app_plugin_registration` tests green.
5. `cargo test -p unreal-module` — dispatch-hook + schedule-reset tests still green.
6. `cargo test -p vivarium-sim` — bird tests use plugin; wander + 4 flocking cases green.
7. `cargo test -p vivarium-sim --features unreal --no-default-features` — UE-bridge registrations + schedule smoke green.
8. `cargo run -p vivarium-standalone` — visual smoke.
9. `cargo build --release -p vivarium-rust-host` — UE dylib builds.
10. `./build-plugin.sh` — C++ plugin builds.
11. UE CLI automation `supplemental.RustPlugin.Vivarium` — EXIT CODE 0.
12. `./scripts/pie.sh vivarium` — birds flock visibly in PIE.
