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
pub static mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_SET_TRACE_RELEVANCY_VOLUME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_VECTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_SPHERE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_LINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_BOX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UChaosVDRuntimeBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTraceRelevancyVolume"),
            &raw mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_SET_TRACE_RELEVANCY_VOLUME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawVector"),
            &raw mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_VECTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawSphere"),
            &raw mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_SPHERE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawLine"),
            &raw mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_LINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecordDebugDrawBox"),
            &raw mut U_CHAOS_VD_RUNTIME_BLUEPRINT_LIBRARY_RECORD_DEBUG_DRAW_BOX,
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
}
