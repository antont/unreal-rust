# PIE-driven perf automation — design

## Problem

Every perf test we have (`BevyMassPerfProfile`, `BevyMassPerfScaleSweep`, and
all the functional specs) drives the simulation by calling
`URustMassBevySubsystem::Tick()` or `RunSimulationProcessorsForTesting()`
directly. That bypasses `FEngineLoop::Tick` — so none of them measure what
the editor actually does each frame: render, Slate, physics, actor ticks,
other Mass subsystems, the global world tick.

Consequence: we can answer "what does our subsystem cost?" but not "what's
the real editor frame cost at 1k ants?" or "where does the 10k-ant
blowup actually live inside an engine frame?" The 200 manual PIE runs
during the food-pickup debugging would have been much cheaper with a
reproducible PIE perf capture.

## Goal

A reproducible, CLI-launchable automation test that starts PIE, ticks the
full engine loop for a fixed window, and captures both a frame-cost summary
and an Unreal Insights trace. Diagnostic-only for now — no pass/fail
thresholds. Threshold promotion is a later project once we know the
numbers.

## Non-goals

- Threshold-based CI pass/fail (future promotion path; see §8).
- Replacing the existing subsystem-level perf tests — those answer a
  different question (pure sim cost) and stay as-is.
- Running in Shipping / Test builds. Editor PIE only.
- CI integration. Local validation first; CI wiring is a follow-up.
- Extending the per-scope CPU trace beyond the five `RustMass_*` scopes
  already in `RustMassBevySubsystem.cpp`.

## Architecture

One new test file: `RustPlugin/Source/RustPluginTests/Private/RustMassGatherersPIE.spec.cpp`.
Kept separate from `RustMassGatherers.spec.cpp` so the fast unit/integration
tests in that file stay fast and their build unit stays small.

Two tests under the path `supplemental.RustPlugin.Gatherers.PIE.*`:

- `FrameCost1k` — loads `Perf_Gatherers_1k.umap`, runs the PIE perf
  scenario, emits `[pie-perf]` log line.
- `FrameCost10k` — same, against `Perf_Gatherers_10k.umap`.

Both tests always succeed at the framework level (no `TestEqual` on frame
times). Numbers go to the log via `AddInfo`. A `.utrace` is written on
every run regardless of result.

## Levels

Under `example/RustExample/Content/Tests/`:

- `Perf_Gatherers_1k.umap`
- `Perf_Gatherers_10k.umap`

Each contains:

- A `PlayerStart` (required; PIE errors without a valid spawn point).
- One `ARustSimActivator` with `bOverrideDefaults=true`, groups set to
  `[ants=N, food=4N]`, seed `42`.
- The existing gatherers visual assets (ant/food ISM templates used by
  `GatherersBevyMass.umap`). We measure the real vis pipeline — a
  bare level would miss the #162 blowup entirely.

The levels live under `Content/Tests/` and are not shipped.

## Per-frame measurement

Inside each test body, after the map is loaded:

1. **Start PIE** via `GEditor->RequestPlaySession(...)` with a one-player
   single-instance config. Then pump the engine one tick at a time — call
   `GEditor->Tick(DeltaTime, false)` in a loop — until a PIE `UWorld` is
   alive (`GEditor->PlayWorld != nullptr`) and
   `URustMassBevySubsystem::HasManagedSimulation()` returns true. 5 s
   wall-clock timeout; bail with a clear `AddError` if exceeded.
2. **Warmup** — tick the editor for 60 frames (`GEditor->Tick(DeltaTime, false)`)
   at fixed `DeltaTime = 1/60 s`. Skips first-frame ISMC allocation, shader
   compile jitter, etc.
3. **Measured window** — emit `TRACE_BOOKMARK("PIEPerf: Begin <scenario>")`,
   then tick 300 frames capturing `FPlatformTime::Seconds()` before and
   after each tick into a `TArray<double>`. Emit
   `TRACE_BOOKMARK("PIEPerf: End <scenario>")`. 300 × 1/60 s = 5 s of
   measured activity.
4. **Summarize** — sort samples, compute avg/p50/p99/min/max, emit one
   `[pie-perf] <scenario> …` line matching the `[perf]` format in
   `BevyMassPerfProfile`.
