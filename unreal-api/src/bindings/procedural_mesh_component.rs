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
}
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
#[repr(C, align(8))]
pub struct FProcMeshSection {
    pub proc_vertex_buffer: TArray<FProcMeshVertex>,
    pub proc_index_buffer: TArray<u32>,
    pub section_local_box: crate::bindings::core_u_object::FBox,
    pub b_enable_collision: bool,
    pub b_section_visible: bool,
}
pub struct UKismetProceduralMeshLibrary {}
pub struct UProceduralMeshComponent {
    pub b_use_complex_as_simple_collision: bool,
    pub b_use_async_cooking: bool,
    pub proc_mesh_body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub proc_mesh_sections: TArray<FProcMeshSection>,
    pub collision_convex_elems: TArray<crate::bindings::engine::FKConvexElem>,
    pub local_bounds: crate::bindings::core_u_object::FBoxSphereBounds,
    pub async_body_setup_queue: TArray<UPtr<crate::bindings::engine::UBodySetup>>,
}
#[allow(non_camel_case_types)]
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
