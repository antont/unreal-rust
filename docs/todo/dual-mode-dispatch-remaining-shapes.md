# Dual-mode dispatch — remaining query shapes

Step B Phase 1 (`23f789f`) wired `DualQueryRef` / `DualQueryMut` and
const-if dispatch so **single-component** `Query<&T>` / `Query<&mut T>`
in `#[mass_system]` auto-selects chunk iteration vs Bevy storage at
monomorphization, based on whether `T: ChunkBacked`. No `#[bevy]`
annotation needed.

Follow-on work extended that to tuple queries and `With` / `Without`
filters. By the time this doc was revisited (2026-04-23), tuple support
had already landed but this page hadn't been updated to match.

## Actual state (audited 2026-04-23)

| Shape | State |
|---|---|
| `Query<&T>`, `Query<&mut T>` | ✅ Auto-dispatches chunk vs Bevy |
| `Query<(&A, &B)>` — both chunk-backed | ✅ Works |
| `Query<(&mut A, &B)>` — mixed R/W, both chunk-backed | ✅ Works |
| `Query<(Entity, &mut T)>` — pure-Bevy `T` | ✅ Works |
| `Query<&T, With<U>>` | ✅ Works (tag → C++ archetype requirement) |
| `Query<&T, Without<U>>` | ✅ Works (per-entity Bevy mask) |
| Tuple + filter combinations | ✅ Works (see `entity_boundary_reflect` etc.) |
| `Query<(&A, &B)>` — **one chunk, one pure-Bevy** | ❌ Blocked by consistency guard (intentional) |
| `Query<&T, Or<(With<A>, With<B>)>>` | ❌ Not implemented |
| `Query<&T, Added<T>>` / `Changed<T>` | ❌ No chunk analogue, deferred |

Evidence: every `#[bevy]` annotation has been removed from the gatherers
crates. `entity_cooldown` (`gatherers-sim/src/movement.rs:30`) uses
`Query<(Entity, &mut Cooldown)>` with `Cooldown` as a pure-Bevy
component — dispatched via const-if, no annotation. Tuple-plus-filter
shapes are live in `gatherers-bevy-mass/src/systems.rs` (search
`(With<`, `Without<`).

## Genuinely remaining gaps (not urgent)

### `Or<(With<A>, With<B>)>` filter predicates

No macro handling for `Or`. `With` / `Without` are parsed by
`extract_filters` (`unreal-api-derive/src/mass_system.rs:369`) but an
`Or` arm would need its own branch and a different codegen strategy —
chunk-side it maps to an archetype-union requirement (no direct Mass
equivalent; likely falls back to Bevy-style per-entity evaluation), and
Bevy-side it delegates to `bevy_ecs::query::Or`. No game system asks
for this today.

### Mixed chunk/Bevy primary queries

`chunk_consistency_assert` at `unreal-api-derive/src/mass_system.rs:768`
fails compile if one `#[mass_system]` primary query mixes a chunk-backed
and a Bevy-only fragment. This is deliberate: the chunk loop runs N
times (once per chunk) while the pre-collected Bevy data is a flat list
— re-iterating it per chunk would double-mutate. Lifting this would
need a "Bevy slice takes turns with chunks" dispatch model that doesn't
exist. Workaround (split into two systems) is fine for now.

### `Added<T>` / `Changed<T>` change-detection filters

UE Mass has no per-fragment mutation tracking. Options if a future
system needs this: (a) force `#[bevy]` for that system, (b) synthesize
change detection by staging writes through a wrapper, (c) document as
unsupported in chunk mode. No concrete demand.

## Invariant: zero-copy writes into UE chunks

Phase 1's `DualQueryMut::Chunk(&mut [T])` aliases UE Mass chunk memory
directly — writes from a `#[mass_system]` land in the chunk with no
copy-out / copy-back step. This is load-bearing for "author in Bevy,
run in UE at UE perf". If any of the above gaps is ever closed:

- **Do not** introduce intermediate `Vec<T>` / `Box<[T]>` staging on
  the write path for the chunk side.
- Mixed tuples where one element is chunk-backed and another is
  Bevy-only: the chunk slot keeps its direct slice; the Bevy slot goes
  through the pointer-vec path. No copying either direction.
- When in doubt, a test that writes a sentinel value through
  `DualQueryMut` and reads it back on the C++ side (see the existing
  `BevyMass*FragmentLayout` / `BevyMassFoodPickup` patterns) will catch
  a regression.

If a future shape genuinely can't stay zero-copy (e.g. `Query<&mut T,
Added<T>>` needing change-detection bookkeeping Mass doesn't provide),
document the opt-in and require an explicit annotation — don't silently
regress other shapes with shared staging infrastructure.

## Priority

None of these are urgent:

- **1k-ant target is met** (see
  `docs/issues/sim-cost-superlinear-at-scale.md` — full editor frame
  hits the 60 Hz v-sync cap at 1k, `RustMass_Tick` ~1 ms). There is no
  scale-driven pressure to squeeze more code onto the chunk path.
- **No game system currently needs the missing shapes**. `Or` could be
  rewritten as two separate `With` queries; mixed storage can be split
  into two systems; `Added`/`Changed` has no caller.

Revisit if (a) a concrete system wants one of these shapes, or (b) the
scale target changes and pushing more work onto the chunk fast path
becomes worthwhile.
