#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UBasicOverlaysFactory {
    __padding_end: [u8; 136],
}
impl UBasicOverlaysFactory {}
#[repr(C, align(8))]
pub struct UBasicOverlaysFactoryNew {
    __padding_end: [u8; 136],
}
impl UBasicOverlaysFactoryNew {}
#[repr(C, align(8))]
pub struct ULocalizedOverlaysFactoryNew {
    __padding_end: [u8; 136],
}
impl ULocalizedOverlaysFactoryNew {}
#[repr(C, align(8))]
pub struct UReimportBasicOverlaysFactory {
    __padding_end: [u8; 168],
}
impl UReimportBasicOverlaysFactory {}
