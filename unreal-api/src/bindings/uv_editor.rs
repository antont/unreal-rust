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
    pub uuv_editor_udim_properties_set_udi_ms_from_texture: *mut crate::ffi::UFunctionOpague,
    pub uuv_editor_udim_properties_set_udi_ms_from_asset: *mut crate::ffi::UFunctionOpague,
    pub uuv_editor_udim_properties_get_asset_names: *mut crate::ffi::UFunctionOpague,
    pub uuv_editor_udim_properties_asset_by_index: *mut crate::ffi::UFunctionOpague,
    pub uuv_editor_uv_channel_properties_get_uv_channel_names: *mut crate::ffi::UFunctionOpague,
    pub uuv_editor_uv_channel_properties_get_asset_names: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            uuv_editor_udim_properties_set_udi_ms_from_texture: std::ptr::null_mut(),
            uuv_editor_udim_properties_set_udi_ms_from_asset: std::ptr::null_mut(),
            uuv_editor_udim_properties_get_asset_names: std::ptr::null_mut(),
            uuv_editor_udim_properties_asset_by_index: std::ptr::null_mut(),
            uuv_editor_uv_channel_properties_get_uv_channel_names: std::ptr::null_mut(),
            uuv_editor_uv_channel_properties_get_asset_names: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUVEditorUDIMProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUDIMsFromTexture"),
            &raw mut __FUNCTION_PTRS.uuv_editor_udim_properties_set_udi_ms_from_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUDIMsFromAsset"),
            &raw mut __FUNCTION_PTRS.uuv_editor_udim_properties_set_udi_ms_from_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetNames"),
            &raw mut __FUNCTION_PTRS.uuv_editor_udim_properties_get_asset_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssetByIndex"),
            &raw mut __FUNCTION_PTRS.uuv_editor_udim_properties_asset_by_index,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUVEditorUVChannelProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUVChannelNames"),
            &raw mut __FUNCTION_PTRS
                .uuv_editor_uv_channel_properties_get_uv_channel_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetNames"),
            &raw mut __FUNCTION_PTRS.uuv_editor_uv_channel_properties_get_asset_names,
        );
    }
}
#[repr(C, align(8))]
pub struct UUVEditorInitializationContext {
    __padding_end: [u8; 72],
}
impl UUVEditorInitializationContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorInitializationContext")
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
pub struct UUVEditorUnwrappedUXProperties {
    __padding_end: [u8; 208],
}
impl UUVEditorUnwrappedUXProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorUnwrappedUXProperties")
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
pub struct UUVEditorLivePreviewUXProperties {
    __padding_end: [u8; 200],
}
impl UUVEditorLivePreviewUXProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorLivePreviewUXProperties")
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
pub struct UUnsetUVsAction {
    __padding_end: [u8; 72],
}
impl UUnsetUVsAction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnsetUVsAction")
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
pub struct UUVEditor {
    __padding_end: [u8; 96],
}
impl UUVEditor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditor")
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
pub struct UUVEditor2DViewportContext {
    __padding_end: [u8; 192],
}
impl UUVEditor2DViewportContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditor2DViewportContext")
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
pub struct UUVEditor3DViewportMode {
    __padding_end: [u8; 312],
}
impl UUVEditor3DViewportMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditor3DViewportMode")
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
pub struct UUVEditorBackgroundPreviewProperties {
    __padding_end: [u8; 232],
}
impl UUVEditorBackgroundPreviewProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorBackgroundPreviewProperties")
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
pub struct UUVEditorBackgroundPreview {
    __padding_end: [u8; 360],
}
impl UUVEditorBackgroundPreview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorBackgroundPreview")
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
pub struct UUVEditorDistortionVisualizationProperties {
    __padding_end: [u8; 280],
}
impl UUVEditorDistortionVisualizationProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorDistortionVisualizationProperties")
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
pub struct UUVEditorDistortionVisualization {
    __padding_end: [u8; 104],
}
impl UUVEditorDistortionVisualization {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorDistortionVisualization")
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
pub struct UUVEditorGridProperties {
    __padding_end: [u8; 192],
}
impl UUVEditorGridProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorGridProperties")
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
pub struct UUVEditorUDIMProperties {
    __padding_end: [u8; 248],
}
impl UUVEditorUDIMProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorUDIMProperties")
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
pub struct UUVEditorMode {
    __padding_end: [u8; 776],
}
impl UUVEditorMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorMode")
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
pub struct UUVEditorUVChannelProperties {
    __padding_end: [u8; 264],
}
impl UUVEditorUVChannelProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorUVChannelProperties")
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
pub struct UUVEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UUVEditorUISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorUISubsystem")
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
pub struct UUVEditorSubsystem {
    __padding_end: [u8; 144],
}
impl UUVEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVEditorSubsystem")
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
pub struct EUVEditorBackgroundSourceType(pub i32);
impl EUVEditorBackgroundSourceType {
    pub const CHECKERBOARD: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(
        0,
    );
    pub const TEXTURE: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(1);
    pub const MATERIAL: EUVEditorBackgroundSourceType = EUVEditorBackgroundSourceType(2);
}
#[repr(transparent)]
pub struct EDistortionMetric(pub u8);
impl EDistortionMetric {
    pub const REED_BETA: EDistortionMetric = EDistortionMetric(0);
    pub const SANDER_L2: EDistortionMetric = EDistortionMetric(1);
    pub const SANDER_L_INF: EDistortionMetric = EDistortionMetric(2);
    pub const TEXEL_DENSITY: EDistortionMetric = EDistortionMetric(3);
}
