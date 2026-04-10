# Testing

## Rust tests

```bash
# All Rust crates
cargo test

# Specific crate
cargo test -p gatherers-bevy-mass
cargo test -p unreal-api
cargo test -p unreal-ffi
cargo test -p gatherers-sim
```

## Rebuild Rust dylib for Unreal

Unreal loads the **release** build. Debug builds are not used.

```bash
cargo build --release -p unreal-rust-host
```

The dylib lands at `target/release/libunreal_rust_host.dylib`. Unreal hot-reloads it via the loader chain (`unreal_rust_loader.dylib` -> `libunreal_rust_host.dylib`).

## Unreal C++ automation tests

Run from CLI — no need to open the full editor GUI. A black window with log output will appear briefly.

```bash
"/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor" \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Gatherers;Quit" \
  -stdout -FullStdOutLogOutput
```

**Important flags to NOT use** (they cause RustPlugin module to fail to load):
- `-NullRHI`
- `-Unattended`
- `-game`

The shell exit code is always non-zero (editor shutdown quirk). Check actual results in the UE log:

```bash
grep "Test Completed" "/Users/tonialatalo/Library/Logs/Unreal Engine/RustExampleEditor/RustExample.log"
grep "TEST COMPLETE. EXIT CODE" "/Users/tonialatalo/Library/Logs/Unreal Engine/RustExampleEditor/RustExample.log"
```

`EXIT CODE: 0` = all tests passed, `EXIT CODE: -1` = failures.

### Running a subset of tests

The filter in `RunTests` is a prefix match:

```bash
# All Gatherers tests
-ExecCmds="Automation RunTests supplemental.RustPlugin.Gatherers;Quit"

# Single test
-ExecCmds="Automation RunTests supplemental.RustPlugin.Gatherers.BevyMassFoodPickup;Quit"
```

### Current tests (17)

- BevyMassAutoInitFromRustDefaults
- BevyMassAutoInitIdempotent
- BevyMassBoundaryReflect
- BevyMassCooldown
- BevyMassDescriptorLayout
- BevyMassFoodFragmentLayout
- BevyMassFoodPickup
- BevyMassIntegration
- BevyMassOnRustReloadedResetsState
- BevyMassReloadCycle
- BevyMassReloadCycleMultiple
- BevyMassRustSimDefaultsFFI
- BevyMassRustSpatialQueryConfigFFI
- BevyMassSpatialQueryLayout
- BevyMassSpawnAndSimulate
- BevyMassSubsystemRegistered
- BevyMassSystemOrdering

C++ test source: `RustPlugin/Source/RustPluginTests/Private/RustMassGatherers.spec.cpp`

### Rust-authored UE tests

Tests authored in Rust that run inside the UE editor with full Mass Entity + physics support.
These appear under `supplemental.RustPlugin.RustTests.*` in the automation test browser.

```bash
# Run only Rust-authored tests
-ExecCmds="Automation RunTests supplemental.RustPlugin.RustTests;Quit"
```

Current Rust-authored tests:
- SpawnAndSimulate
- BoundaryReflect
- CooldownDecrement

Rust test source: `gatherers-bevy-mass/src/ue_tests.rs`
C++ runner: `RustPlugin/Source/RustPluginTests/Private/RustAuthoredTests.cpp`

## Rust hot-reload during PIE

Rust code changes take effect in a running PIE session without restarting the editor.

1. Start PIE (Play in Editor)
2. Edit Rust code (e.g. movement speed in `gatherers-sim/src/movement.rs`)
3. Build the Rust dylib:
   ```bash
   cargo build --release -p unreal-rust-host
   ```
4. The editor detects the timestamp change, reloads the dylib, and resets the Mass simulation automatically

The Output Log should show:
```
LogTemp: Warning: Hotreload
LogTemp: Display: RustMassBevySubsystem: Rust hot-reload detected, resetting simulation
```

The simulation restarts from scratch with the new Rust code — entities are destroyed and re-spawned using `TryAutoInitFromRustDefaults()`.

**Note:** This requires the C++ plugin to be compiled with the `OnRustReloaded()` hook. If you only see `Hotreload` but not the reset message, rebuild the C++ side (UE toolbar compile button or restart the editor after a CLI build).

## Unreal C++ build (CLI)

```bash
"/Users/Shared/Epic Games/UE_5.7/Engine/Build/BatchFiles/Mac/Build.sh" \
  RustExampleEditor Mac Development \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject"
```
