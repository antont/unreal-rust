#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FFeaturePackLevelSet {
    pub detail_levels: TArray<EFeaturePackDetailLevel>,
    pub mount_name: FString,
}
#[repr(C, align(8))]
pub struct FFeatureAdditionalFiles {
    pub destination_files_folder: FString,
    pub additional_files_list: TArray<FString>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFeaturePackDetailLevel(pub u8);
impl EFeaturePackDetailLevel {
    pub const STANDARD: EFeaturePackDetailLevel = EFeaturePackDetailLevel(0);
    pub const HIGH: EFeaturePackDetailLevel = EFeaturePackDetailLevel(1);
}
