#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FDataflowFunctionProperty {}
#[repr(C, align(8))]
pub struct FDataflowPrimitiveNode {}
#[repr(C, align(4))]
pub struct FScalarVertexPropertyGroup {
    pub name: FName,
}
#[repr(C, align(8))]
pub struct FDataflowVertexAttributeEditableNode {}
#[repr(C, align(8))]
pub struct FDataflowCollectionAddScalarVertexPropertyNode {
    pub collection: FManagedArrayCollection,
    pub name: FString,
    pub attribute_key: FCollectionAttributeKey,
    pub vertex_weights: TArray<f32>,
    pub target_group: FScalarVertexPropertyGroup,
    pub override_type: EDataflowWeightMapOverrideType,
}
#[repr(C, align(8))]
pub struct FMakeAttributeKeyDataflowNode {
    pub group_in: FString,
    pub attribute_in: FString,
    pub attribute_key_out: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBreakAttributeKeyDataflowNode {
    pub attribute_key_in: FCollectionAttributeKey,
    pub attribute_out: FString,
    pub group_out: FString,
}
#[repr(C, align(8))]
pub struct FDataflowCollectionEditSkeletonBonesNode {
    pub collection: FManagedArrayCollection,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub skeleton: UPtr<USkeleton>,
    pub tool_skeleton: UPtr<USkeleton>,
    pub skeletal_meshes: TArray<UPtr<USkeletalMesh>>,
}
#[repr(C, align(8))]
pub struct FDataflowSkinWeightData {
    pub bone_weights: TArray<f32>,
    pub bone_indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FDataflowVertexSkinWeightData {}
#[repr(C, align(8))]
pub struct FDataflowCollectionEditSkinWeightsNode {
    pub collection: FManagedArrayCollection,
    pub bone_indices_name: FString,
    pub bone_weights_name: FString,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub b_compress_skin_weights: bool,
    pub skin_weights_deprecated: TArray<FDataflowSkinWeightData>,
    pub vertex_skin_weights: FDataflowVertexSkinWeightData,
    pub skin_weights_ref_skeleton_hash: i32,
    pub skeletal_meshes: TArray<UPtr<USkeletalMesh>>,
    pub dynamic_meshes: TArray<UPtr<UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FDataflowCollectionSetSkinningSkeletalMesh {
    pub collection: FManagedArrayCollection,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub lod_index: i32,
    pub geometry_selection: FDataflowGeometrySelection,
}
#[repr(C, align(8))]
pub struct FFloatOverrideDataflowNode {
    pub property_name: FName,
    pub key_name: FName,
    pub value_out: f32,
}
#[repr(C, align(8))]
pub struct FSelectionSetDataflowNode {
    pub indices: FString,
    pub indices_out: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FGetSkeletalMeshDataflowNode {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FGetSkeletonDataflowNode {
    pub skeleton: UPtr<USkeleton>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshBoneDataflowNode {
    pub bone_name: FName,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub bone_index_out: i32,
    pub property_name: FName,
}
#[repr(C, align(16))]
pub struct FSkeletalMeshReferenceTransformDataflowNode {
    pub skeletal_mesh_in: UPtr<USkeletalMesh>,
    pub bone_index_in: i32,
    pub transform_out: FTransform,
}
#[repr(C, align(8))]
pub struct FGetPhysicsAssetFromSkeletalMeshDataflowNode {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub physics_asset: UPtr<UPhysicsAsset>,
}
#[repr(C, align(8))]
pub struct FSkeletonAssetTerminalNode {
    pub source_skeleton: UPtr<USkeleton>,
    pub skeleton_asset: UPtr<USkeleton>,
}
#[repr(C, align(8))]
pub struct FGetStaticMeshDataflowNode {
    pub static_mesh: UPtr<UStaticMesh>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FGetStaticMeshBoundingBoxDataflowNode {
    pub static_mesh: UPtr<UStaticMesh>,
    pub bounding_box: FBox,
    pub center: FVector,
    pub dimensions: FVector,
}
