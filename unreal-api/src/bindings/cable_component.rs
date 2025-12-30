#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct ACableActor {
    pub cable_component: UPtr<UCableComponent>,
}
pub struct UCableComponent {
    pub b_attach_start: bool,
    pub b_attach_end: bool,
    pub attach_end_to: FComponentReference,
    pub attach_end_to_socket_name: FName,
    pub end_location: FVector,
    pub cable_length: f32,
    pub num_segments: i32,
    pub substep_time: f32,
    pub solver_iterations: i32,
    pub b_enable_stiffness: bool,
    pub b_use_substepping: bool,
    pub b_skip_cable_update_when_not_visible: bool,
    pub b_skip_cable_update_when_not_owner_recently_rendered: bool,
    pub b_enable_collision: bool,
    pub collision_friction: f32,
    pub cable_force: FVector,
    pub cable_gravity_scale: f32,
    pub cable_width: f32,
    pub num_sides: i32,
    pub tile_material: f32,
    pub b_reset_after_teleport: bool,
    pub teleport_distance_threshold: f32,
    pub teleport_rotation_threshold: f32,
    pub b_teleport_after_reattach: bool,
}
