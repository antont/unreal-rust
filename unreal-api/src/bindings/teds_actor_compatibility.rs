#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UActorLevelDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorLevelDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorIconOverrideDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorIconOverrideDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorLabelDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorLabelDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorMobilityDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorMobilityDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorParentDataStorageFactory {
    __padding_end: [u8; 64],
}
impl UActorParentDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorViewportDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorViewportDataStorageFactory {}
#[repr(C, align(8))]
pub struct UActorVisibilityDataStorageFactory {
    __padding_end: [u8; 48],
}
impl UActorVisibilityDataStorageFactory {}
#[repr(C, align(8))]
pub struct UTedsActorCompatibilityFactory {
    __padding_end: [u8; 168],
}
impl UTedsActorCompatibilityFactory {}
