use glam::DVec3;
use gatherers_sim::components::*;
use unreal_api::mass::EntityArchetype;
use unreal_ffi::{MassEntityHandle, MassInitSimulationParams};

/// Internal helper: spawns ant and food entities given counts, bounds, and seed.
fn spawn_entities(
    ant_count: i32,
    food_count: i32,
    bounds_min: [f64; 3],
    bounds_max: [f64; 3],
    random_seed: i32,
) -> (Vec<MassEntityHandle>, Vec<MassEntityHandle>) {
    let mut seed = random_seed as u64;
    let mut rng = || -> f64 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as f64 / (1u64 << 31) as f64
    };

    // Spawn food — position lives in FTransformFragment (shared with native MassRepresentation)
    let food_handles = EntityArchetype::new("food")
        .fragment::<Transform>()
        .fragment::<FoodState>()
        .tag::<Food>()
        .spawn(food_count as u32, |_i, writer| {
            let pos = DVec3::new(
                bounds_min[0] + rng() * (bounds_max[0] - bounds_min[0]),
                bounds_min[1] + rng() * (bounds_max[1] - bounds_min[1]),
                50.0,
            );
            writer.set(&Transform::from_translation(pos));
            writer.set(&FoodState {
                is_loose: true,
            });
        });

    // Spawn ants at random positions within bounds
    let ant_handles = EntityArchetype::new("ant")
        .fragment::<Transform>()
        .fragment::<PreviousTranslation>()
        .fragment::<DesiredMovement>()
        .fragment::<Velocity>()          // Internal — UE's UMassApplyMovementProcessor needs it
        .fragment::<Behavior>()
        .tag::<Ant>()
        .tag::<CodeDrivenMovementTag>()  // Required by UE's UMassApplyMovementProcessor
        .spawn(ant_count as u32, |i, writer| {
            let angle = rng() * std::f64::consts::TAU;
            let spawn_pos = DVec3::new(
                bounds_min[0] + rng() * (bounds_max[0] - bounds_min[0]),
                bounds_min[1] + rng() * (bounds_max[1] - bounds_min[1]),
                50.0,
            );
            writer.set(&Transform::from_translation(spawn_pos));
            writer.set(&PreviousTranslation { value: spawn_pos });
            writer.set(&DesiredMovement::new(DVec3::new(angle.cos(), angle.sin(), 0.0), 100.0));
            writer.set(&Behavior {
                turn_jitter_radians: 0.0,
                random_seed: random_seed + i as i32,
            });
        });

    (ant_handles, food_handles)
}

/// Init: receives named group counts, returns named groups of entity handles.
pub fn init_simulation(params: &MassInitSimulationParams) -> Vec<(String, Vec<MassEntityHandle>)> {
    // Extract group counts by name from the params
    let groups = if params.groups.is_null() || params.num_groups == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(params.groups, params.num_groups as usize) }
    };

    let mut ant_count = 0i32;
    let mut food_count = 0i32;
    for g in groups {
        let name_bytes = unsafe { std::slice::from_raw_parts(g.name.ptr as *const u8, g.name.len) };
        let name = std::str::from_utf8(name_bytes).unwrap_or("");
        match name {
            "ants" => ant_count = g.count,
            "food" => food_count = g.count,
            _ => {}
        }
    }

    let (ant_handles, food_handles) = spawn_entities(
        ant_count,
        food_count,
        params.bounds_min,
        params.bounds_max,
        params.random_seed,
    );

    vec![
        (Ant::ENTITY_GROUP.to_string(), ant_handles),
        (Food::ENTITY_GROUP.to_string(), food_handles),
    ]
}
