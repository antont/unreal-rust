#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FProcMeshTangent {
    pub tangent_x: FVector,
    pub b_flip_tangent_y: bool,
}
#[repr(C, align(8))]
pub struct FProcMeshVertex {
    pub position: FVector,
    pub normal: FVector,
    pub tangent: FProcMeshTangent,
    pub color: FColor,
    pub uv0: FVector2D,
    pub uv1: FVector2D,
    pub uv2: FVector2D,
    pub uv3: FVector2D,
}
#[repr(C, align(8))]
pub struct FProcMeshSection {
    pub proc_vertex_buffer: TArray<FProcMeshVertex>,
    pub proc_index_buffer: TArray<u32>,
    pub section_local_box: FBox,
    pub b_enable_collision: bool,
    pub b_section_visible: bool,
}
pub struct UKismetProceduralMeshLibrary {}
pub struct UProceduralMeshComponent {
    pub b_use_complex_as_simple_collision: bool,
    pub b_use_async_cooking: bool,
    pub proc_mesh_body_setup: UPtr<UBodySetup>,
    pub proc_mesh_sections: TArray<FProcMeshSection>,
    pub collision_convex_elems: TArray<FKConvexElem>,
    pub local_bounds: FBoxSphereBounds,
    pub async_body_setup_queue: TArray<UPtr<UBodySetup>>,
}
