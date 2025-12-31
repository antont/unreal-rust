#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub name: FString,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub vertex_weights: TArray<f32>,
    pub target_group: FScalarVertexPropertyGroup,
    pub override_type: EDataflowWeightMapOverrideType,
}
#[repr(C, align(8))]
pub struct FMakeAttributeKeyDataflowNode {
    pub group_in: FString,
    pub attribute_in: FString,
    pub attribute_key_out: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBreakAttributeKeyDataflowNode {
    pub attribute_key_in: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub attribute_out: FString,
    pub group_out: FString,
}
#[repr(C, align(8))]
pub struct FDataflowCollectionEditSkeletonBonesNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub tool_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub skeletal_meshes: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
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
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bone_indices_name: FString,
    pub bone_weights_name: FString,
    pub vertex_group: FScalarVertexPropertyGroup,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_compress_skin_weights: bool,
    pub skin_weights_deprecated: TArray<FDataflowSkinWeightData>,
    pub vertex_skin_weights: FDataflowVertexSkinWeightData,
    pub skin_weights_ref_skeleton_hash: i32,
    pub skeletal_meshes: TArray<UPtr<crate::bindings::engine::USkeletalMesh>>,
    pub dynamic_meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FDataflowCollectionSetSkinningSkeletalMesh {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub geometry_selection: crate::bindings::dataflow_core::FDataflowGeometrySelection,
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
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FGetSkeletonDataflowNode {
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshBoneDataflowNode {
    pub bone_name: FName,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub bone_index_out: i32,
    pub property_name: FName,
}
#[repr(C, align(16))]
pub struct FSkeletalMeshReferenceTransformDataflowNode {
    pub skeletal_mesh_in: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub bone_index_in: i32,
    pub transform_out: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FGetPhysicsAssetFromSkeletalMeshDataflowNode {
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
}
#[repr(C, align(8))]
pub struct FSkeletonAssetTerminalNode {
    pub source_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub skeleton_asset: UPtr<crate::bindings::engine::USkeleton>,
}
#[repr(C, align(8))]
pub struct FGetStaticMeshDataflowNode {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub property_name: FName,
}
#[repr(C, align(8))]
pub struct FGetStaticMeshBoundingBoxDataflowNode {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub dimensions: crate::bindings::core_u_object::FVector,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowWeightMapOverrideType(pub u8);
impl EDataflowWeightMapOverrideType {
    pub const REPLACE_ALL: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        0,
    );
    pub const ADD_DIFFERENCE: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        1,
    );
    pub const REPLACE_CHANGED: EDataflowWeightMapOverrideType = EDataflowWeightMapOverrideType(
        2,
    );
}
