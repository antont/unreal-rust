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
pub static mut U_DATAFLOW_SIMULATION_ACTOR_PRE_DATAFLOW_SIMULATION_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_SIMULATION_ACTOR_POST_DATAFLOW_SIMULATION_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDataflowSimulationActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PreDataflowSimulationTick"),
            &raw mut U_DATAFLOW_SIMULATION_ACTOR_PRE_DATAFLOW_SIMULATION_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PostDataflowSimulationTick"),
            &raw mut U_DATAFLOW_SIMULATION_ACTOR_POST_DATAFLOW_SIMULATION_TICK,
        );
    }
}
#[repr(C, align(8))]
pub struct FDataflowSimulationAsset {
    pub dataflow_asset: UPtr<crate::bindings::dataflow_engine::UDataflow>,
    pub simulation_groups: TSet<FString>,
}
impl FDataflowSimulationAsset {}
#[repr(C, align(8))]
pub struct FDataflowSimulationProperty {
    __padding_end: [u8; 8],
}
impl FDataflowSimulationProperty {}
pub struct IDataflowGeometryCachable {}
#[repr(C, align(8))]
pub struct UDataflowGeometryCachable {
    __padding_end: [u8; 48],
}
impl UDataflowGeometryCachable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowGeometryCachable")
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
pub struct IDataflowSimulationInterface {}
#[repr(C, align(8))]
pub struct UDataflowSimulationInterface {
    __padding_end: [u8; 48],
}
impl UDataflowSimulationInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationInterface")
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
pub struct UDataflowSimulationManager {
    __padding_end: [u8; 184],
}
impl UDataflowSimulationManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationManager")
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
pub struct IDataflowSimulationActor {}
#[repr(C, align(8))]
pub struct UDataflowSimulationActor {
    __padding_end: [u8; 48],
}
impl UDataflowSimulationActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationActor")
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
pub struct IDataflowCollisionObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowCollisionObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowCollisionObjectInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowCollisionObjectInterface")
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
pub struct IDataflowConstraintObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowConstraintObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowConstraintObjectInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowConstraintObjectInterface")
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
pub struct IDataflowPhysicsObjectInterface {}
#[repr(C, align(8))]
pub struct UDataflowPhysicsObjectInterface {
    __padding_end: [u8; 48],
}
impl UDataflowPhysicsObjectInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowPhysicsObjectInterface")
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
pub struct IDataflowPhysicsSolverInterface {}
#[repr(C, align(8))]
pub struct UDataflowPhysicsSolverInterface {
    __padding_end: [u8; 48],
}
impl UDataflowPhysicsSolverInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowPhysicsSolverInterface")
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
