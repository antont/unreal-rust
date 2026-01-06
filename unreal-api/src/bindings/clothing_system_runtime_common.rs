#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UClothConfigCommon {
    __padding_end: [u8; 48],
}
impl UClothConfigCommon {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothConfigCommon")
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
pub struct UClothSharedConfigCommon {
    __padding_end: [u8; 48],
}
impl UClothSharedConfigCommon {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothSharedConfigCommon")
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
pub struct UClothingAssetCustomData {
    __padding_end: [u8; 48],
}
impl UClothingAssetCustomData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetCustomData")
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
pub struct UClothingAssetCommon {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub cloth_configs: TMap<
        FName,
        UPtr<crate::bindings::clothing_system_runtime_interface::UClothConfigBase>,
    >,
    __padding_end: [u8; 416],
}
impl UClothingAssetCommon {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetCommon")
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
pub struct UClothLODDataCommon_Legacy {
    __padding_end: [u8; 544],
}
impl UClothLODDataCommon_Legacy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothLODDataCommon_Legacy")
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
pub struct EClothingWindMethod_Legacy(pub u8);
impl EClothingWindMethod_Legacy {
    pub const LEGACY: EClothingWindMethod_Legacy = EClothingWindMethod_Legacy(0);
    pub const ACCURATE: EClothingWindMethod_Legacy = EClothingWindMethod_Legacy(1);
}
#[repr(transparent)]
pub struct EWeightMapTargetCommon(pub u8);
impl EWeightMapTargetCommon {
    pub const NONE: EWeightMapTargetCommon = EWeightMapTargetCommon(0);
    pub const MAX_DISTANCE: EWeightMapTargetCommon = EWeightMapTargetCommon(1);
    pub const BACKSTOP_DISTANCE: EWeightMapTargetCommon = EWeightMapTargetCommon(2);
    pub const BACKSTOP_RADIUS: EWeightMapTargetCommon = EWeightMapTargetCommon(3);
    pub const ANIM_DRIVE_STIFFNESS: EWeightMapTargetCommon = EWeightMapTargetCommon(4);
    pub const ANIM_DRIVE_DAMPING_DEPRECATED: EWeightMapTargetCommon = EWeightMapTargetCommon(
        5,
    );
    pub const FIRST_USER_TARGET: EWeightMapTargetCommon = EWeightMapTargetCommon(6);
    pub const LAST_USER_TARGET: EWeightMapTargetCommon = EWeightMapTargetCommon(200);
    pub const TETHER_ENDS_MASK: EWeightMapTargetCommon = EWeightMapTargetCommon(201);
}
#[repr(transparent)]
pub struct EClothMassMode(pub u8);
impl EClothMassMode {
    pub const UNIFORM_MASS: EClothMassMode = EClothMassMode(0);
    pub const TOTAL_MASS: EClothMassMode = EClothMassMode(1);
    pub const DENSITY: EClothMassMode = EClothMassMode(2);
    pub const MAX_CLOTH_MASS_MODE: EClothMassMode = EClothMassMode(3);
}
