# Idiomatic Bevy sim authoring improvements

Make shared sim systems look like pure standard Bevy, with zero Unreal leakage visible to the sim author.

## 1. ~~Auto-generate C++ defaults from Rust `Default`~~ ✅ DONE

`MassFragmentRegistration` now includes a `write_default` function pointer that writes `T::default()` bytes into a buffer. The codegen reads field values at runtime and formats them as C++ literals. No more `#[mass(default = "...")]` attributes — `impl Default` is the single source of truth.

## 2. ~~Make `#[mass_system]` a no-op in Bevy mode~~ ✅ DONE

The macro now emits `#[cfg(feature = "unreal")]` on all Unreal-specific items and `#[cfg(not(feature = "unreal"))]` on the original function pass-through. Systems use `#[mass_system(order = 10)]` unconditionally — no `cfg_attr` needed. The proc macro is re-exported from `bevy_mass::prelude`.

## 3. ~~Use `Res<Time>` instead of custom `DeltaTime`~~ ✅ DONE

Replaced custom `DeltaTime` with standard `bevy_time::Time`. Systems now use `time: Res<Time>` and `time.delta_secs()`. In Unreal mode, `TimePlugin` + `ManualDuration` provides externally-driven time transparently. Standalone runner no longer needs a `sync_delta_time` bridge.

## 4. ~~Document Query type selection rule~~ ✅ DONE

Added module-level docs to `bevy_mass::query` explaining when to use `Query`, `BevyQuery`, and `MassQuery`/`MassQueryAll`.

## 5. Investigate unifying `food_decision_system` wrappers (exploratory)

The standalone and Unreal versions of the food decision system are nearly identical — both read hit messages, call the shared pure function, emit mutations. The only difference is data access (`Query` vs `MassQuery`). If the macro supported `MessageReader`/`MessageWriter` as facade types in shared systems, this wrapper could potentially be written once.

## 6. ~~System ordering documentation~~ ✅ DONE

Added convention comment in `gatherers-sim/src/movement.rs`: order = 10, 20, 30, ... with gaps for insertion; Unreal uses order for C++ processor execution; standalone uses `.chain()`.
