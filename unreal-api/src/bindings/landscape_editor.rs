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
pub struct ALandscapePlaceholder {
    __padding_end: [u8; 1136],
}
impl ALandscapePlaceholder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapePlaceholder")
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
pub struct UActorFactoryLandscape {
    __padding_end: [u8; 144],
}
impl UActorFactoryLandscape {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryLandscape")
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
pub struct ULandscapeEditorObject {
    __padding_end: [u8; 832],
}
impl ULandscapeEditorObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditorObject")
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
pub struct ULandscapeSplineSelection {
    __padding_end: [u8; 120],
}
impl ULandscapeSplineSelection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSplineSelection")
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
pub struct ELandscapeImportResult(pub u8);
impl ELandscapeImportResult {
    pub const SUCCESS: ELandscapeImportResult = ELandscapeImportResult(0);
    pub const WARNING: ELandscapeImportResult = ELandscapeImportResult(1);
    pub const ERROR: ELandscapeImportResult = ELandscapeImportResult(2);
}
#[repr(transparent)]
pub struct ELandscapeToolFlattenMode(pub i8);
impl ELandscapeToolFlattenMode {
    pub const INVALID: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(-1);
    pub const BOTH: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(0);
    pub const RAISE: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(1);
    pub const LOWER: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(2);
    pub const INTERVAL: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(3);
    pub const TERRACE: ELandscapeToolFlattenMode = ELandscapeToolFlattenMode(4);
}
#[repr(transparent)]
pub struct ELandscapeToolErosionMode(pub i8);
impl ELandscapeToolErosionMode {
    pub const INVALID: ELandscapeToolErosionMode = ELandscapeToolErosionMode(-1);
    pub const BOTH: ELandscapeToolErosionMode = ELandscapeToolErosionMode(0);
    pub const RAISE: ELandscapeToolErosionMode = ELandscapeToolErosionMode(1);
    pub const LOWER: ELandscapeToolErosionMode = ELandscapeToolErosionMode(2);
}
#[repr(transparent)]
pub struct ELandscapeToolHydroErosionMode(pub i8);
impl ELandscapeToolHydroErosionMode {
    pub const INVALID: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(
        -1,
    );
    pub const BOTH: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(0);
    pub const POSITIVE: ELandscapeToolHydroErosionMode = ELandscapeToolHydroErosionMode(
        1,
    );
}
#[repr(transparent)]
pub struct ELandscapeToolNoiseMode(pub i8);
impl ELandscapeToolNoiseMode {
    pub const INVALID: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(-1);
    pub const BOTH: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(0);
    pub const ADD: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(1);
    pub const SUB: ELandscapeToolNoiseMode = ELandscapeToolNoiseMode(2);
}
#[repr(transparent)]
pub struct ELandscapeToolPasteMode(pub i8);
impl ELandscapeToolPasteMode {
    pub const INVALID: ELandscapeToolPasteMode = ELandscapeToolPasteMode(-1);
    pub const BOTH: ELandscapeToolPasteMode = ELandscapeToolPasteMode(0);
    pub const RAISE: ELandscapeToolPasteMode = ELandscapeToolPasteMode(1);
    pub const LOWER: ELandscapeToolPasteMode = ELandscapeToolPasteMode(2);
}
#[repr(transparent)]
pub struct ELandscapeMirrorOperation(pub u8);
impl ELandscapeMirrorOperation {
    pub const MINUS_X_TO_PLUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        0,
    );
    pub const PLUS_X_TO_MINUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        1,
    );
    pub const MINUS_Y_TO_PLUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        2,
    );
    pub const PLUS_Y_TO_MINUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        3,
    );
    pub const ROTATE_MINUS_X_TO_PLUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        4,
    );
    pub const ROTATE_PLUS_X_TO_MINUS_X: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        5,
    );
    pub const ROTATE_MINUS_Y_TO_PLUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        6,
    );
    pub const ROTATE_PLUS_Y_TO_MINUS_Y: ELandscapeMirrorOperation = ELandscapeMirrorOperation(
        7,
    );
}
#[repr(transparent)]
pub struct ELandscapeConvertMode(pub i8);
impl ELandscapeConvertMode {
    pub const INVALID: ELandscapeConvertMode = ELandscapeConvertMode(-1);
    pub const EXPAND: ELandscapeConvertMode = ELandscapeConvertMode(0);
    pub const CLIP: ELandscapeConvertMode = ELandscapeConvertMode(1);
    pub const RESAMPLE: ELandscapeConvertMode = ELandscapeConvertMode(2);
}
#[repr(transparent)]
pub struct ELandscapeImportTransformType(pub i8);
impl ELandscapeImportTransformType {
    pub const NONE: ELandscapeImportTransformType = ELandscapeImportTransformType(0);
    pub const EXPAND_OFFSET: ELandscapeImportTransformType = ELandscapeImportTransformType(
        1,
    );
    pub const EXPAND_CENTERED: ELandscapeImportTransformType = ELandscapeImportTransformType(
        2,
    );
    pub const RESAMPLE: ELandscapeImportTransformType = ELandscapeImportTransformType(3);
    pub const SUBREGION: ELandscapeImportTransformType = ELandscapeImportTransformType(
        4,
    );
}
#[repr(transparent)]
pub struct ELandscapeImportExportMode(pub i32);
impl ELandscapeImportExportMode {
    pub const LOADED_ONLY: ELandscapeImportExportMode = ELandscapeImportExportMode(0);
    pub const ALL: ELandscapeImportExportMode = ELandscapeImportExportMode(1);
}
#[repr(transparent)]
pub struct ELandscapeTextureColorChannel(pub i32);
impl ELandscapeTextureColorChannel {
    pub const RED: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(0);
    pub const GREEN: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(1);
    pub const BLUE: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(2);
    pub const ALPHA: ELandscapeTextureColorChannel = ELandscapeTextureColorChannel(3);
}
