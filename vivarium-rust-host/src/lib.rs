// Force-link vivarium-sim so its inventory registrations are included.
// The UE bridge (RustMassBevySubsystem) ticks the sim via its own schedule
// derived from those inventory entries — this host crate only needs to
// exist as a loadable dylib shell.
extern crate vivarium_sim;

use unreal_module::UserModule;

pub struct VivariumHost;

impl VivariumHost {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VivariumHost {
    fn default() -> Self {
        Self::new()
    }
}

impl UserModule for VivariumHost {
    fn initialize(&mut self) {
        unreal_api::registration::register_all_uclasses();
        log::info!("vivarium-rust-host initialize");
    }

    fn tick(&mut self, _dt: f32) {}

    fn begin_play(&mut self) {
        log::info!("vivarium-rust-host begin_play");
    }
}

unreal_module::implement_unreal_module!(VivariumHost::new());
