#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_location_services_stop_location_services: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_start_location_services: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_is_location_accuracy_available: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_init_location_services: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_get_location_services_impl: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_get_last_known_location: *mut crate::ffi::UFunctionOpague,
    pub u_location_services_are_location_services_enabled: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_location_services_stop_location_services: std::ptr::null_mut(),
            u_location_services_start_location_services: std::ptr::null_mut(),
            u_location_services_is_location_accuracy_available: std::ptr::null_mut(),
            u_location_services_init_location_services: std::ptr::null_mut(),
            u_location_services_get_location_services_impl: std::ptr::null_mut(),
            u_location_services_get_last_known_location: std::ptr::null_mut(),
            u_location_services_are_location_services_enabled: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULocationServices::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopLocationServices"),
                &raw mut __FUNCTION_PTRS.u_location_services_stop_location_services,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartLocationServices"),
                &raw mut __FUNCTION_PTRS.u_location_services_start_location_services,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLocationAccuracyAvailable"),
                &raw mut __FUNCTION_PTRS
                    .u_location_services_is_location_accuracy_available,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitLocationServices"),
                &raw mut __FUNCTION_PTRS.u_location_services_init_location_services,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocationServicesImpl"),
                &raw mut __FUNCTION_PTRS.u_location_services_get_location_services_impl,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastKnownLocation"),
                &raw mut __FUNCTION_PTRS.u_location_services_get_last_known_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AreLocationServicesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_location_services_are_location_services_enabled,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FLocationServicesData {
    pub timestamp: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub horizontal_accuracy: f32,
    pub vertical_accuracy: f32,
    pub altitude: f32,
}
impl FLocationServicesData {}
#[repr(C, align(8))]
pub struct ULocationServicesImpl {
    __padding_end: [u8; 72],
}
impl ULocationServicesImpl {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServicesImpl")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServicesImpl")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULocationServices {
    __padding_end: [u8; 48],
}
impl ULocationServices {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServices")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServices")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn stop_location_services() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_stop_location_services,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_stop_location_services,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn start_location_services() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_start_location_services,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_start_location_services,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_location_accuracy_available(accuracy: ELocationAccuracy) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_is_location_accuracy_available,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &accuracy,
                __buffer.add(0).cast::<ELocationAccuracy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_is_location_accuracy_available,
                __buffer,
            )
        };
        std::mem::forget(accuracy);
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn init_location_services(
        accuracy: ELocationAccuracy,
        update_frequency: f32,
        min_distance_filter: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_init_location_services,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &accuracy,
                __buffer.add(0).cast::<ELocationAccuracy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &update_frequency,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &min_distance_filter,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_init_location_services,
                __buffer,
            )
        };
        std::mem::forget(accuracy);
        std::mem::forget(update_frequency);
        std::mem::forget(min_distance_filter);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_location_services_impl() -> UPtr<ULocationServicesImpl> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_get_location_services_impl,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_get_location_services_impl,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ULocationServicesImpl>>().read() }
    }
    pub fn get_last_known_location() -> FLocationServicesData {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_get_last_known_location,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_get_last_known_location,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FLocationServicesData>().read() }
    }
    pub fn are_location_services_enabled() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_are_location_services_enabled,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::location_services_bp_library::ULocationServices::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::location_services_bp_library::__FUNCTION_PTRS
                    .u_location_services_are_location_services_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FLocationServicesImpl_OnLocationChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ELocationAccuracy(pub u8);
impl ELocationAccuracy {
    pub const LA_THREE_KILOMETERS: ELocationAccuracy = ELocationAccuracy(0);
    pub const LA_ONE_KILOMETER: ELocationAccuracy = ELocationAccuracy(1);
    pub const LA_HUNDRED_METERS: ELocationAccuracy = ELocationAccuracy(2);
    pub const LA_TEN_METERS: ELocationAccuracy = ELocationAccuracy(3);
    pub const LA_BEST: ELocationAccuracy = ELocationAccuracy(4);
    pub const LA_NAVIGATION: ELocationAccuracy = ELocationAccuracy(5);
}
