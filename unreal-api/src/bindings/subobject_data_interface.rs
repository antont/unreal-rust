#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_subobject_data_blueprint_function_library_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_scene_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_root_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_root_actor: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_native_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_instanced_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_instanced_actor: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_inherited_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_handle_valid: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_default_scene_root: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_child_actor: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_attached_to: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_is_actor: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_variable_name: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_parent_handle: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_object_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_object: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_handle: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_data: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_get_associated_object: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_reparent: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_rename: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_edit: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_duplicate: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_delete: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_blueprint_function_library_can_copy: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_reparent_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_reparent_subobject: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_rename_subobject_member_variable: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_rename_subobject: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_paste_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_make_new_scene_root: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_k2_gather_subobject_data_for_instance: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_k2_gather_subobject_data_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_k2_find_subobject_data_from_handle: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_k2_delete_subobjects_from_instance: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_k2_delete_subobject_from_instance: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_is_valid_rename: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_find_handle_for_object: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_duplicate_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_detach_subobject: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_delete_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_delete_subobject: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_create_new_cpp_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_create_new_bp_component: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_copy_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_change_subobject_class: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_can_paste_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_can_copy_subobjects: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_attach_subobject: *mut crate::ffi::UFunctionOpague,
    pub u_subobject_data_subsystem_add_new_subobject: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_subobject_data_blueprint_function_library_is_valid: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_scene_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_root_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_root_actor: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_native_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_instanced_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_instanced_actor: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_inherited_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_handle_valid: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_default_scene_root: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_component: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_child_actor: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_attached_to: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_is_actor: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_variable_name: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_parent_handle: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_object_for_blueprint: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_object: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_handle: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_display_name: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_data: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_blueprint: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_get_associated_object: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_reparent: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_rename: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_edit: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_duplicate: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_delete: std::ptr::null_mut(),
            u_subobject_data_blueprint_function_library_can_copy: std::ptr::null_mut(),
            u_subobject_data_subsystem_reparent_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_reparent_subobject: std::ptr::null_mut(),
            u_subobject_data_subsystem_rename_subobject_member_variable: std::ptr::null_mut(),
            u_subobject_data_subsystem_rename_subobject: std::ptr::null_mut(),
            u_subobject_data_subsystem_paste_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_make_new_scene_root: std::ptr::null_mut(),
            u_subobject_data_subsystem_k2_gather_subobject_data_for_instance: std::ptr::null_mut(),
            u_subobject_data_subsystem_k2_gather_subobject_data_for_blueprint: std::ptr::null_mut(),
            u_subobject_data_subsystem_k2_find_subobject_data_from_handle: std::ptr::null_mut(),
            u_subobject_data_subsystem_k2_delete_subobjects_from_instance: std::ptr::null_mut(),
            u_subobject_data_subsystem_k2_delete_subobject_from_instance: std::ptr::null_mut(),
            u_subobject_data_subsystem_is_valid_rename: std::ptr::null_mut(),
            u_subobject_data_subsystem_find_handle_for_object: std::ptr::null_mut(),
            u_subobject_data_subsystem_duplicate_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_detach_subobject: std::ptr::null_mut(),
            u_subobject_data_subsystem_delete_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_delete_subobject: std::ptr::null_mut(),
            u_subobject_data_subsystem_create_new_cpp_component: std::ptr::null_mut(),
            u_subobject_data_subsystem_create_new_bp_component: std::ptr::null_mut(),
            u_subobject_data_subsystem_copy_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_change_subobject_class: std::ptr::null_mut(),
            u_subobject_data_subsystem_can_paste_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_can_copy_subobjects: std::ptr::null_mut(),
            u_subobject_data_subsystem_attach_subobject: std::ptr::null_mut(),
            u_subobject_data_subsystem_add_new_subobject: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USubobjectDataBlueprintFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValid"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSceneComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_scene_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRootComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRootActor"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNativeComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_native_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInstancedComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInstancedActor"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInheritedComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_inherited_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHandleValid"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_handle_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDefaultSceneRoot"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_default_scene_root,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsChildActor"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_child_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAttachedTo"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_attached_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActor"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariableName"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_variable_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_parent_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObject"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayName"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetData"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssociatedObject"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_associated_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanReparent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_reparent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanRename"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_rename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanEdit"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_edit,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanDuplicate"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_duplicate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanDelete"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_delete,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanCopy"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_copy,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USubobjectDataSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReparentSubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_reparent_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReparentSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_reparent_subobject,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameSubobjectMemberVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_rename_subobject_member_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_rename_subobject,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PasteSubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_paste_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeNewSceneRoot"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_make_new_scene_root,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GatherSubobjectDataForInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GatherSubobjectDataForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_FindSubobjectDataFromHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_find_subobject_data_from_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_DeleteSubobjectsFromInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobjects_from_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_DeleteSubobjectFromInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobject_from_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidRename"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_is_valid_rename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindHandleForObject"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_find_handle_for_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DuplicateSubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_duplicate_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DetachSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_detach_subobject,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteSubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_delete_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_delete_subobject,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewCPPComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_cpp_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewBPComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_bp_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CopySubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_copy_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ChangeSubobjectClass"),
                &raw mut __FUNCTION_PTRS
                    .u_subobject_data_subsystem_change_subobject_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanPasteSubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_can_paste_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanCopySubobjects"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_can_copy_subobjects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AttachSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_attach_subobject,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddNewSubobject"),
                &raw mut __FUNCTION_PTRS.u_subobject_data_subsystem_add_new_subobject,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FSubobjectData {
    pub(crate) __padding_end: [u8; 88],
}
impl FSubobjectData {}
#[repr(C, align(8))]
pub struct FSubobjectDataHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FSubobjectDataHandle {}
#[repr(C, align(8))]
pub struct FAddNewSubobjectParams {
    pub parent_handle: FSubobjectDataHandle,
    pub new_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 8],
    pub blueprint_context: UPtr<crate::bindings::engine::UBlueprint>,
    pub flags_40: u8,
}
impl FAddNewSubobjectParams {}
#[repr(C, align(8))]
pub struct FReparentSubobjectParams {
    pub new_parent_handle: FSubobjectDataHandle,
    pub blueprint_context: UPtr<crate::bindings::engine::UBlueprint>,
    pub actor_preview_context: UPtr<crate::bindings::engine::AActor>,
}
impl FReparentSubobjectParams {}
#[repr(C, align(8))]
pub struct USubobjectDataBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USubobjectDataBlueprintFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectDataBlueprintFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectDataBlueprintFunctionLibrary")
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
    pub fn is_valid(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_scene_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_scene_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_scene_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_root_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_root_actor(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_root_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_native_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_native_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_native_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_instanced_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_instanced_actor(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_instanced_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_inherited_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_inherited_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_inherited_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_handle_valid(data_handle: &FSubobjectDataHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_handle_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_handle_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_default_scene_root(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_default_scene_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_default_scene_root,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_component(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_child_actor(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_child_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_child_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn is_attached_to(
        data: &FSubobjectData,
        in_handle: &FSubobjectDataHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_attached_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_handle,
                __buffer.add(88).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_attached_to,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn is_actor(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_is_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn get_variable_name(data: &FSubobjectData) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<100>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_variable_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_variable_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FName>().read() }
    }
    pub fn get_parent_handle(
        data: &FSubobjectData,
        out_handle: &mut FSubobjectDataHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_parent_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_handle,
                __buffer.add(88).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_parent_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<FSubobjectDataHandle>().swap(out_handle);
        }
    }
    pub fn get_object_for_blueprint(
        data: &FSubobjectData,
        blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blueprint,
                __buffer.add(88).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(blueprint);
        unsafe {
            __buffer
                .add(96)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_object(
        data: &FSubobjectData,
        b_even_if_pending_kill: bool,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_even_if_pending_kill,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_object,
                __buffer,
            )
        };
        std::mem::forget(b_even_if_pending_kill);
        unsafe {
            __buffer
                .add(96)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_handle(data: &FSubobjectData, out_handle: &mut FSubobjectDataHandle) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_handle,
                __buffer.add(88).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<FSubobjectDataHandle>().swap(out_handle);
        }
    }
    pub fn get_display_name(data: &FSubobjectData) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FText>().read() }
    }
    pub fn get_data(data_handle: &FSubobjectDataHandle, out_data: &mut FSubobjectData) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_data,
                __buffer.add(16).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FSubobjectData>().swap(out_data);
        }
    }
    pub fn get_blueprint(
        data: &FSubobjectData,
    ) -> UPtr<crate::bindings::engine::UBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<UPtr<crate::bindings::engine::UBlueprint>>().read()
        }
    }
    pub fn get_associated_object(
        data: &FSubobjectData,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_associated_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_get_associated_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn can_reparent(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_reparent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_reparent,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn can_rename(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_rename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_rename,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn can_edit(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_edit,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_edit,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn can_duplicate(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_duplicate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_duplicate,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn can_delete(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_delete,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_delete,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn can_copy(data: &FSubobjectData) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_copy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_blueprint_function_library_can_copy,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct USubobjectDataSubsystem {
    __padding_end: [u8; 136],
}
impl USubobjectDataSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectDataSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubobjectDataSubsystem")
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
    pub fn reparent_subobjects(
        &mut self,
        params: &FReparentSubobjectParams,
        handles_to_move: &TArray<FSubobjectDataHandle>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_reparent_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FReparentSubobjectParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                handles_to_move,
                __buffer.add(32).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_reparent_subobjects,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn reparent_subobject(
        &mut self,
        params: &FReparentSubobjectParams,
        to_reparent_handle: &FSubobjectDataHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_reparent_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FReparentSubobjectParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_reparent_handle,
                __buffer.add(32).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_reparent_subobject,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn rename_subobject_member_variable(
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
        in_handle: &FSubobjectDataHandle,
        new_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_rename_subobject_member_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_handle,
                __buffer.add(8).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_rename_subobject_member_variable,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
        std::mem::forget(new_name);
    }
    pub fn rename_subobject(
        &mut self,
        handle: &FSubobjectDataHandle,
        in_new_name: &FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_rename_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(16).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_rename_subobject,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn paste_subobjects(
        &mut self,
        paste_to_context: &FSubobjectDataHandle,
        new_parent_handles: &TArray<FSubobjectDataHandle>,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
        out_pasted_handles: &mut TArray<FSubobjectDataHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_paste_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                paste_to_context,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_parent_handles,
                __buffer.add(16).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pasted_handles,
                __buffer.add(40).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_paste_subobjects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<FSubobjectDataHandle>>()
                .swap(out_pasted_handles);
        }
        std::mem::forget(bp_context);
    }
    pub fn make_new_scene_root(
        &mut self,
        context: &FSubobjectDataHandle,
        new_scene_root: &FSubobjectDataHandle,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_make_new_scene_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_scene_root,
                __buffer.add(16).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_make_new_scene_root,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn k2_gather_subobject_data_for_instance(
        &mut self,
        context: UPtr<crate::bindings::engine::AActor>,
        out_array: &mut TArray<FSubobjectDataHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_array,
                __buffer.add(8).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FSubobjectDataHandle>>().swap(out_array);
        }
        std::mem::forget(context);
    }
    pub fn k2_gather_subobject_data_for_blueprint(
        &mut self,
        context: UPtr<crate::bindings::engine::UBlueprint>,
        out_array: &mut TArray<FSubobjectDataHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_array,
                __buffer.add(8).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_gather_subobject_data_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FSubobjectDataHandle>>().swap(out_array);
        }
        std::mem::forget(context);
    }
    pub fn k2_find_subobject_data_from_handle(
        &self,
        handle: &FSubobjectDataHandle,
        out_data: &mut FSubobjectData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_find_subobject_data_from_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_data,
                __buffer.add(16).cast::<FSubobjectData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_find_subobject_data_from_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FSubobjectData>().swap(out_data);
        }
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn k2_delete_subobjects_from_instance(
        &mut self,
        context_handle: &FSubobjectDataHandle,
        subobjects_to_delete: &TArray<FSubobjectDataHandle>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobjects_from_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                subobjects_to_delete,
                __buffer.add(16).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobjects_from_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn k2_delete_subobject_from_instance(
        &mut self,
        context_handle: &FSubobjectDataHandle,
        subobject_to_delete: &FSubobjectDataHandle,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobject_from_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                subobject_to_delete,
                __buffer.add(16).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_k2_delete_subobject_from_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn is_valid_rename(
        &self,
        handle: &FSubobjectDataHandle,
        in_new_text: &FText,
        out_error_message: &mut FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_is_valid_rename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_text,
                __buffer.add(16).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_message,
                __buffer.add(32).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_is_valid_rename,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FText>().swap(out_error_message);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn find_handle_for_object(
        &self,
        context: &FSubobjectDataHandle,
        object_to_find: UPtr<crate::bindings::core_u_object::UObject>,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> FSubobjectDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_find_handle_for_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_find,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_find_handle_for_object,
                __buffer,
            )
        };
        std::mem::forget(object_to_find);
        std::mem::forget(bp_context);
        unsafe { __buffer.add(32).cast::<FSubobjectDataHandle>().read() }
    }
    pub fn duplicate_subobjects(
        &mut self,
        context: &FSubobjectDataHandle,
        subobjects_to_dup: &TArray<FSubobjectDataHandle>,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
        out_new_subobjects: &mut TArray<FSubobjectDataHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_duplicate_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                subobjects_to_dup,
                __buffer.add(16).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_new_subobjects,
                __buffer.add(40).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_duplicate_subobjects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<FSubobjectDataHandle>>()
                .swap(out_new_subobjects);
        }
        std::mem::forget(bp_context);
    }
    pub fn detach_subobject(
        &mut self,
        owner_handle: &FSubobjectDataHandle,
        child_to_remove: &FSubobjectDataHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_detach_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                owner_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                child_to_remove,
                __buffer.add(16).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_detach_subobject,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn delete_subobjects(
        &mut self,
        context_handle: &FSubobjectDataHandle,
        subobjects_to_delete: &TArray<FSubobjectDataHandle>,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_delete_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                subobjects_to_delete,
                __buffer.add(16).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_delete_subobjects,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn delete_subobject(
        &mut self,
        context_handle: &FSubobjectDataHandle,
        subobject_to_delete: &FSubobjectDataHandle,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_delete_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                subobject_to_delete,
                __buffer.add(16).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_delete_subobject,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn create_new_cpp_component(
        component_class: TSubclassOf<crate::bindings::engine::UActorComponent>,
        new_class_path: FString,
        new_class_name: FString,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_cpp_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::engine::UActorComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_class_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_class_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_cpp_component,
                __buffer,
            )
        };
        std::mem::forget(component_class);
        std::mem::forget(new_class_path);
        std::mem::forget(new_class_name);
        unsafe {
            __buffer
                .add(40)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn create_new_bp_component(
        component_class: TSubclassOf<crate::bindings::engine::UActorComponent>,
        new_class_path: FString,
        new_class_name: FString,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_bp_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::engine::UActorComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_class_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_class_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::subobject_data_interface::USubobjectDataSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_create_new_bp_component,
                __buffer,
            )
        };
        std::mem::forget(component_class);
        std::mem::forget(new_class_path);
        std::mem::forget(new_class_name);
        unsafe {
            __buffer
                .add(40)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn copy_subobjects(
        &mut self,
        handles: &TArray<FSubobjectDataHandle>,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_copy_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handles,
                __buffer.add(0).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_copy_subobjects,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
    }
    pub fn change_subobject_class(
        &mut self,
        handle: &FSubobjectDataHandle,
        new_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_change_subobject_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_change_subobject_class,
                __buffer,
            )
        };
        std::mem::forget(new_class);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn can_paste_subobjects(
        &self,
        root_handle: &FSubobjectDataHandle,
        bp_context: UPtr<crate::bindings::engine::UBlueprint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_can_paste_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bp_context,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UBlueprint>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_can_paste_subobjects,
                __buffer,
            )
        };
        std::mem::forget(bp_context);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn can_copy_subobjects(&self, handles: &TArray<FSubobjectDataHandle>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_can_copy_subobjects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handles,
                __buffer.add(0).cast::<TArray<FSubobjectDataHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_can_copy_subobjects,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn attach_subobject(
        &mut self,
        owner_handle: &FSubobjectDataHandle,
        child_to_add_handle: &FSubobjectDataHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_attach_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                owner_handle,
                __buffer.add(0).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                child_to_add_handle,
                __buffer.add(16).cast::<FSubobjectDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_attach_subobject,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_new_subobject(
        &mut self,
        params: &FAddNewSubobjectParams,
        fail_reason: &mut FText,
    ) -> FSubobjectDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_add_new_subobject,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FAddNewSubobjectParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                fail_reason,
                __buffer.add(48).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::subobject_data_interface::__FUNCTION_PTRS
                    .u_subobject_data_subsystem_add_new_subobject,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<FText>().swap(fail_reason);
        }
        unsafe { __buffer.add(64).cast::<FSubobjectDataHandle>().read() }
    }
}
