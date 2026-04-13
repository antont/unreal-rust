//! Facade `Query` type that provides Bevy-compatible iteration over entity data.
//!
//! In Bevy mode: thin wrapper around `bevy_ecs::system::Query`.
//! In Unreal mode: iterates over Mass Entity chunk data via `MassSystemChunks`.
//!
//! # Which query type to use
//!
//! - **`Query`** — for data that exists in both Bevy and Unreal modes (MassFragments).
//!   The `#[mass_system]` macro rewrites these to chunk access in Unreal mode.
//!
//! - **`BevyQuery`** — for pure-Bevy components that only exist on Bevy entities
//!   (e.g., dynamically added/removed `Cooldown`). Always resolves to real
//!   `bevy_ecs::system::Query` in both modes. Passed through unchanged by the macro.
//!
//! - **`MassQuery` / `MassQueryAll`** (Unreal-only) — for Unreal-specific access
//!   patterns unavailable in standalone mode: spatial queries, cross-archetype
//!   index-based access. These do not compile without the `unreal` feature.

// Unreal takes precedence when both features are active (Cargo unification).
#[cfg(not(feature = "unreal"))]
mod bevy_backend;

#[cfg(not(feature = "unreal"))]
pub use bevy_backend::Query;
#[cfg(not(feature = "unreal"))]
pub use bevy_backend::BevyQuery;

#[cfg(feature = "unreal")]
mod unreal_backend;

#[cfg(feature = "unreal")]
pub use unreal_backend::Query;
#[cfg(feature = "unreal")]
pub use unreal_backend::BevyQuery;
