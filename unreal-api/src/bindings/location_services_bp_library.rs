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
pub static mut U_LOCATION_SERVICES_STOP_LOCATION_SERVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_START_LOCATION_SERVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_IS_LOCATION_ACCURACY_AVAILABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_INIT_LOCATION_SERVICES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_GET_LOCATION_SERVICES_IMPL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_GET_LAST_KNOWN_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LOCATION_SERVICES_ARE_LOCATION_SERVICES_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULocationServices::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopLocationServices"),
            &raw mut U_LOCATION_SERVICES_STOP_LOCATION_SERVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartLocationServices"),
            &raw mut U_LOCATION_SERVICES_START_LOCATION_SERVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocationAccuracyAvailable"),
            &raw mut U_LOCATION_SERVICES_IS_LOCATION_ACCURACY_AVAILABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitLocationServices"),
            &raw mut U_LOCATION_SERVICES_INIT_LOCATION_SERVICES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLocationServicesImpl"),
            &raw mut U_LOCATION_SERVICES_GET_LOCATION_SERVICES_IMPL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLastKnownLocation"),
            &raw mut U_LOCATION_SERVICES_GET_LAST_KNOWN_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AreLocationServicesEnabled"),
            &raw mut U_LOCATION_SERVICES_ARE_LOCATION_SERVICES_ENABLED,
        );
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServicesImpl")
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
pub struct ULocationServices {
    __padding_end: [u8; 48],
}
impl ULocationServices {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationServices")
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
