#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FProcMeshTangent {
    pub tangent_x: crate::bindings::core_u_object::FVector,
    pub b_flip_tangent_y: bool,
    __padding_end: [u8; 7],
}
impl FProcMeshTangent {}
#[repr(C, align(8))]
pub struct FProcMeshVertex {
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub tangent: FProcMeshTangent,
    pub color: crate::bindings::core_u_object::FColor,
    pub uv0: crate::bindings::core_u_object::FVector2D,
    pub uv1: crate::bindings::core_u_object::FVector2D,
    pub uv2: crate::bindings::core_u_object::FVector2D,
    pub uv3: crate::bindings::core_u_object::FVector2D,
}
impl FProcMeshVertex {}
#[repr(C, align(8))]
pub struct UKismetProceduralMeshLibrary {
    __padding_end: [u8; 48],
}
impl UKismetProceduralMeshLibrary {}
#[repr(C, align(16))]
pub struct UProceduralMeshComponent {
    #[doc(hidden)]
    __padding_1584: [u8; 1584],
    pub b_use_complex_as_simple_collision: bool,
    pub b_use_async_cooking: bool,
    __padding_end: [u8; 126],
}
impl UProceduralMeshComponent {}
#[repr(transparent)]
pub struct EProcMeshSliceCapOption(pub u8);
impl EProcMeshSliceCapOption {
    pub const NO_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(0);
    pub const CREATE_NEW_SECTION_FOR_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(
        1,
    );
    pub const USE_LAST_SECTION_FOR_CAP: EProcMeshSliceCapOption = EProcMeshSliceCapOption(
        2,
    );
}
