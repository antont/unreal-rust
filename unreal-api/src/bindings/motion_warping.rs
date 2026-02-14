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
    pub u_anim_notify_state_motion_warping_on_warp_update: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_on_warp_end: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_on_warp_begin: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_on_root_motion_modifier_update: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_on_root_motion_modifier_deactivate: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_on_root_motion_modifier_activate: *mut crate::ffi::UFunctionOpague,
    pub u_anim_notify_state_motion_warping_add_root_motion_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_utilities_get_motion_warping_windows_from_animation: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_utilities_get_motion_warping_windows_for_warp_target_from_animation: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_utilities_extract_root_motion_from_animation: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_utilities_extract_bone_transform_from_animation_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_remove_warp_targets: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_remove_warp_target: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_remove_all_warp_targets: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_disable_all_root_motion_modifiers: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_switch_off_condition: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_or_update_warp_target_from_transform: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_or_update_warp_target_from_location_and_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_or_update_warp_target_from_location: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_or_update_warp_target_from_component: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_component_add_or_update_warp_target: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_function_library_make_motion_warping_target: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_distance_condition_create_switch_off_distance_condition: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_angle_to_target_condition_create_switch_off_angle_to_target_condition: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_composite_condition_create_switch_off_composite_condition: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_blueprintable_condition_create_switch_off_blueprintable_condition: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_blueprintable_condition_bp_extra_debug_info: *mut crate::ffi::UFunctionOpague,
    pub u_motion_warping_switch_off_blueprintable_condition_bp_check: *mut crate::ffi::UFunctionOpague,
    pub u_root_motion_modifier_scale_add_root_motion_modifier_scale: *mut crate::ffi::UFunctionOpague,
    pub u_root_motion_modifier_precomputed_warp_set_align_offset: *mut crate::ffi::UFunctionOpague,
    pub u_root_motion_modifier_skew_warp_add_root_motion_modifier_skew_warp: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_anim_notify_state_motion_warping_on_warp_update: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_on_warp_end: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_on_warp_begin: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_on_root_motion_modifier_update: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_on_root_motion_modifier_deactivate: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_on_root_motion_modifier_activate: std::ptr::null_mut(),
            u_anim_notify_state_motion_warping_add_root_motion_modifier: std::ptr::null_mut(),
            u_motion_warping_utilities_get_motion_warping_windows_from_animation: std::ptr::null_mut(),
            u_motion_warping_utilities_get_motion_warping_windows_for_warp_target_from_animation: std::ptr::null_mut(),
            u_motion_warping_utilities_extract_root_motion_from_animation: std::ptr::null_mut(),
            u_motion_warping_utilities_extract_bone_transform_from_animation_at_time: std::ptr::null_mut(),
            u_motion_warping_component_remove_warp_targets: std::ptr::null_mut(),
            u_motion_warping_component_remove_warp_target: std::ptr::null_mut(),
            u_motion_warping_component_remove_all_warp_targets: std::ptr::null_mut(),
            u_motion_warping_component_disable_all_root_motion_modifiers: std::ptr::null_mut(),
            u_motion_warping_component_add_switch_off_condition: std::ptr::null_mut(),
            u_motion_warping_component_add_or_update_warp_target_from_transform: std::ptr::null_mut(),
            u_motion_warping_component_add_or_update_warp_target_from_location_and_rotation: std::ptr::null_mut(),
            u_motion_warping_component_add_or_update_warp_target_from_location: std::ptr::null_mut(),
            u_motion_warping_component_add_or_update_warp_target_from_component: std::ptr::null_mut(),
            u_motion_warping_component_add_or_update_warp_target: std::ptr::null_mut(),
            u_motion_warping_function_library_make_motion_warping_target: std::ptr::null_mut(),
            u_motion_warping_switch_off_distance_condition_create_switch_off_distance_condition: std::ptr::null_mut(),
            u_motion_warping_switch_off_angle_to_target_condition_create_switch_off_angle_to_target_condition: std::ptr::null_mut(),
            u_motion_warping_switch_off_composite_condition_create_switch_off_composite_condition: std::ptr::null_mut(),
            u_motion_warping_switch_off_blueprintable_condition_create_switch_off_blueprintable_condition: std::ptr::null_mut(),
            u_motion_warping_switch_off_blueprintable_condition_bp_extra_debug_info: std::ptr::null_mut(),
            u_motion_warping_switch_off_blueprintable_condition_bp_check: std::ptr::null_mut(),
            u_root_motion_modifier_scale_add_root_motion_modifier_scale: std::ptr::null_mut(),
            u_root_motion_modifier_precomputed_warp_set_align_offset: std::ptr::null_mut(),
            u_root_motion_modifier_skew_warp_add_root_motion_modifier_skew_warp: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimNotifyState_MotionWarping::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnWarpUpdate"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnWarpEnd"),
                &raw mut __FUNCTION_PTRS.u_anim_notify_state_motion_warping_on_warp_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnWarpBegin"),
                &raw mut __FUNCTION_PTRS.u_anim_notify_state_motion_warping_on_warp_begin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRootMotionModifierUpdate"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_root_motion_modifier_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRootMotionModifierDeactivate"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_root_motion_modifier_deactivate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRootMotionModifierActivate"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_root_motion_modifier_activate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRootMotionModifier"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_add_root_motion_modifier,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingUtilities::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMotionWarpingWindowsFromAnimation"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_from_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetMotionWarpingWindowsForWarpTargetFromAnimation",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_for_warp_target_from_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExtractRootMotionFromAnimation"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_root_motion_from_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExtractBoneTransformFromAnimationAtTime"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_bone_transform_from_animation_at_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveWarpTargets"),
                &raw mut __FUNCTION_PTRS.u_motion_warping_component_remove_warp_targets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveWarpTarget"),
                &raw mut __FUNCTION_PTRS.u_motion_warping_component_remove_warp_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllWarpTargets"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_remove_all_warp_targets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DisableAllRootMotionModifiers"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_disable_all_root_motion_modifiers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSwitchOffCondition"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_switch_off_condition,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrUpdateWarpTargetFromTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "AddOrUpdateWarpTargetFromLocationAndRotation",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location_and_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrUpdateWarpTargetFromLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrUpdateWarpTargetFromComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOrUpdateWarpTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeMotionWarpingTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_function_library_make_motion_warping_target,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingSwitchOffDistanceCondition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSwitchOffDistanceCondition"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_distance_condition_create_switch_off_distance_condition,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingSwitchOffAngleToTargetCondition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSwitchOffAngleToTargetCondition"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_angle_to_target_condition_create_switch_off_angle_to_target_condition,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingSwitchOffCompositeCondition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSwitchOffCompositeCondition"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_composite_condition_create_switch_off_composite_condition,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMotionWarpingSwitchOffBlueprintableCondition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSwitchOffBlueprintableCondition"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_create_switch_off_blueprintable_condition,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ExtraDebugInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_extra_debug_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_Check"),
                &raw mut __FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_check,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URootMotionModifier_Scale::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRootMotionModifierScale"),
                &raw mut __FUNCTION_PTRS
                    .u_root_motion_modifier_scale_add_root_motion_modifier_scale,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URootMotionModifier_PrecomputedWarp::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAlignOffset"),
                &raw mut __FUNCTION_PTRS
                    .u_root_motion_modifier_precomputed_warp_set_align_offset,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URootMotionModifier_SkewWarp::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRootMotionModifierSkewWarp"),
                &raw mut __FUNCTION_PTRS
                    .u_root_motion_modifier_skew_warp_add_root_motion_modifier_skew_warp,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMotionWarpingWindowData {
    pub anim_notify: UPtr<UAnimNotifyState_MotionWarping>,
    pub start_time: f32,
    pub end_time: f32,
}
impl FMotionWarpingWindowData {}
#[repr(C, align(8))]
pub struct FSwitchOffConditionData {
    pub warp_target_name: FName,
    pub switch_off_conditions: TArray<UPtr<UMotionWarpingSwitchOffCondition>>,
}
impl FSwitchOffConditionData {}
#[repr(C, align(8))]
pub struct FMotionWarpingTarget {
    pub name: FName,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub bone_name: FName,
    pub b_follow_component: bool,
    pub location_offset_direction: EWarpTargetLocationOffsetDirection,
    pub location_offset: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FRotator,
    pub avatar_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub(crate) __padding_end: [u8; 88],
}
impl FMotionWarpingTarget {}
#[repr(C, align(8))]
pub struct UMotionWarpingBaseAdapter {
    __padding_end: [u8; 72],
}
impl UMotionWarpingBaseAdapter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingBaseAdapter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingBaseAdapter")
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
pub struct UAnimNotifyState_MotionWarping {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub root_motion_modifier: UPtr<URootMotionModifier>,
}
impl UAnimNotifyState_MotionWarping {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_MotionWarping")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotifyState_MotionWarping")
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
    pub fn on_warp_update(
        &self,
        motion_warping_comp: UPtr<UMotionWarpingComponent>,
        modifier: UPtr<URootMotionModifier>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_update,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier,
                __buffer.add(8).cast::<UPtr<URootMotionModifier>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_update,
                __buffer,
            )
        };
    }
    pub fn on_warp_end(
        &self,
        motion_warping_comp: UPtr<UMotionWarpingComponent>,
        modifier: UPtr<URootMotionModifier>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier,
                __buffer.add(8).cast::<UPtr<URootMotionModifier>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_end,
                __buffer,
            )
        };
    }
    pub fn on_warp_begin(
        &self,
        motion_warping_comp: UPtr<UMotionWarpingComponent>,
        modifier: UPtr<URootMotionModifier>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_begin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier,
                __buffer.add(8).cast::<UPtr<URootMotionModifier>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_on_warp_begin,
                __buffer,
            )
        };
    }
    pub fn add_root_motion_modifier(
        &self,
        motion_warping_comp: UPtr<UMotionWarpingComponent>,
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        start_time: f32,
        end_time: f32,
    ) -> UPtr<URootMotionModifier> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_add_root_motion_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_time, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_anim_notify_state_motion_warping_add_root_motion_modifier,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<URootMotionModifier>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAttributeBasedRootMotionComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub b_enable_root_motion: bool,
    __padding_end: [u8; 223],
}
impl UAttributeBasedRootMotionComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeBasedRootMotionComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeBasedRootMotionComponent")
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
pub struct UMotionWarpingCharacterAdapter {
    __padding_end: [u8; 80],
}
impl UMotionWarpingCharacterAdapter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingCharacterAdapter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingCharacterAdapter")
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
pub struct UMotionWarpingUtilities {
    __padding_end: [u8; 48],
}
impl UMotionWarpingUtilities {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingUtilities")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingUtilities")
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
    pub fn get_motion_warping_windows_from_animation(
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        out_windows: &mut TArray<FMotionWarpingWindowData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_from_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_windows,
                __buffer.add(8).cast::<TArray<FMotionWarpingWindowData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_from_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FMotionWarpingWindowData>>().swap(out_windows);
        }
    }
    pub fn get_motion_warping_windows_for_warp_target_from_animation(
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        warp_target_name: FName,
        out_windows: &mut TArray<FMotionWarpingWindowData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_for_warp_target_from_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_windows,
                __buffer.add(24).cast::<TArray<FMotionWarpingWindowData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_get_motion_warping_windows_for_warp_target_from_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<FMotionWarpingWindowData>>()
                .swap(out_windows);
        }
    }
    pub fn extract_root_motion_from_animation(
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        start_time: f32,
        end_time: f32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_root_motion_from_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_time, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_root_motion_from_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn extract_bone_transform_from_animation_at_time(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        time: f32,
        b_extract_root_motion: bool,
        bone_name: FName,
        b_local_space: bool,
        out_transform: &mut crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_bone_transform_from_animation_at_time,
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
                &animation,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extract_root_motion,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_local_space,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_transform,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingUtilities::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_utilities_extract_bone_transform_from_animation_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(out_transform);
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub b_search_for_windows_in_anims_within_montages: bool,
    __padding_end: [u8; 151],
}
impl UMotionWarpingComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingComponent")
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
    pub fn remove_warp_targets(&mut self, warp_target_names: &TArray<FName>) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_warp_targets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                warp_target_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_warp_targets,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn remove_warp_target(&mut self, warp_target_name: FName) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_warp_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
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
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_warp_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn remove_all_warp_targets(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_all_warp_targets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_remove_all_warp_targets,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn disable_all_root_motion_modifiers(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_disable_all_root_motion_modifiers,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_disable_all_root_motion_modifiers,
                __buffer,
            )
        };
    }
    pub fn add_switch_off_condition(
        &mut self,
        warp_target_name: FName,
        condition: UPtr<UMotionWarpingSwitchOffCondition>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_switch_off_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &condition,
                __buffer.add(16).cast::<UPtr<UMotionWarpingSwitchOffCondition>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_switch_off_condition,
                __buffer,
            )
        };
    }
    pub fn add_or_update_warp_target_from_transform(
        &mut self,
        warp_target_name: FName,
        target_transform: crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_transform,
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
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_transform,
                __buffer,
            )
        };
    }
    pub fn add_or_update_warp_target_from_location_and_rotation(
        &mut self,
        warp_target_name: FName,
        target_location: crate::bindings::core_u_object::FVector,
        target_rotation: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location_and_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_rotation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location_and_rotation,
                __buffer,
            )
        };
    }
    pub fn add_or_update_warp_target_from_location(
        &mut self,
        warp_target_name: FName,
        target_location: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_location,
                __buffer,
            )
        };
    }
    pub fn add_or_update_warp_target_from_component(
        &mut self,
        warp_target_name: FName,
        component: UPtr<crate::bindings::engine::USceneComponent>,
        bone_name: FName,
        b_follow_component: bool,
        location_offset_direction: EWarpTargetLocationOffsetDirection,
        location_offset: crate::bindings::core_u_object::FVector,
        rotation_offset: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &warp_target_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_follow_component,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location_offset_direction,
                __buffer.add(37).cast::<EWarpTargetLocationOffsetDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location_offset,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation_offset,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target_from_component,
                __buffer,
            )
        };
    }
    pub fn add_or_update_warp_target(&mut self, warp_target: &FMotionWarpingTarget) {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                warp_target,
                __buffer.add(0).cast::<FMotionWarpingTarget>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_component_add_or_update_warp_target,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UMotionWarpingFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingFunctionLibrary")
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
    pub fn make_motion_warping_target(
        name: FName,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        component: UPtr<crate::bindings::engine::USceneComponent>,
        bone_name: FName,
        b_follow_component: bool,
        location_offset_direction: EWarpTargetLocationOffsetDirection,
        avatar_actor: UPtr<crate::bindings::engine::AActor>,
        location_offset: crate::bindings::core_u_object::FVector,
        rotation_offset: crate::bindings::core_u_object::FRotator,
    ) -> FMotionWarpingTarget {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_function_library_make_motion_warping_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer
                    .add(64)
                    .cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(72).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_follow_component,
                __buffer.add(84).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location_offset_direction,
                __buffer.add(85).cast::<EWarpTargetLocationOffsetDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &avatar_actor,
                __buffer.add(88).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location_offset,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation_offset,
                __buffer.add(120).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_function_library_make_motion_warping_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FMotionWarpingTarget>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingSwitchOffCondition {
    __padding_end: [u8; 80],
}
impl UMotionWarpingSwitchOffCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffCondition")
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
pub struct UMotionWarpingSwitchOffDistanceCondition {
    __padding_end: [u8; 88],
}
impl UMotionWarpingSwitchOffDistanceCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffDistanceCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffDistanceCondition")
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
    pub fn create_switch_off_distance_condition(
        in_owner_actor: &mut UPtr<crate::bindings::engine::AActor>,
        in_effect: ESwitchOffConditionEffect,
        in_operator: ESwitchOffConditionDistanceOp,
        in_distance: f32,
        inb_use_warp_target_as_target_location: bool,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UMotionWarpingSwitchOffDistanceCondition> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_distance_condition_create_switch_off_distance_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect,
                __buffer.add(8).cast::<ESwitchOffConditionEffect>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_operator,
                __buffer.add(9).cast::<ESwitchOffConditionDistanceOp>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_distance,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &inb_use_warp_target_as_target_location,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingSwitchOffDistanceCondition::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_distance_condition_create_switch_off_distance_condition,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(in_owner_actor);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<UMotionWarpingSwitchOffDistanceCondition>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingSwitchOffAngleToTargetCondition {
    __padding_end: [u8; 96],
}
impl UMotionWarpingSwitchOffAngleToTargetCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffAngleToTargetCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffAngleToTargetCondition")
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
    pub fn create_switch_off_angle_to_target_condition(
        in_owner_actor: &mut UPtr<crate::bindings::engine::AActor>,
        in_effect: ESwitchOffConditionEffect,
        in_operator: ESwitchOffConditionAngleOp,
        in_angle: f32,
        b_in_ignore_z_axis: bool,
        b_in_use_warp_target_as_target_location: bool,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UMotionWarpingSwitchOffAngleToTargetCondition> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_angle_to_target_condition_create_switch_off_angle_to_target_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect,
                __buffer.add(8).cast::<ESwitchOffConditionEffect>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_operator,
                __buffer.add(9).cast::<ESwitchOffConditionAngleOp>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_angle, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_ignore_z_axis,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_warp_target_as_target_location,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingSwitchOffAngleToTargetCondition::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_angle_to_target_condition_create_switch_off_angle_to_target_condition,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(in_owner_actor);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<UMotionWarpingSwitchOffAngleToTargetCondition>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingSwitchOffCompositeCondition {
    __padding_end: [u8; 104],
}
impl UMotionWarpingSwitchOffCompositeCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffCompositeCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffCompositeCondition")
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
    pub fn create_switch_off_composite_condition(
        in_owner_actor: &mut UPtr<crate::bindings::engine::AActor>,
        in_effect: ESwitchOffConditionEffect,
        in_switch_off_condition_a: &mut UPtr<UMotionWarpingSwitchOffCondition>,
        in_logic_operator: ESwitchOffConditionCompositeOp,
        in_switch_off_condition_b: &mut UPtr<UMotionWarpingSwitchOffCondition>,
        b_in_use_warp_target_as_target_location: bool,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UMotionWarpingSwitchOffCompositeCondition> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_composite_condition_create_switch_off_composite_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect,
                __buffer.add(8).cast::<ESwitchOffConditionEffect>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_switch_off_condition_a,
                __buffer.add(16).cast::<UPtr<UMotionWarpingSwitchOffCondition>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_logic_operator,
                __buffer.add(24).cast::<ESwitchOffConditionCompositeOp>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_switch_off_condition_b,
                __buffer.add(32).cast::<UPtr<UMotionWarpingSwitchOffCondition>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_warp_target_as_target_location,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingSwitchOffCompositeCondition::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_composite_condition_create_switch_off_composite_condition,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(in_owner_actor);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<UMotionWarpingSwitchOffCondition>>()
                .swap(in_switch_off_condition_a);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<UMotionWarpingSwitchOffCondition>>()
                .swap(in_switch_off_condition_b);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<UMotionWarpingSwitchOffCompositeCondition>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingSwitchOffBlueprintableCondition {
    __padding_end: [u8; 80],
}
impl UMotionWarpingSwitchOffBlueprintableCondition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffBlueprintableCondition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingSwitchOffBlueprintableCondition")
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
    pub fn create_switch_off_blueprintable_condition(
        in_owner_actor: &mut UPtr<crate::bindings::engine::AActor>,
        in_effect: ESwitchOffConditionEffect,
        in_blueprintable_condition: TSubclassOf<
            UMotionWarpingSwitchOffBlueprintableCondition,
        >,
        b_in_use_warp_target_as_target_location: bool,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UMotionWarpingSwitchOffBlueprintableCondition> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_create_switch_off_blueprintable_condition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect,
                __buffer.add(8).cast::<ESwitchOffConditionEffect>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprintable_condition,
                __buffer
                    .add(16)
                    .cast::<
                        TSubclassOf<UMotionWarpingSwitchOffBlueprintableCondition>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_warp_target_as_target_location,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::UMotionWarpingSwitchOffBlueprintableCondition::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_create_switch_off_blueprintable_condition,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(in_owner_actor);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<UMotionWarpingSwitchOffBlueprintableCondition>>()
                .read()
        }
    }
    pub fn bp_extra_debug_info(
        &self,
        in_owner_actor: UPtr<crate::bindings::engine::AActor>,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
        in_target_location: crate::bindings::core_u_object::FVector,
        b_in_use_warp_target_as_target_location: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_extra_debug_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_warp_target_as_target_location,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_extra_debug_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
    }
    pub fn bp_check(
        &self,
        in_owner_actor: UPtr<crate::bindings::engine::AActor>,
        in_target_actor: UPtr<crate::bindings::engine::AActor>,
        in_target_location: crate::bindings::core_u_object::FVector,
        b_in_use_warp_target_as_target_location: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_check,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_use_warp_target_as_target_location,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_motion_warping_switch_off_blueprintable_condition_bp_check,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct URootMotionModifier {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub animation: TWeakObjectPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub start_time: f32,
    pub end_time: f32,
    pub previous_position: f32,
    pub current_position: f32,
    pub weight: f32,
    pub play_rate: f32,
    pub start_transform: crate::bindings::core_u_object::FTransform,
    pub actual_start_time: f32,
    pub total_root_motion_within_window: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 112],
}
impl URootMotionModifier {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier")
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
pub struct URootMotionModifier_Warp {
    #[doc(hidden)]
    pub(crate) __padding_392: [u8; 392],
    pub warp_target_name: FName,
    pub warp_point_anim_provider: EWarpPointAnimProvider,
    pub warp_point_anim_transform: crate::bindings::core_u_object::FTransform,
    pub warp_point_anim_bone_name: FName,
    pub b_warp_translation: bool,
    pub b_ignore_z_axis: bool,
    pub b_warp_to_feet_location: bool,
    pub add_translation_easing_func: crate::bindings::engine::EAlphaBlendOption,
    pub add_translation_easing_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    pub b_warp_rotation: bool,
    pub rotation_type: EMotionWarpRotationType,
    pub rotation_method: EMotionWarpRotationMethod,
    pub b_subtract_remaining_root_motion: bool,
    pub warp_rotation_time_multiplier: f32,
    pub warp_max_rotation_rate: f32,
    __padding_end: [u8; 364],
}
impl URootMotionModifier_Warp {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_Warp")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_Warp")
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
pub struct UDEPRECATED_RootMotionModifier_SimpleWarp {
    __padding_end: [u8; 912],
}
impl UDEPRECATED_RootMotionModifier_SimpleWarp {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RootMotionModifier_SimpleWarp")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RootMotionModifier_SimpleWarp")
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
pub struct URootMotionModifier_Scale {
    #[doc(hidden)]
    pub(crate) __padding_392: [u8; 392],
    pub scale: crate::bindings::core_u_object::FVector,
}
impl URootMotionModifier_Scale {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_Scale")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_Scale")
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
    pub fn add_root_motion_modifier_scale(
        in_motion_warping_comp: UPtr<UMotionWarpingComponent>,
        in_animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        in_start_time: f32,
        in_end_time: f32,
        in_scale: crate::bindings::core_u_object::FVector,
    ) -> UPtr<URootMotionModifier_Scale> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_scale_add_root_motion_modifier_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_animation,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_end_time,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_scale,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::URootMotionModifier_Scale::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_scale_add_root_motion_modifier_scale,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URootMotionModifier_Scale>>().read() }
    }
}
#[repr(C, align(16))]
pub struct URootMotionModifier_AdjustmentBlendWarp {
    #[doc(hidden)]
    pub(crate) __padding_904: [u8; 904],
    pub b_warp_ik_bones: bool,
    pub ik_bones: TArray<FName>,
    __padding_end: [u8; 320],
}
impl URootMotionModifier_AdjustmentBlendWarp {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_AdjustmentBlendWarp")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_AdjustmentBlendWarp")
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
pub struct URootMotionModifier_PrecomputedWarp {
    __padding_end: [u8; 1568],
}
impl URootMotionModifier_PrecomputedWarp {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_PrecomputedWarp")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_PrecomputedWarp")
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
    pub fn set_align_offset(
        &mut self,
        new_transform: crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_precomputed_warp_set_align_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_precomputed_warp_set_align_offset,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct URootMotionModifier_SkewWarp {
    #[doc(hidden)]
    pub(crate) __padding_904: [u8; 904],
    pub max_speed_clamp_ratio: f32,
}
impl URootMotionModifier_SkewWarp {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_SkewWarp")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootMotionModifier_SkewWarp")
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
    pub fn add_root_motion_modifier_skew_warp(
        in_motion_warping_comp: UPtr<UMotionWarpingComponent>,
        in_animation: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        in_start_time: f32,
        in_end_time: f32,
        in_warp_target_name: FName,
        in_warp_point_anim_provider: EWarpPointAnimProvider,
        in_warp_point_anim_transform: crate::bindings::core_u_object::FTransform,
        in_warp_point_anim_bone_name: FName,
        b_in_warp_translation: bool,
        b_in_ignore_z_axis: bool,
        b_in_warp_rotation: bool,
        in_rotation_type: EMotionWarpRotationType,
        in_rotation_method: EMotionWarpRotationMethod,
        in_warp_rotation_time_multiplier: f32,
        in_warp_max_rotation_rate: f32,
    ) -> UPtr<URootMotionModifier_SkewWarp> {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_skew_warp_add_root_motion_modifier_skew_warp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_motion_warping_comp,
                __buffer.add(0).cast::<UPtr<UMotionWarpingComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_animation,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_end_time,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_target_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_point_anim_provider,
                __buffer.add(36).cast::<EWarpPointAnimProvider>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_point_anim_transform,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_point_anim_bone_name,
                __buffer.add(144).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_warp_translation,
                __buffer.add(156).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_ignore_z_axis,
                __buffer.add(157).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_warp_rotation,
                __buffer.add(158).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_type,
                __buffer.add(159).cast::<EMotionWarpRotationType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_method,
                __buffer.add(160).cast::<EMotionWarpRotationMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_rotation_time_multiplier,
                __buffer.add(164).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_warp_max_rotation_rate,
                __buffer.add(168).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::motion_warping::URootMotionModifier_SkewWarp::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::motion_warping::__FUNCTION_PTRS
                    .u_root_motion_modifier_skew_warp_add_root_motion_modifier_skew_warp,
                __buffer,
            )
        };
        unsafe { __buffer.add(176).cast::<UPtr<URootMotionModifier_SkewWarp>>().read() }
    }
}
#[repr(C, align(8))]
pub struct FMotionWarpingComponent_OnPreUpdate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRootMotionModifier_OnActivateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRootMotionModifier_OnUpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRootMotionModifier_OnDeactivateDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EWarpTargetLocationOffsetDirection(pub u8);
impl EWarpTargetLocationOffsetDirection {
    pub const TARGETS_FORWARD_VECTOR: EWarpTargetLocationOffsetDirection = EWarpTargetLocationOffsetDirection(
        0,
    );
    pub const VECTOR_FROM_TARGET_TO_OWNER: EWarpTargetLocationOffsetDirection = EWarpTargetLocationOffsetDirection(
        1,
    );
    pub const WORLD_SPACE: EWarpTargetLocationOffsetDirection = EWarpTargetLocationOffsetDirection(
        2,
    );
}
#[repr(transparent)]
pub struct EPrecomputedWarpWeightCurveType(pub i32);
impl EPrecomputedWarpWeightCurveType {
    pub const FROM_ROOT_MOTION_TRANSLATION: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        0,
    );
    pub const FROM_ROOT_MOTION_ROTATION: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        1,
    );
    pub const LINEAR: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        2,
    );
    pub const EASE_IN: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        3,
    );
    pub const EASE_OUT: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        4,
    );
    pub const EASE_IN_OUT: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        5,
    );
    pub const INSTANT: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        6,
    );
    pub const DO_NOT_WARP: EPrecomputedWarpWeightCurveType = EPrecomputedWarpWeightCurveType(
        7,
    );
}
#[repr(transparent)]
pub struct ESwitchOffConditionEffect(pub u8);
impl ESwitchOffConditionEffect {
    pub const CANCEL_FOLLOW: ESwitchOffConditionEffect = ESwitchOffConditionEffect(0);
    pub const CANCEL_WARPING: ESwitchOffConditionEffect = ESwitchOffConditionEffect(1);
    pub const PAUSE_WARPING: ESwitchOffConditionEffect = ESwitchOffConditionEffect(2);
    pub const PAUSE_ROOT_MOTION: ESwitchOffConditionEffect = ESwitchOffConditionEffect(
        3,
    );
}
#[repr(transparent)]
pub struct ESwitchOffConditionDistanceOp(pub u8);
impl ESwitchOffConditionDistanceOp {
    pub const LESS_THAN: ESwitchOffConditionDistanceOp = ESwitchOffConditionDistanceOp(
        0,
    );
    pub const GREATER_THAN: ESwitchOffConditionDistanceOp = ESwitchOffConditionDistanceOp(
        1,
    );
}
#[repr(transparent)]
pub struct ESwitchOffConditionAngleOp(pub u8);
impl ESwitchOffConditionAngleOp {
    pub const LESS_THAN: ESwitchOffConditionAngleOp = ESwitchOffConditionAngleOp(0);
    pub const GREATER_THAN: ESwitchOffConditionAngleOp = ESwitchOffConditionAngleOp(1);
}
#[repr(transparent)]
pub struct ESwitchOffConditionCompositeOp(pub u8);
impl ESwitchOffConditionCompositeOp {
    pub const OR: ESwitchOffConditionCompositeOp = ESwitchOffConditionCompositeOp(0);
    pub const AND: ESwitchOffConditionCompositeOp = ESwitchOffConditionCompositeOp(1);
}
#[repr(transparent)]
pub struct EWarpPointAnimProvider(pub u8);
impl EWarpPointAnimProvider {
    pub const NONE: EWarpPointAnimProvider = EWarpPointAnimProvider(0);
    pub const STATIC: EWarpPointAnimProvider = EWarpPointAnimProvider(1);
    pub const BONE: EWarpPointAnimProvider = EWarpPointAnimProvider(2);
}
#[repr(transparent)]
pub struct EMotionWarpRotationType(pub u8);
impl EMotionWarpRotationType {
    pub const DEFAULT: EMotionWarpRotationType = EMotionWarpRotationType(0);
    pub const FACING: EMotionWarpRotationType = EMotionWarpRotationType(1);
    pub const OPPOSITE_DEFAULT: EMotionWarpRotationType = EMotionWarpRotationType(2);
    pub const OPPOSITE_FACING: EMotionWarpRotationType = EMotionWarpRotationType(3);
}
#[repr(transparent)]
pub struct EMotionWarpRotationMethod(pub u8);
impl EMotionWarpRotationMethod {
    pub const SLERP: EMotionWarpRotationMethod = EMotionWarpRotationMethod(0);
    pub const SLERP_WITH_CLAMPED_RATE: EMotionWarpRotationMethod = EMotionWarpRotationMethod(
        1,
    );
    pub const CONSTANT_RATE: EMotionWarpRotationMethod = EMotionWarpRotationMethod(2);
    pub const SCALE: EMotionWarpRotationMethod = EMotionWarpRotationMethod(3);
}
#[repr(transparent)]
pub struct EAttributeBasedRootMotionMode(pub i32);
impl EAttributeBasedRootMotionMode {
    pub const APPLY_DELTA: EAttributeBasedRootMotionMode = EAttributeBasedRootMotionMode(
        0,
    );
    pub const APPLY_VELOCITY: EAttributeBasedRootMotionMode = EAttributeBasedRootMotionMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESwitchOffConditionDistanceAxesType(pub u8);
impl ESwitchOffConditionDistanceAxesType {
    pub const ALL_AXES: ESwitchOffConditionDistanceAxesType = ESwitchOffConditionDistanceAxesType(
        0,
    );
    pub const IGNORE_Z_AXIS: ESwitchOffConditionDistanceAxesType = ESwitchOffConditionDistanceAxesType(
        1,
    );
    pub const ONLY_Z_AXIS: ESwitchOffConditionDistanceAxesType = ESwitchOffConditionDistanceAxesType(
        2,
    );
}
#[repr(transparent)]
pub struct ERootMotionModifierState(pub u8);
impl ERootMotionModifierState {
    pub const WAITING: ERootMotionModifierState = ERootMotionModifierState(0);
    pub const ACTIVE: ERootMotionModifierState = ERootMotionModifierState(1);
    pub const MARKED_FOR_REMOVAL: ERootMotionModifierState = ERootMotionModifierState(2);
    pub const DISABLED: ERootMotionModifierState = ERootMotionModifierState(3);
}
#[repr(transparent)]
pub struct EPrecomputedWarpUpdateMode(pub i32);
impl EPrecomputedWarpUpdateMode {
    pub const WORLD: EPrecomputedWarpUpdateMode = EPrecomputedWarpUpdateMode(0);
    pub const RELATIVE: EPrecomputedWarpUpdateMode = EPrecomputedWarpUpdateMode(1);
}
