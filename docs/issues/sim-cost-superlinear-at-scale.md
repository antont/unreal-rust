# Sim cost scales superlinearly with entity count

## Status: Open — baseline measured, root-cause analysis pending

Relates to task #163 (scale sweep).

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

## Interpretation

The fixed 4× food:ant ratio means doubling ants also doubles food, so
each step effectively does pairwise ant × food spatial work inside the
10000×10000 bounds. Food density rises linearly with ants at this
ratio, so a near-quadratic slope is consistent with the spatial-query
layer returning O(density) candidates per ant.

Two hypotheses worth testing before any fix:

1. **Density, not cardinality, drives the slope.** A constant-food
   sweep (e.g. food=4000 at every ant count) would decouple the two
   axes: if the growth slope flattens to linear, density is the
   problem and we should look at GridHash cell size / query radius
   rather than the per-system algorithm.
2. **A specific system scales badly.** `ant_collision_prepass` and
   `ant_food_decision` both touch ants × (food-in-radius); a
   per-system breakdown at each scale (via the existing
   `record_mass_system_time` / `drain_mass_system_samples`
   infrastructure) would pinpoint which system owns the slope.
   Currently `[mass-perf]` goes through `eprintln!`, which UE on Mac
   loses when the editor daemonizes — getting this breakdown into
   the UE log needs a small FFI.

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
