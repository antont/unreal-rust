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
pub static mut U_LEVEL_SEQUENCE_REMOVE_META_DATA_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_FIND_OR_ADD_META_DATA_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_FIND_META_DATA_BY_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_COPY_META_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_BURN_IN_OPTIONS_SET_BURN_IN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_SHOW_BURNIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_SET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_SET_REPLICATE_PLAYBACK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_SET_BINDING_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_SET_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_RESET_BINDINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_RESET_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_REMOVE_BINDING_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_REMOVE_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_ON_LEVEL_SEQUENCE_LOADED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_HIDE_BURNIN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_GET_SEQUENCE_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_FIND_NAMED_BINDINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_FIND_NAMED_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_ADD_BINDING_BY_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_ACTOR_ADD_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_BURN_IN_SET_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_BURN_IN_GET_SETTINGS_CLASS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_ON_CREATED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_SEQUENCE_CUSTOM_CLOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_ROOT_SEQUENCE_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_ROOT_SEQUENCE_CUSTOM_CLOCK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_CURRENT_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_PLAYER_GET_ACTIVE_CAMERA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_PLAYER_CREATE_LEVEL_SEQUENCE_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_SUB_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_NO_GOOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_FLAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_FAVORITE_RATING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_SUB_SEQUENCE_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_SUB_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_RECORDED_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_NO_GOOD_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_NO_GOOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_FLAGGED_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_FLAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_FAVORITE_RATING_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_FAVORITE_RATING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE_ASSET_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED_ASSET_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD_ASSET_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED_ASSET_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING_BY_ASSET_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING_ASSET_TAG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_SUB_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_RECORDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_NO_GOOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_FLAGGED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_FAVORITE_RATING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_SYNCHRONIZE_TO_SERVER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_ON_REP_SERVER_START_TIME_SECONDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_GET_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_GET_MEDIA_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequence::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetaDataByClass"),
            &raw mut U_LEVEL_SEQUENCE_REMOVE_META_DATA_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrAddMetaDataByClass"),
            &raw mut U_LEVEL_SEQUENCE_FIND_OR_ADD_META_DATA_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMetaDataByClass"),
            &raw mut U_LEVEL_SEQUENCE_FIND_META_DATA_BY_CLASS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyMetaData"),
            &raw mut U_LEVEL_SEQUENCE_COPY_META_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceBurnInOptions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBurnIn"),
            &raw mut U_LEVEL_SEQUENCE_BURN_IN_OPTIONS_SET_BURN_IN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ALevelSequenceActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowBurnin"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_SHOW_BURNIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequence"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_SET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReplicatePlayback"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_SET_REPLICATE_PLAYBACK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBindingByTag"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_SET_BINDING_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBinding"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_SET_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetBindings"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_RESET_BINDINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetBinding"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_RESET_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBindingByTag"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_REMOVE_BINDING_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBinding"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_REMOVE_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnLevelSequenceLoaded__DelegateSignature"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_ON_LEVEL_SEQUENCE_LOADED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HideBurnin"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_HIDE_BURNIN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequencePlayer"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_GET_SEQUENCE_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNamedBindings"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_FIND_NAMED_BINDINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNamedBinding"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_FIND_NAMED_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBindingByTag"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_ADD_BINDING_BY_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBinding"),
            &raw mut A_LEVEL_SEQUENCE_ACTOR_ADD_BINDING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceBurnIn::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSettings"),
            &raw mut U_LEVEL_SEQUENCE_BURN_IN_SET_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettingsClass"),
            &raw mut U_LEVEL_SEQUENCE_BURN_IN_GET_SETTINGS_CLASS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceDirector::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCreated"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_ON_CREATED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequenceCustomClock"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_SEQUENCE_CUSTOM_CLOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSequenceTime"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_ROOT_SEQUENCE_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSequenceCustomClock"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_ROOT_SEQUENCE_CUSTOM_CLOCK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentTime"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_CURRENT_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjects"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObject"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActors"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActor"),
            &raw mut U_LEVEL_SEQUENCE_DIRECTOR_GET_BOUND_ACTOR,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequencePlayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActiveCameraComponent"),
            &raw mut U_LEVEL_SEQUENCE_PLAYER_GET_ACTIVE_CAMERA_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLevelSequencePlayer"),
            &raw mut U_LEVEL_SEQUENCE_PLAYER_CREATE_LEVEL_SEQUENCE_PLAYER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceShotMetaDataLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsSubSequence"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_SUB_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsRecorded"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsNoGood"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_NO_GOOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsFlagged"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_IS_FLAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFavoriteRating"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_SET_FAVORITE_RATING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsSubSequenceByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_SUB_SEQUENCE_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsSubSequence"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_SUB_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsRecordedByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_RECORDED_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsRecorded"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsNoGoodByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_NO_GOOD_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsNoGood"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_NO_GOOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsFlaggedByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_FLAGGED_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsFlagged"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_IS_FLAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFavoriteRatingByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_FAVORITE_RATING_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFavoriteRating"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_HAS_FAVORITE_RATING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequenceByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequenceAssetTag"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE_ASSET_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequence"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_SUB_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecordedByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecordedAssetTag"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED_ASSET_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecorded"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGoodByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGoodAssetTag"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD_ASSET_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGood"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_NO_GOOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlaggedByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlaggedAssetTag"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED_ASSET_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlagged"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_IS_FLAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRatingByAssetData"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING_BY_ASSET_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRatingAssetTag"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING_ASSET_TAG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRating"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_GET_FAVORITE_RATING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsSubSequence"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_SUB_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsRecorded"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_RECORDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsNoGood"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_NO_GOOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsFlagged"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_IS_FLAGGED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearFavoriteRating"),
            &raw mut U_LEVEL_SEQUENCE_SHOT_META_DATA_LIBRARY_CLEAR_FAVORITE_RATING,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ALevelSequenceMediaController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SynchronizeToServer"),
            &raw mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_SYNCHRONIZE_TO_SERVER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_ServerStartTimeSeconds"),
            &raw mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_ON_REP_SERVER_START_TIME_SECONDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_GET_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaComponent"),
            &raw mut A_LEVEL_SEQUENCE_MEDIA_CONTROLLER_GET_MEDIA_COMPONENT,
        );
    }
}
#[repr(C, align(1))]
pub struct FLevelSequenceCameraSettings {
    pub b_override_aspect_ratio_axis_constraint: bool,
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
}
impl FLevelSequenceCameraSettings {}
#[repr(C, align(8))]
pub struct FLevelSequenceAnimSequenceLinkItem {
    pub skel_track_guid: crate::bindings::core_u_object::FGuid,
    pub path_to_anim_sequence: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_export_transforms: bool,
    pub b_export_morph_targets: bool,
    pub b_export_attribute_curves: bool,
    pub b_export_material_curves: bool,
    pub interpolation: crate::bindings::engine::EAnimInterpolationType,
    pub curve_interpolation: crate::bindings::engine::ERichCurveInterpMode,
    pub b_record_in_world_space: bool,
    pub b_evaluate_all_skeletal_mesh_components: bool,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub warm_up_frames: crate::bindings::core_u_object::FFrameNumber,
    pub delay_before_start: crate::bindings::core_u_object::FFrameNumber,
    pub b_use_custom_time_range: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_display_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
impl FLevelSequenceAnimSequenceLinkItem {}
#[repr(C, align(8))]
pub struct FLevelSequencePlayerSnapshot {
    pub root_name: FString,
    pub root_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub source_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub current_shot_name: FString,
    pub current_shot_local_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub current_shot_source_time: crate::bindings::core_u_object::FQualifiedFrameTime,
    pub source_timecode: FString,
    pub camera_component: TSoftObjectPtr<crate::bindings::engine::UCameraComponent>,
    pub active_shot: UPtr<ULevelSequence>,
    __padding_end: [u8; 8],
}
impl FLevelSequencePlayerSnapshot {}
#[repr(C, align(16))]
pub struct UDefaultLevelSequenceInstanceData {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub transform_origin_actor: UPtr<crate::bindings::engine::AActor>,
    pub transform_origin: crate::bindings::core_u_object::FTransform,
}
impl UDefaultLevelSequenceInstanceData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultLevelSequenceInstanceData")
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
pub struct UAnimSequenceLevelSequenceLink {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub skel_track_guid: crate::bindings::core_u_object::FGuid,
    pub path_to_level_sequence: crate::bindings::core_u_object::FSoftObjectPath,
}
impl UAnimSequenceLevelSequenceLink {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceLevelSequenceLink")
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
pub struct ULevelSequence {
    __padding_end: [u8; 288],
}
impl ULevelSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequence")
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
pub struct ULevelSequenceBurnInInitSettings {
    __padding_end: [u8; 48],
}
impl ULevelSequenceBurnInInitSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceBurnInInitSettings")
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
pub struct ULevelSequenceBurnInOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub b_use_burn_in: bool,
    pub burn_in_class: crate::bindings::core_u_object::FSoftClassPath,
    pub settings: UPtr<ULevelSequenceBurnInInitSettings>,
}
impl ULevelSequenceBurnInOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceBurnInOptions")
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
pub struct ALevelSequenceActor {
    #[doc(hidden)]
    __padding_1160: [u8; 1160],
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<ULevelSequencePlayer>,
    pub level_sequence_asset: UPtr<ULevelSequence>,
    #[doc(hidden)]
    __padding_1256: [u8; 40],
    pub camera_settings: FLevelSequenceCameraSettings,
    pub burn_in_options: UPtr<ULevelSequenceBurnInOptions>,
    pub binding_overrides: UPtr<
        crate::bindings::movie_scene::UMovieSceneBindingOverrides,
    >,
    pub flags_1280: u8,
    pub default_instance_data: UPtr<crate::bindings::core_u_object::UObject>,
    __padding_end: [u8; 56],
}
impl ALevelSequenceActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALevelSequenceActor")
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
pub struct AReplicatedLevelSequenceActor {
    __padding_end: [u8; 1352],
}
impl AReplicatedLevelSequenceActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AReplicatedLevelSequenceActor")
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
pub struct ULevelSequenceAnimSequenceLink {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub anim_sequence_links: TArray<FLevelSequenceAnimSequenceLinkItem>,
}
impl ULevelSequenceAnimSequenceLink {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceAnimSequenceLink")
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
pub struct ULevelSequenceBurnIn {
    #[doc(hidden)]
    __padding_1288: [u8; 1288],
    pub frame_information: FLevelSequencePlayerSnapshot,
    pub level_sequence_actor: UPtr<ALevelSequenceActor>,
}
impl ULevelSequenceBurnIn {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceBurnIn")
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
pub struct ULevelSequenceDirector {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub player: UPtr<ULevelSequencePlayer>,
    __padding_end: [u8; 8],
}
impl ULevelSequenceDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceDirector")
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
pub struct ULegacyLevelSequenceDirectorBlueprint {
    __padding_end: [u8; 1432],
}
impl ULegacyLevelSequenceDirectorBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyLevelSequenceDirectorBlueprint")
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
pub struct ULevelSequencePlayer {
    __padding_end: [u8; 1520],
}
impl ULevelSequencePlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequencePlayer")
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
pub struct ULevelSequenceProjectSettings {
    __padding_end: [u8; 152],
}
impl ULevelSequenceProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceProjectSettings")
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
pub struct ULevelSequenceShotMetaDataLibrary {
    __padding_end: [u8; 48],
}
impl ULevelSequenceShotMetaDataLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceShotMetaDataLibrary")
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
pub struct ALevelSequenceMediaController {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub sequence: UPtr<ALevelSequenceActor>,
    pub media_component: UPtr<crate::bindings::media_assets::UMediaComponent>,
    pub server_start_time_seconds: f32,
    __padding_end: [u8; 12],
}
impl ALevelSequenceMediaController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALevelSequenceMediaController")
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
pub struct FLevelSequencePlayer_OnCameraCut {
    _opague: [u8; 24],
}
