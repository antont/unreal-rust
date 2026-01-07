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
pub static mut A_MEDIA_PLATE_SET_HOLDOUT_COMPOSITE_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_MEDIA_PLATE_IS_HOLDOUT_COMPOSITE_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_PLAY_ONLY_WHEN_VISIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_MESH_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_LOOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_LETTERBOX_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_IS_ASPECT_RATIO_AUTO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SET_ENABLE_AUDIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_SOURCE_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_PLAYLIST_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SELECT_EXTERNAL_MEDIA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_SEEK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_REWIND: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_PREVIOUS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_PLAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_PAUSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_OPEN_LATENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_SUSPENDED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_RESUMED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_OPENED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_END: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_NEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_IS_MEDIA_PLATE_PLAYING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_IS_EVENT_STATE_CHANGE_ALLOWED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_SELECTED_MEDIA_SOURCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_MESH_RANGE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_TEXTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYLIST: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_LOOP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_LETTERBOX_ASPECT_RATIO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_GET_IS_ASPECT_RATIO_AUTO: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MEDIA_PLATE_COMPONENT_CLOSE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AMediaPlate::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHoldoutCompositeEnabled"),
            &raw mut A_MEDIA_PLATE_SET_HOLDOUT_COMPOSITE_ENABLED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHoldoutCompositeEnabled"),
            &raw mut A_MEDIA_PLATE_IS_HOLDOUT_COMPOSITE_ENABLED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMediaPlateComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlayOnlyWhenVisible"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_PLAY_ONLY_WHEN_VISIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeshRange"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_MESH_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLoop"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_LOOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLetterboxAspectRatio"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_LETTERBOX_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsAspectRatioAuto"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_IS_ASPECT_RATIO_AUTO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableAudio"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SET_ENABLE_AUDIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectMediaSourceAsset"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_SOURCE_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectMediaPlaylistAsset"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_PLAYLIST_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectExternalMedia"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SELECT_EXTERNAL_MEDIA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Seek"),
            &raw mut U_MEDIA_PLATE_COMPONENT_SEEK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Rewind"),
            &raw mut U_MEDIA_PLATE_COMPONENT_REWIND,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Previous"),
            &raw mut U_MEDIA_PLATE_COMPONENT_PREVIOUS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Play"),
            &raw mut U_MEDIA_PLATE_COMPONENT_PLAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pause"),
            &raw mut U_MEDIA_PLATE_COMPONENT_PAUSE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenLatent"),
            &raw mut U_MEDIA_PLATE_COMPONENT_OPEN_LATENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Open"),
            &raw mut U_MEDIA_PLATE_COMPONENT_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMediaSuspended"),
            &raw mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_SUSPENDED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMediaResumed"),
            &raw mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_RESUMED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMediaOpened"),
            &raw mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_OPENED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnMediaEnd"),
            &raw mut U_MEDIA_PLATE_COMPONENT_ON_MEDIA_END,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Next"),
            &raw mut U_MEDIA_PLATE_COMPONENT_NEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMediaPlatePlaying"),
            &raw mut U_MEDIA_PLATE_COMPONENT_IS_MEDIA_PLATE_PLAYING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEventStateChangeAllowed"),
            &raw mut U_MEDIA_PLATE_COMPONENT_IS_EVENT_STATE_CHANGE_ALLOWED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedMediaSource"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_SELECTED_MEDIA_SOURCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshRange"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_MESH_RANGE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaTexture"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_TEXTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaPlaylist"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYLIST,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMediaPlayer"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoop"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_LOOP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLetterboxAspectRatio"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_LETTERBOX_ASPECT_RATIO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsAspectRatioAuto"),
            &raw mut U_MEDIA_PLATE_COMPONENT_GET_IS_ASPECT_RATIO_AUTO,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Close"),
            &raw mut U_MEDIA_PLATE_COMPONENT_CLOSE,
        );
    }
}
#[repr(C, align(8))]
pub struct FMediaPlateResource {
    __padding_end: [u8; 128],
}
impl FMediaPlateResource {}
#[repr(C, align(8))]
pub struct AMediaPlate {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub media_plate_component: UPtr<UMediaPlateComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    pub b_enable_holdout_composite: bool,
    __padding_end: [u8; 23],
}
impl AMediaPlate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMediaPlate")
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
    pub fn set_holdout_composite_enabled(&mut self, b_in_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::A_MEDIA_PLATE_SET_HOLDOUT_COMPOSITE_ENABLED,
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
                crate::bindings::media_plate::A_MEDIA_PLATE_SET_HOLDOUT_COMPOSITE_ENABLED,
                __buffer,
            )
        };
    }
    pub fn is_holdout_composite_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::A_MEDIA_PLATE_IS_HOLDOUT_COMPOSITE_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::A_MEDIA_PLATE_IS_HOLDOUT_COMPOSITE_ENABLED,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateAssetUserData")
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
pub struct UMediaPlateComponent {
    #[doc(hidden)]
    __padding_248: [u8; 248],
    pub b_play_on_open: bool,
    pub start_time: f32,
    #[doc(hidden)]
    __padding_296: [u8; 40],
    pub media_plate_resource: FMediaPlateResource,
    pub playlist_index: i32,
    #[doc(hidden)]
    __padding_436: [u8; 8],
    pub b_is_media_plate_playing: bool,
    #[doc(hidden)]
    __padding_456: [u8; 19],
    pub b_play_only_when_visible: bool,
    pub b_loop: bool,
    #[doc(hidden)]
    __padding_464: [u8; 6],
    pub b_is_aspect_ratio_auto: bool,
    __padding_end: [u8; 239],
}
impl UMediaPlateComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMediaPlateComponent")
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
    pub fn set_play_only_when_visible(&mut self, b_in_play_only_when_visible: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_PLAY_ONLY_WHEN_VISIBLE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_PLAY_ONLY_WHEN_VISIBLE,
                __buffer,
            )
        };
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_MESH_RANGE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_MESH_RANGE,
                __buffer,
            )
        };
    }
    pub fn set_loop(&mut self, b_in_loop: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_LOOP,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_LOOP,
                __buffer,
            )
        };
    }
    pub fn set_letterbox_aspect_ratio(&mut self, aspect_ratio: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_LETTERBOX_ASPECT_RATIO,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_LETTERBOX_ASPECT_RATIO,
                __buffer,
            )
        };
    }
    pub fn set_is_aspect_ratio_auto(&mut self, b_in_is_aspect_ratio_auto: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_IS_ASPECT_RATIO_AUTO,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_IS_ASPECT_RATIO_AUTO,
                __buffer,
            )
        };
    }
    pub fn set_enable_audio(&mut self, b_in_enable_audio: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_ENABLE_AUDIO,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SET_ENABLE_AUDIO,
                __buffer,
            )
        };
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_SOURCE_ASSET,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_SOURCE_ASSET,
                __buffer,
            )
        };
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_PLAYLIST_ASSET,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_MEDIA_PLAYLIST_ASSET,
                __buffer,
            )
        };
    }
    pub fn select_external_media(&mut self, in_file_path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_EXTERNAL_MEDIA,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SELECT_EXTERNAL_MEDIA,
                __buffer,
            )
        };
    }
    pub fn seek(&mut self, time: &crate::bindings::core_u_object::FTimespan) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SEEK,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_SEEK,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_REWIND,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_REWIND,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PREVIOUS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PREVIOUS,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PLAY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PLAY,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PAUSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_PAUSE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_OPEN_LATENT,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_OPEN_LATENT,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(45).cast::<bool>().swap(b_out_success);
        }
    }
    pub fn open(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_OPEN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_OPEN,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_NEXT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_NEXT,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_IS_MEDIA_PLATE_PLAYING,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_IS_MEDIA_PLATE_PLAYING,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_IS_EVENT_STATE_CHANGE_ALLOWED,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_IS_EVENT_STATE_CHANGE_ALLOWED,
                __buffer,
            )
        };
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_SELECTED_MEDIA_SOURCE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_SELECTED_MEDIA_SOURCE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MESH_RANGE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MESH_RANGE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_TEXTURE,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_TEXTURE,
                __buffer,
            )
        };
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYLIST,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYLIST,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYER,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_MEDIA_PLAYER,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_LOOP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_LOOP,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_LETTERBOX_ASPECT_RATIO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_LETTERBOX_ASPECT_RATIO,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_IS_ASPECT_RATIO_AUTO,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_GET_IS_ASPECT_RATIO_AUTO,
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
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_CLOSE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::media_plate::U_MEDIA_PLATE_COMPONENT_CLOSE,
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
