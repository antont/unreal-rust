# MassEntity Bridge Design

## Overview

unreal-rust bridges Rust simulation code to UE's MassEntity ECS. Game developers write standard Bevy-style systems in Rust — the framework handles compilation against either pure Bevy (for testing/standalone) or Unreal Mass Entity (for production). In Unreal mode, MassEntity owns the entity data and chunk storage; Rust systems operate on that memory directly through cached pointers with zero per-frame copy overhead.

## Dual-mode architecture

Systems are written using standard Bevy syntax (`Query<&mut T>`, `Res<DeltaTime>`). A `bevy_mass` facade crate and the `#[mass_system]` proc macro handle the compile-time switch:

- **Bevy mode** (default): systems run in a real Bevy `World` with `bevy_ecs`. Used for unit tests and standalone Bevy apps.
- **Unreal mode** (`--features unreal`): the `#[mass_system]` macro rewrites parameter types to `MassQuery<&mut T>` and generates an `extern "C"` wrapper + Bevy resource-based dispatch.

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
#[cfg_attr(feature = "unreal", mass_system(order = 10))]
pub fn entity_movement(
    mut positions: Query<&mut Position>,
    movements: Query<&Movement>,
    time: Res<DeltaTime>,
) {
    let dt = time.0;
    for (mut pos, mov) in positions.iter_mut().zip(movements.iter()) {
        pos.previous_position = pos.position;
        let dir = mov.direction.normalize();
        pos.position += dir * (mov.movement_speed as f64 * dt as f64);
    }
}
```

In Bevy mode this compiles as a standard Bevy system. In Unreal mode the macro rewrites it to use `MassQuery` types backed by C++ chunk memory.

## Query types

| Rust type | Backing | Use case |
|---|---|---|
| `MassQuery<&T>` | `&[T]` slice into one primary chunk | Read-only per-chunk |
| `MassQuery<&mut T>` | `&mut [T]` slice into one primary chunk | Read-write per-chunk |
| `MassQueryAll<&T>` | Chunked descriptors across all matching chunks | Read-only cross-archetype |
| `MassQueryAll<&mut T>` | Chunked descriptors across all matching chunks | Read-write cross-archetype |

Primary queries (`MassQuery`) iterate entities within a single archetype chunk. The system is called once per chunk.

Global queries (`MassQueryAll`) span all chunks of a given fragment type. They support `iter()`, `iter_mut()`, and `get_mut(index)` for O(1) indexed access across chunk boundaries.

## Fragment definition

Fragments are `#[repr(C)]` structs with `#[derive(MassFragment)]`. The derive macro generates C++ USTRUCT headers automatically — no hand-written C++ needed:

```rust
#[derive(MassFragment, Clone, Copy, Debug, Default)]
#[repr(C)]
#[mass(cpp_type = "FGatherersMassPosition")]
pub struct Position {
    pub position: DVec3,
    pub previous_position: DVec3,
}
```

`DVec3` (glam) maps to `FVector` in the generated C++ header. Layout is verified at compile time with `static_assert` on the C++ side and `offset_of!` tests on the Rust side.

## Zero-copy design

The key property enabling zero-copy is **stable entities**: ant and food entities are spawned once at initialization and never created or destroyed during the simulation. This means:

- Chunk memory addresses don't change between frames
- Fragment pointers cached on first `Execute()` remain valid
- No per-frame `ForEachEntityChunk`, no `GetFragmentView`, no `TArray` allocations
- Global queries use `MassGlobalFragmentChunks` descriptors pointing directly into chunk memory

If entities were dynamic (added/removed at runtime), the cache would need invalidation. The `bChunkCacheValid` flag and `ConfigureQueries` invalidation path handle re-initialization (e.g. between Play sessions in editor).

## Zero-C++ game authoring

Game developers write only Rust. The infrastructure handles:

- **Fragment definition**: `#[derive(MassFragment)]` generates C++ USTRUCT headers
- **Entity spawning**: `EntityArchetype::new("food").fragment::<FoodFragment>().spawn(count, |i, writer| { ... })`
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
| `bevy_mass/src/` | Facade crate: `DeltaTime`, `Query` backend switching |
| `unreal-module/src/mass_system_registry.rs` | System discovery FFI, Bevy schedule management |
| `gatherers-sim/src/` | Game simulation logic (movement, food decisions, fragments) |
| `gatherers-bevy-mass/src/` | Unreal-specific systems (spatial queries, food tracking) + UE tests |
| `RustPlugin/Source/RustPlugin/RustMassDynamicProcessor.cpp` | C++ processor: caching, dispatch |
| `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` | C++ subsystem: entity management, fragment read/write |
| `RustPlugin/Source/RustPlugin/Bindings.h` | C++ FFI struct definitions |
