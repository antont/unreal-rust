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
    pub u_ability_task_perform_targeting_perform_targeting_request: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_perform_targeting_perform_filtering_request: *mut crate::ffi::UFunctionOpague,
    pub u_async_action_perform_targeting_perform_targeting_request: *mut crate::ffi::UFunctionOpague,
    pub u_async_action_perform_targeting_perform_filtering_request: *mut crate::ffi::UFunctionOpague,
    pub u_async_action_perform_targeting_get_targeting_handle: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_start_async_targeting_request: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_remove_async_targeting_request_with_handle: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_override_collision_query_task_data: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_get_targeting_source_context: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_get_targeting_results_actors: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_get_targeting_results: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_subsystem_execute_targeting_request: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_task_get_targeting_subsystem: *mut crate::ffi::UFunctionOpague,
    pub u_simple_targeting_filter_task_bp_should_filter_target: *mut crate::ffi::UFunctionOpague,
    pub u_simple_targeting_selection_task_select_targets: *mut crate::ffi::UFunctionOpague,
    pub u_simple_targeting_selection_task_add_target_actor: *mut crate::ffi::UFunctionOpague,
    pub u_simple_targeting_selection_task_add_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_simple_targeting_sort_task_bp_get_score_for_target: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_aoe_get_source_rotation_offset: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_aoe_get_source_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_aoe_get_source_offset: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_aoe_get_source_location: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_trace_length: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_trace_direction: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_swept_trace_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_swept_trace_radius: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_swept_trace_capsule_half_height: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_swept_trace_box_half_extents: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_source_offset: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_source_location: *mut crate::ffi::UFunctionOpague,
    pub u_targeting_selection_task_trace_get_additional_actors_to_ignore: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_ability_task_perform_targeting_perform_targeting_request: std::ptr::null_mut(),
            u_ability_task_perform_targeting_perform_filtering_request: std::ptr::null_mut(),
            u_async_action_perform_targeting_perform_targeting_request: std::ptr::null_mut(),
            u_async_action_perform_targeting_perform_filtering_request: std::ptr::null_mut(),
            u_async_action_perform_targeting_get_targeting_handle: std::ptr::null_mut(),
            u_targeting_subsystem_start_async_targeting_request: std::ptr::null_mut(),
            u_targeting_subsystem_remove_async_targeting_request_with_handle: std::ptr::null_mut(),
            u_targeting_subsystem_override_collision_query_task_data: std::ptr::null_mut(),
            u_targeting_subsystem_get_targeting_source_context: std::ptr::null_mut(),
            u_targeting_subsystem_get_targeting_results_actors: std::ptr::null_mut(),
            u_targeting_subsystem_get_targeting_results: std::ptr::null_mut(),
            u_targeting_subsystem_execute_targeting_request: std::ptr::null_mut(),
            u_targeting_task_get_targeting_subsystem: std::ptr::null_mut(),
            u_simple_targeting_filter_task_bp_should_filter_target: std::ptr::null_mut(),
            u_simple_targeting_selection_task_select_targets: std::ptr::null_mut(),
            u_simple_targeting_selection_task_add_target_actor: std::ptr::null_mut(),
            u_simple_targeting_selection_task_add_hit_result: std::ptr::null_mut(),
            u_simple_targeting_sort_task_bp_get_score_for_target: std::ptr::null_mut(),
            u_targeting_selection_task_aoe_get_source_rotation_offset: std::ptr::null_mut(),
            u_targeting_selection_task_aoe_get_source_rotation: std::ptr::null_mut(),
            u_targeting_selection_task_aoe_get_source_offset: std::ptr::null_mut(),
            u_targeting_selection_task_aoe_get_source_location: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_trace_length: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_trace_direction: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_swept_trace_rotation: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_swept_trace_radius: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_swept_trace_capsule_half_height: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_swept_trace_box_half_extents: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_source_offset: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_source_location: std::ptr::null_mut(),
            u_targeting_selection_task_trace_get_additional_actors_to_ignore: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_PerformTargeting::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformTargetingRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_targeting_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformFilteringRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_filtering_request,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncAction_PerformTargeting::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformTargetingRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_targeting_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformFilteringRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_filtering_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetingHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_async_action_perform_targeting_get_targeting_handle,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTargetingSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartAsyncTargetingRequest"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_subsystem_start_async_targeting_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAsyncTargetingRequestWithHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_subsystem_remove_async_targeting_request_with_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OverrideCollisionQueryTaskData"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_subsystem_override_collision_query_task_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetingSourceContext"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_source_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetingResultsActors"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_results_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetingResults"),
                &raw mut __FUNCTION_PTRS.u_targeting_subsystem_get_targeting_results,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecuteTargetingRequest"),
                &raw mut __FUNCTION_PTRS.u_targeting_subsystem_execute_targeting_request,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTargetingTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetingSubsystem"),
                &raw mut __FUNCTION_PTRS.u_targeting_task_get_targeting_subsystem,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USimpleTargetingFilterTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ShouldFilterTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_simple_targeting_filter_task_bp_should_filter_target,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USimpleTargetingSelectionTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectTargets"),
                &raw mut __FUNCTION_PTRS.u_simple_targeting_selection_task_select_targets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTargetActor"),
                &raw mut __FUNCTION_PTRS
                    .u_simple_targeting_selection_task_add_target_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddHitResult"),
                &raw mut __FUNCTION_PTRS.u_simple_targeting_selection_task_add_hit_result,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USimpleTargetingSortTask::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_GetScoreForTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_simple_targeting_sort_task_bp_get_score_for_target,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTargetingSelectionTask_AOE::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceRotationOffset"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceOffset"),
                &raw mut __FUNCTION_PTRS.u_targeting_selection_task_aoe_get_source_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_location,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTargetingSelectionTask_Trace::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTraceLength"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTraceDirection"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSweptTraceRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSweptTraceRadius"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_radius,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSweptTraceCapsuleHalfHeight"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_capsule_half_height,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSweptTraceBoxHalfExtents"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_box_half_extents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceOffset"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAdditionalActorsToIgnore"),
                &raw mut __FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_additional_actors_to_ignore,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FTargetingRequestHandle {
    pub(crate) __padding_end: [u8; 4],
}
impl FTargetingRequestHandle {}
#[repr(C, align(8))]
pub struct FCollisionQueryTaskData {
    pub ignored_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
}
impl FCollisionQueryTaskData {}
#[repr(C, align(8))]
pub struct FTargetingTaskSet {
    pub tasks: TArray<UPtr<UTargetingTask>>,
}
impl FTargetingTaskSet {}
#[repr(C, align(8))]
pub struct FTargetingDefaultResultData {
    pub hit_result: crate::bindings::engine::FHitResult,
    pub score: f32,
}
impl FTargetingDefaultResultData {}
#[repr(C, align(8))]
pub struct FTargetingSourceContext {
    pub source_actor: UPtr<crate::bindings::engine::AActor>,
    pub instigator_actor: UPtr<crate::bindings::engine::AActor>,
    pub source_location: crate::bindings::core_u_object::FVector,
    pub source_socket_name: FName,
    pub source_object: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FTargetingSourceContext {}
#[repr(C, align(8))]
pub struct UAbilityTask_PerformTargeting {
    __padding_end: [u8; 208],
}
impl UAbilityTask_PerformTargeting {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PerformTargeting")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PerformTargeting")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn perform_targeting_request(
        owning_ability: UPtr<crate::bindings::gameplay_abilities::UGameplayAbility>,
        in_targeting_preset: UPtr<UTargetingPreset>,
        b_allow_async: bool,
    ) -> UPtr<UAbilityTask_PerformTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_targeting_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::gameplay_abilities::UGameplayAbility>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_targeting_preset,
                __buffer.add(8).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_async,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::targeting_system::UAbilityTask_PerformTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_targeting_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UAbilityTask_PerformTargeting>>().read() }
    }
    pub fn perform_filtering_request(
        owning_ability: UPtr<crate::bindings::gameplay_abilities::UGameplayAbility>,
        targeting_preset: UPtr<UTargetingPreset>,
        in_targets: TArray<UPtr<crate::bindings::engine::AActor>>,
        b_allow_async: bool,
    ) -> UPtr<UAbilityTask_PerformTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_filtering_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::gameplay_abilities::UGameplayAbility>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_preset,
                __buffer.add(8).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_targets,
                __buffer.add(16).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_async,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::targeting_system::UAbilityTask_PerformTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_ability_task_perform_targeting_perform_filtering_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UAbilityTask_PerformTargeting>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncAction_PerformTargeting {
    __padding_end: [u8; 120],
}
impl UAsyncAction_PerformTargeting {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncAction_PerformTargeting")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncAction_PerformTargeting")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn perform_targeting_request(
        source_actor: UPtr<crate::bindings::engine::AActor>,
        targeting_preset: UPtr<UTargetingPreset>,
        b_use_async_targeting: bool,
    ) -> UPtr<UAsyncAction_PerformTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_targeting_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_preset,
                __buffer.add(8).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_async_targeting,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::targeting_system::UAsyncAction_PerformTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_targeting_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UAsyncAction_PerformTargeting>>().read() }
    }
    pub fn perform_filtering_request(
        source_actor: UPtr<crate::bindings::engine::AActor>,
        targeting_preset: UPtr<UTargetingPreset>,
        b_use_async_targeting: bool,
        in_targets: TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> UPtr<UAsyncAction_PerformTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_filtering_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_preset,
                __buffer.add(8).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_async_targeting,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_targets,
                __buffer.add(24).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::targeting_system::UAsyncAction_PerformTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_perform_filtering_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UAsyncAction_PerformTargeting>>().read() }
    }
    pub fn get_targeting_handle(&self) -> FTargetingRequestHandle {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_get_targeting_handle,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_async_action_perform_targeting_get_targeting_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FTargetingRequestHandle>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTargetingPreset {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub targeting_task_set: FTargetingTaskSet,
}
impl UTargetingPreset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingPreset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingPreset")
            .copied()
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
pub struct UTargetingSubsystem {
    __padding_end: [u8; 176],
}
impl UTargetingSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn start_async_targeting_request(
        &mut self,
        targeting_preset: UPtr<UTargetingPreset>,
        in_source_context: &FTargetingSourceContext,
        completion_dynamic_delegate: FStartAsyncTargetingRequest_CompletionDynamicDelegate,
    ) -> FTargetingRequestHandle {
        let mut __stack = crate::core_data::StackAlloc::<108>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_start_async_targeting_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_preset,
                __buffer.add(0).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_source_context,
                __buffer.add(8).cast::<FTargetingSourceContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completion_dynamic_delegate,
                __buffer
                    .add(72)
                    .cast::<FStartAsyncTargetingRequest_CompletionDynamicDelegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_start_async_targeting_request,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<FTargetingRequestHandle>().read() }
    }
    pub fn remove_async_targeting_request_with_handle(
        &mut self,
        targeting_handle: &mut FTargetingRequestHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_remove_async_targeting_request_with_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_remove_async_targeting_request_with_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FTargetingRequestHandle>().swap(targeting_handle);
        }
    }
    pub fn override_collision_query_task_data(
        targeting_handle: FTargetingRequestHandle,
        collision_query_data_override: &FCollisionQueryTaskData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_override_collision_query_task_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                collision_query_data_override,
                __buffer.add(8).cast::<FCollisionQueryTaskData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::targeting_system::UTargetingSubsystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_override_collision_query_task_data,
                __buffer,
            )
        };
    }
    pub fn get_targeting_source_context(
        &self,
        targeting_handle: FTargetingRequestHandle,
    ) -> FTargetingSourceContext {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_source_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_source_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FTargetingSourceContext>().read() }
    }
    pub fn get_targeting_results_actors(
        &self,
        targeting_handle: FTargetingRequestHandle,
        targets: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_results_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                targets,
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
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_results_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(targets);
        }
    }
    pub fn get_targeting_results(
        &self,
        targeting_handle: FTargetingRequestHandle,
        out_targets: &mut TArray<crate::bindings::engine::FHitResult>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_results,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_targets,
                __buffer.add(8).cast::<TArray<crate::bindings::engine::FHitResult>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_get_targeting_results,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::engine::FHitResult>>()
                .swap(out_targets);
        }
    }
    pub fn execute_targeting_request(
        &mut self,
        targeting_preset: UPtr<UTargetingPreset>,
        in_source_context: &FTargetingSourceContext,
        completion_dynamic_delegate: FExecuteTargetingRequest_CompletionDynamicDelegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_execute_targeting_request,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &targeting_preset,
                __buffer.add(0).cast::<UPtr<UTargetingPreset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_source_context,
                __buffer.add(8).cast::<FTargetingSourceContext>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completion_dynamic_delegate,
                __buffer
                    .add(72)
                    .cast::<FExecuteTargetingRequest_CompletionDynamicDelegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_subsystem_execute_targeting_request,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UTargetingTask {
    __padding_end: [u8; 48],
}
impl UTargetingTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingTask")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingTask")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_targeting_subsystem(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> UPtr<UTargetingSubsystem> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_task_get_targeting_subsystem,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_task_get_targeting_subsystem,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UTargetingSubsystem>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTargetingFilterTask_BasicFilterTemplate {
    __padding_end: [u8; 48],
}
impl UTargetingFilterTask_BasicFilterTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_BasicFilterTemplate")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_BasicFilterTemplate")
            .copied()
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
pub struct USimpleTargetingFilterTask {
    __padding_end: [u8; 48],
}
impl USimpleTargetingFilterTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingFilterTask")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingFilterTask")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn bp_should_filter_target(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        target_data: &FTargetingDefaultResultData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_filter_task_bp_should_filter_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(8).cast::<FTargetingDefaultResultData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_filter_task_bp_should_filter_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(280).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct USimpleTargetingSelectionTask {
    __padding_end: [u8; 48],
}
impl USimpleTargetingSelectionTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingSelectionTask")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingSelectionTask")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn select_targets(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        source_context: &FTargetingSourceContext,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_select_targets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_context,
                __buffer.add(8).cast::<FTargetingSourceContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_select_targets,
                __buffer,
            )
        };
    }
    pub fn add_target_actor(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_add_target_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_add_target_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_hit_result(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        hit_result: &crate::bindings::engine::FHitResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<273>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_add_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_result,
                __buffer.add(8).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_selection_task_add_hit_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(272).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTargetingSortTask_Base {
    __padding_end: [u8; 56],
}
impl UTargetingSortTask_Base {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSortTask_Base")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSortTask_Base")
            .copied()
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
pub struct USimpleTargetingSortTask {
    __padding_end: [u8; 56],
}
impl USimpleTargetingSortTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingSortTask")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleTargetingSortTask")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn bp_get_score_for_target(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        target_data: &FTargetingDefaultResultData,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<284>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_sort_task_bp_get_score_for_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(8).cast::<FTargetingDefaultResultData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_simple_targeting_sort_task_bp_get_score_for_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(280).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTargetingFilterTask_ActorClass {
    __padding_end: [u8; 80],
}
impl UTargetingFilterTask_ActorClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_ActorClass")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_ActorClass")
            .copied()
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
pub struct UTargetingFilterTask_SortByDistance {
    __padding_end: [u8; 64],
}
impl UTargetingFilterTask_SortByDistance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_SortByDistance")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingFilterTask_SortByDistance")
            .copied()
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
pub struct UTargetingSelectionTask_AOE {
    __padding_end: [u8; 288],
}
impl UTargetingSelectionTask_AOE {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_AOE")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_AOE")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_source_rotation_offset(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation_offset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_source_rotation(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_source_offset(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_offset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_source_location(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_aoe_get_source_location,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UTargetingSelectionTask_SourceActor {
    __padding_end: [u8; 48],
}
impl UTargetingSelectionTask_SourceActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_SourceActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_SourceActor")
            .copied()
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
pub struct UTargetingSelectionTask_Trace {
    __padding_end: [u8; 480],
}
impl UTargetingSelectionTask_Trace {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_Trace")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetingSelectionTask_Trace")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_trace_length(&self, targeting_handle: &FTargetingRequestHandle) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_trace_direction(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_trace_direction,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_swept_trace_rotation(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_swept_trace_radius(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_radius,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_radius,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_swept_trace_capsule_half_height(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_capsule_half_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_capsule_half_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_swept_trace_box_half_extents(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_box_half_extents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_swept_trace_box_half_extents,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_source_offset(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_offset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_source_location(
        &self,
        targeting_handle: &FTargetingRequestHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_source_location,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_additional_actors_to_ignore(
        &self,
        targeting_handle: &FTargetingRequestHandle,
        out_additional_actors_to_ignore: &mut TArray<
            UPtr<crate::bindings::engine::AActor>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_additional_actors_to_ignore,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                targeting_handle,
                __buffer.add(0).cast::<FTargetingRequestHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_additional_actors_to_ignore,
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
                crate::bindings::targeting_system::__FUNCTION_PTRS
                    .u_targeting_selection_task_trace_get_additional_actors_to_ignore,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_additional_actors_to_ignore);
        }
    }
}
#[repr(C, align(8))]
pub struct FTargetingRequestData_TargetingRequestDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FExecuteTargetingRequest_CompletionDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FStartAsyncTargetingRequest_CompletionDynamicDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PerformTargeting_OnTargetReady {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncAction_PerformTargeting_Targeted {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ETargetingAOEShape(pub u8);
impl ETargetingAOEShape {
    pub const BOX: ETargetingAOEShape = ETargetingAOEShape(0);
    pub const CYLINDER: ETargetingAOEShape = ETargetingAOEShape(1);
    pub const SPHERE: ETargetingAOEShape = ETargetingAOEShape(2);
    pub const CAPSULE: ETargetingAOEShape = ETargetingAOEShape(3);
    pub const SOURCE_COMPONENT: ETargetingAOEShape = ETargetingAOEShape(4);
}
#[repr(transparent)]
pub struct ETargetingTraceType(pub u8);
impl ETargetingTraceType {
    pub const LINE: ETargetingTraceType = ETargetingTraceType(0);
    pub const SPHERE: ETargetingTraceType = ETargetingTraceType(1);
    pub const CAPSULE: ETargetingTraceType = ETargetingTraceType(2);
    pub const BOX: ETargetingTraceType = ETargetingTraceType(3);
}
