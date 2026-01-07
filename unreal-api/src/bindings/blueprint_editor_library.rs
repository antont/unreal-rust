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
    pub fn upgrade_operator_nodes(blueprint: UPtr<crate::bindings::engine::UBlueprint>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_UPGRADE_OPERATOR_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_UPGRADE_OPERATOR_NODES,
                __buffer,
            )
        };
    }
    pub fn set_blueprint_variable_instance_editable(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        variable_name: &FName,
        b_instance_editable: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_INSTANCE_EDITABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_instance_editable,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_INSTANCE_EDITABLE,
                __buffer,
            )
        };
    }
    pub fn set_blueprint_variable_expose_to_cinematics(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        variable_name: &FName,
        b_expose_to_cinematics: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_TO_CINEMATICS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_expose_to_cinematics,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_TO_CINEMATICS,
                __buffer,
            )
        };
    }
    pub fn set_blueprint_variable_expose_on_spawn(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        variable_name: &FName,
        b_expose_on_spawn: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_ON_SPAWN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_expose_on_spawn,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_SET_BLUEPRINT_VARIABLE_EXPOSE_ON_SPAWN,
                __buffer,
            )
        };
    }
    pub fn replace_variable_references(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        old_var_name: FName,
        new_var_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REPLACE_VARIABLE_REFERENCES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_var_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_var_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REPLACE_VARIABLE_REFERENCES,
                __buffer,
            )
        };
    }
    pub fn reparent_blueprint(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        new_parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REPARENT_BLUEPRINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REPARENT_BLUEPRINT,
                __buffer,
            )
        };
    }
    pub fn rename_graph(
        graph: UPtr<crate::bindings::engine::UEdGraph>,
        new_name_str: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_RENAME_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UEdGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name_str,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_RENAME_GRAPH,
                __buffer,
            )
        };
    }
    pub fn remove_unused_variables(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_VARIABLES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_VARIABLES,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn remove_unused_nodes(blueprint: UPtr<crate::bindings::engine::UBlueprint>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_UNUSED_NODES,
                __buffer,
            )
        };
    }
    pub fn remove_graph(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        graph: UPtr<crate::bindings::engine::UEdGraph>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UEdGraph>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_GRAPH,
                __buffer,
            )
        };
    }
    pub fn remove_function_graph(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        func_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_FUNCTION_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &func_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REMOVE_FUNCTION_GRAPH,
                __buffer,
            )
        };
    }
    pub fn refresh_open_editors_for_blueprint(
        bp: UPtr<crate::bindings::engine::UBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_OPEN_EDITORS_FOR_BLUEPRINT,
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
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_OPEN_EDITORS_FOR_BLUEPRINT,
                __buffer,
            )
        };
    }
    pub fn refresh_all_open_blueprint_editors() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_ALL_OPEN_BLUEPRINT_EDITORS,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_REFRESH_ALL_OPEN_BLUEPRINT_EDITORS,
                __buffer,
            )
        };
    }
    pub fn get_struct_type(
        struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_STRUCT_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &struct_type,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_STRUCT_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_set_type(
        contained_type: &crate::bindings::engine::FEdGraphPinType,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_SET_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                contained_type,
                __buffer.add(0).cast::<crate::bindings::engine::FEdGraphPinType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_SET_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_saved_by_engine_version(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_SAVED_BY_ENGINE_VERSION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_SAVED_BY_ENGINE_VERSION,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_object_reference_type(
        object_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_OBJECT_REFERENCE_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_OBJECT_REFERENCE_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_map_type(
        key_type: &crate::bindings::engine::FEdGraphPinType,
        value_type: &crate::bindings::engine::FEdGraphPinType,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_MAP_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                key_type,
                __buffer.add(0).cast::<crate::bindings::engine::FEdGraphPinType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value_type,
                __buffer.add(112).cast::<crate::bindings::engine::FEdGraphPinType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_MAP_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(224).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_current_engine_version() -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENT_ENGINE_VERSION,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_CURRENT_ENGINE_VERSION,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_class_reference_type(
        class_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_CLASS_REFERENCE_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_CLASS_REFERENCE_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_blueprint_for_class(
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_does_class_have_blueprint: &mut bool,
    ) -> UPtr<crate::bindings::engine::UBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_FOR_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_does_class_have_blueprint,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_FOR_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_does_class_have_blueprint);
        }
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::UBlueprint>>().read()
        }
    }
    pub fn get_blueprint_asset(
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::engine::UBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_ASSET,
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
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BLUEPRINT_ASSET,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::UBlueprint>>().read()
        }
    }
    pub fn get_basic_type_by_name(
        type_name: FName,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BASIC_TYPE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &type_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_BASIC_TYPE_BY_NAME,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn get_array_type(
        contained_type: &crate::bindings::engine::FEdGraphPinType,
    ) -> crate::bindings::engine::FEdGraphPinType {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_ARRAY_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                contained_type,
                __buffer.add(0).cast::<crate::bindings::engine::FEdGraphPinType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GET_ARRAY_TYPE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::engine::FEdGraphPinType>().read()
        }
    }
    pub fn generated_class(
        blueprint_obj: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GENERATED_CLASS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint_obj,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_GENERATED_CLASS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn find_graph(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        graph_name: FName,
    ) -> UPtr<crate::bindings::engine::UEdGraph> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_FIND_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_FIND_GRAPH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UEdGraph>>().read()
        }
    }
    pub fn find_event_graph(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> UPtr<crate::bindings::engine::UEdGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_FIND_EVENT_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_FIND_EVENT_GRAPH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::UEdGraph>>().read()
        }
    }
    pub fn create_blueprint_asset_with_parent(
        asset_path: FString,
        parent_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::engine::UBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_CREATE_BLUEPRINT_ASSET_WITH_PARENT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_CREATE_BLUEPRINT_ASSET_WITH_PARENT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UBlueprint>>().read()
        }
    }
    pub fn compile_blueprint(blueprint: UPtr<crate::bindings::engine::UBlueprint>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPILE_BLUEPRINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPILE_BLUEPRINT,
                __buffer,
            )
        };
    }
    pub fn compare_soft_object_save_version_to(
        object_to_check: TSoftObjectPtr<crate::bindings::core_u_object::UObject>,
        version_to_check: FString,
        result: &mut EAssetSaveVersionComparisonResults,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_SOFT_OBJECT_SAVE_VERSION_TO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_check,
                __buffer
                    .add(0)
                    .cast::<TSoftObjectPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &version_to_check,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(64).cast::<EAssetSaveVersionComparisonResults>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_SOFT_OBJECT_SAVE_VERSION_TO,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<EAssetSaveVersionComparisonResults>().swap(result);
        }
    }
    pub fn compare_asset_save_version_to(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        version_to_check: FString,
        result: &mut EAssetSaveVersionComparisonResults,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_ASSET_SAVE_VERSION_TO,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &version_to_check,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(24).cast::<EAssetSaveVersionComparisonResults>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_COMPARE_ASSET_SAVE_VERSION_TO,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<EAssetSaveVersionComparisonResults>().swap(result);
        }
    }
    pub fn add_member_variable_with_value(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        member_name: FName,
        default_value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE_WITH_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &member_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                default_value,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE_WITH_VALUE,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_member_variable(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        member_name: FName,
        variable_type: &crate::bindings::engine::FEdGraphPinType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<137>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &member_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_type,
                __buffer.add(24).cast::<crate::bindings::engine::FEdGraphPinType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_MEMBER_VARIABLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<bool>().read() }
    }
    pub fn add_function_graph(
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
        func_name: FString,
    ) -> UPtr<crate::bindings::engine::UEdGraph> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_FUNCTION_GRAPH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &func_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::blueprint_editor_library::UBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::blueprint_editor_library::U_BLUEPRINT_EDITOR_LIBRARY_ADD_FUNCTION_GRAPH,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UEdGraph>>().read()
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
