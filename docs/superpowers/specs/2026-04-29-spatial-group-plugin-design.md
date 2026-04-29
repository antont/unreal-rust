# SpatialGroupPlugin — idiomatic Bevy surface for UE hash-grid enumeration

## Context

Phase 2a (birds) shipped `SpatialQueries::neighbors_within(name, center, radius,
exclude)` with a Bevy-side cell grid (`SpatialGrids` resource) as the backend in
*both* modes. In UE mode the sim runs a `rebuild_bird_grid_system` every frame
that iterates every bird entity and reinserts it into the Rust-side grid — a
complete mirror of information UE's `FNavigationObstacleHashGrid2D` already
holds for GridHash-owned groups.

For 20 birds this is free. The framework goal is 10k entities per group × a
dozen groups — at that scale the mirror is 120k position reads per frame of
pure duplication, plus a scheduled Rust system whose only purpose is to tell
UE what UE already knows.

The architectural goal of this work is to drop the mirror in UE mode and route
`neighbors_within` directly into the engine's hash grid, while keeping the
authoring surface for sim authors pure Bevy (plugin-based). unreal-rust is
meant to let sims author in idiomatic Bevy and have the framework fan
registrations out to UE behind the scenes — the sim crate should never write
`#[cfg(feature = "unreal")]` branches or `inventory::submit!` calls for this
feature. That constraint shapes the whole surface.

Vivarium is the initial consumer. Gatherers' sweep path (`SpatialQueries::call`)
is unaffected — the cleanup of its manual `inventory::submit!
(MassSpatialQueryConfigRegistration { ... })` is deferred to a follow-up.

## Approach

Introduce `SpatialGroupPlugin<Marker, Pos>`, a Bevy plugin that is the sole
authoring surface for registering an enumerable spatial group:

```rust
app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
    "birds",
    FLOCK_NEIGHBOR_RADIUS,
));
```

