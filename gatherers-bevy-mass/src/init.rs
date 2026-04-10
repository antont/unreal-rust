use gatherers_sim::fragments::*;
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

    // Spawn food
    let food_handles = EntityArchetype::new("food")
        .fragment::<FoodFragment>()
        .tag::<FoodTag>()
        .spawn(food_count as u32, |_i, writer| {
            writer.set(&FoodFragment {
                position: [
                    bounds_min[0] + rng() * (bounds_max[0] - bounds_min[0]),
                    bounds_min[1] + rng() * (bounds_max[1] - bounds_min[1]),
                    50.0,
                ],
                is_loose: true,
                _pad: [0; 7],
            });
        });

    // Spawn ants
    let center_x = (bounds_min[0] + bounds_max[0]) / 2.0;
    let center_y = (bounds_min[1] + bounds_max[1]) / 2.0;
    let step = 50.0;
    let half = ant_count as f64 / 2.0;

    let ant_handles = EntityArchetype::new("ant")
        .fragment::<Position>()
        .fragment::<Movement>()
        .fragment::<Cooldown>()
        .fragment::<Carrying>()
        .fragment::<Behavior>()
        .fragment::<AntEncounterFragment>()
        .tag::<BevyMassAntTag>()
        .spawn(ant_count as u32, |i, writer| {
            let angle = rng() * std::f64::consts::TAU;
            writer.set(&Position {
                position: [center_x + (i as f64 - half) * step, center_y + 100.0, 50.0],
                previous_position: [center_x + (i as f64 - half) * step, center_y + 100.0, 50.0],
            });
            writer.set(&Movement {
                direction: [angle.cos(), angle.sin(), 0.0],
                movement_speed: 100.0,
                _pad: [0; 4],
            });
            writer.set(&Behavior {
                turn_jitter_radians: 0.0,
                random_seed: random_seed + i as i32,
            });
            writer.set(&Carrying {
                food_index: -1,
                _pad: 0,
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
        ("ants".to_string(), ant_handles),
        ("food".to_string(), food_handles),
    ]
}
