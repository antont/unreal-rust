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
pub struct FunctionPtrs {}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {}
    }
}
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UParameterizeMeshToolProperties {
    __padding_end: [u8; 192],
}
impl UParameterizeMeshToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolProperties")
            .copied()
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
pub struct UParameterizeMeshToolUVAtlasProperties {
    __padding_end: [u8; 200],
}
impl UParameterizeMeshToolUVAtlasProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolUVAtlasProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolUVAtlasProperties")
            .copied()
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
pub struct UParameterizeMeshToolXAtlasProperties {
    __padding_end: [u8; 192],
}
impl UParameterizeMeshToolXAtlasProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolXAtlasProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolXAtlasProperties")
            .copied()
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
pub struct UParameterizeMeshToolPatchBuilderProperties {
    __padding_end: [u8; 224],
}
impl UParameterizeMeshToolPatchBuilderProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolPatchBuilderProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolPatchBuilderProperties")
            .copied()
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
#[repr(C, align(16))]
pub struct UParameterizeMeshOperatorFactory {
    __padding_end: [u8; 304],
}
impl UParameterizeMeshOperatorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshOperatorFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshOperatorFactory")
            .copied()
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
pub struct EParameterizeMeshUVMethod(pub i32);
impl EParameterizeMeshUVMethod {
    pub const PATCH_BUILDER: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(0);
    pub const UV_ATLAS: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(1);
    pub const X_ATLAS: EParameterizeMeshUVMethod = EParameterizeMeshUVMethod(2);
}
#[repr(transparent)]
pub struct EEmbeddedPolygonOpMethod(pub u8);
impl EEmbeddedPolygonOpMethod {
    pub const TRIM_OUTSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(0);
    pub const TRIM_INSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(1);
    pub const INSERT_POLYGON: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(2);
    pub const CUT_THROUGH: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(3);
    pub const CUT_OUTSIDE: EEmbeddedPolygonOpMethod = EEmbeddedPolygonOpMethod(4);
}
#[repr(transparent)]
pub struct ESimplifyType(pub u8);
impl ESimplifyType {
    pub const QEM: ESimplifyType = ESimplifyType(0);
    pub const ATTRIBUTE: ESimplifyType = ESimplifyType(1);
    pub const UE_STANDARD: ESimplifyType = ESimplifyType(2);
    pub const MINIMAL_EXISTING_VERTEX: ESimplifyType = ESimplifyType(3);
    pub const MINIMAL_PLANAR: ESimplifyType = ESimplifyType(4);
    pub const MINIMAL_POLYGROUP: ESimplifyType = ESimplifyType(5);
    pub const CLUSTER_BASED: ESimplifyType = ESimplifyType(6);
}
#[repr(transparent)]
pub struct ESimplifyTargetType(pub u8);
impl ESimplifyTargetType {
    pub const PERCENTAGE: ESimplifyTargetType = ESimplifyTargetType(0);
    pub const TRIANGLE_COUNT: ESimplifyTargetType = ESimplifyTargetType(1);
    pub const VERTEX_COUNT: ESimplifyTargetType = ESimplifyTargetType(2);
    pub const EDGE_LENGTH: ESimplifyTargetType = ESimplifyTargetType(3);
    pub const MINIMAL_PLANAR: ESimplifyTargetType = ESimplifyTargetType(4);
}
#[repr(transparent)]
pub struct EMeshTangentsType(pub u8);
impl EMeshTangentsType {
    pub const MIKK_T_SPACE: EMeshTangentsType = EMeshTangentsType(0);
    pub const FAST_MIKK_T_SPACE: EMeshTangentsType = EMeshTangentsType(1);
    pub const PER_TRIANGLE: EMeshTangentsType = EMeshTangentsType(2);
    pub const COPY_EXISTING: EMeshTangentsType = EMeshTangentsType(3);
}
