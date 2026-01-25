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
    pub u_chaos_vd_runtime_blueprint_library_set_trace_relevancy_volume: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_vd_runtime_blueprint_library_record_debug_draw_vector: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_vd_runtime_blueprint_library_record_debug_draw_sphere: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_vd_runtime_blueprint_library_record_debug_draw_line: *mut crate::ffi::UFunctionOpague,
    pub u_chaos_vd_runtime_blueprint_library_record_debug_draw_box: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_chaos_vd_runtime_blueprint_library_set_trace_relevancy_volume: std::ptr::null_mut(),
            u_chaos_vd_runtime_blueprint_library_record_debug_draw_vector: std::ptr::null_mut(),
            u_chaos_vd_runtime_blueprint_library_record_debug_draw_sphere: std::ptr::null_mut(),
            u_chaos_vd_runtime_blueprint_library_record_debug_draw_line: std::ptr::null_mut(),
            u_chaos_vd_runtime_blueprint_library_record_debug_draw_box: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosVDRuntimeBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTraceRelevancyVolume"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_vd_runtime_blueprint_library_set_trace_relevancy_volume,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawVector"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_vd_runtime_blueprint_library_record_debug_draw_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawSphere"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_vd_runtime_blueprint_library_record_debug_draw_sphere,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawLine"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_vd_runtime_blueprint_library_record_debug_draw_line,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawBox"),
            &raw mut __FUNCTION_PTRS
                .u_chaos_vd_runtime_blueprint_library_record_debug_draw_box,
        );
    }
}
#[repr(C, align(8))]
pub struct UChaosVDRuntimeBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UChaosVDRuntimeBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChaosVDRuntimeBlueprintLibrary")
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
    pub fn set_trace_relevancy_volume(
        world_context: UPtr<crate::bindings::core_u_object::UObject>,
        relevancy_volume: &crate::bindings::core_u_object::FBox,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_set_trace_relevancy_volume,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                relevancy_volume,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_vd_blueprint::UChaosVDRuntimeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_set_trace_relevancy_volume,
                __buffer,
            )
        };
    }
    pub fn record_debug_draw_vector(
        world_context: UPtr<crate::bindings::core_u_object::UObject>,
        in_start_location: &crate::bindings::core_u_object::FVector,
        in_vector: &crate::bindings::core_u_object::FVector,
        tag: FName,
        color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_start_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_vector,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(56).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(68).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_vd_blueprint::UChaosVDRuntimeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_vector,
                __buffer,
            )
        };
    }
    pub fn record_debug_draw_sphere(
        world_context: UPtr<crate::bindings::core_u_object::UObject>,
        in_center: &crate::bindings::core_u_object::FVector,
        radius: f32,
        tag: FName,
        color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_sphere,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_center,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(36).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_vd_blueprint::UChaosVDRuntimeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_sphere,
                __buffer,
            )
        };
    }
    pub fn record_debug_draw_line(
        world_context: UPtr<crate::bindings::core_u_object::UObject>,
        in_start_location: &crate::bindings::core_u_object::FVector,
        in_end_location: &crate::bindings::core_u_object::FVector,
        tag: FName,
        color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_line,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_start_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_end_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(56).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(68).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_vd_blueprint::UChaosVDRuntimeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_line,
                __buffer,
            )
        };
    }
    pub fn record_debug_draw_box(
        world_context: UPtr<crate::bindings::core_u_object::UObject>,
        in_box: &crate::bindings::core_u_object::FBox,
        tag: FName,
        color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_box,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_box,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(64).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(76).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::chaos_vd_blueprint::UChaosVDRuntimeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::chaos_vd_blueprint::__FUNCTION_PTRS
                    .u_chaos_vd_runtime_blueprint_library_record_debug_draw_box,
                __buffer,
            )
        };
    }
}
