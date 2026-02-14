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
    pub u_smart_object_blueprint_function_library_smart_object_claim_handle_invalid: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_set_value_as_so_claim_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_set_smart_object_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_set_multiple_smart_objects_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_set_blackboard_value_as_so_claim_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_remove_smart_object: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_remove_multiple_smart_objects: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_not_equal_smart_object_slot_handle_smart_object_slot_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_not_equal_smart_object_handle_smart_object_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_mark_smart_object_slot_as_occupied: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_mark_smart_object_slot_as_free: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_mark_smart_object_slot_as_claimed: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_is_valid_smart_object_slot_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_is_valid_smart_object_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_is_valid_smart_object_claim_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_get_value_as_so_claim_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_get_blackboard_value_as_so_claim_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_find_smart_objects_in_targeting_request: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_find_smart_objects_in_list: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_find_smart_objects_in_component: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_find_smart_objects_in_actor: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_equal_smart_object_slot_handle_smart_object_slot_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_equal_smart_object_handle_smart_object_handle: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_conv_smart_object_slot_handle_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_conv_smart_object_request_result_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_conv_smart_object_handle_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_conv_smart_object_definition_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_conv_smart_object_claim_handle_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_add_smart_object: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_add_or_remove_smart_object: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_add_or_remove_multiple_smart_objects: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_blueprint_function_library_add_multiple_smart_objects: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_set_smart_object_enabled_for_reason: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_set_smart_object_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_set_definition: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_receive_on_event: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_is_smart_object_enabled_for_reason: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_is_smart_object_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_is_bound_to_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_component_get_definition: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_set_user_tag_filter: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_k2_get_slots: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_is_valid_slot_index: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_user_tags_filtering_policy: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_user_tag_filter: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_slot_world_transform: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_slot_activity_tags: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_mutable_slot: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_definition_get_activity_tags: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_persistent_collection_rebuild_collection: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_persistent_collection_clear_collection: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_set_slot_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_set_enabled_for_reason: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_remove_tag_from_slot: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_remove_tag_from_instance: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_release: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_is_enabled_for_reason: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_smart_object_component_by_request_result: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_smart_object_component: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_slot_transform_from_request_result: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_slot_transform: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_slot_tags: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_slot_state: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_slot_location: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_instance_tags: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_behavior_definition_by_request_result: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_behavior_definition: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_get_all_slots: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_find_smart_objects_bp: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_find_smart_objects: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_find_smart_object: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_add_tag_to_slot: *mut crate::ffi::UFunctionOpague,
    pub u_smart_object_subsystem_add_tag_to_instance: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_testing_actor_run_tests: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_testing_actor_reset_tests: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_testing_actor_debug_initialize_subsystem_runtime: *mut crate::ffi::UFunctionOpague,
    pub a_smart_object_testing_actor_debug_cleanup_subsystem_runtime: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_smart_object_blueprint_function_library_smart_object_claim_handle_invalid: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_set_value_as_so_claim_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_set_smart_object_enabled: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_set_multiple_smart_objects_enabled: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_set_blackboard_value_as_so_claim_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_remove_smart_object: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_remove_multiple_smart_objects: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_not_equal_smart_object_slot_handle_smart_object_slot_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_not_equal_smart_object_handle_smart_object_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_mark_smart_object_slot_as_occupied: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_mark_smart_object_slot_as_free: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_mark_smart_object_slot_as_claimed: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_is_valid_smart_object_slot_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_is_valid_smart_object_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_is_valid_smart_object_claim_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_get_value_as_so_claim_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_get_blackboard_value_as_so_claim_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_find_smart_objects_in_targeting_request: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_find_smart_objects_in_list: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_find_smart_objects_in_component: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_find_smart_objects_in_actor: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_equal_smart_object_slot_handle_smart_object_slot_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_equal_smart_object_handle_smart_object_handle: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_conv_smart_object_slot_handle_to_string: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_conv_smart_object_request_result_to_string: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_conv_smart_object_handle_to_string: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_conv_smart_object_definition_to_string: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_conv_smart_object_claim_handle_to_string: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_add_smart_object: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_add_or_remove_smart_object: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_add_or_remove_multiple_smart_objects: std::ptr::null_mut(),
            u_smart_object_blueprint_function_library_add_multiple_smart_objects: std::ptr::null_mut(),
            u_smart_object_component_set_smart_object_enabled_for_reason: std::ptr::null_mut(),
            u_smart_object_component_set_smart_object_enabled: std::ptr::null_mut(),
            u_smart_object_component_set_definition: std::ptr::null_mut(),
            u_smart_object_component_receive_on_event: std::ptr::null_mut(),
            u_smart_object_component_is_smart_object_enabled_for_reason: std::ptr::null_mut(),
            u_smart_object_component_is_smart_object_enabled: std::ptr::null_mut(),
            u_smart_object_component_is_bound_to_simulation: std::ptr::null_mut(),
            u_smart_object_component_get_definition: std::ptr::null_mut(),
            u_smart_object_definition_set_user_tag_filter: std::ptr::null_mut(),
            u_smart_object_definition_k2_get_slots: std::ptr::null_mut(),
            u_smart_object_definition_is_valid_slot_index: std::ptr::null_mut(),
            u_smart_object_definition_get_user_tags_filtering_policy: std::ptr::null_mut(),
            u_smart_object_definition_get_user_tag_filter: std::ptr::null_mut(),
            u_smart_object_definition_get_slot_world_transform: std::ptr::null_mut(),
            u_smart_object_definition_get_slot_activity_tags: std::ptr::null_mut(),
            u_smart_object_definition_get_mutable_slot: std::ptr::null_mut(),
            u_smart_object_definition_get_bounds: std::ptr::null_mut(),
            u_smart_object_definition_get_activity_tags: std::ptr::null_mut(),
            a_smart_object_persistent_collection_rebuild_collection: std::ptr::null_mut(),
            a_smart_object_persistent_collection_clear_collection: std::ptr::null_mut(),
            u_smart_object_subsystem_set_slot_enabled: std::ptr::null_mut(),
            u_smart_object_subsystem_set_enabled_for_reason: std::ptr::null_mut(),
            u_smart_object_subsystem_set_enabled: std::ptr::null_mut(),
            u_smart_object_subsystem_remove_tag_from_slot: std::ptr::null_mut(),
            u_smart_object_subsystem_remove_tag_from_instance: std::ptr::null_mut(),
            u_smart_object_subsystem_release: std::ptr::null_mut(),
            u_smart_object_subsystem_is_enabled_for_reason: std::ptr::null_mut(),
            u_smart_object_subsystem_is_enabled: std::ptr::null_mut(),
            u_smart_object_subsystem_get_smart_object_component_by_request_result: std::ptr::null_mut(),
            u_smart_object_subsystem_get_smart_object_component: std::ptr::null_mut(),
            u_smart_object_subsystem_get_slot_transform_from_request_result: std::ptr::null_mut(),
            u_smart_object_subsystem_get_slot_transform: std::ptr::null_mut(),
            u_smart_object_subsystem_get_slot_tags: std::ptr::null_mut(),
            u_smart_object_subsystem_get_slot_state: std::ptr::null_mut(),
            u_smart_object_subsystem_get_slot_location: std::ptr::null_mut(),
            u_smart_object_subsystem_get_instance_tags: std::ptr::null_mut(),
            u_smart_object_subsystem_get_behavior_definition_by_request_result: std::ptr::null_mut(),
            u_smart_object_subsystem_get_behavior_definition: std::ptr::null_mut(),
            u_smart_object_subsystem_get_all_slots: std::ptr::null_mut(),
            u_smart_object_subsystem_find_smart_objects_bp: std::ptr::null_mut(),
            u_smart_object_subsystem_find_smart_objects: std::ptr::null_mut(),
            u_smart_object_subsystem_find_smart_object: std::ptr::null_mut(),
            u_smart_object_subsystem_add_tag_to_slot: std::ptr::null_mut(),
            u_smart_object_subsystem_add_tag_to_instance: std::ptr::null_mut(),
            a_smart_object_testing_actor_run_tests: std::ptr::null_mut(),
            a_smart_object_testing_actor_reset_tests: std::ptr::null_mut(),
            a_smart_object_testing_actor_debug_initialize_subsystem_runtime: std::ptr::null_mut(),
            a_smart_object_testing_actor_debug_cleanup_subsystem_runtime: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USmartObjectBlueprintFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SmartObjectClaimHandle_Invalid"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_smart_object_claim_handle_invalid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsSOClaimHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_value_as_so_claim_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSmartObjectEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_smart_object_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMultipleSmartObjectsEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_multiple_smart_objects_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsSOClaimHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_blackboard_value_as_so_claim_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSmartObject"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_smart_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMultipleSmartObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_multiple_smart_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NotEqual_SmartObjectSlotHandleSmartObjectSlotHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_slot_handle_smart_object_slot_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_SmartObjectHandleSmartObjectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_handle_smart_object_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkSmartObjectSlotAsOccupied"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_occupied,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkSmartObjectSlotAsFree"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_free,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkSmartObjectSlotAsClaimed"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_claimed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidSmartObjectSlotHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_slot_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidSmartObjectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidSmartObjectClaimHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_claim_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsSOClaimHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_value_as_so_claim_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsSOClaimHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_blackboard_value_as_so_claim_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjectsInTargetingRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_targeting_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjectsInList"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjectsInComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjectsInActor"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "Equal_SmartObjectSlotHandleSmartObjectSlotHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_slot_handle_smart_object_slot_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Equal_SmartObjectHandleSmartObjectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_handle_smart_object_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_SmartObjectSlotHandleToString"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_slot_handle_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_SmartObjectRequestResultToString"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_request_result_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_SmartObjectHandleToString"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_handle_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_SmartObjectDefinitionToString"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_definition_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_SmartObjectClaimHandleToString"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_claim_handle_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSmartObject"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_smart_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrRemoveSmartObject"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_smart_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrRemoveMultipleSmartObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_multiple_smart_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMultipleSmartObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_multiple_smart_objects,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USmartObjectComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSmartObjectEnabledForReason"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled_for_reason,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSmartObjectEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefinition"),
                &raw mut __FUNCTION_PTRS.u_smart_object_component_set_definition,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveOnEvent"),
                &raw mut __FUNCTION_PTRS.u_smart_object_component_receive_on_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSmartObjectEnabledForReason"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_component_is_smart_object_enabled_for_reason,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSmartObjectEnabled"),
                &raw mut __FUNCTION_PTRS.u_smart_object_component_is_smart_object_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsBoundToSimulation"),
                &raw mut __FUNCTION_PTRS.u_smart_object_component_is_bound_to_simulation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefinition"),
                &raw mut __FUNCTION_PTRS.u_smart_object_component_get_definition,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USmartObjectDefinition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUserTagFilter"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_set_user_tag_filter,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetSlots"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_k2_get_slots,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidSlotIndex"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_is_valid_slot_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserTagsFilteringPolicy"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_definition_get_user_tags_filtering_policy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserTagFilter"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_get_user_tag_filter,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotWorldTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_definition_get_slot_world_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotActivityTags"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_get_slot_activity_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMutableSlot"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_get_mutable_slot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBounds"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_get_bounds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActivityTags"),
                &raw mut __FUNCTION_PTRS.u_smart_object_definition_get_activity_tags,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ASmartObjectPersistentCollection::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RebuildCollection"),
                &raw mut __FUNCTION_PTRS
                    .a_smart_object_persistent_collection_rebuild_collection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearCollection"),
                &raw mut __FUNCTION_PTRS
                    .a_smart_object_persistent_collection_clear_collection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USmartObjectSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSlotEnabled"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_set_slot_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnabledForReason"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_set_enabled_for_reason,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnabled"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_set_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTagFromSlot"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_remove_tag_from_slot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTagFromInstance"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_subsystem_remove_tag_from_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Release"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_release,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabledForReason"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_is_enabled_for_reason,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_is_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSmartObjectComponentByRequestResult"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component_by_request_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSmartObjectComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotTransformFromRequestResult"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_transform_from_request_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotTransform"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_slot_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotTags"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_slot_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotState"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_slot_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSlotLocation"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_slot_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstanceTags"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_instance_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBehaviorDefinitionByRequestResult"),
                &raw mut __FUNCTION_PTRS
                    .u_smart_object_subsystem_get_behavior_definition_by_request_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBehaviorDefinition"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_behavior_definition,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllSlots"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_get_all_slots,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjects_BP"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_find_smart_objects_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObjects"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_find_smart_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSmartObject"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_find_smart_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTagToSlot"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_add_tag_to_slot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTagToInstance"),
                &raw mut __FUNCTION_PTRS.u_smart_object_subsystem_add_tag_to_instance,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ASmartObjectTestingActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunTests"),
                &raw mut __FUNCTION_PTRS.a_smart_object_testing_actor_run_tests,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetTests"),
                &raw mut __FUNCTION_PTRS.a_smart_object_testing_actor_reset_tests,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DebugInitializeSubsystemRuntime"),
                &raw mut __FUNCTION_PTRS
                    .a_smart_object_testing_actor_debug_initialize_subsystem_runtime,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DebugCleanupSubsystemRuntime"),
                &raw mut __FUNCTION_PTRS
                    .a_smart_object_testing_actor_debug_cleanup_subsystem_runtime,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FSmartObjectEventData {
    pub smart_object_handle: FSmartObjectHandle,
    pub slot_handle: FSmartObjectSlotHandle,
    pub reason: ESmartObjectChangeReason,
    pub tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub(crate) __padding_end: [u8; 20],
}
impl FSmartObjectEventData {}
#[repr(C, align(4))]
pub struct FSmartObjectSlotHandle {
    pub(crate) __padding_end: [u8; 20],
}
impl FSmartObjectSlotHandle {}
#[repr(C, align(4))]
pub struct FSmartObjectHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FSmartObjectHandle {}
#[repr(C, align(16))]
pub struct FSmartObjectRequestFilter {
    pub user_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub claim_priority: ESmartObjectClaimPriority,
    pub activity_requirements: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub behavior_definition_classes: TArray<TSubclassOf<USmartObjectBehaviorDefinition>>,
    pub b_should_evaluate_conditions: bool,
    pub b_should_include_claimed_slots: bool,
    pub b_should_include_disabled_slots: bool,
    pub(crate) __padding_end: [u8; 61],
}
impl FSmartObjectRequestFilter {}
#[repr(C, align(16))]
pub struct FSmartObjectRequest {
    pub query_box: crate::bindings::core_u_object::FBox,
    pub filter: FSmartObjectRequestFilter,
}
impl FSmartObjectRequest {}
#[repr(C, align(4))]
pub struct FSmartObjectRequestResult {
    pub smart_object_handle: FSmartObjectHandle,
    pub slot_handle: FSmartObjectSlotHandle,
}
impl FSmartObjectRequestResult {}
#[repr(C, align(4))]
pub struct FSmartObjectUserCapsuleParams {
    pub radius: f32,
    pub height: f32,
    pub step_height: f32,
}
impl FSmartObjectUserCapsuleParams {}
#[repr(C, align(8))]
pub struct FSmartObjectSlotDefinition {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub offset: crate::bindings::core_u_object::FVector3f,
    pub rotation: crate::bindings::core_u_object::FRotator3f,
    pub b_enabled: bool,
    pub user_tag_filter: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub activity_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub runtime_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    #[doc(hidden)]
    pub(crate) __padding_248: [u8; 40],
    pub behavior_definitions: TArray<UPtr<USmartObjectBehaviorDefinition>>,
    pub(crate) __padding_end: [u8; 32],
}
impl FSmartObjectSlotDefinition {}
#[repr(C, align(8))]
pub struct FSmartObjectDefinitionPreviewData {
    pub object_actor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub object_mesh_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub user_actor_class: TSoftObjectPtr<crate::bindings::core_u_object::UClass>,
    pub user_validation_filter_class: TSoftObjectPtr<
        crate::bindings::core_u_object::UClass,
    >,
}
impl FSmartObjectDefinitionPreviewData {}
#[repr(C, align(4))]
pub struct FSmartObjectClaimHandle {
    pub smart_object_handle: FSmartObjectHandle,
    pub slot_handle: FSmartObjectSlotHandle,
    pub(crate) __padding_end: [u8; 4],
}
impl FSmartObjectClaimHandle {}
#[repr(C, align(4))]
pub struct FSmartObjectSlotEntranceHandle {
    pub slot_handle: FSmartObjectSlotHandle,
    pub(crate) __padding_end: [u8; 4],
}
impl FSmartObjectSlotEntranceHandle {}
#[repr(C, align(8))]
pub struct FSmartObjectSlotEntranceLocationRequest {
    pub user_actor: UPtr<crate::bindings::engine::AActor>,
    pub validation_filter: TSubclassOf<USmartObjectSlotValidationFilter>,
    pub navigation_data: UPtr<crate::bindings::navigation_system::ANavigationData>,
    pub user_capsule_params: FSmartObjectUserCapsuleParams,
    pub search_location: crate::bindings::core_u_object::FVector,
    pub select_method: FSmartObjectSlotEntrySelectionMethod,
    pub location_type: ESmartObjectSlotNavigationLocationType,
    pub b_project_navigation_location: bool,
    pub b_trace_ground_location: bool,
    pub b_check_transition_trajectory: bool,
    pub b_check_entrance_location_overlap: bool,
    pub b_check_slot_location_overlap: bool,
    pub b_use_slot_location_as_fallback: bool,
    pub b_use_up_axis_locked_rotation: bool,
    pub(crate) __padding_end: [u8; 23],
}
impl FSmartObjectSlotEntranceLocationRequest {}
#[repr(C, align(8))]
pub struct FSmartObjectSlotEntranceLocationResult {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 24],
    pub tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub entrance_handle: FSmartObjectSlotEntranceHandle,
    pub b_is_valid: bool,
}
impl FSmartObjectSlotEntranceLocationResult {}
#[repr(C, align(4))]
pub struct FSmartObjectSlotIndex {
    pub(crate) __padding_end: [u8; 4],
}
impl FSmartObjectSlotIndex {}
#[repr(C, align(8))]
pub struct USmartObjectSettings {
    __padding_end: [u8; 128],
}
impl USmartObjectSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSettings")
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
pub struct UBlackboardKeyType_SOClaimHandle {
    __padding_end: [u8; 96],
}
impl UBlackboardKeyType_SOClaimHandle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_SOClaimHandle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_SOClaimHandle")
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
#[repr(C, align(16))]
pub struct UEnvQueryGenerator_SmartObjects {
    __padding_end: [u8; 320],
}
impl UEnvQueryGenerator_SmartObjects {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_SmartObjects")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_SmartObjects")
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
pub struct UEnvQueryItemType_SmartObject {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_SmartObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_SmartObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_SmartObject")
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
pub struct AGenericSmartObject {
    __padding_end: [u8; 1152],
}
impl AGenericSmartObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGenericSmartObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGenericSmartObject")
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
pub struct USmartObjectBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USmartObjectBlueprintFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectBlueprintFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectBlueprintFunctionLibrary")
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
    pub fn smart_object_claim_handle_invalid() -> FSmartObjectClaimHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_smart_object_claim_handle_invalid,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_smart_object_claim_handle_invalid,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSmartObjectClaimHandle>().read() }
    }
    pub fn set_value_as_so_claim_handle(
        blackboard_component: UPtr<crate::bindings::ai_module::UBlackboardComponent>,
        key_name: &FName,
        value: FSmartObjectClaimHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blackboard_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::ai_module::UBlackboardComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(20).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_value_as_so_claim_handle,
                __buffer,
            )
        };
    }
    pub fn set_smart_object_enabled(
        smart_object_actor: UPtr<crate::bindings::engine::AActor>,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_smart_object_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &smart_object_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_smart_object_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_multiple_smart_objects_enabled(
        smart_object_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_multiple_smart_objects_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                smart_object_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_multiple_smart_objects_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_blackboard_value_as_so_claim_handle(
        node_owner: UPtr<crate::bindings::ai_module::UBTNode>,
        key: &crate::bindings::ai_module::FBlackboardKeySelector,
        value: &FSmartObjectClaimHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_blackboard_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::ai_module::UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::ai_module::FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(56).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_set_blackboard_value_as_so_claim_handle,
                __buffer,
            )
        };
    }
    pub fn remove_smart_object(
        smart_object_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_smart_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &smart_object_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_smart_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_multiple_smart_objects(
        smart_object_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                smart_object_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_remove_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn not_equal_smart_object_slot_handle_smart_object_slot_handle(
        a: &FSmartObjectSlotHandle,
        b: &FSmartObjectSlotHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_slot_handle_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(20).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_slot_handle_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn not_equal_smart_object_handle_smart_object_handle(
        a: &FSmartObjectHandle,
        b: &FSmartObjectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_handle_smart_object_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(16).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_not_equal_smart_object_handle_smart_object_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn mark_smart_object_slot_as_occupied(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        claim_handle: FSmartObjectClaimHandle,
        definition_class: TSubclassOf<USmartObjectBehaviorDefinition>,
    ) -> UPtr<USmartObjectBehaviorDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_occupied,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &claim_handle,
                __buffer.add(8).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &definition_class,
                __buffer.add(48).cast::<TSubclassOf<USmartObjectBehaviorDefinition>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_occupied,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<USmartObjectBehaviorDefinition>>().read() }
    }
    pub fn mark_smart_object_slot_as_free(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        claim_handle: FSmartObjectClaimHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_free,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &claim_handle,
                __buffer.add(8).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_free,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn mark_smart_object_slot_as_claimed(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        slot_handle: FSmartObjectSlotHandle,
        user_actor: UPtr<crate::bindings::engine::AActor>,
        claim_priority: ESmartObjectClaimPriority,
    ) -> FSmartObjectClaimHandle {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_claimed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(8).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &claim_priority,
                __buffer.add(40).cast::<ESmartObjectClaimPriority>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_mark_smart_object_slot_as_claimed,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<FSmartObjectClaimHandle>().read() }
    }
    pub fn is_valid_smart_object_slot_handle(handle: &FSmartObjectSlotHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn is_valid_smart_object_handle(handle: &FSmartObjectHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_valid_smart_object_claim_handle(handle: FSmartObjectClaimHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_claim_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_is_valid_smart_object_claim_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_value_as_so_claim_handle(
        blackboard_component: UPtr<crate::bindings::ai_module::UBlackboardComponent>,
        key_name: &FName,
    ) -> FSmartObjectClaimHandle {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blackboard_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::ai_module::UBlackboardComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(8).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<FSmartObjectClaimHandle>().read() }
    }
    pub fn get_blackboard_value_as_so_claim_handle(
        node_owner: UPtr<crate::bindings::ai_module::UBTNode>,
        key: &crate::bindings::ai_module::FBlackboardKeySelector,
    ) -> FSmartObjectClaimHandle {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_blackboard_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::ai_module::UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::ai_module::FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_get_blackboard_value_as_so_claim_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FSmartObjectClaimHandle>().read() }
    }
    pub fn find_smart_objects_in_targeting_request(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        filter: &FSmartObjectRequestFilter,
        targeting_handle: crate::bindings::targeting_system::FTargetingRequestHandle,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<241>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_targeting_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(16).cast::<FSmartObjectRequestFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_handle,
                __buffer
                    .add(208)
                    .cast::<
                        crate::bindings::targeting_system::FTargetingRequestHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(216).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(232).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_targeting_request,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(216)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(240).cast::<bool>().read() }
    }
    pub fn find_smart_objects_in_list(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        filter: &FSmartObjectRequestFilter,
        actor_list: &TArray<UPtr<crate::bindings::engine::AActor>>,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<249>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(16).cast::<FSmartObjectRequestFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actor_list,
                __buffer
                    .add(208)
                    .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(224).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(240).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_list,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(224)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(248).cast::<bool>().read() }
    }
    pub fn find_smart_objects_in_component(
        filter: &FSmartObjectRequestFilter,
        smart_object_component: UPtr<USmartObjectComponent>,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(0).cast::<FSmartObjectRequestFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &smart_object_component,
                __buffer.add(192).cast::<UPtr<USmartObjectComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(200).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(216).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(200)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn find_smart_objects_in_actor(
        filter: &FSmartObjectRequestFilter,
        search_actor: UPtr<crate::bindings::engine::AActor>,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                filter,
                __buffer.add(0).cast::<FSmartObjectRequestFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_actor,
                __buffer.add(192).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(200).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(216).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_find_smart_objects_in_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(200)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn equal_smart_object_slot_handle_smart_object_slot_handle(
        a: &FSmartObjectSlotHandle,
        b: &FSmartObjectSlotHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_slot_handle_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(20).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_slot_handle_smart_object_slot_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn equal_smart_object_handle_smart_object_handle(
        a: &FSmartObjectHandle,
        b: &FSmartObjectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_handle_smart_object_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(16).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_equal_smart_object_handle_smart_object_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn conv_smart_object_slot_handle_to_string(
        handle: &FSmartObjectSlotHandle,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_slot_handle_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_slot_handle_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn conv_smart_object_request_result_to_string(
        result: &FSmartObjectRequestResult,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_request_result_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FSmartObjectRequestResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_request_result_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FString>().read() }
    }
    pub fn conv_smart_object_handle_to_string(handle: &FSmartObjectHandle) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_handle_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_handle_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn conv_smart_object_definition_to_string(
        definition: UPtr<USmartObjectDefinition>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_definition_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &definition,
                __buffer.add(0).cast::<UPtr<USmartObjectDefinition>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_definition_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn conv_smart_object_claim_handle_to_string(
        result: &FSmartObjectClaimHandle,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_claim_handle_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_conv_smart_object_claim_handle_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FString>().read() }
    }
    pub fn add_smart_object(
        smart_object_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_smart_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &smart_object_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_smart_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn add_or_remove_smart_object(
        smart_object: UPtr<crate::bindings::engine::AActor>,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_smart_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &smart_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_smart_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn add_or_remove_multiple_smart_objects(
        smart_object_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        b_add: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                smart_object_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_add, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_or_remove_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn add_multiple_smart_objects(
        smart_object_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                smart_object_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::smart_objects_module::USmartObjectBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_blueprint_function_library_add_multiple_smart_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ADEPRECATED_SmartObjectCollection {
    __padding_end: [u8; 1312],
}
impl ADEPRECATED_SmartObjectCollection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADEPRECATED_SmartObjectCollection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADEPRECATED_SmartObjectCollection")
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
#[repr(C, align(16))]
pub struct USmartObjectComponent {
    #[doc(hidden)]
    pub(crate) __padding_768: [u8; 768],
    pub registered_handle: FSmartObjectHandle,
    #[doc(hidden)]
    pub(crate) __padding_800: [u8; 16],
    pub cached_definition_asset_variation: UPtr<USmartObjectDefinition>,
    #[doc(hidden)]
    pub(crate) __padding_824: [u8; 16],
    pub definition_asset_deprecated: UPtr<USmartObjectDefinition>,
}
impl USmartObjectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectComponent")
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
    pub fn set_smart_object_enabled_for_reason(
        &self,
        reason_tag: crate::bindings::gameplay_tags::FGameplayTag,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled_for_reason,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reason_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled_for_reason,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn set_smart_object_enabled(&self, b_enable: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_smart_object_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_definition(&mut self, definition_asset: UPtr<USmartObjectDefinition>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_definition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &definition_asset,
                __buffer.add(0).cast::<UPtr<USmartObjectDefinition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_set_definition,
                __buffer,
            )
        };
    }
    pub fn receive_on_event(
        &mut self,
        event_data: &FSmartObjectEventData,
        interactor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_receive_on_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                event_data,
                __buffer.add(0).cast::<FSmartObjectEventData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interactor,
                __buffer.add(72).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_receive_on_event,
                __buffer,
            )
        };
    }
    pub fn is_smart_object_enabled_for_reason(
        &self,
        reason_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_smart_object_enabled_for_reason,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reason_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_smart_object_enabled_for_reason,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_smart_object_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_smart_object_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_smart_object_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_bound_to_simulation(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_bound_to_simulation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_is_bound_to_simulation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_definition(&self) -> UPtr<USmartObjectDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_get_definition,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_component_get_definition,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<USmartObjectDefinition>>().read() }
    }
}
#[repr(C, align(16))]
pub struct USmartObjectContainerRenderingComponent {
    __padding_end: [u8; 1504],
}
impl USmartObjectContainerRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectContainerRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectContainerRenderingComponent")
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
#[repr(C, align(16))]
pub struct USmartObjectDebugRenderingComponent {
    __padding_end: [u8; 1632],
}
impl USmartObjectDebugRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectDebugRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectDebugRenderingComponent")
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
pub struct USmartObjectBehaviorDefinition {
    __padding_end: [u8; 48],
}
impl USmartObjectBehaviorDefinition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectBehaviorDefinition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectBehaviorDefinition")
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
#[repr(C, align(16))]
pub struct USmartObjectDefinition {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub preview_data: FSmartObjectDefinitionPreviewData,
    __padding_end: [u8; 592],
}
impl USmartObjectDefinition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectDefinition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectDefinition")
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
    pub fn set_user_tag_filter(
        &mut self,
        in_user_tag_filter: &crate::bindings::gameplay_tags::FGameplayTagQuery,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_set_user_tag_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_user_tag_filter,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_set_user_tag_filter,
                __buffer,
            )
        };
    }
    pub fn k2_get_slots(&self) -> TArray<FSmartObjectSlotDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_k2_get_slots,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_k2_get_slots,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FSmartObjectSlotDefinition>>().read() }
    }
    pub fn is_valid_slot_index(&self, slot_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_is_valid_slot_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&slot_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_is_valid_slot_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_user_tags_filtering_policy(&self) -> ESmartObjectTagFilteringPolicy {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_user_tags_filtering_policy,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_user_tags_filtering_policy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ESmartObjectTagFilteringPolicy>().read() }
    }
    pub fn get_user_tag_filter(
        &self,
    ) -> crate::bindings::gameplay_tags::FGameplayTagQuery {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_user_tag_filter,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_user_tag_filter,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>()
                .read()
        }
    }
    pub fn get_slot_world_transform(
        &self,
        slot_index: i32,
        owner_transform: &crate::bindings::core_u_object::FTransform,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_slot_world_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&slot_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                owner_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_slot_world_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_slot_activity_tags_by_index(
        &self,
        slot_index: i32,
        out_activity_tags: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_slot_activity_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&slot_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_activity_tags,
                __buffer
                    .add(8)
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
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_slot_activity_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(out_activity_tags);
        }
    }
    pub fn get_mutable_slot(&mut self, index: i32) -> FSmartObjectSlotDefinition {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_mutable_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_mutable_slot,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FSmartObjectSlotDefinition>().read() }
    }
    pub fn get_bounds(&self) -> crate::bindings::core_u_object::FBox {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_bounds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_bounds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>().read() }
    }
    pub fn get_activity_tags(
        &self,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_activity_tags,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_definition_get_activity_tags,
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
}
#[repr(C, align(8))]
pub struct USmartObjectSpacePartition {
    __padding_end: [u8; 48],
}
impl USmartObjectSpacePartition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSpacePartition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSpacePartition")
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
pub struct USmartObjectHashGrid {
    __padding_end: [u8; 216],
}
impl USmartObjectHashGrid {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectHashGrid")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectHashGrid")
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
#[repr(C, align(16))]
pub struct USmartObjectOctree {
    __padding_end: [u8; 240],
}
impl USmartObjectOctree {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectOctree")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectOctree")
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
pub struct ASmartObjectPersistentCollection {
    __padding_end: [u8; 1440],
}
impl ASmartObjectPersistentCollection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectPersistentCollection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectPersistentCollection")
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
#[repr(C, align(16))]
pub struct USmartObjectRenderingComponent {
    __padding_end: [u8; 1504],
}
impl USmartObjectRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectRenderingComponent")
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
pub struct USmartObjectSubsystem {
    __padding_end: [u8; 624],
}
impl USmartObjectSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSubsystem")
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
    pub fn set_slot_enabled(
        &mut self,
        slot_handle: FSmartObjectSlotHandle,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_slot_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_slot_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn set_enabled_for_reason(
        &mut self,
        handle: FSmartObjectHandle,
        reason_tag: crate::bindings::gameplay_tags::FGameplayTag,
        b_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_enabled_for_reason,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reason_tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_enabled_for_reason,
                __buffer,
            )
        };
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn set_enabled(&mut self, handle: FSmartObjectHandle, b_enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
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
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_set_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn remove_tag_from_slot(
        &mut self,
        slot_handle: FSmartObjectSlotHandle,
        tag: &crate::bindings::gameplay_tags::FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_remove_tag_from_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag,
                __buffer.add(20).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_remove_tag_from_slot,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_tag_from_instance(
        &mut self,
        handle: FSmartObjectHandle,
        tag: &crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_remove_tag_from_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_remove_tag_from_instance,
                __buffer,
            )
        };
    }
    pub fn release(&mut self, claim_handle: &FSmartObjectClaimHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_release,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                claim_handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_release,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn is_enabled_for_reason(
        &self,
        handle: FSmartObjectHandle,
        reason_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_is_enabled_for_reason,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reason_tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_is_enabled_for_reason,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn is_enabled(&self, handle: FSmartObjectHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_is_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_smart_object_component_by_request_result(
        &self,
        result: &FSmartObjectRequestResult,
        try_spawn_actor_if_dehydrated: ETrySpawnActorIfDehydrated,
    ) -> UPtr<USmartObjectComponent> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component_by_request_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(0).cast::<FSmartObjectRequestResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &try_spawn_actor_if_dehydrated,
                __buffer.add(36).cast::<ETrySpawnActorIfDehydrated>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component_by_request_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<USmartObjectComponent>>().read() }
    }
    pub fn get_smart_object_component(
        &self,
        claim_handle: &FSmartObjectClaimHandle,
        try_spawn_actor_if_dehydrated: ETrySpawnActorIfDehydrated,
    ) -> UPtr<USmartObjectComponent> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                claim_handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &try_spawn_actor_if_dehydrated,
                __buffer.add(40).cast::<ETrySpawnActorIfDehydrated>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_smart_object_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<USmartObjectComponent>>().read() }
    }
    pub fn get_slot_transform_from_request_result(
        &self,
        request_result: &FSmartObjectRequestResult,
        out_slot_transform: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_transform_from_request_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                request_result,
                __buffer.add(0).cast::<FSmartObjectRequestResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_slot_transform,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_transform_from_request_result,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(out_slot_transform);
        }
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn get_slot_transform(
        &self,
        claim_handle: &FSmartObjectClaimHandle,
        out_slot_transform: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                claim_handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_slot_transform,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(out_slot_transform);
        }
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn get_slot_tags(
        &self,
        slot_handle: FSmartObjectSlotHandle,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_slot_state(
        &self,
        slot_handle: FSmartObjectSlotHandle,
    ) -> ESmartObjectSlotState {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<ESmartObjectSlotState>().read() }
    }
    pub fn get_slot_location(
        &self,
        claim_handle: &FSmartObjectClaimHandle,
        out_slot_location: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                claim_handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_slot_location,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_slot_location,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_slot_location);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_instance_tags(
        &self,
        handle: FSmartObjectHandle,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_instance_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_instance_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_behavior_definition_by_request_result(
        &mut self,
        request_result: &FSmartObjectRequestResult,
        definition_class: TSubclassOf<USmartObjectBehaviorDefinition>,
    ) -> UPtr<USmartObjectBehaviorDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_behavior_definition_by_request_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                request_result,
                __buffer.add(0).cast::<FSmartObjectRequestResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &definition_class,
                __buffer.add(40).cast::<TSubclassOf<USmartObjectBehaviorDefinition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_behavior_definition_by_request_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<USmartObjectBehaviorDefinition>>().read() }
    }
    pub fn get_behavior_definition(
        &mut self,
        claim_handle: &FSmartObjectClaimHandle,
        definition_class: TSubclassOf<USmartObjectBehaviorDefinition>,
    ) -> UPtr<USmartObjectBehaviorDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_behavior_definition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                claim_handle,
                __buffer.add(0).cast::<FSmartObjectClaimHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &definition_class,
                __buffer.add(40).cast::<TSubclassOf<USmartObjectBehaviorDefinition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_behavior_definition,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<USmartObjectBehaviorDefinition>>().read() }
    }
    pub fn get_all_slots(
        &self,
        handle: FSmartObjectHandle,
        out_slots: &mut TArray<FSmartObjectSlotHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_all_slots,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_slots,
                __buffer.add(16).cast::<TArray<FSmartObjectSlotHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_get_all_slots,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<FSmartObjectSlotHandle>>().swap(out_slots);
        }
    }
    pub fn find_smart_objects_bp(
        &self,
        request: &FSmartObjectRequest,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_objects_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                request,
                __buffer.add(0).cast::<FSmartObjectRequest>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(256).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(272).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_objects_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(256)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(280).cast::<bool>().read() }
    }
    pub fn find_smart_objects(
        &self,
        request: &FSmartObjectRequest,
        out_results: &mut TArray<FSmartObjectRequestResult>,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                request,
                __buffer.add(0).cast::<FSmartObjectRequest>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_results,
                __buffer.add(256).cast::<TArray<FSmartObjectRequestResult>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(272).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(256)
                .cast::<TArray<FSmartObjectRequestResult>>()
                .swap(out_results);
        }
        unsafe { __buffer.add(280).cast::<bool>().read() }
    }
    pub fn find_smart_object(
        &self,
        request: &FSmartObjectRequest,
        user_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> FSmartObjectRequestResult {
        let mut __stack = crate::core_data::StackAlloc::<300>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                request,
                __buffer.add(0).cast::<FSmartObjectRequest>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_actor,
                __buffer.add(256).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_find_smart_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(264).cast::<FSmartObjectRequestResult>().read() }
    }
    pub fn add_tag_to_slot(
        &mut self,
        slot_handle: FSmartObjectSlotHandle,
        tag: &crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_add_tag_to_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_handle,
                __buffer.add(0).cast::<FSmartObjectSlotHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag,
                __buffer.add(20).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_add_tag_to_slot,
                __buffer,
            )
        };
    }
    pub fn add_tag_to_instance(
        &mut self,
        handle: FSmartObjectHandle,
        tag: &crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_add_tag_to_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FSmartObjectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .u_smart_object_subsystem_add_tag_to_instance,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct USmartObjectSubsystemRenderingComponent {
    __padding_end: [u8; 1632],
}
impl USmartObjectSubsystemRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSubsystemRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSubsystemRenderingComponent")
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
pub struct ASmartObjectSubsystemRenderingActor {
    __padding_end: [u8; 1144],
}
impl ASmartObjectSubsystemRenderingActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectSubsystemRenderingActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectSubsystemRenderingActor")
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
pub struct USmartObjectTest {
    __padding_end: [u8; 56],
}
impl USmartObjectTest {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectTest")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectTest")
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
#[repr(C, align(16))]
pub struct USmartObjectSimpleQueryTest {
    __padding_end: [u8; 336],
}
impl USmartObjectSimpleQueryTest {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSimpleQueryTest")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSimpleQueryTest")
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
#[repr(C, align(16))]
pub struct USmartObjectTestRenderingComponent {
    __padding_end: [u8; 1632],
}
impl USmartObjectTestRenderingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectTestRenderingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectTestRenderingComponent")
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
pub struct ASmartObjectTestingActor {
    __padding_end: [u8; 1176],
}
impl ASmartObjectTestingActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectTestingActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASmartObjectTestingActor")
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
    pub fn run_tests(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .a_smart_object_testing_actor_run_tests,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .a_smart_object_testing_actor_run_tests,
                __buffer,
            )
        };
    }
    pub fn reset_tests(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .a_smart_object_testing_actor_reset_tests,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::smart_objects_module::__FUNCTION_PTRS
                    .a_smart_object_testing_actor_reset_tests,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USmartObjectSlotValidationFilter {
    __padding_end: [u8; 312],
}
impl USmartObjectSlotValidationFilter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSlotValidationFilter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectSlotValidationFilter")
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
pub struct USmartObjectUserComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub validation_filter: TSubclassOf<USmartObjectSlotValidationFilter>,
}
impl USmartObjectUserComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectUserComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectUserComponent")
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
pub struct USmartObjectWorldConditionSchema {
    __padding_end: [u8; 144],
}
impl USmartObjectWorldConditionSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectWorldConditionSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmartObjectWorldConditionSchema")
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
pub struct FSmartObjectComponent_OnSmartObjectEvent {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ESmartObjectChangeReason(pub u8);
impl ESmartObjectChangeReason {
    pub const NONE: ESmartObjectChangeReason = ESmartObjectChangeReason(0);
    pub const ON_EVENT: ESmartObjectChangeReason = ESmartObjectChangeReason(1);
    pub const ON_TAG_ADDED: ESmartObjectChangeReason = ESmartObjectChangeReason(2);
    pub const ON_TAG_REMOVED: ESmartObjectChangeReason = ESmartObjectChangeReason(3);
    pub const ON_CLAIMED: ESmartObjectChangeReason = ESmartObjectChangeReason(4);
    pub const ON_OCCUPIED: ESmartObjectChangeReason = ESmartObjectChangeReason(5);
    pub const ON_RELEASED: ESmartObjectChangeReason = ESmartObjectChangeReason(6);
    pub const ON_SLOT_ENABLED: ESmartObjectChangeReason = ESmartObjectChangeReason(7);
    pub const ON_SLOT_DISABLED: ESmartObjectChangeReason = ESmartObjectChangeReason(8);
    pub const ON_OBJECT_ENABLED: ESmartObjectChangeReason = ESmartObjectChangeReason(9);
    pub const ON_OBJECT_DISABLED: ESmartObjectChangeReason = ESmartObjectChangeReason(
        10,
    );
    pub const ON_COMPONENT_BOUND: ESmartObjectChangeReason = ESmartObjectChangeReason(
        11,
    );
    pub const ON_COMPONENT_UNBOUND: ESmartObjectChangeReason = ESmartObjectChangeReason(
        12,
    );
}
#[repr(transparent)]
pub struct ESmartObjectPropertyPathRetargetingStatus(pub u8);
impl ESmartObjectPropertyPathRetargetingStatus {
    pub const NO_RETARGETING: ESmartObjectPropertyPathRetargetingStatus = ESmartObjectPropertyPathRetargetingStatus(
        0,
    );
    pub const PICKED_PATH: ESmartObjectPropertyPathRetargetingStatus = ESmartObjectPropertyPathRetargetingStatus(
        1,
    );
    pub const RETARGETED_PATH: ESmartObjectPropertyPathRetargetingStatus = ESmartObjectPropertyPathRetargetingStatus(
        2,
    );
}
#[repr(transparent)]
pub struct ESmartObjectClaimPriority(pub u8);
impl ESmartObjectClaimPriority {
    pub const NONE: ESmartObjectClaimPriority = ESmartObjectClaimPriority(0);
    pub const LOW: ESmartObjectClaimPriority = ESmartObjectClaimPriority(1);
    pub const BELOW_NORMAL: ESmartObjectClaimPriority = ESmartObjectClaimPriority(2);
    pub const NORMAL: ESmartObjectClaimPriority = ESmartObjectClaimPriority(3);
    pub const ABOVE_NORMAL: ESmartObjectClaimPriority = ESmartObjectClaimPriority(4);
    pub const HIGH: ESmartObjectClaimPriority = ESmartObjectClaimPriority(5);
    pub const MIN: ESmartObjectClaimPriority = ESmartObjectClaimPriority(0);
    pub const MAX: ESmartObjectClaimPriority = ESmartObjectClaimPriority(5);
}
#[repr(transparent)]
pub struct ESmartObjectEntrancePriority(pub u8);
impl ESmartObjectEntrancePriority {
    pub const LOWEST: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(0);
    pub const LOWER: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(1);
    pub const LOW: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(2);
    pub const BELOW_NORMAL: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(
        3,
    );
    pub const NORMAL: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(4);
    pub const ABOVE_NORMAL: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(
        5,
    );
    pub const HIGH: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(6);
    pub const HIGHER: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(7);
    pub const HIGHEST: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(8);
    pub const MIN: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(0);
    pub const MAX: ESmartObjectEntrancePriority = ESmartObjectEntrancePriority(8);
}
#[repr(transparent)]
pub struct ESmartObjectSlotShape(pub u8);
impl ESmartObjectSlotShape {
    pub const CIRCLE: ESmartObjectSlotShape = ESmartObjectSlotShape(0);
    pub const RECTANGLE: ESmartObjectSlotShape = ESmartObjectSlotShape(1);
}
#[repr(transparent)]
pub struct ESmartObjectSlotState(pub u8);
impl ESmartObjectSlotState {
    pub const INVALID: ESmartObjectSlotState = ESmartObjectSlotState(0);
    pub const FREE: ESmartObjectSlotState = ESmartObjectSlotState(1);
    pub const CLAIMED: ESmartObjectSlotState = ESmartObjectSlotState(2);
    pub const OCCUPIED: ESmartObjectSlotState = ESmartObjectSlotState(3);
    pub const DISABLED: ESmartObjectSlotState = ESmartObjectSlotState(4);
}
#[repr(transparent)]
pub struct FSmartObjectSlotEntrySelectionMethod(pub u8);
impl FSmartObjectSlotEntrySelectionMethod {
    pub const FIRST: FSmartObjectSlotEntrySelectionMethod = FSmartObjectSlotEntrySelectionMethod(
        0,
    );
    pub const NEAREST_TO_SEARCH_LOCATION: FSmartObjectSlotEntrySelectionMethod = FSmartObjectSlotEntrySelectionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct ESmartObjectSlotNavigationLocationType(pub u8);
impl ESmartObjectSlotNavigationLocationType {
    pub const ENTRY: ESmartObjectSlotNavigationLocationType = ESmartObjectSlotNavigationLocationType(
        0,
    );
    pub const EXIT: ESmartObjectSlotNavigationLocationType = ESmartObjectSlotNavigationLocationType(
        1,
    );
}
#[repr(transparent)]
pub struct ESmartObjectTraceType(pub u8);
impl ESmartObjectTraceType {
    pub const BY_CHANNEL: ESmartObjectTraceType = ESmartObjectTraceType(0);
    pub const BY_PROFILE: ESmartObjectTraceType = ESmartObjectTraceType(1);
    pub const BY_OBJECT_TYPES: ESmartObjectTraceType = ESmartObjectTraceType(2);
}
#[repr(transparent)]
pub struct ESmartObjectTagFilteringPolicy(pub u8);
impl ESmartObjectTagFilteringPolicy {
    pub const NO_FILTER: ESmartObjectTagFilteringPolicy = ESmartObjectTagFilteringPolicy(
        0,
    );
    pub const COMBINE: ESmartObjectTagFilteringPolicy = ESmartObjectTagFilteringPolicy(
        1,
    );
    pub const OVERRIDE: ESmartObjectTagFilteringPolicy = ESmartObjectTagFilteringPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct ETrySpawnActorIfDehydrated(pub u8);
impl ETrySpawnActorIfDehydrated {
    pub const NO: ETrySpawnActorIfDehydrated = ETrySpawnActorIfDehydrated(0);
    pub const YES: ETrySpawnActorIfDehydrated = ETrySpawnActorIfDehydrated(1);
}
#[repr(transparent)]
pub struct ESmartObjectTagMergingPolicy(pub u8);
impl ESmartObjectTagMergingPolicy {
    pub const COMBINE: ESmartObjectTagMergingPolicy = ESmartObjectTagMergingPolicy(0);
    pub const OVERRIDE: ESmartObjectTagMergingPolicy = ESmartObjectTagMergingPolicy(1);
}
