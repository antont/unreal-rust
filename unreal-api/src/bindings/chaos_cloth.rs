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
