#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FJsonStringifyOptions {
    pub flags: EJsonStringifyFlags,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EJsonStringifyFlags(pub u8);
impl EJsonStringifyFlags {
    pub const DEFAULT: EJsonStringifyFlags = EJsonStringifyFlags(0);
    pub const FILTER_EDITOR_ONLY_DATA: EJsonStringifyFlags = EJsonStringifyFlags(1);
    pub const DISABLE_DELTA_ENCODING: EJsonStringifyFlags = EJsonStringifyFlags(2);
}
