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
pub struct UCurveEditorFFTFilter {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub cutoff_frequency: f32,
    pub ty: ECurveEditorFFTFilterType,
    pub response: ECurveEditorFFTFilterClass,
    pub order: i32,
}
impl UCurveEditorFFTFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorFFTFilter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorFFTFilter")
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
pub struct UCurveEditorTools_LatticeUndoObject {
    __padding_end: [u8; 248],
}
impl UCurveEditorTools_LatticeUndoObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorTools_LatticeUndoObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorTools_LatticeUndoObject")
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
pub struct UCurveEditorRetimeToolData {
    __padding_end: [u8; 64],
}
impl UCurveEditorRetimeToolData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorRetimeToolData")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEditorRetimeToolData")
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
pub struct EMultiScalePivotType(pub u8);
impl EMultiScalePivotType {
    pub const AVERAGE: EMultiScalePivotType = EMultiScalePivotType(0);
    pub const BOUND_CENTER: EMultiScalePivotType = EMultiScalePivotType(1);
    pub const FIRST_KEY: EMultiScalePivotType = EMultiScalePivotType(2);
    pub const LAST_KEY: EMultiScalePivotType = EMultiScalePivotType(3);
}
#[repr(transparent)]
pub struct EToolTransformInterpType(pub u8);
impl EToolTransformInterpType {
    pub const LINEAR: EToolTransformInterpType = EToolTransformInterpType(0);
    pub const SINUSOIDAL: EToolTransformInterpType = EToolTransformInterpType(1);
    pub const CUBIC: EToolTransformInterpType = EToolTransformInterpType(2);
    pub const CIRCULAR_IN: EToolTransformInterpType = EToolTransformInterpType(3);
    pub const CIRCULAR_OUT: EToolTransformInterpType = EToolTransformInterpType(4);
    pub const EXP_IN: EToolTransformInterpType = EToolTransformInterpType(5);
    pub const EXP_OUT: EToolTransformInterpType = EToolTransformInterpType(6);
}
#[repr(transparent)]
pub struct ECurveEditorFFTFilterType(pub u8);
impl ECurveEditorFFTFilterType {
    pub const LOWPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(0);
    pub const HIGHPASS: ECurveEditorFFTFilterType = ECurveEditorFFTFilterType(1);
}
#[repr(transparent)]
pub struct ECurveEditorFFTFilterClass(pub u8);
impl ECurveEditorFFTFilterClass {
    pub const BUTTERWORTH: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(0);
    pub const CHEBYSHEV: ECurveEditorFFTFilterClass = ECurveEditorFFTFilterClass(1);
}
