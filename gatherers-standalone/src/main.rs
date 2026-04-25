use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_mass::{MovementPlugin, EntityIndex};
use gatherers_sim::components::{
    Transform as SimTransform, PreviousTranslation, DesiredMovement,
    Cooldown, Carrying, Behavior, FoodState, Food, Ant,
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

// Ant:food ratio must stay around 1:4 — the sim only has a useful steady
// state when food is in surplus. With more ants than food, every ant picks
// something up and the sim freezes into a static carpet; nothing drops
// because there's no loose food left to trigger a drop-and-retarget. With
// plenty of food, ants keep picking up, wandering, and dropping in an
// observable loop. 800×800 window can't accommodate the UE-scale counts
// (10k ants / 40k food) — these give a legible simulation at the same ratio.
const DEFAULT_ANT_COUNT: u32 = 150;
const DEFAULT_FOOD_COUNT: u32 = 600;
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

/// Entity counts (settable via `--stress N`; N applies to ants, food = 4*N).
#[derive(Resource, Clone, Copy)]
struct SimSize {
    ants: u32,
    food: u32,
}

impl Default for SimSize {
    fn default() -> Self {
        Self { ants: DEFAULT_ANT_COUNT, food: DEFAULT_FOOD_COUNT }
    }
}

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

fn spawn_entities(mut commands: Commands, size: Res<SimSize>) {
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
    let mut food_entities = Vec::with_capacity(size.food as usize);
    for _ in 0..size.food {
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
    for i in 0..size.ants {
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
            Ant,
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
    // Snapshot loose food (index, entity, position) for proximity search.
    let loose_food: Vec<(i32, Entity, DVec3)> = food_entities
        .entities
        .iter()
        .enumerate()
        .filter_map(|(idx, &entity)| {
            foods.get(entity).ok().and_then(|(f, sim_t)| {
                if f.is_loose {
                    Some((idx as i32, entity, sim_t.translation))
                } else {
                    None
                }
            })
        })
        .collect();

    for (ant_entity, sim_t) in &ants {
        if let Some(&(food_idx, food_entity, food_pos)) = loose_food
            .iter()
            .filter(|(_, _, fp)| (sim_t.translation - *fp).length_squared() < PICKUP_RADIUS * PICKUP_RADIUS)
            .min_by(|(_, _, a), (_, _, b)| {
                (sim_t.translation - *a)
                    .length_squared()
                    .partial_cmp(&(sim_t.translation - *b).length_squared())
                    .unwrap()
            })
        {
            hits.write(AntFoodHit::new(food_idx, food_entity, ant_entity, food_pos));
        }
    }
}

/// Apply food mutations: reads FoodMutation messages, updates FoodState state.
fn apply_food_mutations(
    mut mutations: MessageReader<FoodMutation>,
    mut foods: Query<(&mut FoodState, &mut SimTransform), With<FoodMarker>>,
) {
    for mutation in mutations.read() {
        if let Ok((mut food, mut sim_t)) = foods.get_mut(mutation.food_entity) {
            if mutation.decision == DECISION_PICK_UP {
                food.is_loose = false;
            } else if mutation.decision == DECISION_DROP {
                food.is_loose = true;
                sim_t.translation = mutation.drop_position;
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
        if carry.is_carrying() && (carry.food_index as usize) < carried_positions.len() {
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
    /// When true, pause `Time<Virtual>` and advance it by a fixed 1/60s step
    /// at the head of Update. Makes `time.delta_secs()` in the sim
    /// deterministic frame-to-frame so screenshots are reproducible.
    deterministic_clock: bool,
}

/// Other runtime modes parsed from CLI args.
#[derive(Default, Clone, Copy)]
struct RunOptions {
    /// Skip rendering (use MinimalPlugins). Rendering syncs run only when false.
    headless: bool,
    /// Time each sim system and print a summary on exit.
    profile: bool,
}

struct ParsedArgs {
    capture: CaptureSchedule,
    size: SimSize,
    run: RunOptions,
}

fn parse_args() -> ParsedArgs {
    let mut capture = CaptureSchedule {
        out_dir: "/tmp".to_string(),
        ..default()
    };
    let mut size = SimSize::default();
    let mut run = RunOptions::default();
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot-at" => {
                if let Some(v) = args.get(i + 1) {
                    capture.frames = v
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u64>().ok())
                        .collect();
                    capture.frames.sort();
                    capture.frames.dedup();
                }
                i += 2;
            }
            "--exit-after" => {
                capture.exit_after = args.get(i + 1).and_then(|v| v.parse::<u64>().ok());
                i += 2;
            }
            "--out-dir" => {
                if let Some(v) = args.get(i + 1) {
                    capture.out_dir = v.clone();
                }
                i += 2;
            }
            "--deterministic-clock" => {
                capture.deterministic_clock = true;
                i += 1;
            }
            "--stress" => {
                // N ants, 4*N food (keeps 1:4 ratio invariant).
                if let Some(n) = args.get(i + 1).and_then(|v| v.parse::<u32>().ok()) {
                    size = SimSize { ants: n, food: n.saturating_mul(4) };
                }
                i += 2;
            }
            "--headless" => {
                run.headless = true;
                i += 1;
            }
            "--profile" => {
                run.profile = true;
                i += 1;
            }
            _ => i += 1,
        }
    }
    ParsedArgs { capture, size, run }
}

/// Tracks the current frame for screenshot scheduling + exit trigger.
/// Needs to be a Resource (not Local) so profile mode can read the final
/// value after app.run() returns.
#[derive(Resource, Default)]
struct FrameCounter(u64);

fn capture_scheduled_screenshots(
    mut commands: Commands,
    sched: Res<CaptureSchedule>,
    mut frame: ResMut<FrameCounter>,
    mut exit: MessageWriter<AppExit>,
) {
    let current = frame.0;
    frame.0 += 1;

    if sched.frames.binary_search(&current).is_ok() {
        let path = format!("{}/standalone_frame_{:04}.png", sched.out_dir, current);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }

    if let Some(last) = sched.exit_after {
        // Screenshot capture + save is async through observers and the render
        // pipeline, so give it several frames of slack after the last
        // requested frame. 10 is comfortably over the 2–3 a screenshot
        // usually needs and costs nothing at CI scale.
        if current >= last + 10 {
            exit.write(AppExit::Success);
        }
    }
}

/// Headless/profile mode has no screenshot system; this is the frame driver
/// for those modes (increments counter, handles exit).
fn tick_frame_headless(
    mut frame: ResMut<FrameCounter>,
    sched: Res<CaptureSchedule>,
    mut exit: MessageWriter<AppExit>,
) {
    let current = frame.0;
    frame.0 += 1;
    if let Some(last) = sched.exit_after {
        if current >= last + 10 {
            exit.write(AppExit::Success);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-system profiling
//
// The whole sim chain is `.chain()`-serialized for determinism anyway, so
// bracketing each target system with tiny start/stop systems measures the
// wall-clock cost of that system without disrupting scheduler parallelism
// (there is none to disrupt).
// ---------------------------------------------------------------------------

/// Accumulated wall-clock time per system label. Printed on Drop.
#[derive(Resource, Default)]
struct SystemTimings {
    totals_ns: std::collections::HashMap<&'static str, (u128, u64)>,
}

impl SystemTimings {
    fn record(&mut self, label: &'static str, elapsed: std::time::Duration) {
        let entry = self.totals_ns.entry(label).or_insert((0, 0));
        entry.0 += elapsed.as_nanos();
        entry.1 += 1;
    }
}

/// Carries the current timer's start `Instant` from start_timer → stop_timer.
/// Safe because the systems bracket each other inside `.chain()` — only one
/// timer is live at a time.
#[derive(Resource, Default)]
struct TimerSlot(Option<std::time::Instant>);

fn start_timer(mut slot: ResMut<TimerSlot>) {
    slot.0 = Some(std::time::Instant::now());
}

fn stop_timer_factory(
    label: &'static str,
) -> impl Fn(Res<TimerSlot>, ResMut<SystemTimings>) + Send + Sync + 'static {
    move |slot: Res<TimerSlot>, mut timings: ResMut<SystemTimings>| {
        if let Some(start) = slot.0 {
            timings.record(label, start.elapsed());
        }
    }
}

fn report_timings(timings: &SystemTimings, frames: u64) {
    if timings.totals_ns.is_empty() {
        return;
    }
    let frames = frames.max(1);
    eprintln!("\n=== System timings over {} frames ===", frames);
    eprintln!("{:<28} {:>10} {:>12}", "system", "total_ms", "avg_us/frame");
    let mut rows: Vec<_> = timings.totals_ns.iter().collect();
    rows.sort_by_key(|(_, (ns, _))| std::cmp::Reverse(*ns));
    let mut grand_total_ns: u128 = 0;
    for (label, (total_ns, _runs)) in &rows {
        let total_ms = (*total_ns as f64) / 1_000_000.0;
        let avg_us = (*total_ns as f64) / 1_000.0 / frames as f64;
        eprintln!("{:<28} {:>10.2} {:>12.1}", label, total_ms, avg_us);
        grand_total_ns += *total_ns;
    }
    let grand_ms = (grand_total_ns as f64) / 1_000_000.0;
    let grand_avg_us = (grand_total_ns as f64) / 1_000.0 / frames as f64;
    eprintln!("{:-<54}", "");
    eprintln!("{:<28} {:>10.2} {:>12.1}", "TOTAL (sim systems)", grand_ms, grand_avg_us);
}

fn main() {
    let ParsedArgs { capture, size, run } = parse_args();
    let deterministic = capture.deterministic_clock;
    let headless = run.headless;
    let profile = run.profile;

    eprintln!(
        "gatherers-standalone: {} ants, {} food, {}{}{}",
        size.ants,
        size.food,
        if headless { "headless" } else { "rendered" },
        if deterministic { ", deterministic clock" } else { "" },
        if profile { ", profiling" } else { "" },
    );

    let mut app = App::new();
    if headless {
        // Drop ScheduleRunnerPlugin — we drive Update manually in profile
        // mode so we can read timing resources after the loop.
        app.add_plugins(
            bevy::MinimalPlugins
                .build()
                .disable::<bevy::app::ScheduleRunnerPlugin>(),
        );
    } else {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gatherers Sim".into(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }));
    }
    app.add_plugins(MovementPlugin::<SimTransform, PreviousTranslation, DesiredMovement>::default())
        .insert_resource(capture)
        .insert_resource(size)
        .init_resource::<FrameCounter>()
        .add_message::<AntFoodHit>()
        .add_message::<FoodMutation>();
    if deterministic {
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
    }

    // Startup depends on whether we render
    if headless {
        app.add_systems(Startup, spawn_entities);
    } else {
        app.add_systems(Startup, (setup_camera, spawn_entities));
    }

    // Build the Update chain. With --profile, each sim system is bracketed
    // by start/stop timer systems; otherwise run the plain chain.
    if profile && !headless {
        eprintln!("--profile requires --headless (rendering pollutes the timings). Exiting.");
        return;
    }

    use bevy::ecs::schedule::IntoScheduleConfigs;
    match (profile, headless) {
        (true, true) => {
            app.init_resource::<SystemTimings>()
                .init_resource::<TimerSlot>();
            app.add_systems(Update, (
                start_timer, entity_boundary_reflect, stop_timer_factory("entity_boundary_reflect"),
                start_timer, collision_prepass, stop_timer_factory("collision_prepass"),
                start_timer, food_decision_system, stop_timer_factory("food_decision_system"),
                start_timer, apply_food_mutations, stop_timer_factory("apply_food_mutations"),
                start_timer, entity_cooldown, stop_timer_factory("entity_cooldown"),
                tick_frame_headless,
            ).chain());
        }
        (false, true) => {
            app.add_systems(Update, (
                entity_boundary_reflect,
                collision_prepass,
                food_decision_system,
                apply_food_mutations,
                entity_cooldown,
                tick_frame_headless,
            ).chain());
        }
        (false, false) => {
            app.add_systems(Update, (
                entity_boundary_reflect,
                collision_prepass,
                food_decision_system,
                apply_food_mutations,
                entity_cooldown,
                sync_ant_transforms,
                sync_food_transforms,
                sync_food_colors,
                capture_scheduled_screenshots,
            ).chain());
        }
        (true, false) => unreachable!(),
    }

    if headless {
        // Manual drive loop: finalize plugins, run Startup once, then loop
        // Update until AppExit is written. This keeps the app in scope so we
        // can read FrameCounter + SystemTimings after the loop (unlike
        // `app.run()`, which mem::swaps the app into an internal runner).
        app.finish();
        app.cleanup();
        let t0 = std::time::Instant::now();
        loop {
            app.update();
            if app.should_exit().is_some() {
                break;
            }
        }
        let wall = t0.elapsed();

        if profile {
            let world = app.world();
            let frames = world.get_resource::<FrameCounter>().map(|f| f.0).unwrap_or(0);
            if let Some(timings) = world.get_resource::<SystemTimings>() {
                report_timings(timings, frames);
            }
            eprintln!(
                "Wall time: {:.2}s over {} frames ({:.2} ms/frame)",
                wall.as_secs_f64(),
                frames,
                wall.as_secs_f64() * 1000.0 / frames.max(1) as f64,
            );
        }
    } else {
        app.run();
    }
}
