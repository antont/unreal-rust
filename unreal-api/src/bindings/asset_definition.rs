#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAssetFilterData {}
#[repr(C, align(8))]
pub struct FRevisionInfo {
    pub revision: FString,
    pub changelist: i32,
    pub date: FDateTime,
}
pub struct UAssetDefinition {}
pub struct UAssetDefinitionRegistry {
    pub asset_definitions: TMap<TSoftObjectPtr<UClass>, UPtr<UAssetDefinition>>,
}