- `Marker: Component` — group membership predicate (the sim's `Bird` tag).
- `Pos: Component + TransformLike` — reuses the existing trait from
  `bevy_mass::movement`; vivarium's `Transform` already implements it.
- `name: &'static str` — the name passed to `neighbors_within(name, ...)`.
- `radius: f64` — both the default cell size in Bevy mode and the hash-grid
  registration radius in UE mode.

### Shared registry

One name-keyed registry serves both modes. Lives in a framework resource:

```rust
#[derive(Resource, Default)]
pub struct SpatialGroupRegistry {
    entries: Vec<SpatialGroupEntry>,
}

pub struct SpatialGroupEntry {
    pub name: &'static str,
    pub radius: f64,
    pub marker_tag_cpp_name: &'static str,   // M::CPP_TYPE_NAME
    pub position_fragment_cpp_name: &'static str, // P::CPP_TYPE_NAME
}
```

Plugin `build()` pushes one entry with `name` as the primary key. Re-installing
the same name is a panic (authoring error — a single sim can't register two
meanings for one group name).

Using name as the key (not a type pair) means multiple `<Marker, Pos>` shapes
can register into the same registry structure, and lookups from FFI handlers
are a simple `.find(|e| e.name == n)` instead of a type-erased resource dance.

### Bevy mode

`Plugin::build`:

1. Inserts `SpatialGrids` resource if absent (idempotent).
2. Inserts `SpatialGroupRegistry` resource if absent; pushes one entry.
3. Defines a framework-internal `SpatialGroupSet` enum with two variants:
   `Rebuild`, `Query`. Configures `Query` to run `.after(Rebuild)`.
4. Registers a monomorphised `rebuild_grid_system::<Marker, Pos>` in
   `SpatialGroupSet::Rebuild` during `Update`. The system reads its group
   name from a `const GROUP: &'static str` baked into the plugin struct —
   the plugin constructor stores `name` into a generated
   `PhantomData`-keyed newtype resource, or passes it via closure capture,
   to avoid a runtime name lookup per frame. (Exact mechanism: plan phase;
   the constraint is zero-alloc per-frame name resolution.)

   ```rust
   fn rebuild_grid_system<M, P>(
       mut grids: ResMut<SpatialGrids>,
       entities: Query<(Entity, &P), With<M>>,
       group: Res<PerGroupMeta<(M, P)>>,  // holds name + radius for this (M, P) pair
   ) where M: Component, P: Component + TransformLike {
       let grid = grids.grid_mut(group.name, group.radius);
       grid.clear();
       for (e, p) in &entities {
           grid.insert(e, p.translation());
       }
   }
   ```

Sim authors add their flocking/etc. system with
`.in_set(SpatialGroupSet::Query)` and the ordering constraint is automatic.

Caveat: `PerGroupMeta<(M, P)>` is keyed by type pair — one plugin instance per
`(M, P)`. This is fine for vivarium (`Bird + Transform` is a unique pair). Sims
wanting to register two separate groups with the same component types (e.g.
two different insect factions using the same `Transform`) need distinct marker
types. This is a deliberate trade for type-safe zero-cost name resolution;
we'll revisit if a real use case demands it.

### UE mode — plugin installation path

Today's UE-mode flow: sim systems reach `MassSchedule`'s Bevy app via
`inventory::submit!(MassBevySystemRegistration)` — unreal-module iterates the
collection and calls each `add_to_app` fn pointer on its MassSchedule. The
sim never holds the App handle itself.

Plugins need a parallel hook. The framework gains one new inventory type
plus a single install_plugins fn per sim crate:

```rust
// unreal-api/src/mass.rs — lives alongside the existing inventory
// registrations (MassBevySystemRegistration, MassScheduleOrder, etc.)
pub struct MassAppPluginRegistration {
    pub build_fn: fn(&mut bevy_app::App),
}
inventory::collect!(MassAppPluginRegistration);

// vivarium-sim — one function, called from both standalone and UE:
pub fn install_plugins(app: &mut bevy_app::App) {
    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
        "birds", FLOCK_NEIGHBOR_RADIUS,
    ));
    // future SpatialGroupPlugin calls — one per group — go here.
}

inventory::submit!(MassAppPluginRegistration { build_fn: install_plugins });
```

`vivarium-standalone/src/main.rs` calls `vivarium_sim::install_plugins(&mut
app)` directly, as a peer of its current `app.add_plugins(MovementPlugin::…)`
call. unreal-module runs registered plugins during MassSchedule setup.

This adds one line of framework tax per sim crate (one `install_plugins` fn
+ one inventory submit), at the gain of: no per-group duplication, and the
authoring surface for each new spatial group is one `.add_plugins(...)` line
added to `install_plugins`.

### UE mode — what `Plugin::build` does

In UE mode, `SpatialGroupPlugin::<M, P>::build(app)` does *not* install a
rebuild system. It:

1. Inserts `SpatialGroupRegistry` resource if absent.
2. Pushes `SpatialGroupEntry` with `marker_tag_cpp_name = M::CPP_TYPE_NAME`
   and `position_fragment_cpp_name = P::CPP_TYPE_NAME`. This requires the
   UE-mode plugin impl to constrain `M: MassFragment` and
   `P: MassFragment + TransformLike` — stronger than the Bevy-mode bound,
   enforced by `#[cfg(feature = "unreal")]` on a second `Plugin` impl.

### UE mode — how C++ sees the registry

This is the piece that needs an explicit path: `SpatialGroupRegistry` lives in
the `MassSchedule` world, but the FFI handlers
`get_spatial_query_config_count` / `get_spatial_query_config_desc` in
`unreal-module` are plain fns that don't have direct access to the world.

Solution: module-level cache, written each time `mass_init_simulation` runs
plugin builds against the existing MassSchedule app. The sequence:

1. **Dylib load (existing):** `unreal-module` constructs `MassSchedule`. The
   App exists at this point but plugin builds have not yet run.
2. **`mass_init_simulation` runs (entry point for each sim start):**
   - Iterate `inventory::iter::<MassAppPluginRegistration>()` and call each
     `build_fn(app)` on the existing MassSchedule app — this is the hook
     the spec adds. Idempotent per sim start; subsequent starts overwrite.
     `SpatialGroupPlugin::build` pushes entries into `SpatialGroupRegistry`.
   - Copy the finished `SpatialGroupRegistry` contents into a module-level
     `OnceLock<Mutex<Vec<SpatialGroupEntry>>>` in `unreal-module`. The
     `Mutex` allows replacement on re-init (test harness `reset_sim` path +
     future hot-reload); the `OnceLock` initialises it lazily on first
     write. Same spirit as `register_shadow_world_accessors`, upgraded for
     re-populatable cache.
3. **C++ `SetupSpatialQueriesFromRust` (existing, runs strictly after
   `mass_init_simulation` returns):** calls
   `get_spatial_query_config_count/desc`, which now read from *two* sources
   and concatenate:
   - Legacy: `inventory::iter::<MassSpatialQueryConfigRegistration>()` —
     gatherers' sweep configs, unchanged.
   - New: the module-level cache, synthesising one
     `MassSpatialQueryConfigDesc` per entry with `query_type =
     GridHashEnumerate` (new enum value).
4. C++ recognises the new `GridHashEnumerate` value and installs a per-group
   enumerate callback into a new per-frame slot (see "FFI — enumerate slot"
   below).

Ordering is already correct — no construction-order shuffling needed. The
new step is strictly additive inside `mass_init_simulation`.

### FFI — enumerate slot (direction matches existing sweep)

The existing sweep FFI is a C++ callback passed *into* Rust each frame via
`MassSystemChunkBatch.spatial_queries` (an array of `MassSpatialQuerySlot`).
Enumerate follows the same direction: C++ populates a parallel enumerate
callback slot that Rust calls during system execution.

New FFI structs in `unreal-ffi/src/lib.rs`:

```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MassSpatialNeighbor {
    pub entity_index: i32,
    pub _pad: i32,
    pub position: [f64; 3],
}

/// Enumerate all members of a named spatial group within `radius` of `center`.
/// C++ writes `(entity_index, position)` pairs into `out[0..min(count, max)]`
/// and returns the total count that would have been written if max were
/// infinite — so Rust can detect truncation by testing `returned > max`.
pub type MassSpatialEnumerateFn = unsafe extern "C" fn(
    center: *const f64,     // [f64; 3]
    radius: f32,
    out: *mut MassSpatialNeighbor,
    max: u32,
) -> u32;

/// One enumerate slot (parallel to the existing sweep `MassSpatialQuerySlot`).
#[repr(C)]
pub struct MassSpatialEnumerateSlot {
    pub name: Utf8Str,
    pub enumerate_fn: MassSpatialEnumerateFn,
    pub radius: f32,
    pub _pad: u32,
}
```

`MassFrameDispatchData` gains two new fields: `num_spatial_enumerates` and
`spatial_enumerates: *const MassSpatialEnumerateSlot`. C++ populates the
slot array in `URustMassBevySubsystem::RunSimulationProcessorStep` alongside
the sweep slots. One enumerate callback per registered group (bound by
group name via closure capture, same mechanism as the sweep path).

Per-frame dispatch in Rust stashes the enumerate slots into
`MassSpatialQueries` (the existing Bevy resource) under a new `enumerates:
HashMap<String, (MassSpatialEnumerateFn, f32)>` field, alongside the
existing `queries` (sweep) map.

### UE mode — `SpatialQueries::neighbors_within`

Calls `MassSpatialQueries::enumerate(name, center)`, which:
1. Looks up the per-frame callback by name.
2. Calls it into a `SmallVec<[MassSpatialNeighbor; 64]>` buffer, resizing
   and retrying once if truncated.
3. Resolves each `entity_index` via `MassEntityMap`.
4. Returns `Vec<SpatialNeighbor>`.

Drops the `grids: Option<Res<SpatialGrids>>` field from the UE-mode
`SystemParam`. `SpatialGrids` stays defined but is only used in Bevy mode.

`name_to_group()` cache in `spatial_query.rs unreal_impl` currently reads
only `registered_spatial_query_configs()` (inventory). It needs to also
read the module-level `SpatialGroupEntry` cache so
`resolve_hit_entity(name, ...)` works for enumerate-registered groups too
— relevant only if we ever call sweep against an enumerate-only group,
but trivial to fix and prevents a silent `Entity::PLACEHOLDER` footgun.
Cleanest: add a single unified accessor
`unreal_module::spatial_name_to_group(name) -> Option<&'static str>` that
both cache-populates from.

### C++ enumerate callback (implementation)

Mirrors `ExecuteGridHashSpatialQuery` in
`RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`:

1. Look up the group's ISMC + `EntityToIndex` + `FNavigationObstacleHashGrid2D`.
2. `Grid.QuerySmall(BoundsAroundCenter(center, radius), Candidates)`.
3. For each candidate: validate entity → reverse-map → read transform →
   `DistSquared(pos, center) <= r*r` filter (no closest-point-on-segment).
4. Write `{entity_index, position}` into `out[0..min(count, max)]`; return
   the uncapped count.

### Migration — vivarium

Delete:

- `rebuild_bird_grid_system` (removed from `vivarium-sim/src/bird.rs`).
- Its entry in `MassScheduleOrder` (removed from `vivarium-sim/src/unreal/mod.rs`).
- `.insert_resource(SpatialGrids::default())` in `vivarium-standalone/src/main.rs`.
- The `.chain()` rebuild→flocking ordering in standalone `main.rs`.
- The hand-authored `inventory::submit!(MassSpatialQueryConfigRegistration {
  query_name: "birds", ... })` in `vivarium-sim/src/unreal/mod.rs`.

Add:

- New `vivarium_sim::install_plugins(app: &mut App)` function that calls
  `app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new("birds",
  FLOCK_NEIGHBOR_RADIUS))`.
- `inventory::submit!(MassAppPluginRegistration { build_fn: install_plugins
  })` for UE mode discovery.
- `vivarium-standalone/src/main.rs` calls `vivarium_sim::install_plugins
  (&mut app)` alongside the existing `MovementPlugin` plugin call.
- `flocking_system` gets `.in_set(SpatialGroupSet::Query)` in standalone
  `main.rs`; `rebuild_bird_grid_system` reference and the `.chain()`
  around it are removed.

### SpatialGrids resource becomes Bevy-only

Once UE mode no longer reads `SpatialGrids`, move its type definition behind
`#[cfg(not(feature = "unreal"))]` in `bevy_mass/src/spatial_query.rs`, or
leave it defined cross-mode but document it as "Bevy-mode backing store for
`SpatialGroupPlugin`; unused in UE mode." Decision during implementation —
doesn't affect authoring surface either way.

## Testing

**Bevy-mode unit tests (`bevy_mass`):**
- `SpatialGroupPlugin<M, P>` rebuilds the grid once per frame: spawn 3
  entities with positions, run one `Update`, assert
  `spatial.neighbors_within("group", ...)` returns them.
- Ordering: a system in `SpatialGroupSet::Query` sees the rebuilt grid (fails
  under the old unordered pattern).
- Multiple groups coexist: install the plugin twice with different `Marker`
  types, assert both grids populate independently.

**Bevy-mode vivarium unit tests:**
- `bird.rs` flocking tests keep working after `rebuild_bird_grid_system`
  deletion. The tests currently `.chain()` the rebuild explicitly; they need
  to be updated to install `SpatialGroupPlugin` and use `.in_set(Query)`
  ordering instead.

**Framework UE test (new):**
- A self-contained UE automation test (in `bevy_mass` or a nearby crate)
  that:
  1. Spawns 50 entities via a minimal "birds"-style group using
     `SpatialGroupPlugin::<TestMarker, TestTransform>::new("test_group", R)`.
  2. Places them at known positions (write fragments via the test
     callback).
  3. Calls `neighbors_within("test_group", center, R)` through the
     `SpatialQueries` SystemParam.
  4. Asserts the expected subset is returned (spot-checks count, and
     verifies identity of a few specific hits).

This is the one test that proves the FFI round-trip is wired. If we don't
add it, regressions in `enumerate_in_radius` only surface via the existing
`VivariumBirdsFlockAndStayBounded`, which is too coarse to localize the
fault.

**Vivarium UE test:**
- `VivariumBirdsFlockAndStayBounded` must still pass unchanged — it's the
  end-to-end acceptance gate. No modifications to the test body; only the
  internal wiring changes.

**Bevy-mode `SpatialGrid` tests** (existing in `spatial_query.rs`) are
unchanged.

## Critical files

**To create:**
- `bevy_mass/src/spatial_group.rs` — `SpatialGroupPlugin<M, P>`,
  `SpatialGroupSet` enum, `SpatialGroupMeta<M, P>` resource,
  `rebuild_grid_system` monomorph.

**To modify:**
- `bevy_mass/src/lib.rs` — `pub mod spatial_group;` + re-exports.
- `bevy_mass/src/spatial_query.rs` — drop the `grids: Option<Res<...>>`
  SystemParam field in UE mode; replace `neighbors_within` body with an FFI
  call; in Bevy mode it still reads `SpatialGrids` but via the plugin.
- `unreal-ffi/src/lib.rs` — add `MassSpatialNeighbor`,
  `MassSpatialEnumerateFn`, `MassSpatialEnumerateSlot`; add
  `num_spatial_enumerates: u32` + `spatial_enumerates: *const
  MassSpatialEnumerateSlot` fields to `MassFrameDispatchData`. (No new
  `RustBindings` entry — enumerate slots flow C++→Rust per frame, not as
  Rust exports.)
- `unreal-api/src/mass.rs` — expose `MassSpatialQueries::enumerate(name,
  center, radius) -> Vec<SpatialNeighbor>` alongside the existing `call`;
  wire the new FFI pointer into per-frame dispatch (or to a subsystem-level
  extern).
- `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.{h,cpp}` — add
  `ExecuteGridHashEnumerate` (cousin of `ExecuteGridHashSpatialQuery`);
  register an enumerate callback per group inside
  `SetupSpatialQueriesFromRust`; expose the FFI trampoline.
- `RustPlugin/Source/RustPlugin/RustMassScheduleCoordinator.{h,cpp}` — if
  the enumerate callback lives alongside the sweep trampolines, wire it
  the same way.

**Vivarium migration:**
- `vivarium-sim/src/bird.rs` — delete `rebuild_bird_grid_system` and its
  tests' `.chain()` ordering; use `SpatialGroupSet::Query` instead.
- `vivarium-sim/src/unreal/mod.rs` — delete the bird
  `MassSpatialQueryConfigRegistration` submit; delete
  `"rebuild_bird_grid_system"` from `MassScheduleOrder`.
- `vivarium-standalone/src/main.rs` — remove `.insert_resource(SpatialGrids::
  default())`, add `SpatialGroupPlugin::<Bird, SimTransform>::new("birds",
  FLOCK_NEIGHBOR_RADIUS)` to the plugin list.
- UE-mode plugin installation: whichever path the vivarium-sim crate uses
  today to install bevy-mass plugins in the UE app gets one more plugin
  added (determined during implementation — likely alongside
  `MovementPlugin` in the existing UE App builder).

**Gatherers:** untouched this round.

## Deliberately out of scope

- **Gatherers sweep migration.** Its sweep configs stay as hand-authored
  inventory submits. Follow-up will consider a `SweepPlugin` cousin, but
  sweeps have collision channels / filter fragments that don't map cleanly
  onto the enumerate surface.
- **Moving cell-grid logic to UE's hash grid in Bevy mode.** The Bevy
  backend stays as-is — a simple cell grid is plenty for standalone.
- **Filtering on enumerate.** Enumerate returns all members of the group
  within radius; per-entity filter predicates are applied in the sim code
  after the call. If a concrete sim needs C++-side filtering later (e.g.
  skip asleep birds), it's a separate feature.
- **2D/3D grid choice.** UE's `FNavigationObstacleHashGrid2D` is 2D; this
  is a known limitation carried over from the sweep path. Vivarium flocking
  is genuinely 3D; the 2D grid's XY-only bucketing returns a superset of
  real neighbours in Z, which the `r*r` filter discards. Correct, just
  wider-than-necessary candidate set.

## Verification

1. `cargo test -p bevy_mass` — plugin unit tests green.
2. `cargo test -p vivarium-sim` — updated bird tests green; schedule smoke
   still builds.
3. `cargo test -p vivarium-sim --features unreal` — UE-bridge registrations
   + schedule smoke green.
4. `cargo run -p vivarium-standalone` — visual smoke: birds still flocking.
5. `cargo build --release -p vivarium-rust-host` + C++ plugin rebuild.
6. UE CLI automation: `supplemental.RustPlugin.RustTests.VivariumBirdsFlock
   AndStayBounded` + the new framework-level neighbour-enumeration test both
   EXIT CODE 0.
7. `./scripts/pie.sh vivarium` — PIE visual smoke: birds flock.

## Implementation order

Reviewer-validated slice order, narrowest→widest:

1. **Plugin + Bevy-mode tests.** `SpatialGroupPlugin`, `SpatialGroupRegistry`,
   `SpatialGroupSet`, generic `rebuild_grid_system`. Vivarium unchanged.
   Ships with unit tests proving rebuild runs, ordering works, two plugins
   coexist. No FFI, no C++.
2. **UE enumerate slots — FFI plumbing only.** Add
   `MassSpatialNeighbor`, `MassSpatialEnumerateFn`,
   `MassSpatialEnumerateSlot`, the new `MassFrameDispatchData` fields, and
   the `MassSpatialQueries::enumerate` resource path. C++ side: an empty
   slot array (enumerate_fn pointers exist but return 0 hits). No behaviour
   change anywhere.
3. **C++ enumerate callback + `GridHashEnumerate` config type.** C++
   recognises the new `query_type`, registers a callback, implements
   `ExecuteGridHashEnumerate`. At this point UE-mode
   `SpatialQueries::neighbors_within` has two backends: the old Rust-side
   mirror (still wired) and the new FFI path (wired for registered
   enumerate groups). Test with a synthetic group before touching vivarium.
4. **Vivarium migration.** Swap vivarium to `SpatialGroupPlugin`, delete
   `rebuild_bird_grid_system`, drop the manual
   `MassSpatialQueryConfigRegistration` submit, verify
   `VivariumBirdsFlockAndStayBounded` still passes end-to-end.

Each slice is independently mergeable.

## Open questions to resolve during planning

- **Exact mechanism for passing `name + radius` from plugin constructor to
  rebuild system.** Either a `PerGroupMeta<(M, P)>` resource (type-pair
  keyed) or a closure capture. Type-pair keyed is one more resource per
  group but avoids indirect capture. Plan picks one after a small
  prototype.
- **Exact seam inside `mass_init_simulation`** where plugin builds run and
  the registry snapshot is taken. The design commits to `mass_init_simulation`
  (MassSchedule is created earlier during dylib load, but plugins are re-run
  here so re-init/hot-reload refreshes registry state). Plan phase reads
  `unreal-module/src/lib.rs` and picks the exact line.
- **Cap size for enumerate returns.** The FFI contract says "return
  uncapped count, caller retries with bigger buffer." Initial
  `SmallVec<[_; 64]>` stack buffer, grow to `Vec` on first truncation.
  Plan phase verifies 64 is a reasonable default for vivarium's flocking
  neighbour counts (radius 40, ~20 birds, expected <10 neighbours per
  call).
- **`TransformLike` import source.** Reuse the existing trait from
  `bevy_mass::movement`. Resolved — no new trait needed.
- **`MassFragment::CPP_TYPE_NAME`.** Confirmed already emitted by
  `#[derive(MassFragment)]`. Resolved.
