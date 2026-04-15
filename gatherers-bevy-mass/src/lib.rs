pub mod fragments;
pub mod init;
pub mod systems;
pub mod ue_tests;

inventory::submit!(unreal_api::mass::MassSimInitRegistration {
    name: "gatherers",
    init_fn: init::init_simulation,
});

// Collision group config: tells C++ where to find entity positions for
// spatial-query collision ISMCs. Rendering uses native MassRepresentation.
inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "ants",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(gatherers_sim::fragments::Transform, translation),
    scale: 0.2,
});

inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "food",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(gatherers_sim::fragments::Transform, translation),
    scale: 0.1,
});

// Spatial query config: search "food" group for loose food within 15.0 units
// Uses UE physics sweep via ECC_GameTraceChannel1 ("FoodQuery")
inventory::submit!(unreal_api::mass::MassSpatialQueryConfigRegistration {
    query_name: "food_pickup",
    query_group: "food",
    radius: 15.0,
    query_type: unreal_api::mass::MassSpatialQueryType::PhysicsSweep,
    collision_channel_index: 0, // ECC_GameTraceChannel1 = "FoodQuery"
    filter_fragment_type: "FGatherersMassFoodFragment",
    filter_bool_offset: 0, // is_loose is now the first (only) field in FoodFragment
    filter_bool_must_be: true,
});

// Default simulation parameters (overridable by actor UPROPERTY in editor)
inventory::submit!(unreal_api::mass::MassSimDefaultsRegistration {
    name: "gatherers",
    groups: &[("ants", 100), ("food", 500)],
    bounds_min: [-500.0, -500.0, 0.0],
    bounds_max: [500.0, 500.0, 100.0],
    random_seed: 42,
});

#[cfg(test)]
mod tests {
    #[test]
    fn sim_defaults_registered() {
        let defaults: Vec<_> = unreal_api::mass::registered_sim_defaults().into_iter().collect();
        let d = defaults.iter().find(|d| d.name == "gatherers")
            .expect("MassSimDefaultsRegistration 'gatherers' must be registered");
        assert_eq!(d.groups.len(), 2);
        assert_eq!(d.groups[0], ("ants", 100));
        assert_eq!(d.groups[1], ("food", 500));
        assert_eq!(d.bounds_min, [-500.0, -500.0, 0.0]);
        assert_eq!(d.bounds_max, [500.0, 500.0, 100.0]);
        assert_eq!(d.random_seed, 42);
    }

    #[test]
    fn spatial_query_config_registered() {
        let configs: Vec<_> = unreal_api::mass::registered_spatial_query_configs().into_iter().collect();
        let c = configs.iter().find(|c| c.query_group == "food")
            .expect("MassSpatialQueryConfigRegistration 'food' must be registered");
        assert_eq!(c.query_name, "food_pickup");
        assert_eq!(c.radius, 15.0);
        assert_eq!(c.query_type, unreal_api::mass::MassSpatialQueryType::PhysicsSweep);
        assert_eq!(c.collision_channel_index, 0);
        assert_eq!(c.filter_fragment_type, "FGatherersMassFoodFragment");
        assert_eq!(c.filter_bool_offset, 0); // is_loose is the only field in FoodFragment
        assert!(c.filter_bool_must_be);
    }

    #[test]
    fn visualizer_groups_registered() {
        let groups: Vec<_> = unreal_api::mass::registered_visualizer_groups().into_iter().collect();
        let ants = groups.iter().find(|g| g.name == "ants")
            .expect("visualizer group 'ants' must be registered");
        assert_eq!(ants.position_fragment_type, "FTransformFragment");
        assert_eq!(ants.scale, 0.2);
        let food = groups.iter().find(|g| g.name == "food")
            .expect("visualizer group 'food' must be registered");
        assert_eq!(food.position_fragment_type, "FTransformFragment");
        assert_eq!(food.scale, 0.1);
    }
}
