#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FBoneClipboardData {
    pub bone_name: FName,
    pub global: crate::bindings::core_u_object::FTransform,
    pub parent_index: i32,
}
#[repr(C, align(8))]
pub struct FHierarchyClipboardData {
    pub bones: TArray<FBoneClipboardData>,
}
#[repr(C, align(8))]
pub struct FMirrorOptions {
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub b_mirror_rotation: bool,
    pub left_string: FString,
    pub right_string: FString,
    pub b_mirror_children: bool,
}
#[repr(C, align(8))]
pub struct FOrientOptions {
    pub primary: EOrientAxis,
    pub secondary: EOrientAxis,
    pub b_use_plane_as_secondary: bool,
    pub secondary_target: crate::bindings::core_u_object::FVector,
    pub b_orient_children: bool,
}
pub struct USkeletonModifier {
    pub skeletal_mesh: TWeakObjectPtr<crate::bindings::engine::USkeletalMesh>,
    pub weak_dynamic_mesh: TWeakObjectPtr<
        crate::bindings::geometry_framework::UDynamicMesh,
    >,
}
pub struct USkeletalMeshMergeOptions {
    pub merge_type: ESKeletalMeshMergeType,
    pub b_merge_all: bool,
}
pub struct USkinWeightModifier {
    pub mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOrientAxis(pub u8);
impl EOrientAxis {
    pub const NONE: EOrientAxis = EOrientAxis(0);
    pub const POSITIVE_X: EOrientAxis = EOrientAxis(1);
    pub const POSITIVE_Y: EOrientAxis = EOrientAxis(2);
    pub const POSITIVE_Z: EOrientAxis = EOrientAxis(3);
    pub const NEGATIVE_X: EOrientAxis = EOrientAxis(4);
    pub const NEGATIVE_Y: EOrientAxis = EOrientAxis(5);
    pub const NEGATIVE_Z: EOrientAxis = EOrientAxis(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESKeletalMeshMergeType(pub u8);
impl ESKeletalMeshMergeType {
    pub const NEW: ESKeletalMeshMergeType = ESKeletalMeshMergeType(0);
    pub const MERGE: ESKeletalMeshMergeType = ESKeletalMeshMergeType(1);
}
