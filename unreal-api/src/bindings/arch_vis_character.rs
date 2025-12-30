#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct AArchVisCharacter {
    pub look_up_axis_name: FString,
    pub look_up_at_rate_axis_name: FString,
    pub turn_axis_name: FString,
    pub turn_at_rate_axis_name: FString,
    pub move_forward_axis_name: FString,
    pub move_right_axis_name: FString,
    pub mouse_sensitivity_scale_pitch: f32,
    pub mouse_sensitivity_scale_yaw: f32,
}
pub struct UArchVisCharMovementComponent {
    pub rotational_acceleration: FRotator,
    pub rotational_deceleration: FRotator,
    pub max_rotational_velocity: FRotator,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub walking_friction: f32,
    pub walking_speed: f32,
    pub walking_acceleration: f32,
}
