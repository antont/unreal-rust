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
    pub u_paper_flipbook_is_valid_key_frame_index: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_total_duration: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_sprite_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_sprite_at_frame: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_num_key_frames: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_num_frames: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_get_key_frame_index_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_stop: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_sprite_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_playback_position_in_frames: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_playback_position: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_new_time: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_looping: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_set_flipbook: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_reverse_from_end: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_reverse: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_play_from_start: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_play: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_on_rep_source_flipbook: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_is_reversing: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_is_playing: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_is_looping: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_sprite_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_playback_position_in_frames: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_playback_position: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_flipbook_length_in_frames: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_flipbook_length: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_flipbook_framerate: *mut crate::ffi::UFunctionOpague,
    pub u_paper_flipbook_component_get_flipbook: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_update_instance_transform: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_update_instance_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_sort_instances_along_axis: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_remove_instance: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_get_instance_transform: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_get_instance_count: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_clear_instances: *mut crate::ffi::UFunctionOpague,
    pub u_paper_grouped_sprite_component_add_instance: *mut crate::ffi::UFunctionOpague,
    pub u_paper_sprite_blueprint_library_make_brush_from_sprite: *mut crate::ffi::UFunctionOpague,
    pub u_paper_sprite_component_set_sprite_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_sprite_component_set_sprite: *mut crate::ffi::UFunctionOpague,
    pub u_paper_sprite_component_get_sprite: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_tile_map_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_tile_map: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_tile: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_layer_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_layer_collision: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_set_default_collision_thickness: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_resize_map: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_rebuild_collision: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_owns_tile_map: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_make_tile_map_editable: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_tile_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_tile_map_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_tile_corner_position: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_tile_center_position: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_tile: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_map_size: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_get_layer_color: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_create_new_tile_map: *mut crate::ffi::UFunctionOpague,
    pub u_paper_tile_map_component_add_new_layer: *mut crate::ffi::UFunctionOpague,
    pub u_paper_terrain_component_set_terrain_color: *mut crate::ffi::UFunctionOpague,
    pub u_tile_map_blueprint_library_make_tile: *mut crate::ffi::UFunctionOpague,
    pub u_tile_map_blueprint_library_get_tile_user_data: *mut crate::ffi::UFunctionOpague,
    pub u_tile_map_blueprint_library_get_tile_transform: *mut crate::ffi::UFunctionOpague,
    pub u_tile_map_blueprint_library_break_tile: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_paper_flipbook_is_valid_key_frame_index: std::ptr::null_mut(),
            u_paper_flipbook_get_total_duration: std::ptr::null_mut(),
            u_paper_flipbook_get_sprite_at_time: std::ptr::null_mut(),
            u_paper_flipbook_get_sprite_at_frame: std::ptr::null_mut(),
            u_paper_flipbook_get_num_key_frames: std::ptr::null_mut(),
            u_paper_flipbook_get_num_frames: std::ptr::null_mut(),
            u_paper_flipbook_get_key_frame_index_at_time: std::ptr::null_mut(),
            u_paper_flipbook_component_stop: std::ptr::null_mut(),
            u_paper_flipbook_component_set_sprite_color: std::ptr::null_mut(),
            u_paper_flipbook_component_set_play_rate: std::ptr::null_mut(),
            u_paper_flipbook_component_set_playback_position_in_frames: std::ptr::null_mut(),
            u_paper_flipbook_component_set_playback_position: std::ptr::null_mut(),
            u_paper_flipbook_component_set_new_time: std::ptr::null_mut(),
            u_paper_flipbook_component_set_looping: std::ptr::null_mut(),
            u_paper_flipbook_component_set_flipbook: std::ptr::null_mut(),
            u_paper_flipbook_component_reverse_from_end: std::ptr::null_mut(),
            u_paper_flipbook_component_reverse: std::ptr::null_mut(),
            u_paper_flipbook_component_play_from_start: std::ptr::null_mut(),
            u_paper_flipbook_component_play: std::ptr::null_mut(),
            u_paper_flipbook_component_on_rep_source_flipbook: std::ptr::null_mut(),
            u_paper_flipbook_component_is_reversing: std::ptr::null_mut(),
            u_paper_flipbook_component_is_playing: std::ptr::null_mut(),
            u_paper_flipbook_component_is_looping: std::ptr::null_mut(),
            u_paper_flipbook_component_get_sprite_color: std::ptr::null_mut(),
            u_paper_flipbook_component_get_play_rate: std::ptr::null_mut(),
            u_paper_flipbook_component_get_playback_position_in_frames: std::ptr::null_mut(),
            u_paper_flipbook_component_get_playback_position: std::ptr::null_mut(),
            u_paper_flipbook_component_get_flipbook_length_in_frames: std::ptr::null_mut(),
            u_paper_flipbook_component_get_flipbook_length: std::ptr::null_mut(),
            u_paper_flipbook_component_get_flipbook_framerate: std::ptr::null_mut(),
            u_paper_flipbook_component_get_flipbook: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_update_instance_transform: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_update_instance_color: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_sort_instances_along_axis: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_remove_instance: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_get_instance_transform: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_get_instance_count: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_clear_instances: std::ptr::null_mut(),
            u_paper_grouped_sprite_component_add_instance: std::ptr::null_mut(),
            u_paper_sprite_blueprint_library_make_brush_from_sprite: std::ptr::null_mut(),
            u_paper_sprite_component_set_sprite_color: std::ptr::null_mut(),
            u_paper_sprite_component_set_sprite: std::ptr::null_mut(),
            u_paper_sprite_component_get_sprite: std::ptr::null_mut(),
            u_paper_tile_map_component_set_tile_map_color: std::ptr::null_mut(),
            u_paper_tile_map_component_set_tile_map: std::ptr::null_mut(),
            u_paper_tile_map_component_set_tile: std::ptr::null_mut(),
            u_paper_tile_map_component_set_layer_color: std::ptr::null_mut(),
            u_paper_tile_map_component_set_layer_collision: std::ptr::null_mut(),
            u_paper_tile_map_component_set_default_collision_thickness: std::ptr::null_mut(),
            u_paper_tile_map_component_resize_map: std::ptr::null_mut(),
            u_paper_tile_map_component_rebuild_collision: std::ptr::null_mut(),
            u_paper_tile_map_component_owns_tile_map: std::ptr::null_mut(),
            u_paper_tile_map_component_make_tile_map_editable: std::ptr::null_mut(),
            u_paper_tile_map_component_get_tile_polygon: std::ptr::null_mut(),
            u_paper_tile_map_component_get_tile_map_color: std::ptr::null_mut(),
            u_paper_tile_map_component_get_tile_corner_position: std::ptr::null_mut(),
            u_paper_tile_map_component_get_tile_center_position: std::ptr::null_mut(),
            u_paper_tile_map_component_get_tile: std::ptr::null_mut(),
            u_paper_tile_map_component_get_map_size: std::ptr::null_mut(),
            u_paper_tile_map_component_get_layer_color: std::ptr::null_mut(),
            u_paper_tile_map_component_create_new_tile_map: std::ptr::null_mut(),
            u_paper_tile_map_component_add_new_layer: std::ptr::null_mut(),
            u_paper_terrain_component_set_terrain_color: std::ptr::null_mut(),
            u_tile_map_blueprint_library_make_tile: std::ptr::null_mut(),
            u_tile_map_blueprint_library_get_tile_user_data: std::ptr::null_mut(),
            u_tile_map_blueprint_library_get_tile_transform: std::ptr::null_mut(),
            u_tile_map_blueprint_library_break_tile: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperFlipbook::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidKeyFrameIndex"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_is_valid_key_frame_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTotalDuration"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_total_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpriteAtTime"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_sprite_at_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpriteAtFrame"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_sprite_at_frame,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumKeyFrames"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_num_key_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumFrames"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_num_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetKeyFrameIndexAtTime"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_get_key_frame_index_at_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperFlipbookComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Stop"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_stop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSpriteColor"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_sprite_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackPositionInFrames"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_flipbook_component_set_playback_position_in_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackPosition"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_playback_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNewTime"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_new_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLooping"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFlipbook"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_set_flipbook,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReverseFromEnd"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_reverse_from_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reverse"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_reverse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlayFromStart"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_play_from_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Play"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_SourceFlipbook"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_flipbook_component_on_rep_source_flipbook,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReversing"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_is_reversing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPlaying"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_is_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLooping"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_is_looping,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpriteColor"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_get_sprite_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayRate"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_get_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaybackPositionInFrames"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_flipbook_component_get_playback_position_in_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaybackPosition"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_get_playback_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFlipbookLengthInFrames"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_length_in_frames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFlipbookLength"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_get_flipbook_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFlipbookFramerate"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_framerate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFlipbook"),
                &raw mut __FUNCTION_PTRS.u_paper_flipbook_component_get_flipbook,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperGroupedSpriteComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateInstanceTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateInstanceColor"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SortInstancesAlongAxis"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_sort_instances_along_axis,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveInstance"),
                &raw mut __FUNCTION_PTRS.u_paper_grouped_sprite_component_remove_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstanceTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstanceCount"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearInstances"),
                &raw mut __FUNCTION_PTRS.u_paper_grouped_sprite_component_clear_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInstance"),
                &raw mut __FUNCTION_PTRS.u_paper_grouped_sprite_component_add_instance,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperSpriteBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeBrushFromSprite"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_sprite_blueprint_library_make_brush_from_sprite,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperSpriteComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSpriteColor"),
                &raw mut __FUNCTION_PTRS.u_paper_sprite_component_set_sprite_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSprite"),
                &raw mut __FUNCTION_PTRS.u_paper_sprite_component_set_sprite,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSprite"),
                &raw mut __FUNCTION_PTRS.u_paper_sprite_component_get_sprite,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperTileMapComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTileMapColor"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_set_tile_map_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTileMap"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_set_tile_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTile"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_set_tile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLayerColor"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_set_layer_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLayerCollision"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_set_layer_collision,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefaultCollisionThickness"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_tile_map_component_set_default_collision_thickness,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResizeMap"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_resize_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RebuildCollision"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_rebuild_collision,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OwnsTileMap"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_owns_tile_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeTileMapEditable"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_tile_map_component_make_tile_map_editable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTilePolygon"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_get_tile_polygon,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTileMapColor"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_get_tile_map_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTileCornerPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_corner_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTileCenterPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_center_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTile"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_get_tile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMapSize"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_get_map_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLayerColor"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_get_layer_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewTileMap"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_create_new_tile_map,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddNewLayer"),
                &raw mut __FUNCTION_PTRS.u_paper_tile_map_component_add_new_layer,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPaperTerrainComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTerrainColor"),
                &raw mut __FUNCTION_PTRS.u_paper_terrain_component_set_terrain_color,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTileMapBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeTile"),
                &raw mut __FUNCTION_PTRS.u_tile_map_blueprint_library_make_tile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTileUserData"),
                &raw mut __FUNCTION_PTRS.u_tile_map_blueprint_library_get_tile_user_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTileTransform"),
                &raw mut __FUNCTION_PTRS.u_tile_map_blueprint_library_get_tile_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakTile"),
                &raw mut __FUNCTION_PTRS.u_tile_map_blueprint_library_break_tile,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FIntMargin {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl FIntMargin {}
#[repr(C, align(16))]
pub struct FPaperSpriteSocket {
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub socket_name: FName,
}
impl FPaperSpriteSocket {}
#[repr(C, align(8))]
pub struct FPaperTileInfo {
    pub(crate) __padding_end: [u8; 16],
}
impl FPaperTileInfo {}
#[repr(C, align(8))]
pub struct FPaperTileMetadata {
    pub user_data_name: FName,
    pub(crate) __padding_end: [u8; 60],
}
impl FPaperTileMetadata {}
#[repr(C, align(8))]
pub struct FPaperTerrainMaterialRule {
    pub(crate) __padding_end: [u8; 72],
}
impl FPaperTerrainMaterialRule {}
#[repr(C, align(16))]
pub struct APaperCharacter {
    #[doc(hidden)]
    pub(crate) __padding_2112: [u8; 2112],
    pub sprite: UPtr<UPaperFlipbookComponent>,
}
impl APaperCharacter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperCharacter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperCharacter")
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
pub struct UPaperFlipbook {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub frames_per_second: f32,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 16],
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub collision_source: EFlipbookCollisionMode,
}
impl UPaperFlipbook {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperFlipbook")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperFlipbook")
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
    pub fn is_valid_key_frame_index(&self, index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_is_valid_key_frame_index,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_is_valid_key_frame_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_total_duration(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_total_duration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_total_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_sprite_at_time(
        &self,
        time: f32,
        b_clamp_to_ends: bool,
    ) -> UPtr<UPaperSprite> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_sprite_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clamp_to_ends,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_sprite_at_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UPaperSprite>>().read() }
    }
    pub fn get_sprite_at_frame(&self, frame_index: i32) -> UPtr<UPaperSprite> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_sprite_at_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_sprite_at_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UPaperSprite>>().read() }
    }
    pub fn get_num_key_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_num_key_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_num_key_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_num_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_num_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_key_frame_index_at_time(&self, time: f32, b_clamp_to_ends: bool) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_key_frame_index_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clamp_to_ends,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_get_key_frame_index_at_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct APaperFlipbookActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperFlipbookComponent>,
}
impl APaperFlipbookActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperFlipbookActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperFlipbookActor")
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
pub struct UPaperFlipbookComponent {
    #[doc(hidden)]
    pub(crate) __padding_1608: [u8; 1608],
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 40],
}
impl UPaperFlipbookComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperFlipbookComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperFlipbookComponent")
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
    pub fn stop(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_stop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_stop,
                __buffer,
            )
        };
    }
    pub fn set_sprite_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_sprite_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_sprite_color,
                __buffer,
            )
        };
    }
    pub fn set_play_rate(&mut self, new_rate: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_rate, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_play_rate,
                __buffer,
            )
        };
    }
    pub fn set_playback_position_in_frames(
        &mut self,
        new_frame_position: i32,
        b_fire_events: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_playback_position_in_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_frame_position,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fire_events,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_playback_position_in_frames,
                __buffer,
            )
        };
    }
    pub fn set_playback_position(&mut self, new_position: f32, b_fire_events: bool) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_playback_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_position,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fire_events,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_playback_position,
                __buffer,
            )
        };
    }
    pub fn set_new_time(&mut self, new_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_new_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&new_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_new_time,
                __buffer,
            )
        };
    }
    pub fn set_looping(&mut self, b_new_looping: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_looping,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_looping,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_looping,
                __buffer,
            )
        };
    }
    pub fn set_flipbook(&mut self, new_flipbook: UPtr<UPaperFlipbook>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_flipbook,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_flipbook,
                __buffer.add(0).cast::<UPtr<UPaperFlipbook>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_set_flipbook,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn reverse_from_end(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_reverse_from_end,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_reverse_from_end,
                __buffer,
            )
        };
    }
    pub fn reverse(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_reverse,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_reverse,
                __buffer,
            )
        };
    }
    pub fn play_from_start(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_play_from_start,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_play_from_start,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_play,
                __buffer,
            )
        };
    }
    pub fn is_reversing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_reversing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_reversing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_looping(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_looping,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_is_looping,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_sprite_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_sprite_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_sprite_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_play_rate(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_play_rate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_playback_position_in_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_playback_position_in_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_playback_position_in_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_playback_position(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_playback_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_playback_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_flipbook_length_in_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_length_in_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_length_in_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_flipbook_length(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_length,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_flipbook_framerate(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_framerate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook_framerate,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_flipbook(&mut self) -> UPtr<UPaperFlipbook> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_flipbook_component_get_flipbook,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UPaperFlipbook>>().read() }
    }
}
#[repr(C, align(8))]
pub struct APaperGroupedSpriteActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperGroupedSpriteComponent>,
}
impl APaperGroupedSpriteActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperGroupedSpriteActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperGroupedSpriteActor")
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
pub struct UPaperGroupedSpriteComponent {
    __padding_end: [u8; 1632],
}
impl UPaperGroupedSpriteComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperGroupedSpriteComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperGroupedSpriteComponent")
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
    pub fn update_instance_transform(
        &mut self,
        instance_index: i32,
        new_instance_transform: &crate::bindings::core_u_object::FTransform,
        b_world_space: bool,
        b_mark_render_state_dirty: bool,
        b_teleport: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_instance_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_render_state_dirty,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_teleport,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(115).cast::<bool>().read() }
    }
    pub fn update_instance_color(
        &mut self,
        instance_index: i32,
        new_instance_color: crate::bindings::core_u_object::FLinearColor,
        b_mark_render_state_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_instance_color,
                __buffer.add(4).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_render_state_dirty,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_update_instance_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn sort_instances_along_axis(
        &mut self,
        world_space_sort_axis: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_sort_instances_along_axis,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_sort_axis,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_sort_instances_along_axis,
                __buffer,
            )
        };
    }
    pub fn remove_instance(&mut self, instance_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_remove_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_index,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_remove_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_instance_transform(
        &self,
        instance_index: i32,
        out_instance_transform: &mut crate::bindings::core_u_object::FTransform,
        b_world_space: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_instance_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(out_instance_transform);
        }
        unsafe { __buffer.add(113).cast::<bool>().read() }
    }
    pub fn get_instance_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_get_instance_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn clear_instances(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_clear_instances,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_clear_instances,
                __buffer,
            )
        };
    }
    pub fn add_instance(
        &mut self,
        transform: &crate::bindings::core_u_object::FTransform,
        sprite: UPtr<UPaperSprite>,
        b_world_space: bool,
        color: crate::bindings::core_u_object::FLinearColor,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_add_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sprite,
                __buffer.add(96).cast::<UPtr<UPaperSprite>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &color,
                __buffer.add(108).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_grouped_sprite_component_add_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(124).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPaperRuntimeSettings {
    __padding_end: [u8; 56],
}
impl UPaperRuntimeSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperRuntimeSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperRuntimeSettings")
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
pub struct UPaperSprite {
    #[doc(hidden)]
    pub(crate) __padding_264: [u8; 264],
    pub default_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub alternate_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    __padding_end: [u8; 248],
}
impl UPaperSprite {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSprite")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSprite")
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
pub struct APaperSpriteActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperSpriteComponent>,
}
impl APaperSpriteActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperSpriteActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperSpriteActor")
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
pub struct UPaperSpriteAtlas {
    __padding_end: [u8; 160],
}
impl UPaperSpriteAtlas {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteAtlas")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteAtlas")
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
pub struct UPaperSpriteBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UPaperSpriteBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteBlueprintLibrary")
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
    pub fn make_brush_from_sprite(
        sprite: UPtr<UPaperSprite>,
        width: i32,
        height: i32,
    ) -> crate::bindings::slate_core::FSlateBrush {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_blueprint_library_make_brush_from_sprite,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sprite,
                __buffer.add(0).cast::<UPtr<UPaperSprite>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&width, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::paper2_d::UPaperSpriteBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_blueprint_library_make_brush_from_sprite,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::slate_core::FSlateBrush>().read()
        }
    }
}
#[repr(C, align(16))]
pub struct UPaperSpriteComponent {
    #[doc(hidden)]
    pub(crate) __padding_1576: [u8; 1576],
    pub source_sprite: UPtr<UPaperSprite>,
    #[doc(hidden)]
    pub(crate) __padding_1592: [u8; 8],
    pub sprite_color: crate::bindings::core_u_object::FLinearColor,
}
impl UPaperSpriteComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperSpriteComponent")
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
    pub fn set_sprite_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_set_sprite_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_set_sprite_color,
                __buffer,
            )
        };
    }
    pub fn set_sprite(&mut self, new_sprite: UPtr<UPaperSprite>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_set_sprite,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_sprite,
                __buffer.add(0).cast::<UPtr<UPaperSprite>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_set_sprite,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_sprite(&mut self) -> UPtr<UPaperSprite> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_get_sprite,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_sprite_component_get_sprite,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UPaperSprite>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPaperTileLayer {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub layer_name: FText,
    pub layer_width: i32,
    pub layer_height: i32,
    pub flags_72: u8,
    pub collision_thickness_override: f32,
    pub collision_offset_override: f32,
    pub layer_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 52],
}
impl UPaperTileLayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileLayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileLayer")
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
pub struct UPaperTileMap {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub map_width: i32,
    pub map_height: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    #[doc(hidden)]
    pub(crate) __padding_76: [u8; 12],
    pub separation_per_layer: f32,
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 48],
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub tile_layers: TArray<UPtr<UPaperTileLayer>>,
    pub collision_thickness: f32,
    pub sprite_collision_domain: ESpriteCollisionMode,
    pub projection_mode: ETileMapProjectionMode,
    __padding_end: [u8; 114],
}
impl UPaperTileMap {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileMap")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileMap")
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
pub struct APaperTileMapActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub render_component: UPtr<UPaperTileMapComponent>,
}
impl APaperTileMapActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperTileMapActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperTileMapActor")
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
pub struct UPaperTileMapComponent {
    #[doc(hidden)]
    pub(crate) __padding_1656: [u8; 1656],
    pub tile_map: UPtr<UPaperTileMap>,
    __padding_end: [u8; 16],
}
impl UPaperTileMapComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileMapComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileMapComponent")
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
    pub fn set_tile_map_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile_map_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile_map_color,
                __buffer,
            )
        };
    }
    pub fn set_tile_map(&mut self, new_tile_map: UPtr<UPaperTileMap>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tile_map,
                __buffer.add(0).cast::<UPtr<UPaperTileMap>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_tile(&mut self, x: i32, y: i32, layer: i32, new_value: FPaperTileInfo) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&layer, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer.add(16).cast::<FPaperTileInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_tile,
                __buffer,
            )
        };
    }
    pub fn set_layer_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FLinearColor,
        layer: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_layer_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&layer, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_layer_color,
                __buffer,
            )
        };
    }
    pub fn set_layer_collision(
        &mut self,
        layer: i32,
        b_has_collision: bool,
        b_override_thickness: bool,
        custom_thickness: f32,
        b_override_offset: bool,
        custom_offset: f32,
        b_rebuild_collision: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_layer_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&layer, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_has_collision,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_thickness,
                __buffer.add(5).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_thickness,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_offset,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_offset,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_collision,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_layer_collision,
                __buffer,
            )
        };
    }
    pub fn set_default_collision_thickness(
        &mut self,
        thickness: f32,
        b_rebuild_collision: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_default_collision_thickness,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&thickness, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_collision,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_set_default_collision_thickness,
                __buffer,
            )
        };
    }
    pub fn resize_map(&mut self, new_width_in_tiles: i32, new_height_in_tiles: i32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_resize_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_width_in_tiles,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_height_in_tiles,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_resize_map,
                __buffer,
            )
        };
    }
    pub fn rebuild_collision(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_rebuild_collision,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_rebuild_collision,
                __buffer,
            )
        };
    }
    pub fn owns_tile_map(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_owns_tile_map,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_owns_tile_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn make_tile_map_editable(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_make_tile_map_editable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_make_tile_map_editable,
                __buffer,
            )
        };
    }
    pub fn get_tile_polygon(
        &self,
        tile_x: i32,
        tile_y: i32,
        points: &mut TArray<crate::bindings::core_u_object::FVector>,
        layer_index: i32,
        b_world_space: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_polygon,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                points,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_polygon,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(points);
        }
    }
    pub fn get_tile_map_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_map_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_map_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_tile_corner_position(
        &self,
        tile_x: i32,
        tile_y: i32,
        layer_index: i32,
        b_world_space: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_corner_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_corner_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_tile_center_position(
        &self,
        tile_x: i32,
        tile_y: i32,
        layer_index: i32,
        b_world_space: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_center_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_world_space,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile_center_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_tile(&self, x: i32, y: i32, layer: i32) -> FPaperTileInfo {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&layer, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_tile,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FPaperTileInfo>().read() }
    }
    pub fn get_map_size(
        &mut self,
        map_width: &mut i32,
        map_height: &mut i32,
        num_layers: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_map_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(map_width, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(map_height, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(num_layers, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_map_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(map_width);
        }
        unsafe {
            __buffer.add(4).cast::<i32>().swap(map_height);
        }
        unsafe {
            __buffer.add(8).cast::<i32>().swap(num_layers);
        }
    }
    pub fn get_layer_color(
        &self,
        layer: i32,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_layer_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&layer, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_get_layer_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn create_new_tile_map(
        &mut self,
        map_width: i32,
        map_height: i32,
        tile_width: i32,
        tile_height: i32,
        pixels_per_unreal_unit: f32,
        b_create_layer: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_create_new_tile_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&map_width, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&map_height, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_width, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile_height,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pixels_per_unreal_unit,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_layer,
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
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_create_new_tile_map,
                __buffer,
            )
        };
    }
    pub fn add_new_layer(&mut self) -> UPtr<UPaperTileLayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_add_new_layer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_tile_map_component_add_new_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UPaperTileLayer>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPaperTileSet {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub tile_size: crate::bindings::core_u_object::FIntPoint,
    pub tile_sheet: UPtr<crate::bindings::engine::UTexture2D>,
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 16],
    pub border_margin: FIntMargin,
    pub per_tile_spacing: crate::bindings::core_u_object::FIntPoint,
    pub drawing_offset: crate::bindings::core_u_object::FIntPoint,
    __padding_end: [u8; 80],
}
impl UPaperTileSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTileSet")
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
pub struct UMaterialExpressionSpriteTextureSampler {
    __padding_end: [u8; 656],
}
impl UMaterialExpressionSpriteTextureSampler {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionSpriteTextureSampler")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionSpriteTextureSampler")
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
pub struct APaperTerrainActor {
    #[doc(hidden)]
    pub(crate) __padding_1152: [u8; 1152],
    pub render_component: UPtr<UPaperTerrainComponent>,
}
impl APaperTerrainActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperTerrainActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APaperTerrainActor")
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
pub struct UPaperTerrainComponent {
    #[doc(hidden)]
    pub(crate) __padding_1504: [u8; 1504],
    pub terrain_material: UPtr<UPaperTerrainMaterial>,
    pub b_closed_spline: bool,
    pub b_filled_spline: bool,
    #[doc(hidden)]
    pub(crate) __padding_1536: [u8; 20],
    pub terrain_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 48],
}
impl UPaperTerrainComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainComponent")
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
    pub fn set_terrain_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_terrain_component_set_terrain_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_paper_terrain_component_set_terrain_color,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UPaperTerrainMaterial {
    __padding_end: [u8; 80],
}
impl UPaperTerrainMaterial {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainMaterial")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainMaterial")
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
pub struct UPaperTerrainSplineComponent {
    __padding_end: [u8; 2192],
}
impl UPaperTerrainSplineComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainSplineComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPaperTerrainSplineComponent")
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
pub struct UTileMapBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UTileMapBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTileMapBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTileMapBlueprintLibrary")
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
    pub fn make_tile(
        tile_index: i32,
        tile_set: UPtr<UPaperTileSet>,
        b_flip_h: bool,
        b_flip_v: bool,
        b_flip_d: bool,
    ) -> FPaperTileInfo {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_make_tile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tile_index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile_set,
                __buffer.add(8).cast::<UPtr<UPaperTileSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_flip_h, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_flip_v, __buffer.add(17).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_flip_d, __buffer.add(18).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::paper2_d::UTileMapBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_make_tile,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FPaperTileInfo>().read() }
    }
    pub fn get_tile_user_data(tile: FPaperTileInfo) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_get_tile_user_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile,
                __buffer.add(0).cast::<FPaperTileInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::paper2_d::UTileMapBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_get_tile_user_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_tile_transform(
        tile: FPaperTileInfo,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_get_tile_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile,
                __buffer.add(0).cast::<FPaperTileInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::paper2_d::UTileMapBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_get_tile_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn break_tile(
        tile: FPaperTileInfo,
        tile_index: &mut i32,
        tile_set: &mut UPtr<UPaperTileSet>,
        b_flip_h: &mut bool,
        b_flip_v: &mut bool,
        b_flip_d: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_break_tile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tile,
                __buffer.add(0).cast::<FPaperTileInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tile_index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tile_set,
                __buffer.add(24).cast::<UPtr<UPaperTileSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_flip_h, __buffer.add(32).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_flip_v, __buffer.add(33).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_flip_d, __buffer.add(34).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::paper2_d::UTileMapBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::paper2_d::__FUNCTION_PTRS
                    .u_tile_map_blueprint_library_break_tile,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(tile_index);
        }
        unsafe {
            __buffer.add(24).cast::<UPtr<UPaperTileSet>>().swap(tile_set);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(b_flip_h);
        }
        unsafe {
            __buffer.add(33).cast::<bool>().swap(b_flip_v);
        }
        unsafe {
            __buffer.add(34).cast::<bool>().swap(b_flip_d);
        }
    }
}
#[repr(C, align(8))]
pub struct FPaperFlipbookComponent_OnFinishedPlaying {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ESpriteShapeType(pub u8);
impl ESpriteShapeType {
    pub const BOX: ESpriteShapeType = ESpriteShapeType(0);
    pub const CIRCLE: ESpriteShapeType = ESpriteShapeType(1);
    pub const POLYGON: ESpriteShapeType = ESpriteShapeType(2);
}
#[repr(transparent)]
pub struct ESpritePolygonMode(pub u8);
impl ESpritePolygonMode {
    pub const SOURCE_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(0);
    pub const TIGHT_BOUNDING_BOX: ESpritePolygonMode = ESpritePolygonMode(1);
    pub const SHRINK_WRAPPED: ESpritePolygonMode = ESpritePolygonMode(2);
    pub const FULLY_CUSTOM: ESpritePolygonMode = ESpritePolygonMode(3);
    pub const DICED: ESpritePolygonMode = ESpritePolygonMode(4);
}
#[repr(transparent)]
pub struct EFlipbookCollisionMode(pub u8);
impl EFlipbookCollisionMode {
    pub const NO_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(0);
    pub const FIRST_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(1);
    pub const EACH_FRAME_COLLISION: EFlipbookCollisionMode = EFlipbookCollisionMode(2);
}
#[repr(transparent)]
pub struct ESpriteCollisionMode(pub u8);
impl ESpriteCollisionMode {
    pub const NONE: ESpriteCollisionMode = ESpriteCollisionMode(0);
    pub const USE2_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(1);
    pub const USE3_D_PHYSICS: ESpriteCollisionMode = ESpriteCollisionMode(2);
}
#[repr(transparent)]
pub struct ESpritePivotMode(pub u8);
impl ESpritePivotMode {
    pub const TOP_LEFT: ESpritePivotMode = ESpritePivotMode(0);
    pub const TOP_CENTER: ESpritePivotMode = ESpritePivotMode(1);
    pub const TOP_RIGHT: ESpritePivotMode = ESpritePivotMode(2);
    pub const CENTER_LEFT: ESpritePivotMode = ESpritePivotMode(3);
    pub const CENTER_CENTER: ESpritePivotMode = ESpritePivotMode(4);
    pub const CENTER_RIGHT: ESpritePivotMode = ESpritePivotMode(5);
    pub const BOTTOM_LEFT: ESpritePivotMode = ESpritePivotMode(6);
    pub const BOTTOM_CENTER: ESpritePivotMode = ESpritePivotMode(7);
    pub const BOTTOM_RIGHT: ESpritePivotMode = ESpritePivotMode(8);
    pub const CUSTOM: ESpritePivotMode = ESpritePivotMode(9);
}
#[repr(transparent)]
pub struct EPaperSpriteAtlasPadding(pub u8);
impl EPaperSpriteAtlasPadding {
    pub const DILATE_BORDER: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(0);
    pub const PAD_WITH_ZERO: EPaperSpriteAtlasPadding = EPaperSpriteAtlasPadding(1);
}
#[repr(transparent)]
pub struct ETileMapProjectionMode(pub u8);
impl ETileMapProjectionMode {
    pub const ORTHOGONAL: ETileMapProjectionMode = ETileMapProjectionMode(0);
    pub const ISOMETRIC_DIAMOND: ETileMapProjectionMode = ETileMapProjectionMode(1);
    pub const ISOMETRIC_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(2);
    pub const HEXAGONAL_STAGGERED: ETileMapProjectionMode = ETileMapProjectionMode(3);
}
