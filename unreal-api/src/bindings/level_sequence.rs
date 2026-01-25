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
    pub u_level_sequence_remove_meta_data_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_find_or_add_meta_data_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_find_meta_data_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_copy_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_burn_in_options_set_burn_in: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_show_burnin: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_set_sequence: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_set_replicate_playback: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_set_binding_by_tag: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_set_binding: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_reset_bindings: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_reset_binding: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_remove_binding_by_tag: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_remove_binding: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_on_level_sequence_loaded_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_hide_burnin: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_get_sequence_player: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_find_named_bindings: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_find_named_binding: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_add_binding_by_tag: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_actor_add_binding: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_burn_in_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_burn_in_get_settings_class: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_on_created: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_sequence_custom_clock: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_root_sequence_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_root_sequence_custom_clock: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_current_time: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_bound_objects: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_bound_object: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_bound_actors: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_director_get_bound_actor: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_player_get_active_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_player_create_level_sequence_player: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_set_is_sub_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_set_is_recorded: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_set_is_no_good: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_set_is_flagged: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_set_favorite_rating: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_sub_sequence_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_sub_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_recorded_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_recorded: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_no_good_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_no_good: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_flagged_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_is_flagged: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_favorite_rating_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_has_favorite_rating: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_sub_sequence_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_sub_sequence_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_sub_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_recorded_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_recorded_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_recorded: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_no_good_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_no_good_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_no_good: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_flagged_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_flagged_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_is_flagged: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_favorite_rating_by_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_favorite_rating_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_get_favorite_rating: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_clear_is_sub_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_clear_is_recorded: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_clear_is_no_good: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_clear_is_flagged: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_shot_meta_data_library_clear_favorite_rating: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_media_controller_synchronize_to_server: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_media_controller_play: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_media_controller_on_rep_server_start_time_seconds: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_media_controller_get_sequence: *mut crate::ffi::UFunctionOpague,
    pub a_level_sequence_media_controller_get_media_component: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_level_sequence_remove_meta_data_by_class: std::ptr::null_mut(),
            u_level_sequence_find_or_add_meta_data_by_class: std::ptr::null_mut(),
            u_level_sequence_find_meta_data_by_class: std::ptr::null_mut(),
            u_level_sequence_copy_meta_data: std::ptr::null_mut(),
            u_level_sequence_burn_in_options_set_burn_in: std::ptr::null_mut(),
            a_level_sequence_actor_show_burnin: std::ptr::null_mut(),
            a_level_sequence_actor_set_sequence: std::ptr::null_mut(),
            a_level_sequence_actor_set_replicate_playback: std::ptr::null_mut(),
            a_level_sequence_actor_set_binding_by_tag: std::ptr::null_mut(),
            a_level_sequence_actor_set_binding: std::ptr::null_mut(),
            a_level_sequence_actor_reset_bindings: std::ptr::null_mut(),
            a_level_sequence_actor_reset_binding: std::ptr::null_mut(),
            a_level_sequence_actor_remove_binding_by_tag: std::ptr::null_mut(),
            a_level_sequence_actor_remove_binding: std::ptr::null_mut(),
            a_level_sequence_actor_on_level_sequence_loaded_delegate_signature: std::ptr::null_mut(),
            a_level_sequence_actor_hide_burnin: std::ptr::null_mut(),
            a_level_sequence_actor_get_sequence_player: std::ptr::null_mut(),
            a_level_sequence_actor_get_sequence: std::ptr::null_mut(),
            a_level_sequence_actor_find_named_bindings: std::ptr::null_mut(),
            a_level_sequence_actor_find_named_binding: std::ptr::null_mut(),
            a_level_sequence_actor_add_binding_by_tag: std::ptr::null_mut(),
            a_level_sequence_actor_add_binding: std::ptr::null_mut(),
            u_level_sequence_burn_in_set_settings: std::ptr::null_mut(),
            u_level_sequence_burn_in_get_settings_class: std::ptr::null_mut(),
            u_level_sequence_director_on_created: std::ptr::null_mut(),
            u_level_sequence_director_get_sequence_custom_clock: std::ptr::null_mut(),
            u_level_sequence_director_get_sequence: std::ptr::null_mut(),
            u_level_sequence_director_get_root_sequence_time: std::ptr::null_mut(),
            u_level_sequence_director_get_root_sequence_custom_clock: std::ptr::null_mut(),
            u_level_sequence_director_get_current_time: std::ptr::null_mut(),
            u_level_sequence_director_get_bound_objects: std::ptr::null_mut(),
            u_level_sequence_director_get_bound_object: std::ptr::null_mut(),
            u_level_sequence_director_get_bound_actors: std::ptr::null_mut(),
            u_level_sequence_director_get_bound_actor: std::ptr::null_mut(),
            u_level_sequence_player_get_active_camera_component: std::ptr::null_mut(),
            u_level_sequence_player_create_level_sequence_player: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_set_is_sub_sequence: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_set_is_recorded: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_set_is_no_good: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_set_is_flagged: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_set_favorite_rating: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_sub_sequence_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_sub_sequence: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_recorded_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_recorded: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_no_good_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_no_good: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_flagged_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_is_flagged: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_favorite_rating_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_has_favorite_rating: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_sub_sequence_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_sub_sequence_asset_tag: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_sub_sequence: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_recorded_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_recorded_asset_tag: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_recorded: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_no_good_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_no_good_asset_tag: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_no_good: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_flagged_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_flagged_asset_tag: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_is_flagged: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_favorite_rating_by_asset_data: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_favorite_rating_asset_tag: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_get_favorite_rating: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_clear_is_sub_sequence: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_clear_is_recorded: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_clear_is_no_good: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_clear_is_flagged: std::ptr::null_mut(),
            u_level_sequence_shot_meta_data_library_clear_favorite_rating: std::ptr::null_mut(),
            a_level_sequence_media_controller_synchronize_to_server: std::ptr::null_mut(),
            a_level_sequence_media_controller_play: std::ptr::null_mut(),
            a_level_sequence_media_controller_on_rep_server_start_time_seconds: std::ptr::null_mut(),
            a_level_sequence_media_controller_get_sequence: std::ptr::null_mut(),
            a_level_sequence_media_controller_get_media_component: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequence::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetaDataByClass"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_remove_meta_data_by_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindOrAddMetaDataByClass"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_find_or_add_meta_data_by_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMetaDataByClass"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_find_meta_data_by_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyMetaData"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_copy_meta_data,
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
            &raw mut __FUNCTION_PTRS.u_level_sequence_burn_in_options_set_burn_in,
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
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_show_burnin,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSequence"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_set_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReplicatePlayback"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_set_replicate_playback,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBindingByTag"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_set_binding_by_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBinding"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_set_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetBindings"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_reset_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetBinding"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_reset_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBindingByTag"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_remove_binding_by_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBinding"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_remove_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnLevelSequenceLoaded__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .a_level_sequence_actor_on_level_sequence_loaded_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HideBurnin"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_hide_burnin,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequencePlayer"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_get_sequence_player,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_get_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNamedBindings"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_find_named_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNamedBinding"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_find_named_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBindingByTag"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_add_binding_by_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBinding"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_actor_add_binding,
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
            &raw mut __FUNCTION_PTRS.u_level_sequence_burn_in_set_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSettingsClass"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_burn_in_get_settings_class,
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
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_on_created,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequenceCustomClock"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_sequence_custom_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSequenceTime"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_root_sequence_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootSequenceCustomClock"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_director_get_root_sequence_custom_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentTime"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_current_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjects"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_bound_objects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObject"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_bound_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActors"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_bound_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundActor"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_director_get_bound_actor,
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
            &raw mut __FUNCTION_PTRS.u_level_sequence_player_get_active_camera_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLevelSequencePlayer"),
            &raw mut __FUNCTION_PTRS.u_level_sequence_player_create_level_sequence_player,
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
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_set_is_sub_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsRecorded"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_set_is_recorded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsNoGood"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_set_is_no_good,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsFlagged"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_set_is_flagged,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFavoriteRating"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_set_favorite_rating,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsSubSequenceByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_sub_sequence_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsSubSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_sub_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsRecordedByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_recorded_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsRecorded"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_recorded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsNoGoodByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_no_good_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsNoGood"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_no_good,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsFlaggedByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_flagged_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasIsFlagged"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_is_flagged,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFavoriteRatingByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_favorite_rating_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasFavoriteRating"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_has_favorite_rating,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequenceByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequenceAssetTag"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_asset_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsSubSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_sub_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecordedByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_recorded_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecordedAssetTag"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_recorded_asset_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsRecorded"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_recorded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGoodByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_no_good_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGoodAssetTag"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_no_good_asset_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsNoGood"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_no_good,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlaggedByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_flagged_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlaggedAssetTag"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_flagged_asset_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsFlagged"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_is_flagged,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRatingByAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_favorite_rating_by_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRatingAssetTag"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_favorite_rating_asset_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFavoriteRating"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_get_favorite_rating,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsSubSequence"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_clear_is_sub_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsRecorded"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_clear_is_recorded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsNoGood"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_clear_is_no_good,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearIsFlagged"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_clear_is_flagged,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearFavoriteRating"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_shot_meta_data_library_clear_favorite_rating,
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
            &raw mut __FUNCTION_PTRS
                .a_level_sequence_media_controller_synchronize_to_server,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_media_controller_play,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnRep_ServerStartTimeSeconds"),
            &raw mut __FUNCTION_PTRS
                .a_level_sequence_media_controller_on_rep_server_start_time_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequence"),
            &raw mut __FUNCTION_PTRS.a_level_sequence_media_controller_get_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaComponent"),
            &raw mut __FUNCTION_PTRS
                .a_level_sequence_media_controller_get_media_component,
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
    pub(crate) __padding_end: [u8; 8],
}
impl FLevelSequencePlayerSnapshot {}
#[repr(C, align(16))]
pub struct UDefaultLevelSequenceInstanceData {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
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
    pub(crate) __padding_48: [u8; 48],
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
    pub fn remove_meta_data_by_class(
        &mut self,
        in_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_remove_meta_data_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_remove_meta_data_by_class,
                __buffer,
            )
        };
    }
    pub fn find_or_add_meta_data_by_class(
        &mut self,
        in_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_find_or_add_meta_data_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_find_or_add_meta_data_by_class,
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
    pub fn find_meta_data_by_class(
        &self,
        in_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_find_meta_data_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_find_meta_data_by_class,
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
    pub fn copy_meta_data(
        &mut self,
        in_meta_data: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_copy_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_data,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_copy_meta_data,
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
    pub(crate) __padding_48: [u8; 48],
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
    pub fn set_burn_in(
        &mut self,
        in_burn_in_class: crate::bindings::core_u_object::FSoftClassPath,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_options_set_burn_in,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_burn_in_class,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FSoftClassPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_options_set_burn_in,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ALevelSequenceActor {
    #[doc(hidden)]
    pub(crate) __padding_1160: [u8; 1160],
    pub playback_settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
    pub sequence_player: UPtr<ULevelSequencePlayer>,
    pub level_sequence_asset: UPtr<ULevelSequence>,
    #[doc(hidden)]
    pub(crate) __padding_1256: [u8; 40],
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
    pub fn show_burnin(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_show_burnin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_show_burnin,
                __buffer,
            )
        };
    }
    pub fn set_sequence(&mut self, in_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_sequence,
                __buffer,
            )
        };
    }
    pub fn set_replicate_playback(&mut self, replicate_playback: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_replicate_playback,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replicate_playback,
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
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_replicate_playback,
                __buffer,
            )
        };
    }
    pub fn set_binding_by_tag(
        &mut self,
        binding_tag: FName,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        b_allow_bindings_from_asset: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_binding_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_tag,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(16).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_bindings_from_asset,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_binding_by_tag,
                __buffer,
            )
        };
    }
    pub fn set_binding(
        &mut self,
        binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        b_allow_bindings_from_asset: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(32).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_bindings_from_asset,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_set_binding,
                __buffer,
            )
        };
    }
    pub fn reset_bindings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_reset_bindings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_reset_bindings,
                __buffer,
            )
        };
    }
    pub fn reset_binding(
        &mut self,
        binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_reset_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_reset_binding,
                __buffer,
            )
        };
    }
    pub fn remove_binding_by_tag(
        &mut self,
        tag: FName,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_remove_binding_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_remove_binding_by_tag,
                __buffer,
            )
        };
    }
    pub fn remove_binding(
        &mut self,
        binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_remove_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_remove_binding,
                __buffer,
            )
        };
    }
    pub fn hide_burnin(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_hide_burnin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_hide_burnin,
                __buffer,
            )
        };
    }
    pub fn get_sequence_player(&self) -> UPtr<ULevelSequencePlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_get_sequence_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_get_sequence_player,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ULevelSequencePlayer>>().read() }
    }
    pub fn get_sequence(&self) -> UPtr<ULevelSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_get_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ULevelSequence>>().read() }
    }
    pub fn find_named_bindings(
        &self,
        tag: FName,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneObjectBindingID> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_find_named_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_find_named_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<crate::bindings::movie_scene::FMovieSceneObjectBindingID>,
                >()
                .read()
        }
    }
    pub fn find_named_binding(
        &self,
        tag: FName,
    ) -> crate::bindings::movie_scene::FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_find_named_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_find_named_binding,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>()
                .read()
        }
    }
    pub fn add_binding_by_tag(
        &mut self,
        binding_tag: FName,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_allow_bindings_from_asset: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_add_binding_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_tag,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_bindings_from_asset,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_add_binding_by_tag,
                __buffer,
            )
        };
    }
    pub fn add_binding(
        &mut self,
        binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_allow_bindings_from_asset: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_add_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_bindings_from_asset,
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
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_actor_add_binding,
                __buffer,
            )
        };
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
    pub(crate) __padding_48: [u8; 48],
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
    pub(crate) __padding_1288: [u8; 1288],
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
    pub fn set_settings(
        &mut self,
        in_settings: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_settings_class(&self) -> TSubclassOf<ULevelSequenceBurnInInitSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_get_settings_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_burn_in_get_settings_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<ULevelSequenceBurnInInitSettings>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceDirector {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
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
    pub fn on_created(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_on_created,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_on_created,
                __buffer,
            )
        };
    }
    pub fn get_sequence_custom_clock(
        &self,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneClock> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_sequence_custom_clock,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_sequence_custom_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneClock>>()
                .read()
        }
    }
    pub fn get_sequence(
        &mut self,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneSequence> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>()
                .read()
        }
    }
    pub fn get_root_sequence_time(
        &self,
    ) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_root_sequence_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_root_sequence_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_root_sequence_custom_clock(
        &self,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneClock> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_root_sequence_custom_clock,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_root_sequence_custom_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneClock>>()
                .read()
        }
    }
    pub fn get_current_time(
        &self,
    ) -> crate::bindings::core_u_object::FQualifiedFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_current_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_current_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .read()
        }
    }
    pub fn get_bound_objects(
        &mut self,
        object_binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_bound_object(
        &mut self,
        object_binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_bound_actors(
        &mut self,
        object_binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_bound_actor(
        &mut self,
        object_binding: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_director_get_bound_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>().read()
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
    pub fn get_active_camera_component(
        &self,
    ) -> UPtr<crate::bindings::engine::UCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_player_get_active_camera_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_player_get_active_camera_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UCameraComponent>>()
                .read()
        }
    }
    pub fn create_level_sequence_player(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        level_sequence: UPtr<ULevelSequence>,
        settings: crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
        out_actor: &mut UPtr<ALevelSequenceActor>,
    ) -> UPtr<ULevelSequencePlayer> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_player_create_level_sequence_player,
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
                &level_sequence,
                __buffer.add(8).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &settings,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::movie_scene::FMovieSceneSequencePlaybackSettings,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actor,
                __buffer.add(56).cast::<UPtr<ALevelSequenceActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequencePlayer::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_player_create_level_sequence_player,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<UPtr<ALevelSequenceActor>>().swap(out_actor);
        }
        unsafe { __buffer.add(64).cast::<UPtr<ULevelSequencePlayer>>().read() }
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
    pub fn set_is_sub_sequence(
        in_level_sequence: UPtr<ULevelSequence>,
        b_in_is_sub_sequence: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_sub_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_sub_sequence,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_sub_sequence,
                __buffer,
            )
        };
    }
    pub fn set_is_recorded(
        in_level_sequence: UPtr<ULevelSequence>,
        b_in_is_recorded: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_recorded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_recorded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_recorded,
                __buffer,
            )
        };
    }
    pub fn set_is_no_good(
        in_level_sequence: UPtr<ULevelSequence>,
        b_in_is_no_good: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_no_good,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_no_good,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_no_good,
                __buffer,
            )
        };
    }
    pub fn set_is_flagged(
        in_level_sequence: UPtr<ULevelSequence>,
        b_in_is_flagged: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_flagged,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_flagged,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_is_flagged,
                __buffer,
            )
        };
    }
    pub fn set_favorite_rating(
        in_level_sequence: UPtr<ULevelSequence>,
        in_favorite_rating: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_favorite_rating,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_favorite_rating,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_set_favorite_rating,
                __buffer,
            )
        };
    }
    pub fn has_is_sub_sequence_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_sub_sequence_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_sub_sequence_by_asset_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn has_is_sub_sequence(in_level_sequence: UPtr<ULevelSequence>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_sub_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_sub_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_is_recorded_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_recorded_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_recorded_by_asset_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn has_is_recorded(in_level_sequence: UPtr<ULevelSequence>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_recorded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_recorded,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_is_no_good_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_no_good_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_no_good_by_asset_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn has_is_no_good(in_level_sequence: UPtr<ULevelSequence>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_no_good,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_no_good,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_is_flagged_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_flagged_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_flagged_by_asset_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn has_is_flagged(in_level_sequence: UPtr<ULevelSequence>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_flagged,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_is_flagged,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_favorite_rating_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<153>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_favorite_rating_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_favorite_rating_by_asset_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(152).cast::<bool>().read() }
    }
    pub fn has_favorite_rating(in_level_sequence: UPtr<ULevelSequence>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_favorite_rating,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_has_favorite_rating,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_is_sub_sequence_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        b_out_is_sub_sequence: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<154>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_sub_sequence,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<bool>().swap(b_out_is_sub_sequence);
        }
        unsafe { __buffer.add(153).cast::<bool>().read() }
    }
    pub fn get_is_sub_sequence_asset_tag() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_asset_tag,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence_asset_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_is_sub_sequence(
        in_level_sequence: UPtr<ULevelSequence>,
        b_out_is_sub_sequence: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_sub_sequence,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_sub_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_is_sub_sequence);
        }
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn get_is_recorded_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        b_out_is_recorded: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<154>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_recorded,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<bool>().swap(b_out_is_recorded);
        }
        unsafe { __buffer.add(153).cast::<bool>().read() }
    }
    pub fn get_is_recorded_asset_tag() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded_asset_tag,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded_asset_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_is_recorded(
        in_level_sequence: UPtr<ULevelSequence>,
        b_out_is_recorded: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_recorded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_recorded,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_is_recorded);
        }
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn get_is_no_good_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        b_out_no_good: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<154>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_no_good,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<bool>().swap(b_out_no_good);
        }
        unsafe { __buffer.add(153).cast::<bool>().read() }
    }
    pub fn get_is_no_good_asset_tag() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good_asset_tag,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good_asset_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_is_no_good(
        in_level_sequence: UPtr<ULevelSequence>,
        b_out_no_good: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_no_good,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_no_good,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_no_good);
        }
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn get_is_flagged_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        b_out_is_flagged: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<154>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_flagged,
                __buffer.add(152).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<bool>().swap(b_out_is_flagged);
        }
        unsafe { __buffer.add(153).cast::<bool>().read() }
    }
    pub fn get_is_flagged_asset_tag() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged_asset_tag,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged_asset_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_is_flagged(
        in_level_sequence: UPtr<ULevelSequence>,
        b_out_is_flagged: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_flagged,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_is_flagged,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_is_flagged);
        }
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn get_favorite_rating_by_asset_data(
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        out_favorite_rating: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<157>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_favorite_rating,
                __buffer.add(152).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating_by_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<i32>().swap(out_favorite_rating);
        }
        unsafe { __buffer.add(156).cast::<bool>().read() }
    }
    pub fn get_favorite_rating_asset_tag() -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating_asset_tag,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating_asset_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_favorite_rating(
        in_level_sequence: UPtr<ULevelSequence>,
        out_favorite_rating: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_favorite_rating,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_get_favorite_rating,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(out_favorite_rating);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn clear_is_sub_sequence(in_level_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_sub_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_sub_sequence,
                __buffer,
            )
        };
    }
    pub fn clear_is_recorded(in_level_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_recorded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_recorded,
                __buffer,
            )
        };
    }
    pub fn clear_is_no_good(in_level_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_no_good,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_no_good,
                __buffer,
            )
        };
    }
    pub fn clear_is_flagged(in_level_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_flagged,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_is_flagged,
                __buffer,
            )
        };
    }
    pub fn clear_favorite_rating(in_level_sequence: UPtr<ULevelSequence>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_favorite_rating,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer.add(0).cast::<UPtr<ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::level_sequence::ULevelSequenceShotMetaDataLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .u_level_sequence_shot_meta_data_library_clear_favorite_rating,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ALevelSequenceMediaController {
    #[doc(hidden)]
    pub(crate) __padding_1144: [u8; 1144],
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
    pub fn synchronize_to_server(&mut self, desync_threshold_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_synchronize_to_server,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &desync_threshold_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_synchronize_to_server,
                __buffer,
            )
        };
    }
    pub fn play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_play,
                __buffer,
            )
        };
    }
    pub fn get_sequence(&self) -> UPtr<ALevelSequenceActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_get_sequence,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_get_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ALevelSequenceActor>>().read() }
    }
    pub fn get_media_component(
        &self,
    ) -> UPtr<crate::bindings::media_assets::UMediaComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_get_media_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::level_sequence::__FUNCTION_PTRS
                    .a_level_sequence_media_controller_get_media_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::media_assets::UMediaComponent>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct FLevelSequencePlayer_OnCameraCut {
    _opague: [u8; 24],
}
