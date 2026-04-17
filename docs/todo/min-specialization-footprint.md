# Reduce min_specialization footprint

The dual-mode dispatch relies on `#![feature(min_specialization)]` for the `MaybeFragment` trait — compile-time detection of whether a type has a C++ MassFragment representation. This is the only use of specialization in the codebase.

## Current state

`MaybeFragment` has a blanket impl (`IS_FRAGMENT = false`) and a specialized impl for `T: MassFragment` (`IS_FRAGMENT = true`). The `#[mass_system]` macro uses `MaybeFragment::IS_FRAGMENT` in const-if guards to skip chunk operations for Bevy-only types.

## When to address

When Rust stabilizes specialization (or a subset sufficient for this pattern). At that point, review whether the same effect can be achieved with stable features (e.g. const trait methods, marker trait bounds, or autoref specialization).
