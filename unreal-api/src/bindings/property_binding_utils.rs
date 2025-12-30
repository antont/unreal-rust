#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPropertyBindingBindableStructDescriptor {
    pub _struct: UPtr<UStruct>,
    pub name: FName,
    pub id: FGuid,
    pub category: FString,
}
#[repr(C, align(8))]
pub struct FPropertyBindingBinding {
    pub source_property_path: FPropertyBindingPath,
    pub target_property_path: FPropertyBindingPath,
    pub property_function_node: FInstancedStruct,
}
#[repr(C, align(8))]
pub struct FPropertyBindingPath {
    pub struct_id: FGuid,
    pub segments: TArray<FPropertyBindingPathSegment>,
}
#[repr(C, align(8))]
pub struct FPropertyBindingPathSegment {
    pub name: FName,
    pub array_index: i32,
    pub instance_struct: UPtr<UStruct>,
    pub instanced_struct_access_type: EPropertyBindingPropertyAccessType,
    pub property_guid: FGuid,
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
    pub instance_struct: UPtr<UStruct>,
}
#[repr(C, align(2))]
pub struct FPropertyBindingIndex16 {
    pub value: u16,
}
#[repr(C, align(8))]
pub struct FPropertyBindingCopyInfo {
    pub source_indirection: FPropertyBindingPropertyIndirection,
    pub target_indirection: FPropertyBindingPropertyIndirection,
    pub source_data_handle: FInstancedStruct,
    pub source_struct_type: UPtr<UStruct>,
    pub copy_size: i32,
    pub ty: EPropertyCopyType,
    pub b_copy_from_target_to_source: bool,
}
#[repr(C, align(8))]
pub struct FPropertyBindingCopyInfoBatch {
    pub target_struct: FInstancedStruct,
    pub bindings_begin: FPropertyBindingIndex16,
    pub bindings_end: FPropertyBindingIndex16,
    pub property_functions_begin: FPropertyBindingIndex16,
    pub property_functions_end: FPropertyBindingIndex16,
}
pub struct UPropertyBindingBindingCollectionOwner {}
pub struct IPropertyBindingBindingCollectionOwner {}
