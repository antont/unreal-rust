//! Simulation config constants. Matches vivarium's Phase-1 config at
//! commit 43dd9df (boundary-force-field landing).

pub const WORLD_HALF_SIZE: f64 = 200.0;

pub const INSECT_COUNT: u32 = 200;
pub const INSECT_SPEED: f32 = 30.0;
pub const INSECT_WANDER_STRENGTH_DEG: f32 = 5.0;
pub const INSECT_RADIUS: f32 = 0.3;

pub const BOUNDARY_MARGIN: f64 = 40.0;
pub const BOUNDARY_FORCE: f64 = 5.0;
