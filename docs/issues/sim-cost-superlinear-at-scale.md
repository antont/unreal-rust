# Sim cost scales superlinearly with entity count

## Status: Closed — not pursuing; 1k target is adequate

Slope owner is `ant_collision_prepass` (per-ant `food_pickup` GridHash
query). No further work planned: the project target is ≤1k ants, where
the full editor frame hits the 60 Hz v-sync cap (16.67 ms).

The PIE breakdown below also overturned the earlier "vis pipeline at
scale" concern (#162): at 10k the frame is 76% Rust sim (inside
`ant_collision_prepass`), not vis. #162 is therefore closed too.

Relates to task #163 (scale sweep).

## PIE breakdown at 10k (2026-04-23, Apple M3 Max)

From `supplemental.RustPlugin.Gatherers.PIE.FrameCost10k`, 65 measured
frames inside the `PIEPerf: Begin 10k` → `End 10k` window:

| Scope                     | ms/frame | Share of ~86 ms frame |
|---------------------------|---------:|----------------------:|
| RustMass_Tick (incl)      |     67.2 |                   78% |
| └ RustMass_SimStep        |     65.3 |                   76% |
| └ RustMass_VisPipeline    |      1.7 |                    2% |
| └ RustMass_ApplyFoodEvents|      0.2 |                  0.3% |
| MassUpdateISMProcessor    |      1.0 |                  1.2% |
| Other (render/Slate/...)  |    ~18.8 |                   22% |

`RustMassScheduleCoordinator_1` (sim dispatch) has Excl=Incl=65 ms —
meaning the entire SimStep cost sits in untraced Rust work, which the
`[mass-perf]` logs attribute to 99% `ant_collision_prepass`. Vis is a
rounding error at this scale.

## Decision

At the 1k design target, the full editor frame is 16.67 ms (60 Hz cap),
`RustMass_Tick` is ~1 ms, and there is no perf problem. Investigating
further scaling (constant-food density sweep, grid-cell tuning, etc.)
is out of scope.

## Historical data (kept for reference)

## Summary

Rust sim processors (excluding the UE visualization pipeline) scale at
roughly **O(N^1.9)** on the `gatherers-bevy-mass` workload. At 8000
ants the sim-only portion of one simulation step already costs 42 ms,
well past the 16.7 ms 60 Hz frame budget. This is measured independently
of the vis-pipeline cost tracked in task #162.

## Baseline (main, 2026-04-21, Apple M3 Max)

`supplemental.RustPlugin.Gatherers.BevyMassPerfScaleSweep` runs
120 measured ticks of `RunSimulationProcessorsForTesting` after 20
warmup ticks, with the same 4× food:ant ratio as the single-scale
`BevyMassPerfProfile`.

| ants | food   | p50 (sim+drops) | vs previous 2× step |
|-----:|-------:|----------------:|--------------------:|
| 1000 |  4000  | 0.82 ms         | —                   |
| 2000 |  8000  | 2.96 ms         | ×3.60 (SUPERLINEAR) |
| 4000 | 16000  | 11.27 ms        | ×3.81 (SUPERLINEAR) |
| 8000 | 32000  | 41.72 ms        | ×3.70 (SUPERLINEAR) |

Each 2× step in ant count produces roughly 3.7× growth in p50 — very
close to quadratic.

## Per-system breakdown (2026-04-21, after routing `[mass-perf]` to UE log)

With `UNREAL_RUST_MASS_TIMING=1`, averages across 120 measured ticks
(20 warmup skipped), same sweep:

| ants | prepass | decision | mutations | carry | bounce | cooldown | total |
|-----:|--------:|---------:|----------:|------:|-------:|---------:|------:|
| 1000 |  0.786  |  0.007   |  0.000    | 0.001 | 0.005  | 0.001    | 0.799 |
| 2000 |  2.859  |  0.018   |  0.000    | 0.003 | 0.011  | 0.002    | 2.892 |
| 4000 | 11.052  |  0.046   |  0.001    | 0.019 | 0.026  | 0.005    |11.148 |
| 8000 | 40.810  |  0.120   |  0.002    | 0.086 | 0.064  | 0.011    |41.093 |

All times in ms. `ant_collision_prepass` owns **99.3%** of the cost at
every scale and its 2× ratios are 3.64× / 3.87× / 3.69× — matching the
overall slope exactly. Every other system stays well under 0.15 ms even
at 8k ants.

`ant_collision_prepass` body (`gatherers-bevy-mass/src/systems.rs:169`)
is a single loop over cooldown-free ants calling
`spatial.call("food_pickup", &prev, &curr)` per ant. Each call goes
through `ExecuteGridHashSpatialQuery` (`RustMassBevySubsystem.h:265`).
The slope therefore lives in the grid query itself, not in the Rust
iteration or any other system.

## Interpretation

The fixed 4× food:ant ratio means doubling ants also doubles food, so
each step effectively does pairwise ant × food spatial work inside the
10000×10000 bounds. Food density rises linearly with ants at this
ratio, so a near-quadratic slope is consistent with the spatial-query
layer returning O(density) candidates per ant.

Hypothesis 2 (specific system owns the slope) is now confirmed:
`ant_collision_prepass` — a per-ant `food_pickup` spatial query —
carries effectively all the growth. The per-ant loop is O(ants), so
the superlinear term lives inside each query call: the grid returns
O(density) candidates, and density rises linearly with food count at
the fixed 4× ratio. ants × density → quadratic.

Next step — isolate density vs cardinality. A constant-food sweep
(e.g. food=4000 at every ant count) would decouple them:
  - flat slope → density inside each query dominates. Fix by tuning
    cell size / query radius, or filtering candidates earlier inside
    `ExecuteGridHashSpatialQuery`.
  - still superlinear → something inside the grid call is O(ants) or
    worse independently of density (e.g. the candidate-list scan or
    `EntityToIndex` lookup). Reach for `GridHashCounters` at each
    scale before changing anything.

Task #162 (vis pipeline) is a separate, larger cost at scale (12 s
full Tick at 10k vs 42 ms sim at 8k).

## Reproducing

```bash
"/Users/Shared/Epic Games/UE_5.7/Engine/Binaries/Mac/UnrealEditor" \
  "/Users/tonialatalo/src/unreal-rust/example/RustExample/RustExample.uproject" \
  -ExecCmds="Automation RunTests supplemental.RustPlugin.Gatherers.BevyMassPerfScaleSweep;Quit" \
  -stdout -FullStdOutLogOutput

grep "\[scale-sweep\]" "/Users/tonialatalo/Library/Logs/Unreal Engine/RustExampleEditor/RustExample.log"
```

Test source: `RustPlugin/Source/RustPluginTests/Private/RustMassGatherers.spec.cpp`
(`FGatherersBevyMassPerfScaleSweepTest`).
