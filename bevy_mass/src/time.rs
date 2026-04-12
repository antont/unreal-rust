//! Delta time resource, compatible across both backends.

/// Per-frame delta time in seconds.
///
/// In Bevy mode: populated by the application before running the schedule.
/// In Unreal mode: alias for `MassDeltaTime`, populated by the dispatch layer.

// Unreal takes precedence when both features are active (Cargo unification).
#[cfg(not(feature = "unreal"))]
#[derive(bevy_ecs::prelude::Resource, Default, Clone, Copy, Debug)]
pub struct DeltaTime(pub f32);

#[cfg(feature = "unreal")]
pub type DeltaTime = unreal_api::mass::MassDeltaTime;
