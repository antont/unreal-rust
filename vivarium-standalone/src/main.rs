//! Standalone Bevy harness for the vivarium simulation.
//!
//! Phase 1a: 200 insects spawned with random velocities, integrated by
//! `bevy_mass::MovementPlugin`. No brownian motion or boundary force yet —
//! insects fly off in straight lines. 3D scene, fixed camera.
//!
//! CLI (mirrors `gatherers-standalone`):
//!   --deterministic-clock          fixed 1/60 s dt (pixel-reproducible runs)
//!   --screenshot-at F1,F2,...      capture frames to PNG
//!   --exit-after N                 AppExit after frame N (+10 slack)
//!   --out-dir DIR                  screenshot output dir (default /tmp)

use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy_mass::MovementPlugin;
use glam::DVec3;
use vivarium_sim::components::{
    Insect, PreviousTranslation, Transform as SimTransform, Velocity,
};
use vivarium_sim::config::{INSECT_COUNT, INSECT_RADIUS, INSECT_SPEED, WORLD_HALF_SIZE};

const INSECT_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);

#[derive(Resource, Default)]
struct CaptureSchedule {
    frames: Vec<u64>,
    exit_after: Option<u64>,
    out_dir: String,
}

#[derive(Resource, Default)]
struct FrameCounter(u64);

fn parse_args() -> (CaptureSchedule, bool) {
    let mut sched = CaptureSchedule { out_dir: "/tmp".to_string(), ..default() };
    let mut deterministic = false;
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot-at" => {
                if let Some(v) = args.get(i + 1) {
                    sched.frames = v.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    sched.frames.sort();
                    sched.frames.dedup();
                }
                i += 2;
            }
            "--exit-after" => {
                sched.exit_after = args.get(i + 1).and_then(|v| v.parse().ok());
                i += 2;
            }
            "--out-dir" => {
                if let Some(v) = args.get(i + 1) {
                    sched.out_dir = v.clone();
                }
                i += 2;
            }
            "--deterministic-clock" => {
                deterministic = true;
                i += 1;
            }
            _ => i += 1,
        }
    }
    (sched, deterministic)
}

fn main() {
    let (capture, deterministic) = parse_args();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin::<SimTransform, PreviousTranslation, Velocity>::default())
        .insert_resource(capture)
        .init_resource::<FrameCounter>()
        .add_systems(Startup, (setup_scene, spawn_insects).chain())
        .add_systems(Update, (sync_render_transforms, capture_scheduled_screenshots));

    if deterministic {
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
    }

    app.run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 300.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    let mesh = meshes.add(Sphere::new(INSECT_RADIUS));
    let material = materials.add(StandardMaterial {
        base_color: INSECT_COLOR,
        unlit: true,
        ..default()
    });
    commands.insert_resource(InsectMesh(mesh));
    commands.insert_resource(InsectMaterial(material));
}

#[derive(Resource)]
struct InsectMesh(Handle<Mesh>);

#[derive(Resource)]
struct InsectMaterial(Handle<StandardMaterial>);

fn spawn_insects(
    mut commands: Commands,
    mesh: Res<InsectMesh>,
    material: Res<InsectMaterial>,
) {
    // LCG RNG matching gatherers' seeding style so spawns are reproducible.
    let mut seed: u64 = 42;
    let mut rng = || -> f64 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as f64 / (1u64 << 31) as f64
    };

    for _ in 0..INSECT_COUNT {
        let pos = DVec3::new(
            (rng() * 2.0 - 1.0) * WORLD_HALF_SIZE,
            (rng() * 2.0 - 1.0) * WORLD_HALF_SIZE,
            (rng() * 2.0 - 1.0) * WORLD_HALF_SIZE,
        );
        let theta = rng() * std::f64::consts::TAU;
        let phi = (rng() * 2.0 - 1.0).acos();
        let dir = DVec3::new(
            phi.sin() * theta.cos(),
            phi.sin() * theta.sin(),
            phi.cos(),
        );
        commands.spawn((
            Insect,
            SimTransform::from_translation(pos),
            PreviousTranslation { value: pos },
            Velocity::new(dir, INSECT_SPEED),
            Mesh3d(mesh.0.clone()),
            MeshMaterial3d(material.0.clone()),
            Transform::from_translation(Vec3::new(
                pos.x as f32,
                pos.y as f32,
                pos.z as f32,
            )),
        ));
    }
}

fn sync_render_transforms(
    mut q: Query<(&SimTransform, &mut Transform), With<Insect>>,
) {
    for (sim_t, mut render_t) in &mut q {
        render_t.translation = Vec3::new(
            sim_t.translation.x as f32,
            sim_t.translation.y as f32,
            sim_t.translation.z as f32,
        );
    }
}

fn capture_scheduled_screenshots(
    mut commands: Commands,
    sched: Res<CaptureSchedule>,
    mut frame: ResMut<FrameCounter>,
    mut exit: MessageWriter<AppExit>,
) {
    let current = frame.0;
    frame.0 += 1;

    if sched.frames.binary_search(&current).is_ok() {
        let path = format!("{}/vivarium_frame_{:04}.png", sched.out_dir, current);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }

    if let Some(last) = sched.exit_after {
        if current >= last + 10 {
            exit.write(AppExit::Success);
        }
    }
}
