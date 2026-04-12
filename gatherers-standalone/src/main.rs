use bevy::prelude::*;
use gatherers_sim::fragments::*;
use gatherers_sim::food_decision::{
    ant_food_decision, DECISION_DROP, DECISION_PICK_UP,
};
use gatherers_sim::movement::{
    entity_boundary_reflect, entity_cooldown, entity_movement, SIM_BOUNDS_MAX, SIM_BOUNDS_MIN,
};
use glam::DVec3;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ANT_COUNT: u32 = 100;
const FOOD_COUNT: u32 = 500;
const ANT_SIZE: f32 = 20.0;
const FOOD_SIZE: f32 = 10.0;
const PICKUP_RADIUS: f64 = 15.0;

const COLOR_ANT: Color = Color::srgb(0.8, 0.8, 0.8);
const COLOR_FOOD_LOOSE: Color = Color::srgb(0.75, 0.01, 0.01);
const COLOR_FOOD_CARRIED: Color = Color::srgb(1.0, 0.5, 0.0); // orange, visible on top of ant

const Z_FOOD: f32 = 1.0;
const Z_ANT: f32 = 2.0;

// ---------------------------------------------------------------------------
// Components and resources
// ---------------------------------------------------------------------------

#[derive(Component)]
struct AntMarker;

#[derive(Component)]
struct FoodMarker;

/// Ordered list of food entities — index matches `Carrying.food_index` and
/// `FoodEncounter.food_index` so we can reuse `ant_food_decision()`.
#[derive(Resource)]
struct FoodEntities(Vec<Entity>);

// ---------------------------------------------------------------------------
// Startup
// ---------------------------------------------------------------------------

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Scale factor: sim coordinates → screen pixels.
/// Sim spans 1000 units, window is 800px. We shrink to fit with margin.
const SIM_TO_SCREEN: f32 = 800.0 / 1100.0; // 1100 = 1000 + 10% margin

fn spawn_entities(mut commands: Commands) {
    let bounds_min = DVec3::new(SIM_BOUNDS_MIN[0], SIM_BOUNDS_MIN[1], SIM_BOUNDS_MIN[2]);
    let bounds_max = DVec3::new(SIM_BOUNDS_MAX[0], SIM_BOUNDS_MAX[1], SIM_BOUNDS_MAX[2]);
    let bounds_range = bounds_max - bounds_min;

    // LCG RNG matching gatherers-bevy-mass/src/init.rs
    let mut seed: u64 = 42;
    let mut rng = || -> f64 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as f64 / (1u64 << 31) as f64
    };

    // Spawn food — collect entity IDs for index-based lookup
    let mut food_entities = Vec::with_capacity(FOOD_COUNT as usize);
    for _ in 0..FOOD_COUNT {
        let pos = DVec3::new(
            bounds_min.x + rng() * bounds_range.x,
            bounds_min.y + rng() * bounds_range.y,
            50.0,
        );
        let entity = commands
            .spawn((
                FoodMarker,
                FoodFragment {
                    position: pos,
                    is_loose: true,
                    _pad: [0; 7],
                },
                Sprite {
                    color: COLOR_FOOD_LOOSE,
                    custom_size: Some(Vec2::splat(FOOD_SIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    pos.x as f32 * SIM_TO_SCREEN,
                    pos.y as f32 * SIM_TO_SCREEN,
                    Z_FOOD,
                )),
            ))
            .id();
        food_entities.push(entity);
    }
    commands.insert_resource(FoodEntities(food_entities));

    // Spawn ants at random positions within bounds
    for i in 0..ANT_COUNT {
        let angle = rng() * std::f64::consts::TAU;
        let spawn_pos = DVec3::new(
            bounds_min.x + rng() * bounds_range.x,
            bounds_min.y + rng() * bounds_range.y,
            50.0,
        );
        commands.spawn((
            AntMarker,
            Position {
                position: spawn_pos,
                previous_position: spawn_pos,
            },
            Movement {
                direction: DVec3::new(angle.cos(), angle.sin(), 0.0),
                movement_speed: 100.0,
                _pad: [0; 4],
            },
            Cooldown::default(),
            Carrying::default(),
            Behavior {
                turn_jitter_radians: std::f32::consts::FRAC_PI_2,
                random_seed: 42 + i as i32,
            },
            AntEncounterFragment::default(),
            Sprite {
                color: COLOR_ANT,
                custom_size: Some(Vec2::splat(ANT_SIZE)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                spawn_pos.x as f32 * SIM_TO_SCREEN,
                spawn_pos.y as f32 * SIM_TO_SCREEN,
                Z_ANT,
            )),
        ));
    }
}

// ---------------------------------------------------------------------------
// Simulation systems
// ---------------------------------------------------------------------------

/// Copy Bevy's frame time into the DeltaTime resource used by gatherers-sim systems.
fn sync_delta_time(time: Res<Time>, mut dt: ResMut<bevy_mass::DeltaTime>) {
    dt.0 = time.delta_secs();
}

