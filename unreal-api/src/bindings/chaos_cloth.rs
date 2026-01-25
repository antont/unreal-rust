#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_chaos_clothing_interactor_set_wind: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_velocity_scale: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_velocity_clamps: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_pressure: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_material_linear: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_material_buckling: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_material: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_long_range_attachment_linear: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_long_range_attachment: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_damping: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_collision: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_backstop: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_anim_drive_linear: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_anim_drive: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_set_aerodynamics: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_clothing_interactor_reset_and_teleport: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chaos_clothing_interactor_set_wind: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_velocity_scale: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_velocity_clamps: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_pressure: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_material_linear: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_material_buckling: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_material: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_long_range_attachment_linear: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_long_range_attachment: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_gravity: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_damping: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_collision: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_backstop: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_anim_drive_linear: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_anim_drive: std::ptr::null_mut(),
            u_chaos_clothing_interactor_set_aerodynamics: std::ptr::null_mut(),
            u_chaos_clothing_interactor_reset_and_teleport: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosClothingInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWind"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_wind,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVelocityScale"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_velocity_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVelocityClamps"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_velocity_clamps,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPressure"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_pressure,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialLinear"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_material_linear,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialBuckling"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_material_buckling,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterial"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLongRangeAttachmentLinear"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_clothing_interactor_set_long_range_attachment_linear,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLongRangeAttachment"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_clothing_interactor_set_long_range_attachment,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGravity"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_gravity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDamping"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_damping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCollision"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_collision,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackstop"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_backstop,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDriveLinear"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_anim_drive_linear,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDrive"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_anim_drive,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAerodynamics"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_set_aerodynamics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAndTeleport"),
            &raw mut __FUNCTION_PTRS.u_chaos_clothing_interactor_reset_and_teleport,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_wind,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_wind,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_velocity_scale,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_velocity_scale,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_velocity_clamps,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_velocity_clamps,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_pressure,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_pressure,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material_buckling,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material_buckling,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_material,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_long_range_attachment_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_long_range_attachment_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_long_range_attachment,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_long_range_attachment,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_gravity,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_gravity,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_damping,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_damping,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_collision,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_collision,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_backstop,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_backstop,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_anim_drive_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_anim_drive_linear,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_anim_drive,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_anim_drive,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_aerodynamics,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_set_aerodynamics,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_reset_and_teleport,
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
                crate::bindings::chaos_cloth::__FUNCTION_PTRS
                    .u_chaos_clothing_interactor_reset_and_teleport,
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
