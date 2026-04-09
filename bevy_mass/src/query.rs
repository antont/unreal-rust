//! Facade `Query` type that provides Bevy-compatible iteration over entity data.
//!
//! In Bevy mode: thin wrapper around `bevy_ecs::system::Query`.
//! In Unreal mode: iterates over Mass Entity chunk data via `MassSystemChunks`.

// Unreal takes precedence when both features are active (Cargo unification).
#[cfg(not(feature = "unreal"))]
mod bevy_backend;

#[cfg(not(feature = "unreal"))]
pub use bevy_backend::Query;

#[cfg(feature = "unreal")]
mod unreal_backend;

#[cfg(feature = "unreal")]
pub use unreal_backend::Query;
