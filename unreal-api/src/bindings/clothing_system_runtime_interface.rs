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
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_SET_NUM_SUBSTEPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_SET_NUM_ITERATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_SET_MAX_NUM_ITERATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_SET_ANIM_DRIVE_SPRING_STIFFNESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_PHYSICS_ASSET_UPDATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_SIMULATION_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_SUBSTEPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_KINEMATIC_PARTICLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_ITERATIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_DYNAMIC_PARTICLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_CLOTHS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_GET_CLOTHING_INTERACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_ENABLE_GRAVITY_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_DISABLE_GRAVITY_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CLOTHING_SIMULATION_INTERACTOR_CLOTH_CONFIG_UPDATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UClothingSimulationInteractor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumSubsteps"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_SET_NUM_SUBSTEPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumIterations"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_SET_NUM_ITERATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxNumIterations"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_SET_MAX_NUM_ITERATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimDriveSpringStiffness"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_SET_ANIM_DRIVE_SPRING_STIFFNESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PhysicsAssetUpdated"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_PHYSICS_ASSET_UPDATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSimulationTime"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_SIMULATION_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSubsteps"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_SUBSTEPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKinematicParticles"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_KINEMATIC_PARTICLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumIterations"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_ITERATIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumDynamicParticles"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_DYNAMIC_PARTICLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumCloths"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_NUM_CLOTHS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClothingInteractor"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_GET_CLOTHING_INTERACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableGravityOverride"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_ENABLE_GRAVITY_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisableGravityOverride"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_DISABLE_GRAVITY_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClothConfigUpdated"),
            &raw mut U_CLOTHING_SIMULATION_INTERACTOR_CLOTH_CONFIG_UPDATED,
        );
    }
}
#[repr(C, align(8))]
pub struct UClothConfigBase {
    __padding_end: [u8; 48],
}
impl UClothConfigBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothConfigBase")
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
pub struct UDEPRECATED_ClothSharedSimConfigBase {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_ClothSharedSimConfigBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_ClothSharedSimConfigBase")
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
pub struct UClothingAssetBase {
    __padding_end: [u8; 80],
}
impl UClothingAssetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetBase")
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
pub struct UClothingSimulationFactory {
    __padding_end: [u8; 48],
}
impl UClothingSimulationFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingSimulationFactory")
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
pub struct UClothingInteractor {
    __padding_end: [u8; 56],
}
impl UClothingInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingInteractor")
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
pub struct UClothingSimulationInteractor {
    __padding_end: [u8; 152],
}
impl UClothingSimulationInteractor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingSimulationInteractor")
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
pub struct UClothPhysicalMeshDataBase_Legacy {
    __padding_end: [u8; 248],
}
impl UClothPhysicalMeshDataBase_Legacy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPhysicalMeshDataBase_Legacy")
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
pub struct EClothingTeleportMode(pub u8);
impl EClothingTeleportMode {
    pub const NONE: EClothingTeleportMode = EClothingTeleportMode(0);
    pub const TELEPORT: EClothingTeleportMode = EClothingTeleportMode(1);
    pub const TELEPORT_AND_RESET: EClothingTeleportMode = EClothingTeleportMode(2);
    pub const HARD_RESET: EClothingTeleportMode = EClothingTeleportMode(3);
}
