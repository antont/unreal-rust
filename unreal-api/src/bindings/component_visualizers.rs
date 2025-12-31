#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct AManipulator {
    pub associated_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
}
pub struct USplineComponentVisualizerSelectionState {
    pub spline_property_path: crate::bindings::unreal_ed::FComponentPropertyPath,
    pub selected_keys: TSet<i32>,
    pub last_key_index_selected: i32,
    pub selected_segment_index: i32,
    pub selected_tangent_handle: i32,
    pub selected_tangent_handle_type: ESelectedTangentHandle,
    pub selected_attribute_index: i32,
    pub selected_attribute_name: FName,
    pub selected_spline_position: crate::bindings::core_u_object::FVector,
    pub cached_rotation: crate::bindings::core_u_object::FQuat,
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESelectedTangentHandle(pub i32);
impl ESelectedTangentHandle {
    pub const NONE: ESelectedTangentHandle = ESelectedTangentHandle(0);
    pub const LEAVE: ESelectedTangentHandle = ESelectedTangentHandle(1);
    pub const ARRIVE: ESelectedTangentHandle = ESelectedTangentHandle(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EShapeAddMode(pub u8);
impl EShapeAddMode {
    pub const APPEND_AFTER: EShapeAddMode = EShapeAddMode(1);
    pub const APPEND_BEFORE: EShapeAddMode = EShapeAddMode(2);
    pub const INSERT_AFTER: EShapeAddMode = EShapeAddMode(4);
    pub const INSERT_BEFORE: EShapeAddMode = EShapeAddMode(8);
}
