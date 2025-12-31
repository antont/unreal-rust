#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EEntitlementCacheLevelRetrieved(pub u8);
impl EEntitlementCacheLevelRetrieved {
    pub const NONE: EEntitlementCacheLevelRetrieved = EEntitlementCacheLevelRetrieved(0);
    pub const MEMORY: EEntitlementCacheLevelRetrieved = EEntitlementCacheLevelRetrieved(
        1,
    );
    pub const DISK: EEntitlementCacheLevelRetrieved = EEntitlementCacheLevelRetrieved(2);
}
