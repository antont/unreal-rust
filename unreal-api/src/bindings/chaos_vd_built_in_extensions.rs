#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FChaosVDAABBTreeSelectionContext {}
pub struct UChaosVDGTAccelerationStructuresDataComponent {}
pub struct UChaosVDAccelerationStructureVisualizationSettings {
    pub depth_priority: ESceneDepthPriorityGroup,
    pub base_thickness: f32,
    pub acceleration_structure_data_visualization_flags: u32,
}
pub struct UChaosVDGenericDebugDrawDataComponent {}
pub struct UChaosVDGenericDebugDrawSettings {
    pub b_show_debug_text: bool,
    pub depth_priority: ESceneDepthPriorityGroup,
    pub base_thickness: f32,
    pub debug_draw_flags: u32,
}
