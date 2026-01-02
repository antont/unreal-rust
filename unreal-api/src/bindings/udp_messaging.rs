#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UUdpMessagingSettings {
    __padding_end: [u8; 208],
}
impl UUdpMessagingSettings {}
#[repr(transparent)]
pub struct EUdpMessageFormat(pub u8);
impl EUdpMessageFormat {
    pub const NONE: EUdpMessageFormat = EUdpMessageFormat(0);
    pub const JSON: EUdpMessageFormat = EUdpMessageFormat(1);
    pub const TAGGED_PROPERTY: EUdpMessageFormat = EUdpMessageFormat(2);
    pub const CBOR_PLATFORM_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(3);
    pub const CBOR_STANDARD_ENDIANNESS: EUdpMessageFormat = EUdpMessageFormat(4);
}
