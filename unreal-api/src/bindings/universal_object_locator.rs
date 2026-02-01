#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FUniversalObjectLocator {
    pub(crate) __padding_end: [u8; 16],
}
impl FUniversalObjectLocator {}
#[repr(C, align(8))]
pub struct FUniversalObjectLocatorFragment {
    pub(crate) __padding_end: [u8; 64],
}
impl FUniversalObjectLocatorFragment {}
#[repr(transparent)]
pub struct ELocatorResolveFlags(pub u8);
impl ELocatorResolveFlags {
    pub const NONE: ELocatorResolveFlags = ELocatorResolveFlags(0);
    pub const LOAD: ELocatorResolveFlags = ELocatorResolveFlags(1);
    pub const UNLOAD: ELocatorResolveFlags = ELocatorResolveFlags(2);
    pub const ASYNC: ELocatorResolveFlags = ELocatorResolveFlags(4);
    pub const WILL_WAIT: ELocatorResolveFlags = ELocatorResolveFlags(8);
    pub const ASYNC_WAIT: ELocatorResolveFlags = ELocatorResolveFlags(12);
}
