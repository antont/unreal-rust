# MassEntity Bridge Design Notes

## Background

UE's MassEntity is a data-oriented ECS framework with strong parallels to Bevy ECS:

| MassEntity (UE) | Bevy ECS |
|---|---|
| `FMassFragment` | `Component` |
| `FMassTag` | Zero-sized `Component` |
| `FMassProcessor::Execute()` | `System` |
| `EntityQuery.ForEachEntityChunk` | `Query<&mut T>` iteration |
| Archetype chunks | Archetype tables |
| `FMassEntityHandle` | `Entity` |
| `FMassExecutionContext` | `SystemParam` (delta time, subsystem access) |

The [gatherers](https://github.com/antont/gatherers) project uses MassEntity in C++ for an ant colony simulation. This document explores how unreal-rust could bridge Bevy ECS to MassEntity.

## Current state

unreal-rust already has **generated MassEntity bindings** in `unreal-api/src/bindings/mass_entity.rs` (600+ lines), but these are bare UClass/UProperty reflection wrappers with no ECS integration layer.

The current Rust gameplay flow is:
```
UE C++ tick
  → ARustGameModeBase::Tick(dt)
    → RustBindings.tick(dt)
      → UserModule::tick(dt)
        → manual UE API calls (get_all_actors_of_class, set_actor_location, etc.)
```

The Bevy `App` exists in the example code but isn't used for scheduling — `tick()` calls UE APIs directly.

## Approach: Rust MassEntity Processors

The most practical approach is to let Rust code implement MassEntity processor logic while MassEntity owns the entity data. This keeps MassEntity's UE integration (LOD, replication, visualization) intact.

### C++ side

A generic `URustMassProcessor` subclass delegates its `Execute()` to Rust via FFI:

```cpp
void URustMassProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
    // Pass chunk data to Rust as typed slices
    EntityQuery.ForEachEntityChunk(Context, [](FMassExecutionContext& ChunkContext)
    {
        // Get fragment views as raw pointers + count
        // Call into Rust with pointer, length, delta_time
    });
}
```

### Rust side

Processor logic receives fragment data as slices — the same pattern as Bevy query iteration:

```rust
// What gatherers does in C++:
//   ChunkContext.GetMutableFragmentView<FGatherersMassAntFragment>()
//   for EntityIndex in 0..ChunkContext.GetNumEntities()

// Equivalent Rust with the bridge:
fn ant_movement(ants: &mut [AntFragment], dt: f32, bounds: &SimulationBounds) {
    for ant in ants.iter_mut() {
        ant.previous_position = ant.position;
        ant.position = compute_heading_step(
            ant.position, ant.direction, ant.movement_speed, dt
        );
        clamp_to_bounds(ant, bounds);
    }
}
```

### Minimal FFI surface needed

Based on the gatherers MassEntity patterns, the bridge needs:

| Operation | MassEntity API | Rust equivalent |
|---|---|---|
| Chunk iteration | `ForEachEntityChunk(Context, lambda)` | Callback with slice pointers |
| Fragment access | `GetMutableFragmentView<T>()` | `&mut [T]` slice |
| Read-only access | `GetFragmentView<T>()` | `&[T]` slice |
| Entity count | `GetNumEntities()` | Slice length |
| Delta time | `Context.GetDeltaTimeSeconds()` | `dt: f32` parameter |
| Subsystem access | `Context.GetSubsystemChecked<T>()` | Passed as parameter |
| Entity lifecycle | `EntityManager.CreateEntity()` / `DestroyEntity()` | FFI callbacks |
| Single entity | `FMassEntityView::GetFragmentData<T>()` | Lookup by handle |

### Fragment type mapping

Fragments are `#[repr(C)]` Rust structs mirroring C++ `FMassFragment` subclasses. The existing `#[derive(UClass)]` proc macro could be extended or a new `#[derive(MassFragment)]` macro could generate the FFI glue:

```rust
#[derive(MassFragment)]
#[repr(C)]
pub struct AntFragment {
    pub position: FVector,
    pub direction: FVector,
    pub previous_position: FVector,
    pub movement_speed: f32,
    pub pickup_cooldown: f32,
    pub carried_food: FMassEntityHandle,
}
```

### Query declaration

MassEntity processors declare their queries upfront in `ConfigureQueries()`. The Rust bridge needs a way to express requirements:

```rust
#[mass_processor]
fn ant_movement(
    ants: &mut MassQuery<AntFragment>,       // ReadWrite
    foods: &MassQuery<FoodFragment>,          // ReadOnly
    context: &MassContext,                     // Delta time, subsystems
) {
    for chunk in ants.chunks() {
        for ant in chunk.iter_mut() {
            // ...
        }
    }
}
```

This maps to the C++ `ConfigureQueries` pattern:
```cpp
EntityQuery.AddRequirement<FAntFragment>(EMassFragmentAccess::ReadWrite);
EntityQuery.AddTagRequirement<FAntTag>(EMassFragmentPresence::All);
```

## Alternative: Bevy-only ECS

Use Bevy ECS as the sole data store, bypassing MassEntity entirely. Sync only positions/transforms to UE for rendering.

**Pros:** Simpler, no FFI per chunk iteration, full Rust type safety.
**Cons:** Loses MassEntity UE integration (LOD, replication, visualization pipeline, Mass AI).

## Key challenges

1. **Type erasure boundary** — MassEntity fragments are type-erased at runtime. The bridge must ensure Rust struct layouts match C++ exactly (the existing layout verification test pattern helps here).

2. **Chunk lifetime** — Fragment slice pointers are only valid during `ForEachEntityChunk`. The Rust callback must not store them.

3. **Processing phases** — MassEntity has a processor scheduling pipeline. Rust processors need to participate in the correct phase ordering.

4. **Entity handle stability** — `FMassEntityHandle` includes a serial number for validity checking. The Rust side needs to respect this.

## Implementation priority

1. Add `FMassFragment` FFI types and layout tests (extend existing codegen)
2. Create `URustMassProcessor` C++ base class that delegates to Rust
3. Expose `ForEachEntityChunk` → Rust slice callback
4. Add `#[derive(MassFragment)]` proc macro for type-safe fragment definition
5. Integrate with processor scheduling pipeline
