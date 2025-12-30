#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FMetadataSet {
    pub strings: TMap<FName, FString>,
    pub bools: TMap<FName, bool>,
    pub ints: TMap<FName, i32>,
    pub floats: TMap<FName, f32>,
}
#[repr(C, align(8))]
pub struct FStructMetadata {
    pub fields: TMap<FName, FMetadataSet>,
    pub struct_metadata: FMetadataSet,
}
#[repr(C, align(8))]
pub struct FMetadataConfig {
    pub classes: TMap<FName, FStructMetadata>,
}
#[repr(C, align(4))]
pub struct FEditorConfigTestEnumStruct {
    pub before: i32,
    pub enum_: EEditorConfigTestEnum,
    pub after: i32,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestSimpleStruct {
    pub bool: bool,
    pub int: i32,
    pub string: FString,
    pub float: f32,
    pub array: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestKey {
    pub name: FString,
    pub number: f64,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestComplexArray {
    pub array: TArray<FEditorConfigTestKey>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestSimpleMap {
    pub map: TMap<FString, FString>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestSimpleKeyComplexValueMap {
    pub map: TMap<FString, FEditorConfigTestKey>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestComplexMap {
    pub map: TMap<FEditorConfigTestKey, FEditorConfigTestKey>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestSimpleSet {
    pub set: TSet<FName>,
}
#[repr(C, align(8))]
pub struct FEditorConfigTestComplexSet {
    pub set: TSet<FEditorConfigTestKey>,
}
pub struct UEditorConfigBase {}
pub struct UEditorConfigSubsystem {}
pub struct UEditorMetadataOverrides {}
pub struct UEditorConfigTestObject {
    pub object: UPtr<UObject>,
    pub _struct: FEditorConfigTestSimpleStruct,
    pub number: i32,
}
