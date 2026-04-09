pub mod fragments;
pub mod init;
pub mod systems;

inventory::submit!(unreal_api::mass::MassSimInitRegistration {
    name: "gatherers",
    init_fn: init::init_simulation,
});
