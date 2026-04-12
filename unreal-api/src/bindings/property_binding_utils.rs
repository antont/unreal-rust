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
#[repr(C, align(2))]
pub struct FPropertyBindingIndex16 {
    pub(crate) __padding_end: [u8; 2],
}
impl FPropertyBindingIndex16 {}
pub struct IPropertyBindingBindingCollectionOwner {}
#[repr(C, align(8))]
pub struct UPropertyBindingBindingCollectionOwner {
    __padding_end: [u8; 48],
}
impl UPropertyBindingBindingCollectionOwner {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyBindingBindingCollectionOwner")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyBindingBindingCollectionOwner")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EPropertyBindingPropertyAccessType(pub u8);
impl EPropertyBindingPropertyAccessType {
    pub const OFFSET: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        0,
    );
    pub const OBJECT: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        1,
    );
    pub const WEAK_OBJECT: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        2,
    );
    pub const SOFT_OBJECT: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        3,
    );
    pub const OBJECT_INSTANCE: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        4,
    );
    pub const STRUCT_INSTANCE: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        5,
    );
    pub const INDEX_ARRAY: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        6,
    );
    pub const SHARED_STRUCT: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        7,
    );
    pub const STRUCT_INSTANCE_CONTAINER: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        8,
    );
    pub const UNSET: EPropertyBindingPropertyAccessType = EPropertyBindingPropertyAccessType(
        9,
    );
}
#[repr(transparent)]
pub struct EPropertyCopyType(pub u8);
impl EPropertyCopyType {
    pub const NONE: EPropertyCopyType = EPropertyCopyType(0);
    pub const COPY_PLAIN: EPropertyCopyType = EPropertyCopyType(1);
    pub const COPY_COMPLEX: EPropertyCopyType = EPropertyCopyType(2);
    pub const COPY_BOOL: EPropertyCopyType = EPropertyCopyType(3);
    pub const COPY_STRUCT: EPropertyCopyType = EPropertyCopyType(4);
    pub const COPY_OBJECT: EPropertyCopyType = EPropertyCopyType(5);
    pub const COPY_NAME: EPropertyCopyType = EPropertyCopyType(6);
    pub const COPY_FIXED_ARRAY: EPropertyCopyType = EPropertyCopyType(7);
    pub const STRUCT_REFERENCE: EPropertyCopyType = EPropertyCopyType(8);
    pub const PROMOTE_BOOL_TO_BYTE: EPropertyCopyType = EPropertyCopyType(9);
    pub const PROMOTE_BOOL_TO_INT32: EPropertyCopyType = EPropertyCopyType(10);
    pub const PROMOTE_BOOL_TO_U_INT32: EPropertyCopyType = EPropertyCopyType(11);
    pub const PROMOTE_BOOL_TO_INT64: EPropertyCopyType = EPropertyCopyType(12);
    pub const PROMOTE_BOOL_TO_FLOAT: EPropertyCopyType = EPropertyCopyType(13);
    pub const PROMOTE_BOOL_TO_DOUBLE: EPropertyCopyType = EPropertyCopyType(14);
    pub const PROMOTE_BYTE_TO_INT32: EPropertyCopyType = EPropertyCopyType(15);
    pub const PROMOTE_BYTE_TO_U_INT32: EPropertyCopyType = EPropertyCopyType(16);
    pub const PROMOTE_BYTE_TO_INT64: EPropertyCopyType = EPropertyCopyType(17);
    pub const PROMOTE_BYTE_TO_FLOAT: EPropertyCopyType = EPropertyCopyType(18);
    pub const PROMOTE_BYTE_TO_DOUBLE: EPropertyCopyType = EPropertyCopyType(19);
    pub const PROMOTE_INT32_TO_INT64: EPropertyCopyType = EPropertyCopyType(20);
    pub const PROMOTE_INT32_TO_FLOAT: EPropertyCopyType = EPropertyCopyType(21);
    pub const PROMOTE_INT32_TO_DOUBLE: EPropertyCopyType = EPropertyCopyType(22);
    pub const PROMOTE_U_INT32_TO_INT64: EPropertyCopyType = EPropertyCopyType(23);
    pub const PROMOTE_U_INT32_TO_FLOAT: EPropertyCopyType = EPropertyCopyType(24);
    pub const PROMOTE_U_INT32_TO_DOUBLE: EPropertyCopyType = EPropertyCopyType(25);
    pub const PROMOTE_FLOAT_TO_INT32: EPropertyCopyType = EPropertyCopyType(26);
    pub const PROMOTE_FLOAT_TO_INT64: EPropertyCopyType = EPropertyCopyType(27);
    pub const PROMOTE_FLOAT_TO_DOUBLE: EPropertyCopyType = EPropertyCopyType(28);
    pub const DEMOTE_DOUBLE_TO_INT32: EPropertyCopyType = EPropertyCopyType(29);
    pub const DEMOTE_DOUBLE_TO_INT64: EPropertyCopyType = EPropertyCopyType(30);
    pub const DEMOTE_DOUBLE_TO_FLOAT: EPropertyCopyType = EPropertyCopyType(31);
}
