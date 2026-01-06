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
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_UPGRADE_OPERATOR_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_INSTANCE_EDITABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_TO_CINEMATICS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_ON_SPAWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REPLACE_VARIABLE_REFERENCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REPARENT_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_RENAME_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_VARIABLES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_FUNCTION_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_OPEN_EDITORS_FOR_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_ALL_OPEN_BLUEPRINT_EDITORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_STRUCT_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_SET_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_SAVED_BY_ENGINE_VERSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_OBJECT_REFERENCE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_MAP_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENT_ENGINE_VERSION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_CLASS_REFERENCE_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_FOR_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BASIC_TYPE_BY_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GET_ARRAY_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_GENERATED_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_FIND_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_FIND_EVENT_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_CREATE_BLUEPRINT_ASSET_WITH_PARENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_COMPILE_BLUEPRINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_SOFT_OBJECT_SAVE_VERSION_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_ASSET_SAVE_VERSION_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE_WITH_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_FUNCTION_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintEditorLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpgradeOperatorNodes"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_UPGRADE_OPERATOR_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlueprintVariableInstanceEditable"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_INSTANCE_EDITABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlueprintVariableExposeToCinematics"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_TO_CINEMATICS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlueprintVariableExposeOnSpawn"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_ON_SPAWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceVariableReferences"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REPLACE_VARIABLE_REFERENCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReparentBlueprint"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REPARENT_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_RENAME_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUnusedVariables"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_VARIABLES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUnusedNodes"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveFunctionGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_FUNCTION_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshOpenEditorsForBlueprint"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_OPEN_EDITORS_FOR_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RefreshAllOpenBlueprintEditors"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_ALL_OPEN_BLUEPRINT_EDITORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStructType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_STRUCT_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSetType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_SET_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSavedByEngineVersion"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_SAVED_BY_ENGINE_VERSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectReferenceType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_OBJECT_REFERENCE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMapType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_MAP_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentEngineVersion"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENT_ENGINE_VERSION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClassReferenceType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_CLASS_REFERENCE_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintForClass"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_FOR_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintAsset"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBasicTypeByName"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_BASIC_TYPE_BY_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArrayType"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GET_ARRAY_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GeneratedClass"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_GENERATED_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_FIND_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindEventGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_FIND_EVENT_GRAPH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBlueprintAssetWithParent"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_CREATE_BLUEPRINT_ASSET_WITH_PARENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompileBlueprint"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_COMPILE_BLUEPRINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompareSoftObjectSaveVersionTo"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_SOFT_OBJECT_SAVE_VERSION_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CompareAssetSaveVersionTo"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_ASSET_SAVE_VERSION_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMemberVariableWithValue"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE_WITH_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMemberVariable"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFunctionGraph"),
            &raw mut U_BLUEPRINT_EDITOR_LIBRARY_ADD_FUNCTION_GRAPH,
        );
    }
}
#[repr(C, align(8))]
pub struct UBlueprintEditorLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintEditorLibrary")
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
pub struct EAssetSaveVersionComparisonResults(pub u8);
impl EAssetSaveVersionComparisonResults {
    pub const INVALID_COMPARISON: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        0,
    );
    pub const IDENTICAL: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        1,
    );
    pub const NEWER: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        2,
    );
    pub const OLDER: EAssetSaveVersionComparisonResults = EAssetSaveVersionComparisonResults(
        3,
    );
}
