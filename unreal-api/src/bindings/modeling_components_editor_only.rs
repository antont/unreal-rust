#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct USkeletonCommitter {}
pub struct ISkeletonCommitter {}
pub struct USkeletonProvider {}
pub struct ISkeletonProvider {}
pub struct UEditorModelingObjectsCreationAPI {}
pub struct UDynamicMeshComponentToolTarget {}
pub struct UDynamicMeshComponentToolTargetFactory {}
pub struct USkeletalMeshComponentReadOnlyToolTarget {}
pub struct USkeletalMeshComponentToolTarget {}
pub struct USkeletalMeshComponentReadOnlyToolTargetFactory {}
pub struct USkeletalMeshComponentToolTargetFactory {}
pub struct USkeletalMeshReadOnlyToolTarget {}
pub struct USkeletalMeshToolTarget {}
pub struct USkeletalMeshReadOnlyToolTargetFactory {}
pub struct USkeletalMeshToolTargetFactory {}
pub struct UStaticMeshComponentReadOnlyToolTarget {}
pub struct UStaticMeshComponentToolTarget {}
pub struct UStaticMeshComponentToolTargetFactory {}
pub struct UStaticMeshReadOnlyToolTarget {}
pub struct UStaticMeshToolTarget {}
pub struct UStaticMeshToolTargetFactory {}
pub struct UVolumeComponentToolTarget {}
pub struct UVolumeComponentToolTargetFactory {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubdivisionScheme(pub u8);
impl ESubdivisionScheme {
    pub const BILINEAR: ESubdivisionScheme = ESubdivisionScheme(0);
    pub const CATMULL_CLARK: ESubdivisionScheme = ESubdivisionScheme(1);
    pub const LOOP: ESubdivisionScheme = ESubdivisionScheme(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubdivisionBoundaryScheme(pub u8);
impl ESubdivisionBoundaryScheme {
    pub const SMOOTH_CORNERS: ESubdivisionBoundaryScheme = ESubdivisionBoundaryScheme(0);
    pub const SHARP_CORNERS: ESubdivisionBoundaryScheme = ESubdivisionBoundaryScheme(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubdivisionOutputNormals(pub u8);
impl ESubdivisionOutputNormals {
    pub const INTERPOLATED: ESubdivisionOutputNormals = ESubdivisionOutputNormals(0);
    pub const GENERATED: ESubdivisionOutputNormals = ESubdivisionOutputNormals(1);
    pub const NONE: ESubdivisionOutputNormals = ESubdivisionOutputNormals(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESubdivisionOutputUVs(pub u8);
impl ESubdivisionOutputUVs {
    pub const INTERPOLATED: ESubdivisionOutputUVs = ESubdivisionOutputUVs(0);
    pub const NONE: ESubdivisionOutputUVs = ESubdivisionOutputUVs(1);
}
