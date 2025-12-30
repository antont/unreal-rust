#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FSvgDistanceFieldConfiguration {
    pub distance_field_type: ESvgDistanceFieldType,
    pub base_distance_spread: f32,
    pub extra_outer_distance_spread: f32,
    pub extra_inner_distance_spread: f32,
    pub distance_spread_units: ESvgDistanceFieldUnits,
    pub output_width: i32,
    pub output_height: i32,
    pub scale_mode: ESvgDistanceFieldScaleMode,
    pub scale: f32,
    pub placement_mode: ESvgDistanceFieldPlacementMode,
    pub miter_limit: f32,
}
pub struct USvgDistanceFieldGenerator {}
