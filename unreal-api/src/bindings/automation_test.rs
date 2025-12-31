#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAutomationState(pub u8);
impl EAutomationState {
    pub const NOT_RUN: EAutomationState = EAutomationState(0);
    pub const IN_PROCESS: EAutomationState = EAutomationState(1);
    pub const FAIL: EAutomationState = EAutomationState(2);
    pub const SUCCESS: EAutomationState = EAutomationState(3);
    pub const SKIPPED: EAutomationState = EAutomationState(4);
}
