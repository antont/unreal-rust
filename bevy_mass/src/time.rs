//! Delta time resource, compatible across both backends.

/// Per-frame delta time in seconds.
///
/// In Bevy mode: populated by the application before running the schedule.
/// In Unreal mode: populated from `MassDeltaTime` by the dispatch layer.
#[derive(bevy_ecs::prelude::Resource, Default, Clone, Copy, Debug)]
pub struct DeltaTime(pub f32);
