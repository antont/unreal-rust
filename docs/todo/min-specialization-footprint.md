# Reduce min_specialization footprint

The dual-mode dispatch relies on `#![feature(min_specialization)]` for the `MaybeFragment` and `QueryBackend` traits — compile-time detection of whether a type has a C++ MassFragment representation. This is the only use of specialization in the codebase.

## Current state

`MaybeFragment` has a blanket impl (`IS_FRAGMENT = false`) and a specialized impl for `T: MassFragment` (`IS_FRAGMENT = true`). `QueryBackend` has the same shape keyed on `ChunkBacked`. The `#[mass_system]` macro uses `QueryBackend::IS_CHUNK` / `MaybeFragment::IS_FRAGMENT` in const-if guards to dispatch to chunk iteration or Bevy storage at monomorphization.

This was a deliberate design choice to keep game-code systems idiomatic Bevy (no `ChunkBacked` opt-in annotation on user types, no wrapper helpers at call sites). Moving off `min_specialization` would regress that — users would have to explicitly mark types, or call-sites would need autoref/wrapper gymnastics.

## Safety of our usage

Our specialization usage is on the clearly-sound side of `min_specialization`'s known-unsound edges:

- **All `'static` types.** `ChunkBacked` implies `'static`; `MassFragment: Sized + Copy + 'static`. The soundness holes in specialization are lifetime-parametric (impls that overlap only via different lifetime choices). We don't have that shape.
- **Only associated `const` items.** No `default fn` with references or `&mut`. Specialization's subtle bugs come from methods where a specialized impl observes or mutates lifetimes the default impl promised to treat generically. Constants can't.
- **No lifetime parameters in impls.** `impl<T>` vs `impl<T: ChunkBacked>` — overlap resolved solely by whether `T: ChunkBacked`, not by lifetime choice.
- **Monomorphized const-if only.** `if <T as QueryBackend>::IS_CHUNK { ... } else { ... }` — the compiler eliminates the dead branch at codegen. No `dyn`, no v-tables.

This pattern mirrors stdlib's own `min_specialization` usage (e.g. `TrustedLen`-style marker + associated const, all `'static`).

## Stabilization outlook

`min_specialization` is not expected to stabilize for user crates on any known timeline:

- Tracking issue [rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844) has been open since 2016.
- Full specialization has unresolved soundness issues around lifetime parametricity.
- `min_specialization` was introduced (2018) as a safer subset **for internal stdlib use**, explicitly not positioned for end-user stabilization.
- Stable rustc and stdlib both use it internally (via `rustc_specialization_trait` / `rustc_unsafe_specialization_marker`), but end-user crates on stable cannot enable `#![feature(min_specialization)]`.
- No active RFC pushing toward end-user stabilization as of early 2026.

The "some day" in the original version of this doc was effectively "never, for planning purposes."

## When to address

Only if one of the following happens:

- `min_specialization` (or a sufficient stable subset like const trait methods with overlap resolution) stabilizes for user crates.
- We hit a concrete soundness issue in our specific usage — unlikely given the constraints above, but the trigger for re-evaluation.
- The cost of requiring nightly becomes a material blocker (e.g. a dependency of this project is dropped from nightly, CI/tooling breakage, contributor friction escalates past the idiomatic-Bevy win).

None of those currently apply. Requiring nightly is the deliberate cost of keeping game-code systems idiomatic Bevy.

## If we ever do move off it

The alternatives are all worse for the idiomatic-Bevy goal:

- **Explicit marker-trait bound on users.** Users would write `#[derive(ChunkBacked)]` or equivalent. Breaks the "user types look like vanilla Bevy" property.
- **Autoref / wrapper trick.** `Kind<T>` newtype where inherent method resolution beats trait method resolution — works on stable but clutters call sites in the macro expansion and is harder to reason about.
- **Macro-emitted opt-in.** `#[mass_system]` could require a list of `#[chunk] T` parameter annotations. Regresses to pre-Step-B ergonomics.

None of these are appealing enough to preemptively adopt. Revisit only when the trigger conditions above apply.
