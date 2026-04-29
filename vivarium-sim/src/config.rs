//! Simulation config constants. Matches vivarium's Phase-1 config at
//! commit 43dd9df (boundary-force-field landing).

pub const WORLD_HALF_SIZE: f64 = 200.0;

pub const INSECT_COUNT: u32 = 200;
pub const INSECT_SPEED: f32 = 30.0;
/// Per-axis velocity perturbation magnitude (units/sec). Vivarium's value
/// at commit 43dd9df is 5.0 — naming in the original called it "degrees"
/// but the implementation uses it as a raw Vec3 perturbation magnitude.
pub const INSECT_WANDER_STRENGTH: f32 = 5.0;
pub const INSECT_RADIUS: f32 = 0.3;

pub const BOUNDARY_MARGIN: f64 = 40.0;
pub const BOUNDARY_FORCE: f64 = 5.0;

// Birds — Phase 2a (wander + flocking, no hunt yet).
pub const BIRD_COUNT: u32 = 20;
pub const BIRD_SPEED: f32 = 60.0;
pub const BIRD_RADIUS: f32 = 1.0;
/// Per-frame wander perturbation magnitude (same shape as
/// `INSECT_WANDER_STRENGTH` — bird wander reuses brownian-style rotation).
pub const BIRD_WANDER_STRENGTH: f32 = 15.0;

// Flocking tunables (values lifted from vivarium commit 1b6d1f5).
pub const FLOCK_NEIGHBOR_RADIUS: f64 = 40.0;
pub const SEPARATION_DISTANCE: f64 = 10.0;
pub const SEPARATION_WEIGHT: f64 = 2.0;
pub const ALIGNMENT_WEIGHT: f64 = 1.0;
pub const COHESION_WEIGHT: f64 = 1.0;

// Hunt + eat — Phase 2b. Values lifted from vivarium's src/config.rs.
pub const BIRD_SIGHT_RANGE: f64 = 80.0;
/// Forward-cone half-angle in radians (~40 degrees).
pub const BIRD_SIGHT_HALF_ANGLE: f64 = 0.7;
pub const BIRD_EATING_DISTANCE: f64 = 3.0;
pub const HUNT_CIRCLE_DURATION: f64 = 1.5;
pub const HUNT_CIRCLE_RADIUS: f64 = 25.0;
pub const HUNT_DIVE_SPEED_MULT: f64 = 1.8;
