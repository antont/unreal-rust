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

4. **Bevy schedule is built** from registered systems, ordered by the `order` attribute on `#[mass_system(order = N)]`.

### Every frame

5. **C++ collects all chunk data** into `MassFrameDispatchData` and calls `mass_frame_dispatch()`.

6. **Rust updates chunk resources** in the Bevy World (`MassSystemChunks<Marker, T>` per system per fragment type) and runs the Bevy schedule.

7. **User systems run** with direct memory access to UE chunk memory, wrapped in Bevy-compatible iterators.

### Example system (dual-mode)

```rust
#[mass_system(order = 50)]
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

### Example with QueryAll (index-based global access)

```rust
#[mass_system(order = 45)]
fn carried_food_tracking(
    ants: Query<(&Transform, &Carrying), With<Ant>>,
    food_transforms: QueryAll<&mut Transform, With<Food>>,
) {
    for (transform, carry) in &mut ants {
        if carry.food_index >= 0 {
            if let Some(food_tf) = food_transforms.get_mut(carry.food_index as usize) {
                food_tf.translation = transform.translation + DVec3::new(0.0, 0.0, 15.0);
            }
        }
    }
}
```

`QueryAll` provides `get_mut(index)` across all entities of a type. In UE mode: zero-copy chunk access. In Bevy mode: `EntityIndex<Tag>` + Query lookup (macro-rewritten).

## Query types

### Facade types (portable — use these in game code)

| Type | Bevy backing | UE backing | Use case |
|---|---|---|---|
| `Query<&T>` | `bevy_ecs::Query` | Chunk `&[T]` slice | Read-only per-entity iteration |
| `Query<&mut T>` | `bevy_ecs::Query` | Chunk `&mut [T]` slice | Read-write per-entity iteration |
| `Query<(Entity, &mut T, &U), (With<Tag>, Without<V>)>` | `bevy_ecs::Query` | Tuple facade struct + entity map | Multi-component with filters |
| `QueryAll<&mut T, With<Tag>>` | `EntityIndex<Tag>` + `Query` via `QueryAllWrapper` | `MassQueryAllMut` (zero-copy chunks) | Index-based global access (`get_mut(i)`) |
| `BevyQuery<D, F>` | `bevy_ecs::Query` (always) | `bevy_ecs::Query` (always) | Pure-Bevy components (shadow entities) |

### UE-only types (use only in UE-specific code)

| Type | Backing | Use case |
|---|---|---|
| `SpatialQuery` | Wraps `MassSpatialQueries` (C++ physics sweep results) | Collision detection via `Res<SpatialQuery>` — returns `SpatialHit` with `DVec3` |

Facade `Query` supports tuples with `Entity`, `With<Tag>`/`Without<T>` filters, and multiple mutable fragments. The `#[mass_system]` macro handles all backend-specific rewrites.

`QueryAll` provides `get(index)` and `get_mut(index)` for O(1) indexed access across all entities of a type. The `#[mass_system]` macro rewrites it to `EntityIndex` + `QueryAllWrapper` in Bevy mode and `MassQueryAllMut` in UE mode.

## Fragment definition

### Game components — `#[component]` attribute

Game-authored types use the `#[component]` attribute macro, which adds `#[repr(C)]`, `#[derive(Component, Clone, Copy, Debug)]`, and conditionally `#[derive(MassFragment)]` in UE mode. C++ USTRUCT names are auto-derived from `BEVY_MASS_CPP_PREFIX` (set in the game crate's `build.rs`) + struct name + UE-conventional suffix (`Fragment` for data, `Tag` for unit structs):

```rust
#[component]
pub struct FoodState {
    pub is_loose: bool,
}
// → C++ name: FGatherersFoodStateFragment (auto-derived)

#[component]
pub struct Food;
// → C++ name: FGatherersFoodTag (auto-derived)

#[component(group = "ants")]
pub struct Ant;
// → C++ name: FGatherersAntTag (auto-derived, "group" sets entity archetype)
```

No `cpp_type`, no UE jargon — game code looks like standard Bevy.

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

- **Fragment definition**: `#[component]` generates C++ USTRUCT headers with auto-derived names
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
| `unreal-api-derive/src/mass_fragment.rs` | `#[derive(MassFragment)]` proc macro + C++ header codegen |
| `unreal-api-derive/src/component_attr.rs` | `#[component]` attribute macro: auto-derives C++ names with Fragment/Tag suffixes |
| `bevy_mass/src/` | Facade crate: `Query`, `QueryAll`, `MovementPlugin`, `SpatialQuery`, `EntityIndex`, `Time`, engine types (Transform, Velocity, DesiredMovement) |
| `unreal-module/src/mass_system_registry.rs` | System discovery FFI, Bevy schedule management |
| `gatherers-sim/src/` | Game simulation logic (movement, food decisions, component definitions) |
| `gatherers-bevy-mass/src/` | Game systems (mostly portable) + spatial query integration + UE tests |
| `gatherers-standalone/src/` | Standalone Bevy app running the same simulation without Unreal |
| `RustPlugin/Source/RustPlugin/RustMassDynamicProcessor.cpp` | C++ processor: caching, dispatch |
| `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` | C++ subsystem: entity management, fragment read/write |
| `RustPlugin/Source/RustPlugin/Bindings.h` | C++ FFI struct definitions |
