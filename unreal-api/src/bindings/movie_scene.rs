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
pub static mut U_MOVIE_SCENE_SECTION_SET_ROW_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_PRE_ROLL_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_POST_ROLL_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_OVERLAP_PRIORITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_IS_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_IS_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_COMPLETION_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_COLOR_TINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_SET_BLEND_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_IS_LOCKED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_IS_ACTIVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_ROW_INDEX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_PRE_ROLL_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_POST_ROLL_FRAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_OVERLAP_PRIORITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_COMPLETION_MODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_COLOR_TINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SECTION_GET_BLEND_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_GET_EARLIEST_TIMECODE_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_FIND_BINDINGS_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_FIND_BINDING_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_BINDING_GET_BASE_ENGINE_PRIORITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_BINDING_GET_BASE_CUSTOM_PRIORITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CONDITION_BP_GET_SCOPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CONDITION_BP_GET_CHECK_FREQUENCY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CONDITION_BP_EVALUATE_CONDITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SUB_SECTION_SET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SUB_SECTION_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_BOUND_OBJECT_PROXY_BP_GET_BOUND_OBJECT_FOR_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_BINDING_EVENT_RECEIVER_INTERFACE_ON_OBJECT_UNBOUND_BY_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_BINDING_EVENT_RECEIVER_INTERFACE_ON_OBJECT_BOUND_BY_SEQUENCER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_EASING_FUNCTION_ON_EVALUATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_STOP_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_START_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_REQUEST_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_BUILT_IN_DYNAMIC_BINDING_RESOLVER_LIBRARY_RESOLVE_TO_PLAYER_PAWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_SET_NOTES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_SET_CREATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_SET_AUTHOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_GET_NOTES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_GET_CREATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_META_DATA_GET_AUTHOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_STOP_AT_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_STOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_WEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_TIME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_PLAYBACK_POSITION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_HIDE_HUD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_FRAME_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_DISABLE_CAMERA_CUTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_COMPLETION_MODE_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SCRUB: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_ON_STOP_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_ON_FINISH_PLAYBACK_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_EXPLICIT_SERVER_UPDATE_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RESTORE_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_REQUEST_INVALIDATE_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_REMOVE_WEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_TO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_REVERSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_LOOPING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PAUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_REVERSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_PAUSED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GO_TO_END_AND_STOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_START_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_SEQUENCE_NAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_PLAY_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_OBJECT_BINDINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_HIDE_HUD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_FRAME_RATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_FRAME_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_END_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_DURATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_DISABLE_CAMERA_CUTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_COMPLETION_MODE_OVERRIDE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_BOUND_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_SEQUENCE_PLAYER_CHANGE_PLAYBACK_DIRECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_TEST_MOVIE_SCENE_ARRAY_PROPERTIES_ACTOR_SET_TEST_SETTER_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRowIndex"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_ROW_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreRollFrames"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_PRE_ROLL_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPostRollFrames"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_POST_ROLL_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOverlapPriority"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_OVERLAP_PRIORITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsLocked"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_IS_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsActive"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_IS_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCompletionMode"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_COMPLETION_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorTint"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_COLOR_TINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlendType"),
            &raw mut U_MOVIE_SCENE_SECTION_SET_BLEND_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLocked"),
            &raw mut U_MOVIE_SCENE_SECTION_IS_LOCKED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActive"),
            &raw mut U_MOVIE_SCENE_SECTION_IS_ACTIVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRowIndex"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_ROW_INDEX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreRollFrames"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_PRE_ROLL_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPostRollFrames"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_POST_ROLL_FRAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOverlapPriority"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_OVERLAP_PRIORITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCompletionMode"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_COMPLETION_MODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColorTint"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_COLOR_TINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlendType"),
            &raw mut U_MOVIE_SCENE_SECTION_GET_BLEND_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSequence::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEarliestTimecodeSource"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_GET_EARLIEST_TIMECODE_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBindingsByTag"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_FIND_BINDINGS_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBindingByTag"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_FIND_BINDING_BY_TAG,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCustomBinding::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBaseEnginePriority"),
            &raw mut U_MOVIE_SCENE_CUSTOM_BINDING_GET_BASE_ENGINE_PRIORITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBaseCustomPriority"),
            &raw mut U_MOVIE_SCENE_CUSTOM_BINDING_GET_BASE_CUSTOM_PRIORITY,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCondition::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetScope"),
            &raw mut U_MOVIE_SCENE_CONDITION_BP_GET_SCOPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetCheckFrequency"),
            &raw mut U_MOVIE_SCENE_CONDITION_BP_GET_CHECK_FREQUENCY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_EvaluateCondition"),
            &raw mut U_MOVIE_SCENE_CONDITION_BP_EVALUATE_CONDITION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSubSection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequence"),
            &raw mut U_MOVIE_SCENE_SUB_SECTION_SET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_MOVIE_SCENE_SUB_SECTION_GET_SEQUENCE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneBoundObjectProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BP_GetBoundObjectForSequencer"),
            &raw mut U_MOVIE_SCENE_BOUND_OBJECT_PROXY_BP_GET_BOUND_OBJECT_FOR_SEQUENCER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneBindingEventReceiverInterface::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnObjectUnboundBySequencer"),
            &raw mut U_MOVIE_SCENE_BINDING_EVENT_RECEIVER_INTERFACE_ON_OBJECT_UNBOUND_BY_SEQUENCER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnObjectBoundBySequencer"),
            &raw mut U_MOVIE_SCENE_BINDING_EVENT_RECEIVER_INTERFACE_ON_OBJECT_BOUND_BY_SEQUENCER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneEasingFunction::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnEvaluate"),
            &raw mut U_MOVIE_SCENE_EASING_FUNCTION_ON_EVALUATE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCustomClockSource::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTick"),
            &raw mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStopPlaying"),
            &raw mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_STOP_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStartPlaying"),
            &raw mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_START_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRequestCurrentTime"),
            &raw mut U_MOVIE_SCENE_CUSTOM_CLOCK_SOURCE_ON_REQUEST_CURRENT_TIME,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBuiltInDynamicBindingResolverLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveToPlayerPawn"),
            &raw mut U_BUILT_IN_DYNAMIC_BINDING_RESOLVER_LIBRARY_RESOLVE_TO_PLAYER_PAWN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneMetaData::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNotes"),
            &raw mut U_MOVIE_SCENE_META_DATA_SET_NOTES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCreated"),
            &raw mut U_MOVIE_SCENE_META_DATA_SET_CREATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAuthor"),
            &raw mut U_MOVIE_SCENE_META_DATA_SET_AUTHOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNotes"),
            &raw mut U_MOVIE_SCENE_META_DATA_GET_NOTES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCreated"),
            &raw mut U_MOVIE_SCENE_META_DATA_GET_CREATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAuthor"),
            &raw mut U_MOVIE_SCENE_META_DATA_GET_AUTHOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSequencePlayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopAtCurrentTime"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_STOP_AT_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Stop"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_STOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWeight"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_WEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTimeRange"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_TIME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayRate"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackPosition"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_PLAYBACK_POSITION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHideHud"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_HIDE_HUD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRate"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFrameRange"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_FRAME_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisableCameraCuts"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_DISABLE_CAMERA_CUTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCompletionModeOverride"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SET_COMPLETION_MODE_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Scrub"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_SCRUB,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RPC_OnStopEvent"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_ON_STOP_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RPC_OnFinishPlaybackEvent"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_ON_FINISH_PLAYBACK_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RPC_ExplicitServerUpdateEvent"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RPC_EXPLICIT_SERVER_UPDATE_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RestoreState"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_RESTORE_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestInvalidateBinding"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_REQUEST_INVALIDATE_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveWeight"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_REMOVE_WEIGHT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayTo"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_TO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayReverse"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_REVERSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayLooping"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY_LOOPING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_PAUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReversed"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_REVERSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaying"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPaused"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_IS_PAUSED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GoToEndAndStop"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GO_TO_END_AND_STOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartTime"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_START_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequenceName"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_SEQUENCE_NAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlayRate"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_PLAY_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectBindings"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_OBJECT_BINDINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHideHud"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_HIDE_HUD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameRate"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_FRAME_RATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameDuration"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_FRAME_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndTime"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_END_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDuration"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_DURATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisableCameraCuts"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_DISABLE_CAMERA_CUTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentTime"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCompletionModeOverride"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_COMPLETION_MODE_OVERRIDE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjects"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_GET_BOUND_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangePlaybackDirection"),
            &raw mut U_MOVIE_SCENE_SEQUENCE_PLAYER_CHANGE_PLAYBACK_DIRECTION,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ATestMovieSceneArrayPropertiesActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTestSetterFloat"),
            &raw mut A_TEST_MOVIE_SCENE_ARRAY_PROPERTIES_ACTOR_SET_TEST_SETTER_FLOAT,
        );
    }
}
#[repr(C, align(8))]
pub struct FMovieSceneBindingProxy {
    pub binding_id: crate::bindings::core_u_object::FGuid,
    pub sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneBindingProxy {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingPayloadVariable {
    __padding_end: [u8; 56],
}
impl FMovieSceneDynamicBindingPayloadVariable {}
#[repr(C, align(4))]
pub struct FMovieSceneObjectBindingID {
    __padding_end: [u8; 28],
}
impl FMovieSceneObjectBindingID {}
#[repr(C, align(8))]
pub struct FMovieSceneTimeWarpVariant {
    __padding_end: [u8; 16],
}
impl FMovieSceneTimeWarpVariant {}
#[repr(C, align(8))]
pub struct FMovieSceneNumericVariant {
    __padding_end: [u8; 16],
}
impl FMovieSceneNumericVariant {}
#[repr(C, align(4))]
pub struct FActorForWorldTransforms {
    pub actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
}
impl FActorForWorldTransforms {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionParameters {
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub b_can_loop: bool,
    pub end_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub time_scale: FMovieSceneTimeWarpVariant,
    pub hierarchical_bias: i32,
    pub flags: EMovieSceneSubSectionFlags,
    __padding_end: [u8; 19],
}
impl FMovieSceneSectionParameters {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceLoopCount {
    pub value: i32,
}
impl FMovieSceneSequenceLoopCount {}
#[repr(C, align(4))]
pub struct FMovieSceneSequencePlaybackSettings {
    pub flags_0: u8,
    pub loop_count: FMovieSceneSequenceLoopCount,
    pub tick_interval: FMovieSceneSequenceTickInterval,
    pub play_rate: f32,
    pub start_time: f32,
    pub flags_28: u8,
    #[doc(hidden)]
    __padding_32: [u8; 3],
    pub finish_completion_state_override: EMovieSceneCompletionModeOverride,
    #[doc(hidden)]
    __padding_36: [u8; 3],
    pub flags_36: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSequencePlaybackSettings {}
#[repr(C, align(4))]
pub struct FMovieSceneSequenceTickInterval {
    pub tick_interval_seconds: f32,
    pub evaluation_budget_microseconds: f32,
    pub b_tick_when_paused: bool,
    pub b_allow_rounding: bool,
    __padding_end: [u8; 2],
}
impl FMovieSceneSequenceTickInterval {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FMovieSceneBindingResolveResult {}
#[repr(C, align(8))]
pub struct FMovieSceneBindingResolveContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
}
impl FMovieSceneBindingResolveContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContext {
    pub world_context: UPtr<crate::bindings::core_u_object::UObject>,
    pub binding: FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl FMovieSceneConditionContext {}
#[repr(C, align(8))]
pub struct FMovieSceneConditionContainer {
    pub condition: UPtr<UMovieSceneCondition>,
}
impl FMovieSceneConditionContainer {}
#[repr(C, align(1))]
pub struct FOptionalMovieSceneBlendType {
    pub blend_type: EMovieSceneBlendType,
    pub b_is_valid: bool,
}
impl FOptionalMovieSceneBlendType {}
#[repr(C, align(8))]
pub struct FMovieSceneMarkedFrame {
    pub frame_number: crate::bindings::core_u_object::FFrameNumber,
    pub label: FString,
    #[doc(hidden)]
    __padding_76: [u8; 52],
    pub b_is_determinism_fence: bool,
    pub b_is_inclusive_time: bool,
    __padding_end: [u8; 2],
}
impl FMovieSceneMarkedFrame {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveParams {
    pub sequence: UPtr<UMovieSceneSequence>,
    pub object_binding_id: crate::bindings::core_u_object::FGuid,
    pub root_sequence: UPtr<UMovieSceneSequence>,
}
impl FMovieSceneDynamicBindingResolveParams {}
#[repr(C, align(8))]
pub struct FMovieSceneDynamicBindingResolveResult {
    pub objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub object: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_is_possessed_object: bool,
    __padding_end: [u8; 7],
}
impl FMovieSceneDynamicBindingResolveResult {}
#[repr(C, align(4))]
pub struct FMovieSceneTimecodeSource {
    pub timecode: crate::bindings::core_u_object::FTimecode,
}
impl FMovieSceneTimecodeSource {}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlaybackParams {
    pub frame: crate::bindings::core_u_object::FFrameTime,
    pub time: f32,
    pub marked_frame: FString,
    pub timecode: crate::bindings::core_u_object::FTimecode,
    pub position_type: EMovieScenePositionType,
    pub update_method: EUpdatePositionMethod,
    pub b_has_jumped: bool,
    __padding_end: [u8; 5],
}
impl FMovieSceneSequencePlaybackParams {}
#[repr(C, align(1))]
pub struct FMovieSceneSequencePlayToParams {
    pub b_exclusive: bool,
}
impl FMovieSceneSequencePlayToParams {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersSeconds {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: f32,
    pub inner_end_offset: f32,
    pub first_loop_start_offset: f32,
    pub flags_28: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSectionTimingParametersSeconds {}
#[repr(C, align(8))]
pub struct FMovieSceneSectionTimingParametersFrames {
    pub play_rate: FMovieSceneTimeWarpVariant,
    pub inner_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub inner_end_offset: crate::bindings::core_u_object::FFrameNumber,
    pub first_loop_start_offset: crate::bindings::core_u_object::FFrameNumber,
    pub flags_28: u8,
    __padding_end: [u8; 3],
}
impl FMovieSceneSectionTimingParametersFrames {}
#[repr(C, align(8))]
pub struct FTestMovieSceneStruct {
    pub first: f32,
    pub second: f32,
    pub enum_: ETestMovieSceneEnum,
    pub vector: crate::bindings::core_u_object::FVector,
    pub multiple_integers: TArray<i32>,
    pub multiple_vectors: TArray<crate::bindings::core_u_object::FVector>,
}
impl FTestMovieSceneStruct {}
#[repr(C, align(4))]
pub struct FTestMovieSceneStruct2 {
    pub first: f32,
    pub second: f32,
}
impl FTestMovieSceneStruct2 {}
#[repr(C, align(8))]
pub struct UMovieSceneEntitySystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntitySystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystem")
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
pub struct UMovieSceneSignedObject {
    __padding_end: [u8; 112],
}
impl UMovieSceneSignedObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSignedObject")
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
pub struct UMovieSceneDecorationContainerObject {
    __padding_end: [u8; 128],
}
impl UMovieSceneDecorationContainerObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecorationContainerObject")
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
pub struct UMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UMovieSceneSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSection")
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
pub struct UMovieSceneTrack {
    __padding_end: [u8; 352],
}
impl UMovieSceneTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrack")
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
pub struct UMovieSceneNameableTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneNameableTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNameableTrack")
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
pub struct UMovieSceneSequence {
    __padding_end: [u8; 128],
}
impl UMovieSceneSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequence")
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
pub struct UMovieSceneCustomBinding {
    __padding_end: [u8; 48],
}
impl UMovieSceneCustomBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomBinding")
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
pub struct UMovieSceneReplaceableBindingBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub preview_spawnable: UPtr<UMovieSceneSpawnableBindingBase>,
}
impl UMovieSceneReplaceableBindingBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneReplaceableBindingBase")
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
pub struct UMovieSceneSpawnableBindingBase {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub spawn_ownership: ESpawnOwnership,
    pub b_continuously_respawn: bool,
    __padding_end: [u8; 6],
}
impl UMovieSceneSpawnableBindingBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnableBindingBase")
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
pub struct UMovieSceneChannelOverrideContainer {
    __padding_end: [u8; 112],
}
impl UMovieSceneChannelOverrideContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideContainer")
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
pub struct UMovieSceneCondition {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub b_editor_force_true: bool,
    pub b_invert: bool,
    __padding_end: [u8; 6],
}
impl UMovieSceneCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCondition")
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
pub struct UMovieSceneEntityInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneEntityInstantiatorSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityInstantiatorSystem")
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
pub struct UMovieSceneSubSection {
    #[doc(hidden)]
    __padding_368: [u8; 368],
    pub parameters: FMovieSceneSectionParameters,
    __padding_end: [u8; 2000],
}
impl UMovieSceneSubSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubSection")
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
pub struct UMovieSceneBoolSection {
    __padding_end: [u8; 664],
}
impl UMovieSceneBoolSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoolSection")
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
pub struct UMovieSceneBlenderSystem {
    __padding_end: [u8; 120],
}
impl UMovieSceneBlenderSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystem")
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
pub struct UMovieSceneTrackInstance {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstance")
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
pub struct UMovieSceneSubTrack {
    __padding_end: [u8; 416],
}
impl UMovieSceneSubTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSubTrack")
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
pub struct IMovieSceneBlenderSystemSupport {}
#[repr(C, align(8))]
pub struct UMovieSceneBlenderSystemSupport {
    __padding_end: [u8; 48],
}
impl UMovieSceneBlenderSystemSupport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBlenderSystemSupport")
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
pub struct IMovieSceneBoundObjectProxy {}
#[repr(C, align(8))]
pub struct UMovieSceneBoundObjectProxy {
    __padding_end: [u8; 48],
}
impl UMovieSceneBoundObjectProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundObjectProxy")
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
pub struct IMovieSceneChannelDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelDecoration")
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
pub struct IMovieSceneChannelOverrideProvider {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOverrideProvider {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelOverrideProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOverrideProvider")
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
pub struct IMovieSceneChannelOwner {}
#[repr(C, align(8))]
pub struct UMovieSceneChannelOwner {
    __padding_end: [u8; 48],
}
impl UMovieSceneChannelOwner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneChannelOwner")
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
pub struct IMovieSceneDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDecoration")
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
pub struct IMovieSceneDeterminismSource {}
#[repr(C, align(8))]
pub struct UMovieSceneDeterminismSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneDeterminismSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDeterminismSource")
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
pub struct IMovieSceneEntityDecorator {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityDecorator {
    __padding_end: [u8; 48],
}
impl UMovieSceneEntityDecorator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityDecorator")
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
pub struct IMovieSceneLifetimeDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneLifetimeDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneLifetimeDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLifetimeDecoration")
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
pub struct IMovieSceneMetaDataInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneMetaDataInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneMetaDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaDataInterface")
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
pub struct IMovieScenePlaybackClient {}
#[repr(C, align(8))]
pub struct UMovieScenePlaybackClient {
    __padding_end: [u8; 48],
}
impl UMovieScenePlaybackClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlaybackClient")
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
pub struct IMovieSceneSectionDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneSectionDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneSectionDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionDecoration")
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
pub struct IMovieSceneSequencePlayerObserver {}
#[repr(C, align(8))]
pub struct UMovieSceneSequencePlayerObserver {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequencePlayerObserver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayerObserver")
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
pub struct IMovieSceneTrackDecoration {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackDecoration {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackDecoration")
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
pub struct IMovieSceneBindingEventReceiverInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingEventReceiverInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingEventReceiverInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingEventReceiverInterface")
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
pub struct IMovieSceneBindingOwnerInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneBindingOwnerInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingOwnerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOwnerInterface")
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
pub struct IMovieSceneCachedTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneCachedTrack {
    __padding_end: [u8; 48],
}
impl UMovieSceneCachedTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachedTrack")
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
pub struct IMovieSceneEasingFunction {}
#[repr(C, align(8))]
pub struct UMovieSceneEasingFunction {
    __padding_end: [u8; 48],
}
impl UMovieSceneEasingFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingFunction")
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
pub struct IMovieSceneKeyProxy {}
#[repr(C, align(8))]
pub struct UMovieSceneKeyProxy {
    __padding_end: [u8; 48],
}
impl UMovieSceneKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneKeyProxy")
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
pub struct UMovieSceneNumericVariantGetter {
    __padding_end: [u8; 120],
}
impl UMovieSceneNumericVariantGetter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNumericVariantGetter")
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
pub struct IMovieSceneSequenceTickManagerClient {}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceTickManagerClient {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequenceTickManagerClient {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManagerClient")
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
pub struct UMovieSceneSectionChannelOverrideRegistry {
    __padding_end: [u8; 128],
}
impl UMovieSceneSectionChannelOverrideRegistry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionChannelOverrideRegistry")
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
pub struct IMovieSceneTrackTemplateProducer {}
#[repr(C, align(8))]
pub struct UMovieSceneTrackTemplateProducer {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackTemplateProducer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackTemplateProducer")
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
pub struct UMovieSceneCompiledData {
    __padding_end: [u8; 1080],
}
impl UMovieSceneCompiledData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledData")
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
pub struct UMovieSceneCompiledDataManager {
    __padding_end: [u8; 568],
}
impl UMovieSceneCompiledDataManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCompiledDataManager")
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
pub struct UMovieSceneGroupCondition {
    #[doc(hidden)]
    __padding_120: [u8; 120],
    pub operator: EMovieSceneGroupConditionOperator,
    pub sub_conditions: TArray<FMovieSceneConditionContainer>,
}
impl UMovieSceneGroupCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGroupCondition")
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
pub struct UMovieSceneLanguagePreviewDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneLanguagePreviewDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneLanguagePreviewDecoration")
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
pub struct UMovieSceneMuteSoloDecoration {
    __padding_end: [u8; 64],
}
impl UMovieSceneMuteSoloDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMuteSoloDecoration")
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
pub struct IMovieSceneScalingDriver {}
#[repr(C, align(8))]
pub struct UMovieSceneScalingDriver {
    __padding_end: [u8; 48],
}
impl UMovieSceneScalingDriver {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingDriver")
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
pub struct UMovieSceneTimeWarpGetter {
    __padding_end: [u8; 136],
}
impl UMovieSceneTimeWarpGetter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpGetter")
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
pub struct UMovieScenePlayRateCurve {
    __padding_end: [u8; 496],
}
impl UMovieScenePlayRateCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePlayRateCurve")
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
pub struct UMovieSceneScalingAnchors {
    __padding_end: [u8; 784],
}
impl UMovieSceneScalingAnchors {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScalingAnchors")
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
pub struct UMovieSceneSectionAnchorsDecoration {
    __padding_end: [u8; 88],
}
impl UMovieSceneSectionAnchorsDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionAnchorsDecoration")
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
pub struct IMovieSceneTimeWarpSource {}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneTimeWarpSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSource")
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
pub struct UMovieSceneTimeWarpDecoration {
    __padding_end: [u8; 72],
}
impl UMovieSceneTimeWarpDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpDecoration")
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
pub struct UMovieSceneTrackRowDecoration {
    __padding_end: [u8; 136],
}
impl UMovieSceneTrackRowDecoration {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackRowDecoration")
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
pub struct IMovieSceneEntityProvider {}
#[repr(C, align(8))]
pub struct UMovieSceneEntityProvider {
    __padding_end: [u8; 48],
}
impl UMovieSceneEntityProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityProvider")
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
pub struct UMovieSceneBindingLifetimeSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneBindingLifetimeSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSystem")
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
pub struct UMovieSceneGenericBoundObjectInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneGenericBoundObjectInstantiator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGenericBoundObjectInstantiator")
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
pub struct UMovieSceneBoundSceneComponentInstantiator {
    __padding_end: [u8; 80],
}
impl UMovieSceneBoundSceneComponentInstantiator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBoundSceneComponentInstantiator")
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
pub struct IMovieSceneValueDecomposer {}
#[repr(C, align(8))]
pub struct UMovieSceneValueDecomposer {
    __padding_end: [u8; 48],
}
impl UMovieSceneValueDecomposer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneValueDecomposer")
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
pub struct UMovieSceneEntityGroupingSystem {
    __padding_end: [u8; 360],
}
impl UMovieSceneEntityGroupingSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntityGroupingSystem")
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
pub struct UMovieSceneEntitySystemLinker {
    __padding_end: [u8; 1952],
}
impl UMovieSceneEntitySystemLinker {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEntitySystemLinker")
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
pub struct UMovieSceneEvalTimeSystem {
    __padding_end: [u8; 480],
}
impl UMovieSceneEvalTimeSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvalTimeSystem")
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
pub struct UMovieSceneEvaluationHookSystem {
    __padding_end: [u8; 160],
}
impl UMovieSceneEvaluationHookSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHookSystem")
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
pub struct UMovieSceneInitialValueSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneInitialValueSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneInitialValueSystem")
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
pub struct IMovieScenePreAnimatedStateSystemInterface {}
#[repr(C, align(8))]
pub struct UMovieScenePreAnimatedStateSystemInterface {
    __padding_end: [u8; 48],
}
impl UMovieScenePreAnimatedStateSystemInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePreAnimatedStateSystemInterface")
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
pub struct UMovieSceneCachePreAnimatedStateSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneCachePreAnimatedStateSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCachePreAnimatedStateSystem")
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
pub struct UMovieSceneRestorePreAnimatedStateSystem {
    __padding_end: [u8; 96],
}
impl UMovieSceneRestorePreAnimatedStateSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRestorePreAnimatedStateSystem")
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
pub struct UMovieSceneRootInstantiatorSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneRootInstantiatorSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneRootInstantiatorSystem")
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
pub struct UMovieSceneSpawnablesSystem {
    __padding_end: [u8; 80],
}
impl UMovieSceneSpawnablesSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnablesSystem")
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
pub struct UMovieSceneTrackInstanceInstantiator {
    __padding_end: [u8; 256],
}
impl UMovieSceneTrackInstanceInstantiator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceInstantiator")
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
pub struct UMovieSceneTrackInstanceSystem {
    __padding_end: [u8; 88],
}
impl UMovieSceneTrackInstanceSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackInstanceSystem")
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
pub struct IMovieSceneCustomClockSource {}
#[repr(C, align(8))]
pub struct UMovieSceneCustomClockSource {
    __padding_end: [u8; 48],
}
impl UMovieSceneCustomClockSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCustomClockSource")
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
pub struct IMovieSceneEvaluationHook {}
#[repr(C, align(8))]
pub struct UMovieSceneEvaluationHook {
    __padding_end: [u8; 48],
}
impl UMovieSceneEvaluationHook {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEvaluationHook")
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
pub struct UMovieSceneBuiltInEasingFunction {
    __padding_end: [u8; 64],
}
impl UMovieSceneBuiltInEasingFunction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBuiltInEasingFunction")
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
pub struct UMovieSceneEasingExternalCurve {
    __padding_end: [u8; 64],
}
impl UMovieSceneEasingExternalCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEasingExternalCurve")
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
pub struct INodeAndChannelMappings {}
#[repr(C, align(8))]
pub struct UNodeAndChannelMappings {
    __padding_end: [u8; 48],
}
impl UNodeAndChannelMappings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNodeAndChannelMappings")
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
pub struct UMovieSceneShotMetaData {
    __padding_end: [u8; 72],
}
impl UMovieSceneShotMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneShotMetaData")
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
pub struct UMovieSceneNodeGroup {
    __padding_end: [u8; 120],
}
impl UMovieSceneNodeGroup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroup")
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
pub struct UMovieSceneNodeGroupCollection {
    __padding_end: [u8; 104],
}
impl UMovieSceneNodeGroupCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNodeGroupCollection")
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
pub struct UMovieScene {
    __padding_end: [u8; 1304],
}
impl UMovieScene {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScene")
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
pub struct UMovieSceneBindingOverrides {
    __padding_end: [u8; 152],
}
impl UMovieSceneBindingOverrides {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingOverrides")
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
pub struct UMovieSceneClock {
    __padding_end: [u8; 112],
}
impl UMovieSceneClock {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneClock")
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
pub struct UMovieSceneExternalClock {
    __padding_end: [u8; 152],
}
impl UMovieSceneExternalClock {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneExternalClock")
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
pub struct UBuiltInDynamicBindingResolverLibrary {
    __padding_end: [u8; 48],
}
impl UBuiltInDynamicBindingResolverLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBuiltInDynamicBindingResolverLibrary")
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
pub struct UMovieSceneFolder {
    __padding_end: [u8; 248],
}
impl UMovieSceneFolder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneFolder")
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
pub struct UMovieSceneMetaData {
    __padding_end: [u8; 96],
}
impl UMovieSceneMetaData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMetaData")
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
pub struct UMovieSceneSequencePlayer {
    __padding_end: [u8; 1216],
}
impl UMovieSceneSequencePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequencePlayer")
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
pub struct UMovieSceneSequenceTickManager {
    __padding_end: [u8; 160],
}
impl UMovieSceneSequenceTickManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceTickManager")
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
pub struct UMovieSceneBindingLifetimeSection {
    __padding_end: [u8; 368],
}
impl UMovieSceneBindingLifetimeSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeSection")
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
pub struct UMovieSceneHookSection {
    __padding_end: [u8; 384],
}
impl UMovieSceneHookSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneHookSection")
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
pub struct UMovieSceneSpawnSection {
    __padding_end: [u8; 672],
}
impl UMovieSceneSpawnSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnSection")
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
pub struct UMovieSceneTimeWarpSection {
    __padding_end: [u8; 376],
}
impl UMovieSceneTimeWarpSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpSection")
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
pub struct UTestMovieSceneTrack {
    __padding_end: [u8; 384],
}
impl UTestMovieSceneTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneTrack")
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
pub struct UTestMovieSceneSection {
    __padding_end: [u8; 360],
}
impl UTestMovieSceneSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSection")
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
pub struct UTestMovieSceneSequence {
    __padding_end: [u8; 136],
}
impl UTestMovieSceneSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSequence")
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
pub struct UTestMovieSceneSubTrack {
    __padding_end: [u8; 432],
}
impl UTestMovieSceneSubTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubTrack")
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
pub struct UTestMovieSceneSubSection {
    __padding_end: [u8; 2424],
}
impl UTestMovieSceneSubSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneSubSection")
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
pub struct UTestMovieSceneEvalHookTrack {
    __padding_end: [u8; 368],
}
impl UTestMovieSceneEvalHookTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookTrack")
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
pub struct UTestMovieSceneEvalHookSection {
    __padding_end: [u8; 408],
}
impl UTestMovieSceneEvalHookSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneEvalHookSection")
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
pub struct UTestMovieSceneObject {
    __padding_end: [u8; 48],
}
impl UTestMovieSceneObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestMovieSceneObject")
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
pub struct ATestMovieSceneArrayPropertiesActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub test_bool: bool,
    pub test_enum: ETestMovieSceneEnum,
    pub test_int32: i32,
    pub test_object: UPtr<UTestMovieSceneObject>,
    pub test_vector: crate::bindings::core_u_object::FVector,
    pub multiple_floats: TArray<f32>,
    pub single_struct: FTestMovieSceneStruct,
    pub multiple_structs: TArray<FTestMovieSceneStruct>,
    pub single_instanced_struct: crate::bindings::core_u_object::FInstancedStruct,
    pub multiple_instanced_structs: TArray<
        crate::bindings::core_u_object::FInstancedStruct,
    >,
    pub test_setter_float: f32,
    __padding_end: [u8; 4],
}
impl ATestMovieSceneArrayPropertiesActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ATestMovieSceneArrayPropertiesActor")
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
pub struct UMovieSceneBindingLifetimeTrack {
    __padding_end: [u8; 376],
}
impl UMovieSceneBindingLifetimeTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingLifetimeTrack")
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
pub struct UMovieSceneSpawnTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneSpawnTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSpawnTrack")
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
pub struct UMovieSceneTimeWarpTrack {
    __padding_end: [u8; 384],
}
impl UMovieSceneTimeWarpTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpTrack")
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
pub struct UMovieSceneTimeWarpCurve {
    __padding_end: [u8; 464],
}
impl UMovieSceneTimeWarpCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpCurve")
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
pub struct FMovieSceneSequencePlayer_OnPlay {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnPlayReverse {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnStop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnPause {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMovieSceneSequencePlayer_OnFinished {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMovieSceneCompletionMode(pub u8);
impl EMovieSceneCompletionMode {
    pub const KEEP_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(0);
    pub const RESTORE_STATE: EMovieSceneCompletionMode = EMovieSceneCompletionMode(1);
    pub const PROJECT_DEFAULT: EMovieSceneCompletionMode = EMovieSceneCompletionMode(2);
}
#[repr(transparent)]
pub struct EMovieSceneObjectBindingSpace(pub u8);
impl EMovieSceneObjectBindingSpace {
    pub const LOCAL: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(0);
    pub const ROOT: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(1);
    pub const UNUSED: EMovieSceneObjectBindingSpace = EMovieSceneObjectBindingSpace(2);
}
#[repr(transparent)]
pub struct EMovieSceneSubSectionFlags(pub u8);
impl EMovieSceneSubSectionFlags {
    pub const NONE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(0);
    pub const OVERRIDE_KEEP_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        1,
    );
    pub const OVERRIDE_RESTORE_STATE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        2,
    );
    pub const IGNORE_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        4,
    );
    pub const BLEND_HIERARCHICAL_BIAS: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        8,
    );
    pub const ANY_RESTORE_STATE_OVERRIDE: EMovieSceneSubSectionFlags = EMovieSceneSubSectionFlags(
        3,
    );
}
#[repr(transparent)]
pub struct ESectionEvaluationFlags(pub u8);
impl ESectionEvaluationFlags {
    pub const NONE: ESectionEvaluationFlags = ESectionEvaluationFlags(0);
    pub const PRE_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(1);
    pub const POST_ROLL: ESectionEvaluationFlags = ESectionEvaluationFlags(2);
    pub const FORCE_KEEP_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(4);
    pub const FORCE_RESTORE_STATE: ESectionEvaluationFlags = ESectionEvaluationFlags(8);
}
#[repr(transparent)]
pub struct EMovieSceneCompletionModeOverride(pub u8);
impl EMovieSceneCompletionModeOverride {
    pub const NONE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        0,
    );
    pub const FORCE_KEEP_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        1,
    );
    pub const FORCE_RESTORE_STATE: EMovieSceneCompletionModeOverride = EMovieSceneCompletionModeOverride(
        2,
    );
}
#[repr(transparent)]
pub struct ENavigationToolItemFlags(pub u8);
impl ENavigationToolItemFlags {
    pub const NONE: ENavigationToolItemFlags = ENavigationToolItemFlags(0);
    pub const IGNORE_PENDING_KILL: ENavigationToolItemFlags = ENavigationToolItemFlags(
        1,
    );
    pub const PENDING_REMOVAL: ENavigationToolItemFlags = ENavigationToolItemFlags(2);
    pub const EXPANDED: ENavigationToolItemFlags = ENavigationToolItemFlags(4);
}
#[repr(transparent)]
pub struct EMovieSceneBlendType(pub u8);
impl EMovieSceneBlendType {
    pub const INVALID: EMovieSceneBlendType = EMovieSceneBlendType(0);
    pub const ABSOLUTE: EMovieSceneBlendType = EMovieSceneBlendType(1);
    pub const ADDITIVE: EMovieSceneBlendType = EMovieSceneBlendType(2);
    pub const RELATIVE: EMovieSceneBlendType = EMovieSceneBlendType(4);
    pub const ADDITIVE_FROM_BASE: EMovieSceneBlendType = EMovieSceneBlendType(8);
    pub const OVERRIDE: EMovieSceneBlendType = EMovieSceneBlendType(16);
}
#[repr(transparent)]
pub struct EEvaluationMethod(pub u8);
impl EEvaluationMethod {
    pub const STATIC: EEvaluationMethod = EEvaluationMethod(0);
    pub const SWEPT: EEvaluationMethod = EEvaluationMethod(1);
}
#[repr(transparent)]
pub struct EMovieSceneBreadcrumbMode(pub u8);
impl EMovieSceneBreadcrumbMode {
    pub const SPARSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(0);
    pub const DENSE: EMovieSceneBreadcrumbMode = EMovieSceneBreadcrumbMode(1);
}
#[repr(transparent)]
pub struct EMovieSceneServerClientMask(pub u8);
impl EMovieSceneServerClientMask {
    pub const NONE: EMovieSceneServerClientMask = EMovieSceneServerClientMask(0);
    pub const SERVER: EMovieSceneServerClientMask = EMovieSceneServerClientMask(1);
    pub const CLIENT: EMovieSceneServerClientMask = EMovieSceneServerClientMask(2);
    pub const ALL: EMovieSceneServerClientMask = EMovieSceneServerClientMask(3);
}
#[repr(transparent)]
pub struct EMovieScenePlayerStatus(pub u8);
impl EMovieScenePlayerStatus {
    pub const STOPPED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(0);
    pub const PLAYING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(1);
    pub const SCRUBBING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(2);
    pub const JUMPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(3);
    pub const STEPPING: EMovieScenePlayerStatus = EMovieScenePlayerStatus(4);
    pub const PAUSED: EMovieScenePlayerStatus = EMovieScenePlayerStatus(5);
    pub const MAX: EMovieScenePlayerStatus = EMovieScenePlayerStatus(6);
}
#[repr(transparent)]
pub struct EMovieScenePositionType(pub u8);
impl EMovieScenePositionType {
    pub const FRAME: EMovieScenePositionType = EMovieScenePositionType(0);
    pub const TIME: EMovieScenePositionType = EMovieScenePositionType(1);
    pub const MARKED_FRAME: EMovieScenePositionType = EMovieScenePositionType(2);
    pub const TIMECODE: EMovieScenePositionType = EMovieScenePositionType(3);
}
#[repr(transparent)]
pub struct EUpdatePositionMethod(pub u8);
impl EUpdatePositionMethod {
    pub const PLAY: EUpdatePositionMethod = EUpdatePositionMethod(0);
    pub const JUMP: EUpdatePositionMethod = EUpdatePositionMethod(1);
    pub const SCRUB: EUpdatePositionMethod = EUpdatePositionMethod(2);
}
#[repr(transparent)]
pub struct ESpawnOwnership(pub u8);
impl ESpawnOwnership {
    pub const INNER_SEQUENCE: ESpawnOwnership = ESpawnOwnership(0);
    pub const ROOT_SEQUENCE: ESpawnOwnership = ESpawnOwnership(1);
    pub const EXTERNAL: ESpawnOwnership = ESpawnOwnership(2);
}
#[repr(transparent)]
pub struct ETestMovieSceneEnum(pub u8);
impl ETestMovieSceneEnum {
    pub const ONE: ETestMovieSceneEnum = ETestMovieSceneEnum(0);
    pub const TWO: ETestMovieSceneEnum = ETestMovieSceneEnum(1);
    pub const THREE: ETestMovieSceneEnum = ETestMovieSceneEnum(2);
}
#[repr(transparent)]
pub struct EUpdateClockSource(pub u8);
impl EUpdateClockSource {
    pub const TICK: EUpdateClockSource = EUpdateClockSource(0);
    pub const PLATFORM: EUpdateClockSource = EUpdateClockSource(1);
    pub const AUDIO: EUpdateClockSource = EUpdateClockSource(2);
    pub const RELATIVE_TIMECODE: EUpdateClockSource = EUpdateClockSource(3);
    pub const TIMECODE: EUpdateClockSource = EUpdateClockSource(4);
    pub const PLAY_EVERY_FRAME: EUpdateClockSource = EUpdateClockSource(5);
    pub const CUSTOM: EUpdateClockSource = EUpdateClockSource(6);
}
#[repr(transparent)]
pub struct EMovieSceneTimeUnit(pub u8);
impl EMovieSceneTimeUnit {
    pub const DISPLAY_RATE: EMovieSceneTimeUnit = EMovieSceneTimeUnit(0);
    pub const TICK_RESOLUTION: EMovieSceneTimeUnit = EMovieSceneTimeUnit(1);
}
#[repr(transparent)]
pub struct EMovieSceneConditionCheckFrequency(pub u8);
impl EMovieSceneConditionCheckFrequency {
    pub const ONCE: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        0,
    );
    pub const ON_TICK: EMovieSceneConditionCheckFrequency = EMovieSceneConditionCheckFrequency(
        1,
    );
}
#[repr(transparent)]
pub struct EMovieSceneConditionScope(pub u8);
impl EMovieSceneConditionScope {
    pub const GLOBAL: EMovieSceneConditionScope = EMovieSceneConditionScope(0);
    pub const BINDING: EMovieSceneConditionScope = EMovieSceneConditionScope(1);
    pub const OWNER_OBJECT: EMovieSceneConditionScope = EMovieSceneConditionScope(2);
}
#[repr(transparent)]
pub struct EMovieSceneKeyInterpolation(pub u8);
impl EMovieSceneKeyInterpolation {
    pub const AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(0);
    pub const USER: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(1);
    pub const BREAK: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(2);
    pub const LINEAR: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(3);
    pub const CONSTANT: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(4);
    pub const SMART_AUTO: EMovieSceneKeyInterpolation = EMovieSceneKeyInterpolation(5);
}
#[repr(transparent)]
pub struct EMovieSceneEvaluationType(pub u8);
impl EMovieSceneEvaluationType {
    pub const FRAME_LOCKED: EMovieSceneEvaluationType = EMovieSceneEvaluationType(0);
    pub const WITH_SUB_FRAMES: EMovieSceneEvaluationType = EMovieSceneEvaluationType(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceFlags(pub u8);
impl EMovieSceneSequenceFlags {
    pub const NONE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(0);
    pub const VOLATILE: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
    pub const BLOCKING_EVALUATION: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(
        2,
    );
    pub const DYNAMIC_WEIGHTING: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(4);
    pub const LOOP_CUTS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(8);
    pub const INHERITED_FLAGS: EMovieSceneSequenceFlags = EMovieSceneSequenceFlags(1);
}
#[repr(transparent)]
pub struct EMovieSceneSequenceCompilerMask(pub u8);
impl EMovieSceneSequenceCompilerMask {
    pub const HIERARCHY: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        1,
    );
    pub const EVALUATION_TEMPLATE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        2,
    );
    pub const EVALUATION_TEMPLATE_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        4,
    );
    pub const ENTITY_COMPONENT_FIELD: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(
        8,
    );
    pub const NONE: EMovieSceneSequenceCompilerMask = EMovieSceneSequenceCompilerMask(0);
}
#[repr(transparent)]
pub struct EMovieSceneGroupConditionOperator(pub u8);
impl EMovieSceneGroupConditionOperator {
    pub const AND: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        0,
    );
    pub const OR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        1,
    );
    pub const XOR: EMovieSceneGroupConditionOperator = EMovieSceneGroupConditionOperator(
        2,
    );
}
#[repr(transparent)]
pub struct EMovieSceneBuiltInEasing(pub u8);
impl EMovieSceneBuiltInEasing {
    pub const LINEAR: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(0);
    pub const SIN_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(1);
    pub const SIN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(2);
    pub const SIN_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(3);
    pub const QUAD_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(4);
    pub const QUAD_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(5);
    pub const QUAD_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(6);
    pub const CUBIC: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(7);
    pub const CUBIC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(8);
    pub const CUBIC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(9);
    pub const CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(10);
    pub const HERMITE_CUBIC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(
        11,
    );
    pub const QUART_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(12);
    pub const QUART_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(13);
    pub const QUART_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(14);
    pub const QUINT_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(15);
    pub const QUINT_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(16);
    pub const QUINT_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(17);
    pub const EXPO_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(18);
    pub const EXPO_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(19);
    pub const EXPO_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(20);
    pub const CIRC_IN: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(21);
    pub const CIRC_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(22);
    pub const CIRC_IN_OUT: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(23);
    pub const CUSTOM: EMovieSceneBuiltInEasing = EMovieSceneBuiltInEasing(24);
}
