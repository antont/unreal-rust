#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct ISkeletonCommitter {}
#[repr(C, align(8))]
pub struct USkeletonCommitter {
    __padding_end: [u8; 48],
}
impl USkeletonCommitter {}
pub struct ISkeletonProvider {}
#[repr(C, align(8))]
pub struct USkeletonProvider {
    __padding_end: [u8; 48],
}
impl USkeletonProvider {}
#[repr(C, align(8))]
pub struct UEditorModelingObjectsCreationAPI {
    __padding_end: [u8; 232],
}
impl UEditorModelingObjectsCreationAPI {}
#[repr(C, align(8))]
pub struct UDynamicMeshComponentToolTarget {
    __padding_end: [u8; 152],
}
impl UDynamicMeshComponentToolTarget {}
#[repr(C, align(8))]
pub struct UDynamicMeshComponentToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UDynamicMeshComponentToolTargetFactory {}
#[repr(C, align(8))]
pub struct USkeletalMeshComponentReadOnlyToolTarget {
    __padding_end: [u8; 112],
}
impl USkeletalMeshComponentReadOnlyToolTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshComponentToolTarget {
    __padding_end: [u8; 136],
}
impl USkeletalMeshComponentToolTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshComponentReadOnlyToolTargetFactory {
    __padding_end: [u8; 48],
}
impl USkeletalMeshComponentReadOnlyToolTargetFactory {}
#[repr(C, align(8))]
pub struct USkeletalMeshComponentToolTargetFactory {
    __padding_end: [u8; 56],
}
impl USkeletalMeshComponentToolTargetFactory {}
#[repr(C, align(8))]
pub struct USkeletalMeshReadOnlyToolTarget {
    __padding_end: [u8; 96],
}
impl USkeletalMeshReadOnlyToolTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshToolTarget {
    __padding_end: [u8; 112],
}
impl USkeletalMeshToolTarget {}
#[repr(C, align(8))]
pub struct USkeletalMeshReadOnlyToolTargetFactory {
    __padding_end: [u8; 48],
}
impl USkeletalMeshReadOnlyToolTargetFactory {}
#[repr(C, align(8))]
pub struct USkeletalMeshToolTargetFactory {
    __padding_end: [u8; 56],
}
impl USkeletalMeshToolTargetFactory {}
#[repr(C, align(8))]
pub struct UStaticMeshComponentReadOnlyToolTarget {
    __padding_end: [u8; 192],
}
impl UStaticMeshComponentReadOnlyToolTarget {}
#[repr(C, align(8))]
pub struct UStaticMeshComponentToolTarget {
    __padding_end: [u8; 208],
}
impl UStaticMeshComponentToolTarget {}
#[repr(C, align(8))]
pub struct UStaticMeshComponentToolTargetFactory {
    __padding_end: [u8; 56],
}
impl UStaticMeshComponentToolTargetFactory {}
#[repr(C, align(8))]
pub struct UStaticMeshReadOnlyToolTarget {
    __padding_end: [u8; 176],
}
impl UStaticMeshReadOnlyToolTarget {}
#[repr(C, align(8))]
pub struct UStaticMeshToolTarget {
    __padding_end: [u8; 192],
}
impl UStaticMeshToolTarget {}
#[repr(C, align(8))]
pub struct UStaticMeshToolTargetFactory {
    __padding_end: [u8; 56],
}
impl UStaticMeshToolTargetFactory {}
#[repr(C, align(8))]
pub struct UVolumeComponentToolTarget {
    __padding_end: [u8; 136],
}
impl UVolumeComponentToolTarget {}
#[repr(C, align(8))]
pub struct UVolumeComponentToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UVolumeComponentToolTargetFactory {}
#[repr(transparent)]
pub struct ESubdivisionScheme(pub u8);
impl ESubdivisionScheme {
    pub const BILINEAR: ESubdivisionScheme = ESubdivisionScheme(0);
    pub const CATMULL_CLARK: ESubdivisionScheme = ESubdivisionScheme(1);
    pub const LOOP: ESubdivisionScheme = ESubdivisionScheme(2);
}
#[repr(transparent)]
pub struct ESubdivisionBoundaryScheme(pub u8);
impl ESubdivisionBoundaryScheme {
    pub const SMOOTH_CORNERS: ESubdivisionBoundaryScheme = ESubdivisionBoundaryScheme(0);
    pub const SHARP_CORNERS: ESubdivisionBoundaryScheme = ESubdivisionBoundaryScheme(1);
}
#[repr(transparent)]
pub struct ESubdivisionOutputNormals(pub u8);
impl ESubdivisionOutputNormals {
    pub const INTERPOLATED: ESubdivisionOutputNormals = ESubdivisionOutputNormals(0);
    pub const GENERATED: ESubdivisionOutputNormals = ESubdivisionOutputNormals(1);
    pub const NONE: ESubdivisionOutputNormals = ESubdivisionOutputNormals(2);
}
#[repr(transparent)]
pub struct ESubdivisionOutputUVs(pub u8);
impl ESubdivisionOutputUVs {
    pub const INTERPOLATED: ESubdivisionOutputUVs = ESubdivisionOutputUVs(0);
    pub const NONE: ESubdivisionOutputUVs = ESubdivisionOutputUVs(1);
}
