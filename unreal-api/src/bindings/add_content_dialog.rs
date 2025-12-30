#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
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
