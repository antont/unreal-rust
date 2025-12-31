#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDirectPathObjectLocator {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
}
#[repr(C, align(8))]
pub struct FSubObjectLocator {
    pub path_within_context: FString,
}
#[repr(C, align(8))]
pub struct FUniversalObjectLocator {
    pub fragments: TArray<FUniversalObjectLocatorFragment>,
}
#[repr(C, align(8))]
pub struct FUniversalObjectLocatorFragment {}
#[repr(C, align(1))]
pub struct FUniversalObjectLocatorEmptyPayload {}
#[allow(non_camel_case_types)]
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
