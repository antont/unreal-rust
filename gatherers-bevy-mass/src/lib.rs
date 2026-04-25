pub mod components;
pub mod init;
pub mod systems;
pub mod ue_tests;

inventory::submit!(unreal_api::mass::MassSimInitRegistration {
    name: "gatherers",
    init_fn: init::init_simulation,
});

// System execution order for the gatherers pipeline.
//
// This replaces `order = N` on every `#[mass_system]` — the framework
// maps these names to numeric order values at plugin init time, and
// both the Bevy schedule and the C++ processor pipeline respect the
// result. Insert new systems at the right position; no need to
// renumber anything.
inventory::submit!(unreal_api::mass::MassScheduleOrder {
    systems: &[
        "ant_collision_prepass",
        "food_decision_system",
        "apply_food_mutations",
        "entity_cooldown",
        "carried_food_tracking",
        "entity_boundary_reflect",
    ],
});

// Collision group config: tells C++ where to find entity positions for
// spatial-query collision ISMCs. Rendering uses native MassRepresentation.
inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "ants",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(gatherers_sim::components::Transform, translation),
    scale: 0.2,
});

inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "food",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(gatherers_sim::components::Transform, translation),
    scale: 0.1,
});

// Spatial query config: search "food" group for loose food within 15.0 units.
// Uses UE's Mass navigation hash grid (FNavigationObstacleHashGrid2D).
inventory::submit!(unreal_api::mass::MassSpatialQueryConfigRegistration {
    query_name: "food_pickup",
    query_group: "food",
    radius: 15.0,
    query_type: unreal_api::mass::MassSpatialQueryType::GridHash,
    collision_channel_index: 0, // unused for GridHash
    filter_fragment_type: "FGatherersFoodStateFragment",
    filter_bool_offset: 0, // is_loose is the first (only) field in FoodState
    filter_bool_must_be: true,
});

// Default simulation parameters (overridable by actor UPROPERTY in editor)
inventory::submit!(unreal_api::mass::MassSimDefaultsRegistration {
    name: "gatherers",
    groups: &[("ants", 3000), ("food", 10000)],
    bounds_min: [-5000.0, -5000.0, 0.0],
    bounds_max: [5000.0, 5000.0, 100.0],
    random_seed: 42,
});

#[cfg(test)]
mod tests {
    #[test]
    fn sim_defaults_registered() {
        let defaults: Vec<_> = unreal_api::mass::registered_sim_defaults()
            .into_iter()
            .collect();
        let d = defaults
            .iter()
            .find(|d| d.name == "gatherers")
            .expect("MassSimDefaultsRegistration 'gatherers' must be registered");
        assert_eq!(d.groups.len(), 2);
        assert_eq!(d.groups[0], ("ants", 3000));
        assert_eq!(d.groups[1], ("food", 10000));
        assert_eq!(d.bounds_min, [-5000.0, -5000.0, 0.0]);
        assert_eq!(d.bounds_max, [5000.0, 5000.0, 100.0]);
        assert_eq!(d.random_seed, 42);
    }

    #[test]
    fn spatial_query_config_registered() {
        let configs: Vec<_> = unreal_api::mass::registered_spatial_query_configs()
            .into_iter()
            .collect();
        let c = configs
            .iter()
            .find(|c| c.query_group == "food")
            .expect("MassSpatialQueryConfigRegistration 'food' must be registered");
        assert_eq!(c.query_name, "food_pickup");
        assert_eq!(c.radius, 15.0);
        assert_eq!(
            c.query_type,
            unreal_api::mass::MassSpatialQueryType::GridHash
        );
        assert_eq!(c.collision_channel_index, 0);
        assert_eq!(c.filter_fragment_type, "FGatherersFoodStateFragment");
        assert_eq!(c.filter_bool_offset, 0); // is_loose is the only field in FoodState
        assert!(c.filter_bool_must_be);
    }

    #[test]
    fn visualizer_groups_registered() {
        let groups: Vec<_> = unreal_api::mass::registered_visualizer_groups()
            .into_iter()
            .collect();
        let ants = groups
            .iter()
            .find(|g| g.name == "ants")
            .expect("visualizer group 'ants' must be registered");
        assert_eq!(ants.position_fragment_type, "FTransformFragment");
        assert_eq!(ants.scale, 0.2);
        let food = groups
            .iter()
            .find(|g| g.name == "food")
            .expect("visualizer group 'food' must be registered");
        assert_eq!(food.position_fragment_type, "FTransformFragment");
        assert_eq!(food.scale, 0.1);
    }
}
