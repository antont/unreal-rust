#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(1))]
pub struct FJsonStringifyOptions {
    pub flags: EJsonStringifyFlags,
}
impl FJsonStringifyOptions {}
#[repr(transparent)]
pub struct EJsonStringifyFlags(pub u8);
impl EJsonStringifyFlags {
    pub const DEFAULT: EJsonStringifyFlags = EJsonStringifyFlags(0);
    pub const FILTER_EDITOR_ONLY_DATA: EJsonStringifyFlags = EJsonStringifyFlags(1);
    pub const DISABLE_DELTA_ENCODING: EJsonStringifyFlags = EJsonStringifyFlags(2);
}
