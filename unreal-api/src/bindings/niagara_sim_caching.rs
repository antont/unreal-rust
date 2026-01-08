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
pub struct UMovieSceneNiagaraCacheSection {
    __padding_end: [u8; 528],
}
impl UMovieSceneNiagaraCacheSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraCacheSection")
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
pub struct UMovieSceneNiagaraCacheTrack {
    __padding_end: [u8; 432],
}
impl UMovieSceneNiagaraCacheTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraCacheTrack")
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
#[repr(transparent)]
pub struct ENiagaraSimCacheSectionPlayMode(pub u8);
impl ENiagaraSimCacheSectionPlayMode {
    pub const SIM_WITHOUT_CACHE: ENiagaraSimCacheSectionPlayMode = ENiagaraSimCacheSectionPlayMode(
        0,
    );
    pub const DISPLAY_CACHE_ONLY: ENiagaraSimCacheSectionPlayMode = ENiagaraSimCacheSectionPlayMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraSimCacheSectionStretchMode(pub u8);
impl ENiagaraSimCacheSectionStretchMode {
    pub const REPEAT: ENiagaraSimCacheSectionStretchMode = ENiagaraSimCacheSectionStretchMode(
        0,
    );
    pub const TIME_DILATE: ENiagaraSimCacheSectionStretchMode = ENiagaraSimCacheSectionStretchMode(
        1,
    );
}
