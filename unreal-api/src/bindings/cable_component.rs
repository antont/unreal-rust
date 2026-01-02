#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct ACableActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub cable_component: UPtr<UCableComponent>,
}
impl ACableActor {}
#[repr(C, align(16))]
pub struct UCableComponent {
    #[doc(hidden)]
    __padding_1576: [u8; 1576],
    pub b_attach_start: bool,
    pub b_attach_end: bool,
    #[doc(hidden)]
    __padding_1648: [u8; 64],
    pub end_location: crate::bindings::core_u_object::FVector,
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
    pub cable_force: crate::bindings::core_u_object::FVector,
    pub cable_gravity_scale: f32,
    pub cable_width: f32,
    pub num_sides: i32,
    pub tile_material: f32,
    pub b_reset_after_teleport: bool,
    pub teleport_distance_threshold: f32,
    pub teleport_rotation_threshold: f32,
    pub b_teleport_after_reattach: bool,
    __padding_end: [u8; 307],
}
impl UCableComponent {}
