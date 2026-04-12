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
    pub u_data_layer_editor_subsystem_update_all_view_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_update_all_actors_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_update_actor_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_update_actor_all_views_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layer_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layers_is_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layers_is_dynamically_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layer_is_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_toggle_data_layer_is_dynamically_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_parent_data_layer_for_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_parent_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layer_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layers_is_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layers_is_dynamically_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layer_is_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layer_is_initially_visible: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layer_is_dynamically_loaded_in_editor: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_data_layer_initial_runtime_state: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_set_actor_editor_context_current_external_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_select_actors_in_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_select_actors_in_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_rename_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_selected_actors_from_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_selected_actors_from_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_from_actor_editor_context: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actors_from_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actors_from_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actors_from_all_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actor_from_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actor_from_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_remove_actor_from_all_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_make_all_data_layers_visible: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_is_actor_valid_for_data_layer_instances: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_is_actor_valid_for_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_data_layer_instances: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_data_layer_instance: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_data_layer_from_label: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_all_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_actors_from_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_actors_from_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_get_actor_editor_context_current_external_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_delete_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_delete_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_create_data_layer_instance: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_create_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_append_actors_from_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_append_actors_from_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_to_actor_editor_context: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_selected_actors_to_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_selected_actors_to_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_actor_to_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_actor_to_data_layer: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_actors_to_data_layers: *mut crate::ffi::UFunctionOpague,
    pub u_data_layer_editor_subsystem_add_actors_to_data_layer: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_data_layer_editor_subsystem_update_all_view_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_update_all_actors_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_update_actor_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_update_actor_all_views_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layer_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layers_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layers_is_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layers_is_dynamically_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layer_is_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_toggle_data_layer_is_dynamically_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_parent_data_layer_for_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_parent_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layer_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layers_visibility: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layers_is_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layers_is_dynamically_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layer_is_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layer_is_initially_visible: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layer_is_dynamically_loaded_in_editor: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_data_layer_initial_runtime_state: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_set_actor_editor_context_current_external_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_select_actors_in_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_select_actors_in_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_rename_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_selected_actors_from_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_selected_actors_from_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_from_actor_editor_context: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actors_from_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actors_from_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actors_from_all_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actor_from_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actor_from_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_remove_actor_from_all_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_make_all_data_layers_visible: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_is_actor_valid_for_data_layer_instances: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_is_actor_valid_for_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_data_layer_instances: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_data_layer_instance: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_data_layer_from_label: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_all_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_actors_from_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_actors_from_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_get_actor_editor_context_current_external_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_delete_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_delete_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_create_data_layer_instance: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_create_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_append_actors_from_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_append_actors_from_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_to_actor_editor_context: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_selected_actors_to_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_selected_actors_to_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_actor_to_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_actor_to_data_layer: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_actors_to_data_layers: std::ptr::null_mut(),
            u_data_layer_editor_subsystem_add_actors_to_data_layer: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataLayerEditorSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateAllViewVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_view_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateAllActorsVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_actors_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateActorVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateActorAllViewsVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_all_views_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayerVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayersVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayersIsLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayersIsDynamicallyLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_dynamically_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayerIsLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ToggleDataLayerIsDynamicallyLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_dynamically_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetParentDataLayerForDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer_for_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetParentDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayerVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayersVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayersIsLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayersIsDynamicallyLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_dynamically_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayerIsLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayerIsInitiallyVisible"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_initially_visible,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayerIsDynamicallyLoadedInEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_dynamically_loaded_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataLayerInitialRuntimeState"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_initial_runtime_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "SetActorEditorContextCurrentExternalDataLayer",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_actor_editor_context_current_external_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectActorsInDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectActorsInDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameDataLayer"),
                &raw mut __FUNCTION_PTRS.u_data_layer_editor_subsystem_rename_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSelectedActorsFromDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSelectedActorsFromDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveFromActorEditorContext"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_from_actor_editor_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorsFromDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorsFromDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorsFromAllDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_all_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorFromDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorFromDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActorFromAllDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_all_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeAllDataLayersVisible"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_make_all_data_layers_visible,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActorValidForDataLayerInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActorValidForDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDataLayerInstances"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDataLayerInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDataLayerFromLabel"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_from_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDataLayer"),
                &raw mut __FUNCTION_PTRS.u_data_layer_editor_subsystem_get_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_all_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsFromDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsFromDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetActorEditorContextCurrentExternalDataLayer",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actor_editor_context_current_external_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteDataLayers"),
                &raw mut __FUNCTION_PTRS.u_data_layer_editor_subsystem_delete_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteDataLayer"),
                &raw mut __FUNCTION_PTRS.u_data_layer_editor_subsystem_delete_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateDataLayerInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_create_data_layer_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateDataLayer"),
                &raw mut __FUNCTION_PTRS.u_data_layer_editor_subsystem_create_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AppendActorsFromDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AppendActorsFromDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddToActorEditorContext"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_to_actor_editor_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectedActorsToDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectedActorsToDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddActorToDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddActorToDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddActorsToDataLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddActorsToDataLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layer,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FDataLayerCreationParameters {
    pub data_layer_asset: UPtr<crate::bindings::engine::UDataLayerAsset>,
    pub world_data_layers: UPtr<crate::bindings::engine::AWorldDataLayers>,
    pub b_is_private: bool,
}
impl FDataLayerCreationParameters {}
#[repr(C, align(8))]
pub struct UDataLayerEditorState {
    __padding_end: [u8; 80],
}
impl UDataLayerEditorState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerEditorState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerEditorState")
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
pub struct UActorEditorContextDataLayerState {
    __padding_end: [u8; 72],
}
impl UActorEditorContextDataLayerState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextDataLayerState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextDataLayerState")
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
pub struct UDataLayerEditorSubsystem {
    __padding_end: [u8; 344],
}
impl UDataLayerEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerEditorSubsystem")
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
    pub fn update_all_view_visibility(
        &mut self,
        data_layer_that_changed: UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_view_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer_that_changed,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_view_visibility,
                __buffer,
            )
        };
        std::mem::forget(data_layer_that_changed);
    }
    pub fn update_all_actors_visibility(
        &mut self,
        b_notify_selection_change: bool,
        b_redraw_viewports: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_actors_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_notify_selection_change,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_redraw_viewports,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_all_actors_visibility,
                __buffer,
            )
        };
        std::mem::forget(b_notify_selection_change);
        std::mem::forget(b_redraw_viewports);
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn update_actor_visibility(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_out_selection_changed: &mut bool,
        b_out_actor_modified: &mut bool,
        b_notify_selection_change: bool,
        b_redraw_viewports: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_visibility,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_selection_changed,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_actor_modified,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_notify_selection_change,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_redraw_viewports,
                __buffer.add(11).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_visibility,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_selection_changed);
        }
        unsafe {
            __buffer.add(9).cast::<bool>().swap(b_out_actor_modified);
        }
        std::mem::forget(actor);
        std::mem::forget(b_notify_selection_change);
        std::mem::forget(b_redraw_viewports);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn update_actor_all_views_visibility(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_all_views_visibility,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_update_actor_all_views_visibility,
                __buffer,
            )
        };
        std::mem::forget(actor);
    }
    pub fn toggle_data_layer_visibility(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_visibility,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
    }
    pub fn toggle_data_layers_visibility(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_visibility,
                __buffer,
            )
        };
    }
    pub fn toggle_data_layers_is_loaded_in_editor(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn toggle_data_layers_is_dynamically_loaded_in_editor(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layers_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn toggle_data_layer_is_loaded_in_editor(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn toggle_data_layer_is_dynamically_loaded_in_editor(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_toggle_data_layer_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_parent_data_layer_for_data_layers(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        parent_data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer_for_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent_data_layer,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer_for_data_layers,
                __buffer,
            )
        };
        std::mem::forget(parent_data_layer);
    }
    pub fn set_parent_data_layer(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        parent_data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent_data_layer,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_parent_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(parent_data_layer);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_data_layer_visibility(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        b_is_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_visible,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_visibility,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_visible);
    }
    pub fn set_data_layers_visibility(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        b_is_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_visible,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_visibility,
                __buffer,
            )
        };
        std::mem::forget(b_is_visible);
    }
    pub fn set_data_layers_is_loaded_in_editor(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        b_is_loaded_in_editor: bool,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_loaded_in_editor,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(b_is_loaded_in_editor);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_data_layers_is_dynamically_loaded_in_editor(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>,
        b_is_loaded_in_editor: bool,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_loaded_in_editor,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layers_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(b_is_loaded_in_editor);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_data_layer_is_loaded_in_editor(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        b_is_loaded_in_editor: bool,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_loaded_in_editor,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_loaded_in_editor);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn set_data_layer_is_initially_visible(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        b_is_initially_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_initially_visible,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_initially_visible,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_initially_visible,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_initially_visible);
    }
    pub fn set_data_layer_is_dynamically_loaded_in_editor(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>,
        b_is_loaded_in_editor: bool,
        b_is_from_user_change: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDEPRECATED_DataLayer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_loaded_in_editor,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_from_user_change,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_is_dynamically_loaded_in_editor,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_is_loaded_in_editor);
        std::mem::forget(b_is_from_user_change);
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn set_data_layer_initial_runtime_state(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        initial_runtime_state: crate::bindings::engine::EDataLayerRuntimeState,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_initial_runtime_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &initial_runtime_state,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EDataLayerRuntimeState>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_data_layer_initial_runtime_state,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(initial_runtime_state);
    }
    pub fn set_actor_editor_context_current_external_data_layer(
        &mut self,
        in_external_data_layer_asset: UPtr<
            crate::bindings::engine::UExternalDataLayerAsset,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_actor_editor_context_current_external_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_external_data_layer_asset,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UExternalDataLayerAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_set_actor_editor_context_current_external_data_layer,
                __buffer,
            )
        };
        std::mem::forget(in_external_data_layer_asset);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn select_actors_in_data_layers(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        b_select: bool,
        b_notify: bool,
        b_select_even_if_hidden: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_notify, __buffer.add(17).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_even_if_hidden,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layers,
                __buffer,
            )
        };
        std::mem::forget(b_select);
        std::mem::forget(b_notify);
        std::mem::forget(b_select_even_if_hidden);
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn select_actors_in_data_layer(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        b_select: bool,
        b_notify: bool,
        b_select_even_if_hidden: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_notify, __buffer.add(9).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_even_if_hidden,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_select_actors_in_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        std::mem::forget(b_select);
        std::mem::forget(b_notify);
        std::mem::forget(b_select_even_if_hidden);
        unsafe { __buffer.add(11).cast::<bool>().read() }
    }
    pub fn rename_data_layer(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        new_data_layer_label: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_rename_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_data_layer_label,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_rename_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn remove_selected_actors_from_data_layers(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_selected_actors_from_data_layer(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_selected_actors_from_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_from_actor_editor_context(
        &mut self,
        in_data_layer_instance: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_from_actor_editor_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_layer_instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_from_actor_editor_context,
                __buffer,
            )
        };
        std::mem::forget(in_data_layer_instance);
    }
    pub fn remove_actors_from_data_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_actors_from_data_layer(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_actors_from_all_data_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_all_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actors_from_all_data_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_actor_from_data_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layers,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layers,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_actor_from_data_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        data_layer_to_remove: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layer,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer_to_remove,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_data_layer,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(data_layer_to_remove);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_actor_from_all_data_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_all_data_layers,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_remove_actor_from_all_data_layers,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn make_all_data_layers_visible(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_make_all_data_layers_visible,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_make_all_data_layers_visible,
                __buffer,
            )
        };
    }
    pub fn is_actor_valid_for_data_layer_instances(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        data_layer_instances: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer_instances,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layer_instances,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer_instances,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_actor_valid_for_data_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_is_actor_valid_for_data_layer,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_data_layer_instances(
        &self,
        data_layer_assets: TArray<UPtr<crate::bindings::engine::UDataLayerAsset>>,
    ) -> TArray<UPtr<crate::bindings::engine::UDataLayerInstance>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instances,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer_assets,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerAsset>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instances,
                __buffer,
            )
        };
        std::mem::forget(data_layer_assets);
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>()
                .read()
        }
    }
    pub fn get_data_layer_instance(
        &self,
        data_layer_asset: UPtr<crate::bindings::engine::UDataLayerAsset>,
    ) -> UPtr<crate::bindings::engine::UDataLayerInstance> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UDataLayerAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_instance,
                __buffer,
            )
        };
        std::mem::forget(data_layer_asset);
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>()
                .read()
        }
    }
    pub fn get_data_layer_from_label(
        &self,
        data_layer_label: &FName,
    ) -> UPtr<crate::bindings::engine::UDataLayerInstance> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_from_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layer_label,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer_from_label,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>()
                .read()
        }
    }
    pub fn get_data_layer(
        &self,
        actor_data_layer: &crate::bindings::engine::FActorDataLayer,
    ) -> UPtr<crate::bindings::engine::UDataLayerInstance> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actor_data_layer,
                __buffer.add(0).cast::<crate::bindings::engine::FActorDataLayer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_data_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>()
                .read()
        }
    }
    pub fn get_all_data_layers(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::UDataLayerInstance>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_all_data_layers,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_all_data_layers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>()
                .read()
        }
    }
    pub fn get_actors_from_data_layers(
        &self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actors_from_data_layer(
        &self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actors_from_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actor_editor_context_current_external_data_layer(
        &self,
    ) -> UPtr<crate::bindings::engine::UExternalDataLayerAsset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actor_editor_context_current_external_data_layer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_get_actor_editor_context_current_external_data_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UExternalDataLayerAsset>>()
                .read()
        }
    }
    pub fn delete_data_layers(
        &mut self,
        data_layers_to_delete: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_delete_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers_to_delete,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_delete_data_layers,
                __buffer,
            )
        };
    }
    pub fn delete_data_layer(
        &mut self,
        data_layer_to_delete: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_delete_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer_to_delete,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_delete_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer_to_delete);
    }
    pub fn create_data_layer_instance(
        &mut self,
        parameters: &FDataLayerCreationParameters,
    ) -> UPtr<crate::bindings::engine::UDataLayerInstance> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_create_data_layer_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(0).cast::<FDataLayerCreationParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_create_data_layer_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>()
                .read()
        }
    }
    pub fn create_data_layer(
        &mut self,
        parent_data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> UPtr<crate::bindings::engine::UDataLayerInstance> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_create_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent_data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_create_data_layer,
                __buffer,
            )
        };
        std::mem::forget(parent_data_layer);
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>()
                .read()
        }
    }
    pub fn append_actors_from_data_layers(
        &self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
        in_out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_actors,
                __buffer.add(16).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(in_out_actors);
        }
    }
    pub fn append_actors_from_data_layer(
        &self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
        in_out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_actors,
                __buffer.add(8).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_append_actors_from_data_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(in_out_actors);
        }
        std::mem::forget(data_layer);
    }
    pub fn add_to_actor_editor_context(
        &mut self,
        in_data_layer_instance: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_to_actor_editor_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data_layer_instance,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_to_actor_editor_context,
                __buffer,
            )
        };
        std::mem::forget(in_data_layer_instance);
    }
    pub fn add_selected_actors_to_data_layers(
        &mut self,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_selected_actors_to_data_layer(
        &mut self,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_selected_actors_to_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn add_actor_to_data_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layers,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layers,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_actor_to_data_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layer,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actor_to_data_layer,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(data_layer);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_actors_to_data_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        data_layers: &TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_layers,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::engine::UDataLayerInstance>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_actors_to_data_layer(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        data_layer: UPtr<crate::bindings::engine::UDataLayerInstance>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_layer,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UDataLayerInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::data_layer_editor::__FUNCTION_PTRS
                    .u_data_layer_editor_subsystem_add_actors_to_data_layer,
                __buffer,
            )
        };
        std::mem::forget(data_layer);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDataLayerFactory {
    __padding_end: [u8; 136],
}
impl UDataLayerFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerFactory")
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
pub struct UExternalDataLayerFactory {
    __padding_end: [u8; 136],
}
impl UExternalDataLayerFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExternalDataLayerFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExternalDataLayerFactory")
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
