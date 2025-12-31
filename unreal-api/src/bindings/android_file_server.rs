#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAndroidFileServerBPLibrary {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAFSActiveType(pub u8);
impl EAFSActiveType {
    pub const NONE: EAFSActiveType = EAFSActiveType(0);
    pub const USB_ONLY: EAFSActiveType = EAFSActiveType(1);
    pub const NETWORK_ONLY: EAFSActiveType = EAFSActiveType(2);
    pub const COMBINED: EAFSActiveType = EAFSActiveType(3);
}
