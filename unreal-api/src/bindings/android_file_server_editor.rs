#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAndroidFileServerRuntimeSettings {
    __padding_end: [u8; 96],
}
impl UAndroidFileServerRuntimeSettings {}
#[repr(transparent)]
pub struct EAFSConnectionType(pub u8);
impl EAFSConnectionType {
    pub const USB_ONLY: EAFSConnectionType = EAFSConnectionType(0);
    pub const NETWORK_ONLY: EAFSConnectionType = EAFSConnectionType(1);
    pub const COMBINED: EAFSConnectionType = EAFSConnectionType(2);
}
