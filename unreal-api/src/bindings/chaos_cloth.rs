#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_WIND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_SCALE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_CLAMPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_PRESSURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_LINEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_BUCKLING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT_LINEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_GRAVITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_DAMPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_COLLISION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_BACKSTOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE_LINEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_SET_AERODYNAMICS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_CLOTHING_INTERACTOR_RESET_AND_TELEPORT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosClothingInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWind"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_WIND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVelocityScale"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_SCALE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVelocityClamps"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_CLAMPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPressure"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_PRESSURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialLinear"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_LINEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialBuckling"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_BUCKLING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterial"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLongRangeAttachmentLinear"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT_LINEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLongRangeAttachment"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGravity"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_GRAVITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamping"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_DAMPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollision"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_COLLISION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackstop"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_BACKSTOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDriveLinear"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE_LINEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDrive"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAerodynamics"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_SET_AERODYNAMICS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAndTeleport"),
            &raw mut U_CHAOS_CLOTHING_INTERACTOR_RESET_AND_TELEPORT,
        );
    }
}
#[repr(C, align(8))]
pub struct UChaosClothConfig {
    __padding_end: [u8; 424],
}
impl UChaosClothConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosClothConfig")
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
#[repr(C, align(8))]
pub struct UChaosClothSharedSimConfig {
    __padding_end: [u8; 120],
}
impl UChaosClothSharedSimConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosClothSharedSimConfig")
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
#[repr(C, align(8))]
pub struct UChaosClothingSimulationFactory {
    __padding_end: [u8; 48],
}
impl UChaosClothingSimulationFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosClothingSimulationFactory")
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
#[repr(C, align(8))]
pub struct UChaosClothingInteractor {
    __padding_end: [u8; 88],
}
impl UChaosClothingInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosClothingInteractor")
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
    pub fn set_wind(
        &mut self,
        drag: crate::bindings::core_u_object::FVector2D,
        lift: crate::bindings::core_u_object::FVector2D,
        air_density: f32,
        wind_velocity: crate::bindings::core_u_object::FVector,
        outer_drag: crate::bindings::core_u_object::FVector2D,
        outer_lift: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_WIND,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &drag,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lift,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &air_density,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wind_velocity,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outer_drag,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outer_lift,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_WIND,
                __buffer,
            )
        };
    }
    pub fn set_velocity_scale(
        &mut self,
        linear_velocity_scale: crate::bindings::core_u_object::FVector,
        angular_velocity_scale: f32,
        fictitious_angular_scale: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_SCALE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &linear_velocity_scale,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &angular_velocity_scale,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fictitious_angular_scale,
                __buffer.add(28).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_SCALE,
                __buffer,
            )
        };
    }
    pub fn set_velocity_clamps(
        &mut self,
        b_enable_linear_velocity_clamping: bool,
        max_linear_velocity: crate::bindings::core_u_object::FVector,
        b_enable_linear_acceleration_clamping: bool,
        max_linear_acceleration: crate::bindings::core_u_object::FVector,
        b_enable_angular_velocity_clamping: bool,
        max_angular_velocity: f32,
        b_enable_angular_acceleration_clamping: bool,
        max_angular_acceleration: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_CLAMPS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_linear_velocity_clamping,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_linear_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_linear_acceleration_clamping,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_linear_acceleration,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_angular_velocity_clamping,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_angular_velocity,
                __buffer.add(68).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_angular_acceleration_clamping,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_angular_acceleration,
                __buffer.add(76).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_VELOCITY_CLAMPS,
                __buffer,
            )
        };
    }
    pub fn set_pressure(&mut self, pressure: crate::bindings::core_u_object::FVector2D) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_PRESSURE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pressure,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_PRESSURE,
                __buffer,
            )
        };
    }
    pub fn set_material_linear(
        &mut self,
        edge_stiffness: f32,
        bending_stiffness: f32,
        area_stiffness: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_LINEAR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_stiffness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bending_stiffness,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &area_stiffness,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_LINEAR,
                __buffer,
            )
        };
    }
    pub fn set_material_buckling(
        &mut self,
        buckling_ratio: crate::bindings::core_u_object::FVector2D,
        buckling_stiffness: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_BUCKLING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &buckling_ratio,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &buckling_stiffness,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL_BUCKLING,
                __buffer,
            )
        };
    }
    pub fn set_material(
        &mut self,
        edge_stiffness: crate::bindings::core_u_object::FVector2D,
        bending_stiffness: crate::bindings::core_u_object::FVector2D,
        area_stiffness: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edge_stiffness,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bending_stiffness,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &area_stiffness,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_MATERIAL,
                __buffer,
            )
        };
    }
    pub fn set_long_range_attachment_linear(
        &mut self,
        tether_stiffness: f32,
        tether_scale: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT_LINEAR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tether_stiffness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tether_scale,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT_LINEAR,
                __buffer,
            )
        };
    }
    pub fn set_long_range_attachment(
        &mut self,
        tether_stiffness: crate::bindings::core_u_object::FVector2D,
        tether_scale: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tether_stiffness,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tether_scale,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_LONG_RANGE_ATTACHMENT,
                __buffer,
            )
        };
    }
    pub fn set_gravity(
        &mut self,
        gravity_scale: f32,
        b_is_gravity_overridden: bool,
        gravity_override: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_GRAVITY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gravity_scale,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_gravity_overridden,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gravity_override,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_GRAVITY,
                __buffer,
            )
        };
    }
    pub fn set_damping(
        &mut self,
        damping_coefficient: f32,
        local_damping_coefficient: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_DAMPING,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &damping_coefficient,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_damping_coefficient,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_DAMPING,
                __buffer,
            )
        };
    }
    pub fn set_collision(
        &mut self,
        collision_thickness: f32,
        friction_coefficient: f32,
        b_use_ccd: bool,
        self_collision_thickness: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_COLLISION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_thickness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &friction_coefficient,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_use_ccd, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &self_collision_thickness,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_COLLISION,
                __buffer,
            )
        };
    }
    pub fn set_backstop(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_BACKSTOP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_BACKSTOP,
                __buffer,
            )
        };
    }
    pub fn set_anim_drive_linear(&mut self, anim_drive_stiffness: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE_LINEAR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_drive_stiffness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE_LINEAR,
                __buffer,
            )
        };
    }
    pub fn set_anim_drive(
        &mut self,
        anim_drive_stiffness: crate::bindings::core_u_object::FVector2D,
        anim_drive_damping: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_drive_stiffness,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_drive_damping,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_ANIM_DRIVE,
                __buffer,
            )
        };
    }
    pub fn set_aerodynamics(
        &mut self,
        drag_coefficient: f32,
        lift_coefficient: f32,
        wind_velocity: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_AERODYNAMICS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &drag_coefficient,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lift_coefficient,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wind_velocity,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_SET_AERODYNAMICS,
                __buffer,
            )
        };
    }
    pub fn reset_and_teleport(&mut self, b_reset: bool, b_teleport: bool) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_RESET_AND_TELEPORT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_reset, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_teleport,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_cloth::U_CHAOS_CLOTHING_INTERACTOR_RESET_AND_TELEPORT,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UChaosClothingSimulationInteractor {
    __padding_end: [u8; 184],
}
impl UChaosClothingSimulationInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosClothingSimulationInteractor")
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
#[repr(transparent)]
pub struct EChaosClothTetherMode(pub u8);
impl EChaosClothTetherMode {
    pub const FAST_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(0);
    pub const ACCURATE_TETHER_FAST_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        1,
    );
    pub const ACCURATE_TETHER_ACCURATE_LENGTH: EChaosClothTetherMode = EChaosClothTetherMode(
        2,
    );
    pub const MAX_CHAOS_CLOTH_TETHER_MODE: EChaosClothTetherMode = EChaosClothTetherMode(
        3,
    );
}
