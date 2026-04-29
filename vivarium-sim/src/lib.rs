pub mod bird;
pub mod boundary;
pub mod brownian;
pub mod components;
pub mod config;
pub mod hunt;

#[cfg(feature = "unreal")]
pub mod unreal;

/// Install this sim's Bevy plugins into `app`. Called from both the
/// standalone binary (directly from `main`) and the UE app (via
/// `MassAppPluginRegistration` in `vivarium-sim/src/unreal/mod.rs`).
///
/// Note: this only installs `SpatialGroupPlugin`s — sim systems (wander,
/// flocking, hunt, etc.) are added per-harness (standalone in
/// `vivarium-standalone/src/main.rs`, UE via `MassScheduleOrder` in
/// `unreal/mod.rs`). `BirdHuntStates` lives in the same tier — Task 4
/// wires it into both harnesses. Tests in `hunt.rs` build their own app.
pub fn install_plugins(app: &mut bevy_app::App) {
    use bevy_mass::prelude::*;
    use components::{Bird, Insect, Transform};
    use config::{BIRD_SIGHT_RANGE, FLOCK_NEIGHBOR_RADIUS};

    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
        "birds",
        FLOCK_NEIGHBOR_RADIUS,
    ));
    // Insects are enumerated by birds during `hunt_system`, so the grid
    // must be at least `BIRD_SIGHT_RANGE` wide.
    app.add_plugins(SpatialGroupPlugin::<Insect, Transform>::new(
        "insects",
        BIRD_SIGHT_RANGE,
    ));
}
