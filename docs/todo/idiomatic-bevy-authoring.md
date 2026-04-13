# Idiomatic Bevy sim authoring improvements

Make shared sim systems look like pure standard Bevy, with zero Unreal leakage visible to the sim author.

## 1. Auto-generate C++ defaults from Rust `Default` (high impact)

Remove `#[mass(default = "FVector(1.0f, 0.0f, 0.0f)")]` from fragment definitions. The `gen_fragments` binary already generates C++ headers — extend it to emit default initializers derived from each struct's Rust `Default` impl. Sim authors should never write C++ literals.

## 2. Make `#[mass_system]` a no-op in Bevy mode (medium impact)

Currently every shared system needs `#[cfg_attr(feature = "unreal", mass_system(order = 10))]`. If the macro passes through the original function unchanged when the `unreal` feature is off, systems can just write `#[mass_system(order = 10)]` unconditionally. The macro crate would need a `bevy-backend` feature gate.

## 3. ~~Use `Res<Time>` instead of custom `DeltaTime`~~ ✅ DONE

Replaced custom `DeltaTime` with standard `bevy_time::Time`. Systems now use `time: Res<Time>` and `time.delta_secs()`. In Unreal mode, `TimePlugin` + `ManualDuration` provides externally-driven time transparently. Standalone runner no longer needs a `sync_delta_time` bridge.

## 4. Document Query type selection rule (low effort)

Add a one-liner to the `bevy_mass::prelude` docs: "`Query` for data that exists in both modes. `BevyQuery` for components that only exist on Bevy entities (like dynamically added/removed `Cooldown`)." The three query types (`Query`, `BevyQuery`, `MassQuery`) are justified by the architecture but need a clear decision rule.

## 5. Investigate unifying `food_decision_system` wrappers (exploratory)

The standalone and Unreal versions of the food decision system are nearly identical — both read hit messages, call the shared pure function, emit mutations. The only difference is data access (`Query` vs `MassQuery`). If the macro supported `MessageReader`/`MessageWriter` as facade types in shared systems, this wrapper could potentially be written once.

## 6. System ordering documentation (low priority)

The `order = 10, 20, 30, ...` numbers work but are Unreal-specific. The standalone runner uses `.chain()`. No code change needed, but worth documenting the convention (gaps of 10 for future insertion, standalone uses chain for ordering).
