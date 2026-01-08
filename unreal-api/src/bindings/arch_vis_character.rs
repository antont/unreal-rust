#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
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
}
impl AArchVisCharacter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AArchVisCharacter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
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
impl UArchVisCharMovementComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArchVisCharMovementComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
