//! UE-side simulation init for vivarium.
//!
//! Mirrors `gatherers-bevy-mass/src/init.rs`: reads named group counts
//! from `MassInitSimulationParams`, spawns entities in chunk memory via
//! `EntityArchetype`, and returns named handle lists for UE.
//!
//! Also hosts `populate_sim_bounds` — a `MassSimInitHook` that writes
//! `SimBounds` into the Bevy world from the PIE-supplied bounds.

use crate::components::*;
use crate::config::{INSECT_WANDER_STRENGTH, INSECT_SPEED};
use bevy_mass::prelude::DVec3;
use unreal_api::mass::EntityArchetype;
use unreal_ffi::{MassEntityHandle, MassInitSimulationParams};

/// Extract a named group's count from the FFI group slice, defaulting to 0.
fn group_count(params: &MassInitSimulationParams, wanted: &str) -> i32 {
    let groups = if params.groups.is_null() || params.num_groups == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(params.groups, params.num_groups as usize) }
    };
    for g in groups {
        let name_bytes = unsafe {
            std::slice::from_raw_parts(g.name.ptr as *const u8, g.name.len)
        };
        let name = std::str::from_utf8(name_bytes).unwrap_or("");
        if name == wanted {
            return g.count;
        }
    }
    0
}

/// Spawns `count` insects inside the given bounds, uniform-random positions,
/// uniform-random unit directions at `INSECT_SPEED`, and a per-entity LCG
/// seed derived from `random_seed` for brownian motion.
fn spawn_insects(
    count: i32,
    bounds_min: [f64; 3],
    bounds_max: [f64; 3],
    random_seed: i32,
) -> Vec<MassEntityHandle> {
    let mut seed = random_seed as u64;
    let mut rng = || -> f64 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as f64 / (1u64 << 31) as f64
    };

    EntityArchetype::new("insects")
        .fragment::<Transform>()
        .fragment::<PreviousTranslation>()
        .fragment::<Velocity>()
        .fragment::<BrownianMotion>()
        .tag::<Insect>()
        .tag::<BoundaryWrap>()
        .tag::<SimpleMovementTag>() // Required by UMassSimpleMovementProcessor.
        .spawn(count as u32, |i, writer| {
            let pos = DVec3::new(
                bounds_min[0] + rng() * (bounds_max[0] - bounds_min[0]),
                bounds_min[1] + rng() * (bounds_max[1] - bounds_min[1]),
                bounds_min[2] + rng() * (bounds_max[2] - bounds_min[2]),
            );
            let theta = rng() * std::f64::consts::TAU;
            let phi = (rng() * 2.0 - 1.0).acos();
            let dir = DVec3::new(
                phi.sin() * theta.cos(),
                phi.sin() * theta.sin(),
                phi.cos(),
            );

            writer.set(&Transform::from_translation(pos));
            writer.set(&PreviousTranslation { value: pos });
            writer.set(&Velocity::new(dir, INSECT_SPEED));
            writer.set(&BrownianMotion {
                wander_strength: INSECT_WANDER_STRENGTH,
                random_seed: 0x9E3779B1u32.wrapping_add(i as u32),
            });
        })
}

/// Entry point called by `unreal-module` during `mass_init_simulation`.
pub fn init_simulation(params: &MassInitSimulationParams) -> Vec<(String, Vec<MassEntityHandle>)> {
    let insect_count = group_count(params, "insects");

    let handles = spawn_insects(
        insect_count,
        params.bounds_min,
        params.bounds_max,
        params.random_seed,
    );

    vec![(Insect::ENTITY_GROUP.to_string(), handles)]
}

/// `MassSimInitHook` body: write `SimBounds` into the Bevy world so
/// `boundary_force_system` repels at the PIE-supplied extents, not the
/// standalone default (`WORLD_HALF_SIZE`).
pub fn populate_sim_bounds(
    world: &mut unreal_api::ecs::world::World,
    params: &MassInitSimulationParams,
) {
    world.insert_resource(SimBounds {
        min: DVec3::new(params.bounds_min[0], params.bounds_min[1], params.bounds_min[2]),
        max: DVec3::new(params.bounds_max[0], params.bounds_max[1], params.bounds_max[2]),
    });
}
