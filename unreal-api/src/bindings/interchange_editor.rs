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
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_SCENE_IMPORT_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_LEVEL_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_GET_EDITABLE_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_ENTER_EDIT_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_COMMIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_CAN_RESET_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_CAN_RESET_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeEditorScriptLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetSceneImportAsset"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_SCENE_IMPORT_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetLevelAsset"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_LEVEL_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetActors"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_RESET_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LevelInstanceGetEditableActors"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_GET_EDITABLE_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LevelInstanceEnterEditMode"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_ENTER_EDIT_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LevelInstanceCommit"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_LEVEL_INSTANCE_COMMIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanResetWorld"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_CAN_RESET_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanResetActor"),
            &raw mut U_INTERCHANGE_EDITOR_SCRIPT_LIBRARY_CAN_RESET_ACTOR,
        );
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_InterchangeSceneImportAsset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_InterchangeSceneImportAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_InterchangeSceneImportAsset")
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
pub struct UInterchangeEditorScriptLibrary {
    __padding_end: [u8; 48],
}
impl UInterchangeEditorScriptLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeEditorScriptLibrary")
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
pub struct UInterchangeFbxAssetImportDataConverter {
    __padding_end: [u8; 48],
}
impl UInterchangeFbxAssetImportDataConverter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFbxAssetImportDataConverter")
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
