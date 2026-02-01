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
pub struct UWorldConditionSchema {
    __padding_end: [u8; 64],
}
impl UWorldConditionSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldConditionSchema")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldConditionSchema")
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
pub struct EWorldConditionOperator(pub u8);
impl EWorldConditionOperator {
    pub const AND: EWorldConditionOperator = EWorldConditionOperator(0);
    pub const OR: EWorldConditionOperator = EWorldConditionOperator(1);
    pub const COPY: EWorldConditionOperator = EWorldConditionOperator(2);
}
#[repr(transparent)]
pub struct EWorldConditionResultValue(pub u8);
impl EWorldConditionResultValue {
    pub const IS_FALSE: EWorldConditionResultValue = EWorldConditionResultValue(0);
    pub const IS_TRUE: EWorldConditionResultValue = EWorldConditionResultValue(1);
    pub const INVALID: EWorldConditionResultValue = EWorldConditionResultValue(2);
}
#[repr(transparent)]
pub struct EWorldConditionContextDataType(pub u8);
impl EWorldConditionContextDataType {
    pub const DYNAMIC: EWorldConditionContextDataType = EWorldConditionContextDataType(
        0,
    );
    pub const PERSISTENT: EWorldConditionContextDataType = EWorldConditionContextDataType(
        1,
    );
}
