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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UClothPainterSettings {
    __padding_end: [u8; 136],
}
impl UClothPainterSettings {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPainterSettings")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPainterSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClothingAssetExporter {
    __padding_end: [u8; 48],
}
impl UClothingAssetExporter {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetExporter")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothingAssetExporter")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClothPaintTool_BrushSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_BrushSettings {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_BrushSettings")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_BrushSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClothPaintTool_GradientSettings {
    __padding_end: [u8; 64],
}
impl UClothPaintTool_GradientSettings {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_GradientSettings")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_GradientSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClothPaintTool_SmoothSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_SmoothSettings {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_SmoothSettings")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_SmoothSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UClothPaintTool_FillSettings {
    __padding_end: [u8; 56],
}
impl UClothPaintTool_FillSettings {
    pub fn static_class() -> crate::core_data::UPtr<crate::bindings::core_u_object::UClass> {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_FillSettings")
            .unwrap();
        crate::core_data::UPtr { ptr: ptr.cast() }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClothPaintTool_FillSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct ESourceColorChannel(pub u8);
impl ESourceColorChannel {
    pub const RED: ESourceColorChannel = ESourceColorChannel(0);
    pub const GREEN: ESourceColorChannel = ESourceColorChannel(1);
    pub const BLUE: ESourceColorChannel = ESourceColorChannel(2);
    pub const ALPHA: ESourceColorChannel = ESourceColorChannel(3);
}
