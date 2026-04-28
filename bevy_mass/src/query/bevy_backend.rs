//! Bevy backend: re-export real `bevy_ecs::system::Query`.
//!
//! No wrapper needed — systems are standard Bevy systems operating on
//! real entities and components.

pub use bevy_ecs::system::Query;

/// In Bevy mode, `BevyQuery` is identical to `Query` — both are real Bevy queries.
/// In Unreal mode, `BevyQuery` stays as a real Bevy query while `Query` is rewritten
/// by the `#[mass_system]` macro to access chunk memory.
///
/// **Deprecated**: Use `#[bevy]` parameter attribute instead:
/// ```ignore
/// #[mass_system]
/// fn my_system(#[bevy] cooldowns: Query<(Entity, &mut Cooldown)>) { ... }
/// ```
#[deprecated(note = "Use #[bevy] parameter attribute on Query instead of BevyQuery")]
pub use bevy_ecs::system::Query as BevyQuery;
