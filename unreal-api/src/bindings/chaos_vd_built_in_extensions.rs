#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UChaosVDGTAccelerationStructuresDataComponent {
    __padding_end: [u8; 280],
}
impl UChaosVDGTAccelerationStructuresDataComponent {}
#[repr(C, align(8))]
pub struct UChaosVDAccelerationStructureVisualizationSettings {
    __padding_end: [u8; 112],
}
impl UChaosVDAccelerationStructureVisualizationSettings {}
#[repr(C, align(8))]
pub struct UChaosVDGenericDebugDrawDataComponent {
    __padding_end: [u8; 312],
}
impl UChaosVDGenericDebugDrawDataComponent {}
#[repr(C, align(8))]
pub struct UChaosVDGenericDebugDrawSettings {
    __padding_end: [u8; 112],
}
impl UChaosVDGenericDebugDrawSettings {}
