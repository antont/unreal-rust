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
pub static mut U_BLUEPRINT_EDITOR_TOOL_MENU_CONTEXT_GET_BLUEPRINT_OBJ: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_WRITE_PACKAGE_TO_TEMP_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_WRITE_BLUEPRINT_CLASS_TO_TEMP_FILE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_STRINGIFY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlueprintEditorToolMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintObj"),
            &raw mut U_BLUEPRINT_EDITOR_TOOL_MENU_CONTEXT_GET_BLUEPRINT_OBJ,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UJsonObjectGraphFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WritePackageToTempFile"),
            &raw mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_WRITE_PACKAGE_TO_TEMP_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteBlueprintClassToTempFile"),
            &raw mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_WRITE_BLUEPRINT_CLASS_TO_TEMP_FILE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stringify"),
            &raw mut U_JSON_OBJECT_GRAPH_FUNCTION_LIBRARY_STRINGIFY,
        );
    }
}
#[repr(C, align(8))]
pub struct UBlueprintEditorToolMenuContext {
    __padding_end: [u8; 64],
}
impl UBlueprintEditorToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintEditorToolMenuContext")
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
pub struct USSCSEditorMenuContext {
    __padding_end: [u8; 88],
}
impl USSCSEditorMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USSCSEditorMenuContext")
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
pub struct UBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl UBlueprintCompilerExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCompilerExtension")
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
pub struct UBlueprintPaletteFavorites {
    __padding_end: [u8; 128],
}
impl UBlueprintPaletteFavorites {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintPaletteFavorites")
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
pub struct UJsonObjectGraphFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UJsonObjectGraphFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJsonObjectGraphFunctionLibrary")
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
pub struct USCSEditorExtensionContext {
    __padding_end: [u8; 80],
}
impl USCSEditorExtensionContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USCSEditorExtensionContext")
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
pub struct USubobjectEditorExtensionContext {
    __padding_end: [u8; 64],
}
impl USubobjectEditorExtensionContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectEditorExtensionContext")
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
pub struct EBPDependencyType(pub i32);
impl EBPDependencyType {
    pub const ASSET: EBPDependencyType = EBPDependencyType(0);
    pub const STRUCT: EBPDependencyType = EBPDependencyType(1);
    pub const CLASS: EBPDependencyType = EBPDependencyType(2);
}
#[repr(transparent)]
pub struct EFiBIndexAllPermission(pub i32);
impl EFiBIndexAllPermission {
    pub const NONE: EFiBIndexAllPermission = EFiBIndexAllPermission(0);
    pub const LOAD_ONLY: EFiBIndexAllPermission = EFiBIndexAllPermission(1);
    pub const CHECKOUT_AND_RESAVE: EFiBIndexAllPermission = EFiBIndexAllPermission(2);
}
