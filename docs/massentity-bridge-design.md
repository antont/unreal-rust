# MassEntity Bridge Design

## Overview

unreal-rust bridges Rust simulation code to UE's MassEntity ECS. Game developers write standard Bevy-style systems in Rust — the framework handles compilation against either pure Bevy (for testing/standalone) or Unreal Mass Entity (for production). In Unreal mode, MassEntity owns the entity data and chunk storage; Rust systems operate on that memory directly through cached pointers with zero per-frame copy overhead.

## Dual-mode architecture

Systems are written using standard Bevy syntax (`Query<&mut T>`, `Res<Time>`, `QueryAll<&mut T, With<Tag>>`). A `bevy_mass` facade crate and the `#[mass_system]` proc macro handle the compile-time switch:

- **Bevy mode** (default): systems run in a real Bevy `World` with `bevy_ecs`. `Query` is a real Bevy query; `QueryAll` is rewritten by the macro to `EntityIndex<Tag>` + `Query` with a `QueryAllWrapper`. Used for unit tests and standalone Bevy apps.
- **Unreal mode** (`--features unreal`): the `#[mass_system]` macro rewrites `Query` to chunk iteration, `QueryAll` to `MassQueryAllMut`, and generates an `extern "C"` wrapper + Bevy resource-based dispatch.

See `docs/bevy-mass-architecture-options.md` for the design rationale.

```
UE MassEntity (owns entity data in archetype chunks)
  |
  |  cached raw pointers (built once on first frame)
  v
C++ URustMassDynamicProcessor::Execute()
  |
  |  MassFrameDispatchData (all chunks + dt)
  v
Rust mass_frame_dispatch()
  |
  |  updates MassSystemChunks<Marker, T> resources in Bevy World
  |  runs Bevy Schedule (systems execute in order)
  v
User's Rust system functions (standard Bevy syntax)
```

## Execution flow

### Startup (once)

1. **Rust systems register** via `inventory` at load time. Each `#[mass_system]` function produces both a C++ registration (name, `extern "C"` wrapper, fragment requirements) and a Bevy system registration.

2. **C++ discovers systems** by calling `get_mass_system_count()` / `get_mass_system_descriptor()`. Creates one `URustMassDynamicProcessor` per system.

3. **C++ configures queries** from the requirement list. Each requirement specifies the C++ fragment type name, access mode (read/write), whether it's a tag, and query scope (primary or global).

4. **Bevy schedule is built** from registered systems. Execution order comes from a plugin-level `MassScheduleOrder` inventory submission that lists system names in order; the framework resolves that to numeric order values (stride 10) that both the Bevy schedule and the C++ processor pipeline respect. Explicit `order = N` on `#[mass_system]` is still honored for legacy cases.

### Every frame

5. **C++ collects all chunk data** into `MassFrameDispatchData` and calls `mass_frame_dispatch()`.

6. **Rust updates chunk resources** in the Bevy World (`MassSystemChunks<Marker, T>` per system per fragment type) and runs the Bevy schedule.

7. **User systems run** with direct memory access to UE chunk memory, wrapped in Bevy-compatible iterators.

### Example system (dual-mode)

```rust
#[mass_system]
pub fn entity_boundary_reflect(
    transforms: Query<&Transform>,
    mut movements: Query<&mut DesiredMovement>,
) {
    for (transform, mut movement) in transforms.iter().zip(movements.iter_mut()) {
        let pos = transform.translation;
        // Reflect velocity at simulation boundaries
        let inward_normal = compute_boundary_normal(pos);
        if inward_normal.length() > 1e-8 {
            movement.velocity = reflect_velocity(movement.velocity, inward_normal);
        }
    }
}
```

`#[mass_system]` works unconditionally — in Bevy mode it passes through the original function; in Unreal mode it rewrites to chunk-based iteration. No `#[cfg_attr]` needed.

### Example: UE-bridge system producing hit events

Game logic in `gatherers-sim` reads plain `MessageReader<AntFoodHit>` — no UE
types visible. The UE bridge crate (`gatherers-bevy-mass`) holds the one
system that touches UE-specific resources and produces those messages:

```rust
#[mass_system]
fn ant_collision_prepass(
    ants: Query<(Entity, &Transform, &PreviousTranslation), (With<Ant>, Without<Cooldown>)>,
    spatial: Res<SpatialQuery>,
    entity_map: Res<MassEntityMap>,
    mut hits: MessageWriter<AntFoodHit>,
) {
    let spatial = spatial.with_map(&entity_map);
    for (entity, transform, prev) in &mut ants {
        if let Some(hit) = spatial.call("food_pickup", &prev.value, &transform.translation) {
            hits.write(AntFoodHit::new(hit.entity_index, hit.entity, entity, hit.position));
        }
    }
}
```

