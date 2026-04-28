# Movement processor history: UMassApplyMovementProcessor → UMassSimpleMovementProcessor

Archival note. Captures how the Rust sim originally drove UE's
`UMassApplyMovementProcessor` and why we switched to
`UMassSimpleMovementProcessor`. Kept because the original path is a
reasonable thing to want back (force accumulation, velocity
clamping, LOD-gated update), and because the transition between the
two is a concrete example of "test-driving UE built-ins vs. owning
the sim step".

## Original setup (UMassApplyMovementProcessor)

Engine source: `Engine/Plugins/Runtime/MassGameplay/Source/MassMovement/Private/MassMovementProcessors.cpp`.

Entity query:
- `FMassVelocityFragment` (read/write)
- `FTransformFragment` (read/write)
- `FMassDesiredMovementFragment` (read-only)
- **tag required absent**: `FMassOffLODTag`
- **tag required present**: `FMassCodeDrivenMovementTag`

Execute body: `Velocity.Value = DesiredMovement.DesiredVelocity;`
then `Transform += Velocity * dt`.

The Rust side authored `DesiredMovement.DesiredVelocity` each frame
(a `DVec3` magnitude = speed in units/s). Archetype composition
in `gatherers-bevy-mass/src/init.rs::spawn_entities`:

```rust
.fragment::<Transform>()
.fragment::<PreviousTranslation>()
.fragment::<DesiredMovement>()   // -> FMassDesiredMovementFragment
.fragment::<Velocity>()          // -> FMassVelocityFragment (engine writes it)
.fragment::<Behavior>()
.tag::<Ant>()
.tag::<CodeDrivenMovementTag>()  // -> FMassCodeDrivenMovementTag (required)
```

C++ pipeline order in `RustMassBevySubsystem::EnsureProcessors`:

```
ScheduleCoordinator (runs Rust systems via URustMassDynamicProcessor)
URustMassPostMovementProcessor      // save PreviousTranslation + clamp bounds
UMassApplyMovementProcessor          // DesiredMovement -> Velocity -> Transform
```

The name "PostMovement" is historical — in an earlier iteration it
ran *after* movement. By the time we landed this shape it was
pre-movement (captures `PreviousTranslation` before the native
processor updates `Transform`).

### Why this was appealing

- **Uses a UE built-in.** One of the stated goals of the bridge is
  to test-drive interop with existing Mass Entity systems; consuming
  `UMassApplyMovementProcessor` directly was the minimum viable
  proof that chunk-layout-compatible Rust fragments work with stock
  engine processors.
- **Semantic split between "desired" and "actual" velocity.** The
  Rust sim says "I want to move with velocity V"; the engine decides
  how to realize that. Leaves room to add
  `UMassApplyForceProcessor` into the chain (forces accumulate into
  `FMassForceFragment` → integrated into `DesiredVelocity` →
  integrated into `Velocity` → integrated into `Transform`).

### Why we dropped it

- **`FMassOffLODTag` gating.** `UMassApplyMovementProcessor` skips
  every entity tagged off-LOD. UE's
  `UMassVisualizationLODProcessor` (which we enable for ISMC render
  tiers) assigns that tag to distance-culled entities. Result: in
  PIE at 1k ants ~1% of ants were stationary or moving at 1/10
  speed, even though `DesiredVelocity.Size() == 100` was correct —
  the movement processor was declining the chunk.
- **The sim should not be gated on the renderer's LOD.** Visual LOD
  is a presentation concern. Sim correctness is not negotiable. The
  engine offers a non-LOD-gated variant specifically for this
  ("logic runs for everyone; visualization decides what to draw").
  See
  `Engine/Plugins/Runtime/MassGameplay/Source/MassMovement/Private/MassSimpleMovementTrait.cpp::UMassSimpleMovementProcessor`.

