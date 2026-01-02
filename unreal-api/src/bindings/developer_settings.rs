#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UDeveloperSettings {
    __padding_end: [u8; 104],
}
impl UDeveloperSettings {}
#[repr(C, align(8))]
pub struct UDeveloperSettingsBackedByCVars {
    __padding_end: [u8; 104],
}
impl UDeveloperSettingsBackedByCVars {}
#[repr(C, align(8))]
pub struct UPlatformSettings {
    __padding_end: [u8; 80],
}
impl UPlatformSettings {}
#[repr(C, align(8))]
pub struct UPlatformSettingsManager {
    __padding_end: [u8; 144],
}
impl UPlatformSettingsManager {}
