pub mod bird;
pub mod boundary;
pub mod brownian;
pub mod components;
pub mod config;

#[cfg(feature = "unreal")]
pub mod unreal;

/// Install this sim's Bevy plugins into `app`. Called from both the
/// standalone binary (directly from `main`) and the UE app (via
/// `MassAppPluginRegistration` in `vivarium-sim/src/unreal/mod.rs`).
pub fn install_plugins(app: &mut bevy_app::App) {
    use bevy_mass::prelude::*;
    use components::{Bird, Transform};
    use config::FLOCK_NEIGHBOR_RADIUS;

    app.add_plugins(SpatialGroupPlugin::<Bird, Transform>::new(
        "birds",
        FLOCK_NEIGHBOR_RADIUS,
    ));
}
