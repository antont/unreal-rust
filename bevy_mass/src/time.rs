//! Time resource — standard `bevy_time::Time` in both backends.
//!
//! Systems use `Res<Time>` and `time.delta_secs()` — standard Bevy API.
//! In Unreal mode, `TimePlugin` + `ManualDuration` strategy provides
//! externally-driven time transparently.

pub use bevy_time::Time;
