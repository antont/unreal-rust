#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct AArchVisCharacter {
    #[doc(hidden)]
    __padding_2112: [u8; 2112],
    pub look_up_axis_name: FString,
    pub look_up_at_rate_axis_name: FString,
    pub turn_axis_name: FString,
    pub turn_at_rate_axis_name: FString,
    pub move_forward_axis_name: FString,
    pub move_right_axis_name: FString,
    pub mouse_sensitivity_scale_pitch: f32,
    pub mouse_sensitivity_scale_yaw: f32,
    __padding_end: [u8; 8],
}
impl AArchVisCharacter {}
#[repr(C, align(16))]
pub struct UArchVisCharMovementComponent {
    #[doc(hidden)]
    __padding_4120: [u8; 4120],
    pub rotational_acceleration: crate::bindings::core_u_object::FRotator,
    pub rotational_deceleration: crate::bindings::core_u_object::FRotator,
    pub max_rotational_velocity: crate::bindings::core_u_object::FRotator,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub walking_friction: f32,
    pub walking_speed: f32,
    pub walking_acceleration: f32,
    __padding_end: [u8; 60],
}
impl UArchVisCharMovementComponent {}