5. **End PIE** via `GEditor->RequestEndPlayMap()`, pump the ticker until
   the PIE world is gone. 5 s timeout; fail-loud if it hangs so the next
   test doesn't inherit a broken state.

The fixed `DeltaTime` is what the tick is *told* the frame is; the
wall-clock time we *measure* is what the frame actually costs. That's what
we want — a predictable logical-time advance and a measured real-time cost.

Tracing piggybacks on `-trace=cpu,frame,bookmark` at editor launch time.
No new tracing APIs needed. Bookmarks let Insights slice the utrace into
warmup vs measured vs teardown.

## Harness

Extract PIE plumbing into a helper header so a third scenario can be added
later without duplicating the machinery:

```cpp
// Private/PIEPerfHelpers.h
struct FPIEPerfConfig {
    FString MapPath;           // "/Game/Tests/Perf_Gatherers_1k"
    FString ScenarioName;      // "1k"
    int32   WarmupFrames    = 60;
    int32   MeasuredFrames  = 300;
    float   FixedDeltaTime  = 1.0f / 60.0f;
};

struct FPIEPerfResult {
    double AvgMs, P50Ms, P99Ms, MinMs, MaxMs, TotalMs, WallMs;
    TArray<double> FrameSamples;
};

bool RunPIEPerfScenario(FAutomationTestBase& T,
                        const FPIEPerfConfig& Cfg,
                        FPIEPerfResult& Out);
```

Each test body is ~10 lines: fill `FPIEPerfConfig`, call
`RunPIEPerfScenario`, let the helper emit its own `[pie-perf]` line.

## Failure modes

Explicit handling for each so a bad run doesn't corrupt follow-up tests:

- **PIE failed to start** → `AddError` with the engine's own error text,
  return `false`. Don't leave PIE partially initialized.
- **World not alive after 5 s of pump** → `AddError`, attempt
  `RequestEndPlayMap`, return `false`.
- **Activator didn't initialize the sim** (`HasManagedSimulation` false
  after the 5 s pump) → `AddError` stating which pre-PIE invariant was
  violated (empty groups, missing ISM templates, etc), return `false`.
- **End-PIE hangs** → timeout on the teardown pump, `AddError`, leave a
  note for the run log. Don't block indefinitely.

Every failure path emits the `.utrace` anyway so we can see what happened
in the seconds up to the failure.

## Documentation

- `docs/testing.md` gets a new "PIE perf tests" section with:
  - The CLI invocation: `Automation RunTests
    supplemental.RustPlugin.Gatherers.PIE`.
  - Where `.utrace` files land (`Saved/Profiling/` under the project).
  - How to open them in `UnrealInsights.app`.
  - The warn-only semantics (no CI pass/fail yet).

## Success criteria

The project is done when:

1. `Automation RunTests supplemental.RustPlugin.Gatherers.PIE;Quit` runs
   both scenarios locally from the CLI, EXIT CODE: 0.
2. Each run writes a named `.utrace` to `Saved/Profiling/` that opens in
   `UnrealInsights.app` and shows RustMass_* scopes inside a real engine
   frame (RenderThread, GameThread, MassSimulation, etc).
3. Both `[pie-perf]` lines land in the UE log with avg/p50/p99 frame
   times.
4. Failure modes above are exercised at least once during implementation
   (intentionally break each, confirm the harness fails loud).
5. `docs/testing.md` section lets someone else repro the run.

## Future promotion (explicitly not now)

- **Threshold pass/fail**: once we have 10 days of runs at 1k and know
  what "acceptable" means, swap `AddInfo` → `TestTrue` against absolute
  budgets. Likely 16.7 ms avg for 1k; 10k stays warn-only until #162
  lands.
- **Baseline-file regression** (perf-baseline.json + 20% threshold):
  considered and rejected for now — adds an artifact to maintain and a
  percentage knob to tune before we know the numbers.
- **Headless/CI**: this test requires `-NullRHI` to be *off* (the same
  constraint as the existing tests — see `docs/testing.md`), so CI needs
  a macOS runner with a display context. Defer until we see whether the
  output is stable enough to gate merges on.
