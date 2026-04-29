//! UE-bridge wiring for the vivarium sim.
//!
//! All items here are gated by `feature = "unreal"` from the crate root.
//! Mirrors `gatherers-bevy-mass/src/lib.rs` but lives inside `vivarium-sim`
//! so we don't need a separate bridge crate — `bevy_mass` already handles
//! the generic facade, and a feature-gated module is enough to host the
//! per-sim UE registrations.

pub mod init;
pub mod ue_tests;

use crate::config::WORLD_HALF_SIZE;

// Plugin discovery entry: the framework finds this by name and calls
// `init::init_simulation` when UE's `URustMassBevySubsystem` spins up the
// "vivarium" sim.
inventory::submit!(unreal_api::mass::MassSimInitRegistration {
    name: "vivarium",
    init_fn: init::init_simulation,
});

// Explicit pipeline order. Insect brownian + bird wander / flocking run
// before the boundary force field clamps/repels near the bounds. Position
// integration is performed by C++ `UMassSimpleMovementProcessor` after the
// Bevy schedule dispatch, so no Rust movement system is listed here.
// `rebuild_bird_grid_system` must run before `flocking_system` so
// `SpatialQueries::neighbors_within("birds", ...)` sees the current frame.
inventory::submit!(unreal_api::mass::MassScheduleOrder {
    systems: &[
        "brownian_motion_system",
        "wander_system",
        "rebuild_bird_grid_system",
        "flocking_system",
        "boundary_force_system",
    ],
});

// Visualizer groups — tell C++ where to find each entity's position so the
// native MassRepresentation draws them.
inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "insects",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(crate::components::Transform, translation),
    scale: 0.3,
});
inventory::submit!(unreal_api::mass::MassVisualizerGroupRegistration {
    name: "birds",
    position_fragment_type: "FTransformFragment",
    position_offset: std::mem::offset_of!(crate::components::Transform, translation),
    scale: 1.0,
});

// Spatial-query config for the `"birds"` group. Lets the UE navigation hash
// grid track bird positions so a future `enumerate_in_radius` FFI can serve
// `SpatialQueries::neighbors_within("birds", ...)` from the C++ side. Until
// that FFI lands, `flocking_system` falls back to the Rust-side
// `SpatialGrids` resource populated by `rebuild_bird_grid_system`.
inventory::submit!(unreal_api::mass::MassSpatialQueryConfigRegistration {
    query_name: "birds",
    query_group: "birds",
    radius: crate::config::FLOCK_NEIGHBOR_RADIUS as f32,
    query_type: unreal_api::mass::MassSpatialQueryType::GridHash,
    collision_channel_index: 0,
    filter_fragment_type: "",
    filter_bool_offset: 0,
    filter_bool_must_be: false,
});

// Defaults for PIE spawning — editable on the level actor UPROPERTY.
inventory::submit!(unreal_api::mass::MassSimDefaultsRegistration {
    name: "vivarium",
    groups: &[
        ("insects", crate::config::INSECT_COUNT as i32),
        ("birds", crate::config::BIRD_COUNT as i32),
    ],
    bounds_min: [-WORLD_HALF_SIZE, -WORLD_HALF_SIZE, -WORLD_HALF_SIZE],
    bounds_max: [WORLD_HALF_SIZE, WORLD_HALF_SIZE, WORLD_HALF_SIZE],
    random_seed: 42,
});

