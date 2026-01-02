#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FDynamicMeshChangeInfo {
    pub ty: EDynamicMeshChangeType,
    pub flags: EDynamicMeshAttributeChangeFlags,
    pub b_is_revert_change: bool,
    __padding_end: [u8; 29],
}
impl FDynamicMeshChangeInfo {}
#[repr(C, align(8))]
pub struct UDynamicMeshProcessorBlueprint {
    __padding_end: [u8; 56],
}
impl UDynamicMeshProcessorBlueprint {}
pub struct UMeshCommandChangeTarget {}
pub struct IMeshCommandChangeTarget {}
pub struct UMeshReplacementCommandChangeTarget {}
pub struct IMeshReplacementCommandChangeTarget {}
pub struct UMeshVertexCommandChangeTarget {}
pub struct IMeshVertexCommandChangeTarget {}
#[repr(C, align(16))]
pub struct UBaseDynamicMeshComponent {
    #[doc(hidden)]
    __padding_1608: [u8; 1608],
    pub b_explicit_show_wireframe: bool,
    pub wireframe_color: crate::bindings::core_u_object::FLinearColor,
    pub color_mode: EDynamicMeshComponentColorOverrideMode,
    pub constant_color: crate::bindings::core_u_object::FColor,
    pub color_space_mode: EDynamicMeshVertexColorTransformMode,
    #[doc(hidden)]
    __padding_1638: [u8; 1],
    pub b_enable_flat_shading: bool,
    pub b_enable_view_mode_overrides: bool,
    #[doc(hidden)]
    __padding_1680: [u8; 40],
    pub b_enable_raytracing: bool,
    pub draw_path: EDynamicMeshDrawPath,
    __padding_end: [u8; 30],
}
impl UBaseDynamicMeshComponent {}
#[repr(C, align(16))]
pub struct UDynamicMeshComponent {
    #[doc(hidden)]
    __padding_2040: [u8; 2040],
    pub tangents_type: EDynamicMeshComponentTangentsMode,
    #[doc(hidden)]
    __padding_2272: [u8; 231],
    pub collision_type: crate::bindings::physics_core::ECollisionTraceFlag,
    pub b_use_async_cooking: bool,
    pub b_enable_complex_collision: bool,
    pub b_defer_collision_updates: bool,
    pub b_disable_mesh_uv_hit_results: bool,
    __padding_end: [u8; 251],
}
impl UDynamicMeshComponent {}
#[repr(C, align(8))]
pub struct ADynamicMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub b_enable_compute_mesh_pool: bool,
    __padding_end: [u8; 15],
}
impl ADynamicMeshActor {}
#[repr(C, align(8))]
pub struct UDynamicMeshGenerator {
    __padding_end: [u8; 48],
}
impl UDynamicMeshGenerator {}
#[repr(C, align(8))]
pub struct UDynamicMesh {
    __padding_end: [u8; 216],
}
impl UDynamicMesh {}
#[repr(C, align(8))]
pub struct UDynamicMeshPool {
    __padding_end: [u8; 120],
}
impl UDynamicMeshPool {}
#[repr(C, align(8))]
pub struct FDynamicMesh_MeshModifiedBPEvent {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EDynamicMeshChangeType(pub u8);
impl EDynamicMeshChangeType {
    pub const GENERAL_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(0);
    pub const MESH_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(1);
    pub const MESH_REPLACEMENT_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(
        2,
    );
    pub const MESH_VERTEX_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(3);
    pub const DEFORMATION_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(4);
    pub const ATTRIBUTE_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(5);
}
#[repr(transparent)]
pub struct EDynamicMeshAttributeChangeFlags(pub u8);
impl EDynamicMeshAttributeChangeFlags {
    pub const UNKNOWN: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        0,
    );
    pub const MESH_TOPOLOGY: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        1,
    );
    pub const VERTEX_POSITIONS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        2,
    );
    pub const NORMALS_TANGENTS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        4,
    );
    pub const VERTEX_COLORS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        8,
    );
    pub const U_VS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        16,
    );
    pub const TRIANGLE_GROUPS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        32,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentColorOverrideMode(pub u8);
impl EDynamicMeshComponentColorOverrideMode {
    pub const NONE: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        0,
    );
    pub const VERTEX_COLORS: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        1,
    );
    pub const POLYGROUPS: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        2,
    );
    pub const CONSTANT: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        3,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentDistanceFieldMode(pub u8);
impl EDynamicMeshComponentDistanceFieldMode {
    pub const NO_DISTANCE_FIELD: EDynamicMeshComponentDistanceFieldMode = EDynamicMeshComponentDistanceFieldMode(
        0,
    );
    pub const ASYNC_CPU_DISTANCE_FIELD: EDynamicMeshComponentDistanceFieldMode = EDynamicMeshComponentDistanceFieldMode(
        1,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshDrawPath(pub u8);
impl EDynamicMeshDrawPath {
    pub const DYNAMIC_DRAW: EDynamicMeshDrawPath = EDynamicMeshDrawPath(0);
    pub const STATIC_DRAW: EDynamicMeshDrawPath = EDynamicMeshDrawPath(1);
}
#[repr(transparent)]
pub struct EDynamicMeshVertexColorTransformMode(pub u8);
impl EDynamicMeshVertexColorTransformMode {
    pub const NO_TRANSFORM: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        0,
    );
    pub const LINEAR_TO_SRGB: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        1,
    );
    pub const SRGB_TO_LINEAR: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        2,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentTangentsMode(pub u8);
impl EDynamicMeshComponentTangentsMode {
    pub const NO_TANGENTS: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        0,
    );
    pub const AUTO_CALCULATED: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        1,
    );
    pub const EXTERNALLY_PROVIDED: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        2,
    );
    pub const DEFAULT: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        255,
    );
}
