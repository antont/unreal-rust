# RecreatePhysicsState per-frame optimization

`URustMassBevySubsystem` calls `RecreatePhysicsState` on collision primitives every frame when the food-physics-dirty flag is set. This works correctly but is heavier than necessary — ideally only the specific primitives whose state changed would be updated.

## Current behavior

When the Rust dispatch returns a "food physics dirty" flag, the subsystem iterates all food ISM instances and calls `RecreatePhysicsState()` on each. At demo scale (~5000 entities) this is fine.

## Potential improvement

Track which specific instances changed and only recreate physics for those. Or batch the recreation less frequently (e.g. every N frames).

## When to address

Profile first. Only worth doing if scaling to larger entity counts and physics recreation shows up in profiling.