`SpatialQuery::with_map(&MassEntityMap)` yields a short-lived helper whose
`call()` returns `SpatialHit { entity_index, entity, position }` — the
named query's `query_group` is looked up via inventory once, so game code
never mentions the group string (`"food"`) or calls `entity_map.get(…)`.
Downstream consumers (in `gatherers-sim`) see `AntFoodHit.hittable_entity`
and use standard `Query::get(entity)` lookups — vanilla Bevy.

When a chunk-backed FFI payload stores an `i32` index (e.g. the
`FoodPickupEvents`/`FoodDropEvents` queues consumed by C++), the
bridging system that drains those queues builds a one-shot
`HashMap<Entity, usize>` from `Res<EntityIndex<Food>>` to map
message entities back to indices. In-sim component state —
including handles like `Carrying(Option<Entity>)` on shadow
entities — uses `Entity` directly; the index only reappears at
the FFI boundary.

## Query types

### Facade types (portable — use these in game code)

| Type | Bevy backing | UE backing | Use case |
|---|---|---|---|
| `Query<&T>` | `bevy_ecs::Query` | Chunk `&[T]` slice | Read-only per-entity iteration |
| `Query<&mut T>` | `bevy_ecs::Query` | Chunk `&mut [T]` slice | Read-write per-entity iteration |
| `Query<(Entity, &mut T, &U), (With<Tag>, Without<V>)>` | `bevy_ecs::Query` | Tuple facade struct + entity map | Multi-component with filters |
| `QueryAll<&mut T, With<Tag>>` | `EntityIndex<Tag>` + `Query` via `QueryAllWrapper` | `MassQueryAllMut` (zero-copy chunks) | Index-based global access (`get_mut(i)`). Use when you have an `i32` chunk-slot index (e.g. from a `#[repr(C)]` message payload); for Entity-keyed access on chunk-backed types, prefer `Query<&mut T>` with `entity_map.get(...)` |

### UE-only types (use only in UE-bridge code)

| Type | Backing | Use case |
|---|---|---|
| `SpatialQuery` | Wraps `MassSpatialQueries` (C++ ISMC overlap, physics sweep, or MassNavigation grid hash — backend selected per query in `MassSpatialQueryConfigRegistration`) | Collision detection via `Res<SpatialQuery>`. Call `.with_map(&entity_map).call(name, prev, curr)` → `Option<SpatialHit { entity_index, entity, position }>`; the query's target group is resolved via inventory so game code never mentions the group string |
| `MassEntityMap` | C++ maintains per-group `Vec<Entity>` of shadow Bevy entities | Used implicitly by `SpatialQuery::with_map`. Systems only take `Res<MassEntityMap>` when they need to resolve group-indexed lookups; most game code reaches entities via `EntityIndex<Tag>` or shadow-component queries instead |

Facade `Query` supports tuples with `Entity`, `With<Tag>`/`Without<T>` filters, and multiple mutable fragments. The `#[mass_system]` macro handles all backend-specific rewrites. Components that don't implement `MassFragment` (pure-Bevy components on shadow entities) are auto-detected via `QueryBackend::IS_CHUNK` and dispatched to Bevy entity storage — no annotation needed.

`QueryAll` provides `get(index)` and `get_mut(index)` for O(1) indexed access across all entities of a type. The `#[mass_system]` macro rewrites it to `EntityIndex` + `QueryAllWrapper` in Bevy mode and `MassQueryAllMut` in UE mode.

## Fragment definition

### Game components — `#[derive(Component, MassFragment)]`

