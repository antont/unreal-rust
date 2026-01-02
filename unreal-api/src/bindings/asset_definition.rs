#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FRevisionInfo {
    pub revision: FString,
    pub changelist: i32,
    pub date: crate::bindings::core_u_object::FDateTime,
}
impl FRevisionInfo {}
#[repr(C, align(8))]
pub struct UAssetDefinition {
    __padding_end: [u8; 72],
}
impl UAssetDefinition {}
#[repr(C, align(8))]
pub struct UAssetDefinitionRegistry {
    __padding_end: [u8; 208],
}
impl UAssetDefinitionRegistry {}
