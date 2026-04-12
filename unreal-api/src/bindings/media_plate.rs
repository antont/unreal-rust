#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub a_media_plate_set_holdout_composite_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_media_plate_is_holdout_composite_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_play_only_when_visible: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_mesh_range: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_loop: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_letterbox_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_is_aspect_ratio_auto: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_set_enable_audio: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_select_media_source_asset: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_select_media_playlist_asset: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_select_external_media: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_seek: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_rewind: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_previous: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_play: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_pause: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_open_latent: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_open: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_on_media_suspended: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_on_media_resumed: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_on_media_opened: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_on_media_end: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_next: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_is_media_plate_playing: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_is_event_state_change_allowed: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_selected_media_source: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_mesh_range: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_media_texture: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_media_playlist: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_media_player: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_loop: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_letterbox_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_get_is_aspect_ratio_auto: *mut crate::ffi::UFunctionOpague,
    pub u_media_plate_component_close: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_media_plate_set_holdout_composite_enabled: std::ptr::null_mut(),
            a_media_plate_is_holdout_composite_enabled: std::ptr::null_mut(),
            u_media_plate_component_set_play_only_when_visible: std::ptr::null_mut(),
            u_media_plate_component_set_mesh_range: std::ptr::null_mut(),
            u_media_plate_component_set_loop: std::ptr::null_mut(),
            u_media_plate_component_set_letterbox_aspect_ratio: std::ptr::null_mut(),
            u_media_plate_component_set_is_aspect_ratio_auto: std::ptr::null_mut(),
            u_media_plate_component_set_enable_audio: std::ptr::null_mut(),
            u_media_plate_component_select_media_source_asset: std::ptr::null_mut(),
            u_media_plate_component_select_media_playlist_asset: std::ptr::null_mut(),
            u_media_plate_component_select_external_media: std::ptr::null_mut(),
            u_media_plate_component_seek: std::ptr::null_mut(),
            u_media_plate_component_rewind: std::ptr::null_mut(),
            u_media_plate_component_previous: std::ptr::null_mut(),
            u_media_plate_component_play: std::ptr::null_mut(),
            u_media_plate_component_pause: std::ptr::null_mut(),
            u_media_plate_component_open_latent: std::ptr::null_mut(),
            u_media_plate_component_open: std::ptr::null_mut(),
            u_media_plate_component_on_media_suspended: std::ptr::null_mut(),
            u_media_plate_component_on_media_resumed: std::ptr::null_mut(),
            u_media_plate_component_on_media_opened: std::ptr::null_mut(),
            u_media_plate_component_on_media_end: std::ptr::null_mut(),
            u_media_plate_component_next: std::ptr::null_mut(),
            u_media_plate_component_is_media_plate_playing: std::ptr::null_mut(),
            u_media_plate_component_is_event_state_change_allowed: std::ptr::null_mut(),
            u_media_plate_component_get_selected_media_source: std::ptr::null_mut(),
            u_media_plate_component_get_mesh_range: std::ptr::null_mut(),
            u_media_plate_component_get_media_texture: std::ptr::null_mut(),
            u_media_plate_component_get_media_playlist: std::ptr::null_mut(),
            u_media_plate_component_get_media_player: std::ptr::null_mut(),
            u_media_plate_component_get_loop: std::ptr::null_mut(),
            u_media_plate_component_get_letterbox_aspect_ratio: std::ptr::null_mut(),
            u_media_plate_component_get_is_aspect_ratio_auto: std::ptr::null_mut(),
            u_media_plate_component_close: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AMediaPlate::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHoldoutCompositeEnabled"),
                &raw mut __FUNCTION_PTRS.a_media_plate_set_holdout_composite_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHoldoutCompositeEnabled"),
                &raw mut __FUNCTION_PTRS.a_media_plate_is_holdout_composite_enabled,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMediaPlateComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlayOnlyWhenVisible"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_set_play_only_when_visible,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMeshRange"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_set_mesh_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLoop"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_set_loop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLetterboxAspectRatio"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_set_letterbox_aspect_ratio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsAspectRatioAuto"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_set_is_aspect_ratio_auto,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableAudio"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_set_enable_audio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectMediaSourceAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_select_media_source_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectMediaPlaylistAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_select_media_playlist_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectExternalMedia"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_select_external_media,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Seek"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_seek,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Rewind"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_rewind,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Previous"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_previous,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Play"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Pause"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_pause,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenLatent"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_open_latent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Open"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_open,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMediaSuspended"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_on_media_suspended,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMediaResumed"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_on_media_resumed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMediaOpened"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_on_media_opened,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMediaEnd"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_on_media_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Next"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_next,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsMediaPlatePlaying"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_is_media_plate_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEventStateChangeAllowed"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_is_event_state_change_allowed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedMediaSource"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_get_selected_media_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMeshRange"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_mesh_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaTexture"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_media_texture,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaPlaylist"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_media_playlist,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMediaPlayer"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_media_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLoop"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_loop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLetterboxAspectRatio"),
                &raw mut __FUNCTION_PTRS
                    .u_media_plate_component_get_letterbox_aspect_ratio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIsAspectRatioAuto"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_get_is_aspect_ratio_auto,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Close"),
                &raw mut __FUNCTION_PTRS.u_media_plate_component_close,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMediaPlateResource {
    pub(crate) __padding_end: [u8; 128],
}
impl FMediaPlateResource {}
#[repr(C, align(8))]
pub struct AMediaPlate {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub media_plate_component: UPtr<UMediaPlateComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub b_enable_holdout_composite: bool,
    __padding_end: [u8; 23],
}
impl AMediaPlate {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMediaPlate")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMediaPlate")
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
    pub fn set_holdout_composite_enabled(&mut self, b_in_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .a_media_plate_set_holdout_composite_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enabled,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .a_media_plate_set_holdout_composite_enabled,
                __buffer,
            )
        };
        std::mem::forget(b_in_enabled);
    }
    pub fn is_holdout_composite_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .a_media_plate_is_holdout_composite_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .a_media_plate_is_holdout_composite_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMediaPlateAssetUserData {
    __padding_end: [u8; 72],
}
impl UMediaPlateAssetUserData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateAssetUserData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateAssetUserData")
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
pub struct UMediaPlateComponent {
    #[doc(hidden)]
    pub(crate) __padding_248: [u8; 248],
    pub b_play_on_open: bool,
    pub start_time: f32,
    #[doc(hidden)]
    pub(crate) __padding_296: [u8; 40],
    pub media_plate_resource: FMediaPlateResource,
    pub playlist_index: i32,
    #[doc(hidden)]
    pub(crate) __padding_436: [u8; 8],
    pub b_is_media_plate_playing: bool,
    #[doc(hidden)]
    pub(crate) __padding_456: [u8; 19],
    pub b_play_only_when_visible: bool,
    pub b_loop: bool,
    #[doc(hidden)]
    pub(crate) __padding_464: [u8; 6],
    pub b_is_aspect_ratio_auto: bool,
    __padding_end: [u8; 239],
}
impl UMediaPlateComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateComponent")
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
    pub fn set_play_only_when_visible(&mut self, b_in_play_only_when_visible: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_play_only_when_visible,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_play_only_when_visible,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_play_only_when_visible,
                __buffer,
            )
        };
        std::mem::forget(b_in_play_only_when_visible);
    }
    pub fn set_mesh_range(
        &mut self,
        in_mesh_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_mesh_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mesh_range,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_mesh_range,
                __buffer,
            )
        };
        std::mem::forget(in_mesh_range);
    }
    pub fn set_loop(&mut self, b_in_loop: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_loop,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_in_loop, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_loop,
                __buffer,
            )
        };
        std::mem::forget(b_in_loop);
    }
    pub fn set_letterbox_aspect_ratio(&mut self, aspect_ratio: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_letterbox_aspect_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &aspect_ratio,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_letterbox_aspect_ratio,
                __buffer,
            )
        };
        std::mem::forget(aspect_ratio);
    }
    pub fn set_is_aspect_ratio_auto(&mut self, b_in_is_aspect_ratio_auto: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_is_aspect_ratio_auto,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_aspect_ratio_auto,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_is_aspect_ratio_auto,
                __buffer,
            )
        };
        std::mem::forget(b_in_is_aspect_ratio_auto);
    }
    pub fn set_enable_audio(&mut self, b_in_enable_audio: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_enable_audio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enable_audio,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_set_enable_audio,
                __buffer,
            )
        };
        std::mem::forget(b_in_enable_audio);
    }
    pub fn select_media_source_asset(
        &mut self,
        in_media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_media_source_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_media_source,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::media_assets::UMediaSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_media_source_asset,
                __buffer,
            )
        };
        std::mem::forget(in_media_source);
    }
    pub fn select_media_playlist_asset(
        &mut self,
        in_media_playlist: UPtr<crate::bindings::media_assets::UMediaPlaylist>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_media_playlist_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_media_playlist,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::media_assets::UMediaPlaylist>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_media_playlist_asset,
                __buffer,
            )
        };
        std::mem::forget(in_media_playlist);
    }
    pub fn select_external_media(&mut self, in_file_path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_external_media,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_file_path,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_select_external_media,
                __buffer,
            )
        };
        std::mem::forget(in_file_path);
    }
    pub fn seek(&mut self, time: &crate::bindings::core_u_object::FTimespan) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_seek,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTimespan>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_seek,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn rewind(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_rewind,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_rewind,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn previous(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_previous,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_previous,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn play(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_play,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_play,
                __buffer,
            )
        };
    }
    pub fn pause(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_pause,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_pause,
                __buffer,
            )
        };
    }
    pub fn open_latent(
        &mut self,
        in_world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_latent_info: crate::bindings::engine::FLatentActionInfo,
        in_timeout: f32,
        b_in_wait_for_texture: bool,
        b_out_success: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<46>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_open_latent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_latent_info,
                __buffer.add(8).cast::<crate::bindings::engine::FLatentActionInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_timeout,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_wait_for_texture,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_success,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_open_latent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(45).cast::<bool>().swap(b_out_success);
        }
        std::mem::forget(in_world_context_object);
        std::mem::forget(in_latent_info);
        std::mem::forget(in_timeout);
        std::mem::forget(b_in_wait_for_texture);
    }
    pub fn open(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_open,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_open,
                __buffer,
            )
        };
    }
    pub fn next(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_next,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_next,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_media_plate_playing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_is_media_plate_playing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_is_media_plate_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_event_state_change_allowed(
        &self,
        in_request_event_state: EMediaPlateEventState,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_is_event_state_change_allowed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_request_event_state,
                __buffer.add(0).cast::<EMediaPlateEventState>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_is_event_state_change_allowed,
                __buffer,
            )
        };
        std::mem::forget(in_request_event_state);
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_selected_media_source(
        &self,
    ) -> UPtr<crate::bindings::media_assets::UMediaSource> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_selected_media_source,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_selected_media_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::media_assets::UMediaSource>>()
                .read()
        }
    }
    pub fn get_mesh_range(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_mesh_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_mesh_range,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_media_texture(
        &mut self,
        index: i32,
    ) -> UPtr<crate::bindings::media_assets::UMediaTexture> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_texture,
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
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_texture,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::media_assets::UMediaTexture>>()
                .read()
        }
    }
    pub fn get_media_playlist(
        &self,
    ) -> UPtr<crate::bindings::media_assets::UMediaPlaylist> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_playlist,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_playlist,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::media_assets::UMediaPlaylist>>()
                .read()
        }
    }
    pub fn get_media_player(
        &mut self,
    ) -> UPtr<crate::bindings::media_assets::UMediaPlayer> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_player,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_media_player,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::media_assets::UMediaPlayer>>()
                .read()
        }
    }
    pub fn get_loop(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_loop,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_loop,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_letterbox_aspect_ratio(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_letterbox_aspect_ratio,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_letterbox_aspect_ratio,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_is_aspect_ratio_auto(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_is_aspect_ratio_auto,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_get_is_aspect_ratio_auto,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn close(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_close,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::__FUNCTION_PTRS
                    .u_media_plate_component_close,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EMediaPlateResourceType(pub u8);
impl EMediaPlateResourceType {
    pub const PLAYLIST: EMediaPlateResourceType = EMediaPlateResourceType(0);
    pub const EXTERNAL: EMediaPlateResourceType = EMediaPlateResourceType(1);
    pub const ASSET: EMediaPlateResourceType = EMediaPlateResourceType(2);
}
#[repr(transparent)]
pub struct EMediaPlateEventState(pub u8);
impl EMediaPlateEventState {
    pub const PLAY: EMediaPlateEventState = EMediaPlateEventState(0);
    pub const OPEN: EMediaPlateEventState = EMediaPlateEventState(1);
    pub const CLOSE: EMediaPlateEventState = EMediaPlateEventState(2);
    pub const PAUSE: EMediaPlateEventState = EMediaPlateEventState(3);
    pub const REVERSE: EMediaPlateEventState = EMediaPlateEventState(4);
    pub const FORWARD: EMediaPlateEventState = EMediaPlateEventState(5);
    pub const REWIND: EMediaPlateEventState = EMediaPlateEventState(6);
    pub const NEXT: EMediaPlateEventState = EMediaPlateEventState(7);
    pub const PREVIOUS: EMediaPlateEventState = EMediaPlateEventState(8);
    pub const MAX: EMediaPlateEventState = EMediaPlateEventState(9);
}
