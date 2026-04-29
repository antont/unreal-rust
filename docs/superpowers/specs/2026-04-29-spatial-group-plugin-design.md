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

### Bevy mode

`Plugin::build`:

1. Inserts `SpatialGrids` resource if absent (idempotent).
2. Defines a framework-internal `SpatialGroupSet` enum with two variants:
   `Rebuild`, `Query`. Configures `Query` to run `.after(Rebuild)`.
3. Registers a monomorphised `rebuild_grid_system::<Marker, Pos>` in
   `SpatialGroupSet::Rebuild` during `Update`. The system:
   ```rust
   fn rebuild_grid_system<M: Component, P: Component + TransformLike>(
       mut grids: ResMut<SpatialGrids>,
       entities: Query<(Entity, &P), With<M>>,
       group: Res<SpatialGroupMeta<M, P>>, // holds name + radius
   ) {
       let grid = grids.grid_mut(group.name, group.radius);
       grid.clear();
       for (e, p) in &entities {
           grid.insert(e, p.translation());
       }
   }
   ```
4. Inserts `SpatialGroupMeta<M, P> { name, radius, _pd: … }` as a typed
   resource so many groups of distinct types coexist.

Sim authors add their flocking/etc. system with
`.in_set(SpatialGroupSet::Query)` and the ordering constraint is automatic.

### UE mode — plugin installation path

Today's UE-mode flow: sim systems reach `MassSchedule`'s Bevy app via
`inventory::submit!(MassBevySystemRegistration)` — unreal-module iterates the
collection and calls each `add_to_app` fn pointer on its MassSchedule. The
sim never holds the App handle itself.

Plugins need a parallel hook. The framework gains one new inventory type
plus a single install_plugins fn per sim crate:

```rust
// bevy_mass (or unreal-api/mass) — new type
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

1. Inserts a framework `SpatialGroupRegistry` resource if absent.
2. Pushes `SpatialGroupEntry { name, radius, marker_tag: "BirdTag",
   position_fragment: "FTransformFragment" }` into the registry.
   - `marker_tag` is the C++-side USTRUCT name of the group's tag fragment
     (discoverable at compile time from `M`'s `MassFragment` metadata, same
     mechanism `MassVisualizerGroupRegistration` uses).
   - `position_fragment` is `P`'s C++ USTRUCT name, used by the C++ side to
     read positions during enumerate. For vivarium both birds and insects
     use `FTransformFragment`.

### UE mode — how C++ sees the registry

C++ calls `get_spatial_query_config_count` / `get_spatial_query_config_desc`
at sim init to discover spatial queries (see `SetupSpatialQueriesFromRust`).
Two sources contribute into that FFI return:

- Legacy: `inventory::iter::<MassSpatialQueryConfigRegistration>()` — gatherers'
  sweep configs, unchanged.
- New: `SpatialGroupRegistry` entries from plugin `build()`.

This means MassSchedule's app must be built *before* the C++ side reads
configs. Today's sequence is: C++ loads dylib → `mass_init_simulation` →
`SetupSpatialQueriesFromRust`. unreal-module builds MassSchedule during
`mass_init_simulation`. The plan phase verifies this ordering and — if it's
wrong — moves MassSchedule construction earlier (e.g. into the first call
after dylib load).

`SpatialGroupRegistry` entries are converted into `MassSpatialQueryConfigDesc`
values on the fly in the `get_spatial_query_config_desc` FFI handler,
synthesising a `query_type: GridHashEnumerate` (new value, alongside the
existing `GridHash` sweep). C++ recognises the new value and installs the
enumerate callback (see below) instead of the sweep callback.

### UE mode — `SpatialQueries::neighbors_within`

Calls the new FFI entry `enumerate_in_radius(name, center, radius, out_buf,
max) -> u32`, resolves each returned `entity_index` via `MassEntityMap`, and
returns the list. Drops the `grids: Option<Res<SpatialGrids>>` field from the
UE-mode `SystemParam`. `SpatialGrids` stays defined but is only used in Bevy
mode.

### New FFI entry

```rust
// in unreal-ffi/src/lib.rs
#[repr(C)]
pub struct MassSpatialNeighbor {
    pub entity_index: i32,
    pub _pad: i32,
    pub position: [f64; 3],
}

pub type MassSpatialEnumerateFn = unsafe extern "C" fn(
    name: Utf8Str,
    center: *const f64,    // [f64; 3]
    radius: f32,
    out: *mut MassSpatialNeighbor,
    max: u32,
) -> u32;  // returns count written (≤ max). If actual hit count > max,
           // returns max; caller may have lost some. C++ logs once per name.
```

Exposed to Rust as an additional per-frame slot alongside the existing sweep
trampoline array, or as a single subsystem-level extern pointer (decided
during implementation). Adds one field on `RustBindings`:
`pub mass_spatial_enumerate: Option<MassSpatialEnumerateFn>`.

### C++ enumerate callback

Mirrors `ExecuteGridHashSpatialQuery` in
`RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp`:

1. Look up the group's ISMC + `EntityToIndex` + `FNavigationObstacleHashGrid2D`.
2. `Grid.QuerySmall(BoundsAroundCenter(radius), Candidates)`.
3. For each candidate: validate entity → reverse-map → read transform →
   `DistSquared <= r*r` filter (no closest-point-on-segment).
4. Write `{entity_index, position}` into `out` until `max` reached.

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
  `MassSpatialEnumerateFn`, `mass_spatial_enumerate: Option<...>` on
  `RustBindings`; update `uninit()` stub.
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

## Open questions to resolve during planning

- **Ordering of MassSchedule build vs `SetupSpatialQueriesFromRust`.** The
  design assumes `MassSchedule` (and therefore all plugin `build()` calls)
  run before C++ reads spatial configs. Plan phase verifies the actual
  dylib-init sequence and, if needed, moves MassSchedule construction
  earlier.
- **Marker-tag and position-fragment name discovery.** The design assumes
  `M` and `P` expose their C++ USTRUCT name via some trait already
  implemented by `#[derive(MassFragment)]`. Plan phase confirms that
  mechanism (and adds it if it's actually missing — e.g. a `MassFragment`
  associated const `CPP_TYPE_NAME`). `MassVisualizerGroupRegistration`
  takes these as string literals today, which suggests the derive does
  not yet emit them automatically.
- **`TransformLike` import source.** Reuse the existing trait from
  `bevy_mass::movement`. No new trait.
