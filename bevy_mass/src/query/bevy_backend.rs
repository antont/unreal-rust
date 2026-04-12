//! Bevy backend: re-export real `bevy_ecs::system::Query`.
//!
//! No wrapper needed — systems are standard Bevy systems operating on
//! real entities and components.

pub use bevy_ecs::system::Query;
