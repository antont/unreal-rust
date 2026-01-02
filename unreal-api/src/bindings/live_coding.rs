#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct ULiveCodingSettings {
    __padding_end: [u8; 72],
}
impl ULiveCodingSettings {}
#[repr(transparent)]
pub struct ELiveCodingStartupMode(pub u8);
impl ELiveCodingStartupMode {
    pub const AUTOMATIC: ELiveCodingStartupMode = ELiveCodingStartupMode(0);
    pub const AUTOMATIC_BUT_HIDDEN: ELiveCodingStartupMode = ELiveCodingStartupMode(1);
    pub const MANUAL: ELiveCodingStartupMode = ELiveCodingStartupMode(2);
}
