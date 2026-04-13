use bevy::prelude::*;
use gatherers_sim::fragments::*;
use gatherers_sim::food_decision::{food_decision_system, DECISION_PICK_UP, DECISION_DROP};
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
        // Ants spawn WITHOUT Cooldown — it's added dynamically via Commands
        // after pickup/drop, and removed when the timer expires.
        // This matches the idiomatic Bevy pattern from the original gatherers.
        commands.spawn((
            AntMarker,
            Position {
                position: spawn_pos,
                previous_position: spawn_pos,
            },
            Movement {
                direction: DVec3::new(angle.cos(), angle.sin(), 0.0),
                movement_speed: 100.0,
            },
            Carrying::default(),
            Behavior {
                turn_jitter_radians: std::f32::consts::FRAC_PI_2,
                random_seed: 42 + i as i32,
            },
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

/// Collision prepass: brute-force proximity search, emits HitEvent messages.
/// Matches the original gatherers CollisionPlugin pattern.
fn collision_prepass(
    ants: Query<(Entity, &Position), (With<AntMarker>, Without<Cooldown>)>,
    foods: Query<&FoodFragment, With<FoodMarker>>,
    food_entities: Res<FoodEntities>,
    mut hits: MessageWriter<AntFoodHit>,
) {
    // Snapshot loose food positions for proximity search
    let loose_food: Vec<(i32, DVec3)> = food_entities
        .0
        .iter()
        .enumerate()
        .filter_map(|(idx, &entity)| {
            foods.get(entity).ok().and_then(|f| {
                if f.is_loose {
                    Some((idx as i32, f.position))
                } else {
                    None
                }
            })
        })
        .collect();

    for (ant_entity, pos) in &ants {
        if let Some(&(food_idx, food_pos)) = loose_food
            .iter()
            .filter(|(_, fp)| (pos.position - *fp).length_squared() < PICKUP_RADIUS * PICKUP_RADIUS)
            .min_by(|(_, a), (_, b)| {
                (pos.position - *a)
                    .length_squared()
                    .partial_cmp(&(pos.position - *b).length_squared())
                    .unwrap()
            })
        {
            hits.write(AntFoodHit::new(food_idx, ant_entity, food_pos));
        }
    }
}

/// Apply food mutations: reads FoodMutation messages, updates FoodFragment state.
fn apply_food_mutations(
    mut mutations: MessageReader<FoodMutation>,
    mut foods: Query<&mut FoodFragment, With<FoodMarker>>,
    food_entities: Res<FoodEntities>,
) {
    for mutation in mutations.read() {
        let idx = mutation.food_index as usize;
        if idx < food_entities.0.len() {
            if let Ok(mut food) = foods.get_mut(food_entities.0[idx]) {
                if mutation.decision == DECISION_PICK_UP {
                    food.is_loose = false;
                } else if mutation.decision == DECISION_DROP {
                    food.is_loose = true;
                    food.position = mutation.drop_position;
                }
            }
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
        .add_message::<AntFoodHit>()
        .add_message::<FoodMutation>()
        .add_systems(Startup, (setup_camera, spawn_entities))
        .add_systems(
            Update,
            (
                sync_delta_time,
                entity_movement,
                entity_boundary_reflect,
                collision_prepass,
                food_decision_system,
                apply_food_mutations,
                entity_cooldown,
                sync_ant_transforms,
                sync_food_transforms,
                sync_food_colors,
            )
                .chain(),
        )
        .run();
}
