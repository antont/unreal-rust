#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAutomationTestExcludeOptions {
    pub test: FName,
    pub reason: FName,
    pub rh_is: TSet<FName>,
    pub platforms: TSet<FName>,
    pub warn: bool,
}
#[repr(C, align(8))]
pub struct FAutomationTestExcludelistEntry {
    pub map: FName,
    pub test: FName,
    pub reason: FName,
    pub rh_is: TSet<FName>,
    pub warn: bool,
}
pub struct UAutomationTestPlatformSettings {}
pub struct UAutomationTestExcludelistSettings {
    pub supported_rh_is: TArray<FName>,
}
pub struct UAutomationTestExcludelistConfig {
    pub task_tracker_url_hashtag: FString,
    pub task_tracker_url_base: FString,
    pub exclude_test: TArray<FAutomationTestExcludelistEntry>,
}
pub struct UAutomationTestExcludelist {
    pub default_config: UPtr<UAutomationTestExcludelistConfig>,
    pub platform_configs: TMap<FName, UPtr<UAutomationTestExcludelistConfig>>,
}
