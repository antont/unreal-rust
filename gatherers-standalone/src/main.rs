use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_mass::{MovementPlugin, EntityIndex};
use gatherers_sim::components::{
    Transform as SimTransform, PreviousTranslation, DesiredMovement,
    Cooldown, Carrying, Behavior, FoodState, Food,
    AntFoodHit, FoodMutation,
};
use gatherers_sim::food_decision::{food_decision_system, DECISION_PICK_UP, DECISION_DROP};
use gatherers_sim::movement::{
    entity_boundary_reflect, entity_cooldown, SIM_BOUNDS_MAX, SIM_BOUNDS_MIN,
};
use glam::DVec3;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

// 800×800 window at current scale can't visually accommodate the UE-scale
// counts (10k ants / 5k food); entities end up packed so tight that every
// food is within pickup radius of an ant at spawn, and the ants themselves
// carpet the window. These counts give a legible simulation.
const ANT_COUNT: u32 = 300;
const FOOD_COUNT: u32 = 150;
const ANT_SIZE: f32 = 8.0;
const FOOD_SIZE: f32 = 6.0;
const PICKUP_RADIUS: f64 = 15.0;

// Matches /Users/tonialatalo/src/gatherers/src/config.rs
const COLOR_BACKGROUND: Color = Color::srgb(95.0 / 255.0, 151.0 / 255.0, 212.0 / 255.0);
const COLOR_ANT: Color = Color::srgb(0.8, 0.8, 0.8);
const COLOR_FOOD_LOOSE: Color = Color::srgb(192.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0);
// Original sim doesn't recolor carried food, just raises its Z layer. We do
// the same — carried food stays the same red and is visible on top of ants
// via Z ordering.
const COLOR_FOOD_CARRIED: Color = COLOR_FOOD_LOOSE;

const Z_FOOD: f32 = 1.0;
const Z_ANT: f32 = 2.0;

// ---------------------------------------------------------------------------
// Components and resources
// ---------------------------------------------------------------------------

#[derive(Component)]
struct AntMarker;

#[derive(Component)]
struct FoodMarker;

// ---------------------------------------------------------------------------
// Startup
// ---------------------------------------------------------------------------

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(COLOR_BACKGROUND),
            ..default()
        },
    ));
}

