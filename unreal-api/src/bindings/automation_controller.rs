#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UAutomationControllerSettings {
    __padding_end: [u8; 152],
}
impl UAutomationControllerSettings {}
#[repr(transparent)]
pub struct EAutomationArtifactType(pub u8);
impl EAutomationArtifactType {
    pub const NONE: EAutomationArtifactType = EAutomationArtifactType(0);
    pub const IMAGE: EAutomationArtifactType = EAutomationArtifactType(1);
    pub const COMPARISON: EAutomationArtifactType = EAutomationArtifactType(2);
}
