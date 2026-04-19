# Parallelism across archetypes — component design guidance

A design note for authoring simulations with multiple entity types (ants, birds, NPCs, projectiles…) that share mechanics but need different per-type logic. Target: UE Mass Entity execution perf, Bevy-like authoring ergonomics, no hidden serialization.

## The scheduler conflict model

Bevy's scheduler detects conflicts at **component-type granularity**, not per-archetype. Two systems that both claim `&mut Velocity`, even with disjoint `With<Ant>` / `With<Bird>` filters over non-overlapping entity sets, are serialized by the scheduler. Filters narrow iteration at runtime; they do not narrow the system's declared access set.

UE Mass is stricter by construction: a processor targets a specific fragment set over specific archetypes, and archetypes are physically disjoint chunk storage, so cross-archetype parallelism is trivially safe.

This library sits between the two. Authoring should feel Bevy-native; execution on UE Mass should get archetype-parallel perf without users thinking about it.

## What parallelizes in Bevy today

1. **Within a single system, `par_iter_mut`** — rows of that query are fanned across Rayon threads. Spans multiple archetypes contiguously; no conflict because there's only one accessor.
2. **Across systems with non-overlapping component access** — the scheduler runs them concurrently.
3. **Across systems with overlapping access (even if entity sets are disjoint at runtime)** — serialized. This is the subtle one.

## What does not parallelize

- Two systems writing `&mut Velocity`, one `With<Ant>`, one `With<Bird>`. Same component, scheduler serializes.
- Reads of a component and writes of the same component in different systems. Serialized.
- `ParamSet` resolves conflicts *within* one system, not across systems.

Archetype-level conflict detection is not on Bevy's roadmap as of 2026-04; the cost of tracking it has historically been considered too high for the payoff in typical apps.

## Design pattern: split the writers, unify the consumer

When movement-decision systems (AI, steering, flocking, pathfinding) are the heavy lifters and should run in parallel across entity types, split **the component they write**:

```rust
#[derive(Component)] struct AntVelocity(Vec3);
#[derive(Component)] struct BirdVelocity(Vec3);

// These run in parallel — disjoint component access.
fn ant_movement(mut q: Query<(&Transform, &mut AntVelocity, /* inputs */)>) { ... }
fn bird_movement(mut q: Query<(&Transform, &mut BirdVelocity, /* inputs */)>) { ... }

// Unified integrator registered per type. Each monomorphization is a
// distinct system from the scheduler's perspective, so the two integrator
// instances can also run in parallel, and each uses par_iter_mut internally.
trait VelocityComponent: Component {
    fn get(&self) -> Vec3;
}
impl VelocityComponent for AntVelocity { fn get(&self) -> Vec3 { self.0 } }
impl VelocityComponent for BirdVelocity { fn get(&self) -> Vec3 { self.0 } }

fn apply_velocity<V: VelocityComponent>(
    mut q: Query<(&mut Transform, &V)>,
    time: Res<Time>,
) {
    q.par_iter_mut().for_each(|(mut t, v)| {
        t.translation += v.get() * time.delta_secs();
    });
}

app.add_systems(Update, (
    apply_velocity::<AntVelocity>,
    apply_velocity::<BirdVelocity>,
));
```

Properties:

- **Writers parallelize** — disjoint `&mut AntVelocity` / `&mut BirdVelocity` access.
- **Integrators parallelize with each other** — distinct monomorphizations, distinct component access.
- **Locality preserved** — each system streams its target archetype contiguously. ECS archetype storage is column-wise per archetype, so `(Transform, AntVelocity)` in the Ant table sits side-by-side, same for Birds. Two contiguous passes over N/2 entities each are not slower than one over N.
- **No branching, no runtime dispatch** — the monomorphized integrator is the same code a hand-written `Vec<Ant>` loop would produce.

## When to collapse instead of split

Keep one component when:

- Every entity type integrates the same way and the interesting per-type logic is *upstream* (input to the writer, not the writer itself). Then a single `DesiredVelocity` + single `par_iter_mut` integrator system is correct — one accessor, no conflict, full parallelism within.
- The writers are cheap or infrequent; cross-type writer parallelism isn't worth the component-type fragmentation cost.
- Consumers that want "any mover's velocity" (debug draw, network replication, save/load) would outnumber the writers and each would need per-type enumeration.

The rule of thumb: **split the component that causes scheduler conflict, keep everything else shared.**

## Don't split reflexively

`Intent` separate from `Velocity` makes sense only when there's a real transformation step between them — steering blends, speed clamping, obstacle avoidance, physics-driven overrides. If the AI system already outputs final velocity and the integrator just does `translation += v * dt`, one component is right. Don't introduce redundant layers to "feel" more decoupled.

## UE Mass mapping

On UE Mass this pattern lands well:

- Each split velocity component is its own `MassFragment`. Processors target fragment sets; a processor reading `AntVelocity` touches only ant archetypes, another reading `BirdVelocity` touches only bird archetypes. UE's parallel processor scheduling is a natural fit.
- Chunk iteration stays zero-copy per archetype. See `dual-mode-dispatch-remaining-shapes.md` for the invariant that `DualQueryMut` must preserve.
- The generic integrator pattern (`apply_velocity<V>`) registers N concrete processors on the UE side, one per monomorphization. That's what Mass wants anyway — UE processors are typed at registration time.

The library should make both the shared-component single-integrator case **and** the split-component-generic-integrator case straightforward. Anything that forces a choice between "Bevy-ergonomic" and "Mass-parallel" is a gap to close.

## Open questions for the library

- Does `#[mass_system]` emit correctly for a generic system registered per type? Test with `apply_velocity<AntVelocity>` / `apply_velocity<BirdVelocity>` and verify two distinct C++ processors are produced.
- For the split-writer pattern, can the DualQuery const-if path be exercised end-to-end with two archetypes, two writer systems, one typed-integrator-per-archetype? Unit test gap.
- The "trait-based generic component consumer" pattern above — is there friction in the current macro when `V: VelocityComponent` bounds appear in a `Query<&V>` param? Phase 1 handled concrete types; generics-over-`MassFragment` is worth validating explicitly.

These should graduate from "open questions" to tests before we recommend the pattern for game code.
