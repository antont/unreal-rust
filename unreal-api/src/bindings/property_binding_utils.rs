#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPropertyBindingBindableStructDescriptor {
    pub _struct: UPtr<crate::bindings::core_u_object::UStruct>,
    pub name: FName,
    pub id: crate::bindings::core_u_object::FGuid,
    pub category: FString,
}
#[repr(C, align(8))]
pub struct FPropertyBindingBinding {
    pub source_property_path: FPropertyBindingPath,
    pub target_property_path: FPropertyBindingPath,
    pub property_function_node: crate::bindings::core_u_object::FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FPropertyBindingPath {
    pub struct_id: crate::bindings::core_u_object::FGuid,
    pub segments: TArray<FPropertyBindingPathSegment>,
}
#[repr(C, align(8))]
pub struct FPropertyBindingPathSegment {
    pub name: FName,
    pub array_index: i32,
    pub instance_struct: UPtr<crate::bindings::core_u_object::UStruct>,
    pub instanced_struct_access_type: EPropertyBindingPropertyAccessType,
    pub property_guid: crate::bindings::core_u_object::FGuid,
}
#[repr(C, align(16))]
pub struct FPropertyBindingBindingCollection {
    pub copy_batches: TArray<FPropertyBindingCopyInfoBatch>,
    pub property_copies: TArray<FPropertyBindingCopyInfo>,
    pub property_indirections: TArray<FPropertyBindingPropertyIndirection>,
    pub bindings_owner: TScriptInterface<IPropertyBindingBindingCollectionOwner>,
}
#[repr(C, align(8))]
pub struct FPropertyBindingPropertyIndirection {
    pub array_index: FPropertyBindingIndex16,
    pub offset: u16,
    pub next_index: FPropertyBindingIndex16,
    pub ty: EPropertyBindingPropertyAccessType,
    pub instance_struct: UPtr<crate::bindings::core_u_object::UStruct>,
}
#[repr(C, align(2))]
pub struct FPropertyBindingIndex16 {
    pub value: u16,
}
#[repr(C, align(8))]
pub struct FPropertyBindingCopyInfo {
    pub source_indirection: FPropertyBindingPropertyIndirection,
    pub target_indirection: FPropertyBindingPropertyIndirection,
    pub source_data_handle: crate::bindings::core_u_object::FInstancedStruct,
    pub source_struct_type: UPtr<crate::bindings::core_u_object::UStruct>,
    pub copy_size: i32,
    pub ty: EPropertyCopyType,
    pub b_copy_from_target_to_source: bool,
}
#[repr(C, align(8))]
pub struct FPropertyBindingCopyInfoBatch {
    pub target_struct: crate::bindings::core_u_object::FInstancedStruct,
    pub bindings_begin: FPropertyBindingIndex16,
    pub bindings_end: FPropertyBindingIndex16,
    pub property_functions_begin: FPropertyBindingIndex16,
    pub property_functions_end: FPropertyBindingIndex16,
}
pub struct UPropertyBindingBindingCollectionOwner {}
pub struct IPropertyBindingBindingCollectionOwner {}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
