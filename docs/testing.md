# Testing

## When Rust tests are not enough — run UE automation

`cargo test` covers ~11,000 tests but does not build a real Bevy
`App` with game-crate inventory items wired in. Some failure classes
only surface when the full schedule is constructed at UE runtime:

- Macro-emitted systems with conflicting `SystemParam` shapes
  (Bevy error B0001).
- Dispatch-order bugs that need the C++ coordinator to trigger.
- FFI layout or null-pointer regressions in the loader chain.

There's a game-crate smoke test
(`gatherers-bevy-mass::tests::schedule_builds_and_runs_without_conflicts`)
that builds the schedule from inventory and runs it once on an empty
world, which catches B0001-class regressions at `cargo test` time.
It still isn't a substitute for UE automation — it doesn't exercise
real chunk memory, the coordinator, or hot-reload.

**Required before committing any change to any of these:**

- `unreal-api-derive/` (proc-macro crate; emission bugs surface only
  in real schedules)
- `unreal-api/src/mass.rs` (schedule / SystemParam / dispatch hooks)
- `unreal-module/src/mass_system_registry.rs` (schedule build, hook
  dispatch, reload plumbing)
- `RustPlugin/Source/RustPlugin/RustMassDynamicProcessor.cpp` and the
  coordinator
- Any macro attribute that rewrites system signatures
  (`#[mass_system]`, `#[component]`, `#[bevy]`)

Run `cargo build --release -p unreal-rust-host` followed by the UE
automation command below and grep for `EXIT CODE: 0` before opening
a PR. Cost is ~90 seconds on top of the Rust suite.

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
# Gatherers (default host)
cargo build --release -p unreal-rust-host

# Vivarium (separate host dylib)
cargo build --release -p vivarium-rust-host
```

Each sim is built as its own cdylib (one dylib per sim). The loader
selects which one to load at runtime via the `UNREAL_RUST_HOST_NAME`
environment variable — default is `unreal_rust_host` (gatherers). To
point UE at vivarium:

```bash
export UNREAL_RUST_HOST_NAME=vivarium_rust_host
```

The dylibs land at `target/release/libunreal_rust_host.dylib` and
`target/release/libvivarium_rust_host.dylib`. Unreal hot-reloads via the
loader chain (`unreal_rust_loader.dylib` → selected host dylib).

A future phase will surface the host-selection as a UE project setting so
per-map / per-blueprint instantiation becomes possible; for now the env
var is the only knob.

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

### PIE perf tests (full engine frame cost)

Unlike the other `[perf]` / `[scale-sweep]` tests — which call `Subsystem->Tick()` or `RunSimulationProcessorsForTesting()` directly and therefore bypass `FEngineLoop::Tick` — the PIE perf tests run inside a real Play-In-Editor session. Each measured frame includes render, Slate, actor ticks, other Mass subsystems, and the global world tick. These are the only automation tests that answer "what does a real editor frame cost at N ants?"

Diagnostic-only: no pass/fail on frame times. Results land in the UE log as `[pie-perf]` lines and in the utrace bracketed by `PIEPerf: Begin/End <scenario>` bookmarks.

```bash
# Run both PIE scenarios (1k and 10k ants) with trace capture
"/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor" \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
  -trace=cpu,frame,bookmark \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Gatherers.PIE;Quit" \
  -stdout -FullStdOutLogOutput

# Read the per-scenario summary lines
grep "\[pie-perf\]" \
  "/Users/tonialatalo/Library/Logs/Unreal Engine/RustExampleEditor/RustExample.log"
```

Traces captured with `-trace` are written to the UnrealTraceServer store at `~/UnrealEngine/UnrealTrace/Store/001/*.utrace`. Open the newest one in `UnrealInsights.app` to see `PIEPerf: Begin/End` bookmarks alongside RenderThread / GameThread / MassSimulation scopes and the five `RustMass_*` CPU scopes inside the subsystem.

The `-tracefile=<path>` flag often fails on Mac when Unreal Insights is already running (the trace server auto-connects via local socket instead); the store-written trace is the authoritative one.

Tests:
- `supplemental.RustPlugin.Gatherers.PIE.FrameCost1k` — 1k ants, 4k food
- `supplemental.RustPlugin.Gatherers.PIE.FrameCost10k` — 10k ants, 40k food

Both reuse `/Game/Gatherers/GatherersBevyMass` and re-initialize the sim from the test side (`ResetSimulation` + `InitializeSimulation`) after PIE starts, so there are no test-only map assets to maintain.

Test source: `RustPlugin/Source/RustPluginTests/Private/RustMassGatherersPIE.spec.cpp`

#### Exporting aggregate timer data from Unreal Insights

The `[pie-perf]` line gives per-frame wall times; the utrace gives per-scope breakdown. For offline analysis (grep/spreadsheet/diffing between runs) you can export the **Timers** panel to CSV:

1. Open the utrace in `UnrealInsights.app`.
2. (Optional) Select a frame range in the Frames track to scope the aggregates. Without a selection, timers aggregate the whole trace.
3. Make sure the Timers panel search box is **empty** — a filter there will silently restrict the export.
4. Right-click any row in the Timers panel → **Export → Export to File…** (NOT the *Timing Events* panel, which dumps raw per-event rows with unresolved IDs).

Output is a UTF-16 CSV with columns `Name,Count,Incl,Excl`; times are in **seconds**. Convert to UTF-8 for grepping:

```bash
iconv -f UTF-16 -t UTF-8 TimerStats.csv | grep -E "RustMass_|RustMassDynamicProcessor|RustMassScheduleCoordinator"
```

`RustMassScheduleCoordinator_1` holds the whole dispatched Rust sim cost; `RustMassDynamicProcessor_N` rows are the per-system chunk-pointer cache rebuild (cache-only mode — the actual system work happens inside the coordinator).

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
