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
    pub u_interchange_editor_script_library_reset_scene_import_asset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_reset_level_asset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_reset_actors: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_level_instance_get_editable_actors: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_level_instance_enter_edit_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_level_instance_commit: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_can_reset_world: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_editor_script_library_can_reset_actor: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_interchange_editor_script_library_reset_scene_import_asset: std::ptr::null_mut(),
            u_interchange_editor_script_library_reset_level_asset: std::ptr::null_mut(),
            u_interchange_editor_script_library_reset_actors: std::ptr::null_mut(),
            u_interchange_editor_script_library_level_instance_get_editable_actors: std::ptr::null_mut(),
            u_interchange_editor_script_library_level_instance_enter_edit_mode: std::ptr::null_mut(),
            u_interchange_editor_script_library_level_instance_commit: std::ptr::null_mut(),
            u_interchange_editor_script_library_can_reset_world: std::ptr::null_mut(),
            u_interchange_editor_script_library_can_reset_actor: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeEditorScriptLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetSceneImportAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_scene_import_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetLevelAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_level_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetActors"),
                &raw mut __FUNCTION_PTRS.u_interchange_editor_script_library_reset_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LevelInstanceGetEditableActors"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_get_editable_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LevelInstanceEnterEditMode"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_enter_edit_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LevelInstanceCommit"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_commit,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanResetWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanResetActor"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_actor,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_InterchangeSceneImportAsset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_InterchangeSceneImportAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_InterchangeSceneImportAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_InterchangeSceneImportAsset")
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
pub struct UInterchangeEditorScriptLibrary {
    __padding_end: [u8; 48],
}
impl UInterchangeEditorScriptLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeEditorScriptLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeEditorScriptLibrary")
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
    pub fn reset_scene_import_asset(
        scene_import_asset: UPtr<
            crate::bindings::interchange_engine::UInterchangeSceneImportAsset,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_scene_import_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_import_asset,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_engine::UInterchangeSceneImportAsset,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_scene_import_asset,
                __buffer,
            )
        };
    }
    pub fn reset_level_asset(world: UPtr<crate::bindings::engine::UWorld>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_level_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_level_asset,
                __buffer,
            )
        };
    }
    pub fn reset_actors(actors: TArray<UPtr<crate::bindings::engine::AActor>>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_reset_actors,
                __buffer,
            )
        };
    }
    pub fn level_instance_get_editable_actors(
        level_instance: UPtr<crate::bindings::engine::ALevelInstance>,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_get_editable_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ALevelInstance>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_get_editable_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn level_instance_enter_edit_mode(
        level_instance: UPtr<crate::bindings::engine::ALevelInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_enter_edit_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ALevelInstance>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_enter_edit_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn level_instance_commit(
        level_instance: UPtr<crate::bindings::engine::ALevelInstance>,
        b_discard_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_commit,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ALevelInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_discard_changes,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_level_instance_commit,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn can_reset_world(world: UPtr<crate::bindings::engine::UWorld>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn can_reset_actor(actor: UPtr<crate::bindings::engine::AActor>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_editor::UInterchangeEditorScriptLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_editor::__FUNCTION_PTRS
                    .u_interchange_editor_script_library_can_reset_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeFbxAssetImportDataConverter {
    __padding_end: [u8; 48],
}
impl UInterchangeFbxAssetImportDataConverter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFbxAssetImportDataConverter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFbxAssetImportDataConverter")
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
