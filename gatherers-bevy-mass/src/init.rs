use gatherers_sim::fragments::*;
use unreal_api::mass::EntityArchetype;
use unreal_ffi::{MassEntityHandle, MassInitSimulationParams};

pub fn init_simulation(
    params: &MassInitSimulationParams,
) -> (Vec<MassEntityHandle>, Vec<MassEntityHandle>) {
    let mut seed = params.random_seed as u64;
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
        .spawn(params.food_count as u32, |_i, writer| {
            writer.set(&FoodFragment {
                position: [
                    params.bounds_min[0]
                        + rng() * (params.bounds_max[0] - params.bounds_min[0]),
                    params.bounds_min[1]
                        + rng() * (params.bounds_max[1] - params.bounds_min[1]),
                    50.0,
                ],
                is_loose: true,
                _pad: [0; 7],
            });
        });

    // Spawn ants
    let center_x = (params.bounds_min[0] + params.bounds_max[0]) / 2.0;
    let center_y = (params.bounds_min[1] + params.bounds_max[1]) / 2.0;
    let step = 50.0;
    let half = params.ant_count as f64 / 2.0;

    let ant_handles = EntityArchetype::new("ant")
        .fragment::<Position>()
        .fragment::<Movement>()
        .fragment::<Cooldown>()
        .fragment::<Carrying>()
        .fragment::<Behavior>()
        .fragment::<AntEncounterFragment>()
        .tag::<BevyMassAntTag>()
        .spawn(params.ant_count as u32, |i, writer| {
            let angle = rng() * std::f64::consts::TAU;
            writer.set(&Position {
                position: [
                    center_x + (i as f64 - half) * step,
                    center_y + 100.0,
                    50.0,
                ],
                previous_position: [
                    center_x + (i as f64 - half) * step,
                    center_y + 100.0,
                    50.0,
                ],
            });
            writer.set(&Movement {
                direction: [angle.cos(), angle.sin(), 0.0],
                movement_speed: 100.0,
                _pad: [0; 4],
            });
            writer.set(&Behavior {
                turn_jitter_radians: 0.0,
                random_seed: params.random_seed + i as i32,
            });
            writer.set(&Carrying {
                food_index: -1,
                _pad: 0,
            });
        });

    (ant_handles, food_handles)
}
