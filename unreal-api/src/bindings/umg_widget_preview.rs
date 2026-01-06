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
pub static mut U_WIDGET_PREVIEW_GET_WIDGET_SLOT_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WIDGET_PREVIEW_GET_AVAILABLE_WIDGET_SLOT_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWidgetPreview::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidgetSlotNames"),
            &raw mut U_WIDGET_PREVIEW_GET_WIDGET_SLOT_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAvailableWidgetSlotNames"),
            &raw mut U_WIDGET_PREVIEW_GET_AVAILABLE_WIDGET_SLOT_NAMES,
        );
    }
}
#[repr(C, align(8))]
pub struct FPreviewableWidgetVariant {
    pub object_path: crate::bindings::core_u_object::FSoftObjectPath,
    __padding_end: [u8; 16],
}
impl FPreviewableWidgetVariant {}
#[repr(C, align(8))]
pub struct UAssetDefinition_WidgetPreview {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_WidgetPreview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_WidgetPreview")
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
pub struct UWidgetPreview {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub widget_type: FPreviewableWidgetVariant,
    pub slot_widget_types: TMap<FName, FPreviewableWidgetVariant>,
    pub b_should_override_widget_size: bool,
    pub overridden_widget_size: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 88],
}
impl UWidgetPreview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetPreview")
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
pub struct UWidgetPreviewEditor {
    __padding_end: [u8; 72],
}
impl UWidgetPreviewEditor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetPreviewEditor")
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
pub struct UWidgetPreviewFactory {
    __padding_end: [u8; 136],
}
impl UWidgetPreviewFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWidgetPreviewFactory")
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
