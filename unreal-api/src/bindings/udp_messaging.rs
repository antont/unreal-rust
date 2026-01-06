#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(8))]
pub struct UUdpMessagingSettings {
    __padding_end: [u8; 208],
}
impl UUdpMessagingSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUdpMessagingSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EUdpMessageFormat(pub u8);
impl EUdpMessageFormat {
    pub const NONE: EUdpMessageFormat = EUdpMessageFormat(0);
    pub const JSON: EUdpMessageFormat = EUdpMessageFormat(1);
    pub const TAGGED_PROPERTY: EUdpMessageFormat = EUdpMessageFormat(2);
    pub const CBOR_PLATFORM_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(3);
    pub const CBOR_STANDARD_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(4);
}
