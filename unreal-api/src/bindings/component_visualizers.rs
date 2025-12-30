#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct AManipulator {
    pub associated_component: UPtr<USceneComponent>,
    pub static_mesh_component: UPtr<UStaticMeshComponent>,
}
pub struct USplineComponentVisualizerSelectionState {
    pub spline_property_path: FComponentPropertyPath,
    pub selected_keys: TSet<i32>,
    pub last_key_index_selected: i32,
    pub selected_segment_index: i32,
    pub selected_tangent_handle: i32,
    pub selected_tangent_handle_type: ESelectedTangentHandle,
    pub selected_attribute_index: i32,
    pub selected_attribute_name: FName,
    pub selected_spline_position: FVector,
    pub cached_rotation: FQuat,
}
pub struct USplineGeneratorBase {
    pub shape_add_mode: EShapeAddMode,
}
pub struct UCircleSplineGenerator {
    pub number_of_points: i32,
    pub radius: f32,
    pub b_reverse_dir: bool,
    pub b_keep_first_key_tangent: bool,
    pub b_branch_right: bool,
}
pub struct UArcSplineGenerator {
    pub number_of_points: i32,
    pub radius: f32,
    pub degrees: f32,
    pub b_reverse_dir: bool,
    pub b_keep_first_key_tangent: bool,
    pub b_branch_right: bool,
}
pub struct USquareSplineGenerator {
    pub length: f32,
    pub b_branch_right: bool,
}
pub struct UEllipseSplineGenerator {
    pub number_of_points: i32,
    pub length: f32,
    pub width: f32,
    pub b_reverse_dir: bool,
    pub b_keep_first_key_tangent: bool,
    pub b_branch_right: bool,
}
pub struct URectangleSplineGenerator {
    pub length: f32,
    pub width: f32,
    pub b_branch_right: bool,
}
pub struct ULineSplineGenerator {
    pub number_of_points: i32,
    pub length: f64,
    pub b_enable_up_to_next_point: bool,
    pub b_up_to_next_point: bool,
}
