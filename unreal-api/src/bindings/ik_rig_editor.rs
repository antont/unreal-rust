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
    pub uik_retarget_batch_operation_duplicate_and_retarget: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_snap_bone_to_ground: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_source_chain: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_rotation_offset_for_retarget_pose_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_root_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_root_offset_in_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_retarget_op_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_retarget_chain_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_parent_op_by_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_op_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_ik_rig: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_global_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_set_current_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_run_op_initial_setup: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_reset_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_reset_chain_settings_to_default: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_reset_chain_settings_in_all_ops: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_rename_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_remove_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_remove_retarget_op: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_remove_all_ops: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_move_retarget_op_in_stack: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_target_ik_rig_for_op: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_source_chain: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_rotation_offset_for_retarget_pose_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_root_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_root_offset_in_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_retarget_poses: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_retarget_op_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_retarget_chain_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_parent_op_by_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_op_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_op_controller: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_num_retarget_ops: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_index_of_op_by_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_ik_rig: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_global_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_current_retarget_pose_name: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_current_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_controller: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_all_target_ik_rigs: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_get_all_chain_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_duplicate_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_create_retarget_pose: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_auto_map_chains: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_auto_align_bones: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_auto_align_all_bones: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_assign_ik_rig_to_all_ops: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_add_retarget_op: *mut crate::ffi::UFunctionOpague,
    pub uik_retargeter_controller_add_default_ops: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_start_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_solver_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_retarget_root: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_retarget_chain_start_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_retarget_chain_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_retarget_chain_end_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_goal_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_end_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_set_bone_excluded: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_rename_retarget_chain: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_rename_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_remove_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_remove_retarget_chain: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_remove_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_remove_bone_setting: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_move_solver_in_stack: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_is_skeletal_mesh_compatible: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_is_goal_connected_to_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_is_goal_connected_to_any_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_start_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_solver_enabled: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_solver_controller: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_retarget_root: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_retarget_chain_start_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_retarget_chains: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_retarget_chain_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_retarget_chain_end_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_ref_pose_transform_of_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_num_solvers: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_index_of_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_goal_settings_for_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_goal_name_for_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_end_bone: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_controller: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_bone_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_bone_for_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_bone_excluded: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_get_all_goals: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_does_solver_have_custom_goal_settings: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_disconnect_goal_from_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_connect_goal_to_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_apply_auto_generated_retarget_definition: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_apply_auto_fbik: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_add_solver: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_add_retarget_chain: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_add_new_goal: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_controller_add_bone_setting: *mut crate::ffi::UFunctionOpague,
    pub uik_rig_definition_factory_create_new_ik_rig_asset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            uik_retarget_batch_operation_duplicate_and_retarget: std::ptr::null_mut(),
            uik_retargeter_controller_snap_bone_to_ground: std::ptr::null_mut(),
            uik_retargeter_controller_set_source_chain: std::ptr::null_mut(),
            uik_retargeter_controller_set_rotation_offset_for_retarget_pose_bone: std::ptr::null_mut(),
            uik_retargeter_controller_set_root_settings: std::ptr::null_mut(),
            uik_retargeter_controller_set_root_offset_in_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_set_retarget_op_enabled: std::ptr::null_mut(),
            uik_retargeter_controller_set_retarget_chain_settings: std::ptr::null_mut(),
            uik_retargeter_controller_set_preview_mesh: std::ptr::null_mut(),
            uik_retargeter_controller_set_parent_op_by_name: std::ptr::null_mut(),
            uik_retargeter_controller_set_op_name: std::ptr::null_mut(),
            uik_retargeter_controller_set_ik_rig: std::ptr::null_mut(),
            uik_retargeter_controller_set_global_settings: std::ptr::null_mut(),
            uik_retargeter_controller_set_current_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_run_op_initial_setup: std::ptr::null_mut(),
            uik_retargeter_controller_reset_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_reset_chain_settings_to_default: std::ptr::null_mut(),
            uik_retargeter_controller_reset_chain_settings_in_all_ops: std::ptr::null_mut(),
            uik_retargeter_controller_rename_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_remove_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_remove_retarget_op: std::ptr::null_mut(),
            uik_retargeter_controller_remove_all_ops: std::ptr::null_mut(),
            uik_retargeter_controller_move_retarget_op_in_stack: std::ptr::null_mut(),
            uik_retargeter_controller_get_target_ik_rig_for_op: std::ptr::null_mut(),
            uik_retargeter_controller_get_source_chain: std::ptr::null_mut(),
            uik_retargeter_controller_get_rotation_offset_for_retarget_pose_bone: std::ptr::null_mut(),
            uik_retargeter_controller_get_root_settings: std::ptr::null_mut(),
            uik_retargeter_controller_get_root_offset_in_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_get_retarget_poses: std::ptr::null_mut(),
            uik_retargeter_controller_get_retarget_op_enabled: std::ptr::null_mut(),
            uik_retargeter_controller_get_retarget_chain_settings: std::ptr::null_mut(),
            uik_retargeter_controller_get_preview_mesh: std::ptr::null_mut(),
            uik_retargeter_controller_get_parent_op_by_name: std::ptr::null_mut(),
            uik_retargeter_controller_get_op_name: std::ptr::null_mut(),
            uik_retargeter_controller_get_op_controller: std::ptr::null_mut(),
            uik_retargeter_controller_get_num_retarget_ops: std::ptr::null_mut(),
            uik_retargeter_controller_get_index_of_op_by_name: std::ptr::null_mut(),
            uik_retargeter_controller_get_ik_rig: std::ptr::null_mut(),
            uik_retargeter_controller_get_global_settings: std::ptr::null_mut(),
            uik_retargeter_controller_get_current_retarget_pose_name: std::ptr::null_mut(),
            uik_retargeter_controller_get_current_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_get_controller: std::ptr::null_mut(),
            uik_retargeter_controller_get_all_target_ik_rigs: std::ptr::null_mut(),
            uik_retargeter_controller_get_all_chain_settings: std::ptr::null_mut(),
            uik_retargeter_controller_duplicate_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_create_retarget_pose: std::ptr::null_mut(),
            uik_retargeter_controller_auto_map_chains: std::ptr::null_mut(),
            uik_retargeter_controller_auto_align_bones: std::ptr::null_mut(),
            uik_retargeter_controller_auto_align_all_bones: std::ptr::null_mut(),
            uik_retargeter_controller_assign_ik_rig_to_all_ops: std::ptr::null_mut(),
            uik_retargeter_controller_add_retarget_op: std::ptr::null_mut(),
            uik_retargeter_controller_add_default_ops: std::ptr::null_mut(),
            uik_rig_controller_set_start_bone: std::ptr::null_mut(),
            uik_rig_controller_set_solver_enabled: std::ptr::null_mut(),
            uik_rig_controller_set_skeletal_mesh: std::ptr::null_mut(),
            uik_rig_controller_set_retarget_root: std::ptr::null_mut(),
            uik_rig_controller_set_retarget_chain_start_bone: std::ptr::null_mut(),
            uik_rig_controller_set_retarget_chain_goal: std::ptr::null_mut(),
            uik_rig_controller_set_retarget_chain_end_bone: std::ptr::null_mut(),
            uik_rig_controller_set_goal_bone: std::ptr::null_mut(),
            uik_rig_controller_set_end_bone: std::ptr::null_mut(),
            uik_rig_controller_set_bone_excluded: std::ptr::null_mut(),
            uik_rig_controller_rename_retarget_chain: std::ptr::null_mut(),
            uik_rig_controller_rename_goal: std::ptr::null_mut(),
            uik_rig_controller_remove_solver: std::ptr::null_mut(),
            uik_rig_controller_remove_retarget_chain: std::ptr::null_mut(),
            uik_rig_controller_remove_goal: std::ptr::null_mut(),
            uik_rig_controller_remove_bone_setting: std::ptr::null_mut(),
            uik_rig_controller_move_solver_in_stack: std::ptr::null_mut(),
            uik_rig_controller_is_skeletal_mesh_compatible: std::ptr::null_mut(),
            uik_rig_controller_is_goal_connected_to_solver: std::ptr::null_mut(),
            uik_rig_controller_is_goal_connected_to_any_solver: std::ptr::null_mut(),
            uik_rig_controller_get_start_bone: std::ptr::null_mut(),
            uik_rig_controller_get_solver_enabled: std::ptr::null_mut(),
            uik_rig_controller_get_solver_controller: std::ptr::null_mut(),
            uik_rig_controller_get_skeletal_mesh: std::ptr::null_mut(),
            uik_rig_controller_get_retarget_root: std::ptr::null_mut(),
            uik_rig_controller_get_retarget_chain_start_bone: std::ptr::null_mut(),
            uik_rig_controller_get_retarget_chains: std::ptr::null_mut(),
            uik_rig_controller_get_retarget_chain_goal: std::ptr::null_mut(),
            uik_rig_controller_get_retarget_chain_end_bone: std::ptr::null_mut(),
            uik_rig_controller_get_ref_pose_transform_of_bone: std::ptr::null_mut(),
            uik_rig_controller_get_num_solvers: std::ptr::null_mut(),
            uik_rig_controller_get_index_of_solver: std::ptr::null_mut(),
            uik_rig_controller_get_goal_settings_for_solver: std::ptr::null_mut(),
            uik_rig_controller_get_goal_name_for_bone: std::ptr::null_mut(),
            uik_rig_controller_get_goal: std::ptr::null_mut(),
            uik_rig_controller_get_end_bone: std::ptr::null_mut(),
            uik_rig_controller_get_controller: std::ptr::null_mut(),
            uik_rig_controller_get_bone_settings: std::ptr::null_mut(),
            uik_rig_controller_get_bone_for_goal: std::ptr::null_mut(),
            uik_rig_controller_get_bone_excluded: std::ptr::null_mut(),
            uik_rig_controller_get_all_goals: std::ptr::null_mut(),
            uik_rig_controller_does_solver_have_custom_goal_settings: std::ptr::null_mut(),
            uik_rig_controller_disconnect_goal_from_solver: std::ptr::null_mut(),
            uik_rig_controller_connect_goal_to_solver: std::ptr::null_mut(),
            uik_rig_controller_apply_auto_generated_retarget_definition: std::ptr::null_mut(),
            uik_rig_controller_apply_auto_fbik: std::ptr::null_mut(),
            uik_rig_controller_add_solver: std::ptr::null_mut(),
            uik_rig_controller_add_retarget_chain: std::ptr::null_mut(),
            uik_rig_controller_add_new_goal: std::ptr::null_mut(),
            uik_rig_controller_add_bone_setting: std::ptr::null_mut(),
            uik_rig_definition_factory_create_new_ik_rig_asset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargetBatchOperation::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAndRetarget"),
            &raw mut __FUNCTION_PTRS.uik_retarget_batch_operation_duplicate_and_retarget,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRetargeterController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapBoneToGround"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_snap_bone_to_ground,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceChain"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_source_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRotationOffsetForRetargetPoseBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_set_rotation_offset_for_retarget_pose_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootSettings"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_root_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootOffsetInRetargetPose"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_set_root_offset_in_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetOpEnabled"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_retarget_op_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetChainSettings"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_set_retarget_chain_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewMesh"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_preview_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParentOpByName"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_parent_op_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOpName"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_op_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIKRig"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_ik_rig,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalSettings"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_global_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurrentRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_set_current_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunOpInitialSetup"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_run_op_initial_setup,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_reset_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetChainSettingsToDefault"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_reset_chain_settings_to_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetChainSettingsInAllOps"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_reset_chain_settings_in_all_ops,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_rename_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_remove_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveRetargetOp"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_remove_retarget_op,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllOps"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_remove_all_ops,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveRetargetOpInStack"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_move_retarget_op_in_stack,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetIKRigForOp"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_target_ik_rig_for_op,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceChain"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_source_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRotationOffsetForRetargetPoseBone"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_get_rotation_offset_for_retarget_pose_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSettings"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_root_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootOffsetInRetargetPose"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_get_root_offset_in_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetPoses"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_retarget_poses,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetOpEnabled"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_retarget_op_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetChainSettings"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_get_retarget_chain_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewMesh"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_preview_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentOpByName"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_parent_op_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOpName"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_op_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOpController"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_op_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumRetargetOps"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_num_retarget_ops,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIndexOfOpByName"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_index_of_op_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIKRig"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_ik_rig,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalSettings"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_global_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentRetargetPoseName"),
            &raw mut __FUNCTION_PTRS
                .uik_retargeter_controller_get_current_retarget_pose_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_current_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetController"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllTargetIKRigs"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_all_target_ik_rigs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllChainSettings"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_get_all_chain_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_duplicate_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateRetargetPose"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_create_retarget_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AutoMapChains"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_auto_map_chains,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AutoAlignBones"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_auto_align_bones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AutoAlignAllBones"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_auto_align_all_bones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssignIKRigToAllOps"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_assign_ik_rig_to_all_ops,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRetargetOp"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_add_retarget_op,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDefaultOps"),
            &raw mut __FUNCTION_PTRS.uik_retargeter_controller_add_default_ops,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_start_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSolverEnabled"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_solver_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetRoot"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_retarget_root,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetChainStartBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_retarget_chain_start_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetChainGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_retarget_chain_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRetargetChainEndBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_retarget_chain_end_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGoalBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_goal_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_end_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoneExcluded"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_set_bone_excluded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameRetargetChain"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_rename_retarget_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_rename_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_remove_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveRetargetChain"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_remove_retarget_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_remove_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBoneSetting"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_remove_bone_setting,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveSolverInStack"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_move_solver_in_stack,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSkeletalMeshCompatible"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_is_skeletal_mesh_compatible,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGoalConnectedToSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_is_goal_connected_to_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGoalConnectedToAnySolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_is_goal_connected_to_any_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_start_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverEnabled"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_solver_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSolverController"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_solver_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetRoot"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_retarget_root,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetChainStartBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_retarget_chain_start_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetChains"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_retarget_chains,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetChainGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_retarget_chain_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRetargetChainEndBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_retarget_chain_end_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRefPoseTransformOfBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_ref_pose_transform_of_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSolvers"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_num_solvers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIndexOfSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_index_of_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGoalSettingsForSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_goal_settings_for_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGoalNameForBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_goal_name_for_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndBone"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_end_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetController"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_controller,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneSettings"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_bone_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneForGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_bone_for_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneExcluded"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_bone_excluded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllGoals"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_get_all_goals,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesSolverHaveCustomGoalSettings"),
            &raw mut __FUNCTION_PTRS
                .uik_rig_controller_does_solver_have_custom_goal_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectGoalFromSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_disconnect_goal_from_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectGoalToSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_connect_goal_to_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAutoGeneratedRetargetDefinition"),
            &raw mut __FUNCTION_PTRS
                .uik_rig_controller_apply_auto_generated_retarget_definition,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyAutoFBIK"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_apply_auto_fbik,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSolver"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_add_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRetargetChain"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_add_retarget_chain,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNewGoal"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_add_new_goal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBoneSetting"),
            &raw mut __FUNCTION_PTRS.uik_rig_controller_add_bone_setting,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UIKRigDefinitionFactory::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewIKRigAsset"),
            &raw mut __FUNCTION_PTRS.uik_rig_definition_factory_create_new_ik_rig_asset,
        );
    }
}
#[repr(C, align(8))]
pub struct FAnimNode_PreviewRetargetPose {
    pub(crate) __padding_end: [u8; 224],
}
impl FAnimNode_PreviewRetargetPose {}
#[repr(C, align(16))]
pub struct UIKRigStructViewer {
    __padding_end: [u8; 208],
}
impl UIKRigStructViewer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigStructViewer")
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
#[repr(C, align(16))]
pub struct UIKRigStructWrapperBase {
    __padding_end: [u8; 256],
}
impl UIKRigStructWrapperBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigStructWrapperBase")
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
#[repr(C, align(16))]
pub struct UBodyMoverSettingsWrapper {
    __padding_end: [u8; 320],
}
impl UBodyMoverSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBodyMoverSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UBodyMoverGoalSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UBodyMoverGoalSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBodyMoverGoalSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UFBIKSettingsWrapper {
    __padding_end: [u8; 336],
}
impl UFBIKSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFBIKSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UFBIKGoalSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UFBIKGoalSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFBIKGoalSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UFBIKBoneSettingsWrapper {
    __padding_end: [u8; 352],
}
impl UFBIKBoneSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFBIKBoneSettingsWrapper")
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
#[repr(C, align(16))]
pub struct ULimbSolverSettingsWrapper {
    __padding_end: [u8; 336],
}
impl ULimbSolverSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULimbSolverSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UPoleSolverSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UPoleSolverSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoleSolverSettingsWrapper")
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
#[repr(C, align(16))]
pub struct USetTransformSettingsWrapper {
    __padding_end: [u8; 304],
}
impl USetTransformSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetTransformSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UStretchLimbSettingsWrapper {
    __padding_end: [u8; 352],
}
impl UStretchLimbSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchLimbSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UStretchLimbBoneSettingsWrapper {
    __padding_end: [u8; 304],
}
impl UStretchLimbBoneSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchLimbBoneSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UIKRetargetAnimInstance {
    __padding_end: [u8; 2512],
}
impl UIKRetargetAnimInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetAnimInstance")
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
pub struct UIKRetargetBatchOperation {
    __padding_end: [u8; 320],
}
impl UIKRetargetBatchOperation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetBatchOperation")
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
    pub fn duplicate_and_retarget(
        assets_to_retarget: &TArray<crate::bindings::core_u_object::FAssetData>,
        source_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        target_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        ik_retarget_asset: UPtr<crate::bindings::ik_rig::UIKRetargeter>,
        search: FString,
        replace: FString,
        prefix: FString,
        suffix: FString,
        b_include_referenced_assets: bool,
        b_overwrite_existing_files: bool,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retarget_batch_operation_duplicate_and_retarget,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_retarget,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_mesh,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ik_retarget_asset,
                __buffer.add(32).cast::<UPtr<crate::bindings::ik_rig::UIKRetargeter>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replace,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prefix,
                __buffer.add(72).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &suffix,
                __buffer.add(88).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_referenced_assets,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_overwrite_existing_files,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig_editor::UIKRetargetBatchOperation::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retarget_batch_operation_duplicate_and_retarget,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
}
#[repr(C, align(16))]
pub struct UIKRetargetBoneDetails {
    __padding_end: [u8; 480],
}
impl UIKRetargetBoneDetails {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetBoneDetails")
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
pub struct UIKRetargeterController {
    __padding_end: [u8; 248],
}
impl UIKRetargeterController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargeterController")
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
    pub fn snap_bone_to_ground(
        &mut self,
        reference_bone: FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_snap_bone_to_ground,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_bone,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_snap_bone_to_ground,
                __buffer,
            )
        };
    }
    pub fn set_source_chain(
        &self,
        in_source_chain_name: FName,
        in_target_chain_name: FName,
        in_op_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_source_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_chain_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_source_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn set_rotation_offset_for_retarget_pose_bone(
        &self,
        bone_name: &FName,
        rotation_offset: &crate::bindings::core_u_object::FQuat,
        skeleton_mode: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_rotation_offset_for_retarget_pose_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(bone_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation_offset,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton_mode,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_rotation_offset_for_retarget_pose_bone,
                __buffer,
            )
        };
    }
    pub fn set_root_settings(
        &self,
        root_settings: &crate::bindings::ik_rig::FTargetRootSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_root_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_settings,
                __buffer.add(0).cast::<crate::bindings::ik_rig::FTargetRootSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_root_settings,
                __buffer,
            )
        };
    }
    pub fn set_root_offset_in_retarget_pose(
        &self,
        translation_offset: &crate::bindings::core_u_object::FVector,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_root_offset_in_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                translation_offset,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_root_offset_in_retarget_pose,
                __buffer,
            )
        };
    }
    pub fn set_retarget_op_enabled(
        &self,
        in_retarget_op_index: i32,
        b_is_enabled: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_retarget_op_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_retarget_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_retarget_op_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_retarget_chain_settings(
        &self,
        target_chain_name: &FName,
        settings: &crate::bindings::ik_rig::FTargetChainSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<209>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_retarget_chain_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                settings,
                __buffer.add(16).cast::<crate::bindings::ik_rig::FTargetChainSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_retarget_chain_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(208).cast::<bool>().read() }
    }
    pub fn set_preview_mesh(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
        in_preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_preview_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_preview_mesh,
                __buffer,
            )
        };
    }
    pub fn set_parent_op_by_name(
        &self,
        in_child_op_name: FName,
        in_parent_op_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_parent_op_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child_op_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_parent_op_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_op_name(&self, in_name: FName, in_op_index: i32) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_op_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_op_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn set_ik_rig(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
        ik_rig: UPtr<crate::bindings::ik_rig::UIKRigDefinition>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_ik_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ik_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_ik_rig,
                __buffer,
            )
        };
    }
    pub fn set_global_settings(
        &self,
        global_settings: &crate::bindings::ik_rig::FRetargetGlobalSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_global_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_settings,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::FRetargetGlobalSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_global_settings,
                __buffer,
            )
        };
    }
    pub fn set_current_retarget_pose(
        &self,
        current_pose: FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_current_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &current_pose,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_set_current_retarget_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn run_op_initial_setup(&self, in_op_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_run_op_initial_setup,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_run_op_initial_setup,
                __buffer,
            )
        };
    }
    pub fn reset_retarget_pose(
        &self,
        pose_to_reset: &FName,
        bones_to_reset: &TArray<FName>,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose_to_reset,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                bones_to_reset,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_retarget_pose,
                __buffer,
            )
        };
    }
    pub fn reset_chain_settings_to_default(
        &self,
        in_target_chain_name: FName,
        in_op_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_chain_settings_to_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_chain_settings_to_default,
                __buffer,
            )
        };
    }
    pub fn reset_chain_settings_in_all_ops(&self, in_target_chain_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_chain_settings_in_all_ops,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_reset_chain_settings_in_all_ops,
                __buffer,
            )
        };
    }
    pub fn rename_retarget_pose(
        &self,
        old_pose_name: FName,
        new_pose_name: FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_rename_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_pose_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_pose_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_rename_retarget_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn remove_retarget_pose(
        &self,
        pose_to_remove: &FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose_to_remove,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_retarget_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn remove_retarget_op(&self, in_op_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_retarget_op,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_retarget_op,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn remove_all_ops(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_all_ops,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_remove_all_ops,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn move_retarget_op_in_stack(
        &self,
        in_op_to_move_index: i32,
        in_target_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_move_retarget_op_in_stack,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_to_move_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_move_retarget_op_in_stack,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_target_ik_rig_for_op(
        &self,
        in_op_name: FName,
    ) -> UPtr<crate::bindings::ik_rig::UIKRigDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_target_ik_rig_for_op,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_target_ik_rig_for_op,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>()
                .read()
        }
    }
    pub fn get_source_chain(
        &self,
        in_target_chain_name: &FName,
        in_op_name: FName,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_source_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_source_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn get_rotation_offset_for_retarget_pose_bone(
        &self,
        bone_name: &FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_rotation_offset_for_retarget_pose_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(bone_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_rotation_offset_for_retarget_pose_bone,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_root_settings(&self) -> crate::bindings::ik_rig::FTargetRootSettings {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_root_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_root_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::ik_rig::FTargetRootSettings>().read()
        }
    }
    pub fn get_root_offset_in_retarget_pose(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_root_offset_in_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_root_offset_in_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_retarget_poses(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> TMap<FName, crate::bindings::ik_rig::FIKRetargetPose> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_poses,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_poses,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TMap<FName, crate::bindings::ik_rig::FIKRetargetPose>>()
                .read()
        }
    }
    pub fn get_retarget_op_enabled(&self, in_retarget_op_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_op_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_retarget_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_op_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_retarget_chain_settings(
        &self,
        target_chain_name: &FName,
    ) -> crate::bindings::ik_rig::FTargetChainSettings {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_chain_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_retarget_chain_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::ik_rig::FTargetChainSettings>()
                .read()
        }
    }
    pub fn get_preview_mesh(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_parent_op_by_name(&self, in_op_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_parent_op_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_parent_op_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_op_name(&self, in_op_index: i32) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_op_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_op_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FName>().read() }
    }
    pub fn get_op_controller(
        &mut self,
        in_op_index: i32,
    ) -> UPtr<crate::bindings::ik_rig::UIKRetargetOpControllerBase> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_op_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_op_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRetargetOpControllerBase>>()
                .read()
        }
    }
    pub fn get_num_retarget_ops(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_num_retarget_ops,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_num_retarget_ops,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_index_of_op_by_name(&self, in_op_name: FName) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_index_of_op_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_index_of_op_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_ik_rig(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> UPtr<crate::bindings::ik_rig::UIKRigDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_ik_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_ik_rig,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>()
                .read()
        }
    }
    pub fn get_global_settings(
        &self,
    ) -> crate::bindings::ik_rig::FRetargetGlobalSettings {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_global_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_global_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::ik_rig::FRetargetGlobalSettings>()
                .read()
        }
    }
    pub fn get_current_retarget_pose_name(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_current_retarget_pose_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_current_retarget_pose_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FName>().read() }
    }
    pub fn get_current_retarget_pose(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> crate::bindings::ik_rig::FIKRetargetPose {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_current_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_current_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::ik_rig::FIKRetargetPose>().read()
        }
    }
    pub fn get_controller(
        in_retargeter_asset: UPtr<crate::bindings::ik_rig::UIKRetargeter>,
    ) -> UPtr<UIKRetargeterController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_retargeter_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::ik_rig::UIKRetargeter>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig_editor::UIKRetargeterController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UIKRetargeterController>>().read() }
    }
    pub fn get_all_target_ik_rigs(
        &self,
    ) -> TArray<UPtr<crate::bindings::ik_rig::UIKRigDefinition>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_all_target_ik_rigs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_all_target_ik_rigs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>>()
                .read()
        }
    }
    pub fn get_all_chain_settings(
        &self,
    ) -> TArray<UPtr<crate::bindings::ik_rig::URetargetChainSettings>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_all_chain_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_get_all_chain_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::ik_rig::URetargetChainSettings>>>()
                .read()
        }
    }
    pub fn duplicate_retarget_pose(
        &self,
        pose_to_duplicate: FName,
        new_name: FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_duplicate_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pose_to_duplicate,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_duplicate_retarget_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<FName>().read() }
    }
    pub fn create_retarget_pose(
        &self,
        new_pose_name: &FName,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_create_retarget_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_pose_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_create_retarget_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn auto_map_chains(
        &self,
        auto_map_type: crate::bindings::ik_rig::EAutoMapChainType,
        b_force_remap: bool,
        in_op_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_map_chains,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &auto_map_type,
                __buffer.add(0).cast::<crate::bindings::ik_rig::EAutoMapChainType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_remap,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_name,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_map_chains,
                __buffer,
            )
        };
    }
    pub fn auto_align_bones(
        &self,
        bones_to_align: &TArray<FName>,
        method: ERetargetAutoAlignMethod,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_align_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                bones_to_align,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &method,
                __buffer.add(16).cast::<ERetargetAutoAlignMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(17)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_align_bones,
                __buffer,
            )
        };
    }
    pub fn auto_align_all_bones(
        &self,
        source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
        method: ERetargetAutoAlignMethod,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_align_all_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &method,
                __buffer.add(1).cast::<ERetargetAutoAlignMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_auto_align_all_bones,
                __buffer,
            )
        };
    }
    pub fn assign_ik_rig_to_all_ops(
        &self,
        in_source_or_target: crate::bindings::ik_rig::ERetargetSourceOrTarget,
        in_ik_rig: UPtr<crate::bindings::ik_rig::UIKRigDefinition>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_assign_ik_rig_to_all_ops,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_or_target,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::ik_rig::ERetargetSourceOrTarget>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ik_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_assign_ik_rig_to_all_ops,
                __buffer,
            )
        };
    }
    pub fn add_retarget_op(&self, in_ik_retarget_op_type: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_add_retarget_op,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ik_retarget_op_type,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_add_retarget_op,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn add_default_ops(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_add_default_ops,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_retargeter_controller_add_default_ops,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UIKRetargeterThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UIKRetargeterThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargeterThumbnailRenderer")
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
pub struct UIKRetargetFactory {
    __padding_end: [u8; 144],
}
impl UIKRetargetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRetargetFactory")
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
#[repr(C, align(16))]
pub struct URetargetFKChainSettingsWrapper {
    __padding_end: [u8; 288],
}
impl URetargetFKChainSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetFKChainSettingsWrapper")
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
#[repr(C, align(16))]
pub struct URetargetIKChainSettingsWrapper {
    __padding_end: [u8; 416],
}
impl URetargetIKChainSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetIKChainSettingsWrapper")
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
#[repr(C, align(16))]
pub struct URetargetStrideWarpSettingsWrapper {
    __padding_end: [u8; 272],
}
impl URetargetStrideWarpSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetStrideWarpSettingsWrapper")
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
#[repr(C, align(16))]
pub struct URetargetSpeedPlantSettingsWrapper {
    __padding_end: [u8; 272],
}
impl URetargetSpeedPlantSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetSpeedPlantSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UPoleVectorSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UPoleVectorSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoleVectorSettingsWrapper")
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
#[repr(C, align(16))]
pub struct URetargetPoseOpSettingsWrapper {
    __padding_end: [u8; 336],
}
impl URetargetPoseOpSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetargetPoseOpSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UStretchChainSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UStretchChainSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStretchChainSettingsWrapper")
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
#[repr(C, align(16))]
pub struct UFloorConstraintSettingsWrapper {
    __padding_end: [u8; 288],
}
impl UFloorConstraintSettingsWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloorConstraintSettingsWrapper")
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
pub struct UBatchExportOptions {
    #[doc(hidden)]
    pub(crate) __padding_49: [u8; 49],
    pub b_include_referenced_assets: bool,
    pub b_retain_additive_flags: bool,
}
impl UBatchExportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBatchExportOptions")
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
pub struct UBatchRetargetSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_auto_generate_retargeter: bool,
    pub retarget_asset: UPtr<crate::bindings::ik_rig::UIKRetargeter>,
}
impl UBatchRetargetSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBatchRetargetSettings")
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
#[repr(C, align(16))]
pub struct UIKRigAnimInstance {
    __padding_end: [u8; 2112],
}
impl UIKRigAnimInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigAnimInstance")
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
pub struct UIKRigController {
    __padding_end: [u8; 296],
}
impl UIKRigController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigController")
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
    pub fn set_start_bone(&self, root_bone_name: FName, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_start_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &root_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_start_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_solver_enabled(&self, solver_index: i32, b_is_enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_solver_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_solver_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_skeletal_mesh(
        &self,
        skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_skeletal_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_retarget_root(&self, root_bone_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &root_bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_root,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_retarget_chain_start_bone(
        &self,
        chain_name: FName,
        start_bone_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_start_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_start_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_retarget_chain_goal(&self, chain_name: FName, goal_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_goal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_retarget_chain_end_bone(
        &self,
        chain_name: FName,
        end_bone_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_end_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_retarget_chain_end_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_goal_bone(&self, goal_name: FName, new_bone_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_goal_bone,
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
                &new_bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_goal_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_end_bone(&self, end_bone_name: FName, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_end_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_end_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_bone_excluded(&self, bone_name: FName, b_exclude: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_bone_excluded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exclude,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_set_bone_excluded,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn rename_retarget_chain(
        &self,
        chain_name: FName,
        new_chain_name: FName,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_rename_retarget_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_rename_retarget_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn rename_goal(&self, old_name: FName, potential_new_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_rename_goal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&old_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &potential_new_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_rename_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn remove_solver(&self, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_solver,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn remove_retarget_chain(&self, chain_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_retarget_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_retarget_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_goal(&self, goal_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_goal,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_bone_setting(&self, bone_name: FName, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_bone_setting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_remove_bone_setting,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn move_solver_in_stack(
        &self,
        solver_to_move_index: i32,
        target_solver_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_move_solver_in_stack,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_to_move_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_solver_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_move_solver_in_stack,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_skeletal_mesh_compatible(
        &self,
        skeletal_mesh_to_check: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_skeletal_mesh_compatible,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_to_check,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_skeletal_mesh_compatible,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_goal_connected_to_solver(
        &self,
        goal_name: FName,
        solver_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_goal_connected_to_solver,
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
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_goal_connected_to_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_goal_connected_to_any_solver(&self, goal_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_goal_connected_to_any_solver,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_is_goal_connected_to_any_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_start_bone(&self, solver_index: i32) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_start_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_start_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FName>().read() }
    }
    pub fn get_solver_enabled(&self, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_solver_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_solver_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_solver_controller(
        &mut self,
        solver_index: i32,
    ) -> UPtr<crate::bindings::ik_rig::UIKRigSolverControllerBase> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_solver_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_solver_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRigSolverControllerBase>>()
                .read()
        }
    }
    pub fn get_skeletal_mesh(&self) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_retarget_root(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_root,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_root,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_retarget_chain_start_bone(&self, chain_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_start_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_start_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_retarget_chains(&self) -> TArray<crate::bindings::ik_rig::FBoneChain> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chains,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chains,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<crate::bindings::ik_rig::FBoneChain>>().read()
        }
    }
    pub fn get_retarget_chain_goal(&self, chain_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_goal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_retarget_chain_end_bone(&self, chain_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_end_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_retarget_chain_end_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_ref_pose_transform_of_bone(
        &self,
        bone_name: FName,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_ref_pose_transform_of_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_ref_pose_transform_of_bone,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_num_solvers(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_num_solvers,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_num_solvers,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_index_of_solver(
        &mut self,
        in_controller: UPtr<crate::bindings::ik_rig::UIKRigSolverControllerBase>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_index_of_solver,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::ik_rig::UIKRigSolverControllerBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_index_of_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_goal_settings_for_solver(
        &self,
        goal_name: FName,
        solver_index: i32,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal_settings_for_solver,
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
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal_settings_for_solver,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_goal_name_for_bone(&self, bone_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal_name_for_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal_name_for_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_goal(
        &self,
        goal_name: FName,
    ) -> UPtr<crate::bindings::ik_rig::UIKRigEffectorGoal> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_goal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRigEffectorGoal>>()
                .read()
        }
    }
    pub fn get_end_bone(&self, solver_index: i32) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_end_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_end_bone,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FName>().read() }
    }
    pub fn get_controller(
        in_ik_rig_definition: UPtr<crate::bindings::ik_rig::UIKRigDefinition>,
    ) -> UPtr<UIKRigController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ik_rig_definition,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig_editor::UIKRigController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UIKRigController>>().read() }
    }
    pub fn get_bone_settings(
        &self,
        bone_name: FName,
        solver_index: i32,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_bone_for_goal(&self, goal_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_for_goal,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_for_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_bone_excluded(&self, bone_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_excluded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_bone_excluded,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_all_goals(
        &self,
    ) -> TArray<UPtr<crate::bindings::ik_rig::UIKRigEffectorGoal>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_all_goals,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_get_all_goals,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::ik_rig::UIKRigEffectorGoal>>>()
                .read()
        }
    }
    pub fn does_solver_have_custom_goal_settings(
        &self,
        goal_name: FName,
        solver_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_does_solver_have_custom_goal_settings,
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
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_does_solver_have_custom_goal_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn disconnect_goal_from_solver(
        &self,
        goal_to_remove: FName,
        solver_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_disconnect_goal_from_solver,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_to_remove,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_disconnect_goal_from_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn connect_goal_to_solver(&self, goal_name: FName, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_connect_goal_to_solver,
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
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_connect_goal_to_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn apply_auto_generated_retarget_definition(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_apply_auto_generated_retarget_definition,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_apply_auto_generated_retarget_definition,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn apply_auto_fbik(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_apply_auto_fbik,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_apply_auto_fbik,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_solver(&self, in_ik_rig_solver_type: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_solver,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ik_rig_solver_type,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_solver,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn add_retarget_chain(
        &self,
        chain_name: FName,
        start_bone_name: FName,
        end_bone_name: FName,
        goal_name: FName,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_retarget_chain,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &chain_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_bone_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_bone_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_name,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_retarget_chain,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FName>().read() }
    }
    pub fn add_new_goal(&self, goal_name: FName, bone_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_new_goal,
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
                &bone_name,
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
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_new_goal,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FName>().read() }
    }
    pub fn add_bone_setting(&self, bone_name: FName, solver_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_bone_setting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &solver_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_controller_add_bone_setting,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UIKRigDefinitionFactory {
    __padding_end: [u8; 136],
}
impl UIKRigDefinitionFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigDefinitionFactory")
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
    pub fn create_new_ik_rig_asset(
        in_package_path: FString,
        in_asset_name: FString,
    ) -> UPtr<crate::bindings::ik_rig::UIKRigDefinition> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_definition_factory_create_new_ik_rig_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_package_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ik_rig_editor::UIKRigDefinitionFactory::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ik_rig_editor::__FUNCTION_PTRS
                    .uik_rig_definition_factory_create_new_ik_rig_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::ik_rig::UIKRigDefinition>>()
                .read()
        }
    }
}
#[repr(C, align(16))]
pub struct UIKRigBoneDetails {
    __padding_end: [u8; 272],
}
impl UIKRigBoneDetails {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigBoneDetails")
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
pub struct UIKRigThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UIKRigThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIKRigThumbnailRenderer")
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
pub struct ERetargetAutoAlignMethod(pub u8);
impl ERetargetAutoAlignMethod {
    pub const CHAIN_TO_CHAIN: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(0);
    pub const MESH_TO_MESH: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(1);
    pub const LOCAL_ROTATION_AXES: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(
        2,
    );
    pub const GLOBAL_ROTATION_AXES: ERetargetAutoAlignMethod = ERetargetAutoAlignMethod(
        3,
    );
}
