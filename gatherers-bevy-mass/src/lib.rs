pub mod fragments;
pub mod init;
pub mod systems;

inventory::submit!(unreal_api::mass::MassSimInitRegistration {
    name: "gatherers",
    init_fn: init::init_simulation,
});

inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "ants",
    position_fragment_type: "FGatherersPosition",
    position_offset: 0, // Position.position is at offset 0
    scale: 0.2,
});

inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "food",
    position_fragment_type: "FGatherersMassFoodFragment",
    position_offset: 0, // FoodFragment.position is at offset 0
    scale: 0.1,
});
