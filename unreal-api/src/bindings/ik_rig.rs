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
    pub uik_goal_creator_interface_add_ik_goals: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_component_set_ik_rig_goal_transform: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_component_set_ik_rig_goal_position_and_rotation: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_component_set_ik_rig_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_component_clear_all_goals: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_root_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_global_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_chain_speed_plant_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_chain_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_chain_ik_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_set_chain_fk_settings_in_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_has_target_ik_rig: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_has_source_ik_rig: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_root_settings_from_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_root_settings_from_retarget_asset: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_global_settings_from_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_global_settings_from_retarget_asset: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_chain_using_goal_from_retarget_asset: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_chain_settings_from_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_get_chain_settings_from_retarget_asset: *mut crate::ffi::UFunctionOpague,
    pub u_retarget_profile_library_get_op_controller_from_retarget_profile: *mut crate::ffi::UFunctionOpague,
    pub u_retarget_profile_library_copy_retarget_profile_from_retarget_asset: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_align_pole_vector_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_align_pole_vector_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_set_copy_from_start: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_reset_bones_to_exclude: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_get_copy_from_start: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_get_bones_to_exclude: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_copy_base_pose_controller_add_bone_to_exclude: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_curve_remap_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_curve_remap_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_filter_bone_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_filter_bone_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_filter_bone_controller_get_all_bones_to_filter: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_filter_bone_controller_clear_bones_to_filter: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_filter_bone_controller_add_bone_to_filter: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_fk_chains_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_fk_chains_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_floor_goals_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_floor_goals_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_ik_chains_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_ik_chains_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_set_target_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_set_source_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_get_target_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_get_source_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pelvis_motion_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pin_bone_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pin_bone_controller_set_bone_pair: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pin_bone_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pin_bone_controller_get_all_bone_pairs: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_pin_bone_controller_clear_all_bone_pairs: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_additive_pose_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_additive_pose_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_set_target_root_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_set_target_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_set_source_root_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_get_target_root_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_get_target_pelvis_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_get_source_root_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_root_motion_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_run_ik_rig_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_run_ik_rig_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_scale_source_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_scale_source_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_speed_planting_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_speed_planting_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_stretch_chain_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_stretch_chain_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_stride_warping_controller_set_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retarget_stride_warping_controller_get_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_solver_controller_base_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_solver_controller_base_get_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_body_mover_controller_set_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_body_mover_controller_set_goal_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_body_mover_controller_get_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_body_mover_controller_get_goal_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_set_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_set_goal_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_set_bone_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_get_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_get_goal_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_controller_get_bone_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_solver_get_effectors: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_fbik_solver_get_bone_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_limb_solver_controller_set_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_limb_solver_controller_get_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_pole_solver_controller_set_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_pole_solver_controller_get_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_set_transform_controller_set_solver_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_set_transform_controller_get_solver_settings: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            uik_goal_creator_interface_add_ik_goals: std::ptr::null_mut(),
            uik_rig_component_set_ik_rig_goal_transform: std::ptr::null_mut(),
            uik_rig_component_set_ik_rig_goal_position_and_rotation: std::ptr::null_mut(),
            uik_rig_component_set_ik_rig_goal: std::ptr::null_mut(),
            uik_rig_component_clear_all_goals: std::ptr::null_mut(),
            uik_retargeter_set_root_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_set_global_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_set_chain_speed_plant_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_set_chain_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_set_chain_ik_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_set_chain_fk_settings_in_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_has_target_ik_rig: std::ptr::null_mut(),
            uik_retargeter_has_source_ik_rig: std::ptr::null_mut(),
            uik_retargeter_get_root_settings_from_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_get_root_settings_from_retarget_asset: std::ptr::null_mut(),
            uik_retargeter_get_global_settings_from_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_get_global_settings_from_retarget_asset: std::ptr::null_mut(),
            uik_retargeter_get_chain_using_goal_from_retarget_asset: std::ptr::null_mut(),
            uik_retargeter_get_chain_settings_from_retarget_profile: std::ptr::null_mut(),
            uik_retargeter_get_chain_settings_from_retarget_asset: std::ptr::null_mut(),
            u_retarget_profile_library_get_op_controller_from_retarget_profile: std::ptr::null_mut(),
            u_retarget_profile_library_copy_retarget_profile_from_retarget_asset: std::ptr::null_mut(),
            uik_retarget_align_pole_vector_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_align_pole_vector_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_set_copy_from_start: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_reset_bones_to_exclude: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_get_copy_from_start: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_get_bones_to_exclude: std::ptr::null_mut(),
            uik_retarget_copy_base_pose_controller_add_bone_to_exclude: std::ptr::null_mut(),
            uik_retarget_curve_remap_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_curve_remap_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_filter_bone_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_filter_bone_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_filter_bone_controller_get_all_bones_to_filter: std::ptr::null_mut(),
            uik_retarget_filter_bone_controller_clear_bones_to_filter: std::ptr::null_mut(),
            uik_retarget_filter_bone_controller_add_bone_to_filter: std::ptr::null_mut(),
            uik_retarget_fk_chains_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_fk_chains_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_floor_goals_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_floor_goals_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_ik_chains_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_ik_chains_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_set_target_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_set_source_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_get_target_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_get_source_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_pelvis_motion_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_pin_bone_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_pin_bone_controller_set_bone_pair: std::ptr::null_mut(),
            uik_retarget_pin_bone_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_pin_bone_controller_get_all_bone_pairs: std::ptr::null_mut(),
            uik_retarget_pin_bone_controller_clear_all_bone_pairs: std::ptr::null_mut(),
            uik_retarget_additive_pose_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_additive_pose_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_set_target_root_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_set_target_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_set_source_root_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_get_target_root_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_get_target_pelvis_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_get_source_root_bone: std::ptr::null_mut(),
            uik_retarget_root_motion_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_run_ik_rig_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_run_ik_rig_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_scale_source_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_scale_source_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_speed_planting_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_speed_planting_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_stretch_chain_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_stretch_chain_controller_get_settings: std::ptr::null_mut(),
            uik_retarget_stride_warping_controller_set_settings: std::ptr::null_mut(),
            uik_retarget_stride_warping_controller_get_settings: std::ptr::null_mut(),
            uik_rig_solver_controller_base_set_enabled: std::ptr::null_mut(),
            uik_rig_solver_controller_base_get_enabled: std::ptr::null_mut(),
            uik_rig_body_mover_controller_set_solver_settings: std::ptr::null_mut(),
            uik_rig_body_mover_controller_set_goal_settings: std::ptr::null_mut(),
            uik_rig_body_mover_controller_get_solver_settings: std::ptr::null_mut(),
            uik_rig_body_mover_controller_get_goal_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_set_solver_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_set_goal_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_set_bone_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_get_solver_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_get_goal_settings: std::ptr::null_mut(),
            uik_rig_fbik_controller_get_bone_settings: std::ptr::null_mut(),
            uik_rig_fbik_solver_get_effectors: std::ptr::null_mut(),
            uik_rig_fbik_solver_get_bone_settings: std::ptr::null_mut(),
            uik_rig_limb_solver_controller_set_solver_settings: std::ptr::null_mut(),
            uik_rig_limb_solver_controller_get_solver_settings: std::ptr::null_mut(),
            uik_rig_pole_solver_controller_set_solver_settings: std::ptr::null_mut(),
            uik_rig_pole_solver_controller_get_solver_settings: std::ptr::null_mut(),
            uik_rig_set_transform_controller_set_solver_settings: std::ptr::null_mut(),
            uik_rig_set_transform_controller_get_solver_settings: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKGoalCreatorInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddIKGoals"),
            &raw mut __FUNCTION_PTRS.uik_goal_creator_interface_add_ik_goals,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIKRigGoalTransform"),
            &raw mut __FUNCTION_PTRS.uik_rig_component_set_ik_rig_goal_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIKRigGoalPositionAndRotation"),
            &raw mut __FUNCTION_PTRS
                .uik_rig_component_set_ik_rig_goal_position_and_rotation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIKRigGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_component_set_ik_rig_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllGoals"),
            &raw mut __FUNCTION_PTRS.uik_rig_component_clear_all_goals,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargeter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_set_root_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_set_global_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChainSpeedPlantSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_set_chain_speed_plant_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChainSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_set_chain_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChainIKSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_set_chain_ik_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetChainFKSettingsInRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_set_chain_fk_settings_in_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasTargetIKRig"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_has_target_ik_rig,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasSourceIKRig"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_has_source_ik_rig,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSettingsFromRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_root_settings_from_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSettingsFromRetargetAsset"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_get_root_settings_from_retarget_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalSettingsFromRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_global_settings_from_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalSettingsFromRetargetAsset"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_global_settings_from_retarget_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChainUsingGoalFromRetargetAsset"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_chain_using_goal_from_retarget_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChainSettingsFromRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_chain_settings_from_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChainSettingsFromRetargetAsset"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_get_chain_settings_from_retarget_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URetargetProfileLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOpControllerFromRetargetProfile"),
            &raw mut __FUNCTION_PTRS
                .u_retarget_profile_library_get_op_controller_from_retarget_profile,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyRetargetProfileFromRetargetAsset"),
            &raw mut __FUNCTION_PTRS
                .u_retarget_profile_library_copy_retarget_profile_from_retarget_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetAlignPoleVectorController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_align_pole_vector_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_align_pole_vector_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetCopyBasePoseController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_copy_base_pose_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCopyFromStart"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_copy_base_pose_controller_set_copy_from_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetBonesToExclude"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_copy_base_pose_controller_reset_bones_to_exclude,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_copy_base_pose_controller_get_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCopyFromStart"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_copy_base_pose_controller_get_copy_from_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonesToExclude"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_copy_base_pose_controller_get_bones_to_exclude,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBoneToExclude"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_copy_base_pose_controller_add_bone_to_exclude,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetCurveRemapController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_curve_remap_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_curve_remap_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetFilterBoneController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_filter_bone_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_filter_bone_controller_get_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllBonesToFilter"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_filter_bone_controller_get_all_bones_to_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearBonesToFilter"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_filter_bone_controller_clear_bones_to_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBoneToFilter"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_filter_bone_controller_add_bone_to_filter,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetFKChainsController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_fk_chains_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_fk_chains_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetFloorGoalsController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_floor_goals_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_floor_goals_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetIKChainsController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_ik_chains_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_ik_chains_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetPelvisMotionController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetPelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_pelvis_motion_controller_set_target_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourcePelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_pelvis_motion_controller_set_source_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pelvis_motion_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetPelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_pelvis_motion_controller_get_target_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourcePelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_pelvis_motion_controller_get_source_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pelvis_motion_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetPinBoneController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pin_bone_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBonePair"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pin_bone_controller_set_bone_pair,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pin_bone_controller_get_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllBonePairs"),
            &raw mut __FUNCTION_PTRS.uik_retarget_pin_bone_controller_get_all_bone_pairs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAllBonePairs"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_pin_bone_controller_clear_all_bone_pairs,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetAdditivePoseController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_additive_pose_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_additive_pose_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetRootMotionController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetRootBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_set_target_root_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetPelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_set_target_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceRootBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_set_source_root_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_root_motion_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetRootBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_get_target_root_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetPelvisBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_get_target_pelvis_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceRootBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retarget_root_motion_controller_get_source_root_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_root_motion_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetRunIKRigController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_run_ik_rig_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_run_ik_rig_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetScaleSourceController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_scale_source_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_scale_source_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetSpeedPlantingController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_speed_planting_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_speed_planting_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetStretchChainController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_stretch_chain_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_stretch_chain_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetStrideWarpingController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_stride_warping_controller_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettings"),
            &raw mut __FUNCTION_PTRS.uik_retarget_stride_warping_controller_get_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigSolverControllerBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnabled"),
            &raw mut __FUNCTION_PTRS.uik_rig_solver_controller_base_set_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnabled"),
            &raw mut __FUNCTION_PTRS.uik_rig_solver_controller_base_get_enabled,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigBodyMoverController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_body_mover_controller_set_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGoalSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_body_mover_controller_set_goal_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_body_mover_controller_get_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGoalSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_body_mover_controller_get_goal_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigFBIKController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_set_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGoalSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_set_goal_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoneSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_set_bone_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_get_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGoalSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_get_goal_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_controller_get_bone_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigFBIKSolver::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEffectors"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_solver_get_effectors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_fbik_solver_get_bone_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigLimbSolverController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_limb_solver_controller_set_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_limb_solver_controller_get_solver_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigPoleSolverController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_pole_solver_controller_set_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_pole_solver_controller_get_solver_settings,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigSetTransformController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_set_transform_controller_set_solver_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_set_transform_controller_get_solver_settings,
        );
    }
}
#[repr(C, align(8))]
pub struct FAnimNode_IKRig {
    #[doc(hidden)]
    pub(crate) __padding_248: [u8; 248],
    pub goals: TArray<FIKRigGoal>,
    #[doc(hidden)]
    pub(crate) __padding_272: [u8; 8],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub(crate) __padding_end: [u8; 516],
}
impl FAnimNode_IKRig {}
#[repr(C, align(16))]
pub struct FIKRigGoal {
    pub name: FName,
    pub bone_name: FName,
    pub transform_source: EIKRigGoalTransformSource,
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 16],
    pub position: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub position_space: EIKRigGoalSpace,
    pub rotation_space: EIKRigGoalSpace,
    pub(crate) __padding_end: [u8; 86],
}
impl FIKRigGoal {}
#[repr(C, align(8))]
pub struct FAnimNode_RetargetPoseFromMesh {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub retarget_from: ERetargetSourceMode,
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub ik_retargeter_asset: UPtr<UIKRetargeter>,
    pub custom_retarget_profile: FRetargetProfile,
    pub lod_threshold: i32,
    pub lod_threshold_for_ik: i32,
    pub b_suppress_warnings: bool,
    pub(crate) __padding_end: [u8; 599],
}
impl FAnimNode_RetargetPoseFromMesh {}
#[repr(C, align(8))]
pub struct FRetargetProfile {
    pub retarget_op_profiles: TArray<FRetargetOpProfile>,
    pub b_apply_target_retarget_pose: bool,
    pub target_retarget_pose_name: FName,
    pub b_apply_source_retarget_pose: bool,
    pub source_retarget_pose_name: FName,
    pub b_force_all_ik_off: bool,
    pub b_apply_chain_settings: bool,
    pub chain_settings: TMap<FName, FTargetChainSettings>,
    pub b_apply_root_settings: bool,
    pub root_settings: FTargetRootSettings,
    pub b_apply_global_settings: bool,
    pub global_settings: FRetargetGlobalSettings,
}
impl FRetargetProfile {}
#[repr(C, align(4))]
pub struct FRetargetGlobalSettings {
    pub b_enable_root: bool,
    pub b_enable_fk: bool,
    pub b_enable_ik: bool,
    pub b_enable_post: bool,
    pub b_copy_base_pose: bool,
    pub copy_base_pose_root: FName,
    pub source_scale_factor: f32,
    pub b_warping: bool,
    pub direction_source: EWarpingDirectionSource,
    pub forward_direction: EBasicAxis,
    pub direction_chain: FName,
    pub warp_forwards: f32,
    pub sideways_offset: f32,
    pub warp_splay: f32,
}
impl FRetargetGlobalSettings {}
#[repr(C, align(8))]
pub struct FTargetRootSettings {
    pub rotation_alpha: f32,
    pub translation_alpha: f32,
    pub blend_to_source: f32,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub scale_horizontal: f32,
    pub scale_vertical: f32,
    pub translation_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub affect_ik_horizontal: f32,
    pub affect_ik_vertical: f32,
}
impl FTargetRootSettings {}
#[repr(C, align(8))]
pub struct FTargetChainSettings {
    pub fk: FTargetChainFKSettings,
    pub ik: FTargetChainIKSettings,
    pub speed_planting: FTargetChainSpeedPlantSettings,
}
impl FTargetChainSettings {}
#[repr(C, align(4))]
pub struct FTargetChainSpeedPlantSettings {
    pub enable_speed_planting: bool,
    pub speed_curve_name: FName,
    pub speed_threshold: f32,
    pub unplant_stiffness: f32,
    pub unplant_critical_damping: f32,
}
impl FTargetChainSpeedPlantSettings {}
#[repr(C, align(8))]
pub struct FTargetChainIKSettings {
    pub enable_ik: bool,
    pub blend_to_source: f32,
    pub blend_to_source_translation: f32,
    pub blend_to_source_rotation: f32,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub static_offset: crate::bindings::core_u_object::FVector,
    pub static_local_offset: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub scale_vertical: f32,
    pub extension: f32,
    pub b_affected_by_ik_warping: bool,
}
impl FTargetChainIKSettings {}
#[repr(C, align(4))]
pub struct FTargetChainFKSettings {
    pub enable_fk: bool,
    pub rotation_mode: ERetargetRotationMode,
    pub rotation_alpha: f32,
    pub translation_mode: ERetargetTranslationMode,
    pub translation_alpha: f32,
    pub pole_vector_matching: f32,
    pub pole_vector_maintain_offset: bool,
    pub pole_vector_offset: f32,
}
impl FTargetChainFKSettings {}
#[repr(C, align(8))]
pub struct FRetargetOpProfile {
    pub op_to_apply_settings_to: FName,
    pub settings_to_apply: crate::bindings::core_u_object::FInstancedStruct,
    pub(crate) __padding_end: [u8; 8],
}
impl FRetargetOpProfile {}
#[repr(C, align(16))]
pub struct FIKRigGoalInput {
    pub(crate) __padding_end: [u8; 128],
}
impl FIKRigGoalInput {}
#[repr(C, align(8))]
pub struct FRigUnit_IKRig {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub goals: TArray<FIKRigGoalInput>,
    pub(crate) __padding_end: [u8; 320],
}
impl FRigUnit_IKRig {}
#[repr(C, align(8))]
pub struct FIKRetargetPose {
    pub(crate) __padding_end: [u8; 112],
}
impl FIKRetargetPose {}
#[repr(C, align(8))]
pub struct FIKRetargetOpSettingsBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub lod_threshold: i32,
    pub(crate) __padding_end: [u8; 52],
}
impl FIKRetargetOpSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRetargetOpBase {
    pub(crate) __padding_end: [u8; 56],
}
impl FIKRetargetOpBase {}
#[repr(C, align(8))]
pub struct FRetargetPoleVectorSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub align_alpha: f64,
    pub static_angular_offset: f64,
    pub maintain_offset: bool,
}
impl FRetargetPoleVectorSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_align: TArray<FRetargetPoleVectorSettings>,
}
impl FIKRetargetAlignPoleVectorOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAlignPoleVectorOp {
    pub(crate) __padding_end: [u8; 192],
}
impl FIKRetargetAlignPoleVectorOp {}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub b_copy_base_pose: bool,
    pub(crate) __padding_end: [u8; 55],
}
impl FIKRetargetCopyBasePoseOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetCopyBasePoseOp {
    pub(crate) __padding_end: [u8; 272],
}
impl FIKRetargetCopyBasePoseOp {}
#[repr(C, align(4))]
pub struct FCurveRemapPair {
    pub source_curve: FName,
    pub target_curve: FName,
}
impl FCurveRemapPair {}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub b_copy_all_source_curves: bool,
    pub b_remap_curves: bool,
    pub curves_to_remap: TArray<FCurveRemapPair>,
}
impl FIKRetargetCurveRemapOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetCurveRemapOp {
    pub(crate) __padding_end: [u8; 176],
}
impl FIKRetargetCurveRemapOp {}
#[repr(C, align(16))]
pub struct FFilterBoneData {
    pub(crate) __padding_end: [u8; 128],
}
impl FFilterBoneData {}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub bones_to_filter: TArray<FFilterBoneData>,
    pub min_frequency: f64,
    pub responsiveness: f64,
    pub velocity_cutoff_hz: f64,
    pub b_reset_playback: bool,
}
impl FIKRetargetFilterBoneOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFilterBoneOp {
    pub(crate) __padding_end: [u8; 176],
}
impl FIKRetargetFilterBoneOp {}
#[repr(C, align(8))]
pub struct FRetargetFKChainSettings {
    pub target_chain_name: FName,
    pub enable_fk: bool,
    pub rotation_mode: EFKChainRotationMode,
    pub rotation_alpha: f64,
    pub translation_mode: EFKChainTranslationMode,
    pub translation_alpha: f64,
}
impl FRetargetFKChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_retarget: TArray<FRetargetFKChainSettings>,
    pub(crate) __padding_end: [u8; 56],
}
impl FIKRetargetFKChainsOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFKChainsOp {
    pub(crate) __padding_end: [u8; 296],
}
impl FIKRetargetFKChainsOp {}
#[repr(C, align(8))]
pub struct FFloorConstraintChainSettings {
    pub target_chain_name: FName,
    pub enable_floor_constraint: bool,
    pub alpha: f64,
    pub maintain_height_offset: f64,
}
impl FFloorConstraintChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub chains_to_affect: TArray<FFloorConstraintChainSettings>,
    pub height_falloff_offset: f64,
    pub height_falloff_distance: f64,
}
impl FIKRetargetFloorConstraintOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetFloorConstraintOp {
    pub(crate) __padding_end: [u8; 168],
}
impl FIKRetargetFloorConstraintOp {}
#[repr(C, align(8))]
pub struct FRetargetIKChainSettings {
    pub target_chain_name: FName,
    pub enable_ik: bool,
    pub blend_to_source: f64,
    pub blend_to_source_translation: f64,
    pub blend_to_source_rotation: f64,
    pub blend_to_source_weights: crate::bindings::core_u_object::FVector,
    pub apply_pelvis_offset_to_source_goals: bool,
    pub static_offset: crate::bindings::core_u_object::FVector,
    pub static_local_offset: crate::bindings::core_u_object::FVector,
    pub static_rotation_offset: crate::bindings::core_u_object::FRotator,
    pub scale_vertical: f64,
    pub extension: f64,
}
impl FRetargetIKChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub chains_to_retarget: TArray<FRetargetIKChainSettings>,
    pub(crate) __padding_end: [u8; 24],
}
impl FIKRetargetIKChainsOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetIKChainsOp {
    pub(crate) __padding_end: [u8; 216],
}
impl FIKRetargetIKChainsOp {}
#[repr(C, align(8))]
pub struct FIKRetargetPelvisMotionOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub source_crotch_offset: f64,
    pub target_crotch_offset: f64,
    pub rotation_alpha: f64,
    pub rotation_offset_local: crate::bindings::core_u_object::FRotator,
    pub rotation_offset_global: crate::bindings::core_u_object::FRotator,
    pub translation_alpha: f64,
    pub translation_offset_local: crate::bindings::core_u_object::FVector,
    pub translation_offset_global: crate::bindings::core_u_object::FVector,
    pub blend_to_source_translation: f64,
    pub blend_to_source_translation_weights: crate::bindings::core_u_object::FVector,
    pub scale_horizontal: f64,
    pub scale_vertical: f64,
    pub affect_ik_horizontal: f64,
    pub affect_ik_vertical: f64,
    pub(crate) __padding_end: [u8; 72],
}
impl FIKRetargetPelvisMotionOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetPelvisMotionOp {
    pub(crate) __padding_end: [u8; 864],
}
impl FIKRetargetPelvisMotionOp {}
#[repr(C, align(16))]
pub struct FPinBoneData {
    pub(crate) __padding_end: [u8; 400],
}
impl FPinBoneData {}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub bones_to_pin: TArray<FPinBoneData>,
    pub skeleton_to_copy_from: ERetargetSourceOrTarget,
    pub b_copy_translation: bool,
    pub translation_mode: EPinBoneTranslationMode,
    pub b_copy_rotation: bool,
    pub rotation_mode: EPinBoneRotationMode,
    pub b_copy_scale: bool,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub local_offset: crate::bindings::core_u_object::FTransform,
}
impl FIKRetargetPinBoneOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetPinBoneOp {
    pub(crate) __padding_end: [u8; 368],
}
impl FIKRetargetPinBoneOp {}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub pose_to_apply: FName,
    pub alpha: f32,
}
impl FIKRetargetAdditivePoseOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetAdditivePoseOp {
    pub(crate) __padding_end: [u8; 152],
}
impl FIKRetargetAdditivePoseOp {}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_104: [u8; 104],
    pub root_motion_source: ERootMotionSource,
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 23],
    pub b_rotate_with_pelvis: bool,
    pub root_height_source: ERootMotionHeightSource,
    pub global_offset: crate::bindings::core_u_object::FTransform,
    pub b_maintain_offset_from_pelvis: bool,
    pub b_propagate_to_non_retargeted_children: bool,
    pub(crate) __padding_end: [u8; 46],
}
impl FIKRetargetRootMotionOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetRootMotionOp {
    pub(crate) __padding_end: [u8; 768],
}
impl FIKRetargetRootMotionOp {}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub excluded_goals: TArray<FName>,
    pub(crate) __padding_end: [u8; 24],
}
impl FIKRetargetRunIKRigOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetRunIKRigOp {
    pub(crate) __padding_end: [u8; 600],
}
impl FIKRetargetRunIKRigOp {}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub source_scale_factor: f64,
    pub scale_pivot: EScaleSourcePivot,
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 23],
    pub b_project_scale_pivot_to_floor: bool,
}
impl FIKRetargetScaleSourceOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetScaleSourceOp {
    pub(crate) __padding_end: [u8; 176],
}
impl FIKRetargetScaleSourceOp {}
#[repr(C, align(4))]
pub struct FRetargetSpeedPlantingSettings {
    pub target_chain_name: FName,
    pub speed_curve_name: FName,
}
impl FRetargetSpeedPlantingSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub chains_to_speed_plant: TArray<FRetargetSpeedPlantingSettings>,
    pub speed_threshold: f64,
    pub stiffness: f64,
    pub critical_damping: f64,
}
impl FIKRetargetSpeedPlantingOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetSpeedPlantingOp {
    pub(crate) __padding_end: [u8; 184],
}
impl FIKRetargetSpeedPlantingOp {}
#[repr(C, align(8))]
pub struct FRetargetStretchChainSettings {
    pub target_chain_name: FName,
    pub b_enabled: bool,
    pub match_source_length: f64,
    pub scale_chain_length: f64,
}
impl FRetargetStretchChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub ik_rig_asset: UPtr<UIKRigDefinition>,
    pub chains_to_stretch: TArray<FRetargetStretchChainSettings>,
}
impl FIKRetargetStretchChainOpSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStretchChainOp {
    pub(crate) __padding_end: [u8; 192],
}
impl FIKRetargetStretchChainOp {}
#[repr(C, align(4))]
pub struct FRetargetStrideWarpChainSettings {
    pub target_chain_name: FName,
    pub enable_stride_warping: bool,
}
impl FRetargetStrideWarpChainSettings {}
#[repr(C, align(8))]
pub struct FIKRetargetStrideWarpingOpSettings {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub chain_settings: TArray<FRetargetStrideWarpChainSettings>,
    pub direction_source: EWarpingDirectionSource,
    pub forward_direction: EBasicAxis,
    pub direction_chain: FName,
    pub warp_forwards: f64,
    pub sideways_offset: f64,
    pub warp_splay: f64,
    pub debug_draw_size: f64,
    pub debug_draw_thickness: f64,
    pub(crate) __padding_end: [u8; 8],
}
impl FIKRetargetStrideWarpingOpSettings {}
#[repr(C, align(16))]
pub struct FIKRetargetStrideWarpingOp {
    pub(crate) __padding_end: [u8; 336],
}
impl FIKRetargetStrideWarpingOp {}
#[repr(C, align(4))]
pub struct FBoneChain {
    pub chain_name: FName,
    #[doc(hidden)]
    pub(crate) __padding_52: [u8; 40],
    pub ik_goal_name: FName,
}
impl FBoneChain {}
#[repr(C, align(8))]
pub struct FRetargetDefinition {
    pub root_bone: FName,
    pub bone_chains: TArray<FBoneChain>,
}
impl FRetargetDefinition {}
#[repr(C, align(8))]
pub struct FIKRigSettingsBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FIKRigSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigGoalSettingsBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub goal: FName,
}
impl FIKRigGoalSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverGoalSettings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub bone_name: FName,
    pub influence_multiplier: f32,
}
impl FIKRigBodyMoverGoalSettings {}
#[repr(C, align(8))]
pub struct FIKRigSolverSettingsBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FIKRigSolverSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start_bone: FName,
    pub position_alpha: f32,
    pub position_positive_x: f32,
    pub position_negative_x: f32,
    pub position_positive_y: f32,
    pub position_negative_y: f32,
    pub position_positive_z: f32,
    pub position_negative_z: f32,
    pub rotation_alpha: f32,
    pub rotate_x_alpha: f32,
    pub rotate_y_alpha: f32,
    pub rotate_z_alpha: f32,
}
impl FIKRigBodyMoverSettings {}
#[repr(C, align(8))]
pub struct FIKRigSolverBase {
    pub(crate) __padding_end: [u8; 24],
}
impl FIKRigSolverBase {}
#[repr(C, align(8))]
pub struct FIKRigBodyMoverSolver {
    pub(crate) __padding_end: [u8; 112],
}
impl FIKRigBodyMoverSolver {}
#[repr(C, align(8))]
pub struct FIKRigFBIKGoalSettings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub bone_name: FName,
    pub chain_depth: i32,
    pub strength_alpha: f32,
    pub pull_chain_alpha: f32,
    pub pin_rotation: f32,
}
impl FIKRigFBIKGoalSettings {}
#[repr(C, align(8))]
pub struct FIKRigBoneSettingsBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bone: FName,
}
impl FIKRigBoneSettingsBase {}
#[repr(C, align(8))]
pub struct FIKRigFBIKBoneSettings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub rotation_stiffness: f32,
    pub position_stiffness: f32,
    pub x: crate::bindings::pbik::EPBIKLimitType,
    pub min_x: f32,
    pub max_x: f32,
    pub y: crate::bindings::pbik::EPBIKLimitType,
    pub min_y: f32,
    pub max_y: f32,
    pub z: crate::bindings::pbik::EPBIKLimitType,
    pub min_z: f32,
    pub max_z: f32,
    pub b_use_preferred_angles: bool,
    pub preferred_angles: crate::bindings::core_u_object::FVector,
}
impl FIKRigFBIKBoneSettings {}
#[repr(C, align(8))]
pub struct FIKRigFBIKSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub root_bone: FName,
    pub iterations: i32,
    pub sub_iterations: i32,
    pub mass_multiplier: f32,
    pub b_allow_stretch: bool,
    pub root_behavior: crate::bindings::pbik::EPBIKRootBehavior,
    pub pre_pull_root_settings: crate::bindings::pbik::FRootPrePullSettings,
    pub global_pull_chain_alpha: f32,
    pub max_angle: f32,
    pub over_relaxation: f32,
}
impl FIKRigFBIKSettings {}
#[repr(C, align(8))]
pub struct FIKRigFullBodyIKSolver {
    pub(crate) __padding_end: [u8; 224],
}
impl FIKRigFullBodyIKSolver {}
#[repr(C, align(8))]
pub struct FLimbSolverSettings {
    pub(crate) __padding_end: [u8; 48],
}
impl FLimbSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigLimbSolverSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub start_bone: FName,
    pub goal_name: FName,
    pub end_bone: FName,
}
impl FIKRigLimbSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigLimbSolver {
    pub(crate) __padding_end: [u8; 152],
}
impl FIKRigLimbSolver {}
#[repr(C, align(8))]
pub struct FIKRigPoleSolverSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start_bone: FName,
    pub end_bone: FName,
    pub aim_at_goal: FName,
    pub alpha: f32,
}
impl FIKRigPoleSolverSettings {}
#[repr(C, align(8))]
pub struct FIKRigPoleSolver {
    pub(crate) __padding_end: [u8; 104],
}
impl FIKRigPoleSolver {}
#[repr(C, align(8))]
pub struct FIKRigSetTransformSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub goal: FName,
    pub bone_to_affect: FName,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub alpha: f32,
}
impl FIKRigSetTransformSettings {}
#[repr(C, align(8))]
pub struct FIKRigSetTransform {
    pub(crate) __padding_end: [u8; 80],
}
impl FIKRigSetTransform {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbBoneSettings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub squash_direction: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 8],
}
impl FIKRigStretchLimbBoneSettings {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start_bone: FName,
    pub end_bone: FName,
    pub goal: FName,
    pub b_enable_stretching: bool,
    pub maximum_stretch_distance: f64,
    pub stretch_start_percent: f64,
    pub rotation_mode: EStretchLimbRotationMode,
    pub rotate_end_bone_with_goal: f64,
    pub iterations: i32,
    pub squash_mode: EStretchLimbSquashMode,
    pub squash_strength: f64,
}
impl FIKRigStretchLimbSettings {}
#[repr(C, align(8))]
pub struct FIKRigStretchLimbSolver {
    pub(crate) __padding_end: [u8; 240],
}
impl FIKRigStretchLimbSolver {}
pub struct IIKGoalCreatorInterface {}
#[repr(C, align(8))]
pub struct UIKGoalCreatorInterface {
    __padding_end: [u8; 48],
}
impl UIKGoalCreatorInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKGoalCreatorInterface")
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
    pub fn add_ik_goals(&mut self, out_goals: &mut TMap<FName, FIKRigGoal>) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_goal_creator_interface_add_ik_goals,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_goals,
                __buffer.add(0).cast::<TMap<FName, FIKRigGoal>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_goal_creator_interface_add_ik_goals,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TMap<FName, FIKRigGoal>>().swap(out_goals);
        }
    }
}
#[repr(C, align(8))]
pub struct UIKRigComponent {
    __padding_end: [u8; 272],
}
impl UIKRigComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigComponent")
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
    pub fn set_ik_rig_goal_transform(
        &mut self,
        goal_name: FName,
        transform: crate::bindings::core_u_object::FTransform,
        position_alpha: f32,
        rotation_alpha: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position_alpha,
                __buffer.add(112).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation_alpha,
                __buffer.add(116).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal_transform,
                __buffer,
            )
        };
    }
    pub fn set_ik_rig_goal_position_and_rotation(
        &mut self,
        goal_name: FName,
        position: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FQuat,
        position_alpha: f32,
        rotation_alpha: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal_position_and_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &position_alpha,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation_alpha,
                __buffer.add(84).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal_position_and_rotation,
                __buffer,
            )
        };
    }
    pub fn set_ik_rig_goal(&mut self, goal: &FIKRigGoal) {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(goal, __buffer.add(0).cast::<FIKRigGoal>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_set_ik_rig_goal,
                __buffer,
            )
        };
    }
    pub fn clear_all_goals(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_clear_all_goals,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_component_clear_all_goals,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct URetargetChainSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub source_chain: FName,
    pub target_chain: FName,
    pub settings: FTargetChainSettings,
    __padding_end: [u8; 152],
}
impl URetargetChainSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetChainSettings")
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
pub struct URetargetRootSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub settings: FTargetRootSettings,
    __padding_end: [u8; 88],
}
impl URetargetRootSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetRootSettings")
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
pub struct UIKRetargetGlobalSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub settings: FRetargetGlobalSettings,
}
impl UIKRetargetGlobalSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetGlobalSettings")
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
pub struct UIKRetargeter {
    __padding_end: [u8; 848],
}
impl UIKRetargeter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargeter")
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
    pub fn set_root_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        root_settings: &FTargetRootSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<416>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_root_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_settings,
                __buffer.add(312).cast::<FTargetRootSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_root_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn set_global_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        global_settings: &FRetargetGlobalSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<372>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_global_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_settings,
                __buffer.add(312).cast::<FRetargetGlobalSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_global_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn set_chain_speed_plant_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        speed_plant_settings: &FTargetChainSpeedPlantSettings,
        target_chain_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<352>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_speed_plant_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                speed_plant_settings,
                __buffer.add(312).cast::<FTargetChainSpeedPlantSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(340).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_speed_plant_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn set_chain_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        chain_settings: &FTargetChainSettings,
        target_chain_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<516>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                chain_settings,
                __buffer.add(312).cast::<FTargetChainSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(504).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn set_chain_ik_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        ik_settings: &FTargetChainIKSettings,
        target_chain_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<452>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_ik_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ik_settings,
                __buffer.add(312).cast::<FTargetChainIKSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(440).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_ik_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn set_chain_fk_settings_in_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        fk_settings: &FTargetChainFKSettings,
        target_chain_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<352>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_fk_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                fk_settings,
                __buffer.add(312).cast::<FTargetChainFKSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(340).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_set_chain_fk_settings_in_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
    }
    pub fn has_target_ik_rig(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_has_target_ik_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_has_target_ik_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_source_ik_rig(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_has_source_ik_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_has_source_ik_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_root_settings_from_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
    ) -> FTargetRootSettings {
        let mut __stack = crate::core_data::StackAlloc::<416>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_root_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_root_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
        unsafe { __buffer.add(312).cast::<FTargetRootSettings>().read() }
    }
    pub fn get_root_settings_from_retarget_asset(
        retarget_asset: UPtr<UIKRetargeter>,
        optional_profile_name: FName,
        out_settings: &mut FTargetRootSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_root_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &retarget_asset,
                __buffer.add(0).cast::<UPtr<UIKRetargeter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_profile_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_settings,
                __buffer.add(24).cast::<FTargetRootSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_root_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FTargetRootSettings>().swap(out_settings);
        }
    }
    pub fn get_global_settings_from_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
    ) -> FRetargetGlobalSettings {
        let mut __stack = crate::core_data::StackAlloc::<372>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_global_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_global_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
        unsafe { __buffer.add(312).cast::<FRetargetGlobalSettings>().read() }
    }
    pub fn get_global_settings_from_retarget_asset(
        retarget_asset: UPtr<UIKRetargeter>,
        optional_profile_name: FName,
        out_settings: &mut FRetargetGlobalSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_global_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &retarget_asset,
                __buffer.add(0).cast::<UPtr<UIKRetargeter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_profile_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_settings,
                __buffer.add(20).cast::<FRetargetGlobalSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_global_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(20).cast::<FRetargetGlobalSettings>().swap(out_settings);
        }
    }
    pub fn get_chain_using_goal_from_retarget_asset(
        retarget_asset: UPtr<UIKRetargeter>,
        ik_goal_name: FName,
    ) -> FTargetChainSettings {
        let mut __stack = crate::core_data::StackAlloc::<216>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_using_goal_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &retarget_asset,
                __buffer.add(0).cast::<UPtr<UIKRetargeter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ik_goal_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_using_goal_from_retarget_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FTargetChainSettings>().read() }
    }
    pub fn get_chain_settings_from_retarget_profile(
        retarget_profile: &mut FRetargetProfile,
        target_chain_name: FName,
    ) -> FTargetChainSettings {
        let mut __stack = crate::core_data::StackAlloc::<520>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(312).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_settings_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(retarget_profile);
        }
        unsafe { __buffer.add(328).cast::<FTargetChainSettings>().read() }
    }
    pub fn get_chain_settings_from_retarget_asset(
        retarget_asset: UPtr<UIKRetargeter>,
        target_chain_name: FName,
        optional_profile_name: FName,
    ) -> FTargetChainSettings {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &retarget_asset,
                __buffer.add(0).cast::<UPtr<UIKRetargeter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_chain_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_profile_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::UIKRetargeter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retargeter_get_chain_settings_from_retarget_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FTargetChainSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetOpControllerBase {
    __padding_end: [u8; 56],
}
impl UIKRetargetOpControllerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetOpControllerBase")
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
pub struct URetargetOpBase {
    __padding_end: [u8; 56],
}
impl URetargetOpBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetOpBase")
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
pub struct URetargetOpStack {
    __padding_end: [u8; 64],
}
impl URetargetOpStack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetOpStack")
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
pub struct UIKRetargetProcessor {
    __padding_end: [u8; 720],
}
impl UIKRetargetProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetProcessor")
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
pub struct URetargetProfileLibrary {
    __padding_end: [u8; 48],
}
impl URetargetProfileLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetProfileLibrary")
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
    pub fn get_op_controller_from_retarget_profile(
        in_retarget_profile: &mut FRetargetProfile,
        in_retarget_op_name: FName,
    ) -> UPtr<UIKRetargetOpControllerBase> {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .u_retarget_profile_library_get_op_controller_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_retarget_profile,
                __buffer.add(0).cast::<FRetargetProfile>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_retarget_op_name,
                __buffer.add(312).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::URetargetProfileLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .u_retarget_profile_library_get_op_controller_from_retarget_profile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FRetargetProfile>().swap(in_retarget_profile);
        }
        unsafe { __buffer.add(328).cast::<UPtr<UIKRetargetOpControllerBase>>().read() }
    }
    pub fn copy_retarget_profile_from_retarget_asset(
        in_retarget_asset: UPtr<UIKRetargeter>,
    ) -> FRetargetProfile {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .u_retarget_profile_library_copy_retarget_profile_from_retarget_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_retarget_asset,
                __buffer.add(0).cast::<UPtr<UIKRetargeter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig::URetargetProfileLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .u_retarget_profile_library_copy_retarget_profile_from_retarget_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FRetargetProfile>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetAlignPoleVectorController {
    __padding_end: [u8; 56],
}
impl UIKRetargetAlignPoleVectorController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetAlignPoleVectorController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetAlignPoleVectorOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_align_pole_vector_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetAlignPoleVectorOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_align_pole_vector_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetAlignPoleVectorOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_align_pole_vector_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_align_pole_vector_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetAlignPoleVectorOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetCopyBasePoseController {
    __padding_end: [u8; 56],
}
impl UIKRetargetCopyBasePoseController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetCopyBasePoseController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetCopyBasePoseOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetCopyBasePoseOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_copy_from_start(&self, in_bone_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_set_copy_from_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_set_copy_from_start,
                __buffer,
            )
        };
    }
    pub fn reset_bones_to_exclude(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_reset_bones_to_exclude,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_reset_bones_to_exclude,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetCopyBasePoseOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetCopyBasePoseOpSettings>().read() }
    }
    pub fn get_copy_from_start(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_copy_from_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_copy_from_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_bones_to_exclude(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_bones_to_exclude,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_get_bones_to_exclude,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn add_bone_to_exclude(&self, in_bone_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_add_bone_to_exclude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_copy_base_pose_controller_add_bone_to_exclude,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetCurveRemapController {
    __padding_end: [u8; 56],
}
impl UIKRetargetCurveRemapController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetCurveRemapController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetCurveRemapOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_curve_remap_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetCurveRemapOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_curve_remap_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetCurveRemapOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_curve_remap_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_curve_remap_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetCurveRemapOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCurveRemapOp {
    __padding_end: [u8; 80],
}
impl UCurveRemapOp {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveRemapOp")
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
pub struct UIKRetargetFilterBoneController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFilterBoneController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetFilterBoneController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetFilterBoneOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetFilterBoneOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetFilterBoneOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetFilterBoneOpSettings>().read() }
    }
    pub fn get_all_bones_to_filter(&mut self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_get_all_bones_to_filter,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_get_all_bones_to_filter,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn clear_bones_to_filter(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_clear_bones_to_filter,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_clear_bones_to_filter,
                __buffer,
            )
        };
    }
    pub fn add_bone_to_filter(&mut self, in_target_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_add_bone_to_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_filter_bone_controller_add_bone_to_filter,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetFKChainsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFKChainsController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetFKChainsController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetFKChainsOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_fk_chains_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetFKChainsOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_fk_chains_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetFKChainsOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_fk_chains_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_fk_chains_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetFKChainsOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetFloorGoalsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetFloorGoalsController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetFloorGoalsController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetIKChainsOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_floor_goals_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetIKChainsOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_floor_goals_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetIKChainsOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_floor_goals_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_floor_goals_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetIKChainsOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetIKChainsController {
    __padding_end: [u8; 56],
}
impl UIKRetargetIKChainsController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetIKChainsController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetIKChainsOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_ik_chains_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetIKChainsOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_ik_chains_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetIKChainsOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_ik_chains_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_ik_chains_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetIKChainsOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetPelvisMotionController {
    __padding_end: [u8; 56],
}
impl UIKRetargetPelvisMotionController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetPelvisMotionController")
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
    pub fn set_target_pelvis_bone(&mut self, in_target_pelvis_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_target_pelvis_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_pelvis_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_target_pelvis_bone,
                __buffer,
            )
        };
    }
    pub fn set_source_pelvis_bone(&mut self, in_source_pelvis_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_source_pelvis_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_pelvis_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_source_pelvis_bone,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, in_settings: FIKRetargetPelvisMotionOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetPelvisMotionOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_target_pelvis_bone(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_target_pelvis_bone,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_target_pelvis_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_source_pelvis_bone(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_source_pelvis_bone,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_source_pelvis_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_settings(&mut self) -> FIKRetargetPelvisMotionOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pelvis_motion_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetPelvisMotionOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetPinBoneController {
    __padding_end: [u8; 56],
}
impl UIKRetargetPinBoneController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetPinBoneController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetPinBoneOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetPinBoneOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn set_bone_pair(
        &mut self,
        in_bone_to_copy_from: FName,
        in_bone_to_copy_to: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_set_bone_pair,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_to_copy_from,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_to_copy_to,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_set_bone_pair,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetPinBoneOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetPinBoneOpSettings>().read() }
    }
    pub fn get_all_bone_pairs(&mut self) -> TMap<FName, FName> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_get_all_bone_pairs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_get_all_bone_pairs,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<FName, FName>>().read() }
    }
    pub fn clear_all_bone_pairs(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_clear_all_bone_pairs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_pin_bone_controller_clear_all_bone_pairs,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UPinBoneOp {
    __padding_end: [u8; 288],
}
impl UPinBoneOp {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPinBoneOp")
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
pub struct UIKRetargetAdditivePoseController {
    __padding_end: [u8; 56],
}
impl UIKRetargetAdditivePoseController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetAdditivePoseController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetAdditivePoseOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_additive_pose_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetAdditivePoseOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_additive_pose_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetAdditivePoseOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_additive_pose_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_additive_pose_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetAdditivePoseOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetRootMotionController {
    __padding_end: [u8; 56],
}
impl UIKRetargetRootMotionController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetRootMotionController")
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
    pub fn set_target_root_bone(&mut self, in_target_root_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_target_root_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_root_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_target_root_bone,
                __buffer,
            )
        };
    }
    pub fn set_target_pelvis_bone(&mut self, in_target_pelvis_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_target_pelvis_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_pelvis_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_target_pelvis_bone,
                __buffer,
            )
        };
    }
    pub fn set_source_root_bone(&mut self, in_source_root_bone: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_source_root_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_root_bone,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_source_root_bone,
                __buffer,
            )
        };
    }
    pub fn set_settings(&mut self, in_settings: FIKRetargetRootMotionOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetRootMotionOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_target_root_bone(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_target_root_bone,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_target_root_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_target_pelvis_bone(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_target_pelvis_bone,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_target_pelvis_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_source_root_bone(&mut self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_source_root_bone,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_source_root_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_settings(&mut self) -> FIKRetargetRootMotionOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_root_motion_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetRootMotionOpSettings>().read() }
    }
}
#[repr(C, align(16))]
pub struct URootMotionGeneratorOp {
    __padding_end: [u8; 208],
}
impl URootMotionGeneratorOp {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionGeneratorOp")
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
pub struct UIKRetargetRunIKRigController {
    __padding_end: [u8; 56],
}
impl UIKRetargetRunIKRigController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetRunIKRigController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetRunIKRigOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_run_ik_rig_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetRunIKRigOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_run_ik_rig_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetRunIKRigOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_run_ik_rig_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_run_ik_rig_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetRunIKRigOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetScaleSourceController {
    __padding_end: [u8; 56],
}
impl UIKRetargetScaleSourceController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetScaleSourceController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetScaleSourceOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_scale_source_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetScaleSourceOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_scale_source_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetScaleSourceOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_scale_source_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_scale_source_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetScaleSourceOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetSpeedPlantingController {
    __padding_end: [u8; 56],
}
impl UIKRetargetSpeedPlantingController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetSpeedPlantingController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetSpeedPlantingOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_speed_planting_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetSpeedPlantingOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_speed_planting_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetSpeedPlantingOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_speed_planting_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_speed_planting_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetSpeedPlantingOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetStretchChainController {
    __padding_end: [u8; 56],
}
impl UIKRetargetStretchChainController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetStretchChainController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetStretchChainOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stretch_chain_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetStretchChainOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stretch_chain_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetStretchChainOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stretch_chain_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stretch_chain_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetStretchChainOpSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRetargetStrideWarpingController {
    __padding_end: [u8; 56],
}
impl UIKRetargetStrideWarpingController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetStrideWarpingController")
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
    pub fn set_settings(&mut self, in_settings: FIKRetargetStrideWarpingOpSettings) {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stride_warping_controller_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRetargetStrideWarpingOpSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stride_warping_controller_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings(&mut self) -> FIKRetargetStrideWarpingOpSettings {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stride_warping_controller_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_retarget_stride_warping_controller_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRetargetStrideWarpingOpSettings>().read() }
    }
}
#[repr(C, align(16))]
pub struct UIKRigEffectorGoal {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub goal_name: FName,
    pub bone_name: FName,
    pub position_alpha: f32,
    pub rotation_alpha: f32,
    pub current_transform: crate::bindings::core_u_object::FTransform,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub preview_mode: EIKRigGoalPreviewMode,
    pub size_multiplier: f32,
    pub thickness_multiplier: f32,
    pub b_expose_position: bool,
    pub b_expose_rotation: bool,
}
impl UIKRigEffectorGoal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigEffectorGoal")
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
pub struct UIKRigDefinition {
    __padding_end: [u8; 320],
}
impl UIKRigDefinition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigDefinition")
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
pub struct UIKRigProcessor {
    __padding_end: [u8; 416],
}
impl UIKRigProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigProcessor")
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
pub struct UIKRigSolverControllerBase {
    __padding_end: [u8; 56],
}
impl UIKRigSolverControllerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigSolverControllerBase")
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
    pub fn set_enabled(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_solver_controller_base_set_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_solver_controller_base_set_enabled,
                __buffer,
            )
        };
    }
    pub fn get_enabled(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_solver_controller_base_get_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_solver_controller_base_get_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRigBodyMoverController {
    __padding_end: [u8; 56],
}
impl UIKRigBodyMoverController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigBodyMoverController")
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
    pub fn set_solver_settings(&mut self, in_settings: FIKRigBodyMoverSettings) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_set_solver_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRigBodyMoverSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_set_solver_settings,
                __buffer,
            )
        };
    }
    pub fn set_goal_settings(
        &mut self,
        in_goal_name: FName,
        in_settings: FIKRigBodyMoverGoalSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_set_goal_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_goal_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FIKRigBodyMoverGoalSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_set_goal_settings,
                __buffer,
            )
        };
    }
    pub fn get_solver_settings(&mut self) -> FIKRigBodyMoverSettings {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_get_solver_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_get_solver_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRigBodyMoverSettings>().read() }
    }
    pub fn get_goal_settings(
        &mut self,
        in_goal_name: FName,
    ) -> FIKRigBodyMoverGoalSettings {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_get_goal_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_goal_name,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_body_mover_controller_get_goal_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FIKRigBodyMoverGoalSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRig_BodyMoverEffector {
    __padding_end: [u8; 80],
}
impl UIKRig_BodyMoverEffector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_BodyMoverEffector")
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
pub struct UIKRigSolver {
    __padding_end: [u8; 56],
}
impl UIKRigSolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigSolver")
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
pub struct UIKRig_BodyMover {
    __padding_end: [u8; 128],
}
impl UIKRig_BodyMover {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_BodyMover")
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
pub struct UIKRigFBIKController {
    __padding_end: [u8; 56],
}
impl UIKRigFBIKController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigFBIKController")
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
    pub fn set_solver_settings(&mut self, in_settings: FIKRigFBIKSettings) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_solver_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRigFBIKSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_solver_settings,
                __buffer,
            )
        };
    }
    pub fn set_goal_settings(
        &mut self,
        in_goal_name: FName,
        in_settings: FIKRigFBIKGoalSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_goal_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_goal_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FIKRigFBIKGoalSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_goal_settings,
                __buffer,
            )
        };
    }
    pub fn set_bone_settings(
        &mut self,
        in_bone_name: FName,
        in_settings: FIKRigFBIKBoneSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_bone_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FIKRigFBIKBoneSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_set_bone_settings,
                __buffer,
            )
        };
    }
    pub fn get_solver_settings(&mut self) -> FIKRigFBIKSettings {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_solver_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_solver_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRigFBIKSettings>().read() }
    }
    pub fn get_goal_settings(&mut self, in_goal_name: FName) -> FIKRigFBIKGoalSettings {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_goal_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_goal_name,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_goal_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FIKRigFBIKGoalSettings>().read() }
    }
    pub fn get_bone_settings(&mut self, in_bone_name: FName) -> FIKRigFBIKBoneSettings {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_bone_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_name,
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
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_fbik_controller_get_bone_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FIKRigFBIKBoneSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRig_FBIKEffector {
    __padding_end: [u8; 88],
}
impl UIKRig_FBIKEffector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_FBIKEffector")
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
pub struct UIKRig_FBIKBoneSettings {
    __padding_end: [u8; 136],
}
impl UIKRig_FBIKBoneSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_FBIKBoneSettings")
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
pub struct UIKRigFBIKSolver {
    __padding_end: [u8; 160],
}
impl UIKRigFBIKSolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigFBIKSolver")
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
pub struct UIKRigLimbSolverController {
    __padding_end: [u8; 56],
}
impl UIKRigLimbSolverController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigLimbSolverController")
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
    pub fn set_solver_settings(&mut self, in_settings: FIKRigLimbSolverSettings) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_limb_solver_controller_set_solver_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRigLimbSolverSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_limb_solver_controller_set_solver_settings,
                __buffer,
            )
        };
    }
    pub fn get_solver_settings(&mut self) -> FIKRigLimbSolverSettings {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_limb_solver_controller_get_solver_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_limb_solver_controller_get_solver_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRigLimbSolverSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRig_LimbEffector {
    __padding_end: [u8; 72],
}
impl UIKRig_LimbEffector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_LimbEffector")
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
pub struct UIKRig_LimbSolver {
    __padding_end: [u8; 112],
}
impl UIKRig_LimbSolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_LimbSolver")
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
pub struct UIKRigPoleSolverController {
    __padding_end: [u8; 56],
}
impl UIKRigPoleSolverController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigPoleSolverController")
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
    pub fn set_solver_settings(&mut self, in_settings: FIKRigPoleSolverSettings) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_pole_solver_controller_set_solver_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRigPoleSolverSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_pole_solver_controller_set_solver_settings,
                __buffer,
            )
        };
    }
    pub fn get_solver_settings(&mut self) -> FIKRigPoleSolverSettings {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_pole_solver_controller_get_solver_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_pole_solver_controller_get_solver_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRigPoleSolverSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRig_PoleSolverEffector {
    __padding_end: [u8; 80],
}
impl UIKRig_PoleSolverEffector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_PoleSolverEffector")
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
pub struct UIKRig_PoleSolver {
    __padding_end: [u8; 88],
}
impl UIKRig_PoleSolver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_PoleSolver")
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
pub struct UIKRigSetTransformController {
    __padding_end: [u8; 56],
}
impl UIKRigSetTransformController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigSetTransformController")
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
    pub fn set_solver_settings(&mut self, in_settings: FIKRigSetTransformSettings) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_set_transform_controller_set_solver_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<FIKRigSetTransformSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_set_transform_controller_set_solver_settings,
                __buffer,
            )
        };
    }
    pub fn get_solver_settings(&mut self) -> FIKRigSetTransformSettings {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_set_transform_controller_get_solver_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig::__FUNCTION_PTRS
                    .uik_rig_set_transform_controller_get_solver_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FIKRigSetTransformSettings>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRig_SetTransformEffector {
    __padding_end: [u8; 56],
}
impl UIKRig_SetTransformEffector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_SetTransformEffector")
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
pub struct UIKRig_SetTransform {
    __padding_end: [u8; 88],
}
impl UIKRig_SetTransform {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRig_SetTransform")
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
pub struct EIKRigGoalTransformSource(pub u8);
impl EIKRigGoalTransformSource {
    pub const MANUAL: EIKRigGoalTransformSource = EIKRigGoalTransformSource(0);
    pub const BONE: EIKRigGoalTransformSource = EIKRigGoalTransformSource(1);
    pub const ACTOR_COMPONENT: EIKRigGoalTransformSource = EIKRigGoalTransformSource(2);
}
#[repr(transparent)]
pub struct EIKRigGoalSpace(pub u8);
impl EIKRigGoalSpace {
    pub const COMPONENT: EIKRigGoalSpace = EIKRigGoalSpace(0);
    pub const ADDITIVE: EIKRigGoalSpace = EIKRigGoalSpace(1);
    pub const WORLD: EIKRigGoalSpace = EIKRigGoalSpace(2);
}
#[repr(transparent)]
pub struct ERetargetSourceMode(pub u8);
impl ERetargetSourceMode {
    pub const PARENT_SKELETAL_MESH_COMPONENT: ERetargetSourceMode = ERetargetSourceMode(
        0,
    );
    pub const CUSTOM_SKELETAL_MESH_COMPONENT: ERetargetSourceMode = ERetargetSourceMode(
        1,
    );
    pub const SOURCE_POSE_PIN: ERetargetSourceMode = ERetargetSourceMode(2);
}
#[repr(transparent)]
pub struct ERetargetRotationMode(pub u8);
impl ERetargetRotationMode {
    pub const INTERPOLATED: ERetargetRotationMode = ERetargetRotationMode(0);
    pub const ONE_TO_ONE: ERetargetRotationMode = ERetargetRotationMode(1);
    pub const ONE_TO_ONE_REVERSED: ERetargetRotationMode = ERetargetRotationMode(2);
    pub const MATCH_CHAIN: ERetargetRotationMode = ERetargetRotationMode(3);
    pub const MATCH_SCALED_CHAIN: ERetargetRotationMode = ERetargetRotationMode(4);
    pub const NONE: ERetargetRotationMode = ERetargetRotationMode(5);
}
#[repr(transparent)]
pub struct EWarpingDirectionSource(pub i32);
impl EWarpingDirectionSource {
    pub const GOALS: EWarpingDirectionSource = EWarpingDirectionSource(0);
    pub const CHAIN: EWarpingDirectionSource = EWarpingDirectionSource(1);
    pub const ROOT_BONE: EWarpingDirectionSource = EWarpingDirectionSource(2);
}
#[repr(transparent)]
pub struct EBasicAxis(pub i32);
impl EBasicAxis {
    pub const X: EBasicAxis = EBasicAxis(0);
    pub const Y: EBasicAxis = EBasicAxis(1);
    pub const Z: EBasicAxis = EBasicAxis(2);
    pub const NEG_X: EBasicAxis = EBasicAxis(3);
    pub const NEG_Y: EBasicAxis = EBasicAxis(4);
    pub const NEG_Z: EBasicAxis = EBasicAxis(5);
}
#[repr(transparent)]
pub struct ERetargetTranslationMode(pub u8);
impl ERetargetTranslationMode {
    pub const NONE: ERetargetTranslationMode = ERetargetTranslationMode(0);
    pub const GLOBALLY_SCALED: ERetargetTranslationMode = ERetargetTranslationMode(1);
    pub const ABSOLUTE: ERetargetTranslationMode = ERetargetTranslationMode(2);
    pub const STRETCH_BONE_LENGTH_UNIFORMLY: ERetargetTranslationMode = ERetargetTranslationMode(
        3,
    );
    pub const STRETCH_BONE_LENGTH_NON_UNIFORMLY: ERetargetTranslationMode = ERetargetTranslationMode(
        4,
    );
}
#[repr(transparent)]
pub struct EFKChainRotationMode(pub u8);
impl EFKChainRotationMode {
    pub const NONE: EFKChainRotationMode = EFKChainRotationMode(5);
    pub const INTERPOLATED: EFKChainRotationMode = EFKChainRotationMode(0);
    pub const ONE_TO_ONE: EFKChainRotationMode = EFKChainRotationMode(1);
    pub const ONE_TO_ONE_REVERSED: EFKChainRotationMode = EFKChainRotationMode(2);
    pub const MATCH_CHAIN: EFKChainRotationMode = EFKChainRotationMode(3);
    pub const MATCH_SCALED_CHAIN: EFKChainRotationMode = EFKChainRotationMode(4);
    pub const COPY_LOCAL: EFKChainRotationMode = EFKChainRotationMode(6);
}
#[repr(transparent)]
pub struct EFKChainTranslationMode(pub u8);
impl EFKChainTranslationMode {
    pub const NONE: EFKChainTranslationMode = EFKChainTranslationMode(0);
    pub const GLOBALLY_SCALED: EFKChainTranslationMode = EFKChainTranslationMode(1);
    pub const ABSOLUTE: EFKChainTranslationMode = EFKChainTranslationMode(2);
    pub const STRETCH_BONE_LENGTH_UNIFORMLY: EFKChainTranslationMode = EFKChainTranslationMode(
        3,
    );
    pub const STRETCH_BONE_LENGTH_NON_UNIFORMLY: EFKChainTranslationMode = EFKChainTranslationMode(
        4,
    );
    pub const ORIENT_AND_SCALE: EFKChainTranslationMode = EFKChainTranslationMode(5);
}
#[repr(transparent)]
pub struct ERetargetSourceOrTarget(pub u8);
impl ERetargetSourceOrTarget {
    pub const SOURCE: ERetargetSourceOrTarget = ERetargetSourceOrTarget(0);
    pub const TARGET: ERetargetSourceOrTarget = ERetargetSourceOrTarget(1);
}
#[repr(transparent)]
pub struct EPinBoneTranslationMode(pub u8);
impl EPinBoneTranslationMode {
    pub const COPY_GLOBAL_POSITION: EPinBoneTranslationMode = EPinBoneTranslationMode(0);
    pub const COPY_LOCAL_POSITION: EPinBoneTranslationMode = EPinBoneTranslationMode(1);
    pub const COPY_LOCAL_POSITION_RELATIVE_OFFSET: EPinBoneTranslationMode = EPinBoneTranslationMode(
        2,
    );
    pub const COPY_LOCAL_POSITION_RELATIVE_SCALED: EPinBoneTranslationMode = EPinBoneTranslationMode(
        3,
    );
    pub const COPY_GLOBAL_POSITION_AND_MAINTAIN_OFFSET: EPinBoneTranslationMode = EPinBoneTranslationMode(
        4,
    );
}
#[repr(transparent)]
pub struct EPinBoneRotationMode(pub u8);
impl EPinBoneRotationMode {
    pub const COPY_GLOBAL_ROTATION: EPinBoneRotationMode = EPinBoneRotationMode(0);
    pub const MAINTAIN_OFFSET_FROM_BONE_TO_COPY_FROM: EPinBoneRotationMode = EPinBoneRotationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ERootMotionSource(pub u8);
impl ERootMotionSource {
    pub const COPY_FROM_SOURCE_ROOT: ERootMotionSource = ERootMotionSource(0);
    pub const GENERATE_FROM_TARGET_PELVIS: ERootMotionSource = ERootMotionSource(1);
}
#[repr(transparent)]
pub struct ERootMotionHeightSource(pub u8);
impl ERootMotionHeightSource {
    pub const COPY_HEIGHT_FROM_SOURCE: ERootMotionHeightSource = ERootMotionHeightSource(
        0,
    );
    pub const SNAP_TO_GROUND: ERootMotionHeightSource = ERootMotionHeightSource(1);
}
#[repr(transparent)]
pub struct EScaleSourcePivot(pub u8);
impl EScaleSourcePivot {
    pub const COMPONENT_ORIGIN: EScaleSourcePivot = EScaleSourcePivot(0);
    pub const BONE: EScaleSourcePivot = EScaleSourcePivot(1);
}
#[repr(transparent)]
pub struct EStretchLimbRotationMode(pub u8);
impl EStretchLimbRotationMode {
    pub const NONE: EStretchLimbRotationMode = EStretchLimbRotationMode(0);
    pub const ORIENT_TO_GOAL: EStretchLimbRotationMode = EStretchLimbRotationMode(1);
}
#[repr(transparent)]
pub struct EStretchLimbSquashMode(pub u8);
impl EStretchLimbSquashMode {
    pub const NONE: EStretchLimbSquashMode = EStretchLimbSquashMode(0);
    pub const UNIFORM: EStretchLimbSquashMode = EStretchLimbSquashMode(1);
    pub const BULGE: EStretchLimbSquashMode = EStretchLimbSquashMode(2);
}
#[repr(transparent)]
pub struct EAutoMapChainType(pub u8);
impl EAutoMapChainType {
    pub const EXACT: EAutoMapChainType = EAutoMapChainType(0);
    pub const FUZZY: EAutoMapChainType = EAutoMapChainType(1);
    pub const CLEAR: EAutoMapChainType = EAutoMapChainType(2);
}
#[repr(transparent)]
pub struct EPinBoneType(pub u8);
impl EPinBoneType {
    pub const FULL_TRANSFORM: EPinBoneType = EPinBoneType(0);
    pub const TRANSLATE_ONLY: EPinBoneType = EPinBoneType(1);
    pub const ROTATE_ONLY: EPinBoneType = EPinBoneType(2);
    pub const SCALE_ONLY: EPinBoneType = EPinBoneType(3);
}
#[repr(transparent)]
pub struct EIKRigGoalPreviewMode(pub u8);
impl EIKRigGoalPreviewMode {
    pub const ADDITIVE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(0);
    pub const ABSOLUTE: EIKRigGoalPreviewMode = EIKRigGoalPreviewMode(1);
}
