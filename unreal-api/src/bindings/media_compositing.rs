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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UMovieSceneMediaPlayerPropertySection {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub b_loop: bool,
}
impl UMovieSceneMediaPlayerPropertySection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMediaPlayerPropertySection")
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
pub struct UMovieSceneMediaPlayerPropertyTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneMediaPlayerPropertyTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMediaPlayerPropertyTrack")
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
pub struct UMovieSceneMediaSection {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub media_source: UPtr<crate::bindings::media_assets::UMediaSource>,
    pub media_source_proxy_index: i32,
    pub b_looping: bool,
    pub start_frame_offset: crate::bindings::core_u_object::FFrameNumber,
    pub media_texture: UPtr<crate::bindings::media_assets::UMediaTexture>,
    pub media_sound_component: UPtr<crate::bindings::media_assets::UMediaSoundComponent>,
    pub b_use_external_media_player: bool,
    pub external_media_player: UPtr<crate::bindings::media_assets::UMediaPlayer>,
    pub cache_settings: crate::bindings::media_assets::FMediaSourceCacheSettings,
    pub texture_index: i32,
    pub b_manual_frame_rate_alignment: bool,
    pub frame_rate_alignment: crate::bindings::core_u_object::FFrameRate,
    __padding_end: [u8; 328],
}
impl UMovieSceneMediaSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMediaSection")
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
pub struct UMovieSceneMediaTrack {
    __padding_end: [u8; 456],
}
impl UMovieSceneMediaTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMediaTrack")
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
