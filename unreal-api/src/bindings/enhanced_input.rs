#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_enhanced_input_component_get_bound_action_value: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_request_rebuild_control_mappings_using_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_make_input_action_value_of_type: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_is_action_key_mapping_player_mappable: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_get_player_mappable_key_settings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_get_mapping_name: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_get_bound_action_value: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_flush_player_input: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_trigger_event_value_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_input_action_value_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_input_action_value_to_bool: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_input_action_value_to_axis3_d: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_input_action_value_to_axis2_d: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_conv_input_action_value_to_axis1_d: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_library_break_input_action_value: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_platform_data_get_context_redirect: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_player_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_player_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_player_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_set_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_request_rebuild_control_mappings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_remove_tags_from_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_remove_tag_from_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_remove_mapping_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_query_map_key_in_context_set: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_query_map_key_in_active_context_set: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_query_keys_mapped_to_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_on_user_settings_changed: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_on_user_key_profile_changed: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_inject_input_vector_for_player_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_inject_input_vector_for_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_inject_input_for_player_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_inject_input_for_action: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_has_mapping_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_get_user_settings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_get_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_get_all_player_mappable_action_key_mappings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_clear_all_mappings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_append_tags_to_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_add_tag_to_input_mode: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_subsystem_interface_add_mapping_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_local_player_subsystem_on_user_settings_post_initialized_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_local_player_subsystem_on_mapping_context_removed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_local_player_subsystem_on_mapping_context_added_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_local_player_subsystem_on_control_mappings_rebuilt_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_world_subsystem_remove_actor_input_component: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_world_subsystem_add_actor_input_component: *mut crate::ffi::UFunctionOpague,
    pub u_input_mapping_context_unmap_key: *mut crate::ffi::UFunctionOpague,
    pub u_input_mapping_context_unmap_all_keys_from_action: *mut crate::ffi::UFunctionOpague,
    pub u_input_mapping_context_unmap_all: *mut crate::ffi::UFunctionOpague,
    pub u_input_mapping_context_should_show_input_mode_query: *mut crate::ffi::UFunctionOpague,
    pub u_input_mapping_context_map_key: *mut crate::ffi::UFunctionOpague,
    pub u_input_modifier_modify_raw: *mut crate::ffi::UFunctionOpague,
    pub u_input_modifier_get_visualization_color: *mut crate::ffi::UFunctionOpague,
    pub u_input_trigger_update_state: *mut crate::ffi::UFunctionOpague,
    pub u_input_trigger_is_actuated: *mut crate::ffi::UFunctionOpague,
    pub u_input_trigger_get_trigger_type: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_is_deprecated: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_player_mappable_keys: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_mapping_contexts: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_mapping_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_keys_bound_to_action: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_input_config_get_config_name: *mut crate::ffi::UFunctionOpague,
    pub u_player_mappable_key_settings_get_known_mapping_names: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_set_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_reset_mapping_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_query_player_mapped_keys: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_k2_find_key_mapping: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_get_profile_id_string: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_get_profile_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_get_player_mapping_rows: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_get_mapping_names_for_key: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_get_mapped_keys_in_row: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_dump_profile_to_log: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_player_mappable_key_profile_does_mapping_pass_query_options: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_unregister_input_mapping_contexts: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_unregister_input_mapping_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_un_map_player_key: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_set_active_key_profile: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_save_settings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_reset_key_profile_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_reset_key_profile_id_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_reset_all_player_keys_in_row: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_register_input_mapping_contexts: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_register_input_mapping_context: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_map_player_key: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_mapping_context_registered_with_settings_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_mappable_key_profile_changed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_is_mapping_context_registered: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_get_key_profile_with_id: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_get_active_key_profile_id: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_get_active_key_profile: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_find_mappings_in_row: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_enhanced_input_user_settings_changed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_enhanced_input_user_settings_applied_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_create_new_key_profile: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_async_save_settings: *mut crate::ffi::UFunctionOpague,
    pub u_enhanced_input_user_settings_apply_settings: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_enhanced_input_component_get_bound_action_value: std::ptr::null_mut(),
            u_enhanced_input_library_request_rebuild_control_mappings_using_context: std::ptr::null_mut(),
            u_enhanced_input_library_make_input_action_value_of_type: std::ptr::null_mut(),
            u_enhanced_input_library_is_action_key_mapping_player_mappable: std::ptr::null_mut(),
            u_enhanced_input_library_get_player_mappable_key_settings: std::ptr::null_mut(),
            u_enhanced_input_library_get_mapping_name: std::ptr::null_mut(),
            u_enhanced_input_library_get_bound_action_value: std::ptr::null_mut(),
            u_enhanced_input_library_flush_player_input: std::ptr::null_mut(),
            u_enhanced_input_library_conv_trigger_event_value_to_string: std::ptr::null_mut(),
            u_enhanced_input_library_conv_input_action_value_to_string: std::ptr::null_mut(),
            u_enhanced_input_library_conv_input_action_value_to_bool: std::ptr::null_mut(),
            u_enhanced_input_library_conv_input_action_value_to_axis3_d: std::ptr::null_mut(),
            u_enhanced_input_library_conv_input_action_value_to_axis2_d: std::ptr::null_mut(),
            u_enhanced_input_library_conv_input_action_value_to_axis1_d: std::ptr::null_mut(),
            u_enhanced_input_library_break_input_action_value: std::ptr::null_mut(),
            u_enhanced_input_platform_data_get_context_redirect: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_player_mapping: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_player_mapping: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_player_mapping: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_set_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_request_rebuild_control_mappings: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_remove_tags_from_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_remove_tag_from_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_remove_mapping_context: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_query_map_key_in_context_set: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_query_map_key_in_active_context_set: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_query_keys_mapped_to_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_on_user_settings_changed: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_on_user_key_profile_changed: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_inject_input_vector_for_player_mapping: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_inject_input_vector_for_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_inject_input_for_player_mapping: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_inject_input_for_action: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_has_mapping_context: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_get_user_settings: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_get_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_get_all_player_mappable_action_key_mappings: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_clear_all_mappings: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_append_tags_to_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_add_tag_to_input_mode: std::ptr::null_mut(),
            u_enhanced_input_subsystem_interface_add_mapping_context: std::ptr::null_mut(),
            u_enhanced_input_local_player_subsystem_on_user_settings_post_initialized_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_local_player_subsystem_on_mapping_context_removed_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_local_player_subsystem_on_mapping_context_added_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_local_player_subsystem_on_control_mappings_rebuilt_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_world_subsystem_remove_actor_input_component: std::ptr::null_mut(),
            u_enhanced_input_world_subsystem_add_actor_input_component: std::ptr::null_mut(),
            u_input_mapping_context_unmap_key: std::ptr::null_mut(),
            u_input_mapping_context_unmap_all_keys_from_action: std::ptr::null_mut(),
            u_input_mapping_context_unmap_all: std::ptr::null_mut(),
            u_input_mapping_context_should_show_input_mode_query: std::ptr::null_mut(),
            u_input_mapping_context_map_key: std::ptr::null_mut(),
            u_input_modifier_modify_raw: std::ptr::null_mut(),
            u_input_modifier_get_visualization_color: std::ptr::null_mut(),
            u_input_trigger_update_state: std::ptr::null_mut(),
            u_input_trigger_is_actuated: std::ptr::null_mut(),
            u_input_trigger_get_trigger_type: std::ptr::null_mut(),
            u_player_mappable_input_config_reset_to_default: std::ptr::null_mut(),
            u_player_mappable_input_config_is_deprecated: std::ptr::null_mut(),
            u_player_mappable_input_config_get_player_mappable_keys: std::ptr::null_mut(),
            u_player_mappable_input_config_get_metadata: std::ptr::null_mut(),
            u_player_mappable_input_config_get_mapping_contexts: std::ptr::null_mut(),
            u_player_mappable_input_config_get_mapping_by_name: std::ptr::null_mut(),
            u_player_mappable_input_config_get_keys_bound_to_action: std::ptr::null_mut(),
            u_player_mappable_input_config_get_display_name: std::ptr::null_mut(),
            u_player_mappable_input_config_get_config_name: std::ptr::null_mut(),
            u_player_mappable_key_settings_get_known_mapping_names: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_to_string: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_set_display_name: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_reset_to_default: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_reset_mapping_to_default: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_query_player_mapped_keys: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_k2_find_key_mapping: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_get_profile_id_string: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_get_profile_display_name: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_get_player_mapping_rows: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_get_mapping_names_for_key: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_get_mapped_keys_in_row: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_dump_profile_to_log: std::ptr::null_mut(),
            u_enhanced_player_mappable_key_profile_does_mapping_pass_query_options: std::ptr::null_mut(),
            u_enhanced_input_user_settings_unregister_input_mapping_contexts: std::ptr::null_mut(),
            u_enhanced_input_user_settings_unregister_input_mapping_context: std::ptr::null_mut(),
            u_enhanced_input_user_settings_un_map_player_key: std::ptr::null_mut(),
            u_enhanced_input_user_settings_set_active_key_profile: std::ptr::null_mut(),
            u_enhanced_input_user_settings_save_settings: std::ptr::null_mut(),
            u_enhanced_input_user_settings_reset_key_profile_to_default: std::ptr::null_mut(),
            u_enhanced_input_user_settings_reset_key_profile_id_to_default: std::ptr::null_mut(),
            u_enhanced_input_user_settings_reset_all_player_keys_in_row: std::ptr::null_mut(),
            u_enhanced_input_user_settings_register_input_mapping_contexts: std::ptr::null_mut(),
            u_enhanced_input_user_settings_register_input_mapping_context: std::ptr::null_mut(),
            u_enhanced_input_user_settings_map_player_key: std::ptr::null_mut(),
            u_enhanced_input_user_settings_mapping_context_registered_with_settings_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_user_settings_mappable_key_profile_changed_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_user_settings_is_mapping_context_registered: std::ptr::null_mut(),
            u_enhanced_input_user_settings_get_key_profile_with_id: std::ptr::null_mut(),
            u_enhanced_input_user_settings_get_active_key_profile_id: std::ptr::null_mut(),
            u_enhanced_input_user_settings_get_active_key_profile: std::ptr::null_mut(),
            u_enhanced_input_user_settings_find_mappings_in_row: std::ptr::null_mut(),
            u_enhanced_input_user_settings_enhanced_input_user_settings_changed_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_user_settings_enhanced_input_user_settings_applied_delegate_signature: std::ptr::null_mut(),
            u_enhanced_input_user_settings_create_new_key_profile: std::ptr::null_mut(),
            u_enhanced_input_user_settings_async_save_settings: std::ptr::null_mut(),
            u_enhanced_input_user_settings_apply_settings: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActionValue"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_component_get_bound_action_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRebuildControlMappingsUsingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_request_rebuild_control_mappings_using_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeInputActionValueOfType"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_make_input_action_value_of_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActionKeyMappingPlayerMappable"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_is_action_key_mapping_player_mappable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerMappableKeySettings"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_get_player_mappable_key_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMappingName"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_library_get_mapping_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActionValue"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_library_get_bound_action_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FlushPlayerInput"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_library_flush_player_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_TriggerEventValueToString"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_trigger_event_value_to_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_InputActionValueToString"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_input_action_value_to_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_InputActionValueToBool"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_input_action_value_to_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_InputActionValueToAxis3D"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_input_action_value_to_axis3_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_InputActionValueToAxis2D"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_input_action_value_to_axis2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_InputActionValueToAxis1D"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_library_conv_input_action_value_to_axis1_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakInputActionValue"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_library_break_input_action_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputPlatformData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetContextRedirect"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_platform_data_get_context_redirect,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputSubsystemInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "UpdateValueOfContinuousInputInjectionForPlayerMapping",
            ),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_player_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateValueOfContinuousInputInjectionForAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopContinuousInputInjectionForPlayerMapping"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_player_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopContinuousInputInjectionForAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartContinuousInputInjectionForPlayerMapping"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_player_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartContinuousInputInjectionForAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInputMode"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_subsystem_interface_set_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestRebuildControlMappings"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_request_rebuild_control_mappings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTagsFromInputMode"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_remove_tags_from_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTagFromInputMode"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_remove_tag_from_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMappingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_remove_mapping_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryMapKeyInContextSet"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_query_map_key_in_context_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryMapKeyInActiveContextSet"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_query_map_key_in_active_context_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryKeysMappedToAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_query_keys_mapped_to_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnUserSettingsChanged"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_on_user_settings_changed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnUserKeyProfileChanged"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_on_user_key_profile_changed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InjectInputVectorForPlayerMapping"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_inject_input_vector_for_player_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InjectInputVectorForAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_inject_input_vector_for_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InjectInputForPlayerMapping"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_inject_input_for_player_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InjectInputForAction"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_inject_input_for_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasMappingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_has_mapping_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUserSettings"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_get_user_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputMode"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_subsystem_interface_get_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllPlayerMappableActionKeyMappings"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_get_all_player_mappable_action_key_mappings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllMappings"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_clear_all_mappings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AppendTagsToInputMode"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_append_tags_to_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTagToInputMode"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_add_tag_to_input_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMappingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_subsystem_interface_add_mapping_context,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputLocalPlayerSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnUserSettingsPostInitialized__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_local_player_subsystem_on_user_settings_post_initialized_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMappingContextRemoved__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_local_player_subsystem_on_mapping_context_removed_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMappingContextAdded__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_local_player_subsystem_on_mapping_context_added_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnControlMappingsRebuilt__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_local_player_subsystem_on_control_mappings_rebuilt_delegate_signature,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputWorldSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorInputComponent"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_world_subsystem_remove_actor_input_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorInputComponent"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_world_subsystem_add_actor_input_component,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInputMappingContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnmapKey"),
            &raw mut __FUNCTION_PTRS.u_input_mapping_context_unmap_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnmapAllKeysFromAction"),
            &raw mut __FUNCTION_PTRS.u_input_mapping_context_unmap_all_keys_from_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnmapAll"),
            &raw mut __FUNCTION_PTRS.u_input_mapping_context_unmap_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldShowInputModeQuery"),
            &raw mut __FUNCTION_PTRS.u_input_mapping_context_should_show_input_mode_query,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MapKey"),
            &raw mut __FUNCTION_PTRS.u_input_mapping_context_map_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInputModifier::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ModifyRaw"),
            &raw mut __FUNCTION_PTRS.u_input_modifier_modify_raw,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVisualizationColor"),
            &raw mut __FUNCTION_PTRS.u_input_modifier_get_visualization_color,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInputTrigger::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateState"),
            &raw mut __FUNCTION_PTRS.u_input_trigger_update_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActuated"),
            &raw mut __FUNCTION_PTRS.u_input_trigger_is_actuated,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTriggerType"),
            &raw mut __FUNCTION_PTRS.u_input_trigger_get_trigger_type,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlayerMappableInputConfig::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToDefault"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_reset_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsDeprecated"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_is_deprecated,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerMappableKeys"),
            &raw mut __FUNCTION_PTRS
                .u_player_mappable_input_config_get_player_mappable_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetadata"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_get_metadata,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMappingContexts"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_get_mapping_contexts,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMappingByName"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_get_mapping_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysBoundToAction"),
            &raw mut __FUNCTION_PTRS
                .u_player_mappable_input_config_get_keys_bound_to_action,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_get_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConfigName"),
            &raw mut __FUNCTION_PTRS.u_player_mappable_input_config_get_config_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlayerMappableKeySettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKnownMappingNames"),
            &raw mut __FUNCTION_PTRS
                .u_player_mappable_key_settings_get_known_mapping_names,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedPlayerMappableKeyProfile::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToString"),
            &raw mut __FUNCTION_PTRS.u_enhanced_player_mappable_key_profile_to_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayName"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_set_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToDefault"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_reset_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetMappingToDefault"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_reset_mapping_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("QueryPlayerMappedKeys"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_query_player_mapped_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_FindKeyMapping"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_k2_find_key_mapping,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetProfileIdString"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_get_profile_id_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetProfileDisplayName"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_get_profile_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayerMappingRows"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_get_player_mapping_rows,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMappingNamesForKey"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_get_mapping_names_for_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMappedKeysInRow"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_get_mapped_keys_in_row,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DumpProfileToLog"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_dump_profile_to_log,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesMappingPassQueryOptions"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_player_mappable_key_profile_does_mapping_pass_query_options,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEnhancedInputUserSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterInputMappingContexts"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_unregister_input_mapping_contexts,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterInputMappingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_unregister_input_mapping_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnMapPlayerKey"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_un_map_player_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActiveKeyProfile"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_set_active_key_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveSettings"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_save_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetKeyProfileToDefault"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_reset_key_profile_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetKeyProfileIdToDefault"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_reset_key_profile_id_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAllPlayerKeysInRow"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_reset_all_player_keys_in_row,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterInputMappingContexts"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_register_input_mapping_contexts,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterInputMappingContext"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_register_input_mapping_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MapPlayerKey"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_map_player_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "MappingContextRegisteredWithSettings__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_mapping_context_registered_with_settings_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MappableKeyProfileChanged__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_mappable_key_profile_changed_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMappingContextRegistered"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_is_mapping_context_registered,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeyProfileWithId"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_get_key_profile_with_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveKeyProfileId"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_get_active_key_profile_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveKeyProfile"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_get_active_key_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMappingsInRow"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_find_mappings_in_row,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "EnhancedInputUserSettingsChanged__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_enhanced_input_user_settings_changed_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "EnhancedInputUserSettingsApplied__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_enhanced_input_user_settings_applied_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewKeyProfile"),
            &raw mut __FUNCTION_PTRS
                .u_enhanced_input_user_settings_create_new_key_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AsyncSaveSettings"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_async_save_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplySettings"),
            &raw mut __FUNCTION_PTRS.u_enhanced_input_user_settings_apply_settings,
        );
    }
}
#[repr(C, align(8))]
pub struct FInputActionValue {
    __padding_end: [u8; 32],
}
impl FInputActionValue {}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyProfileCreationArgs {
    pub profile_type: TSubclassOf<UEnhancedPlayerMappableKeyProfile>,
    #[doc(hidden)]
    __padding_24: [u8; 16],
    pub profile_string_identifier: FString,
    pub user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub display_name: FText,
    pub flags_64: u8,
}
impl FPlayerMappableKeyProfileCreationArgs {}
#[repr(C, align(8))]
pub struct FPlayerKeyMapping {
    pub mapping_name: FName,
    pub display_name: FText,
    pub display_category: FText,
    pub slot: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub default_key: crate::bindings::input_core::FKey,
    pub current_key: crate::bindings::input_core::FKey,
    pub hardware_device_id: crate::bindings::engine::FHardwareDeviceIdentifier,
    pub associated_input_action: UPtr<UInputAction>,
    pub associated_input_action_soft: TSoftObjectPtr<UInputAction>,
}
impl FPlayerKeyMapping {}
#[repr(C, align(8))]
pub struct FMapPlayerKeyArgs {
    pub mapping_name: FName,
    pub slot: EPlayerMappableKeySlot,
    pub new_key: crate::bindings::input_core::FKey,
    pub hardware_device_id: FName,
    #[doc(hidden)]
    __padding_72: [u8; 8],
    pub profile_id_string: FString,
    pub flags_88: u8,
}
impl FMapPlayerKeyArgs {}
#[repr(C, align(8))]
pub struct FMappingQueryIssue {
    pub issue: EMappingQueryIssue,
    pub blocking_context: UPtr<UInputMappingContext>,
    pub blocking_action: UPtr<UInputAction>,
}
impl FMappingQueryIssue {}
#[repr(C, align(8))]
pub struct FEnhancedActionKeyMapping {
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub action: UPtr<UInputAction>,
    pub key: crate::bindings::input_core::FKey,
    __padding_end: [u8; 16],
}
impl FEnhancedActionKeyMapping {}
#[repr(C, align(1))]
pub struct FModifyContextOptions {
    pub flags_0: u8,
}
impl FModifyContextOptions {}
#[repr(C, align(8))]
pub struct FInputActionInstance {
    pub source_action: UPtr<UInputAction>,
    #[doc(hidden)]
    __padding_19: [u8; 11],
    pub trigger_event: ETriggerEvent,
    pub last_triggered_world_time: f32,
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    #[doc(hidden)]
    __padding_88: [u8; 32],
    pub elapsed_processed_time: f32,
    pub elapsed_triggered_time: f32,
}
impl FInputActionInstance {}
#[repr(C, align(8))]
pub struct FInputMappingContextMappingData {
    pub mappings: TArray<FEnhancedActionKeyMapping>,
}
impl FInputMappingContextMappingData {}
#[repr(C, align(8))]
pub struct FInputComboStepData {
    pub combo_step_action: UPtr<UInputAction>,
    pub combo_step_completion_states: u8,
    pub time_to_press_key: f32,
}
impl FInputComboStepData {}
#[repr(C, align(8))]
pub struct FInputCancelAction {
    pub cancel_action: UPtr<UInputAction>,
    pub cancellation_states: u8,
}
impl FInputCancelAction {}
#[repr(C, align(8))]
pub struct FKeyMappingRow {
    pub mappings: TSet<FPlayerKeyMapping>,
}
impl FKeyMappingRow {}
#[repr(C, align(8))]
pub struct FPlayerMappableKeyQueryOptions {
    pub mapping_name: FName,
    pub key_to_match: crate::bindings::input_core::FKey,
    pub slot_to_match: EPlayerMappableKeySlot,
    pub flags_49: u8,
    pub required_device_type: crate::bindings::engine::EHardwareDevicePrimaryType,
    pub required_device_flags: i32,
}
impl FPlayerMappableKeyQueryOptions {}
#[repr(C, align(8))]
pub struct UEnhancedInputActionDelegateBinding {
    __padding_end: [u8; 64],
}
impl UEnhancedInputActionDelegateBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputActionDelegateBinding")
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
pub struct UEnhancedInputActionValueBinding {
    __padding_end: [u8; 64],
}
impl UEnhancedInputActionValueBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputActionValueBinding")
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
pub struct UEnhancedInputComponent {
    __padding_end: [u8; 432],
}
impl UEnhancedInputComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputComponent")
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
    pub fn get_bound_action_value(
        &self,
        action: UPtr<UInputAction>,
    ) -> FInputActionValue {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_component_get_bound_action_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_component_get_bound_action_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FInputActionValue>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEnhancedInputDeveloperSettings {
    __padding_end: [u8; 424],
}
impl UEnhancedInputDeveloperSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputDeveloperSettings")
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
pub struct UEnhancedInputLibrary {
    __padding_end: [u8; 48],
}
impl UEnhancedInputLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputLibrary")
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
    pub fn request_rebuild_control_mappings_using_context(
        context: UPtr<UInputMappingContext>,
        b_force_immediately: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_request_rebuild_control_mappings_using_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_immediately,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_request_rebuild_control_mappings_using_context,
                __buffer,
            )
        };
    }
    pub fn make_input_action_value_of_type(
        x: f64,
        y: f64,
        z: f64,
        value_type: EInputActionValueType,
    ) -> FInputActionValue {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_make_input_action_value_of_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(0).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&z, __buffer.add(16).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value_type,
                __buffer.add(24).cast::<EInputActionValueType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_make_input_action_value_of_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FInputActionValue>().read() }
    }
    pub fn is_action_key_mapping_player_mappable(
        action_key_mapping: &FEnhancedActionKeyMapping,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_is_action_key_mapping_player_mappable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                action_key_mapping,
                __buffer.add(0).cast::<FEnhancedActionKeyMapping>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_is_action_key_mapping_player_mappable,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn get_player_mappable_key_settings(
        action_key_mapping: &FEnhancedActionKeyMapping,
    ) -> UPtr<UPlayerMappableKeySettings> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_player_mappable_key_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                action_key_mapping,
                __buffer.add(0).cast::<FEnhancedActionKeyMapping>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_player_mappable_key_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<UPtr<UPlayerMappableKeySettings>>().read() }
    }
    pub fn get_mapping_name(action_key_mapping: &FEnhancedActionKeyMapping) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<100>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_mapping_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                action_key_mapping,
                __buffer.add(0).cast::<FEnhancedActionKeyMapping>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_mapping_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FName>().read() }
    }
    pub fn get_bound_action_value(
        actor: UPtr<crate::bindings::engine::AActor>,
        action: UPtr<UInputAction>,
    ) -> FInputActionValue {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_bound_action_value,
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
                &action,
                __buffer.add(8).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_get_bound_action_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FInputActionValue>().read() }
    }
    pub fn flush_player_input(
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_flush_player_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_flush_player_input,
                __buffer,
            )
        };
    }
    pub fn conv_trigger_event_value_to_string(trigger_event: ETriggerEvent) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_trigger_event_value_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_event,
                __buffer.add(0).cast::<ETriggerEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_trigger_event_value_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn conv_input_action_value_to_string(
        action_value: FInputActionValue,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn conv_input_action_value_to_bool(in_value: FInputActionValue) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn conv_input_action_value_to_axis3_d(
        action_value: FInputActionValue,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis3_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis3_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn conv_input_action_value_to_axis2_d(
        in_value: FInputActionValue,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn conv_input_action_value_to_axis1_d(in_value: FInputActionValue) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis1_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_conv_input_action_value_to_axis1_d,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f64>().read() }
    }
    pub fn break_input_action_value(
        in_action_value: FInputActionValue,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
        ty: &mut EInputActionValueType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_break_input_action_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_action_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(x, __buffer.add(32).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(y, __buffer.add(40).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(z, __buffer.add(48).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ty,
                __buffer.add(56).cast::<EInputActionValueType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::enhanced_input::UEnhancedInputLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_library_break_input_action_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<f64>().swap(x);
        }
        unsafe {
            __buffer.add(40).cast::<f64>().swap(y);
        }
        unsafe {
            __buffer.add(48).cast::<f64>().swap(z);
        }
        unsafe {
            __buffer.add(56).cast::<EInputActionValueType>().swap(ty);
        }
    }
}
#[repr(C, align(8))]
pub struct UEnhancedInputPlatformData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub mapping_context_redirects: TMap<
        UPtr<UInputMappingContext>,
        UPtr<UInputMappingContext>,
    >,
}
impl UEnhancedInputPlatformData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputPlatformData")
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
    pub fn get_context_redirect(
        &self,
        in_context: UPtr<UInputMappingContext>,
    ) -> UPtr<UInputMappingContext> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_platform_data_get_context_redirect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_platform_data_get_context_redirect,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInputMappingContext>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEnhancedInputPlatformSettings {
    __padding_end: [u8; 120],
}
impl UEnhancedInputPlatformSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputPlatformSettings")
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
pub struct IEnhancedInputSubsystemInterface {}
#[repr(C, align(8))]
pub struct UEnhancedInputSubsystemInterface {
    __padding_end: [u8; 48],
}
impl UEnhancedInputSubsystemInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputSubsystemInterface")
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
    pub fn update_value_of_continuous_input_injection_for_player_mapping(
        &mut self,
        mapping_name: FName,
        raw_value: FInputActionValue,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(16).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
    }
    pub fn update_value_of_continuous_input_injection_for_action(
        &mut self,
        action: UPtr<UInputAction>,
        raw_value: FInputActionValue,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(8).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_update_value_of_continuous_input_injection_for_action,
                __buffer,
            )
        };
    }
    pub fn stop_continuous_input_injection_for_player_mapping(
        &mut self,
        mapping_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
    }
    pub fn stop_continuous_input_injection_for_action(
        &mut self,
        action: UPtr<UInputAction>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_stop_continuous_input_injection_for_action,
                __buffer,
            )
        };
    }
    pub fn start_continuous_input_injection_for_player_mapping(
        &mut self,
        mapping_name: FName,
        raw_value: FInputActionValue,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(16).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(48).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(64).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_player_mapping,
                __buffer,
            )
        };
    }
    pub fn start_continuous_input_injection_for_action(
        &mut self,
        action: UPtr<UInputAction>,
        raw_value: FInputActionValue,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(8).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(40).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(56).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_start_continuous_input_injection_for_action,
                __buffer,
            )
        };
    }
    pub fn set_input_mode(
        &mut self,
        new_mode: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_set_input_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_mode,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_set_input_mode,
                __buffer,
            )
        };
    }
    pub fn request_rebuild_control_mappings(
        &mut self,
        options: &FModifyContextOptions,
        rebuild_type: EInputMappingRebuildType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_request_rebuild_control_mappings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(0).cast::<FModifyContextOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rebuild_type,
                __buffer.add(1).cast::<EInputMappingRebuildType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_request_rebuild_control_mappings,
                __buffer,
            )
        };
    }
    pub fn remove_tags_from_input_mode(
        &mut self,
        tags_to_remove: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_tags_from_input_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags_to_remove,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_tags_from_input_mode,
                __buffer,
            )
        };
    }
    pub fn remove_tag_from_input_mode(
        &mut self,
        tag_to_remove: &crate::bindings::gameplay_tags::FGameplayTag,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_tag_from_input_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_to_remove,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(12).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_tag_from_input_mode,
                __buffer,
            )
        };
    }
    pub fn remove_mapping_context(
        &mut self,
        mapping_context: UPtr<UInputMappingContext>,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_mapping_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(8).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_remove_mapping_context,
                __buffer,
            )
        };
    }
    pub fn query_map_key_in_context_set(
        &mut self,
        prioritized_active_contexts: &TArray<UPtr<UInputMappingContext>>,
        input_context: UPtr<UInputMappingContext>,
        action: UPtr<UInputAction>,
        key: crate::bindings::input_core::FKey,
        out_issues: &mut TArray<FMappingQueryIssue>,
        blocking_issues: EMappingQueryIssue,
    ) -> EMappingQueryResult {
        let mut __stack = crate::core_data::StackAlloc::<82>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_map_key_in_context_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                prioritized_active_contexts,
                __buffer.add(0).cast::<TArray<UPtr<UInputMappingContext>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_context,
                __buffer.add(16).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(24).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(32).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_issues,
                __buffer.add(64).cast::<TArray<FMappingQueryIssue>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blocking_issues,
                __buffer.add(80).cast::<EMappingQueryIssue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_map_key_in_context_set,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<TArray<FMappingQueryIssue>>().swap(out_issues);
        }
        unsafe { __buffer.add(81).cast::<EMappingQueryResult>().read() }
    }
    pub fn query_map_key_in_active_context_set(
        &mut self,
        input_context: UPtr<UInputMappingContext>,
        action: UPtr<UInputAction>,
        key: crate::bindings::input_core::FKey,
        out_issues: &mut TArray<FMappingQueryIssue>,
        blocking_issues: EMappingQueryIssue,
    ) -> EMappingQueryResult {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_map_key_in_active_context_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(8).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(16).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_issues,
                __buffer.add(48).cast::<TArray<FMappingQueryIssue>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blocking_issues,
                __buffer.add(64).cast::<EMappingQueryIssue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_map_key_in_active_context_set,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<TArray<FMappingQueryIssue>>().swap(out_issues);
        }
        unsafe { __buffer.add(65).cast::<EMappingQueryResult>().read() }
    }
    pub fn query_keys_mapped_to_action(
        &self,
        action: UPtr<UInputAction>,
    ) -> TArray<crate::bindings::input_core::FKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_keys_mapped_to_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_query_keys_mapped_to_action,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<crate::bindings::input_core::FKey>>().read()
        }
    }
    pub fn inject_input_vector_for_player_mapping(
        &mut self,
        mapping_name: FName,
        value: crate::bindings::core_u_object::FVector,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_vector_for_player_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(40).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(56).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_vector_for_player_mapping,
                __buffer,
            )
        };
    }
    pub fn inject_input_vector_for_action(
        &mut self,
        action: UPtr<UInputAction>,
        value: crate::bindings::core_u_object::FVector,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_vector_for_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(32).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(48).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_vector_for_action,
                __buffer,
            )
        };
    }
    pub fn inject_input_for_player_mapping(
        &mut self,
        mapping_name: FName,
        raw_value: FInputActionValue,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_for_player_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(16).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(48).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(64).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_for_player_mapping,
                __buffer,
            )
        };
    }
    pub fn inject_input_for_action(
        &mut self,
        action: UPtr<UInputAction>,
        raw_value: FInputActionValue,
        modifiers: &TArray<UPtr<UInputModifier>>,
        triggers: &TArray<UPtr<UInputTrigger>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_for_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_value,
                __buffer.add(8).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifiers,
                __buffer.add(40).cast::<TArray<UPtr<UInputModifier>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                triggers,
                __buffer.add(56).cast::<TArray<UPtr<UInputTrigger>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_inject_input_for_action,
                __buffer,
            )
        };
    }
    pub fn has_mapping_context(
        &self,
        mapping_context: UPtr<UInputMappingContext>,
        out_found_priority: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_has_mapping_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_found_priority,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_has_mapping_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(out_found_priority);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_user_settings(&self) -> UPtr<UEnhancedInputUserSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_user_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_user_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UEnhancedInputUserSettings>>().read() }
    }
    pub fn get_input_mode(
        &self,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_input_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_input_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_all_player_mappable_action_key_mappings(
        &self,
    ) -> TArray<FEnhancedActionKeyMapping> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_all_player_mappable_action_key_mappings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_get_all_player_mappable_action_key_mappings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FEnhancedActionKeyMapping>>().read() }
    }
    pub fn clear_all_mappings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_clear_all_mappings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_clear_all_mappings,
                __buffer,
            )
        };
    }
    pub fn append_tags_to_input_mode(
        &mut self,
        tags_to_add: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_append_tags_to_input_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags_to_add,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_append_tags_to_input_mode,
                __buffer,
            )
        };
    }
    pub fn add_tag_to_input_mode(
        &mut self,
        tag_to_add: &crate::bindings::gameplay_tags::FGameplayTag,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_add_tag_to_input_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_to_add,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(12).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_add_tag_to_input_mode,
                __buffer,
            )
        };
    }
    pub fn add_mapping_context(
        &mut self,
        mapping_context: UPtr<UInputMappingContext>,
        priority: i32,
        options: &FModifyContextOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_add_mapping_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_context,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&priority, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(12).cast::<FModifyContextOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_subsystem_interface_add_mapping_context,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEnhancedInputLocalPlayerSubsystem {
    __padding_end: [u8; 576],
}
impl UEnhancedInputLocalPlayerSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputLocalPlayerSubsystem")
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
pub struct UEnhancedInputWorldSubsystem {
    __padding_end: [u8; 520],
}
impl UEnhancedInputWorldSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputWorldSubsystem")
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
    pub fn remove_actor_input_component(
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_world_subsystem_remove_actor_input_component,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_world_subsystem_remove_actor_input_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn add_actor_input_component(
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_world_subsystem_add_actor_input_component,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_world_subsystem_add_actor_input_component,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEnhancedPlayerInput {
    __padding_end: [u8; 2312],
}
impl UEnhancedPlayerInput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedPlayerInput")
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
pub struct UInputAction {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub action_description: FText,
    pub b_trigger_when_paused: bool,
    pub b_consume_input: bool,
    pub b_consumes_action_and_axis_mappings: bool,
    pub b_reserve_all_mappings: bool,
    pub trigger_events_that_consume_legacy_keys: i32,
    pub value_type: EInputActionValueType,
    pub accumulation_behavior: EInputActionAccumulationBehavior,
    pub triggers: TArray<UPtr<UInputTrigger>>,
    pub modifiers: TArray<UPtr<UInputModifier>>,
    pub player_mappable_key_settings: UPtr<UPlayerMappableKeySettings>,
}
impl UInputAction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAction")
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
pub struct UInputDebugKeyDelegateBinding {
    __padding_end: [u8; 64],
}
impl UInputDebugKeyDelegateBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputDebugKeyDelegateBinding")
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
pub struct UInputMappingContext {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub mappings: TArray<FEnhancedActionKeyMapping>,
    pub default_key_mappings: FInputMappingContextMappingData,
    pub mapping_profile_overrides: TMap<FString, FInputMappingContextMappingData>,
    pub input_mode_filter_options: EMappingContextInputModeFilterOptions,
    pub input_mode_query_override: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub registration_tracking_mode: EMappingContextRegistrationTrackingMode,
    pub context_description: FText,
}
impl UInputMappingContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputMappingContext")
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
    pub fn unmap_key(
        &mut self,
        action: UPtr<UInputAction>,
        key: crate::bindings::input_core::FKey,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(8).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_key,
                __buffer,
            )
        };
    }
    pub fn unmap_all_keys_from_action(&mut self, action: UPtr<UInputAction>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_all_keys_from_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_all_keys_from_action,
                __buffer,
            )
        };
    }
    pub fn unmap_all(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_all,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_unmap_all,
                __buffer,
            )
        };
    }
    pub fn map_key(
        &mut self,
        action: UPtr<UInputAction>,
        to_key: crate::bindings::input_core::FKey,
    ) -> FEnhancedActionKeyMapping {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_map_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_key,
                __buffer.add(8).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_mapping_context_map_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FEnhancedActionKeyMapping>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInputModifier {
    __padding_end: [u8; 48],
}
impl UInputModifier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifier")
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
    pub fn modify_raw(
        &self,
        player_input: UPtr<UEnhancedPlayerInput>,
        current_value: FInputActionValue,
        delta_time: f32,
    ) -> FInputActionValue {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_modifier_modify_raw,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_input,
                __buffer.add(0).cast::<UPtr<UEnhancedPlayerInput>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &current_value,
                __buffer.add(8).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_modifier_modify_raw,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FInputActionValue>().read() }
    }
    pub fn get_visualization_color(
        &self,
        sample_value: FInputActionValue,
        final_value: FInputActionValue,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_modifier_get_visualization_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sample_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &final_value,
                __buffer.add(32).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_modifier_get_visualization_color,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UInputModifierSmoothDelta {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub smoothing_method: ENormalizeInputSmoothingType,
    pub speed: f32,
    pub easing_exponent: f32,
    __padding_end: [u8; 52],
}
impl UInputModifierSmoothDelta {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierSmoothDelta")
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
pub struct UInputModifierDeadZone {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub lower_threshold: f32,
    pub upper_threshold: f32,
    pub ty: EDeadZoneType,
}
impl UInputModifierDeadZone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierDeadZone")
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
pub struct UInputModifierScalar {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub scalar: crate::bindings::core_u_object::FVector,
}
impl UInputModifierScalar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierScalar")
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
pub struct UInputModifierScaleByDeltaTime {
    __padding_end: [u8; 48],
}
impl UInputModifierScaleByDeltaTime {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierScaleByDeltaTime")
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
pub struct UInputModifierNegate {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_x: bool,
    pub b_y: bool,
    pub b_z: bool,
}
impl UInputModifierNegate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierNegate")
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
pub struct UInputModifierSmooth {
    __padding_end: [u8; 96],
}
impl UInputModifierSmooth {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierSmooth")
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
pub struct UInputModifierResponseCurveExponential {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub curve_exponent: crate::bindings::core_u_object::FVector,
}
impl UInputModifierResponseCurveExponential {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierResponseCurveExponential")
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
pub struct UInputModifierResponseCurveUser {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub response_x: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_y: UPtr<crate::bindings::engine::UCurveFloat>,
    pub response_z: UPtr<crate::bindings::engine::UCurveFloat>,
}
impl UInputModifierResponseCurveUser {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierResponseCurveUser")
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
pub struct UInputModifierFOVScaling {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub fov_scale: f32,
    pub fov_scaling_type: EFOVScalingType,
}
impl UInputModifierFOVScaling {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierFOVScaling")
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
pub struct UInputModifierToWorldSpace {
    __padding_end: [u8; 48],
}
impl UInputModifierToWorldSpace {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierToWorldSpace")
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
pub struct UInputModifierSwizzleAxis {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub order: EInputAxisSwizzle,
}
impl UInputModifierSwizzleAxis {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputModifierSwizzleAxis")
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
pub struct UInputTrigger {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub actuation_threshold: f32,
    pub b_should_always_tick: bool,
    pub last_value: FInputActionValue,
}
impl UInputTrigger {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTrigger")
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
    pub fn update_state(
        &mut self,
        player_input: UPtr<UEnhancedPlayerInput>,
        modified_value: FInputActionValue,
        delta_time: f32,
    ) -> ETriggerState {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_update_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_input,
                __buffer.add(0).cast::<UPtr<UEnhancedPlayerInput>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modified_value,
                __buffer.add(8).cast::<FInputActionValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_time,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_update_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<ETriggerState>().read() }
    }
    pub fn is_actuated(&self, for_value: &FInputActionValue) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_is_actuated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                for_value,
                __buffer.add(0).cast::<FInputActionValue>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_is_actuated,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_trigger_type(&self) -> ETriggerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_get_trigger_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_input_trigger_get_trigger_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ETriggerType>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInputTriggerTimedBase {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub held_duration: f32,
    pub b_affected_by_time_dilation: bool,
}
impl UInputTriggerTimedBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerTimedBase")
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
pub struct UInputTriggerDown {
    __padding_end: [u8; 88],
}
impl UInputTriggerDown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerDown")
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
pub struct UInputTriggerPressed {
    __padding_end: [u8; 88],
}
impl UInputTriggerPressed {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerPressed")
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
pub struct UInputTriggerReleased {
    __padding_end: [u8; 88],
}
impl UInputTriggerReleased {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerReleased")
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
pub struct UInputTriggerHold {
    #[doc(hidden)]
    __padding_100: [u8; 100],
    pub hold_time_threshold: f32,
    pub b_is_one_shot: bool,
}
impl UInputTriggerHold {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerHold")
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
pub struct UInputTriggerHoldAndRelease {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub hold_time_threshold: f32,
}
impl UInputTriggerHoldAndRelease {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerHoldAndRelease")
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
pub struct UInputTriggerTap {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub tap_release_time_threshold: f32,
}
impl UInputTriggerTap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerTap")
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
pub struct UInputTriggerRepeatedTap {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub repeat_delay: f64,
    #[doc(hidden)]
    __padding_112: [u8; 8],
    pub number_of_taps_which_trigger_repeat: i32,
    pub tap_release_time_threshold: f32,
    __padding_end: [u8; 8],
}
impl UInputTriggerRepeatedTap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerRepeatedTap")
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
pub struct UInputTriggerPulse {
    #[doc(hidden)]
    __padding_100: [u8; 100],
    pub b_trigger_on_start: bool,
    pub interval: f32,
    pub trigger_limit: i32,
}
impl UInputTriggerPulse {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerPulse")
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
pub struct UInputTriggerChordAction {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub chord_action: UPtr<UInputAction>,
}
impl UInputTriggerChordAction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerChordAction")
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
pub struct UInputTriggerChordBlocker {
    __padding_end: [u8; 96],
}
impl UInputTriggerChordBlocker {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerChordBlocker")
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
pub struct UInputTriggerCombo {
    #[doc(hidden)]
    __padding_88: [u8; 88],
    pub current_combo_step_index: i32,
    pub current_time_between_combo_steps: f32,
    pub combo_actions: TArray<FInputComboStepData>,
    pub input_cancel_actions: TArray<FInputCancelAction>,
}
impl UInputTriggerCombo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputTriggerCombo")
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
pub struct UPlayerMappableInputConfig {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub config_name: FName,
    pub config_display_name: FText,
    pub b_is_deprecated: bool,
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub contexts: TMap<UPtr<UInputMappingContext>, i32>,
}
impl UPlayerMappableInputConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayerMappableInputConfig")
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
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_reset_to_default,
                __buffer,
            )
        };
    }
    pub fn is_deprecated(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_is_deprecated,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_is_deprecated,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_player_mappable_keys(&self) -> TArray<FEnhancedActionKeyMapping> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_player_mappable_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_player_mappable_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FEnhancedActionKeyMapping>>().read() }
    }
    pub fn get_metadata(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_metadata,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_metadata,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_mapping_contexts(&self) -> TMap<UPtr<UInputMappingContext>, i32> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_mapping_contexts,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_mapping_contexts,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<UPtr<UInputMappingContext>, i32>>().read() }
    }
    pub fn get_mapping_by_name(&self, mapping_name: FName) -> FEnhancedActionKeyMapping {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_mapping_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_mapping_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FEnhancedActionKeyMapping>().read() }
    }
    pub fn get_keys_bound_to_action(
        &self,
        in_action: UPtr<UInputAction>,
    ) -> TArray<FEnhancedActionKeyMapping> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_keys_bound_to_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_action,
                __buffer.add(0).cast::<UPtr<UInputAction>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_keys_bound_to_action,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FEnhancedActionKeyMapping>>().read() }
    }
    pub fn get_display_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_display_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_config_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_config_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_player_mappable_input_config_get_config_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPlayerMappableKeySettings {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub metadata: UPtr<crate::bindings::core_u_object::UObject>,
    pub name: FName,
    pub display_name: FText,
    pub display_category: FText,
    #[doc(hidden)]
    __padding_136: [u8; 32],
    pub supported_key_profile_ids: TArray<FString>,
}
impl UPlayerMappableKeySettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayerMappableKeySettings")
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
pub struct UEnhancedPlayerMappableKeyProfile {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub profile_identifier_string: FString,
    pub owning_user_id: crate::bindings::core_u_object::FPlatformUserId,
    pub display_name: FText,
    pub player_mapped_keys: TMap<FName, FKeyMappingRow>,
}
impl UEnhancedPlayerMappableKeyProfile {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedPlayerMappableKeyProfile")
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
    pub fn to_string(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_to_string,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn set_display_name(&mut self, new_display_name: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_set_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_display_name,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_set_display_name,
                __buffer,
            )
        };
    }
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_reset_to_default,
                __buffer,
            )
        };
    }
    pub fn reset_mapping_to_default(&mut self, in_mapping_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_reset_mapping_to_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mapping_name,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_reset_mapping_to_default,
                __buffer,
            )
        };
    }
    pub fn query_player_mapped_keys(
        &self,
        options: &FPlayerMappableKeyQueryOptions,
        out_keys: &mut TArray<crate::bindings::input_core::FKey>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_query_player_mapped_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(0).cast::<FPlayerMappableKeyQueryOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_keys,
                __buffer.add(56).cast::<TArray<crate::bindings::input_core::FKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_query_player_mapped_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::input_core::FKey>>()
                .swap(out_keys);
        }
        unsafe { __buffer.add(72).cast::<i32>().read() }
    }
    pub fn k2_find_key_mapping(
        &self,
        out_key_mapping: &mut FPlayerKeyMapping,
        in_args: &FMapPlayerKeyArgs,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_k2_find_key_mapping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_key_mapping,
                __buffer.add(0).cast::<FPlayerKeyMapping>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_args,
                __buffer.add(208).cast::<FMapPlayerKeyArgs>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_k2_find_key_mapping,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPlayerKeyMapping>().swap(out_key_mapping);
        }
    }
    pub fn get_profile_id_string(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_profile_id_string,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_profile_id_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_profile_display_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_profile_display_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_profile_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_player_mapping_rows(&self) -> TMap<FName, FKeyMappingRow> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_player_mapping_rows,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_player_mapping_rows,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<FName, FKeyMappingRow>>().read() }
    }
    pub fn get_mapping_names_for_key(
        &self,
        in_key: &crate::bindings::input_core::FKey,
        out_mapping_names: &mut TArray<FName>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_mapping_names_for_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<crate::bindings::input_core::FKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_mapping_names,
                __buffer.add(32).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_mapping_names_for_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<TArray<FName>>().swap(out_mapping_names);
        }
        unsafe { __buffer.add(48).cast::<i32>().read() }
    }
    pub fn get_mapped_keys_in_row(
        &self,
        mapping_name: FName,
        out_keys: &mut TArray<crate::bindings::input_core::FKey>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_mapped_keys_in_row,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_keys,
                __buffer.add(16).cast::<TArray<crate::bindings::input_core::FKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_get_mapped_keys_in_row,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::input_core::FKey>>()
                .swap(out_keys);
        }
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn dump_profile_to_log(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_dump_profile_to_log,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_dump_profile_to_log,
                __buffer,
            )
        };
    }
    pub fn does_mapping_pass_query_options(
        &self,
        player_mapping: &FPlayerKeyMapping,
        options: &FPlayerMappableKeyQueryOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<265>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_does_mapping_pass_query_options,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                player_mapping,
                __buffer.add(0).cast::<FPlayerKeyMapping>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(208).cast::<FPlayerMappableKeyQueryOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_player_mappable_key_profile_does_mapping_pass_query_options,
                __buffer,
            )
        };
        unsafe { __buffer.add(264).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEnhancedInputUserSettings {
    __padding_end: [u8; 432],
}
impl UEnhancedInputUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnhancedInputUserSettings")
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
    pub fn unregister_input_mapping_contexts(
        &mut self,
        mapping_contexts: &TSet<UPtr<UInputMappingContext>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_unregister_input_mapping_contexts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mapping_contexts,
                __buffer.add(0).cast::<TSet<UPtr<UInputMappingContext>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_unregister_input_mapping_contexts,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn unregister_input_mapping_context(
        &mut self,
        imc: UPtr<UInputMappingContext>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_unregister_input_mapping_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &imc,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_unregister_input_mapping_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn un_map_player_key(
        &mut self,
        in_args: &FMapPlayerKeyArgs,
        failure_reason: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_un_map_player_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_args,
                __buffer.add(0).cast::<FMapPlayerKeyArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                failure_reason,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_un_map_player_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(failure_reason);
        }
    }
    pub fn set_active_key_profile(&mut self, in_profile_id: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_set_active_key_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_profile_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_set_active_key_profile,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn save_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_save_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_save_settings,
                __buffer,
            )
        };
    }
    pub fn reset_key_profile_to_default(
        &mut self,
        profile_id: &crate::bindings::gameplay_tags::FGameplayTag,
        failure_reason: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_key_profile_to_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                profile_id,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                failure_reason,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_key_profile_to_default,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(failure_reason);
        }
    }
    pub fn reset_key_profile_id_to_default(
        &mut self,
        profile_id: FString,
        failure_reason: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_key_profile_id_to_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &profile_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                failure_reason,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_key_profile_id_to_default,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(failure_reason);
        }
    }
    pub fn reset_all_player_keys_in_row(
        &mut self,
        in_args: &FMapPlayerKeyArgs,
        failure_reason: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_all_player_keys_in_row,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_args,
                __buffer.add(0).cast::<FMapPlayerKeyArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                failure_reason,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_reset_all_player_keys_in_row,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(failure_reason);
        }
    }
    pub fn register_input_mapping_contexts(
        &mut self,
        mapping_contexts: &TSet<UPtr<UInputMappingContext>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_register_input_mapping_contexts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mapping_contexts,
                __buffer.add(0).cast::<TSet<UPtr<UInputMappingContext>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_register_input_mapping_contexts,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn register_input_mapping_context(
        &mut self,
        imc: UPtr<UInputMappingContext>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_register_input_mapping_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &imc,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_register_input_mapping_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn map_player_key(
        &mut self,
        in_args: &FMapPlayerKeyArgs,
        failure_reason: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_map_player_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_args,
                __buffer.add(0).cast::<FMapPlayerKeyArgs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                failure_reason,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_map_player_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(failure_reason);
        }
    }
    pub fn is_mapping_context_registered(
        &self,
        imc: UPtr<UInputMappingContext>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_is_mapping_context_registered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &imc,
                __buffer.add(0).cast::<UPtr<UInputMappingContext>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_is_mapping_context_registered,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_key_profile_with_id(
        &self,
        profile_id: FString,
    ) -> UPtr<UEnhancedPlayerMappableKeyProfile> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_key_profile_with_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &profile_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_key_profile_with_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UEnhancedPlayerMappableKeyProfile>>().read()
        }
    }
    pub fn get_active_key_profile_id(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_active_key_profile_id,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_active_key_profile_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_active_key_profile(&self) -> UPtr<UEnhancedPlayerMappableKeyProfile> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_active_key_profile,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_get_active_key_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<UEnhancedPlayerMappableKeyProfile>>().read()
        }
    }
    pub fn find_mappings_in_row(&self, mapping_name: FName) -> TSet<FPlayerKeyMapping> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_find_mappings_in_row,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mapping_name,
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
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_find_mappings_in_row,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TSet<FPlayerKeyMapping>>().read() }
    }
    pub fn create_new_key_profile(
        &mut self,
        in_args: &FPlayerMappableKeyProfileCreationArgs,
    ) -> UPtr<UEnhancedPlayerMappableKeyProfile> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_create_new_key_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_args,
                __buffer.add(0).cast::<FPlayerMappableKeyProfileCreationArgs>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_create_new_key_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<UPtr<UEnhancedPlayerMappableKeyProfile>>().read()
        }
    }
    pub fn async_save_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_async_save_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_async_save_settings,
                __buffer,
            )
        };
    }
    pub fn apply_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_apply_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::enhanced_input::__FUNCTION_PTRS
                    .u_enhanced_input_user_settings_apply_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FEnhancedInputLocalPlayerSubsystem_ControlMappingsRebuiltDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextAdded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputLocalPlayerSubsystem_OnMappingContextRemoved {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputLocalPlayerSubsystem_OnPostUserSettingsInitialized {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputUserSettings_OnSettingsChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputUserSettings_OnSettingsApplied {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnhancedInputUserSettings_OnKeyProfileChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EPlayerMappableKeySlot(pub u8);
impl EPlayerMappableKeySlot {
    pub const FIRST: EPlayerMappableKeySlot = EPlayerMappableKeySlot(0);
    pub const SECOND: EPlayerMappableKeySlot = EPlayerMappableKeySlot(1);
    pub const THIRD: EPlayerMappableKeySlot = EPlayerMappableKeySlot(2);
    pub const FOURTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(3);
    pub const FIFTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(4);
    pub const SIXTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(5);
    pub const SEVENTH: EPlayerMappableKeySlot = EPlayerMappableKeySlot(6);
    pub const UNSPECIFIED: EPlayerMappableKeySlot = EPlayerMappableKeySlot(7);
    pub const MAX: EPlayerMappableKeySlot = EPlayerMappableKeySlot(8);
}
#[repr(transparent)]
pub struct EMappingQueryIssue(pub u8);
impl EMappingQueryIssue {
    pub const NO_ISSUE: EMappingQueryIssue = EMappingQueryIssue(0);
    pub const RESERVED_BY_ACTION: EMappingQueryIssue = EMappingQueryIssue(1);
    pub const HIDES_EXISTING_MAPPING: EMappingQueryIssue = EMappingQueryIssue(2);
    pub const HIDDEN_BY_EXISTING_MAPPING: EMappingQueryIssue = EMappingQueryIssue(4);
    pub const COLLISION_WITH_MAPPING_IN_SAME_CONTEXT: EMappingQueryIssue = EMappingQueryIssue(
        8,
    );
    pub const FORCES_TYPE_PROMOTION: EMappingQueryIssue = EMappingQueryIssue(16);
    pub const FORCES_TYPE_DEMOTION: EMappingQueryIssue = EMappingQueryIssue(32);
}
#[repr(transparent)]
pub struct EPlayerMappableKeySettingBehaviors(pub u8);
impl EPlayerMappableKeySettingBehaviors {
    pub const INHERIT_SETTINGS_FROM_ACTION: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        0,
    );
    pub const OVERRIDE_SETTINGS: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        1,
    );
    pub const IGNORE_SETTINGS: EPlayerMappableKeySettingBehaviors = EPlayerMappableKeySettingBehaviors(
        2,
    );
}
#[repr(transparent)]
pub struct ETriggerEvent(pub u8);
impl ETriggerEvent {
    pub const NONE: ETriggerEvent = ETriggerEvent(0);
    pub const TRIGGERED: ETriggerEvent = ETriggerEvent(1);
    pub const STARTED: ETriggerEvent = ETriggerEvent(2);
    pub const ONGOING: ETriggerEvent = ETriggerEvent(4);
    pub const CANCELED: ETriggerEvent = ETriggerEvent(8);
    pub const COMPLETED: ETriggerEvent = ETriggerEvent(16);
}
#[repr(transparent)]
pub struct EInputActionValueType(pub u8);
impl EInputActionValueType {
    pub const BOOLEAN: EInputActionValueType = EInputActionValueType(0);
    pub const AXIS1_D: EInputActionValueType = EInputActionValueType(1);
    pub const AXIS2_D: EInputActionValueType = EInputActionValueType(2);
    pub const AXIS3_D: EInputActionValueType = EInputActionValueType(3);
}
#[repr(transparent)]
pub struct EMappingQueryResult(pub u8);
impl EMappingQueryResult {
    pub const ERROR_ENHANCED_INPUT_NOT_ENABLED: EMappingQueryResult = EMappingQueryResult(
        0,
    );
    pub const ERROR_INPUT_CONTEXT_NOT_IN_ACTIVE_CONTEXTS: EMappingQueryResult = EMappingQueryResult(
        1,
    );
    pub const ERROR_INVALID_ACTION: EMappingQueryResult = EMappingQueryResult(2);
    pub const NOT_MAPPABLE: EMappingQueryResult = EMappingQueryResult(3);
    pub const MAPPING_AVAILABLE: EMappingQueryResult = EMappingQueryResult(4);
}
#[repr(transparent)]
pub struct EInputMappingRebuildType(pub u8);
impl EInputMappingRebuildType {
    pub const NONE: EInputMappingRebuildType = EInputMappingRebuildType(0);
    pub const REBUILD: EInputMappingRebuildType = EInputMappingRebuildType(1);
    pub const REBUILD_WITH_FLUSH: EInputMappingRebuildType = EInputMappingRebuildType(2);
}
#[repr(transparent)]
pub struct ETriggerType(pub u8);
impl ETriggerType {
    pub const EXPLICIT: ETriggerType = ETriggerType(0);
    pub const IMPLICIT: ETriggerType = ETriggerType(1);
    pub const BLOCKER: ETriggerType = ETriggerType(2);
}
#[repr(transparent)]
pub struct ETriggerState(pub u8);
impl ETriggerState {
    pub const NONE: ETriggerState = ETriggerState(0);
    pub const ONGOING: ETriggerState = ETriggerState(1);
    pub const TRIGGERED: ETriggerState = ETriggerState(2);
}
#[repr(transparent)]
pub struct EInputActionAccumulationBehavior(pub u8);
impl EInputActionAccumulationBehavior {
    pub const TAKE_HIGHEST_ABSOLUTE_VALUE: EInputActionAccumulationBehavior = EInputActionAccumulationBehavior(
        0,
    );
    pub const CUMULATIVE: EInputActionAccumulationBehavior = EInputActionAccumulationBehavior(
        1,
    );
}
#[repr(transparent)]
pub struct EMappingContextInputModeFilterOptions(pub u8);
impl EMappingContextInputModeFilterOptions {
    pub const USE_PROJECT_DEFAULT_QUERY: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        0,
    );
    pub const USE_CUSTOM_QUERY: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        1,
    );
    pub const DO_NOT_FILTER: EMappingContextInputModeFilterOptions = EMappingContextInputModeFilterOptions(
        2,
    );
}
#[repr(transparent)]
pub struct EMappingContextRegistrationTrackingMode(pub u8);
impl EMappingContextRegistrationTrackingMode {
    pub const UNTRACKED: EMappingContextRegistrationTrackingMode = EMappingContextRegistrationTrackingMode(
        0,
    );
    pub const COUNT_REGISTRATIONS: EMappingContextRegistrationTrackingMode = EMappingContextRegistrationTrackingMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENormalizeInputSmoothingType(pub u8);
impl ENormalizeInputSmoothingType {
    pub const NONE: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(0);
    pub const LERP: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(1);
    pub const INTERP_TO: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(2);
    pub const INTERP_CONSTANT_TO: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        3,
    );
    pub const INTERP_CIRCULAR_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        4,
    );
    pub const INTERP_CIRCULAR_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        5,
    );
    pub const INTERP_CIRCULAR_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        6,
    );
    pub const INTERP_EASE_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        7,
    );
    pub const INTERP_EASE_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        8,
    );
    pub const INTERP_EASE_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        9,
    );
    pub const INTERP_EXPO_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        10,
    );
    pub const INTERP_EXPO_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        11,
    );
    pub const INTERP_EXPO_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        12,
    );
    pub const INTERP_SIN_IN: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        13,
    );
    pub const INTERP_SIN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        14,
    );
    pub const INTERP_SIN_IN_OUT: ENormalizeInputSmoothingType = ENormalizeInputSmoothingType(
        15,
    );
}
#[repr(transparent)]
pub struct EDeadZoneType(pub u8);
impl EDeadZoneType {
    pub const AXIAL: EDeadZoneType = EDeadZoneType(0);
    pub const RADIAL: EDeadZoneType = EDeadZoneType(1);
    pub const UNSCALED_RADIAL: EDeadZoneType = EDeadZoneType(2);
}
#[repr(transparent)]
pub struct EFOVScalingType(pub u8);
impl EFOVScalingType {
    pub const STANDARD: EFOVScalingType = EFOVScalingType(0);
    pub const UE4_BACK_COMPAT: EFOVScalingType = EFOVScalingType(1);
}
#[repr(transparent)]
pub struct EInputAxisSwizzle(pub u8);
impl EInputAxisSwizzle {
    pub const YXZ: EInputAxisSwizzle = EInputAxisSwizzle(0);
    pub const ZYX: EInputAxisSwizzle = EInputAxisSwizzle(1);
    pub const XZY: EInputAxisSwizzle = EInputAxisSwizzle(2);
    pub const YZX: EInputAxisSwizzle = EInputAxisSwizzle(3);
    pub const ZXY: EInputAxisSwizzle = EInputAxisSwizzle(4);
}
