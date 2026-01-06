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
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_REGENERATE_ASSET_FROM_DATAFLOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_OBJECT_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_INT_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_INT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_FLOAT_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_BOOL_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATAFLOW_BLUEPRINT_LIBRARY_EVALUATE_TERMINAL_NODE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDataflowBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegenerateAssetFromDataflow"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_REGENERATE_ASSET_FROM_DATAFLOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableObjectArray"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_OBJECT_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableObject"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableIntArray"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_INT_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableInt"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_INT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableFloatArray"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_FLOAT_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableFloat"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableBoolArray"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_BOOL_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideDataflowVariableBool"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_OVERRIDE_DATAFLOW_VARIABLE_BOOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateTerminalNodeByName"),
            &raw mut U_DATAFLOW_BLUEPRINT_LIBRARY_EVALUATE_TERMINAL_NODE_BY_NAME,
        );
    }
}
#[repr(C, align(8))]
pub struct FStringValuePair {
    pub key: FString,
    pub value: FString,
}
impl FStringValuePair {}
#[repr(C, align(8))]
pub struct UDataflowBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UDataflowBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBlueprintLibrary")
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
pub struct IDataflowContentOwner {}
#[repr(C, align(8))]
pub struct UDataflowContentOwner {
    __padding_end: [u8; 48],
}
impl UDataflowContentOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContentOwner")
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
pub struct UDataflowContextObject {
    __padding_end: [u8; 128],
}
impl UDataflowContextObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowContextObject")
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
pub struct UDataflowBaseContent {
    __padding_end: [u8; 208],
}
impl UDataflowBaseContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBaseContent")
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
pub struct UDataflowSkeletalContent {
    __padding_end: [u8; 232],
}
impl UDataflowSkeletalContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSkeletalContent")
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
pub struct UDataflowDebugDrawComponent {
    __padding_end: [u8; 1616],
}
impl UDataflowDebugDrawComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowDebugDrawComponent")
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
pub struct UDataflowEdNode {
    __padding_end: [u8; 256],
}
impl UDataflowEdNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEdNode")
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
pub struct IDataflowInstanceInterface {}
#[repr(C, align(8))]
pub struct UDataflowInstanceInterface {
    __padding_end: [u8; 48],
}
impl UDataflowInstanceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowInstanceInterface")
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
pub struct UDataflowMesh {
    __padding_end: [u8; 72],
}
impl UDataflowMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowMesh")
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
pub struct UDataflow {
    __padding_end: [u8; 720],
}
impl UDataflow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflow")
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
pub struct UDataflowSubGraph {
    __padding_end: [u8; 216],
}
impl UDataflowSubGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSubGraph")
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
pub struct EDataflowDebugDrawRenderType(pub u8);
impl EDataflowDebugDrawRenderType {
    pub const WIREFRAME: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(0);
    pub const SHADED: EDataflowDebugDrawRenderType = EDataflowDebugDrawRenderType(1);
}
#[repr(transparent)]
pub struct EDataflowSphereCoveringColorMethod(pub u8);
impl EDataflowSphereCoveringColorMethod {
    pub const SINGLE: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        0,
    );
    pub const COLOR_BY_RADIUS: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        1,
    );
    pub const RANDOM: EDataflowSphereCoveringColorMethod = EDataflowSphereCoveringColorMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowType(pub u8);
impl EDataflowType {
    pub const CONSTRUCTION: EDataflowType = EDataflowType(0);
    pub const SIMULATION: EDataflowType = EDataflowType(1);
}
