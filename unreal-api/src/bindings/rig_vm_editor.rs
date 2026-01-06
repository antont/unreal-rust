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
pub static mut U_RIG_VM_EDITOR_MENU_CONTEXT_IS_ALT_DOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_HOST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_ASSET_INTERFACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_GRAPH_MENU_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_REQUEST_AUTO_VM_RECOMPILATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_RECOMPILE_VM_IF_REQUIRED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_RECOMPILE_VM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_NODE_FILTER_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_BLUEPRINT_FILTER_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_FILTER_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_AND_NODE_FILTERS_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_AND_BLUEPRINT_FILTERS_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_MODEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_CONTROLLER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_ASSETS_WITH_FILTER_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMEditorMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAltDown"),
            &raw mut U_RIG_VM_EDITOR_MENU_CONTEXT_IS_ALT_DOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMHost"),
            &raw mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_HOST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMBlueprint"),
            &raw mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRigVMAssetInterface"),
            &raw mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_RIG_VM_ASSET_INTERFACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphMenuContext"),
            &raw mut U_RIG_VM_EDITOR_MENU_CONTEXT_GET_GRAPH_MENU_CONTEXT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URigVMEditorBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestAutoVMRecompilation"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_REQUEST_AUTO_VM_RECOMPILATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVMIfRequired"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_RECOMPILE_VM_IF_REQUIRED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RecompileVM"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_RECOMPILE_VM,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithNodeFilter_ForBlueprint"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_NODE_FILTER_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithBlueprintFilter_ForBlueprint"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_BLUEPRINT_FILTER_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsWithAssetDataFilter_ForBlueprint"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_FILTER_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "LoadAssetsWithAssetDataAndNodeFilters_ForBlueprint",
            ),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_AND_NODE_FILTERS_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "LoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint",
            ),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_WITH_ASSET_DATA_AND_BLUEPRINT_FILTERS_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssetsByClass"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAssets"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_LOAD_ASSETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetModel"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_MODEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetController"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_CONTROLLER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsWithFilter_ForBlueprint"),
            &raw mut U_RIG_VM_EDITOR_BLUEPRINT_LIBRARY_GET_ASSETS_WITH_FILTER_FOR_BLUEPRINT,
        );
    }
}
#[repr(C, align(8))]
pub struct FRigVMBlueprintLoadLogEntry {
    pub severity: ERigVMBlueprintLoadLogSeverity,
    pub subject: UPtr<crate::bindings::core_u_object::UObject>,
    pub message: FString,
}
impl FRigVMBlueprintLoadLogEntry {}
#[repr(C, align(8))]
pub struct FRigVMEditorGraphMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FRigVMEditorGraphMenuContext {}
#[repr(C, align(8))]
pub struct URigVMEdGraphNodeBlueprintSpawner {
    __padding_end: [u8; 288],
}
impl URigVMEdGraphNodeBlueprintSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphNodeBlueprintSpawner")
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
pub struct URigVMBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl URigVMBlueprintCompilerExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprintCompilerExtension")
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
pub struct URigVMDetailsViewWrapperObject {
    __padding_end: [u8; 96],
}
impl URigVMDetailsViewWrapperObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMDetailsViewWrapperObject")
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
pub struct URigVMEditorMenuContext {
    __padding_end: [u8; 88],
}
impl URigVMEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorMenuContext")
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
pub struct URigVMEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl URigVMEditorBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorBlueprintLibrary")
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
pub struct FGetAssetsWithFilter_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndBlueprintFilters_ForBlueprint_InBlueprintFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataAndNodeFilters_ForBlueprint_InNodeFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithAssetDataFilter_ForBlueprint_InAssetDataFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithBlueprintFilter_ForBlueprint_InBlueprintFilter {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FLoadAssetsWithNodeFilter_ForBlueprint_InNodeFilter {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct ERigVMBlueprintLoadLogSeverity(pub u8);
impl ERigVMBlueprintLoadLogSeverity {
    pub const DISPLAY: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        0,
    );
    pub const WARNING: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(
        1,
    );
    pub const ERROR: ERigVMBlueprintLoadLogSeverity = ERigVMBlueprintLoadLogSeverity(2);
}
