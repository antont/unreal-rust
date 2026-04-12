//! Bevy backend: re-export real `bevy_ecs::system::Query`.
//!
//! No wrapper needed — systems are standard Bevy systems operating on
//! real entities and components.

pub use bevy_ecs::system::Query;

/// In Bevy mode, `BevyQuery` is identical to `Query` — both are real Bevy queries.
/// In Unreal mode, `BevyQuery` stays as a real Bevy query while `Query` is rewritten
/// by the `#[mass_system]` macro to access chunk memory.
pub use bevy_ecs::system::Query as BevyQuery;