// Hook runs during `mass_init_simulation` after entities are spawned. Writes
// the real PIE bounds into `SimBounds` so `boundary_force_system` uses the
// scene extents rather than the standalone default.
inventory::submit!(unreal_api::mass::MassSimInitHook {
    name: "vivarium_sim_bounds",
    hook_fn: init::populate_sim_bounds,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_defaults_registered() {
        let defaults: Vec<_> = unreal_api::mass::registered_sim_defaults()
            .into_iter()
            .collect();
        let d = defaults
            .iter()
            .find(|d| d.name == "vivarium")
            .expect("MassSimDefaultsRegistration 'vivarium' must be registered");
        assert_eq!(d.groups.len(), 2);
        assert_eq!(d.groups[0], ("insects", crate::config::INSECT_COUNT as i32));
        assert_eq!(d.groups[1], ("birds", crate::config::BIRD_COUNT as i32));
        assert_eq!(d.bounds_min, [-WORLD_HALF_SIZE, -WORLD_HALF_SIZE, -WORLD_HALF_SIZE]);
        assert_eq!(d.bounds_max, [WORLD_HALF_SIZE, WORLD_HALF_SIZE, WORLD_HALF_SIZE]);
        assert_eq!(d.random_seed, 42);
    }

    #[test]
    fn visualizer_group_registered() {
        let groups: Vec<_> = unreal_api::mass::registered_visualizer_groups()
            .into_iter()
            .collect();
        let insects = groups
            .iter()
            .find(|g| g.name == "insects")
            .expect("visualizer group 'insects' must be registered");
        assert_eq!(insects.position_fragment_type, "FTransformFragment");
        assert_eq!(insects.scale, 0.3);

        let birds = groups
            .iter()
            .find(|g| g.name == "birds")
            .expect("visualizer group 'birds' must be registered");
        assert_eq!(birds.position_fragment_type, "FTransformFragment");
        assert_eq!(birds.scale, 1.0);
    }

    #[test]
    fn bird_spatial_query_config_registered() {
        let configs: Vec<_> = unreal_api::mass::registered_spatial_query_configs()
            .into_iter()
            .collect();
        let c = configs
            .iter()
            .find(|c| c.query_group == "birds")
            .expect("MassSpatialQueryConfigRegistration 'birds' must be registered");
        assert_eq!(c.query_name, "birds");
        assert_eq!(c.radius, crate::config::FLOCK_NEIGHBOR_RADIUS as f32);
    }

    #[test]
    fn sim_init_registered() {
        let inits: Vec<_> = unreal_api::mass::registered_sim_inits()
            .into_iter()
            .collect();
        assert!(
            inits.iter().any(|r| r.name == "vivarium"),
            "MassSimInitRegistration 'vivarium' must be registered",
        );
    }

    /// Schedule-build smoke test — mirrors
    /// `gatherers-bevy-mass::tests::schedule_builds_and_runs_without_conflicts`.
    /// Constructs a `MassSchedule` from every `#[mass_system]` this crate
    /// submits and runs it once on an empty world. Bevy's access-conflict
    /// analysis runs on the first `run`, so system-param conflicts like
    /// B0001 panic here instead of first surfacing at UE runtime.
    #[test]
    fn schedule_builds_and_runs_without_conflicts() {
        use unreal_api::ecs::schedule::IntoScheduleConfigs;
        use unreal_api::mass::{
            MassSchedule, MassSystemStage, effective_order,
            registered_bevy_mass_systems, resolved_schedule_orders,
        };

        let mut sched = MassSchedule::new();
        let mut regs: Vec<_> = registered_bevy_mass_systems().into_iter().collect();
        let overrides = resolved_schedule_orders();
        regs.sort_by_key(|r| effective_order(r.name, r.order, &overrides));

        for i in 1..regs.len() {
            sched.app_mut().configure_sets(
                unreal_api::ecs::Update,
                MassSystemStage(i as u32).after(MassSystemStage((i - 1) as u32)),
            );
        }

        for (i, reg) in regs.iter().enumerate() {
            (reg.init_resources)(sched.world_mut());
            (reg.register_messages)(sched.app_mut());
            (reg.add_to_app)(sched.app_mut(), MassSystemStage(i as u32));
        }

        // `boundary_force_system` reads `Res<SimBounds>`; the UE path inserts
        // it via `populate_sim_bounds`. `flocking_system` reads
        // `SpatialQueries`, which in UE mode needs `SpatialQuery` as a
        // resource (the frame dispatcher installs this at runtime). Insert
        // both so param resolution can complete and access analysis runs.
        sched.world_mut().insert_resource(crate::components::SimBounds::default());
        sched.world_mut().insert_resource(bevy_mass::SpatialQuery::default());

        sched.run();

        assert!(
            !regs.is_empty(),
            "expected at least one Bevy-registered mass system",
        );
    }
}
