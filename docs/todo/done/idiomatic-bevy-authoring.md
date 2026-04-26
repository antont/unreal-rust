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

## 11. `#[component]` attribute macro for game types

Added `#[component]` attribute macro to `unreal-api-derive`, re-exported from `bevy_mass::prelude`. Replaces `mass_fragment!`/`mass_tag!` for game-authored types. Auto-detects tags (unit structs) vs fragments (structs with fields), adds `#[repr(C)]`, `#[derive(Component, Clone, Copy, Debug)]`, and conditionally `#[derive(MassFragment)]` + `#[mass(cpp_type = "...")]` in UE mode. C++ type names are auto-derived from `BEVY_MASS_CPP_PREFIX` env var with UE-conventional suffixes: `F{Prefix}{StructName}Fragment` for data structs, `F{Prefix}{StructName}Tag` for unit structs. Explicit `cpp_type` override still available but no game component uses it. Engine types (Transform, DesiredMovement, etc.) live in `bevy_mass::components` and keep using `mass_fragment!` with `existing`/`include`.

## 13. Engine types moved to framework

Moved Transform, Velocity, DesiredMovement, and CodeDrivenMovementTag from `gatherers-sim` to `bevy_mass::components`. Game code re-exports them for backward compatibility. This separates engine-level types (framework) from game-authored types (game crate).

## 14. Dead code cleanup: AntMassTag deleted

`AntMassTag` was never used in any system, query, or entity archetype. Deleted from `gatherers-sim` and all re-exports.

## 12. `#[derive(Component, MassFragment)]` replaces `#[component]`

Game components declare chunk-backed fragments with vanilla-looking syntax plus an explicit opt-in: `#[repr(C)] #[derive(Component, MassFragment, Clone, Copy, Debug)] pub struct ...`. The `MassFragment` derive auto-detects tags (unit structs), resolves `cpp_type` from `BEVY_MASS_CPP_PREFIX` + struct name + "Fragment"/"Tag" suffix, emits `ChunkBacked` impl, and supports `#[mass(group = "...")]` for tag grouping. All emitted `unreal_api::mass::*` references are `#[cfg(feature = "unreal")]`-gated so the derive is a no-op in pure-Bevy builds. The `#[component]` attribute macro is retained but deprecated. Game code now reads as vanilla Bevy except for the `#[repr(C)]` line on chunk-backed fragments.

## 9. Entity references in cross-archetype messages

`HitEvent` and `FoodMutation` now carry the hittable's shadow Bevy `Entity` alongside the existing chunk-slot `i32` index. Translation happens once at the spatial-query boundary (UE: `MassEntityMap::get(group, idx)`; standalone: `EntityIndex<Food>` in scope). `Carrying::is_carrying()` helper replaces scattered `food_index >= 0` checks. Fragments stay `#[repr(C)]` with `i32` indices (matching the C++ layout — the design-doc constraint from `docs/todo/entity-references-in-messages.md`). Consumer systems can now look up cross-archetype entities through the normal Bevy `Query::get(entity)` interface instead of index-keyed lookups, when the const-if dispatch supports it on the primary query.

## 16. `BevyQuery` removal (tracked with item 9)

`BevyQuery` is now a deprecated re-export; game code uses the `#[bevy]` parameter attribute (or nothing — `QueryBackend::IS_CHUNK` const-if picks the right backend automatically). `QueryAll` retirement is partial: decision-system consumers kept using it as the index-based chunk-access facade until entity-keyed access lands on primary queries; one consumer (`carried_food_tracking`) migrated to `Query<&mut Transform>` via `MassEntityMap`/`EntityIndex` resolution.

## 15. Bevy-style `.chain()` ordering

Game systems no longer declare `#[mass_system(order = N)]`. Execution order comes from a plugin-level `MassScheduleOrder` inventory submission that lists the system names in order; the framework maps the list to numeric order values at plugin init (stride 10) and both the Bevy schedule and the C++ processor pipeline read the resolved value. `#[mass_system]` order is now optional (sentinel `u32::MAX`). Insert new systems at the right position in the list — no renumbering required. Standalone mode continues to use real Bevy `.chain()` in `app.add_systems(Update, (...).chain())`, which is already the idiomatic path there.
