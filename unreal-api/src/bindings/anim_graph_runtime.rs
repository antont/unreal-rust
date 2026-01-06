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
pub static mut U_SEQUENCER_ANIMATION_OVERRIDE_GET_SEQUENCER_ANIM_SLOT_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_ANIMATION_OVERRIDE_ALLOWS_CINEMATIC_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_SET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_IS_STATE_BLENDING_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_IS_STATE_BLENDING_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_RELEVANT_ANIM_TIME_REMAINING_FRACTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_RELEVANT_ANIM_TIME_REMAINING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_RESULT_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_RESULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_MACHINE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_MACHINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_PROTOTYPE_THREAD_SAFE_ANIM_UPDATE_CALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_PROTOTYPE_THREAD_SAFE_ANIM_NODE_CALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_IS_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_DELTA_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_CURRENT_WEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_ANIM_NODE_REFERENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_ANIM_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_UPDATE_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_POSE_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_INITIALIZATION_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_COMPONENT_SPACE_POSE_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_LIST_BASE_LIBRARY_RESET_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_LIST_BASE_LIBRARY_CONVERT_TO_BLEND_LIST_BASE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_LIBRARY_SNAP_TO_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_LIBRARY_GET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_LIBRARY_GET_FILTERED_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_LIBRARY_CONVERT_TO_BLEND_SPACE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_LIBRARY_CONVERT_TO_BLEND_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SNAP_TO_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SHOULD_RESET_PLAY_TIME_WHEN_BLEND_SPACE_CHANGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_RESET_PLAY_TIME_WHEN_BLEND_SPACE_CHANGES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_LOOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_BLEND_SPACE_WITH_INERTIAL_BLENDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_BLEND_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_START_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_LOOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_BLEND_SPACE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_CONVERT_TO_BLEND_SPACE_PLAYER_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BLEND_SPACE_PLAYER_LIBRARY_CONVERT_TO_BLEND_SPACE_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_NODE_RIGID_BODY_LIBRARY_SET_OVERRIDE_PHYSICS_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_NODE_RIGID_BODY_LIBRARY_CONVERT_TO_RIGID_BODY_ANIM_NODE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_NODE_RIGID_BODY_LIBRARY_CONVERT_TO_RIGID_BODY_ANIM_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_TWO_BONE_IK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_START_PROFILING_TIMER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_MAKE_PERLIN_NOISE_VECTOR_AND_REMAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_MAKE_PERLIN_NOISE_AND_REMAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_LOOK_AT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_END_PROFILING_TIMER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_DISTANCE_BETWEEN_TWO_SOCKETS_AND_MAP_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_DIRECTION_BETWEEN_SOCKETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_CALCULATE_VELOCITY_FROM_SOCKETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_K2_CALCULATE_VELOCITY_FROM_POSITION_HISTORY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_KISMET_ANIMATION_LIBRARY_CALCULATE_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERED_BONE_BLEND_LIBRARY_SET_BLEND_MASK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERED_BONE_BLEND_LIBRARY_GET_NUM_POSES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERED_BONE_BLEND_LIBRARY_CONVERT_TO_LAYERED_BONE_BLEND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERED_BONE_BLEND_LIBRARY_CONVERT_TO_LAYERED_BLEND_PER_BONE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINKED_ANIM_GRAPH_LIBRARY_HAS_LINKED_ANIM_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINKED_ANIM_GRAPH_LIBRARY_GET_LINKED_ANIM_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINKED_ANIM_GRAPH_LIBRARY_CONVERT_TO_LINKED_ANIM_GRAPH_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINKED_ANIM_GRAPH_LIBRARY_CONVERT_TO_LINKED_ANIM_GRAPH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_SET_MIRROR_TRANSITION_BLEND_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_SET_MIRROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR_TRANSITION_BLEND_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR_DATA_TABLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_CONVERT_TO_MIRROR_NODE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MIRROR_ANIM_LIBRARY_CONVERT_TO_MIRROR_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_CURVE_MAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_APPLY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_ALPHA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_GET_APPLY_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_GET_ALPHA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_CONVERT_TO_MODIFY_CURVE_NODE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODIFY_CURVE_ANIM_LIBRARY_CONVERT_TO_MODIFY_CURVE_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_NOTIFY_END_RECEIVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_NOTIFY_BEGIN_RECEIVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_MONTAGE_ENDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_MONTAGE_BLENDING_OUT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PLAY_MONTAGE_CALLBACK_PROXY_CREATE_PROXY_OBJECT_FOR_PLAY_MONTAGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_SEQUENCE_WITH_INERTIAL_BLENDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_EXPLICIT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_EXPLICIT_FRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_GET_ACCUMULATED_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_CONVERT_TO_SEQUENCE_EVALUATOR_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_CONVERT_TO_SEQUENCE_EVALUATOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_EVALUATOR_LIBRARY_ADVANCE_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_SET_START_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_SET_SEQUENCE_WITH_INERTIAL_BLENDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_SET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_SET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_SET_ACCUMULATED_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_START_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_SEQUENCE_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_LOOP_ANIMATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_GET_ACCUMULATED_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_CONVERT_TO_SEQUENCE_PLAYER_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_CONVERT_TO_SEQUENCE_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCE_PLAYER_LIBRARY_COMPUTE_PLAY_RATE_FROM_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_CONTROL_LIBRARY_SET_ALPHA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_CONTROL_LIBRARY_GET_ALPHA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_CONTROL_LIBRARY_CONVERT_TO_SKELETAL_CONTROL_PURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_CONTROL_LIBRARY_CONVERT_TO_SKELETAL_CONTROL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerAnimationOverride::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequencerAnimSlotNames"),
            &raw mut U_SEQUENCER_ANIMATION_OVERRIDE_GET_SEQUENCER_ANIM_SLOT_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AllowsCinematicOverride"),
            &raw mut U_SEQUENCER_ANIMATION_OVERRIDE_ALLOWS_CINEMATIC_OVERRIDE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationStateMachineLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetState"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_SET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStateBlendingOut"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_IS_STATE_BLENDING_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsStateBlendingIn"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_IS_STATE_BLENDING_IN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRelevantAnimTimeRemainingFraction"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_RELEVANT_ANIM_TIME_REMAINING_FRACTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRelevantAnimTimeRemaining"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_GET_RELEVANT_ANIM_TIME_REMAINING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToAnimationStateResultPure"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_RESULT_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToAnimationStateResult"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_RESULT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToAnimationStateMachinePure"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_MACHINE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToAnimationStateMachine"),
            &raw mut U_ANIMATION_STATE_MACHINE_LIBRARY_CONVERT_TO_ANIMATION_STATE_MACHINE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimExecutionContextLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Prototype_ThreadSafeAnimUpdateCall"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_PROTOTYPE_THREAD_SAFE_ANIM_UPDATE_CALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Prototype_ThreadSafeAnimNodeCall"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_PROTOTYPE_THREAD_SAFE_ANIM_NODE_CALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActive"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_IS_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDeltaTime"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_DELTA_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentWeight"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_CURRENT_WEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimNodeReference"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_ANIM_NODE_REFERENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimInstance"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_GET_ANIM_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToUpdateContext"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_UPDATE_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToPoseContext"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_POSE_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToInitializationContext"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_INITIALIZATION_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToComponentSpacePoseContext"),
            &raw mut U_ANIM_EXECUTION_CONTEXT_LIBRARY_CONVERT_TO_COMPONENT_SPACE_POSE_CONTEXT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlendListBaseLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetNode"),
            &raw mut U_BLEND_LIST_BASE_LIBRARY_RESET_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToBlendListBase"),
            &raw mut U_BLEND_LIST_BASE_LIBRARY_CONVERT_TO_BLEND_LIST_BASE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlendSpaceLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapToPosition"),
            &raw mut U_BLEND_SPACE_LIBRARY_SNAP_TO_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPosition"),
            &raw mut U_BLEND_SPACE_LIBRARY_GET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFilteredPosition"),
            &raw mut U_BLEND_SPACE_LIBRARY_GET_FILTERED_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToBlendSpacePure"),
            &raw mut U_BLEND_SPACE_LIBRARY_CONVERT_TO_BLEND_SPACE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToBlendSpace"),
            &raw mut U_BLEND_SPACE_LIBRARY_CONVERT_TO_BLEND_SPACE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBlendSpacePlayerLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SnapToPosition"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SNAP_TO_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldResetPlayTimeWhenBlendSpaceChanges"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SHOULD_RESET_PLAY_TIME_WHEN_BLEND_SPACE_CHANGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetResetPlayTimeWhenBlendSpaceChanges"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_RESET_PLAY_TIME_WHEN_BLEND_SPACE_CHANGES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayRate"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLoop"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_LOOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlendSpaceWithInertialBlending"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_BLEND_SPACE_WITH_INERTIAL_BLENDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlendSpace"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_SET_BLEND_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartPosition"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_START_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPosition"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayRate"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoop"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_LOOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlendSpace"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_GET_BLEND_SPACE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToBlendSpacePlayerPure"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_CONVERT_TO_BLEND_SPACE_PLAYER_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToBlendSpacePlayer"),
            &raw mut U_BLEND_SPACE_PLAYER_LIBRARY_CONVERT_TO_BLEND_SPACE_PLAYER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimNodeRigidBodyLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOverridePhysicsAsset"),
            &raw mut U_ANIM_NODE_RIGID_BODY_LIBRARY_SET_OVERRIDE_PHYSICS_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToRigidBodyAnimNodePure"),
            &raw mut U_ANIM_NODE_RIGID_BODY_LIBRARY_CONVERT_TO_RIGID_BODY_ANIM_NODE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToRigidBodyAnimNode"),
            &raw mut U_ANIM_NODE_RIGID_BODY_LIBRARY_CONVERT_TO_RIGID_BODY_ANIM_NODE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UKismetAnimationLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_TwoBoneIK"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_TWO_BONE_IK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_StartProfilingTimer"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_START_PROFILING_TIMER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_MakePerlinNoiseVectorAndRemap"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_MAKE_PERLIN_NOISE_VECTOR_AND_REMAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_MakePerlinNoiseAndRemap"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_MAKE_PERLIN_NOISE_AND_REMAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_LookAt"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_LOOK_AT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_EndProfilingTimer"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_END_PROFILING_TIMER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_DistanceBetweenTwoSocketsAndMapRange"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_DISTANCE_BETWEEN_TWO_SOCKETS_AND_MAP_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_DirectionBetweenSockets"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_DIRECTION_BETWEEN_SOCKETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CalculateVelocityFromSockets"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_CALCULATE_VELOCITY_FROM_SOCKETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_CalculateVelocityFromPositionHistory"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_K2_CALCULATE_VELOCITY_FROM_POSITION_HISTORY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CalculateDirection"),
            &raw mut U_KISMET_ANIMATION_LIBRARY_CALCULATE_DIRECTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULayeredBoneBlendLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlendMask"),
            &raw mut U_LAYERED_BONE_BLEND_LIBRARY_SET_BLEND_MASK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumPoses"),
            &raw mut U_LAYERED_BONE_BLEND_LIBRARY_GET_NUM_POSES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToLayeredBoneBlend"),
            &raw mut U_LAYERED_BONE_BLEND_LIBRARY_CONVERT_TO_LAYERED_BONE_BLEND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToLayeredBlendPerBonePure"),
            &raw mut U_LAYERED_BONE_BLEND_LIBRARY_CONVERT_TO_LAYERED_BLEND_PER_BONE_PURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULinkedAnimGraphLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasLinkedAnimInstance"),
            &raw mut U_LINKED_ANIM_GRAPH_LIBRARY_HAS_LINKED_ANIM_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinkedAnimInstance"),
            &raw mut U_LINKED_ANIM_GRAPH_LIBRARY_GET_LINKED_ANIM_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToLinkedAnimGraphPure"),
            &raw mut U_LINKED_ANIM_GRAPH_LIBRARY_CONVERT_TO_LINKED_ANIM_GRAPH_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToLinkedAnimGraph"),
            &raw mut U_LINKED_ANIM_GRAPH_LIBRARY_CONVERT_TO_LINKED_ANIM_GRAPH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMirrorAnimLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMirrorTransitionBlendTime"),
            &raw mut U_MIRROR_ANIM_LIBRARY_SET_MIRROR_TRANSITION_BLEND_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMirror"),
            &raw mut U_MIRROR_ANIM_LIBRARY_SET_MIRROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMirrorTransitionBlendTime"),
            &raw mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR_TRANSITION_BLEND_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMirrorDataTable"),
            &raw mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR_DATA_TABLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMirror"),
            &raw mut U_MIRROR_ANIM_LIBRARY_GET_MIRROR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToMirrorNodePure"),
            &raw mut U_MIRROR_ANIM_LIBRARY_CONVERT_TO_MIRROR_NODE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToMirrorNode"),
            &raw mut U_MIRROR_ANIM_LIBRARY_CONVERT_TO_MIRROR_NODE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UModifyCurveAnimLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveMap"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_CURVE_MAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetApplyMode"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_APPLY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlpha"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_SET_ALPHA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetApplyMode"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_GET_APPLY_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAlpha"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_GET_ALPHA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToModifyCurveNodePure"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_CONVERT_TO_MODIFY_CURVE_NODE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToModifyCurveNode"),
            &raw mut U_MODIFY_CURVE_ANIM_LIBRARY_CONVERT_TO_MODIFY_CURVE_NODE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlayMontageCallbackProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNotifyEndReceived"),
            &raw mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_NOTIFY_END_RECEIVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNotifyBeginReceived"),
            &raw mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_NOTIFY_BEGIN_RECEIVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMontageEnded"),
            &raw mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_MONTAGE_ENDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMontageBlendingOut"),
            &raw mut U_PLAY_MONTAGE_CALLBACK_PROXY_ON_MONTAGE_BLENDING_OUT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateProxyObjectForPlayMontage"),
            &raw mut U_PLAY_MONTAGE_CALLBACK_PROXY_CREATE_PROXY_OBJECT_FOR_PLAY_MONTAGE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequenceEvaluatorLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequenceWithInertialBlending"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_SEQUENCE_WITH_INERTIAL_BLENDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequence"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExplicitTime"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_EXPLICIT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetExplicitFrame"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_SET_EXPLICIT_FRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAccumulatedTime"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_GET_ACCUMULATED_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSequenceEvaluatorPure"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_CONVERT_TO_SEQUENCE_EVALUATOR_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSequenceEvaluator"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_CONVERT_TO_SEQUENCE_EVALUATOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AdvanceTime"),
            &raw mut U_SEQUENCE_EVALUATOR_LIBRARY_ADVANCE_TIME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencePlayerLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartPosition"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_SET_START_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequenceWithInertialBlending"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_SET_SEQUENCE_WITH_INERTIAL_BLENDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequence"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_SET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayRate"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_SET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAccumulatedTime"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_SET_ACCUMULATED_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartPosition"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_START_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequencePure"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_SEQUENCE_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayRate"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoopAnimation"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_LOOP_ANIMATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAccumulatedTime"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_GET_ACCUMULATED_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSequencePlayerPure"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_CONVERT_TO_SEQUENCE_PLAYER_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSequencePlayer"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_CONVERT_TO_SEQUENCE_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputePlayRateFromDuration"),
            &raw mut U_SEQUENCE_PLAYER_LIBRARY_COMPUTE_PLAY_RATE_FROM_DURATION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkeletalControlLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAlpha"),
            &raw mut U_SKELETAL_CONTROL_LIBRARY_SET_ALPHA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAlpha"),
            &raw mut U_SKELETAL_CONTROL_LIBRARY_GET_ALPHA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSkeletalControlPure"),
            &raw mut U_SKELETAL_CONTROL_LIBRARY_CONVERT_TO_SKELETAL_CONTROL_PURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToSkeletalControl"),
            &raw mut U_SKELETAL_CONTROL_LIBRARY_CONVERT_TO_SKELETAL_CONTROL,
        );
    }
}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraphBase {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub x: f32,
    pub y: f32,
    pub group_name: FName,
    pub group_role: crate::bindings::engine::EAnimGroupRole,
    __padding_end: [u8; 99],
}
impl FAnimNode_BlendSpaceGraphBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceGraph {
    __padding_end: [u8; 256],
}
impl FAnimNode_BlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceSampleResult {
    __padding_end: [u8; 200],
}
impl FAnimNode_BlendSpaceSampleResult {}
#[repr(C, align(8))]
pub struct FAnimNode_SkeletalControlBase {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub component_pose: crate::bindings::engine::FComponentSpacePoseLink,
    pub lod_threshold: i32,
    #[doc(hidden)]
    __padding_168: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    __padding_end: [u8; 108],
}
impl FAnimNode_SkeletalControlBase {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationMultiplier {
    #[doc(hidden)]
    __padding_464: [u8; 464],
    pub multiplier: f32,
    __padding_end: [u8; 4],
}
impl FAnimNode_RotationMultiplier {}
#[repr(C, align(16))]
pub struct FRotationRetargetingInfo {
    __padding_end: [u8; 416],
}
impl FRotationRetargetingInfo {}
#[repr(C, align(8))]
pub struct FPositionHistory {
    __padding_end: [u8; 48],
}
impl FPositionHistory {}
#[repr(C, align(8))]
pub struct FAnimationStateResultReference {
    __padding_end: [u8; 16],
}
impl FAnimationStateResultReference {}
#[repr(C, align(8))]
pub struct FAnimationStateMachineReference {
    __padding_end: [u8; 16],
}
impl FAnimationStateMachineReference {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayerBase {
    __padding_end: [u8; 232],
}
impl FAnimNode_BlendSpacePlayerBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer {
    __padding_end: [u8; 280],
}
impl FAnimNode_BlendSpacePlayer {}
#[repr(C, align(16))]
pub struct FAnimNode_AimOffsetLookAt {
    #[doc(hidden)]
    __padding_480: [u8; 480],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub source_socket_name: FName,
    pub pivot_socket_name: FName,
    pub look_at_location: crate::bindings::core_u_object::FVector,
    pub socket_axis: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
    __padding_end: [u8; 84],
}
impl FAnimNode_AimOffsetLookAt {}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyAdditive {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub lod_threshold: i32,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    __padding_336: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    __padding_end: [u8; 6],
}
impl FAnimNode_ApplyAdditive {}
#[repr(C, align(4))]
pub struct FBlendBoneByChannelEntry {
    __padding_end: [u8; 44],
}
impl FBlendBoneByChannelEntry {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendBoneByChannel {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    #[doc(hidden)]
    __padding_216: [u8; 32],
    pub alpha: f32,
    #[doc(hidden)]
    __padding_224: [u8; 4],
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    __padding_end: [u8; 8],
}
impl FAnimNode_BlendBoneByChannel {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListBase {
    __padding_end: [u8; 240],
}
impl FAnimNode_BlendListBase {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByBool {
    __padding_end: [u8; 256],
}
impl FAnimNode_BlendListByBool {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByEnum {
    __padding_end: [u8; 264],
}
impl FAnimNode_BlendListByEnum {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendListByInt {
    __padding_end: [u8; 248],
}
impl FAnimNode_BlendListByInt {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpaceEvaluator {
    #[doc(hidden)]
    __padding_280: [u8; 280],
    pub normalized_time: f32,
    __padding_end: [u8; 4],
}
impl FAnimNode_BlendSpaceEvaluator {}
#[repr(C, align(8))]
pub struct FAnimNode_BlendSpacePlayer_Standalone {
    __padding_end: [u8; 280],
}
impl FAnimNode_BlendSpacePlayer_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_CallFunction {
    __padding_end: [u8; 224],
}
impl FAnimNode_CallFunction {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyPoseFromMesh {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub flags_144: u8,
    pub b_copy_custom_attributes: bool,
    pub flags_146: u8,
    pub root_bone_to_copy: FName,
    __padding_end: [u8; 304],
}
impl FAnimNode_CopyPoseFromMesh {}
#[repr(C, align(8))]
pub struct FAnimNode_CurveSource {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub source_binding: FName,
    pub alpha: f32,
    __padding_end: [u8; 16],
}
impl FAnimNode_CurveSource {}
#[repr(C, align(8))]
pub struct FAnimNode_LayeredBoneBlend {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub blend_poses: TArray<crate::bindings::engine::FPoseLink>,
    #[doc(hidden)]
    __padding_216: [u8; 40],
    pub blend_weights: TArray<f32>,
    #[doc(hidden)]
    __padding_348: [u8; 116],
    pub lod_threshold: i32,
    pub b_mesh_space_rotation_blend: bool,
    pub b_root_space_rotation_blend: bool,
    pub b_mesh_space_scale_blend: bool,
    pub curve_blend_option: crate::bindings::engine::ECurveBlendOption,
    __padding_end: [u8; 4],
}
impl FAnimNode_LayeredBoneBlend {}
#[repr(C, align(8))]
pub struct FAnimNode_MakeDynamicAdditive {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub base: crate::bindings::engine::FPoseLink,
    pub additive: crate::bindings::engine::FPoseLink,
    pub b_mesh_space_additive: bool,
    __padding_end: [u8; 7],
}
impl FAnimNode_MakeDynamicAdditive {}
#[repr(C, align(8))]
pub struct FAnimNode_MirrorBase {
    __padding_end: [u8; 200],
}
impl FAnimNode_MirrorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror {
    __padding_end: [u8; 224],
}
impl FAnimNode_Mirror {}
#[repr(C, align(8))]
pub struct FAnimNode_Mirror_Standalone {
    __padding_end: [u8; 224],
}
impl FAnimNode_Mirror_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyCurve {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source_pose: crate::bindings::engine::FPoseLink,
    pub curve_map: TMap<FName, f32>,
    pub curve_values: TArray<f32>,
    #[doc(hidden)]
    __padding_408: [u8; 152],
    pub alpha: f32,
    pub apply_mode: EModifyCurveApplyMode,
    __padding_end: [u8; 3],
}
impl FAnimNode_ModifyCurve {}
#[repr(C, align(8))]
pub struct FAnimNode_MultiWayBlend {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub poses: TArray<crate::bindings::engine::FPoseLink>,
    pub desired_alphas: TArray<f32>,
    #[doc(hidden)]
    __padding_184: [u8; 16],
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub b_additive_node: bool,
    pub b_normalize_alpha: bool,
    __padding_end: [u8; 6],
}
impl FAnimNode_MultiWayBlend {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseHandler {
    #[doc(hidden)]
    __padding_184: [u8; 184],
    pub pose_asset: UPtr<crate::bindings::engine::UPoseAsset>,
    __padding_end: [u8; 128],
}
impl FAnimNode_PoseHandler {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseBlendNode {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub source_pose: crate::bindings::engine::FPoseLink,
    __padding_end: [u8; 40],
}
impl FAnimNode_PoseBlendNode {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseByName {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub pose_name: FName,
    pub pose_weight: f32,
    __padding_end: [u8; 16],
}
impl FAnimNode_PoseByName {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseDriver {
    #[doc(hidden)]
    __padding_320: [u8; 320],
    pub source_pose: crate::bindings::engine::FPoseLink,
    __padding_end: [u8; 288],
}
impl FAnimNode_PoseDriver {}
#[repr(C, align(8))]
pub struct FRBFParams {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub solver_type: ERBFSolverType,
    pub radius: f32,
    pub b_automatic_radius: bool,
    pub function: ERBFFunctionType,
    pub distance_method: ERBFDistanceMethod,
    pub twist_axis: crate::bindings::engine::EBoneAxis,
    pub weight_threshold: f32,
    pub normalize_method: ERBFNormalizeMethod,
    pub median_reference: crate::bindings::core_u_object::FVector,
    pub median_min: f32,
    pub median_max: f32,
}
impl FRBFParams {}
#[repr(C, align(8))]
pub struct FAnimNode_PoseSnapshot {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub snapshot_name: FName,
    pub snapshot: crate::bindings::engine::FPoseSnapshot,
    pub mode: ESnapshotSourceMode,
    __padding_end: [u8; 79],
}
impl FAnimNode_PoseSnapshot {}
#[repr(C, align(8))]
pub struct FRandomPlayerSequenceEntry {
    pub sequence: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    pub chance_to_play: f32,
    pub min_loop_count: i32,
    pub max_loop_count: i32,
    pub min_play_rate: f32,
    pub max_play_rate: f32,
    __padding_end: [u8; 52],
}
impl FRandomPlayerSequenceEntry {}
#[repr(C, align(8))]
pub struct FAnimNode_RandomPlayer {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub entries: TArray<FRandomPlayerSequenceEntry>,
    #[doc(hidden)]
    __padding_236: [u8; 84],
    pub blend_weight: f32,
    pub b_shuffle_mode: bool,
    __padding_end: [u8; 7],
}
impl FAnimNode_RandomPlayer {}
#[repr(C, align(8))]
pub struct FAnimNode_RotateRootBone {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub yaw_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    pub mesh_to_component: crate::bindings::core_u_object::FRotator,
    pub b_rotate_root_motion_attribute: bool,
    __padding_end: [u8; 15],
}
impl FAnimNode_RotateRootBone {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpace {
    #[doc(hidden)]
    __padding_280: [u8; 280],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    __padding_456: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    __padding_end: [u8; 6],
}
impl FAnimNode_RotationOffsetBlendSpace {}
#[repr(C, align(8))]
pub struct FAnimNode_RotationOffsetBlendSpaceGraph {
    #[doc(hidden)]
    __padding_256: [u8; 256],
    pub base_pose: crate::bindings::engine::FPoseLink,
    pub lod_threshold: i32,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    __padding_432: [u8; 4],
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub b_alpha_bool_enabled: bool,
    __padding_end: [u8; 6],
}
impl FAnimNode_RotationOffsetBlendSpaceGraph {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluatorBase {
    __padding_end: [u8; 192],
}
impl FAnimNode_SequenceEvaluatorBase {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator {
    __padding_end: [u8; 240],
}
impl FAnimNode_SequenceEvaluator {}
#[repr(C, align(8))]
pub struct FAnimNode_SequenceEvaluator_Standalone {
    __padding_end: [u8; 240],
}
impl FAnimNode_SequenceEvaluator_Standalone {}
#[repr(C, align(8))]
pub struct FAnimNode_Slot {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub source: crate::bindings::engine::FPoseLink,
    pub slot_name: FName,
    pub b_always_update_source_pose: bool,
    __padding_end: [u8; 35],
}
impl FAnimNode_Slot {}
#[repr(C, align(8))]
pub struct FAnimNode_Sync {
    __padding_end: [u8; 176],
}
impl FAnimNode_Sync {}
#[repr(C, align(8))]
pub struct FAnimNode_TwoWayBlend {
    #[doc(hidden)]
    __padding_136: [u8; 136],
    pub a: crate::bindings::engine::FPoseLink,
    pub b: crate::bindings::engine::FPoseLink,
    pub alpha_input_type: crate::bindings::engine::EAnimAlphaInputType,
    pub flags_185: u8,
    pub alpha: f32,
    pub alpha_scale_bias: crate::bindings::engine::FInputScaleBias,
    pub alpha_bool_blend: crate::bindings::engine::FInputAlphaBoolBlend,
    pub alpha_curve_name: FName,
    pub alpha_scale_bias_clamp: crate::bindings::engine::FInputScaleBiasClamp,
    __padding_end: [u8; 4],
}
impl FAnimNode_TwoWayBlend {}
#[repr(C, align(8))]
pub struct FBlendListBaseReference {
    __padding_end: [u8; 16],
}
impl FBlendListBaseReference {}
#[repr(C, align(8))]
pub struct FBlendSpaceReference {
    __padding_end: [u8; 16],
}
impl FBlendSpaceReference {}
#[repr(C, align(8))]
pub struct FBlendSpacePlayerReference {
    __padding_end: [u8; 16],
}
impl FBlendSpacePlayerReference {}
#[repr(C, align(8))]
pub struct FAnimPhysSimSpaceSettings {
    pub sim_space_angular_alpha: f32,
    pub max_angular_velocity: f32,
    pub max_angular_acceleration: f32,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
impl FAnimPhysSimSpaceSettings {}
#[repr(C, align(16))]
pub struct FAnimNode_AnimDynamics {
    #[doc(hidden)]
    __padding_424: [u8; 424],
    pub linear_damping_override: f32,
    pub angular_damping_override: f32,
    #[doc(hidden)]
    __padding_800: [u8; 368],
    pub gravity_scale: f32,
    pub gravity_override: crate::bindings::core_u_object::FVector,
    pub linear_spring_constant: f32,
    pub angular_spring_constant: f32,
    #[doc(hidden)]
    __padding_960: [u8; 120],
    pub angular_bias_override: f32,
    #[doc(hidden)]
    __padding_1032: [u8; 68],
    pub simulation_space: AnimPhysSimSpaceType,
    #[doc(hidden)]
    __padding_1036: [u8; 3],
    pub flags_1036: u8,
    __padding_end: [u8; 883],
}
impl FAnimNode_AnimDynamics {}
#[repr(C, align(8))]
pub struct FAnimNode_ApplyLimits {
    #[doc(hidden)]
    __padding_440: [u8; 440],
    pub angular_offsets: TArray<crate::bindings::core_u_object::FVector>,
}
impl FAnimNode_ApplyLimits {}
#[repr(C, align(8))]
pub struct FAnimNode_BoneDrivenController {
    __padding_end: [u8; 536],
}
impl FAnimNode_BoneDrivenController {}
#[repr(C, align(16))]
pub struct FAnimNode_CCDIK {
    __padding_end: [u8; 720],
}
impl FAnimNode_CCDIK {}
#[repr(C, align(16))]
pub struct FAnimNode_Constraint {
    __padding_end: [u8; 704],
}
impl FAnimNode_Constraint {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBone {
    #[doc(hidden)]
    __padding_464: [u8; 464],
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    __padding_end: [u8; 5],
}
impl FAnimNode_CopyBone {}
#[repr(C, align(8))]
pub struct FAnimNode_CopyBoneDelta {
    #[doc(hidden)]
    __padding_464: [u8; 464],
    pub b_copy_translation: bool,
    pub b_copy_rotation: bool,
    pub b_copy_scale: bool,
    pub translation_multiplier: f32,
    pub rotation_multiplier: f32,
    pub scale_multiplier: f32,
}
impl FAnimNode_CopyBoneDelta {}
#[repr(C, align(16))]
pub struct FAnimNode_Fabrik {
    #[doc(hidden)]
    __padding_432: [u8; 432],
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    __padding_end: [u8; 336],
}
impl FAnimNode_Fabrik {}
#[repr(C, align(8))]
pub struct FAnimNode_HandIKRetargeting {
    #[doc(hidden)]
    __padding_520: [u8; 520],
    pub per_axis_alpha: crate::bindings::core_u_object::FVector,
    pub hand_fk_weight: f32,
    __padding_end: [u8; 4],
}
impl FAnimNode_HandIKRetargeting {}
#[repr(C, align(8))]
pub struct FAnimNode_LegIK {
    __padding_end: [u8; 480],
}
impl FAnimNode_LegIK {}
#[repr(C, align(16))]
pub struct FAnimNode_LookAt {
    #[doc(hidden)]
    __padding_608: [u8; 608],
    pub look_at_location: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_665: [u8; 33],
    pub interpolation_type: EInterpolationBlend,
    #[doc(hidden)]
    __padding_704: [u8; 36],
    pub look_at_clamp: f32,
    pub interpolation_time: f32,
    pub interpolation_trigger_threashold: f32,
    __padding_end: [u8; 548],
}
impl FAnimNode_LookAt {}
#[repr(C, align(8))]
pub struct FAnimNode_ModifyBone {
    #[doc(hidden)]
    __padding_448: [u8; 448],
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub scale: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 8],
}
impl FAnimNode_ModifyBone {}
#[repr(C, align(8))]
pub struct FAnimNode_ObserveBone {
    __padding_end: [u8; 520],
}
impl FAnimNode_ObserveBone {}
#[repr(C, align(8))]
pub struct FAnimNode_ResetRoot {
    __padding_end: [u8; 440],
}
impl FAnimNode_ResetRoot {}
#[repr(C, align(8))]
pub struct FSimSpaceSettings {
    pub world_alpha: f32,
    #[doc(hidden)]
    __padding_8: [u8; 4],
    pub velocity_scale_z: f32,
    pub damping_alpha: f32,
    pub max_linear_velocity: f32,
    pub max_angular_velocity: f32,
    pub max_linear_acceleration: f32,
    pub max_angular_acceleration: f32,
    #[doc(hidden)]
    __padding_40: [u8; 8],
    pub external_linear_drag_v: crate::bindings::core_u_object::FVector,
    pub external_linear_velocity: crate::bindings::core_u_object::FVector,
    pub external_angular_velocity: crate::bindings::core_u_object::FVector,
}
impl FSimSpaceSettings {}
#[repr(C, align(16))]
pub struct FAnimNode_RigidBody {
    #[doc(hidden)]
    __padding_744: [u8; 744],
    pub b_use_local_lod_threshold_only: bool,
    __padding_end: [u8; 1895],
}
impl FAnimNode_RigidBody {}
#[repr(C, align(8))]
pub struct FRigidBodyAnimNodeReference {
    __padding_end: [u8; 16],
}
impl FRigidBodyAnimNodeReference {}
#[repr(C, align(8))]
pub struct FAnimNode_ScaleChainLength {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub default_chain_length: f32,
    #[doc(hidden)]
    __padding_208: [u8; 40],
    pub target_location: crate::bindings::core_u_object::FVector,
    pub alpha: f32,
    __padding_end: [u8; 36],
}
impl FAnimNode_ScaleChainLength {}
#[repr(C, align(8))]
pub struct FAnimNode_SplineIK {
    #[doc(hidden)]
    __padding_472: [u8; 472],
    pub control_points: TArray<crate::bindings::core_u_object::FTransform>,
    pub roll: f32,
    pub twist_start: f32,
    pub twist_end: f32,
    #[doc(hidden)]
    __padding_552: [u8; 52],
    pub stretch: f32,
    pub offset: f32,
    __padding_end: [u8; 296],
}
impl FAnimNode_SplineIK {}
#[repr(C, align(8))]
pub struct FAnimNode_SpringBone {
    #[doc(hidden)]
    __padding_448: [u8; 448],
    pub max_displacement: f64,
    __padding_end: [u8; 160],
}
impl FAnimNode_SpringBone {}
#[repr(C, align(16))]
pub struct FAnimNode_Trail {
    #[doc(hidden)]
    __padding_568: [u8; 568],
    pub relaxation_speed_scale: f32,
    #[doc(hidden)]
    __padding_712: [u8; 140],
    pub relaxation_speed_scale_input_processor: crate::bindings::engine::FInputScaleBiasClamp,
    #[doc(hidden)]
    __padding_776: [u8; 16],
    pub rotation_offsets: TArray<crate::bindings::core_u_object::FVector>,
    __padding_end: [u8; 168],
}
impl FAnimNode_Trail {}
#[repr(C, align(8))]
pub struct FAnimNode_TwistCorrectiveNode {
    __padding_end: [u8; 616],
}
impl FAnimNode_TwistCorrectiveNode {}
#[repr(C, align(16))]
pub struct FAnimNode_TwoBoneIK {
    #[doc(hidden)]
    __padding_512: [u8; 512],
    pub effector_location: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    __padding_704: [u8; 168],
    pub joint_target_location: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 312],
}
impl FAnimNode_TwoBoneIK {}
#[repr(C, align(8))]
pub struct FIKFootPelvisPullDownSolver {
    __padding_end: [u8; 128],
}
impl FIKFootPelvisPullDownSolver {}
#[repr(C, align(8))]
pub struct FWarpingVectorValue {
    pub mode: EWarpingVectorMode,
    pub value: crate::bindings::core_u_object::FVector,
}
impl FWarpingVectorValue {}
#[repr(C, align(8))]
pub struct FLayeredBoneBlendReference {
    __padding_end: [u8; 16],
}
impl FLayeredBoneBlendReference {}
#[repr(C, align(8))]
pub struct FLinkedAnimGraphReference {
    __padding_end: [u8; 16],
}
impl FLinkedAnimGraphReference {}
#[repr(C, align(8))]
pub struct FMirrorAnimNodeReference {
    __padding_end: [u8; 16],
}
impl FMirrorAnimNodeReference {}
#[repr(C, align(8))]
pub struct FModifyCurveAnimNodeReference {
    __padding_end: [u8; 16],
}
impl FModifyCurveAnimNodeReference {}
#[repr(C, align(8))]
pub struct FSequenceEvaluatorReference {
    __padding_end: [u8; 16],
}
impl FSequenceEvaluatorReference {}
#[repr(C, align(8))]
pub struct FSequencePlayerReference {
    __padding_end: [u8; 16],
}
impl FSequencePlayerReference {}
#[repr(C, align(8))]
pub struct FSkeletalControlReference {
    __padding_end: [u8; 16],
}
impl FSkeletalControlReference {}
pub struct ISequencerAnimationOverride {}
#[repr(C, align(8))]
pub struct USequencerAnimationOverride {
    __padding_end: [u8; 48],
}
impl USequencerAnimationOverride {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationOverride")
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
pub struct UAnimationStateMachineLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationStateMachineLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationStateMachineLibrary")
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
pub struct UAnimExecutionContextLibrary {
    __padding_end: [u8; 48],
}
impl UAnimExecutionContextLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimExecutionContextLibrary")
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
pub struct UAnimNotify_PlayMontageNotify {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub notify_name: FName,
    __padding_end: [u8; 4],
}
impl UAnimNotify_PlayMontageNotify {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotify")
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
pub struct UAnimNotify_PlayMontageNotifyWindow {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub notify_name: FName,
    __padding_end: [u8; 4],
}
impl UAnimNotify_PlayMontageNotifyWindow {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_PlayMontageNotifyWindow")
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
pub struct UAnimSequencerInstance {
    __padding_end: [u8; 1136],
}
impl UAnimSequencerInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequencerInstance")
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
pub struct UBlendListBaseLibrary {
    __padding_end: [u8; 48],
}
impl UBlendListBaseLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendListBaseLibrary")
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
pub struct UBlendSpaceLibrary {
    __padding_end: [u8; 48],
}
impl UBlendSpaceLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceLibrary")
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
pub struct UBlendSpacePlayerLibrary {
    __padding_end: [u8; 48],
}
impl UBlendSpacePlayerLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpacePlayerLibrary")
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
pub struct UAnimNodeRigidBodyLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNodeRigidBodyLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNodeRigidBodyLibrary")
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
pub struct UKismetAnimationLibrary {
    __padding_end: [u8; 48],
}
impl UKismetAnimationLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKismetAnimationLibrary")
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
pub struct ULayeredBoneBlendLibrary {
    __padding_end: [u8; 48],
}
impl ULayeredBoneBlendLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayeredBoneBlendLibrary")
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
pub struct ULinkedAnimGraphLibrary {
    __padding_end: [u8; 48],
}
impl ULinkedAnimGraphLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinkedAnimGraphLibrary")
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
pub struct UMirrorAnimLibrary {
    __padding_end: [u8; 48],
}
impl UMirrorAnimLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorAnimLibrary")
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
pub struct UModifyCurveAnimLibrary {
    __padding_end: [u8; 48],
}
impl UModifyCurveAnimLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyCurveAnimLibrary")
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
pub struct UPlayMontageCallbackProxy {
    __padding_end: [u8; 232],
}
impl UPlayMontageCallbackProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayMontageCallbackProxy")
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
pub struct USequenceEvaluatorLibrary {
    __padding_end: [u8; 48],
}
impl USequenceEvaluatorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceEvaluatorLibrary")
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
pub struct USequencePlayerLibrary {
    __padding_end: [u8; 48],
}
impl USequencePlayerLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencePlayerLibrary")
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
pub struct ISequencerAnimationSupport {}
#[repr(C, align(8))]
pub struct USequencerAnimationSupport {
    __padding_end: [u8; 48],
}
impl USequencerAnimationSupport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerAnimationSupport")
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
pub struct USkeletalControlLibrary {
    __padding_end: [u8; 48],
}
impl USkeletalControlLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalControlLibrary")
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
pub struct FPlayMontageCallbackProxy_OnCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnBlendOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnInterrupted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnNotifyBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPlayMontageCallbackProxy_OnNotifyEnd {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERotationComponent(pub u8);
impl ERotationComponent {
    pub const EULER_X: ERotationComponent = ERotationComponent(0);
    pub const EULER_Y: ERotationComponent = ERotationComponent(1);
    pub const EULER_Z: ERotationComponent = ERotationComponent(2);
    pub const QUATERNION_ANGLE: ERotationComponent = ERotationComponent(3);
    pub const SWING_ANGLE: ERotationComponent = ERotationComponent(4);
    pub const TWIST_ANGLE: ERotationComponent = ERotationComponent(5);
}
#[repr(transparent)]
pub struct EEasingFuncType(pub u8);
impl EEasingFuncType {
    pub const LINEAR: EEasingFuncType = EEasingFuncType(0);
    pub const SINUSOIDAL: EEasingFuncType = EEasingFuncType(1);
    pub const CUBIC: EEasingFuncType = EEasingFuncType(2);
    pub const QUADRATIC_IN_OUT: EEasingFuncType = EEasingFuncType(3);
    pub const CUBIC_IN_OUT: EEasingFuncType = EEasingFuncType(4);
    pub const HERMITE_CUBIC: EEasingFuncType = EEasingFuncType(5);
    pub const QUARTIC_IN_OUT: EEasingFuncType = EEasingFuncType(6);
    pub const QUINTIC_IN_OUT: EEasingFuncType = EEasingFuncType(7);
    pub const CIRCULAR_IN: EEasingFuncType = EEasingFuncType(8);
    pub const CIRCULAR_OUT: EEasingFuncType = EEasingFuncType(9);
    pub const CIRCULAR_IN_OUT: EEasingFuncType = EEasingFuncType(10);
    pub const EXP_IN: EEasingFuncType = EEasingFuncType(11);
    pub const EXP_OUT: EEasingFuncType = EEasingFuncType(12);
    pub const EXP_IN_OUT: EEasingFuncType = EEasingFuncType(13);
    pub const CUSTOM_CURVE: EEasingFuncType = EEasingFuncType(14);
}
#[repr(transparent)]
pub struct EBlendListTransitionType(pub u8);
impl EBlendListTransitionType {
    pub const STANDARD_BLEND: EBlendListTransitionType = EBlendListTransitionType(0);
    pub const INERTIALIZATION: EBlendListTransitionType = EBlendListTransitionType(1);
}
#[repr(transparent)]
pub struct EBlendListChildUpdateMode(pub u8);
impl EBlendListChildUpdateMode {
    pub const DEFAULT: EBlendListChildUpdateMode = EBlendListChildUpdateMode(0);
    pub const RESET_CHILD_ON_ACTIVATE: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        1,
    );
    pub const ALWAYS_TICK_CHILDREN: EBlendListChildUpdateMode = EBlendListChildUpdateMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAnimFunctionCallSite(pub i32);
impl EAnimFunctionCallSite {
    pub const ON_INITIALIZE: EAnimFunctionCallSite = EAnimFunctionCallSite(0);
    pub const ON_UPDATE: EAnimFunctionCallSite = EAnimFunctionCallSite(1);
    pub const ON_BECOME_RELEVANT: EAnimFunctionCallSite = EAnimFunctionCallSite(2);
    pub const ON_EVALUATE: EAnimFunctionCallSite = EAnimFunctionCallSite(3);
    pub const ON_INITIALIZE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        4,
    );
    pub const ON_UPDATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(5);
    pub const ON_BECOME_RELEVANT_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        6,
    );
    pub const ON_EVALUATE_POST_RECURSION: EAnimFunctionCallSite = EAnimFunctionCallSite(
        7,
    );
    pub const ON_STARTED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(8);
    pub const ON_STARTED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(9);
    pub const ON_FINISHED_BLENDING_OUT: EAnimFunctionCallSite = EAnimFunctionCallSite(
        10,
    );
    pub const ON_FINISHED_BLENDING_IN: EAnimFunctionCallSite = EAnimFunctionCallSite(11);
}
#[repr(transparent)]
pub struct ELayeredBoneBlendMode(pub u8);
impl ELayeredBoneBlendMode {
    pub const BRANCH_FILTER: ELayeredBoneBlendMode = ELayeredBoneBlendMode(0);
    pub const BLEND_MASK: ELayeredBoneBlendMode = ELayeredBoneBlendMode(1);
}
#[repr(transparent)]
pub struct EModifyCurveApplyMode(pub u8);
impl EModifyCurveApplyMode {
    pub const ADD: EModifyCurveApplyMode = EModifyCurveApplyMode(0);
    pub const SCALE: EModifyCurveApplyMode = EModifyCurveApplyMode(1);
    pub const BLEND: EModifyCurveApplyMode = EModifyCurveApplyMode(2);
    pub const WEIGHTED_MOVING_AVERAGE: EModifyCurveApplyMode = EModifyCurveApplyMode(3);
    pub const REMAP_CURVE: EModifyCurveApplyMode = EModifyCurveApplyMode(4);
}
#[repr(transparent)]
pub struct ERBFDistanceMethod(pub u8);
impl ERBFDistanceMethod {
    pub const EUCLIDEAN: ERBFDistanceMethod = ERBFDistanceMethod(0);
    pub const QUATERNION: ERBFDistanceMethod = ERBFDistanceMethod(1);
    pub const SWING_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(2);
    pub const TWIST_ANGLE: ERBFDistanceMethod = ERBFDistanceMethod(3);
    pub const DEFAULT_METHOD: ERBFDistanceMethod = ERBFDistanceMethod(4);
}
#[repr(transparent)]
pub struct ERBFFunctionType(pub u8);
impl ERBFFunctionType {
    pub const GAUSSIAN: ERBFFunctionType = ERBFFunctionType(0);
    pub const EXPONENTIAL: ERBFFunctionType = ERBFFunctionType(1);
    pub const LINEAR: ERBFFunctionType = ERBFFunctionType(2);
    pub const CUBIC: ERBFFunctionType = ERBFFunctionType(3);
    pub const QUINTIC: ERBFFunctionType = ERBFFunctionType(4);
    pub const DEFAULT_FUNCTION: ERBFFunctionType = ERBFFunctionType(5);
}
#[repr(transparent)]
pub struct ERBFSolverType(pub u8);
impl ERBFSolverType {
    pub const ADDITIVE: ERBFSolverType = ERBFSolverType(0);
    pub const INTERPOLATIVE: ERBFSolverType = ERBFSolverType(1);
}
#[repr(transparent)]
pub struct EPoseDriverSource(pub u8);
impl EPoseDriverSource {
    pub const ROTATION: EPoseDriverSource = EPoseDriverSource(0);
    pub const TRANSLATION: EPoseDriverSource = EPoseDriverSource(1);
}
#[repr(transparent)]
pub struct EPoseDriverOutput(pub u8);
impl EPoseDriverOutput {
    pub const DRIVE_POSES: EPoseDriverOutput = EPoseDriverOutput(0);
    pub const DRIVE_CURVES: EPoseDriverOutput = EPoseDriverOutput(1);
}
#[repr(transparent)]
pub struct EPoseDriverType(pub u8);
impl EPoseDriverType {
    pub const SWING_AND_TWIST: EPoseDriverType = EPoseDriverType(0);
    pub const SWING_ONLY: EPoseDriverType = EPoseDriverType(1);
    pub const TRANSLATION: EPoseDriverType = EPoseDriverType(2);
}
#[repr(transparent)]
pub struct ERBFNormalizeMethod(pub u8);
impl ERBFNormalizeMethod {
    pub const ONLY_NORMALIZE_ABOVE_ONE: ERBFNormalizeMethod = ERBFNormalizeMethod(0);
    pub const ALWAYS_NORMALIZE: ERBFNormalizeMethod = ERBFNormalizeMethod(1);
    pub const NORMALIZE_WITHIN_MEDIAN: ERBFNormalizeMethod = ERBFNormalizeMethod(2);
    pub const NO_NORMALIZATION: ERBFNormalizeMethod = ERBFNormalizeMethod(3);
}
#[repr(transparent)]
pub struct ESnapshotSourceMode(pub u8);
impl ESnapshotSourceMode {
    pub const NAMED_SNAPSHOT: ESnapshotSourceMode = ESnapshotSourceMode(0);
    pub const SNAPSHOT_PIN: ESnapshotSourceMode = ESnapshotSourceMode(1);
}
#[repr(transparent)]
pub struct ERefPoseType(pub u8);
impl ERefPoseType {
    pub const EIT_LOCAL_SPACE: ERefPoseType = ERefPoseType(0);
    pub const EIT_ADDITIVE: ERefPoseType = ERefPoseType(1);
}
#[repr(transparent)]
pub struct ESequenceEvalReinit(pub u8);
impl ESequenceEvalReinit {
    pub const NO_RESET: ESequenceEvalReinit = ESequenceEvalReinit(0);
    pub const START_POSITION: ESequenceEvalReinit = ESequenceEvalReinit(1);
    pub const EXPLICIT_TIME: ESequenceEvalReinit = ESequenceEvalReinit(2);
}
#[repr(transparent)]
pub struct AnimPhysLinearConstraintType(pub u8);
impl AnimPhysLinearConstraintType {
    pub const FREE: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(0);
    pub const LIMITED: AnimPhysLinearConstraintType = AnimPhysLinearConstraintType(1);
}
#[repr(transparent)]
pub struct AnimPhysAngularConstraintType(pub u8);
impl AnimPhysAngularConstraintType {
    pub const ANGULAR: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(0);
    pub const CONE: AnimPhysAngularConstraintType = AnimPhysAngularConstraintType(1);
}
#[repr(transparent)]
pub struct ESphericalLimitType(pub u8);
impl ESphericalLimitType {
    pub const INNER: ESphericalLimitType = ESphericalLimitType(0);
    pub const OUTER: ESphericalLimitType = ESphericalLimitType(1);
}
#[repr(transparent)]
pub struct AnimPhysSimSpaceType(pub u8);
impl AnimPhysSimSpaceType {
    pub const COMPONENT: AnimPhysSimSpaceType = AnimPhysSimSpaceType(0);
    pub const ACTOR: AnimPhysSimSpaceType = AnimPhysSimSpaceType(1);
    pub const WORLD: AnimPhysSimSpaceType = AnimPhysSimSpaceType(2);
    pub const ROOT_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(3);
    pub const BONE_RELATIVE: AnimPhysSimSpaceType = AnimPhysSimSpaceType(4);
}
#[repr(transparent)]
pub struct EDrivenDestinationMode(pub u8);
impl EDrivenDestinationMode {
    pub const BONE: EDrivenDestinationMode = EDrivenDestinationMode(0);
    pub const MORPH_TARGET: EDrivenDestinationMode = EDrivenDestinationMode(1);
    pub const MATERIAL_PARAMETER: EDrivenDestinationMode = EDrivenDestinationMode(2);
}
#[repr(transparent)]
pub struct EDrivenBoneModificationMode(pub u8);
impl EDrivenBoneModificationMode {
    pub const ADD_TO_INPUT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(0);
    pub const REPLACE_COMPONENT: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        1,
    );
    pub const ADD_TO_REF_POSE: EDrivenBoneModificationMode = EDrivenBoneModificationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EConstraintOffsetOption(pub u8);
impl EConstraintOffsetOption {
    pub const NONE: EConstraintOffsetOption = EConstraintOffsetOption(0);
    pub const OFFSET_REF_POSE: EConstraintOffsetOption = EConstraintOffsetOption(1);
}
#[repr(transparent)]
pub struct CopyBoneDeltaMode(pub u8);
impl CopyBoneDeltaMode {
    pub const ACCUMULATE: CopyBoneDeltaMode = CopyBoneDeltaMode(0);
    pub const COPY: CopyBoneDeltaMode = CopyBoneDeltaMode(1);
}
#[repr(transparent)]
pub struct EInterpolationBlend(pub u8);
impl EInterpolationBlend {
    pub const LINEAR: EInterpolationBlend = EInterpolationBlend(0);
    pub const CUBIC: EInterpolationBlend = EInterpolationBlend(1);
    pub const SINUSOIDAL: EInterpolationBlend = EInterpolationBlend(2);
    pub const EASE_IN_OUT_EXPONENT2: EInterpolationBlend = EInterpolationBlend(3);
    pub const EASE_IN_OUT_EXPONENT3: EInterpolationBlend = EInterpolationBlend(4);
    pub const EASE_IN_OUT_EXPONENT4: EInterpolationBlend = EInterpolationBlend(5);
    pub const EASE_IN_OUT_EXPONENT5: EInterpolationBlend = EInterpolationBlend(6);
    pub const MAX: EInterpolationBlend = EInterpolationBlend(7);
}
#[repr(transparent)]
pub struct EBoneModificationMode(pub u8);
impl EBoneModificationMode {
    pub const BMM_IGNORE: EBoneModificationMode = EBoneModificationMode(0);
    pub const BMM_REPLACE: EBoneModificationMode = EBoneModificationMode(1);
    pub const BMM_ADDITIVE: EBoneModificationMode = EBoneModificationMode(2);
}
#[repr(transparent)]
pub struct ESimulationSpace(pub u8);
impl ESimulationSpace {
    pub const COMPONENT_SPACE: ESimulationSpace = ESimulationSpace(0);
    pub const WORLD_SPACE: ESimulationSpace = ESimulationSpace(1);
    pub const BASE_BONE_SPACE: ESimulationSpace = ESimulationSpace(2);
}
#[repr(transparent)]
pub struct ESimulationTiming(pub u8);
impl ESimulationTiming {
    pub const DEFAULT: ESimulationTiming = ESimulationTiming(0);
    pub const SYNCHRONOUS: ESimulationTiming = ESimulationTiming(1);
    pub const DEFERRED: ESimulationTiming = ESimulationTiming(2);
}
#[repr(transparent)]
pub struct EScaleChainInitialLength(pub u8);
impl EScaleChainInitialLength {
    pub const FIXED_DEFAULT_LENGTH_VALUE: EScaleChainInitialLength = EScaleChainInitialLength(
        0,
    );
    pub const DISTANCE: EScaleChainInitialLength = EScaleChainInitialLength(1);
    pub const CHAIN_LENGTH: EScaleChainInitialLength = EScaleChainInitialLength(2);
}
#[repr(transparent)]
pub struct ESplineBoneAxis(pub u8);
impl ESplineBoneAxis {
    pub const NONE: ESplineBoneAxis = ESplineBoneAxis(0);
    pub const X: ESplineBoneAxis = ESplineBoneAxis(1);
    pub const Y: ESplineBoneAxis = ESplineBoneAxis(2);
    pub const Z: ESplineBoneAxis = ESplineBoneAxis(3);
}
#[repr(transparent)]
pub struct EWarpingVectorMode(pub u8);
impl EWarpingVectorMode {
    pub const COMPONENT_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(0);
    pub const ACTOR_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(1);
    pub const WORLD_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(2);
    pub const IK_FOOT_ROOT_LOCAL_SPACE_VECTOR: EWarpingVectorMode = EWarpingVectorMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESwapRootBone(pub u8);
impl ESwapRootBone {
    pub const SWAP_ROOT_BONE_COMPONENT: ESwapRootBone = ESwapRootBone(0);
    pub const SWAP_ROOT_BONE_ACTOR: ESwapRootBone = ESwapRootBone(1);
    pub const SWAP_ROOT_BONE_NONE: ESwapRootBone = ESwapRootBone(2);
}
