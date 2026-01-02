#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAutomationTestPlatformSettings {
    __padding_end: [u8; 64],
}
impl UAutomationTestPlatformSettings {}
#[repr(C, align(8))]
pub struct UAutomationTestExcludelistSettings {
    __padding_end: [u8; 80],
}
impl UAutomationTestExcludelistSettings {}
#[repr(C, align(8))]
pub struct UAutomationTestExcludelistConfig {
    __padding_end: [u8; 168],
}
impl UAutomationTestExcludelistConfig {}
#[repr(C, align(8))]
pub struct UAutomationTestExcludelist {
    __padding_end: [u8; 216],
}
impl UAutomationTestExcludelist {}
#[repr(transparent)]
pub struct EAutomationState(pub u8);
impl EAutomationState {
    pub const NOT_RUN: EAutomationState = EAutomationState(0);
    pub const IN_PROCESS: EAutomationState = EAutomationState(1);
    pub const FAIL: EAutomationState = EAutomationState(2);
    pub const SUCCESS: EAutomationState = EAutomationState(3);
    pub const SKIPPED: EAutomationState = EAutomationState(4);
}
