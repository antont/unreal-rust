# Vivarium porting roadmap

A staged plan for porting the [vivarium](https://github.com/tonialatalo/vivarium) Bevy
simulation onto unreal-rust. Phases follow vivarium's own construction order
(as seen in its git history) so each step lands a runnable milestone.

For each phase: **what vivarium needs**, **what unreal-rust has today**, **what
is missing**, and **porting strategy** — including deliberate divergences where
UE-native mechanics beat a 1:1 port.

Related docs:
- [bevy-mass-architecture-options.md](bevy-mass-architecture-options.md) — facade rationale
- [parallelism-and-component-design.md](parallelism-and-component-design.md) — split-writer pattern
- [todo/idiomatic-bevy-authoring.md](todo/idiomatic-bevy-authoring.md) — authoring gaps

## Guiding principles

1. **Port simulation logic; reimplement presentation; delegate engine-strong
   mechanics.** State machines and steering travel as Rust code. Rendering,
   hierarchical visuals, surface walking, camera, UI — reimplement using
   UE-native tech. Spatial queries are a third category: the *call site* stays
   in Rust, but the *backend* is engine-specific — UE mode uses
   `SpatialQueries` (wrapping `MassSpatialQueries`), standalone mode uses
   direct Bevy queries, just like gatherers does today. Don't hand-roll a
   HashMap spatial grid in the portable layer.
2. **Each phase is standalone-runnable.** Every phase should also compile under
   pure Bevy (facade mode) so we can iterate without UE round-trips.
3. **Drive unreal-rust features from concrete need.** Don't generalise ahead of
   a phase that exercises the feature.

---

## Phase 1 — Insects (brownian wanderers, boundary-bounded)

**Vivarium scope:** ~200 insects, `Insect` tag + `Velocity` + `BrownianMotion` +
`BoundaryWrap`, a spatial index, brownian motion system, movement integrator,
boundary force field.

**unreal-rust status:** ✅ Mostly ready. Matches what the current
`example/UnrealRustExample` exercises.

**Gaps — none blocking.**

**Strategy:** Direct port. One chunk-backed fragment per component.

- **Spatial index:** don't port vivarium's `HashMap<(i32,i32,i32), Vec<Entity>>`.
  Use the `SpatialQueries` facade from `bevy_mass` — same shape as gatherers:
  UE mode dispatches to `MassSpatialQueries` (engine-optimised), standalone
  mode falls back to direct Bevy queries. Insect neighbour lookups
  (swarm cohesion) register as a named query and get called per-frame.
- Good baseline for measuring serial iteration cost at 200–2000 entities.

---

## Phase 2 — Birds (flocking, hunt FSM, predator-prey)

**Vivarium scope:** ~20 birds with `Flocking`, `Predator`, `HuntState`
(3-phase FSM: Searching/Circling/Diving), `Wander`. Eating system despawns
insects and writes `InsectEaten` messages.

**unreal-rust status:** Partial.

**Gaps:**
- **Enum components.** `HuntPhase` is an enum. Chunk-backed fragments require
  `repr(C) + Copy` POD — enums work only as `repr(u8)` C-style. `HuntState`
  bundles an enum + `f32` + `Vec3`; fine as `repr(C)` if we encode the enum.
  Alternative: keep the FSM state as a **shadow Bevy component** (outside
  chunks), which removes the POD constraint.
- **Despawn from a system.** `eating_system` despawns insects mid-frame.
  unreal-rust does not yet support spawn/despawn in chunk-iterated systems —
  entity-group spawning is explicitly deferred
  ([todo/idiomatic-bevy-authoring.md §17](todo/idiomatic-bevy-authoring.md)).
  Needs: deferred despawn queue drained between schedules.
- **Split-writer pattern for parallelism.** `movement_system` in vivarium is
  one system over both insects and birds via `Option<&Insect>/Option<&Bird>`.
  For UE-Mass parallelism, follow
  [parallelism-and-component-design.md](parallelism-and-component-design.md) —
  split into per-type velocity components or per-type integrators.

**Strategy:**
- Shadow-side: `HuntState`, `Flocking` weights (rarely written, big struct).
- Chunk-side: `Velocity`, `Transform`, `Predator` sight params, tag components.
- Implement despawn queue as the first "real" Commands operation.
- Messages: `InsectEaten` is trivially Clone — supported today.
- **Spatial queries:** bird hunt detection (sight cone against insects) and
  boids (nearby birds) register named `SpatialQueries` — cone sweeps for hunt,
  radius sweeps for flocking. UE runs these via `MassSpatialQueries`; the
  standalone fallback does direct Bevy iteration.

---

## Phase 3 — Static trees (L-system geometry, ground plane)

**Vivarium scope:** Three procedural trees, each a parent-child hierarchy of
cylinder segments; ground plane.

**unreal-rust status:** Hierarchy not supported.

**Decision: don't port the hierarchy.** Trees are static scene dressing.

**Strategy (UE-native):**
- Author trees as UE `AActor`s (Blueprint or static mesh actors). One actor per
  tree. No entities on the Rust side.
- Navigation nodes (for squirrel pathfinding later) live on the actor as named
  sockets or a `UDataAsset`, published to Rust as a `NavGraph` resource.
- Ground plane: UE static mesh.

**Standalone-mode substitute:** Keep vivarium's L-system tree code under a
`#[cfg(feature = "standalone")]` for the Bevy runner. UE mode never runs it.

---

## Phase 4 — Wind (scene-wide field)

**Vivarium scope:** `Wind` resource oscillates direction/strength. Affects
insect/bird movement (small drift) and **tree bending via hierarchy**.

**unreal-rust status:** Resource support ready. Wind-on-agents is a one-liner
in the movement integrator.

**Gaps:** Tree bending requires hierarchy — **skip on the Rust side**.

**Strategy:**
- Port `Wind` as a `Resource` read by movement integrators. Trivial.
- Tree bending: UE vertex shader or UE animation on the tree actors, driven by
  an exposed `FVector` cvar/param written from Rust each frame. No Rust ECS
  involvement in tree deformation.

---

## Phase 5 — Squirrels (pathfinding, surface walking)

**Vivarium scope:** 3 squirrels with `SquirrelState` (FSM +
`Vec<usize>` path + progress + last surface normal). Moves along nav graph,
**projects onto tree surface each frame** (cylinder math).

**unreal-rust status:** Big gap.

**Gaps:**
- `Vec<usize>` in a component — not POD. Must be shadow-side.
- Multi-part visual hierarchy (body/head/ears/tail/feet) — not portable.
- Surface projection is non-trivial and mostly a visual concern.

**Decision: split logic from presentation.**

**Strategy:**
- **Logic side (Rust ECS):** `SquirrelState` FSM, pathfinding,
  target selection lives as shadow components. Emits a desired world position
  + facing per frame.
- **Presentation side (UE):** A squirrel `AActor` with UE skeletal mesh (or
  multi-part static mesh assembly). Surface walking via UE's existing
  navmesh/surface-projection capabilities — UE has `SplineMeshComponent`,
  `NavLinkProxy`, and surface-follow movement components that outperform
  hand-rolled cylinder math.
- The Rust ECS writes the actor's target transform; UE handles interpolation,
  surface conforming, and anim blending.

This is the cleanest case for the logic/presentation split — lean into it.

---

## Phase 6 — Bird nesting & predation (lifecycle FSM, dynamic spawning)

**Vivarium scope:** `BirdNestingState` (7-state FSM + `Option<Entity>` nest ref
+ timer + counter), `Nest`, `Hatchling` components. Birds spawn nests, spawn
hatchlings, lay eggs. Squirrels detect hatchlings, insert `SquirrelTarget`
component mid-frame. `hatchling_alert_system` triggers Defending mode.

**unreal-rust status:** Significant gaps.

**Gaps:**
- **Spawn from a system.** Birds spawn `Nest` and `Hatchling` entities at
  runtime. Requires a deferred-spawn queue analogous to the Phase 2 despawn
  queue. This is the big feature work for this phase.
- **Dynamic component insertion** (`SquirrelTarget` added mid-frame). unreal-rust
  supports `Commands::insert`/`remove` — should work via shadow components.
- **`Option<Entity>` references between entities.** Entity refs in messages
  already landed — [todo/done/entity-references-in-messages.md](todo/done/entity-references-in-messages.md).
  Storing them in chunk fragments needs care (cross-archetype lookup via
  `MassEntityMap`). Simpler path: nest refs as shadow-side components.

**Strategy:**
- Build the spawn queue. Make it the headline feature of this phase.
- Keep all lifecycle FSM state shadow-side — lifecycle transitions are rare
  enough that chunk-backed perf doesn't matter.
- `Nest` and `Hatchling` as first-class entities with shadow components; UE
  spawns actors to match if they need to be visible.

---

## Phase 7 — UI, camera, input

**Vivarium scope:** Status text overlay, wind indicator, orbit camera with
WASD, squirrel-focus modes, mouse orbit/zoom.

**Decision: native UE, not Rust ECS.**

**Strategy:**
- UE UMG widgets for status and wind indicator. Rust exposes counts/state via
  a small read-only API (one C ABI struct per frame).
- UE `APlayerController` + spring-arm camera for orbit/follow. Rust is
  uninvolved.
- Keep Bevy UI versions only for standalone mode.

---

## Phase 8 — Performance pass

**Vivarium scope:** `par_iter_mut`, allocation-free spatial queries, shared
mesh/material handles, `SIM_SCALE` env var.

**unreal-rust status:** Single-threaded scheduler enforced. Under UE-Mass,
archetype parallelism is the win — see
[parallelism-and-component-design.md](parallelism-and-component-design.md).

**Strategy:**
- Measure at 10× scale before parallelising.
- If Rust-side parallelism needed: UE-Mass already parallelises across
  archetypes; the split-writer pattern already in our design notes is the
  lever.
- `SIM_SCALE` maps to a UE cvar that controls spawn counts.

---

## Feature work implied by this roadmap, in order

Extracted from the phases above so it can be tracked independently:

1. **Deferred despawn queue** (Phase 2) — enables eating.
2. **Deferred spawn queue** (Phase 6) — enables nesting lifecycle.
3. **Shadow-component ergonomics** (Phase 2, 5, 6) — clearer docs +
   examples for when to pick shadow vs chunk.
4. **Split-writer authoring examples** (Phase 2, 8) — makes parallelism
   reachable without re-reading the design note each time.
5. **`Or<With<A>, With<B>>` filter codegen** (Phase 1, only if we unify insect +
   bird spatial rebuild) — parser exists, codegen doesn't. Low priority, has
   easy workaround (two queries).

Items deliberately *not* on this list because UE handles them:
- Parent/child hierarchy
- Surface walking / navmesh following
- Tree wind bending
- Skeletal visuals and multi-part actors
- Camera, input, UI

## Open questions

- Standalone (pure Bevy) target: do we keep vivarium-as-vivarium runnable at
  each phase, or only verify the unreal-rust port runs under Bevy via the
  facade? The facade path is cheaper; vivarium-as-vivarium is higher signal.
- Nav graph authoring: UE data asset driven by tree actor sockets, or
  hand-coded and shared with standalone mode? Leaning actor-driven; standalone
  can fall back to a fixture.
- Entity refs in chunk fragments vs shadow: draw the line now or defer until
  Phase 6 forces the choice.
