#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
impl FSvgDistanceFieldConfiguration {}
#[repr(C, align(8))]
pub struct USvgDistanceFieldGenerator {
    __padding_end: [u8; 48],
}
impl USvgDistanceFieldGenerator {}
#[repr(transparent)]
pub struct ESvgDistanceFieldType(pub u8);
impl ESvgDistanceFieldType {
    pub const SIMPLE: ESvgDistanceFieldType = ESvgDistanceFieldType(0);
    pub const PERPENDICULAR: ESvgDistanceFieldType = ESvgDistanceFieldType(1);
    pub const MULTI_CHANNEL_AND_SIMPLE: ESvgDistanceFieldType = ESvgDistanceFieldType(2);
}
#[repr(transparent)]
pub struct ESvgDistanceFieldUnits(pub u8);
impl ESvgDistanceFieldUnits {
    pub const SVG_UNITS: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(0);
    pub const OUTPUT_PIXELS: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(1);
    pub const PROPORTIONAL_TO_MAX_DIMENSION: ESvgDistanceFieldUnits = ESvgDistanceFieldUnits(
        2,
    );
}
#[repr(transparent)]
pub struct ESvgDistanceFieldScaleMode(pub u8);
impl ESvgDistanceFieldScaleMode {
    pub const EXPLICIT_SCALE: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(0);
    pub const FIT_CANVAS: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(1);
    pub const FIT_PADDED_CANVAS: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(
        2,
    );
    pub const FIT_BOUNDING_BOX: ESvgDistanceFieldScaleMode = ESvgDistanceFieldScaleMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESvgDistanceFieldPlacementMode(pub u8);
impl ESvgDistanceFieldPlacementMode {
    pub const DO_NOT_TRANSLATE: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        0,
    );
    pub const PAD_WITH_OUTER_SPREAD: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        1,
    );
    pub const CENTER_CANVAS: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        2,
    );
    pub const CENTER_BOUNDING_BOX: ESvgDistanceFieldPlacementMode = ESvgDistanceFieldPlacementMode(
        3,
    );
}
