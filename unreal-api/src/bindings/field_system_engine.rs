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
pub struct FunctionPtrs {
    pub u_field_system_component_reset_field_system: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_remove_persistent_fields: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_uniform_vector_falloff_force: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_strain_field: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_stay_dynamic_field: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_radial_vector_falloff_force: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_radial_force: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_physics_field: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_apply_linear_force: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_add_persistent_field: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_component_add_field_command: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_meta_data_iteration_set_meta_data_iteration: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_meta_data_processing_resolution_set_meta_dataa_processing_resolution_type: *mut crate::ffi::UFunctionOpague,
    pub u_field_system_meta_data_filter_set_meta_data_filter_type: *mut crate::ffi::UFunctionOpague,
    pub u_uniform_integer_set_uniform_integer: *mut crate::ffi::UFunctionOpague,
    pub u_radial_int_mask_set_radial_int_mask: *mut crate::ffi::UFunctionOpague,
    pub u_uniform_scalar_set_uniform_scalar: *mut crate::ffi::UFunctionOpague,
    pub u_wave_scalar_set_wave_scalar: *mut crate::ffi::UFunctionOpague,
    pub u_radial_falloff_set_radial_falloff: *mut crate::ffi::UFunctionOpague,
    pub u_plane_falloff_set_plane_falloff: *mut crate::ffi::UFunctionOpague,
    pub u_box_falloff_set_box_falloff: *mut crate::ffi::UFunctionOpague,
    pub u_noise_field_set_noise_field: *mut crate::ffi::UFunctionOpague,
    pub u_uniform_vector_set_uniform_vector: *mut crate::ffi::UFunctionOpague,
    pub u_radial_vector_set_radial_vector: *mut crate::ffi::UFunctionOpague,
    pub u_random_vector_set_random_vector: *mut crate::ffi::UFunctionOpague,
    pub u_operator_field_set_operator_field: *mut crate::ffi::UFunctionOpague,
    pub u_to_integer_field_set_to_integer_field: *mut crate::ffi::UFunctionOpague,
    pub u_to_float_field_set_to_float_field: *mut crate::ffi::UFunctionOpague,
    pub u_culling_field_set_culling_field: *mut crate::ffi::UFunctionOpague,
    pub u_return_results_terminal_set_return_results_terminal: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_field_system_component_reset_field_system: std::ptr::null_mut(),
            u_field_system_component_remove_persistent_fields: std::ptr::null_mut(),
            u_field_system_component_apply_uniform_vector_falloff_force: std::ptr::null_mut(),
            u_field_system_component_apply_strain_field: std::ptr::null_mut(),
            u_field_system_component_apply_stay_dynamic_field: std::ptr::null_mut(),
            u_field_system_component_apply_radial_vector_falloff_force: std::ptr::null_mut(),
            u_field_system_component_apply_radial_force: std::ptr::null_mut(),
            u_field_system_component_apply_physics_field: std::ptr::null_mut(),
            u_field_system_component_apply_linear_force: std::ptr::null_mut(),
            u_field_system_component_add_persistent_field: std::ptr::null_mut(),
            u_field_system_component_add_field_command: std::ptr::null_mut(),
            u_field_system_meta_data_iteration_set_meta_data_iteration: std::ptr::null_mut(),
            u_field_system_meta_data_processing_resolution_set_meta_dataa_processing_resolution_type: std::ptr::null_mut(),
            u_field_system_meta_data_filter_set_meta_data_filter_type: std::ptr::null_mut(),
            u_uniform_integer_set_uniform_integer: std::ptr::null_mut(),
            u_radial_int_mask_set_radial_int_mask: std::ptr::null_mut(),
            u_uniform_scalar_set_uniform_scalar: std::ptr::null_mut(),
            u_wave_scalar_set_wave_scalar: std::ptr::null_mut(),
            u_radial_falloff_set_radial_falloff: std::ptr::null_mut(),
            u_plane_falloff_set_plane_falloff: std::ptr::null_mut(),
            u_box_falloff_set_box_falloff: std::ptr::null_mut(),
            u_noise_field_set_noise_field: std::ptr::null_mut(),
            u_uniform_vector_set_uniform_vector: std::ptr::null_mut(),
            u_radial_vector_set_radial_vector: std::ptr::null_mut(),
            u_random_vector_set_random_vector: std::ptr::null_mut(),
            u_operator_field_set_operator_field: std::ptr::null_mut(),
            u_to_integer_field_set_to_integer_field: std::ptr::null_mut(),
            u_to_float_field_set_to_float_field: std::ptr::null_mut(),
            u_culling_field_set_culling_field: std::ptr::null_mut(),
            u_return_results_terminal_set_return_results_terminal: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetFieldSystem"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_reset_field_system,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemovePersistentFields"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_remove_persistent_fields,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyUniformVectorFalloffForce"),
            &raw mut __FUNCTION_PTRS
                .u_field_system_component_apply_uniform_vector_falloff_force,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyStrainField"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_apply_strain_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyStayDynamicField"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_apply_stay_dynamic_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyRadialVectorFalloffForce"),
            &raw mut __FUNCTION_PTRS
                .u_field_system_component_apply_radial_vector_falloff_force,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyRadialForce"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_apply_radial_force,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyPhysicsField"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_apply_physics_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyLinearForce"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_apply_linear_force,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPersistentField"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_add_persistent_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFieldCommand"),
            &raw mut __FUNCTION_PTRS.u_field_system_component_add_field_command,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataIteration::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataIteration"),
            &raw mut __FUNCTION_PTRS
                .u_field_system_meta_data_iteration_set_meta_data_iteration,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataProcessingResolution::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataaProcessingResolutionType"),
            &raw mut __FUNCTION_PTRS
                .u_field_system_meta_data_processing_resolution_set_meta_dataa_processing_resolution_type,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFieldSystemMetaDataFilter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetaDataFilterType"),
            &raw mut __FUNCTION_PTRS
                .u_field_system_meta_data_filter_set_meta_data_filter_type,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformInteger::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformInteger"),
            &raw mut __FUNCTION_PTRS.u_uniform_integer_set_uniform_integer,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialIntMask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialIntMask"),
            &raw mut __FUNCTION_PTRS.u_radial_int_mask_set_radial_int_mask,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformScalar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformScalar"),
            &raw mut __FUNCTION_PTRS.u_uniform_scalar_set_uniform_scalar,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWaveScalar::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWaveScalar"),
            &raw mut __FUNCTION_PTRS.u_wave_scalar_set_wave_scalar,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialFalloff"),
            &raw mut __FUNCTION_PTRS.u_radial_falloff_set_radial_falloff,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlaneFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaneFalloff"),
            &raw mut __FUNCTION_PTRS.u_plane_falloff_set_plane_falloff,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBoxFalloff::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoxFalloff"),
            &raw mut __FUNCTION_PTRS.u_box_falloff_set_box_falloff,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNoiseField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNoiseField"),
            &raw mut __FUNCTION_PTRS.u_noise_field_set_noise_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUniformVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUniformVector"),
            &raw mut __FUNCTION_PTRS.u_uniform_vector_set_uniform_vector,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URadialVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRadialVector"),
            &raw mut __FUNCTION_PTRS.u_radial_vector_set_radial_vector,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URandomVector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRandomVector"),
            &raw mut __FUNCTION_PTRS.u_random_vector_set_random_vector,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOperatorField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOperatorField"),
            &raw mut __FUNCTION_PTRS.u_operator_field_set_operator_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UToIntegerField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToIntegerField"),
            &raw mut __FUNCTION_PTRS.u_to_integer_field_set_to_integer_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UToFloatField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetToFloatField"),
            &raw mut __FUNCTION_PTRS.u_to_float_field_set_to_float_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCullingField::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCullingField"),
            &raw mut __FUNCTION_PTRS.u_culling_field_set_culling_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UReturnResultsTerminal::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReturnResultsTerminal"),
            &raw mut __FUNCTION_PTRS
                .u_return_results_terminal_set_return_results_terminal,
        );
    }
}
#[repr(C, align(8))]
pub struct FFieldObjectCommands {
    pub target_names: TArray<FName>,
    pub root_nodes: TArray<UPtr<UFieldNodeBase>>,
    pub meta_datas: TArray<UPtr<UFieldSystemMetaData>>,
}
impl FFieldObjectCommands {}
#[repr(C, align(8))]
pub struct AFieldSystemActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub field_system_component: UPtr<UFieldSystemComponent>,
}
impl AFieldSystemActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AFieldSystemActor")
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
pub struct UFieldSystem {
    __padding_end: [u8; 64],
}
impl UFieldSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystem")
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
pub struct UFieldSystemComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub field_system: UPtr<UFieldSystem>,
    __padding_end: [u8; 200],
}
impl UFieldSystemComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemComponent")
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
    pub fn reset_field_system(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_reset_field_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_reset_field_system,
                __buffer,
            )
        };
    }
    pub fn remove_persistent_fields(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_remove_persistent_fields,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_remove_persistent_fields,
                __buffer,
            )
        };
    }
    pub fn apply_uniform_vector_falloff_force(
        &mut self,
        enabled: bool,
        position: crate::bindings::core_u_object::FVector,
        direction: crate::bindings::core_u_object::FVector,
        radius: f32,
        magnitude: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_uniform_vector_falloff_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(60).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_uniform_vector_falloff_force,
                __buffer,
            )
        };
    }
    pub fn apply_strain_field(
        &mut self,
        enabled: bool,
        position: crate::bindings::core_u_object::FVector,
        radius: f32,
        magnitude: f32,
        iterations: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_strain_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(36).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &iterations,
                __buffer.add(40).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_strain_field,
                __buffer,
            )
        };
    }
    pub fn apply_stay_dynamic_field(
        &mut self,
        enabled: bool,
        position: crate::bindings::core_u_object::FVector,
        radius: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_stay_dynamic_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_stay_dynamic_field,
                __buffer,
            )
        };
    }
    pub fn apply_radial_vector_falloff_force(
        &mut self,
        enabled: bool,
        position: crate::bindings::core_u_object::FVector,
        radius: f32,
        magnitude: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_radial_vector_falloff_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(36).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_radial_vector_falloff_force,
                __buffer,
            )
        };
    }
    pub fn apply_radial_force(
        &mut self,
        enabled: bool,
        position: crate::bindings::core_u_object::FVector,
        magnitude: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_radial_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_radial_force,
                __buffer,
            )
        };
    }
    pub fn apply_physics_field(
        &mut self,
        enabled: bool,
        target: crate::bindings::chaos::EFieldPhysicsType,
        meta_data: UPtr<UFieldSystemMetaData>,
        field: UPtr<UFieldNodeBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_physics_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(1).cast::<crate::bindings::chaos::EFieldPhysicsType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data,
                __buffer.add(8).cast::<UPtr<UFieldSystemMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field,
                __buffer.add(16).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_physics_field,
                __buffer,
            )
        };
    }
    pub fn apply_linear_force(
        &mut self,
        enabled: bool,
        direction: crate::bindings::core_u_object::FVector,
        magnitude: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_linear_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_apply_linear_force,
                __buffer,
            )
        };
    }
    pub fn add_persistent_field(
        &mut self,
        enabled: bool,
        target: crate::bindings::chaos::EFieldPhysicsType,
        meta_data: UPtr<UFieldSystemMetaData>,
        field: UPtr<UFieldNodeBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_add_persistent_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(1).cast::<crate::bindings::chaos::EFieldPhysicsType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data,
                __buffer.add(8).cast::<UPtr<UFieldSystemMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field,
                __buffer.add(16).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_add_persistent_field,
                __buffer,
            )
        };
    }
    pub fn add_field_command(
        &mut self,
        enabled: bool,
        target: crate::bindings::chaos::EFieldPhysicsType,
        meta_data: UPtr<UFieldSystemMetaData>,
        field: UPtr<UFieldNodeBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_add_field_command,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(1).cast::<crate::bindings::chaos::EFieldPhysicsType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data,
                __buffer.add(8).cast::<UPtr<UFieldSystemMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field,
                __buffer.add(16).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_component_add_field_command,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UFieldSystemMetaData {
    __padding_end: [u8; 240],
}
impl UFieldSystemMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaData")
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
pub struct UFieldSystemMetaDataIteration {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub iterations: i32,
}
impl UFieldSystemMetaDataIteration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataIteration")
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
    pub fn set_meta_data_iteration(
        &mut self,
        iterations: i32,
    ) -> UPtr<UFieldSystemMetaDataIteration> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_iteration_set_meta_data_iteration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&iterations, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_iteration_set_meta_data_iteration,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UFieldSystemMetaDataIteration>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UFieldSystemMetaDataProcessingResolution {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub resolution_type: crate::bindings::chaos::EFieldResolutionType,
}
impl UFieldSystemMetaDataProcessingResolution {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataProcessingResolution")
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
    pub fn set_meta_dataa_processing_resolution_type(
        &mut self,
        resolution_type: crate::bindings::chaos::EFieldResolutionType,
    ) -> UPtr<UFieldSystemMetaDataProcessingResolution> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_processing_resolution_set_meta_dataa_processing_resolution_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &resolution_type,
                __buffer.add(0).cast::<crate::bindings::chaos::EFieldResolutionType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_processing_resolution_set_meta_dataa_processing_resolution_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<UFieldSystemMetaDataProcessingResolution>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UFieldSystemMetaDataFilter {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub filter_type: crate::bindings::chaos::EFieldFilterType,
    pub object_type: crate::bindings::chaos::EFieldObjectType,
    pub position_type: crate::bindings::chaos::EFieldPositionType,
}
impl UFieldSystemMetaDataFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldSystemMetaDataFilter")
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
    pub fn set_meta_data_filter_type(
        &mut self,
        filter_type: crate::bindings::chaos::EFieldFilterType,
        object_type: crate::bindings::chaos::EFieldObjectType,
        position_type: crate::bindings::chaos::EFieldPositionType,
    ) -> UPtr<UFieldSystemMetaDataFilter> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_filter_set_meta_data_filter_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_type,
                __buffer.add(0).cast::<crate::bindings::chaos::EFieldFilterType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_type,
                __buffer.add(1).cast::<crate::bindings::chaos::EFieldObjectType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position_type,
                __buffer.add(2).cast::<crate::bindings::chaos::EFieldPositionType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_field_system_meta_data_filter_set_meta_data_filter_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UFieldSystemMetaDataFilter>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UFieldNodeBase {
    __padding_end: [u8; 240],
}
impl UFieldNodeBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeBase")
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
pub struct UFieldNodeInt {
    __padding_end: [u8; 240],
}
impl UFieldNodeInt {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeInt")
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
pub struct UFieldNodeFloat {
    __padding_end: [u8; 240],
}
impl UFieldNodeFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeFloat")
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
pub struct UFieldNodeVector {
    __padding_end: [u8; 240],
}
impl UFieldNodeVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldNodeVector")
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
pub struct UUniformInteger {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: i32,
}
impl UUniformInteger {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformInteger")
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
    pub fn set_uniform_integer(&mut self, magnitude: i32) -> UPtr<UUniformInteger> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_integer_set_uniform_integer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_integer_set_uniform_integer,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UUniformInteger>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URadialIntMask {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition: crate::bindings::chaos::ESetMaskConditionType,
}
impl URadialIntMask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialIntMask")
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
    pub fn set_radial_int_mask(
        &mut self,
        radius: f32,
        position: crate::bindings::core_u_object::FVector,
        interior_value: i32,
        exterior_value: i32,
        set_mask_condition_in: crate::bindings::chaos::ESetMaskConditionType,
    ) -> UPtr<URadialIntMask> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_int_mask_set_radial_int_mask,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interior_value,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &exterior_value,
                __buffer.add(36).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_mask_condition_in,
                __buffer.add(40).cast::<crate::bindings::chaos::ESetMaskConditionType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_int_mask_set_radial_int_mask,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URadialIntMask>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UUniformScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
}
impl UUniformScalar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformScalar")
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
    pub fn set_uniform_scalar(&mut self, magnitude: f32) -> UPtr<UUniformScalar> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_scalar_set_uniform_scalar,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_scalar_set_uniform_scalar,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UUniformScalar>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UWaveScalar {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function: crate::bindings::chaos::EWaveFunctionType,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
impl UWaveScalar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaveScalar")
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
    pub fn set_wave_scalar(
        &mut self,
        magnitude: f32,
        position: crate::bindings::core_u_object::FVector,
        wavelength: f32,
        period: f32,
        time: f32,
        function: crate::bindings::chaos::EWaveFunctionType,
        falloff: crate::bindings::chaos::EFieldFalloffType,
    ) -> UPtr<UWaveScalar> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_wave_scalar_set_wave_scalar,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wavelength,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&period, __buffer.add(36).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(40).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &function,
                __buffer.add(44).cast::<crate::bindings::chaos::EWaveFunctionType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &falloff,
                __buffer.add(45).cast::<crate::bindings::chaos::EFieldFalloffType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_wave_scalar_set_wave_scalar,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<UWaveScalar>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URadialFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub radius: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
impl URadialFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialFalloff")
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
    pub fn set_radial_falloff(
        &mut self,
        magnitude: f32,
        min_range: f32,
        max_range: f32,
        default: f32,
        radius: f32,
        position: crate::bindings::core_u_object::FVector,
        falloff: crate::bindings::chaos::EFieldFalloffType,
    ) -> UPtr<URadialFalloff> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_falloff_set_radial_falloff,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&min_range, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_range, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&default, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &falloff,
                __buffer.add(48).cast::<crate::bindings::chaos::EFieldFalloffType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_falloff_set_radial_falloff,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URadialFalloff>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPlaneFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub distance: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
impl UPlaneFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneFalloff")
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
    pub fn set_plane_falloff(
        &mut self,
        magnitude: f32,
        min_range: f32,
        max_range: f32,
        default: f32,
        distance: f32,
        position: crate::bindings::core_u_object::FVector,
        normal: crate::bindings::core_u_object::FVector,
        falloff: crate::bindings::chaos::EFieldFalloffType,
    ) -> UPtr<UPlaneFalloff> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_plane_falloff_set_plane_falloff,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&min_range, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_range, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&default, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&distance, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normal,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &falloff,
                __buffer.add(72).cast::<crate::bindings::chaos::EFieldFalloffType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_plane_falloff_set_plane_falloff,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<UPlaneFalloff>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UBoxFalloff {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub falloff: crate::bindings::chaos::EFieldFalloffType,
}
impl UBoxFalloff {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoxFalloff")
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
    pub fn set_box_falloff(
        &mut self,
        magnitude: f32,
        min_range: f32,
        max_range: f32,
        default: f32,
        transform: crate::bindings::core_u_object::FTransform,
        falloff: crate::bindings::chaos::EFieldFalloffType,
    ) -> UPtr<UBoxFalloff> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_box_falloff_set_box_falloff,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&min_range, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_range, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&default, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &falloff,
                __buffer.add(112).cast::<crate::bindings::chaos::EFieldFalloffType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_box_falloff_set_box_falloff,
                __buffer,
            )
        };
        unsafe { __buffer.add(120).cast::<UPtr<UBoxFalloff>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UNoiseField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub min_range: f32,
    pub max_range: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl UNoiseField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNoiseField")
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
    pub fn set_noise_field(
        &mut self,
        min_range: f32,
        max_range: f32,
        transform: crate::bindings::core_u_object::FTransform,
    ) -> UPtr<UNoiseField> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_noise_field_set_noise_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&min_range, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_range, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_noise_field_set_noise_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<UPtr<UNoiseField>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UUniformVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub direction: crate::bindings::core_u_object::FVector,
}
impl UUniformVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUniformVector")
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
    pub fn set_uniform_vector(
        &mut self,
        magnitude: f32,
        direction: crate::bindings::core_u_object::FVector,
    ) -> UPtr<UUniformVector> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_vector_set_uniform_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction,
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
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_uniform_vector_set_uniform_vector,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UUniformVector>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URadialVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
}
impl URadialVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URadialVector")
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
    pub fn set_radial_vector(
        &mut self,
        magnitude: f32,
        position: crate::bindings::core_u_object::FVector,
    ) -> UPtr<URadialVector> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_vector_set_radial_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
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
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_radial_vector_set_radial_vector,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<URadialVector>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URandomVector {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
}
impl URandomVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URandomVector")
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
    pub fn set_random_vector(&mut self, magnitude: f32) -> UPtr<URandomVector> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_random_vector_set_random_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_random_vector_set_random_vector,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URandomVector>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UOperatorField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub magnitude: f32,
    pub right_field: UPtr<UFieldNodeBase>,
    pub left_field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldOperationType,
}
impl UOperatorField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOperatorField")
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
    pub fn set_operator_field(
        &mut self,
        magnitude: f32,
        left_field: UPtr<UFieldNodeBase>,
        right_field: UPtr<UFieldNodeBase>,
        operation: crate::bindings::chaos::EFieldOperationType,
    ) -> UPtr<UOperatorField> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_operator_field_set_operator_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &left_field,
                __buffer.add(8).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &right_field,
                __buffer.add(16).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &operation,
                __buffer.add(24).cast::<crate::bindings::chaos::EFieldOperationType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_operator_field_set_operator_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UOperatorField>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UToIntegerField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub float_field: UPtr<UFieldNodeFloat>,
}
impl UToIntegerField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToIntegerField")
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
    pub fn set_to_integer_field(
        &mut self,
        float_field: UPtr<UFieldNodeFloat>,
    ) -> UPtr<UToIntegerField> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_to_integer_field_set_to_integer_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &float_field,
                __buffer.add(0).cast::<UPtr<UFieldNodeFloat>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_to_integer_field_set_to_integer_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UToIntegerField>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UToFloatField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub int_field: UPtr<UFieldNodeInt>,
}
impl UToFloatField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToFloatField")
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
    pub fn set_to_float_field(
        &mut self,
        integer_field: UPtr<UFieldNodeInt>,
    ) -> UPtr<UToFloatField> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_to_float_field_set_to_float_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &integer_field,
                __buffer.add(0).cast::<UPtr<UFieldNodeInt>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_to_float_field_set_to_float_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UToFloatField>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCullingField {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub culling: UPtr<UFieldNodeBase>,
    pub field: UPtr<UFieldNodeBase>,
    pub operation: crate::bindings::chaos::EFieldCullingOperationType,
}
impl UCullingField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCullingField")
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
    pub fn set_culling_field(
        &mut self,
        culling: UPtr<UFieldNodeBase>,
        field: UPtr<UFieldNodeBase>,
        operation: crate::bindings::chaos::EFieldCullingOperationType,
    ) -> UPtr<UCullingField> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_culling_field_set_culling_field,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &culling,
                __buffer.add(0).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field,
                __buffer.add(8).cast::<UPtr<UFieldNodeBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &operation,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::chaos::EFieldCullingOperationType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_culling_field_set_culling_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UCullingField>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UReturnResultsTerminal {
    __padding_end: [u8; 240],
}
impl UReturnResultsTerminal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReturnResultsTerminal")
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
    pub fn set_return_results_terminal(&mut self) -> UPtr<UReturnResultsTerminal> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_return_results_terminal_set_return_results_terminal,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::field_system_engine::__FUNCTION_PTRS
                    .u_return_results_terminal_set_return_results_terminal,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UReturnResultsTerminal>>().read() }
    }
}
