#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UPIEPreviewDeviceSpecification {
    __padding_end: [u8; 208],
}
impl UPIEPreviewDeviceSpecification {}
#[repr(transparent)]
pub struct EPIEPreviewDeviceType(pub u8);
impl EPIEPreviewDeviceType {
    pub const UNSET: EPIEPreviewDeviceType = EPIEPreviewDeviceType(0);
    pub const ANDROID: EPIEPreviewDeviceType = EPIEPreviewDeviceType(1);
    pub const IOS: EPIEPreviewDeviceType = EPIEPreviewDeviceType(2);
    pub const TVOS: EPIEPreviewDeviceType = EPIEPreviewDeviceType(3);
    pub const SWITCH: EPIEPreviewDeviceType = EPIEPreviewDeviceType(4);
    pub const MAX: EPIEPreviewDeviceType = EPIEPreviewDeviceType(5);
}
