# Implicit auto-lift of shadow components into chunk-backed primary tuples

Attempted and reverted. This note captures what the **implicit**
flavour of auto-lift tried to do, why it failed, and what a future
attempt — specifically an **explicit** marker-based variant — would
need. The two variants are related in motivation but structurally
different; don't mentally group them.

## What was attempted (implicit variant)

Let `#[mass_system]` authors write a primary `Query<(...)>` that mixes
chunk-backed `MassFragment` components (e.g. `Transform`,
`DesiredMovement`, `Behavior`) with pure-Rust shadow-only components
(e.g. `Carrying`) **without any author annotation**, and have the
macro auto-synthesize the shadow-world fetch — removing the boilerplate
of a separate `#[bevy]` passthrough `Query<&mut Carrying>` +
`.get_mut(entity)` inside the loop body.

The "implicit" label matters: the macro had to guess, from syntax
alone, which tuple slots referred to chunk-backed vs shadow-only
components. That guess is what fails (see below).

Target shape:

```rust
#[mass_system]
fn food_decision_system(
    ants: Query<(Entity, &mut Transform, &mut DesiredMovement,
                 &mut Behavior, &mut Carrying),
                (With<Ant>, Without<Cooldown>)>,
    // ...no more `#[bevy] carrying_q`, no more `.get_mut(entity)` inside
) { ... }
```

Planned in `/plans/lazy-munching-sundae.md` (archived on the plan), five
staged steps. Steps 1–4 (framework/macro changes) landed on commit
`1758f16`; step 5 (consumer migration) was then developed and the UE
automation hung. The whole thing was reverted; the `#[bevy]` passthrough
shape for `Carrying` remains the status quo.

## Why it failed

The macro cannot syntactically tell chunk-backed `&mut T` apart from
shadow-only `&mut T`. Our runtime dispatch uses
`<T as QueryBackend>::IS_CHUNK` as a const-if that the compiler
monomorphizes away — that part works.

But Bevy's scheduler does **access analysis** on the *system signature*
before monomorphization. The attempted emission, for every fragment in
the primary tuple, synthesized a wrapper
`__shadow_N: Query<'_, '_, &mut T>` to source the shadow fetch. For a
tuple mixing chunk + shadow fragments the emitted signature looked like:

```rust
fn food_decision_system_bevy(
    __bevy_ants: Query<(Entity, &mut Transform, &mut DesiredMovement,
                        &mut Behavior), ...>,
    __shadow_0:  Query<&mut Transform>,       // B0001 with __bevy_ants
    __shadow_1:  Query<&mut DesiredMovement>, // B0001
    __shadow_2:  Query<&mut Behavior>,        // B0001
    mut carrying_q: Query<&mut Carrying>,     // author's #[bevy]
    // ...
)
```

Bevy sees four mutable aliases of `Transform` / `DesiredMovement` /
`Behavior` per entity across sibling `Query` params and refuses to run
the system (error B0001). The whole schedule then fails to build
("Main schedule not found" cascade), exit code -1.

This wasn't caught before committing because `cargo test` (Rust unit
tests only) passed — the B0001 is surfaced only when Bevy actually
builds the schedule in UE runtime. UE automation was not re-run for
commit `1758f16`.

## Why the dispatch can't be fixed in-place

- Runtime const-if selects the right data source per fragment, but the
  wrapper `Query<&mut T>` has to exist in the signature regardless so
  the shadow code path has somewhere to read from. Its presence is
  what trips B0001.
- A per-fragment `ParamSet` wrapping only the shadow queries wouldn't
  help — the chunk-backed fragments are already mutably aliased by
  `__bevy_ants`, and the shadow wrappers would need to be inside the
  same `ParamSet` to be disjoint. That partitioning is exactly what
  the macro can't do from syntax alone.
- Omitting the shadow wrappers when IS_CHUNK is true isn't possible
  from the macro either — `IS_CHUNK` is a trait-associated const,
  only evaluable at monomorphization, and the wrapper must be declared
  in the signature.

## What a future attempt would need

Any one of these unlocks the pattern:

1. **Explicit auto-lift via an author-visible marker inside the
   tuple**: e.g. `Query<(Entity, &mut Transform,
   #[shadow] &mut Carrying, ...)>`. The macro partitions at
   classification time by the explicit marker — no guessing — and
   emits only the shadow wrappers it actually needs. Small syntactic
   cost to the author; zero scheduler risk. This is a **different
   design from the reverted implicit variant**; the failure above
   does not carry over.
2. **A trait-based `IS_CHUNK` that Bevy's scheduler can see**
   (i.e., a const-generic `Query` or similar). Today Bevy's access
   graph is type-level only; adding const-level filtering is an
   upstream bevy_ecs change.
3. **Keep status quo**: the `#[bevy] shadow_q: Query<&T>` +
   `.get(entity)` pattern inside the loop is ~4 lines of scaffolding
   per system. Acceptable for a small handful of systems;
   self-documenting (reader sees "shadow lookup" at the call site).

Option 1 (explicit auto-lift) is the realistic path if this is
revisited. Option 3 is the current state.

## Why it's not worth re-attempting now

Applies specifically to option 1 (explicit auto-lift via `#[shadow]`).
Option 2 depends on upstream bevy_ecs changes and isn't ours to drive.

- Only two consumers would benefit: `food_decision_system` and
  `carried_food_tracking`. Each would save ~4 lines.
- The `#[bevy]` pattern is understood and documented; authors reading
  existing sim code see the shape immediately.
- Implementing explicit auto-lift is still several hundred lines of
  macro work for a modest ergonomic win. Revisit only when either:
  - There are ≥5 systems paying the `#[bevy]` tax, **or**
  - A new shadow component lands that would be naturally mixed into
    an existing chunk-backed tuple (not just a parallel lookup).

## References

- Reverted commit: `ce9ba22 Revert "feat: auto-lift shadow components in primary query tuples"`
- Original attempt: `1758f16 feat: auto-lift shadow components in primary query tuples`
- Diagnosis was via `cargo expand -p gatherers-sim --lib --features unreal`
  — the emitted `__shadow_N` wrapper params next to `__bevy_ants`
  are what B0001 fires on. Faster than an LLDB session on the UE
  editor since the panic is Rust-side (bevy_ecs schedule build).
