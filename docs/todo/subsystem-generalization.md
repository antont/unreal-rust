# Generalize GatherersBevyMassSubsystem into a Reusable Plugin Subsystem

## Status: DONE

Subsystem lives in `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.{h,cpp}` as `URustMassBevySubsystem` with `TMap<FString, TArray<FMassEntityHandle>> EntityGroups` for dynamic group storage. Game-specific C++ module (`RustMassGatherers`) deleted.

## Problem (original)

`GatherersBevyMassSubsystem` in `RustPlugin/Source/RustMassGatherers/` is currently game-specific C++.
It hardcodes entity group names ("ants", "food") and manages `ManagedAntEntities` / `ManagedFoodEntities`
arrays directly. This means every new game using the Bevy-Mass facade needs its own C++ subsystem.

## Proposed Solution

Move the subsystem into `RustPlugin/Source/RustPlugin/` as a generic `URustMassBevySubsystem` that:

1. **Uses a TMap<FString, TArray<FMassEntityHandle>>** instead of named member arrays.
   Rust init function returns group names + entity handles; C++ stores them dynamically.

2. **Matches visualizer groups to entity groups by name** (already done via `BuildGroupEntities()`).

3. **Spatial query setup becomes data-driven**: Rust declares which group is "queryable" (has collision
   enabled on its ISMC) and which collision channel to use, instead of hardcoding food-specific logic.

4. **Game-specific C++ module (`RustMassGatherers`)** becomes optional or empty — only needed for
   UE module boilerplate, not simulation logic.

## Dependencies

- Spatial query callback must first be converted to use UE sweep queries (in progress)
- Entity spawning already delegated to Rust via `mass_init_simulation`
- Visualization already generic via `URustMassGenericVisualizer`

## Scope

~200 lines of C++ refactoring. No FFI changes expected — the subsystem just reorganizes
how it stores and maps entity handles internally.
