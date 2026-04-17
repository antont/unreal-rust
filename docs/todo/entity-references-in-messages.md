# Entity references in messages — replace indices with Entity where possible

## Context

Cross-archetype entity references currently use numeric spawn-order indices (`i32`) throughout the message pipeline: `SpatialHit.entity_index` → `HitEvent.hittable_index` → `FoodMutation.food_index`. This is asymmetric — ants already use `Entity` (`hitter_entity`) but food uses `i32`. The index-to-Entity mapping infrastructure already exists (`MassEntityMap` in UE mode, `EntityIndex<Food>` in Bevy mode) but is used late in the pipeline.

This change pushes the index→Entity translation to the spatial query boundary, so downstream message-based code uses `Entity`. Fragment types (`Carrying`, `FoodEncounter`) stay as `i32` — they're `#[repr(C)]` with matching C++ layout.

Related: `docs/todo/idiomatic-bevy-authoring.md` item 9, `docs/todo/recreate-physics-state-optimization.md`.

## Design constraints

- **Fragments stay `#[repr(C)]`**: `Carrying { food_index: i32 }` and `FoodEncounter { food_index: i32 }` cannot hold `Entity` — no C++ equivalent, would break chunk layout.
- **Messages are Rust-only**: `HitEvent`, `FoodMutation` are `#[derive(Message)]` — no layout constraint, can hold `Entity`.
- **`QueryAll` still needed**: `apply_food_mutations` and `carried_food_tracking` use `QueryAll<&mut T, With<Food>>` with index-based `.get_mut(i)`. In UE mode this is zero-copy chunk access. Even with Entity on messages, the fragment-level index path for `Carrying` remains.
- **Pure decision function untouched**: `ant_food_decision()` takes `FoodEncounter` (repr(C)) — no Entity there.

## Steps

### Step 1: Add `food_entity: Entity` to `HitEvent` and `FoodMutation`

**File: `gatherers-sim/src/components.rs`**

Add `food_entity: Entity` field to both messages. Keep `hittable_index`/`food_index` — still needed by `FoodEncounter` construction and `QueryAll` lookup.

Update `HitEvent::new()` to accept `hittable_entity: Entity`.

### Step 2: Translate index→Entity at spatial query boundary

**File: `gatherers-bevy-mass/src/systems.rs`** — `ant_collision_prepass`

Add `Res<MassEntityMap>` parameter. After `spatial.call()` returns `SpatialHit { entity_index }`, look up the Entity from MassEntityMap.

**File: `gatherers-standalone/src/main.rs`** — standalone collision prepass

Already has `EntityIndex<Food>` in scope — look up Entity from there.

### Step 3: Propagate `hittable_entity` through decision systems

**File: `gatherers-bevy-mass/src/systems.rs`** — `ant_food_decision`

Add Entity to the hit_map tuple, propagate into `FoodMutation { food_entity, ... }`.

**File: `gatherers-sim/src/food_decision.rs`** — `food_decision_system` (standalone mode)

Same pattern.

### Step 4: Add `Carrying` helper methods

**File: `gatherers-sim/src/components.rs`**

`Carrying::is_carrying() -> bool` — replace scattered `carry.food_index >= 0` checks. No layout change.

### Step 5: Update tests

Update `HitEvent::new()` calls and `FoodMutation` construction in all test files.

### Step 6: Update docs

Note partial progress on `idiomatic-bevy-authoring.md` items 9 and 16.

## What this does NOT change

- `Carrying.food_index` stays `i32` (repr(C) constraint)
- `FoodEncounter.food_index` stays `i32` (repr(C) constraint, fed to pure decision function)
- `QueryAll` usage in `apply_food_mutations` and `carried_food_tracking` — still index-based
- No C++ changes, no FFI changes
