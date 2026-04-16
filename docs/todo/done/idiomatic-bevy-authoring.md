# Idiomatic Bevy sim authoring improvements (completed)

Make shared sim systems look like pure standard Bevy, with zero Unreal leakage visible to the sim author.

## 1. Auto-generate C++ defaults from Rust `Default`

`MassFragmentRegistration` now includes a `write_default` function pointer that writes `T::default()` bytes into a buffer. The codegen reads field values at runtime and formats them as C++ literals. No more `#[mass(default = "...")]` attributes — `impl Default` is the single source of truth.

## 2. Make `#[mass_system]` a no-op in Bevy mode

The macro now emits `#[cfg(feature = "unreal")]` on all Unreal-specific items and `#[cfg(not(feature = "unreal"))]` on the original function pass-through. Systems use `#[mass_system(order = 10)]` unconditionally — no `cfg_attr` needed. The proc macro is re-exported from `bevy_mass::prelude`.

## 3. Use `Res<Time>` instead of custom `DeltaTime`

Replaced custom `DeltaTime` with standard `bevy_time::Time`. Systems now use `time: Res<Time>` and `time.delta_secs()`. In Unreal mode, `TimePlugin` + `ManualDuration` provides externally-driven time transparently. Standalone runner no longer needs a `sync_delta_time` bridge.

## 4. Document Query type selection rule

Added module-level docs to `bevy_mass::query` explaining when to use `Query`, `BevyQuery`, and `MassQuery`/`MassQueryAll`.

## 5. Unify system wrappers via facade Query + QueryAll

Converted `ant_food_decision`, `apply_food_mutations`, `carried_food_tracking` from `MassQuery`/`MassQueryAll` to facade `Query`/`QueryAll`. Added `QueryAll<D, F>` to bevy_mass with `EntityIndex<Tag>` + `QueryAllWrapper` backing in Bevy mode. Extended `#[mass_system]` macro to rewrite `QueryAll` params in Bevy mode. 5/6 systems are now portable with zero `#[cfg]` gates.

## 6. System ordering documentation

Added convention comment in `gatherers-sim/src/movement.rs`: order = 10, 20, 30, ... with gaps for insertion; Unreal uses order for C++ processor execution; standalone uses `.chain()`.

## 7. Standard Transform/Velocity fragments

Replaced custom `Position` (48 bytes) and `Movement` (32 bytes) with:
- `Transform` — matches UE's `FTransformFragment` layout exactly (96 bytes, align 16: FQuat rotation + padded FVector translation + padded FVector scale). Uses `existing` codegen flag — no USTRUCT generated, only `sizeof` verification.
- `Velocity` — renamed from `Movement` (`direction: DVec3, speed: f32`).
- `PreviousTranslation` — split out from Position for spatial sweep queries.

Added `mass_fragment!(existing)` support: codegen skips USTRUCT generation for types that already exist in UE, emits only `static_assert(sizeof(...))` for layout verification.

## 8. Framework-provided movement infrastructure

Added `bevy_mass::MovementPlugin<T, P, D>` with `TransformLike`, `PrevTranslationLike`, `DesiredMovementLike` traits. In Bevy mode: `apply_movement` system (pos += vel * dt, save PreviousTranslation). In UE mode: no-op (C++ `UMassApplyMovementProcessor` and `URustMassPostMovementProcessor` handle it). Deleted `entity_movement` system. Made `entity_boundary_reflect` read-only Transform (pure velocity reflection, no position clamping). Removed position clamping from C++ PostMovementProcessor.

## 10. SpatialQuery facade

Added `SpatialQuery` resource to `bevy_mass` that wraps UE's `MassSpatialQueries` with a cleaner Rust API (`call()` returns `Option<SpatialHit>` with `DVec3` instead of raw FFI arrays). UE-mode `ant_collision_prepass` uses `Res<SpatialQuery>`. Standalone keeps direct Bevy queries for collision — this is intentional: collision detection and movement application are engine-specific infrastructure (UE uses native physics sweeps, standalone uses Bevy queries). Both produce identical `HitEvent` messages consumed by shared game logic.