/// Brute-force proximity food interaction using ant_food_decision().
fn simple_food_interaction(
    mut ants: Query<
        (&mut Position, &mut Movement, &mut Cooldown, &mut Carrying, &mut Behavior),
        With<AntMarker>,
    >,
    mut foods: Query<&mut FoodFragment, With<FoodMarker>>,
    food_entities: Res<FoodEntities>,
) {
    // Snapshot loose food positions for proximity search
    let loose_food: Vec<(usize, DVec3)> = food_entities
        .0
        .iter()
        .enumerate()
        .filter_map(|(idx, &entity)| {
            foods.get(entity).ok().and_then(|f| {
                if f.is_loose {
                    Some((idx, f.position))
                } else {
                    None
                }
            })
        })
        .collect();

    for (mut pos, mut mov, mut cd, mut carry, mut beh) in &mut ants {
        // Find nearest loose food within pickup radius
        let encounter = loose_food
            .iter()
            .filter(|(_, food_pos)| {
                let d = pos.position - *food_pos;
                d.x * d.x + d.y * d.y < PICKUP_RADIUS * PICKUP_RADIUS
            })
            .min_by(|(_, a), (_, b)| {
                let da = (pos.position - *a).length_squared();
                let db = (pos.position - *b).length_squared();
                da.partial_cmp(&db).unwrap()
            })
            .map(|(idx, food_pos)| FoodEncounter {
                food_index: *idx as i32,
                _pad: 0,
                encounter_position: *food_pos,
            });

        let old_food_index = carry.food_index;
        let pos_before_snap = pos.position;

        let decision = ant_food_decision(
            &mut pos.position,
            &mut mov,
            &mut cd,
            &mut carry,
            &mut beh,
            encounter.as_ref(),
        );

        match decision {
            DECISION_PICK_UP => {
                // Mark picked-up food as not loose
                let idx = carry.food_index as usize;
                if idx < food_entities.0.len() {
                    if let Ok(mut food) = foods.get_mut(food_entities.0[idx]) {
                        food.is_loose = false;
                    }
                }
            }
            DECISION_DROP => {
                // Mark previously-carried food as loose at the ant's position
                // BEFORE the snap — otherwise it stacks on the encounter food.
                if old_food_index >= 0 {
                    let idx = old_food_index as usize;
                    if idx < food_entities.0.len() {
                        if let Ok(mut food) = foods.get_mut(food_entities.0[idx]) {
                            food.is_loose = true;
                            food.position = pos_before_snap;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering sync systems
// ---------------------------------------------------------------------------

fn sync_ant_transforms(mut ants: Query<(&Position, &mut Transform), With<AntMarker>>) {
    for (pos, mut transform) in &mut ants {
        transform.translation.x = pos.position.x as f32 * SIM_TO_SCREEN;
        transform.translation.y = pos.position.y as f32 * SIM_TO_SCREEN;
    }
}

fn sync_food_transforms(
    ants: Query<(&Position, &Carrying), With<AntMarker>>,
    mut foods: Query<(&FoodFragment, &mut Transform), With<FoodMarker>>,
    food_entities: Res<FoodEntities>,
) {
    // Build a map: food_index → ant position (for carried food)
    let mut carried_positions: Vec<Option<DVec3>> = vec![None; food_entities.0.len()];
    for (pos, carry) in &ants {
        if carry.food_index >= 0 && (carry.food_index as usize) < carried_positions.len() {
            carried_positions[carry.food_index as usize] = Some(pos.position);
        }
    }

    for (idx, &entity) in food_entities.0.iter().enumerate() {
        if let Ok((food, mut transform)) = foods.get_mut(entity) {
            if let Some(ant_pos) = carried_positions[idx] {
                // Carried: follow the ant, rendered above it
                transform.translation.x = ant_pos.x as f32 * SIM_TO_SCREEN;
                transform.translation.y = ant_pos.y as f32 * SIM_TO_SCREEN;
                transform.translation.z = Z_ANT + 1.0;
            } else {
                // Loose: show at food's own position
                transform.translation.x = food.position.x as f32 * SIM_TO_SCREEN;
                transform.translation.y = food.position.y as f32 * SIM_TO_SCREEN;
                transform.translation.z = Z_FOOD;
            }
        }
    }
}

fn sync_food_colors(mut foods: Query<(&FoodFragment, &mut Sprite), With<FoodMarker>>) {
    for (food, mut sprite) in &mut foods {
        sprite.color = if food.is_loose {
            COLOR_FOOD_LOOSE
        } else {
            COLOR_FOOD_CARRIED
        };
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gatherers Sim".into(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(bevy_mass::DeltaTime(0.0))
        .add_systems(Startup, (setup_camera, spawn_entities))
        .add_systems(
            Update,
            (
                sync_delta_time,
                entity_movement,
                entity_boundary_reflect,
                entity_cooldown,
                simple_food_interaction,
                sync_ant_transforms,
                sync_food_transforms,
                sync_food_colors,
            )
                .chain(),
        )
        .run();
}
