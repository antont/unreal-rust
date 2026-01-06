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
pub static mut U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UControlRigBlueprint::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateExposedModuleConnectors"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_UPDATE_EXPOSED_MODULE_CONNECTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TurnIntoStandaloneRig_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TurnIntoControlRigModule_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_TURN_INTO_CONTROL_RIG_MODULE_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_SET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileModularRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_RECOMPILE_MODULAR_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsControlRigModule"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_IS_CONTROL_RIG_MODULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigModuleIcon"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_RIG_MODULE_ICON,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewMesh"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_PREVIEW_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModularRigController"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_MODULAR_RIG_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHierarchyController"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_HIERARCHY_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDebuggedControlRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_DEBUGGED_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentlyOpenRigBlueprints"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_CURRENTLY_OPEN_RIG_BLUEPRINTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetControlRigClass"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_GET_CONTROL_RIG_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindReferencesToModule"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_FIND_REFERENCES_TO_MODULE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateControlRig"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CREATE_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertHierarchyElementsToSpawnerNodes"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CONVERT_HIERARCHY_ELEMENTS_TO_SPAWNER_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanTurnIntoStandaloneRig_Blueprint"),
            &raw mut U_CONTROL_RIG_BLUEPRINT_CAN_TURN_INTO_STANDALONE_RIG_BLUEPRINT,
        );
    }
}
#[repr(C, align(8))]
pub struct UAnimGraphNode_ControlRig {
    __padding_end: [u8; 2104],
}
impl UAnimGraphNode_ControlRig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimGraphNode_ControlRig")
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
pub struct IControlRigAssetInterface {}
#[repr(C, align(8))]
pub struct UControlRigAssetInterface {
    __padding_end: [u8; 48],
}
impl UControlRigAssetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigAssetInterface")
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
pub struct UControlRigBlueprint {
    #[doc(hidden)]
    __padding_4648: [u8; 4648],
    pub hierarchy: UPtr<crate::bindings::control_rig::URigHierarchy>,
    pub modular_rig_model: crate::bindings::control_rig::FModularRigModel,
    __padding_end: [u8; 288],
}
impl UControlRigBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprint")
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
pub struct UControlRigSchema {
    __padding_end: [u8; 80],
}
impl UControlRigSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSchema")
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
pub struct UControlRigGraph {
    __padding_end: [u8; 592],
}
impl UControlRigGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraph")
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
pub struct UControlRigGraphNode {
    __padding_end: [u8; 1152],
}
impl UControlRigGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphNode")
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
pub struct UControlRigGraphSchema {
    __padding_end: [u8; 168],
}
impl UControlRigGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigGraphSchema")
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
