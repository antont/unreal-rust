#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UMobileInstalledContent {
    __padding_end: [u8; 80],
}
impl UMobileInstalledContent {}
#[repr(C, align(8))]
pub struct UMobilePendingContent {
    __padding_end: [u8; 144],
}
impl UMobilePendingContent {}
#[repr(C, align(8))]
pub struct UMobilePatchingLibrary {
    __padding_end: [u8; 48],
}
impl UMobilePatchingLibrary {}
#[repr(transparent)]
pub struct FStartInstall_OnSucceeded {
    _opague: u8,
}
#[repr(transparent)]
pub struct FStartInstall_OnFailed {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRequestContent_OnSucceeded {
    _opague: u8,
}
#[repr(transparent)]
pub struct FRequestContent_OnFailed {
    _opague: u8,
}
