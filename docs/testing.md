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

Test source: `RustPlugin/Source/RustPluginTests/Private/RustMassGatherers.spec.cpp`

## Unreal C++ build (CLI)

```bash
"/Users/Shared/Epic Games/UE_5.7/Engine/Build/BatchFiles/Mac/Build.sh" \
  RustExampleEditor Mac Development \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject"
```
