#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_blueprint_editor_tool_menu_context_get_blueprint_obj: *mut crate::ffi::UFunctionOpague,
    pub u_json_object_graph_function_library_write_package_to_temp_file: *mut crate::ffi::UFunctionOpague,
    pub u_json_object_graph_function_library_write_blueprint_class_to_temp_file: *mut crate::ffi::UFunctionOpague,
    pub u_json_object_graph_function_library_stringify: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_blueprint_editor_tool_menu_context_get_blueprint_obj: std::ptr::null_mut(),
            u_json_object_graph_function_library_write_package_to_temp_file: std::ptr::null_mut(),
            u_json_object_graph_function_library_write_blueprint_class_to_temp_file: std::ptr::null_mut(),
            u_json_object_graph_function_library_stringify: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintEditorToolMenuContext::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlueprintObj"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_editor_tool_menu_context_get_blueprint_obj,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UJsonObjectGraphFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WritePackageToTempFile"),
                &raw mut __FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_package_to_temp_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WriteBlueprintClassToTempFile"),
                &raw mut __FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_blueprint_class_to_temp_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Stringify"),
                &raw mut __FUNCTION_PTRS.u_json_object_graph_function_library_stringify,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintEditorToolMenuContext {
    __padding_end: [u8; 64],
}
impl UBlueprintEditorToolMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintEditorToolMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintEditorToolMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_blueprint_obj(&self) -> UPtr<crate::bindings::engine::UBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_blueprint_editor_tool_menu_context_get_blueprint_obj,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_blueprint_editor_tool_menu_context_get_blueprint_obj,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct USSCSEditorMenuContext {
    __padding_end: [u8; 88],
}
impl USSCSEditorMenuContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USSCSEditorMenuContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USSCSEditorMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCompilerExtension {
    __padding_end: [u8; 48],
}
impl UBlueprintCompilerExtension {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCompilerExtension")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCompilerExtension")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintPaletteFavorites {
    __padding_end: [u8; 128],
}
impl UBlueprintPaletteFavorites {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintPaletteFavorites")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintPaletteFavorites")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UJsonObjectGraphFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UJsonObjectGraphFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJsonObjectGraphFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UJsonObjectGraphFunctionLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn write_package_to_temp_file(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        label: FString,
        options: crate::bindings::json_object_graph::FJsonStringifyOptions,
        out_filename: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_package_to_temp_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&label, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::json_object_graph::FJsonStringifyOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_filename,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::kismet::UJsonObjectGraphFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_package_to_temp_file,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_filename);
        }
    }
    pub fn write_blueprint_class_to_temp_file(
        bp: UPtr<crate::bindings::engine::UBlueprint>,
        label: FString,
        options: crate::bindings::json_object_graph::FJsonStringifyOptions,
        out_filename: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_blueprint_class_to_temp_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&label, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::json_object_graph::FJsonStringifyOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_filename,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::kismet::UJsonObjectGraphFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_write_blueprint_class_to_temp_file,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_filename);
        }
    }
    pub fn stringify(
        root_objects: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        options: crate::bindings::json_object_graph::FJsonStringifyOptions,
        result_string: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_stringify,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_objects,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::json_object_graph::FJsonStringifyOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result_string,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::kismet::UJsonObjectGraphFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::kismet::__FUNCTION_PTRS
                    .u_json_object_graph_function_library_stringify,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FString>().swap(result_string);
        }
    }
}
#[repr(C, align(8))]
pub struct USCSEditorExtensionContext {
    __padding_end: [u8; 80],
}
impl USCSEditorExtensionContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USCSEditorExtensionContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USCSEditorExtensionContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USubobjectEditorExtensionContext {
    __padding_end: [u8; 64],
}
impl USubobjectEditorExtensionContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectEditorExtensionContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectEditorExtensionContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
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