Diagnostic test that catches the regression:
`RustPluginTests/Private/RustMassGatherersPIE.spec.cpp::FRustPIESlowAntsTest`
— runs in PIE (required, since direct `Subsystem->Tick()` doesn't
execute UE's LOD processors), measures per-ant displacement,
asserts no ant is stationary and none moves below 70% of nominal.

## Current setup (UMassSimpleMovementProcessor)

Entity query:
- `FMassVelocityFragment` (read-only)
- `FTransformFragment` (read/write)
- **tag required present**: `FMassSimpleMovementTag`
- **no `FMassOffLODTag` requirement** — runs for all tagged
  entities regardless of visual LOD

Execute body: `Transform += Velocity * dt`. That's it.

### What changed

- Rust side authors `FMassVelocityFragment.Value` directly
  (same `DVec3` semantics, magnitude = units/s) instead of
  `FMassDesiredMovementFragment.DesiredVelocity`. Both backends
  share the authoring channel: game systems mutate `Velocity`,
  `apply_movement` (Bevy mode) or `UMassSimpleMovementProcessor`
  (Unreal mode) integrates it into `Transform`.
- Archetype composition: `DesiredMovement` and
  `CodeDrivenMovementTag` are deleted from the framework
  (`bevy_mass::components`) — nothing depends on them anymore.
  Replaced with `SimpleMovementTag` → `FMassSimpleMovementTag`.
  If force composition becomes load-bearing later, re-introduce
  `DesiredMovement` + a narrow force processor (see follow-ups).
- C++ pipeline: `UMassApplyMovementProcessor` replaced with
  `UMassSimpleMovementProcessor`. Ordering unchanged.

### What we keep

- **Visualization LOD tiers still work.** Our archetype retains
  `FMassVisualizationLODProcessorTag`, and the LOD collector +
  visualization LOD processors continue to run. ISMC render-tier
  selection is unchanged. Only the sim-logic side is decoupled
  from the LOD tag.
- **Test-drives a UE built-in.** We swapped one stock Mass
  Entity processor for another — still consuming engine code,
  still chunk-layout-compatible. The engine's deliberate split
  between "apply with LOD gating + force composition"
  (`UMassApplyMovementProcessor`) and "just integrate"
  (`UMassSimpleMovementProcessor`) is exactly what we needed.

## Bevy standalone: how it moves without either processor

`gatherers-standalone` and unit tests run on pure Bevy with no
Mass processors at all. The framework provides the equivalent in
`bevy_mass/src/movement.rs::apply_movement`:

```rust
#[cfg(not(feature = "unreal"))]
pub fn apply_movement<T, P, V>(
    mut entities: Query<(&mut T, &mut P, &V)>,
    time: Res<Time>,
) where T: TransformLike, P: PrevTranslationLike, V: VelocityLike {
    for (mut transform, mut prev, velocity) in &mut entities {
        prev.set_prev(transform.translation());
        let vel = velocity.velocity();
        let speed = vel.length();
        if speed < 1e-8 { continue; }
        let step = speed * dt;
        transform.set_translation(transform.translation() + vel / speed * step);
    }
}
```

`MovementPlugin<T, P, V>` wires this into `Update` in Bevy mode
and is a no-op in Unreal mode (the C++ processor handles it).

Dual-mode symmetry: game code writes to `Velocity`, and the
framework applies it to `Transform` — via `apply_movement` in
Bevy mode, via `UMassSimpleMovementProcessor` in Unreal mode.
The `VelocityLike` trait is the contract both modes share.

## UE hack: `UMassRandomVelocityInitializer` corrupts spawn-time velocity

The B1 swap (→ `UMassSimpleMovementProcessor`) surfaced a second
engine quirk unrelated to LOD gating. Noting it here so we can
check whether future UE versions remove it — if they do, the
corresponding workaround in
`RustPlugin/Source/RustPlugin/RustUtils.cpp` (the
`SetEntityFragmentValues` call after `CreateEntity`) becomes
deletable.

### What it is

`UMassRandomVelocityInitializer` lives at
`Engine/Plugins/Runtime/MassGameplay/Source/MassMovement/Public/Example/MassVelocityRandomizerTrait.h`
and is declared as a `UMassObserverProcessor`. Its constructor
sets:

```cpp
ObservedType = FMassVelocityFragment::StaticStruct();
ObservedOperations = EMassObservedOperationFlags::Add;
```

`UMassObserverProcessor::bAutoRegisterWithObserverRegistry`
defaults to `true` (see
`Engine/Source/Runtime/MassEntity/Public/MassObserverProcessor.h`),
so this class auto-registers **engine-wide** as an `Add` observer
on `FMassVelocityFragment`. It fires for every entity that ever
adds that fragment, regardless of trait composition.

### What it does

At `MassVelocityRandomizerTrait.cpp:41-58` the observer reinterprets
the freshly-written velocity vector as a parameter tuple:

```cpp
// VelocityFragment.Value.X encodes MinSpeed
// VelocityFragment.Value.Y encodes MaxSpeed
// VelocityFragment.Value.Z encodes bSetZComponent (boolean as float)
VelocityFragment.Value = RandomVector
    * RandomStream.FRandRange(VelocityFragment.Value.X, VelocityFragment.Value.Y);
```

The engine author is aware — lines 18-19 of
`MassVelocityRandomizerTrait.cpp` literally call it a `@hack` and
note: *"A proper solution will allow users to specify a 'lambda'
initializer that will be used during creation"*. The partner
trait `UMassVelocityRandomizerTrait::BuildTemplate` is the
intended producer — it stuffs `MinSpeed/MaxSpeed/bSetZComponent`
into the vector's X/Y/Z slots at template-build time, then this
observer decodes them.

### Why it broke us

When `RustMassSpawn::SpawnOneEntity` calls
`EntityManager->CreateEntity(Fragments)` with a real velocity
(`direction * speed`) in `FMassVelocityFragment`, the observer
fires immediately after the fragment lands in chunk memory and
overwrites the authored value with a random vector scaled by
`FRandRange(X, Y)` — where X and Y are our direction components
interpreted as speeds. Direction and speed are both destroyed.

This was invisible before the B1 swap because the old
`UMassApplyMovementProcessor` skipped off-LOD entities; most ants
never moved, so nobody noticed their velocity had been clobbered
at spawn. `UMassSimpleMovementProcessor` runs on every entity,
immediately made this a visible regression, and was caught by
`FRustPIESlowAntsTest` (ants moving at non-uniform speeds).

### The workaround

`RustMassSpawn::SpawnOneEntity` follows `CreateEntity` with
`EntityManager->SetEntityFragmentValues(Handle, Fragments)`. That
path writes straight into chunk memory **without re-firing
observers**, so our authored velocity survives.

### Check for this when bumping UE versions

If a future UE release:
- Marks `UMassRandomVelocityInitializer` `bAutoRegisterWithObserverRegistry
  = false` (scopes it to the trait that intentionally opts in), or
- Replaces the fragment-slot hack with the proper "lambda
  initializer" approach the engine comment foreshadows, or
- Deletes the observer entirely (it lives under `Example/`),

then the `SetEntityFragmentValues` call in `RustUtils.cpp` can be
removed. A good canary is whether `FRustPIESlowAntsTest` still
passes without it.

## Follow-ups

- **Return of forces.** If force accumulation becomes load-bearing
  (steering, avoidance, per-agent behavior layered), the right
  path is probably a narrow `URustMassForceProcessor`: consume
  `DesiredMovement + Force → Velocity` (clamped, no LOD gate), so
  `UMassSimpleMovementProcessor` keeps doing the position
  integration. That preserves the "desired vs actual velocity"
  semantic split while keeping movement un-gated by LOD.
- **Variable-tick.** `UMassSimpleMovementProcessor` already has
  optional support for `FMassSimulationVariableTickFragment` +
  `FMassSimulationVariableTickChunkFragment`. We don't use it
  today (every chunk ticks at the full rate). If sim cost becomes
  a bottleneck at scale, this is the lever — distant entities can
  tick at a lower rate without the movement processor skipping
  them entirely.
