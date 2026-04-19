# Dual-mode dispatch — remaining query shapes

Step B Phase 1 (`23f789f`) wired DualQueryRef/DualQueryMut and const-if dispatch so **single-component** `Query<&T>` / `Query<&mut T>` in `#[mass_system]` auto-selects chunk iteration vs Bevy storage at monomorphization, based on whether `T: MassFragment`. No `#[bevy]` annotation needed for single-component queries.

Phase 2 and beyond — everything not yet covered still requires `#[bevy]` on the param, which forces the whole system to Bevy-only iteration (no chunk path).

## Shapes not yet covered

### Tuple reads
`Query<(&A, &B)>`, `Query<(&A, &B, &C)>`. Needs a `DualQueryRefTuple2<A, B>` / 3-ary etc. plus macro recognition. Mixed `MassFragment`/non-fragment tuples need to decide per-element whether to pull from chunk or Bevy.

### Mixed read/write tuples
`Query<(&A, &mut B)>`. Same as above but with a `DualQueryMut` slot. Care needed around iterator aliasing — chunk iteration returns `(&A, &mut B)` by slicing the chunk at a single index; Bevy path is pointer-based.

### Filter predicates
`Query<&T, With<U>>`, `Query<&T, Without<U>>`, `Query<&T, Or<(With<A>, With<B>)>>`. Chunk path can evaluate filters at chunk-registration time (whole-chunk include/exclude). Bevy path delegates to `bevy_ecs`'s filter machinery. Single-bit filters are easy; tuple filters need thought.

### Added/Changed
`Query<&T, Added<T>>`, `Query<&T, Changed<T>>`. Chunk path has no change detection analogue (UE Mass doesn't track per-fragment mutation state). Options: (a) force `#[bevy]`, (b) synthesize change detection by staging writes through a wrapper, (c) document as unsupported-in-chunk-mode.

### Resources alongside queries
Works today — `Res<R>` / `ResMut<R>` don't care which path the query takes. Only mentioned here so it's explicit: they're not a gap, just the sanity check.

### Disjoint queries in one system
`fn sys(a: Query<&A>, b: Query<&B>)` with `A`/`B` potentially one fragment and one Bevy-only. Each query dispatches independently; current Phase 1 already supports this for single queries. Verify with a test when tuples land.

## Why this matters

Each `#[bevy]` annotation is a silent perf regression in UE mode — the system skips chunk iteration entirely and round-trips through Bevy storage. For tuple queries today that's unavoidable. Closing these gaps is what lets game code stay in idiomatic `Query<(&A, &mut B)>` shape without losing the chunk fast-path.

## Invariant: zero-copy writes into UE chunks

Phase 1's `DualQueryMut::Chunk(&mut [T])` aliases UE Mass chunk memory directly — writes from a `#[mass_system]` land in the chunk with no copy-out / copy-back step. This is load-bearing for the "author in Bevy, run in UE at UE perf" promise. When extending to tuples / filters / mixed read-write:

- **Do not** introduce intermediate `Vec<T>` / `Box<[T]>` staging on the write path.
- Tuple writes (`Query<(&A, &mut B)>`) must still hand out `&mut T` aliased to chunk memory for each chunked component; non-chunked tuple slots are the ones that fall back.
- Mixed tuples where one element is chunk-backed and another is Bevy-only: the chunk slot keeps its direct slice; the Bevy slot goes through the pointer-vec path. No copying either direction.
- When in doubt, a test that writes a sentinel value through `DualQueryMut` and reads it back on the C++ side via the existing `BevyMass*FragmentLayout` / `BevyMassFoodPickup` patterns will catch a regression.

If a future shape genuinely can't stay zero-copy (e.g. a hypothetical `Query<&mut T, Added<T>>` that needs change-detection bookkeeping Mass doesn't provide), document the opt-in and require an explicit annotation — don't silently regress the other shapes with shared staging infrastructure.

## Ordering suggestion

1. Tuple reads (2-ary, then 3-ary). Covers most real game-code shapes.
2. Mixed read/write tuples. Unlocks the common "read one thing, write another" pattern.
3. `With` / `Without` filter predicates. Pure wins, low risk.
4. Defer `Added`/`Changed` until a concrete system needs them.

Each step is its own `#[mass_system]` macro upgrade + a `DualQuery*` shape in `unreal-api/src/mass.rs` + unit tests + a working game-code example moved off `#[bevy]`.
