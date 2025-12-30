#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPropertyPathSegment {
    pub name: FName,
    pub array_index: i32,
    pub _struct: UPtr<UStruct>,
}
#[repr(C, align(8))]
pub struct FCachedPropertyPath {
    pub segments: TArray<FPropertyPathSegment>,
    pub cached_function: UPtr<UFunction>,
}
#[repr(C, align(8))]
pub struct FPropertyPathTestBaseStruct {}
#[repr(C, align(8))]
pub struct FPropertyPathTestInnerStruct {
    pub float: f32,
    pub bool: bool,
    pub enum_one: EPropertyPathTestEnum,
    pub enum_two: EPropertyPathTestEnum,
    pub enum_three: EPropertyPathTestEnum,
    pub enum_four: EPropertyPathTestEnum,
    pub integer: i32,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FPropertyPathTestStruct {
    pub bool: bool,
    pub integer: i32,
    pub enum_one: EPropertyPathTestEnum,
    pub enum_two: EPropertyPathTestEnum,
    pub enum_three: EPropertyPathTestEnum,
    pub enum_four: EPropertyPathTestEnum,
    pub string: FString,
    pub float: f32,
    pub inner_struct: FPropertyPathTestInnerStruct,
    pub inner_object: UPtr<UPropertyPathTestObject>,
}
#[repr(C, align(8))]
pub struct FPropertyPathTestBed {
    pub object: UPtr<UPropertyPathTestObject>,
    pub modified_object: UPtr<UPropertyPathTestObject>,
    pub modified_struct: FPropertyPathTestStruct,
    pub default_struct: FPropertyPathTestStruct,
}
pub struct UPropertyPathTestObject {
    pub bool: bool,
    pub enum_one: EPropertyPathTestEnum,
    pub enum_two: EPropertyPathTestEnum,
    pub enum_three: EPropertyPathTestEnum,
    pub enum_four: EPropertyPathTestEnum,
    pub integer: i32,
    pub string: FString,
    pub float: f32,
    pub _struct: FPropertyPathTestStruct,
    pub struct_ref: FPropertyPathTestStruct,
    pub struct_const_ref: FPropertyPathTestStruct,
    pub inner_object: UPtr<UPropertyPathTestObject>,
}
