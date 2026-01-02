#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMeshApproximationSettingsObject {
    __padding_end: [u8; 352],
}
impl UMeshApproximationSettingsObject {}
#[repr(C, align(8))]
pub struct UMeshInstancingSettingsObject {
    __padding_end: [u8; 72],
}
impl UMeshInstancingSettingsObject {}
#[repr(C, align(8))]
pub struct UMeshMergingSettingsObject {
    __padding_end: [u8; 376],
}
impl UMeshMergingSettingsObject {}
#[repr(C, align(8))]
pub struct UMeshProxySettingsObject {
    __padding_end: [u8; 368],
}
impl UMeshProxySettingsObject {}