/// Scale factor: sim coordinates → screen pixels.
/// Sim spans (SIM_BOUNDS_MAX − SIM_BOUNDS_MIN).x = 1000 units on each axis;
/// fit into the 800px window with a 10% margin.
const SIM_TO_SCREEN: f32 = 800.0 / 1100.0;

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
                Food,
                SimTransform::from_translation(pos),
                FoodState {
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
    commands.insert_resource(EntityIndex::<Food>::new(food_entities));

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
            SimTransform::from_translation(spawn_pos),
            PreviousTranslation { value: spawn_pos },
            DesiredMovement::new(DVec3::new(angle.cos(), angle.sin(), 0.0), 100.0),
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

/// Collision prepass: brute-force proximity search, emits HitEvent messages.
fn collision_prepass(
    ants: Query<(Entity, &SimTransform), (With<AntMarker>, Without<Cooldown>)>,
    foods: Query<(&FoodState, &SimTransform), With<FoodMarker>>,
    food_entities: Res<EntityIndex<Food>>,
    mut hits: MessageWriter<AntFoodHit>,
) {
    // Snapshot loose food positions for proximity search
    let loose_food: Vec<(i32, DVec3)> = food_entities
        .entities
        .iter()
        .enumerate()
        .filter_map(|(idx, &entity)| {
            foods.get(entity).ok().and_then(|(f, sim_t)| {
                if f.is_loose {
                    Some((idx as i32, sim_t.translation))
                } else {
                    None
                }
            })
        })
        .collect();

    for (ant_entity, sim_t) in &ants {
        if let Some(&(food_idx, food_pos)) = loose_food
            .iter()
            .filter(|(_, fp)| (sim_t.translation - *fp).length_squared() < PICKUP_RADIUS * PICKUP_RADIUS)
            .min_by(|(_, a), (_, b)| {
                (sim_t.translation - *a)
                    .length_squared()
                    .partial_cmp(&(sim_t.translation - *b).length_squared())
                    .unwrap()
            })
        {
            hits.write(AntFoodHit::new(food_idx, ant_entity, food_pos));
        }
    }
}

/// Apply food mutations: reads FoodMutation messages, updates FoodState state.
fn apply_food_mutations(
    mut mutations: MessageReader<FoodMutation>,
    mut foods: Query<(&mut FoodState, &mut SimTransform), With<FoodMarker>>,
    food_entities: Res<EntityIndex<Food>>,
) {
    for mutation in mutations.read() {
        let idx = mutation.food_index as usize;
        if idx < food_entities.entities.len() {
            if let Ok((mut food, mut sim_t)) = foods.get_mut(food_entities.entities[idx]) {
                if mutation.decision == DECISION_PICK_UP {
                    food.is_loose = false;
                } else if mutation.decision == DECISION_DROP {
                    food.is_loose = true;
                    sim_t.translation = mutation.drop_position;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering sync systems
// ---------------------------------------------------------------------------

fn sync_ant_transforms(mut ants: Query<(&SimTransform, &mut Transform), With<AntMarker>>) {
    for (sim_t, mut transform) in &mut ants {
        transform.translation.x = sim_t.translation.x as f32 * SIM_TO_SCREEN;
        transform.translation.y = sim_t.translation.y as f32 * SIM_TO_SCREEN;
    }
}

fn sync_food_transforms(
    ants: Query<(&SimTransform, &Carrying), With<AntMarker>>,
    mut foods: Query<(&SimTransform, &mut Transform), With<FoodMarker>>,
    food_entities: Res<EntityIndex<Food>>,
) {
    // Build a map: food_index → ant position (for carried food)
    let mut carried_positions: Vec<Option<DVec3>> = vec![None; food_entities.entities.len()];
    for (sim_t, carry) in &ants {
        if carry.food_index >= 0 && (carry.food_index as usize) < carried_positions.len() {
            carried_positions[carry.food_index as usize] = Some(sim_t.translation);
        }
    }

    for (idx, &entity) in food_entities.entities.iter().enumerate() {
        if let Ok((sim_t, mut transform)) = foods.get_mut(entity) {
            if let Some(ant_pos) = carried_positions[idx] {
                // Carried: follow the ant, rendered above it
                transform.translation.x = ant_pos.x as f32 * SIM_TO_SCREEN;
                transform.translation.y = ant_pos.y as f32 * SIM_TO_SCREEN;
                transform.translation.z = Z_ANT + 1.0;
            } else {
                // Loose food: sync render transform from sim position
                transform.translation.x = sim_t.translation.x as f32 * SIM_TO_SCREEN;
                transform.translation.y = sim_t.translation.y as f32 * SIM_TO_SCREEN;
                transform.translation.z = Z_FOOD;
            }
        }
    }
}

fn sync_food_colors(mut foods: Query<(&FoodState, &mut Sprite), With<FoodMarker>>) {
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

/// Parsed from `--screenshot-at <frame>,<frame>,...` / `--exit-after <frame>`.
/// Lets us drive the app headfully and capture frames to disk for
/// autonomous inspection.
#[derive(Resource, Default)]
struct CaptureSchedule {
    /// Frames at which to spawn a Screenshot entity (sorted, deduped).
    frames: Vec<u64>,
    /// Frame after which to AppExit. None = run forever.
    exit_after: Option<u64>,
    /// Directory where screenshots are written.
    out_dir: String,
}

fn parse_capture_schedule() -> CaptureSchedule {
    let mut sched = CaptureSchedule {
        out_dir: "/tmp".to_string(),
        ..default()
    };
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot-at" => {
                if let Some(v) = args.get(i + 1) {
                    sched.frames = v
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u64>().ok())
                        .collect();
                    sched.frames.sort();
                    sched.frames.dedup();
                }
                i += 2;
            }
            "--exit-after" => {
                sched.exit_after = args.get(i + 1).and_then(|v| v.parse::<u64>().ok());
                i += 2;
            }
            "--out-dir" => {
                if let Some(v) = args.get(i + 1) {
                    sched.out_dir = v.clone();
                }
                i += 2;
            }
            _ => i += 1,
        }
    }
    sched
}

fn capture_scheduled_screenshots(
    mut commands: Commands,
    sched: Res<CaptureSchedule>,
    mut frame: Local<u64>,
    mut exit: MessageWriter<AppExit>,
) {
    let current = *frame;
    *frame += 1;

    if sched.frames.binary_search(&current).is_ok() {
        let path = format!("{}/standalone_frame_{:04}.png", sched.out_dir, current);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }

    if let Some(last) = sched.exit_after {
        // Give the screenshot one extra frame to finish capturing before we exit.
        if current >= last + 2 {
            exit.write(AppExit::Success);
        }
    }
}

fn main() {
    let sched = parse_capture_schedule();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gatherers Sim".into(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MovementPlugin::<SimTransform, PreviousTranslation, DesiredMovement>::default())
        .insert_resource(sched)
        .add_message::<AntFoodHit>()
        .add_message::<FoodMutation>()
        .add_systems(Startup, (setup_camera, spawn_entities))
        .add_systems(
            Update,
            (
                entity_boundary_reflect,
                collision_prepass,
                food_decision_system,
                apply_food_mutations,
                entity_cooldown,
                sync_ant_transforms,
                sync_food_transforms,
                sync_food_colors,
                capture_scheduled_screenshots,
            )
                .chain(),
        )
        .run();
}
