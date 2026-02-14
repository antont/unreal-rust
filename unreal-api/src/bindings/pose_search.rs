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
    pub u_multi_anim_asset_bp_get_origin: *mut crate::ffi::UFunctionOpague,
    pub u_multi_anim_asset_bp_get_animation_asset: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_predictor_interface_predict: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_predictor_interface_get_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_predictor_interface_get_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_predictor_interface_get_current_state: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_set_interrupt_mode: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_set_database_to_search: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_set_databases_to_search: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_reset_databases_to_search: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_override_motion_matching_blend_settings: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_get_motion_matching_search_result: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_get_motion_matching_blend_settings: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_convert_to_motion_matching_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_anim_node_library_convert_to_motion_matching_node: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_interaction_anim_node_library_set_availabilities: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_interaction_anim_node_library_is_interacting: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_asset_sampler_library_sample_pose: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_asset_sampler_library_get_transform_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_asset_sampler_library_draw: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_database_get_num_animation_assets: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_database_get_animation_asset: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_event_library_update_pose_search_event: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_curve_bp_get_curve_value: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_distance_bp_get_distance: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_heading_bp_get_world_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_position_bp_get_world_position: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_time_to_event_bp_get_time_to_event: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_feature_channel_velocity_bp_get_world_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_history_collector_anim_node_library_set_pose_history_node_transform_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_history_collector_anim_node_library_get_pose_history_reference: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_history_collector_anim_node_library_get_pose_history_node_transform_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node_pure: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_interaction_library_motion_match_interaction_pure: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_interaction_library_motion_match_interaction: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_interaction_library_get_montage_continuing_properties: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_library_motion_match: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_library_is_animation_asset_looping: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_library_get_database_tags: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_pose_search_generate_transform_trajectory_with_predictor: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_pose_search_generate_transform_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_pose_search_generate_predictor_transform_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions_with_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_get_transform_trajectory_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_get_transform_trajectory_sample_transform: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_get_transform_trajectory_sample_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_get_transform_trajectory_angular_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_pose_search_trajectory_library_draw_transform_trajectory: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_multi_anim_asset_bp_get_origin: std::ptr::null_mut(),
            u_multi_anim_asset_bp_get_animation_asset: std::ptr::null_mut(),
            u_pose_search_trajectory_predictor_interface_predict: std::ptr::null_mut(),
            u_pose_search_trajectory_predictor_interface_get_velocity: std::ptr::null_mut(),
            u_pose_search_trajectory_predictor_interface_get_gravity: std::ptr::null_mut(),
            u_pose_search_trajectory_predictor_interface_get_current_state: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_set_interrupt_mode: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_set_database_to_search: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_set_databases_to_search: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_reset_databases_to_search: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_override_motion_matching_blend_settings: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_get_motion_matching_search_result: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_get_motion_matching_blend_settings: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_convert_to_motion_matching_node_pure: std::ptr::null_mut(),
            u_motion_matching_anim_node_library_convert_to_motion_matching_node: std::ptr::null_mut(),
            u_motion_matching_interaction_anim_node_library_set_availabilities: std::ptr::null_mut(),
            u_motion_matching_interaction_anim_node_library_is_interacting: std::ptr::null_mut(),
            u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node_pure: std::ptr::null_mut(),
            u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node: std::ptr::null_mut(),
            u_pose_search_asset_sampler_library_sample_pose: std::ptr::null_mut(),
            u_pose_search_asset_sampler_library_get_transform_by_name: std::ptr::null_mut(),
            u_pose_search_asset_sampler_library_draw: std::ptr::null_mut(),
            u_pose_search_database_get_num_animation_assets: std::ptr::null_mut(),
            u_pose_search_database_get_animation_asset: std::ptr::null_mut(),
            u_pose_search_event_library_update_pose_search_event: std::ptr::null_mut(),
            u_pose_search_feature_channel_curve_bp_get_curve_value: std::ptr::null_mut(),
            u_pose_search_feature_channel_distance_bp_get_distance: std::ptr::null_mut(),
            u_pose_search_feature_channel_heading_bp_get_world_rotation: std::ptr::null_mut(),
            u_pose_search_feature_channel_position_bp_get_world_position: std::ptr::null_mut(),
            u_pose_search_feature_channel_time_to_event_bp_get_time_to_event: std::ptr::null_mut(),
            u_pose_search_feature_channel_velocity_bp_get_world_velocity: std::ptr::null_mut(),
            u_pose_search_history_collector_anim_node_library_set_pose_history_node_transform_trajectory: std::ptr::null_mut(),
            u_pose_search_history_collector_anim_node_library_get_pose_history_reference: std::ptr::null_mut(),
            u_pose_search_history_collector_anim_node_library_get_pose_history_node_transform_trajectory: std::ptr::null_mut(),
            u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node_pure: std::ptr::null_mut(),
            u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node: std::ptr::null_mut(),
            u_pose_search_interaction_library_motion_match_interaction_pure: std::ptr::null_mut(),
            u_pose_search_interaction_library_motion_match_interaction: std::ptr::null_mut(),
            u_pose_search_interaction_library_get_montage_continuing_properties: std::ptr::null_mut(),
            u_pose_search_library_motion_match: std::ptr::null_mut(),
            u_pose_search_library_is_animation_asset_looping: std::ptr::null_mut(),
            u_pose_search_library_get_database_tags: std::ptr::null_mut(),
            u_pose_search_trajectory_library_pose_search_generate_transform_trajectory_with_predictor: std::ptr::null_mut(),
            u_pose_search_trajectory_library_pose_search_generate_transform_trajectory: std::ptr::null_mut(),
            u_pose_search_trajectory_library_pose_search_generate_predictor_transform_trajectory: std::ptr::null_mut(),
            u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions_with_gravity: std::ptr::null_mut(),
            u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions: std::ptr::null_mut(),
            u_pose_search_trajectory_library_get_transform_trajectory_velocity: std::ptr::null_mut(),
            u_pose_search_trajectory_library_get_transform_trajectory_sample_transform: std::ptr::null_mut(),
            u_pose_search_trajectory_library_get_transform_trajectory_sample_at_time: std::ptr::null_mut(),
            u_pose_search_trajectory_library_get_transform_trajectory_angular_velocity: std::ptr::null_mut(),
            u_pose_search_trajectory_library_draw_transform_trajectory: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMultiAnimAsset::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetOrigin"),
                &raw mut __FUNCTION_PTRS.u_multi_anim_asset_bp_get_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetAnimationAsset"),
                &raw mut __FUNCTION_PTRS.u_multi_anim_asset_bp_get_animation_asset,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchTrajectoryPredictorInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Predict"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_predict,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVelocity"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGravity"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_gravity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentState"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_current_state,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionMatchingAnimNodeLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInterruptMode"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_interrupt_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDatabaseToSearch"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_database_to_search,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDatabasesToSearch"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_databases_to_search,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetDatabasesToSearch"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_reset_databases_to_search,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideMotionMatchingBlendSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_override_motion_matching_blend_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMotionMatchingSearchResult"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_search_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMotionMatchingBlendSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_blend_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMotionMatchingNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMotionMatchingNode"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionMatchingInteractionAnimNodeLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAvailabilities"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_set_availabilities,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInteracting"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_is_interacting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMotionMatchingInteractionNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToMotionMatchingInteractionNode"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchAssetSamplerLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SamplePose"),
                &raw mut __FUNCTION_PTRS.u_pose_search_asset_sampler_library_sample_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformByName"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_get_transform_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Draw"),
                &raw mut __FUNCTION_PTRS.u_pose_search_asset_sampler_library_draw,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchDatabase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumAnimationAssets"),
                &raw mut __FUNCTION_PTRS.u_pose_search_database_get_num_animation_assets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimationAsset"),
                &raw mut __FUNCTION_PTRS.u_pose_search_database_get_animation_asset,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchEventLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdatePoseSearchEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_event_library_update_pose_search_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_Curve::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetCurveValue"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_curve_bp_get_curve_value,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_Distance::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_distance_bp_get_distance,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_Heading::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetWorldRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_heading_bp_get_world_rotation,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_Position::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetWorldPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_position_bp_get_world_position,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_TimeToEvent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetTimeToEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_time_to_event_bp_get_time_to_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchFeatureChannel_Velocity::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetWorldVelocity"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_feature_channel_velocity_bp_get_world_velocity,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchHistoryCollectorAnimNodeLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPoseHistoryNodeTransformTrajectory"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_set_pose_history_node_transform_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPoseHistoryReference"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPoseHistoryNodeTransformTrajectory"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_node_transform_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToPoseHistoryNodePure"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToPoseHistoryNode"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchInteractionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MotionMatchInteraction_Pure"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MotionMatchInteraction"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMontageContinuingProperties"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_interaction_library_get_montage_continuing_properties,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MotionMatch"),
                &raw mut __FUNCTION_PTRS.u_pose_search_library_motion_match,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAnimationAssetLooping"),
                &raw mut __FUNCTION_PTRS.u_pose_search_library_is_animation_asset_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDatabaseTags"),
                &raw mut __FUNCTION_PTRS.u_pose_search_library_get_database_tags,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPoseSearchTrajectoryLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "PoseSearchGenerateTransformTrajectoryWithPredictor",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory_with_predictor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PoseSearchGenerateTransformTrajectory"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "PoseSearchGeneratePredictorTransformTrajectory",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_predictor_transform_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "HandleTransformTrajectoryWorldCollisionsWithGravity",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions_with_gravity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleTransformTrajectoryWorldCollisions"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformTrajectoryVelocity"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformTrajectorySampleTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformTrajectorySampleAtTime"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_at_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformTrajectoryAngularVelocity"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_angular_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DrawTransformTrajectory"),
                &raw mut __FUNCTION_PTRS
                    .u_pose_search_trajectory_library_draw_transform_trajectory,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FPoseSearchInteractionAvailability {
    pub database: UPtr<UPoseSearchDatabase>,
    pub tag: FName,
    pub roles_filter: TArray<FName>,
    pub broad_phase_radius: f32,
    pub broad_phase_radius_increment_on_interaction: f32,
    pub b_disable_collisions: bool,
    pub tick_priority: i32,
}
impl FPoseSearchInteractionAvailability {}
#[repr(C, align(8))]
pub struct FPoseSearchInteractionAvailabilityEx {
    pub(crate) __padding_end: [u8; 80],
}
impl FPoseSearchInteractionAvailabilityEx {}
#[repr(C, align(16))]
pub struct FAnimNode_MotionMatching {
    pub(crate) __padding_end: [u8; 1072],
}
impl FAnimNode_MotionMatching {}
#[repr(C, align(8))]
pub struct FPoseSearchBlueprintResult {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub selected_anim: UPtr<crate::bindings::core_u_object::UObject>,
    pub selected_time: f32,
    pub b_is_continuing_pose_search: bool,
    pub wanted_play_rate: f32,
    pub b_loop: bool,
    pub b_is_mirrored: bool,
    pub blend_parameters: crate::bindings::core_u_object::FVector,
    pub selected_database: UPtr<UPoseSearchDatabase>,
    pub search_cost: f32,
    pub b_is_interaction: bool,
    pub role: FName,
    pub actor_root_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub actor_root_bone_transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub(crate) __padding_end: [u8; 16],
}
impl FPoseSearchBlueprintResult {}
#[repr(C, align(4))]
pub struct FPoseSearchEvent {
    pub event_tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub time_to_event: f32,
    pub b_enable_pose_filters: bool,
    pub b_use_play_rate_range_override: bool,
    pub play_rate_range_override: crate::bindings::core_u_object::FFloatInterval,
}
impl FPoseSearchEvent {}
#[repr(C, align(16))]
pub struct FAnimNode_MotionMatchingInteraction {
    #[doc(hidden)]
    pub(crate) __padding_352: [u8; 352],
    pub blend_time: f32,
    pub blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
    pub blend_option: crate::bindings::engine::EAlphaBlendOption,
    pub(crate) __padding_end: [u8; 367],
}
impl FAnimNode_MotionMatchingInteraction {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSearchHistoryCollector_Base {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub pose_count: i32,
    pub sampling_interval: f32,
    #[doc(hidden)]
    pub(crate) __padding_180: [u8; 36],
    pub root_bone_recovery_time: f32,
    pub root_bone_translation_recovery_ratio: f32,
    pub root_bone_rotation_recovery_ratio: f32,
    #[doc(hidden)]
    pub(crate) __padding_268: [u8; 76],
    pub trajectory_history_count: i32,
    pub trajectory_prediction_count: i32,
    pub prediction_sampling_interval: f32,
    pub(crate) __padding_end: [u8; 608],
}
impl FAnimNode_PoseSearchHistoryCollector_Base {}
#[repr(C, align(8))]
pub struct FPoseSearchTrajectoryData {
    pub(crate) __padding_end: [u8; 296],
}
impl FPoseSearchTrajectoryData {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSearchHistoryCollector {
    #[doc(hidden)]
    pub(crate) __padding_888: [u8; 888],
    pub source: crate::bindings::engine::FPoseLink,
}
impl FAnimNode_PoseSearchHistoryCollector {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSearchComponentSpaceHistoryCollector {
    #[doc(hidden)]
    pub(crate) __padding_888: [u8; 888],
    pub source: crate::bindings::engine::FComponentSpacePoseLink,
}
impl FAnimNode_PoseSearchComponentSpaceHistoryCollector {}
#[repr(C, align(8))]
pub struct FMotionMatchingBlueprintBlendSettings {
    pub blend_time: f32,
    pub blend_profile: UPtr<crate::bindings::engine::UBlendProfile>,
    pub blend_option: crate::bindings::engine::EAlphaBlendOption,
    pub b_use_inertial_blend: bool,
}
impl FMotionMatchingBlueprintBlendSettings {}
#[repr(C, align(8))]
pub struct FMotionMatchingAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FMotionMatchingAnimNodeReference {}
#[repr(C, align(8))]
pub struct FMotionMatchingInteractionAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FMotionMatchingInteractionAnimNodeReference {}
#[repr(C, align(4))]
pub struct FPoseSearchIKWindowConstraint {
    pub(crate) __padding_end: [u8; 64],
}
impl FPoseSearchIKWindowConstraint {}
#[repr(C, align(16))]
pub struct FPoseSearchAssetSamplerInput {
    pub animation: UPtr<crate::bindings::engine::UAnimationAsset>,
    pub animation_time: f32,
    pub root_transform_origin: crate::bindings::core_u_object::FTransform,
    pub b_mirrored: bool,
    pub mirror_data_table: UPtr<crate::bindings::engine::UMirrorDataTable>,
    pub blend_parameters: crate::bindings::core_u_object::FVector,
    pub root_transform_sampling_rate: i32,
}
impl FPoseSearchAssetSamplerInput {}
#[repr(C, align(16))]
pub struct FPoseSearchAssetSamplerPose {
    pub(crate) __padding_end: [u8; 224],
}
impl FPoseSearchAssetSamplerPose {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseAnimationAsset {
    pub(crate) __padding_end: [u8; 64],
}
impl FPoseSearchDatabaseAnimationAsset {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseSequence {
    pub(crate) __padding_end: [u8; 40],
}
impl FPoseSearchDatabaseSequence {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseBlendSpace {
    pub(crate) __padding_end: [u8; 64],
}
impl FPoseSearchDatabaseBlendSpace {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseAnimComposite {
    pub(crate) __padding_end: [u8; 40],
}
impl FPoseSearchDatabaseAnimComposite {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseAnimMontage {
    pub(crate) __padding_end: [u8; 40],
}
impl FPoseSearchDatabaseAnimMontage {}
#[repr(C, align(8))]
pub struct FPoseSearchDatabaseMultiAnimAsset {
    pub(crate) __padding_end: [u8; 48],
}
impl FPoseSearchDatabaseMultiAnimAsset {}
#[repr(C, align(8))]
pub struct FPoseHistoryReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FPoseHistoryReference {}
#[repr(C, align(8))]
pub struct FPoseSearchHistoryCollectorAnimNodeReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FPoseSearchHistoryCollectorAnimNodeReference {}
#[repr(C, align(8))]
pub struct FPoseSearchFutureProperties {
    pub animation: UPtr<crate::bindings::core_u_object::UObject>,
    pub animation_time: f32,
    pub interval_time: f32,
}
impl FPoseSearchFutureProperties {}
#[repr(C, align(8))]
pub struct FPoseSearchContinuingProperties {
    pub playing_asset: UPtr<crate::bindings::core_u_object::UObject>,
    pub playing_asset_accumulated_time: f32,
    pub b_is_playing_asset_mirrored: bool,
    pub playing_asset_blend_parameters: crate::bindings::core_u_object::FVector,
    pub interrupt_mode: EPoseSearchInterruptMode,
    pub b_is_continuing_interaction: bool,
    pub playing_asset_database: UPtr<UPoseSearchDatabase>,
}
impl FPoseSearchContinuingProperties {}
#[repr(C, align(4))]
pub struct FPoseSearchTrajectory_WorldCollisionResults {
    pub time_to_land: f32,
    pub land_speed: f32,
}
impl FPoseSearchTrajectory_WorldCollisionResults {}
#[repr(C, align(16))]
pub struct FPoseSearchQueryTrajectorySample {
    pub(crate) __padding_end: [u8; 64],
}
impl FPoseSearchQueryTrajectorySample {}
#[repr(C, align(8))]
pub struct FPoseSearchQueryTrajectory {
    pub(crate) __padding_end: [u8; 16],
}
impl FPoseSearchQueryTrajectory {}
#[repr(C, align(8))]
pub struct UMultiAnimAsset {
    __padding_end: [u8; 48],
}
impl UMultiAnimAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiAnimAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiAnimAsset")
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
    pub fn bp_get_origin(
        &self,
        role: &FName,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_multi_anim_asset_bp_get_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(role, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_multi_anim_asset_bp_get_origin,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn bp_get_animation_asset(
        &self,
        role: &FName,
    ) -> UPtr<crate::bindings::engine::UAnimationAsset> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_multi_anim_asset_bp_get_animation_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(role, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_multi_anim_asset_bp_get_animation_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>()
                .read()
        }
    }
}
pub struct IPoseSearchTrajectoryPredictorInterface {}
#[repr(C, align(8))]
pub struct UPoseSearchTrajectoryPredictorInterface {
    __padding_end: [u8; 48],
}
impl UPoseSearchTrajectoryPredictorInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchTrajectoryPredictorInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchTrajectoryPredictorInterface")
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
    pub fn predict(
        &mut self,
        in_out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        num_prediction_samples: i32,
        seconds_per_prediction_sample: f32,
        num_history_samples: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_predict,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_trajectory,
                __buffer.add(0).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_prediction_samples,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seconds_per_prediction_sample,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_history_samples,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_predict,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(in_out_trajectory);
        }
    }
    pub fn get_velocity(
        &mut self,
        out_velocity: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_velocity);
        }
    }
    pub fn get_gravity(
        &mut self,
        out_gravity_accel: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_gravity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_gravity_accel,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_gravity,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_gravity_accel);
        }
    }
    pub fn get_current_state(
        &mut self,
        out_position: &mut crate::bindings::core_u_object::FVector,
        out_facing: &mut crate::bindings::core_u_object::FQuat,
        out_velocity: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_current_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_facing,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_velocity,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_predictor_interface_get_current_state,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_position);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FQuat>()
                .swap(out_facing);
        }
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_velocity);
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionMatchingAnimNodeLibrary {
    __padding_end: [u8; 48],
}
impl UMotionMatchingAnimNodeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionMatchingAnimNodeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionMatchingAnimNodeLibrary")
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
    pub fn set_interrupt_mode(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        interrupt_mode: EPoseSearchInterruptMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_interrupt_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interrupt_mode,
                __buffer.add(16).cast::<EPoseSearchInterruptMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_interrupt_mode,
                __buffer,
            )
        };
    }
    pub fn set_database_to_search(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        database: UPtr<UPoseSearchDatabase>,
        interrupt_mode: EPoseSearchInterruptMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_database_to_search,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &database,
                __buffer.add(16).cast::<UPtr<UPoseSearchDatabase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interrupt_mode,
                __buffer.add(24).cast::<EPoseSearchInterruptMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_database_to_search,
                __buffer,
            )
        };
    }
    pub fn set_databases_to_search(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        databases: &TArray<UPtr<UPoseSearchDatabase>>,
        interrupt_mode: EPoseSearchInterruptMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_databases_to_search,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                databases,
                __buffer.add(16).cast::<TArray<UPtr<UPoseSearchDatabase>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interrupt_mode,
                __buffer.add(32).cast::<EPoseSearchInterruptMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_set_databases_to_search,
                __buffer,
            )
        };
    }
    pub fn reset_databases_to_search(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        interrupt_mode: EPoseSearchInterruptMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_reset_databases_to_search,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interrupt_mode,
                __buffer.add(16).cast::<EPoseSearchInterruptMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_reset_databases_to_search,
                __buffer,
            )
        };
    }
    pub fn override_motion_matching_blend_settings(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        blend_settings: &FMotionMatchingBlueprintBlendSettings,
        b_is_result_valid: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_override_motion_matching_blend_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_settings,
                __buffer.add(16).cast::<FMotionMatchingBlueprintBlendSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_result_valid,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_override_motion_matching_blend_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<bool>().swap(b_is_result_valid);
        }
    }
    pub fn get_motion_matching_search_result(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        result: &mut FPoseSearchBlueprintResult,
        b_is_result_valid: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_search_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(16).cast::<FPoseSearchBlueprintResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_result_valid,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_search_result,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FPoseSearchBlueprintResult>().swap(result);
        }
        unsafe {
            __buffer.add(152).cast::<bool>().swap(b_is_result_valid);
        }
    }
    pub fn get_motion_matching_blend_settings(
        motion_matching_node: &FMotionMatchingAnimNodeReference,
        blend_settings: &mut FMotionMatchingBlueprintBlendSettings,
        b_is_result_valid: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_blend_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(0).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blend_settings,
                __buffer.add(16).cast::<FMotionMatchingBlueprintBlendSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_result_valid,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_get_motion_matching_blend_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FMotionMatchingBlueprintBlendSettings>()
                .swap(blend_settings);
        }
        unsafe {
            __buffer.add(40).cast::<bool>().swap(b_is_result_valid);
        }
    }
    pub fn convert_to_motion_matching_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        motion_matching_node: &mut FMotionMatchingAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_node,
                __buffer.add(16).cast::<FMotionMatchingAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FMotionMatchingAnimNodeReference>()
                .swap(motion_matching_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_motion_matching_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FMotionMatchingAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_anim_node_library_convert_to_motion_matching_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FMotionMatchingAnimNodeReference>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMotionMatchingInteractionAnimNodeLibrary {
    __padding_end: [u8; 48],
}
impl UMotionMatchingInteractionAnimNodeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionMatchingInteractionAnimNodeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionMatchingInteractionAnimNodeLibrary")
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
    pub fn set_availabilities(
        motion_matching_interaction_node: &FMotionMatchingInteractionAnimNodeReference,
        availabilities: &TArray<FPoseSearchInteractionAvailability>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_set_availabilities,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_interaction_node,
                __buffer.add(0).cast::<FMotionMatchingInteractionAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                availabilities,
                __buffer.add(16).cast::<TArray<FPoseSearchInteractionAvailability>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingInteractionAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_set_availabilities,
                __buffer,
            )
        };
    }
    pub fn is_interacting(
        motion_matching_interaction_node: &FMotionMatchingInteractionAnimNodeReference,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_is_interacting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_interaction_node,
                __buffer.add(0).cast::<FMotionMatchingInteractionAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingInteractionAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_is_interacting,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn convert_to_motion_matching_interaction_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        motion_matching_interaction_node: &mut FMotionMatchingInteractionAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                motion_matching_interaction_node,
                __buffer.add(16).cast::<FMotionMatchingInteractionAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingInteractionAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FMotionMatchingInteractionAnimNodeReference>()
                .swap(motion_matching_interaction_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_motion_matching_interaction_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FMotionMatchingInteractionAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UMotionMatchingInteractionAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_motion_matching_interaction_anim_node_library_convert_to_motion_matching_interaction_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer.add(24).cast::<FMotionMatchingInteractionAnimNodeReference>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimNotifyState_PoseSearchBase {
    __padding_end: [u8; 56],
}
impl UAnimNotifyState_PoseSearchBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBase")
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
pub struct UAnimNotifyState_PoseSearchExcludeFromDatabase {
    __padding_end: [u8; 56],
}
impl UAnimNotifyState_PoseSearchExcludeFromDatabase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchExcludeFromDatabase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchExcludeFromDatabase")
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
pub struct UAnimNotifyState_PoseSearchBlockTransition {
    __padding_end: [u8; 56],
}
impl UAnimNotifyState_PoseSearchBlockTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBlockTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBlockTransition")
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
pub struct UAnimNotifyState_PoseSearchModifyCost {
    __padding_end: [u8; 64],
}
impl UAnimNotifyState_PoseSearchModifyCost {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchModifyCost")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchModifyCost")
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
pub struct UAnimNotifyState_PoseSearchOverrideContinuingPoseCostBias {
    __padding_end: [u8; 64],
}
impl UAnimNotifyState_PoseSearchOverrideContinuingPoseCostBias {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchOverrideContinuingPoseCostBias")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchOverrideContinuingPoseCostBias")
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
pub struct UAnimNotifyState_PoseSearchSamplingEvent {
    __padding_end: [u8; 64],
}
impl UAnimNotifyState_PoseSearchSamplingEvent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchSamplingEvent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchSamplingEvent")
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
pub struct UAnimNotifyState_PoseSearchSamplingAttribute {
    __padding_end: [u8; 176],
}
impl UAnimNotifyState_PoseSearchSamplingAttribute {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchSamplingAttribute")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchSamplingAttribute")
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
pub struct UAnimNotifyState_PoseSearchBranchIn {
    __padding_end: [u8; 64],
}
impl UAnimNotifyState_PoseSearchBranchIn {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBranchIn")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchBranchIn")
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
pub struct UAnimNotifyState_PoseSearchIKWindow {
    __padding_end: [u8; 72],
}
impl UAnimNotifyState_PoseSearchIKWindow {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchIKWindow")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_PoseSearchIKWindow")
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
pub struct UAnimNotify_PoseSearchBase {
    __padding_end: [u8; 64],
}
impl UAnimNotify_PoseSearchBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PoseSearchBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PoseSearchBase")
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
pub struct UAnimNotify_PoseSearchEvent {
    __padding_end: [u8; 80],
}
impl UAnimNotify_PoseSearchEvent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PoseSearchEvent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PoseSearchEvent")
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
pub struct UPoseSearchAssetSamplerLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchAssetSamplerLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchAssetSamplerLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchAssetSamplerLibrary")
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
    pub fn sample_pose(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        input: FPoseSearchAssetSamplerInput,
    ) -> FPoseSearchAssetSamplerPose {
        let mut __stack = crate::core_data::StackAlloc::<400>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_sample_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input,
                __buffer.add(16).cast::<FPoseSearchAssetSamplerInput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchAssetSamplerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_sample_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(176).cast::<FPoseSearchAssetSamplerPose>().read() }
    }
    pub fn get_transform_by_name(
        asset_sampler_pose: &mut FPoseSearchAssetSamplerPose,
        bone_name: FName,
        space: EPoseSearchAssetSamplerSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_get_transform_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_sampler_pose,
                __buffer.add(0).cast::<FPoseSearchAssetSamplerPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(224).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(236).cast::<EPoseSearchAssetSamplerSpace>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchAssetSamplerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_get_transform_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<FPoseSearchAssetSamplerPose>()
                .swap(asset_sampler_pose);
        }
        unsafe {
            __buffer.add(240).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn draw(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        asset_sampler_pose: &mut FPoseSearchAssetSamplerPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_draw,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_sampler_pose,
                __buffer.add(16).cast::<FPoseSearchAssetSamplerPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchAssetSamplerLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_asset_sampler_library_draw,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FPoseSearchAssetSamplerPose>()
                .swap(asset_sampler_pose);
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchDatabase {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub schema: UPtr<UPoseSearchSchema>,
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 64],
    pub tags: TArray<FName>,
    pub normalization_set: UPtr<UPoseSearchNormalizationSet>,
    __padding_end: [u8; 472],
}
impl UPoseSearchDatabase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchDatabase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchDatabase")
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
    pub fn get_num_animation_assets(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_database_get_num_animation_assets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_database_get_num_animation_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_animation_asset(
        &self,
        index: i32,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_database_get_animation_asset,
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
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_database_get_animation_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchEventLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchEventLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchEventLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchEventLibrary")
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
    pub fn update_pose_search_event(
        in_new_event: &FPoseSearchEvent,
        b_is_new_event_valid: bool,
        delta_seconds: f32,
        in_out_current_event: &mut FPoseSearchEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_event_library_update_pose_search_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_event,
                __buffer.add(0).cast::<FPoseSearchEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_new_event_valid,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_current_event,
                __buffer.add(36).cast::<FPoseSearchEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchEventLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_event_library_update_pose_search_event,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(36).cast::<FPoseSearchEvent>().swap(in_out_current_event);
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel {
    __padding_end: [u8; 72],
}
impl UPoseSearchFeatureChannel {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel")
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
pub struct UPoseSearchFeatureChannel_Curve {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub curve_name: FName,
    __padding_end: [u8; 60],
}
impl UPoseSearchFeatureChannel_Curve {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Curve")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Curve")
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
    pub fn bp_get_curve_value(
        &self,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_curve_bp_get_curve_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_curve_bp_get_curve_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel_Distance {
    __padding_end: [u8; 200],
}
impl UPoseSearchFeatureChannel_Distance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Distance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Distance")
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
    pub fn bp_get_distance(
        &self,
        context: &mut crate::bindings::chooser::FChooserEvaluationContext,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<140>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_distance_bp_get_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::chooser::FChooserEvaluationContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_distance_bp_get_distance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::chooser::FChooserEvaluationContext>()
                .swap(context);
        }
        unsafe { __buffer.add(136).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel_FilterCrashingLegs {
    __padding_end: [u8; 184],
}
impl UPoseSearchFeatureChannel_FilterCrashingLegs {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_FilterCrashingLegs")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_FilterCrashingLegs")
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
pub struct UPoseSearchFeatureChannel_GroupBase {
    __padding_end: [u8; 88],
}
impl UPoseSearchFeatureChannel_GroupBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_GroupBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_GroupBase")
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
pub struct UPoseSearchFeatureChannel_Group {
    #[doc(hidden)]
    pub(crate) __padding_88: [u8; 88],
    pub sub_channels: TArray<UPtr<UPoseSearchFeatureChannel>>,
}
impl UPoseSearchFeatureChannel_Group {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Group")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Group")
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
pub struct UPoseSearchFeatureChannel_Heading {
    __padding_end: [u8; 200],
}
impl UPoseSearchFeatureChannel_Heading {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Heading")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Heading")
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
    pub fn bp_get_world_rotation(
        &self,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_heading_bp_get_world_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_heading_bp_get_world_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel_Padding {
    __padding_end: [u8; 80],
}
impl UPoseSearchFeatureChannel_Padding {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Padding")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Padding")
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
pub struct UPoseSearchFeatureChannel_PermutationTime {
    __padding_end: [u8; 80],
}
impl UPoseSearchFeatureChannel_PermutationTime {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_PermutationTime")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_PermutationTime")
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
pub struct UPoseSearchFeatureChannel_Phase {
    __padding_end: [u8; 144],
}
impl UPoseSearchFeatureChannel_Phase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Phase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Phase")
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
pub struct UPoseSearchFeatureChannel_Pose {
    __padding_end: [u8; 152],
}
impl UPoseSearchFeatureChannel_Pose {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Pose")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Pose")
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
pub struct UPoseSearchFeatureChannel_Position {
    __padding_end: [u8; 208],
}
impl UPoseSearchFeatureChannel_Position {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Position")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Position")
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
    pub fn bp_get_world_position(
        &self,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_position_bp_get_world_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_position_bp_get_world_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel_SamplingTime {
    __padding_end: [u8; 80],
}
impl UPoseSearchFeatureChannel_SamplingTime {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_SamplingTime")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_SamplingTime")
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
pub struct UPoseSearchFeatureChannel_TimeToEvent {
    __padding_end: [u8; 88],
}
impl UPoseSearchFeatureChannel_TimeToEvent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_TimeToEvent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_TimeToEvent")
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
    pub fn bp_get_time_to_event(
        &self,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_time_to_event_bp_get_time_to_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_time_to_event_bp_get_time_to_event,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchFeatureChannel_Trajectory {
    __padding_end: [u8; 128],
}
impl UPoseSearchFeatureChannel_Trajectory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Trajectory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Trajectory")
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
pub struct UPoseSearchFeatureChannel_Velocity {
    __padding_end: [u8; 200],
}
impl UPoseSearchFeatureChannel_Velocity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Velocity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchFeatureChannel_Velocity")
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
    pub fn bp_get_world_velocity(
        &self,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_velocity_bp_get_world_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_feature_channel_velocity_bp_get_world_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchHistoryCollectorAnimNodeLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchHistoryCollectorAnimNodeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchHistoryCollectorAnimNodeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchHistoryCollectorAnimNodeLibrary")
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
    pub fn set_pose_history_node_transform_trajectory(
        pose_search_history_collector_node: &FPoseSearchHistoryCollectorAnimNodeReference,
        trajectory: &crate::bindings::engine::FTransformTrajectory,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_set_pose_history_node_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose_search_history_collector_node,
                __buffer.add(0).cast::<FPoseSearchHistoryCollectorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                trajectory,
                __buffer.add(16).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchHistoryCollectorAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_set_pose_history_node_transform_trajectory,
                __buffer,
            )
        };
    }
    pub fn get_pose_history_reference(
        node: &FPoseSearchHistoryCollectorAnimNodeReference,
    ) -> FPoseHistoryReference {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FPoseSearchHistoryCollectorAnimNodeReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchHistoryCollectorAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_reference,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FPoseHistoryReference>().read() }
    }
    pub fn get_pose_history_node_transform_trajectory(
        pose_search_history_collector_node: &FPoseSearchHistoryCollectorAnimNodeReference,
        trajectory: &mut crate::bindings::engine::FTransformTrajectory,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_node_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose_search_history_collector_node,
                __buffer.add(0).cast::<FPoseSearchHistoryCollectorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                trajectory,
                __buffer.add(16).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchHistoryCollectorAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_get_pose_history_node_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(trajectory);
        }
    }
    pub fn convert_to_pose_history_node_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        pose_search_history_collector_node: &mut FPoseSearchHistoryCollectorAnimNodeReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose_search_history_collector_node,
                __buffer.add(16).cast::<FPoseSearchHistoryCollectorAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchHistoryCollectorAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<FPoseSearchHistoryCollectorAnimNodeReference>()
                .swap(pose_search_history_collector_node);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_pose_history_node(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FPoseSearchHistoryCollectorAnimNodeReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNodeReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::engine::EAnimNodeReferenceConversionResult,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchHistoryCollectorAnimNodeLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_history_collector_anim_node_library_convert_to_pose_history_node,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<FPoseSearchHistoryCollectorAnimNodeReference>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchInteractionAsset {
    __padding_end: [u8; 96],
}
impl UPoseSearchInteractionAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionAsset")
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
pub struct UPoseSearchInteractionLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchInteractionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionLibrary")
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
    pub fn motion_match_interaction_pure(
        availabilities: TArray<FPoseSearchInteractionAvailability>,
        anim_context: UPtr<crate::bindings::core_u_object::UObject>,
        pose_history_name: FName,
        b_validate_result_against_availabilities: bool,
    ) -> FPoseSearchBlueprintResult {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction_pure,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &availabilities,
                __buffer.add(0).cast::<TArray<FPoseSearchInteractionAvailability>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_context,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pose_history_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_validate_result_against_availabilities,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchInteractionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction_pure,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FPoseSearchBlueprintResult>().read() }
    }
    pub fn motion_match_interaction(
        availabilities: TArray<FPoseSearchInteractionAvailability>,
        anim_context: UPtr<crate::bindings::core_u_object::UObject>,
        pose_history_name: FName,
        b_validate_result_against_availabilities: bool,
    ) -> FPoseSearchBlueprintResult {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &availabilities,
                __buffer.add(0).cast::<TArray<FPoseSearchInteractionAvailability>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_context,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pose_history_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_validate_result_against_availabilities,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchInteractionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_motion_match_interaction,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FPoseSearchBlueprintResult>().read() }
    }
    pub fn get_montage_continuing_properties(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
    ) -> FPoseSearchContinuingProperties {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_get_montage_continuing_properties,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchInteractionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_interaction_library_get_montage_continuing_properties,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FPoseSearchContinuingProperties>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchInteractionSubsystem {
    __padding_end: [u8; 136],
}
impl UPoseSearchInteractionSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchInteractionSubsystem")
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
pub struct UPoseSearchLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchLibrary")
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
    pub fn motion_match(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        assets_to_search: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        pose_history_name: FName,
        continuing_properties: FPoseSearchContinuingProperties,
        future: FPoseSearchFutureProperties,
        result: &mut FPoseSearchBlueprintResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_motion_match,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &assets_to_search,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pose_history_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &continuing_properties,
                __buffer.add(40).cast::<FPoseSearchContinuingProperties>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &future,
                __buffer.add(96).cast::<FPoseSearchFutureProperties>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result,
                __buffer.add(112).cast::<FPoseSearchBlueprintResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_motion_match,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<FPoseSearchBlueprintResult>().swap(result);
        }
    }
    pub fn is_animation_asset_looping(
        asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_asset_looping: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_is_animation_asset_looping,
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
                b_is_asset_looping,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_is_animation_asset_looping,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_is_asset_looping);
        }
    }
    pub fn get_database_tags(
        database: UPtr<UPoseSearchDatabase>,
        tags: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_get_database_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &database,
                __buffer.add(0).cast::<UPtr<UPoseSearchDatabase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_library_get_database_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(tags);
        }
    }
}
#[repr(C, align(8))]
pub struct UPoseSearchNormalizationSet {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub databases: TArray<UPtr<UPoseSearchDatabase>>,
}
impl UPoseSearchNormalizationSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchNormalizationSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchNormalizationSet")
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
pub struct UPoseSearchSchema {
    __padding_end: [u8; 136],
}
impl UPoseSearchSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchSchema")
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
pub struct UPoseSearchTrajectoryLibrary {
    __padding_end: [u8; 48],
}
impl UPoseSearchTrajectoryLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchTrajectoryLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseSearchTrajectoryLibrary")
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
    pub fn pose_search_generate_transform_trajectory_with_predictor(
        in_predictor: TScriptInterface<UPoseSearchTrajectoryPredictorInterface>,
        in_delta_time: f32,
        in_out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_out_desired_controller_yaw_last_update: &mut f32,
        out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_history_sampling_interval: f32,
        in_trajectory_history_count: i32,
        in_prediction_sampling_interval: f32,
        in_trajectory_prediction_count: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory_with_predictor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_predictor,
                __buffer
                    .add(0)
                    .cast::<TScriptInterface<UPoseSearchTrajectoryPredictorInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_delta_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_trajectory,
                __buffer.add(24).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_desired_controller_yaw_last_update,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory,
                __buffer.add(48).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_history_sampling_interval,
                __buffer.add(64).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_history_count,
                __buffer.add(68).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_prediction_sampling_interval,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_prediction_count,
                __buffer.add(76).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory_with_predictor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(in_out_trajectory);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<f32>()
                .swap(in_out_desired_controller_yaw_last_update);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(out_trajectory);
        }
    }
    pub fn pose_search_generate_transform_trajectory(
        in_anim_instance: UPtr<crate::bindings::core_u_object::UObject>,
        in_trajectory_data: &FPoseSearchTrajectoryData,
        in_delta_time: f32,
        in_out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_out_desired_controller_yaw_last_update: &mut f32,
        out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_history_sampling_interval: f32,
        in_trajectory_history_count: i32,
        in_prediction_sampling_interval: f32,
        in_trajectory_prediction_count: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory_data,
                __buffer.add(8).cast::<FPoseSearchTrajectoryData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_delta_time,
                __buffer.add(304).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_trajectory,
                __buffer
                    .add(312)
                    .cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_desired_controller_yaw_last_update,
                __buffer.add(328).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory,
                __buffer
                    .add(336)
                    .cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_history_sampling_interval,
                __buffer.add(352).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_history_count,
                __buffer.add(356).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_prediction_sampling_interval,
                __buffer.add(360).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_prediction_count,
                __buffer.add(364).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(312)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(in_out_trajectory);
        }
        unsafe {
            __buffer
                .add(328)
                .cast::<f32>()
                .swap(in_out_desired_controller_yaw_last_update);
        }
        unsafe {
            __buffer
                .add(336)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(out_trajectory);
        }
    }
    pub fn pose_search_generate_predictor_transform_trajectory(
        in_predictor: UPtr<crate::bindings::core_u_object::UObject>,
        in_trajectory_data: &FPoseSearchTrajectoryData,
        in_delta_time: f32,
        in_out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_out_desired_controller_yaw_last_update: &mut f32,
        out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        in_history_sampling_interval: f32,
        in_trajectory_history_count: i32,
        in_prediction_sampling_interval: f32,
        in_trajectory_prediction_count: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_predictor_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_predictor,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory_data,
                __buffer.add(8).cast::<FPoseSearchTrajectoryData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_delta_time,
                __buffer.add(304).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_trajectory,
                __buffer
                    .add(312)
                    .cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_desired_controller_yaw_last_update,
                __buffer.add(328).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory,
                __buffer
                    .add(336)
                    .cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_history_sampling_interval,
                __buffer.add(352).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_history_count,
                __buffer.add(356).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_prediction_sampling_interval,
                __buffer.add(360).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_trajectory_prediction_count,
                __buffer.add(364).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_pose_search_generate_predictor_transform_trajectory,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(312)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(in_out_trajectory);
        }
        unsafe {
            __buffer
                .add(328)
                .cast::<f32>()
                .swap(in_out_desired_controller_yaw_last_update);
        }
        unsafe {
            __buffer
                .add(336)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(out_trajectory);
        }
    }
    pub fn handle_transform_trajectory_world_collisions_with_gravity(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        starting_velocity: crate::bindings::core_u_object::FVector,
        b_apply_gravity: bool,
        gravity_accel: crate::bindings::core_u_object::FVector,
        floor_collisions_offset: f32,
        out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        collision_result: &mut FPoseSearchTrajectory_WorldCollisionResults,
        trace_channel: crate::bindings::engine::ETraceTypeQuery,
        b_trace_complex: bool,
        actors_to_ignore: &TArray<UPtr<crate::bindings::engine::AActor>>,
        draw_debug_type: crate::bindings::engine::EDrawDebugTrace,
        b_ignore_self: bool,
        max_obstacle_height: f32,
        trace_color: crate::bindings::core_u_object::FLinearColor,
        trace_hit_color: crate::bindings::core_u_object::FLinearColor,
        draw_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<180>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions_with_gravity,
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
                in_trajectory,
                __buffer.add(8).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &starting_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_gravity,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gravity_accel,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &floor_collisions_offset,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory,
                __buffer.add(88).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                collision_result,
                __buffer.add(104).cast::<FPoseSearchTrajectory_WorldCollisionResults>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_channel,
                __buffer.add(112).cast::<crate::bindings::engine::ETraceTypeQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_trace_complex,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_ignore,
                __buffer
                    .add(120)
                    .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_debug_type,
                __buffer.add(136).cast::<crate::bindings::engine::EDrawDebugTrace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_ignore_self,
                __buffer.add(137).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_obstacle_height,
                __buffer.add(140).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_color,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_hit_color,
                __buffer.add(160).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_time,
                __buffer.add(176).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions_with_gravity,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(out_trajectory);
        }
        unsafe {
            __buffer
                .add(104)
                .cast::<FPoseSearchTrajectory_WorldCollisionResults>()
                .swap(collision_result);
        }
    }
    pub fn handle_transform_trajectory_world_collisions(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        b_apply_gravity: bool,
        floor_collisions_offset: f32,
        out_trajectory: &mut crate::bindings::engine::FTransformTrajectory,
        collision_result: &mut FPoseSearchTrajectory_WorldCollisionResults,
        trace_channel: crate::bindings::engine::ETraceTypeQuery,
        b_trace_complex: bool,
        actors_to_ignore: &TArray<UPtr<crate::bindings::engine::AActor>>,
        draw_debug_type: crate::bindings::engine::EDrawDebugTrace,
        b_ignore_self: bool,
        max_obstacle_height: f32,
        trace_color: crate::bindings::core_u_object::FLinearColor,
        trace_hit_color: crate::bindings::core_u_object::FLinearColor,
        draw_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<132>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions,
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
                &anim_instance,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory,
                __buffer.add(16).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_gravity,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &floor_collisions_offset,
                __buffer.add(36).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory,
                __buffer.add(40).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                collision_result,
                __buffer.add(56).cast::<FPoseSearchTrajectory_WorldCollisionResults>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_channel,
                __buffer.add(64).cast::<crate::bindings::engine::ETraceTypeQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_trace_complex,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_ignore,
                __buffer.add(72).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_debug_type,
                __buffer.add(88).cast::<crate::bindings::engine::EDrawDebugTrace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_ignore_self,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_obstacle_height,
                __buffer.add(92).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_color,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trace_hit_color,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &draw_time,
                __buffer.add(128).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_handle_transform_trajectory_world_collisions,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::engine::FTransformTrajectory>()
                .swap(out_trajectory);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<FPoseSearchTrajectory_WorldCollisionResults>()
                .swap(collision_result);
        }
    }
    pub fn get_transform_trajectory_velocity(
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        time1: f32,
        time2: f32,
        out_velocity: &mut crate::bindings::core_u_object::FVector,
        b_extrapolate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory,
                __buffer.add(0).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time1, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time2, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extrapolate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_velocity);
        }
    }
    pub fn get_transform_trajectory_sample_transform(
        in_trajectory_sample: &crate::bindings::engine::FTransformTrajectorySample,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory_sample,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::FTransformTrajectorySample>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_transform_trajectory_sample_at_time(
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        time: f32,
        out_trajectory_sample: &mut crate::bindings::engine::FTransformTrajectorySample,
        b_extrapolate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory,
                __buffer.add(0).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_trajectory_sample,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::engine::FTransformTrajectorySample>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extrapolate,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_sample_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::engine::FTransformTrajectorySample>()
                .swap(out_trajectory_sample);
        }
    }
    pub fn get_transform_trajectory_angular_velocity(
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        time1: f32,
        time2: f32,
        out_angular_velocity: &mut crate::bindings::core_u_object::FVector,
        b_extrapolate: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trajectory,
                __buffer.add(0).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time1, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time2, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_angular_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extrapolate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_get_transform_trajectory_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_angular_velocity);
        }
    }
    pub fn draw_transform_trajectory(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_trajectory: &crate::bindings::engine::FTransformTrajectory,
        debug_thickness: f32,
        height_offset: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_draw_transform_trajectory,
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
                in_trajectory,
                __buffer.add(8).cast::<crate::bindings::engine::FTransformTrajectory>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &debug_thickness,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &height_offset,
                __buffer.add(28).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::pose_search::UPoseSearchTrajectoryLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::pose_search::__FUNCTION_PTRS
                    .u_pose_search_trajectory_library_draw_transform_trajectory,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EPoseSearchMirrorOption(pub i32);
impl EPoseSearchMirrorOption {
    pub const UNMIRRORED_ONLY: EPoseSearchMirrorOption = EPoseSearchMirrorOption(0);
    pub const MIRRORED_ONLY: EPoseSearchMirrorOption = EPoseSearchMirrorOption(1);
    pub const UNMIRRORED_AND_MIRRORED: EPoseSearchMirrorOption = EPoseSearchMirrorOption(
        2,
    );
}
#[repr(transparent)]
pub struct EPoseSearchInterruptMode(pub u8);
impl EPoseSearchInterruptMode {
    pub const DO_NOT_INTERRUPT: EPoseSearchInterruptMode = EPoseSearchInterruptMode(0);
    pub const INTERRUPT_ON_DATABASE_CHANGE: EPoseSearchInterruptMode = EPoseSearchInterruptMode(
        1,
    );
    pub const INTERRUPT_ON_DATABASE_CHANGE_AND_INVALIDATE_CONTINUING_POSE: EPoseSearchInterruptMode = EPoseSearchInterruptMode(
        2,
    );
    pub const FORCE_INTERRUPT: EPoseSearchInterruptMode = EPoseSearchInterruptMode(3);
    pub const FORCE_INTERRUPT_AND_INVALIDATE_CONTINUING_POSE: EPoseSearchInterruptMode = EPoseSearchInterruptMode(
        4,
    );
}
#[repr(transparent)]
pub struct EPoseSearchAssetSamplerSpace(pub u8);
impl EPoseSearchAssetSamplerSpace {
    pub const LOCAL: EPoseSearchAssetSamplerSpace = EPoseSearchAssetSamplerSpace(0);
    pub const COMPONENT: EPoseSearchAssetSamplerSpace = EPoseSearchAssetSamplerSpace(1);
    pub const WORLD: EPoseSearchAssetSamplerSpace = EPoseSearchAssetSamplerSpace(2);
}
#[repr(transparent)]
pub struct EPoseSearchMode(pub i32);
impl EPoseSearchMode {
    pub const BRUTE_FORCE: EPoseSearchMode = EPoseSearchMode(0);
    pub const PCAKD_TREE: EPoseSearchMode = EPoseSearchMode(1);
    pub const VP_TREE: EPoseSearchMode = EPoseSearchMode(2);
    pub const EVENT_ONLY: EPoseSearchMode = EPoseSearchMode(3);
}
#[repr(transparent)]
pub struct EInputQueryPose(pub u8);
impl EInputQueryPose {
    pub const USE_CHARACTER_POSE: EInputQueryPose = EInputQueryPose(0);
    pub const USE_CONTINUING_POSE: EInputQueryPose = EInputQueryPose(1);
}
#[repr(transparent)]
pub struct EPermutationTimeType(pub u8);
impl EPermutationTimeType {
    pub const USE_SAMPLE_TIME: EPermutationTimeType = EPermutationTimeType(0);
    pub const USE_PERMUTATION_TIME: EPermutationTimeType = EPermutationTimeType(1);
    pub const USE_SAMPLE_TO_PERMUTATION_TIME: EPermutationTimeType = EPermutationTimeType(
        2,
    );
}
#[repr(transparent)]
pub struct EHeadingAxis(pub u8);
impl EHeadingAxis {
    pub const X: EHeadingAxis = EHeadingAxis(0);
    pub const Y: EHeadingAxis = EHeadingAxis(1);
    pub const Z: EHeadingAxis = EHeadingAxis(2);
    pub const NUM: EHeadingAxis = EHeadingAxis(3);
}
#[repr(transparent)]
pub struct EComponentStrippingVector(pub u8);
impl EComponentStrippingVector {
    pub const NONE: EComponentStrippingVector = EComponentStrippingVector(0);
    pub const STRIP_XY: EComponentStrippingVector = EComponentStrippingVector(1);
    pub const STRIP_Z: EComponentStrippingVector = EComponentStrippingVector(2);
}
#[repr(transparent)]
pub struct EPoseSearchDataPreprocessor(pub i32);
impl EPoseSearchDataPreprocessor {
    pub const NONE: EPoseSearchDataPreprocessor = EPoseSearchDataPreprocessor(0);
    pub const NORMALIZE: EPoseSearchDataPreprocessor = EPoseSearchDataPreprocessor(1);
    pub const NORMALIZE_ONLY_BY_DEVIATION: EPoseSearchDataPreprocessor = EPoseSearchDataPreprocessor(
        2,
    );
    pub const NORMALIZE_WITH_COMMON_SCHEMA: EPoseSearchDataPreprocessor = EPoseSearchDataPreprocessor(
        3,
    );
}
