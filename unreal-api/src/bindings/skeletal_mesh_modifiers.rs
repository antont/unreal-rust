#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(16))]
pub struct FBoneClipboardData {
    pub bone_name: FName,
    pub global: FTransform,
    pub parent_index: i32,
}
#[repr(C, align(8))]
pub struct FHierarchyClipboardData {
    pub bones: TArray<FBoneClipboardData>,
}
#[repr(C, align(8))]
pub struct FMirrorOptions {
    pub mirror_axis: EAxis,
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
    pub secondary_target: FVector,
    pub b_orient_children: bool,
}
pub struct USkeletonModifier {
    pub skeletal_mesh: TWeakObjectPtr<USkeletalMesh>,
    pub weak_dynamic_mesh: TWeakObjectPtr<UDynamicMesh>,
}
pub struct USkeletalMeshMergeOptions {
    pub merge_type: ESKeletalMeshMergeType,
    pub b_merge_all: bool,
}
pub struct USkinWeightModifier {
    pub mesh: UPtr<USkeletalMesh>,
}
