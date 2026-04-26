//! Unreal backend: `Query` is a marker type recognized by `#[mass_system]`.
//!
//! In Unreal mode, `Query<&T>` and `Query<&mut T>` appear in function
//! signatures but are never constructed directly. The `#[mass_system]`
//! proc macro rewrites the function to iterate over `MassSystemChunks`
//! resources backed by C++ Mass Entity chunk memory.
//!
//! This provides source-code compatibility with the Bevy backend where
//! `Query` is `bevy_ecs::system::Query`.

use std::marker::PhantomData;

/// Marker type for Mass Entity queries in `#[mass_system]` function signatures.
///
/// The `#[mass_system]` macro recognizes this type and generates chunk-pointer
/// iteration code. The type itself is never constructed at runtime.
///
/// `D` is the data access pattern (`&T` or `&mut T`).
/// `F` is the filter (unused in Phase 1, for API compatibility with Bevy).
pub struct Query<D, F = ()> {
    _phantom: PhantomData<(D, F)>,
}

/// In Unreal mode, `BevyQuery` is a real `bevy_ecs::Query` operating on Bevy entity
/// storage. Use this for pure-Bevy components that don't cross the FFI boundary.
/// The `#[mass_system]` macro passes `BevyQuery` params through unchanged.
///
/// **Deprecated**: Use `#[bevy]` parameter attribute instead:
/// ```ignore
/// #[mass_system]
/// fn my_system(#[bevy] cooldowns: Query<(Entity, &mut Cooldown)>) { ... }
/// ```
#[deprecated(note = "Use #[bevy] parameter attribute on Query instead of BevyQuery")]
pub use bevy_ecs::system::Query as BevyQuery;