Game-authored types use vanilla Bevy `#[derive(Component)]` alongside an
explicit `MassFragment` opt-in. `#[repr(C)]` is required on data fragments
(matching the C++ USTRUCT layout) and omitted on tags (unit structs).
C++ USTRUCT names are auto-derived from `BEVY_MASS_CPP_PREFIX` (set in
the game crate's `build.rs`) + struct name + UE-conventional suffix
(`Fragment` for data, `Tag` for unit structs):

```rust
#[repr(C)]
#[derive(Component, MassFragment, Clone, Copy, Debug)]
pub struct FoodState {
    pub is_loose: bool,
}
// → C++ name: FGatherersFoodStateFragment (auto-derived)

#[derive(Component, MassFragment, Clone, Copy, Debug)]
pub struct Food;
// → C++ name: FGatherersFoodTag (auto-derived)

#[derive(Component, MassFragment, Clone, Copy, Debug)]
#[cfg_attr(feature = "unreal", mass(group = "ants"))]
pub struct Ant;
// → C++ name: FGatherersAntTag (auto-derived, "group" sets entity archetype)
```

No `cpp_type`, no UE jargon — game code reads as vanilla Bevy except for
the `#[repr(C)]` line on chunk-backed data fragments. The `MassFragment`
derive emits nothing in pure-Bevy builds (all chunk-registration tokens
are `#[cfg(feature = "unreal")]`-gated).

> The older `#[component]` attribute macro is deprecated; see
> `docs/todo/component-derive-and-attribute-cleanup.md` for the removal
> plan. Behavior is identical — the derive form just uses Bevy's real
> `Component` derive instead of shadowing it.

### Engine types — `mass_fragment!` macro

Types that map to existing C++ USTRUCTs (e.g., `FTransformFragment`) use `mass_fragment!` with the `existing` flag. These live in the `bevy_mass` framework crate, not in game code:

```rust
mass_fragment!(cpp_type = "FTransformFragment", existing, include = "MassCommonFragments.h",
    pub struct Transform { /* matching layout */ }
);
```

Codegen skips USTRUCT generation for `existing` types and emits only `static_assert(sizeof(...))` for layout verification.

`DVec3` (glam) maps to `FVector` in generated C++ headers. Layout is verified at compile time with `static_assert` on the C++ side and `offset_of!` tests on the Rust side.

## Zero-copy design

The key property enabling zero-copy is **stable entities**: ant and food entities are spawned once at initialization and never created or destroyed during the simulation. This means:

- Chunk memory addresses don't change between frames
- Fragment pointers cached on first `Execute()` remain valid
- No per-frame `ForEachEntityChunk`, no `GetFragmentView`, no `TArray` allocations
- Global queries use `MassGlobalFragmentChunks` descriptors pointing directly into chunk memory

If entities were dynamic (added/removed at runtime), the cache would need invalidation. The `bChunkCacheValid` flag and `ConfigureQueries` invalidation path handle re-initialization (e.g. between Play sessions in editor).

## Zero-C++ game authoring

Game developers write only Rust. The infrastructure handles:

- **Fragment definition**: `#[derive(Component, MassFragment)]` generates C++ USTRUCT headers with auto-derived names
- **Entity spawning**: `EntityArchetype::new("food").fragment::<FoodState>().spawn(count, |i, writer| { ... })`
- **System registration**: `#[mass_system]` registers with both C++ and Bevy
- **Sim defaults**: `inventory::submit!(MassSimDefaults { ... })` configures entity counts, bounds, etc.
- **Spatial queries**: `inventory::submit!(MassSpatialQueryConfig { ... })` registers collision queries
- **Visualization**: `inventory::submit!(MassVisualizerGroupConfig { ... })` configures UE instanced meshes
- **Testing**: `inventory::submit!(MassTestRegistration { ... })` registers Rust-authored UE tests
- **Hot reload**: editing Rust + `cargo build --release` reloads in a running PIE session

## File map

| File | Role |
|---|---|
| `unreal-ffi/src/lib.rs` | FFI types: `MassChunkData`, `MassFragmentSlice`, `MassGlobalChunkSlice` |
| `unreal-api/src/mass.rs` | Rust query types, TestCtx, schedule, system registration |
| `unreal-api-derive/src/mass_system.rs` | `#[mass_system]` proc macro: generates wrapper + Bevy system + registration |
| `unreal-api-derive/src/mass_fragment.rs` | `#[derive(MassFragment)]` proc macro + C++ header codegen, auto-derives C++ names with Fragment/Tag suffixes |
| `unreal-api-derive/src/component_attr.rs` | Deprecated `#[component]` attribute macro (kept for back-compat; to be removed, see `docs/todo/component-derive-and-attribute-cleanup.md`) |
| `bevy_mass/src/` | Facade crate: `Query`, `QueryAll`, `MovementPlugin`, `SpatialQuery`, `EntityIndex`, `Time`, engine types (Transform, Velocity, DesiredMovement) |
| `unreal-module/src/mass_system_registry.rs` | System discovery FFI, Bevy schedule management |
| `gatherers-sim/src/` | Engine-agnostic game logic: component definitions, pure decision fn, movement, message types (`HitEvent`, `FoodMutation`). No UE types. Runs unmodified under standalone Bevy |
| `gatherers-bevy-mass/src/` | UE bridge: the few systems that need UE-specific resources (`Res<SpatialQuery>`, `Res<MassEntityMap>`) to produce hit-event messages, plus UE-mode variants of the decision system. Also hosts UE-only tests |
| `gatherers-standalone/src/` | Standalone Bevy app running the same simulation without Unreal |
| `RustPlugin/Source/RustPlugin/RustMassDynamicProcessor.cpp` | C++ processor: caching, dispatch |
| `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` | C++ subsystem: entity management, fragment read/write |
| `RustPlugin/Source/RustPlugin/Bindings.h` | C++ FFI struct definitions |
