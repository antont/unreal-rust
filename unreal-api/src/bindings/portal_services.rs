#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPortalUserDetails {
    pub display_name: FText,
    pub email: FText,
    pub real_name: FText,
    pub is_signed_in: bool,
}
#[repr(C, align(8))]
pub struct FPortalUserIsEntitledToItemResult {
    pub item_id: FString,
    pub is_entitled: bool,
    pub retrieved_from_cache_level: EEntitlementCacheLevelRetrieved,
}
