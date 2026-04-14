# Lit mode rendering artifacts on MassRepresentation ISM entities

## Status: Open (workaround: use unlit mode)

## Symptoms

Moving entities rendered via UE's native MassRepresentation (ISM) show
visual ghosting/flickering artifacts in lit rendering mode. The entities
appear stable and correct in unlit mode.

The effect looks like a small smeared trail or jitter around the actual
entity position — as if motion blur or temporal anti-aliasing (TAA) is
using incorrect previous-frame data.

## Investigation so far

1. **Rust no longer writes any transforms** — all `&mut Transform` removed
   from Rust systems in Unreal mode. Only the C++ `URustMassMovementApplyProcessor`
   writes `FTransformFragment`.

2. **LOD processor removed** — was defaulting all entities to LOD=Off /
   CulledByDistance because our custom pipeline doesn't run the LODCollector
   that populates `FMassViewerInfoFragment`. This caused the vis processor to
   skip ISM transform updates on most frames (periodic-only update for culled
   entities). Removing it fixed the update frequency issue.

3. **PrevTransform updated each frame** — `FMassRepresentationFragment::PrevTransform`
   is now set to the current transform before applying velocity in the
   MovementApplyProcessor. This gives the renderer correct per-frame motion
   vectors. However, the artifact persists.

## Possible remaining causes

- **ISM per-instance PrevTransform not fed to renderer**: The
  `MassVisualizationComponent` feeds `PrevInstanceTransforms` to ISM components
  via `SetPreviousTransformById()`, but this data comes from a separate path
  (`FMassLODSignificanceRange::SharedData`) that may not be connected to our
  `FMassRepresentationFragment::PrevTransform`.

- **TAA jitter**: Temporal anti-aliasing adds sub-pixel camera jitter each
  frame. Without correct per-instance motion vectors, TAA blends wrong frames
  for moving objects. Disabling TAA (or using unlit) eliminates the artifact.

- **Motion blur shader**: UE's per-object motion blur uses velocity from
  current vs previous transform. If the ISM component-level previous transform
  is used instead of per-instance, all instances share the same (zero) velocity.

## Workaround

Use unlit viewport mode. Entities render correctly without lit-mode
temporal effects.

## Files involved

- `RustPlugin/Source/RustPlugin/RustMassMovementApplyProcessor.cpp` — writes transforms + PrevTransform
- `RustPlugin/Source/RustPlugin/RustMassVisualizationSetup.cpp` — configures MassRepresentation
- `RustPlugin/Source/RustPlugin/RustMassBevySubsystem.cpp` — runs vis pipeline
