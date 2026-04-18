# Testing

## Rust tests (11,293 total)

```bash
# All Rust crates
cargo test

# Specific crate
cargo test -p gatherers-bevy-mass
cargo test -p unreal-api         # includes 11,165 layout verification tests
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

### Current C++ tests (23)

- BevyMassAutoInitFromRustDefaults
- BevyMassBoundaryReflect
- BevyMassCarriedFoodTracking
- BevyMassDescriptorLayout
- BevyMassFoodFragmentLayout
- BevyMassFoodPickup
- BevyMassHotReloadReInit
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
- BevyMassTickPathMovement
- BevyMassVisMovement
- BevyMassVisProcessorExecution
- BevyMassVisProcessorRegistration
- BevyMassVisualizationFragments
- BevyMassVisSimCoexistence

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
- FoodPickup
- FoodDrop
- Integration

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

### Automated hot-reload tests (CLI)

Two external-process scripts exercise the full reload cycle end-to-end. Both launch a headless editor, verify initial plugin load, trigger a reload, and assert the editor picked up the new dylib. Exit 0 on success.

```bash
# ~15s — tests the reload infrastructure only (timestamp detect + unique-name + codesign).
# Touches the dylib; no cargo rebuild.
./scripts/hot_reload_smoke_test.sh

# ~30s — edits a Rust source file, runs `cargo build --release -p unreal-rust-host`,
# asserts a unique marker from the edit appears in the editor log after reload.
# Auto-reverts the source edit on exit (success or failure).
./scripts/hot_reload_rebuild_test.sh
```

The in-process UE automation test `BevyMassReloadPreservesDispatchHooks` covers `OnRustReloaded()`'s state-cleanup paths (inventory bindings, drop cache) but does NOT actually rebuild or reload the dylib — these shell scripts do.

## Standalone screenshot regression (pre-commit hook)

`gatherers-standalone` is a pure-Bevy harness (no Unreal) that runs the same `gatherers-sim` + `bevy_mass` crates the UE plugin uses. A pre-commit hook rebuilds it, runs it with a deterministic clock + fixed seed, and compares three frames against committed references with `odiff`. This catches sim regressions (ordering changes, movement/collision tweaks, renames that compile but break visuals) before they leave your machine.

### Install

```bash
chmod +x scripts/pre-commit.sh scripts/verify_standalone.sh scripts/update_standalone_references.sh
ln -sf ../../scripts/pre-commit.sh .git/hooks/pre-commit
```

Requires `odiff` on PATH (install with `npm i -g odiff-bin` or Homebrew).

### What triggers the hook

Only when staged paths touch one of:
- `gatherers-standalone/`
- `gatherers-sim/src/`
- `bevy_mass/src/`
- `scripts/verify_standalone.sh`
- `gatherers-standalone/test/reference_screenshots/`

Unrelated commits (README, UE C++, docs) exit the hook instantly.

### Commands

```bash
# Run the check manually
scripts/verify_standalone.sh

# Custom tolerance (default 0.5%)
TOLERANCE=1.0 scripts/verify_standalone.sh

# Bypass the hook for an intentional or emergency commit
git commit --no-verify
```

### Intentional regressions

When a change legitimately alters the sim's output, regenerate the references:

```bash
scripts/update_standalone_references.sh       # interactive confirm
scripts/update_standalone_references.sh -y    # no prompt
git add gatherers-standalone/test/reference_screenshots/
```

The determinism comes from `bevy::time::TimeUpdateStrategy::ManualDuration(1/60s)`, set when the standalone is invoked with `--deterministic-clock`. Combined with the seed-42 LCG spawn in `gatherers-standalone/src/main.rs`, runs are pixel-reproducible. Diff images land in `/tmp/standalone_regression/` for inspection on failure.

## Unreal C++ build (CLI)

```bash
"/Users/Shared/Epic Games/UE_5.7/Engine/Build/BatchFiles/Mac/Build.sh" \
  RustExampleEditor Mac Development \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject"
```
