#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAttachGuidesRootsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_index: i32,
    pub kinematic_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FAttachCurveRootsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub kinematic_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildCurveWeightsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub curve_weights: crate::bindings::engine::FRuntimeFloatCurve,
    pub weights_attribute: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(16))]
pub struct FTransferSkinWeightsGroomNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub group_index: i32,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    pub curves_type: EGroomCollectionType,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(16))]
pub struct FTransferGeometrySkinWeightsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildGroomSplineSkinWeightsNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bone_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub root_bones: TArray<FString>,
    pub lod_index: i32,
    pub samples_per_segment: i32,
    pub group_index: i32,
    pub curves_type: EGroomCollectionType,
    pub spline_param_name: FString,
    pub spline_bones_param_name: FString,
}
#[repr(C, align(8))]
pub struct FConvertSplineToLinearSkinWeightsNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub spline_param_name: FString,
    pub spline_bones_name: FString,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bones_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub group_index: i32,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub curves_type: EGroomCollectionType,
}
#[repr(C, align(8))]
pub struct FConvertLinearToSplineSkinWeightsNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_index: i32,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bone_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_param_name: FString,
    pub spline_bones_name: FString,
    pub curves_type: EGroomCollectionType,
}
#[repr(C, align(8))]
pub struct FBuildSplineSkinWeightsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub root_bones: TArray<FString>,
    pub samples_per_segment: i32,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bones_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FSplineToLinearSkinWeightsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bones_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FLinearToSplineSkinWeightsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub bone_indices_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub bone_weights_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_param_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub spline_bones_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildGuidesLODsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_parents_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub curve_lods_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildCurveLODsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub curve_parents_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub curve_lods_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FGenerateGuidesCurvesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub guides_count: i32,
}
#[repr(C, align(8))]
pub struct FGenerateCurveGeometryDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub source_curves: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub curve_count: i32,
    pub b_merge_curves: bool,
}
#[repr(C, align(8))]
pub struct FGetGroomAssetDataflowNode {
    pub groom_asset: UPtr<crate::bindings::hair_strands_core::UGroomAsset>,
    pub curves_type: EGroomCollectionType,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetGroomAssetDataflowNode_v2 {
    pub groom_asset: UPtr<crate::bindings::hair_strands_core::UGroomAsset>,
}
#[repr(C, align(8))]
pub struct FGroomAssetToCollectionDataflowNode {
    pub groom_asset: UPtr<crate::bindings::hair_strands_core::UGroomAsset>,
    pub curves_type: EGroomCollectionType,
    pub curves_thickness: f32,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetGroomAttributesDataflowNode {
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub curves_type: EGroomCollectionType,
    pub attribute_type: EGroomAttributeType,
}
#[repr(C, align(8))]
pub struct FGetCurveAttributesDataflowNode {
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub attribute_type: EGroomAttributeType,
}
#[repr(C, align(8))]
pub struct FGroomAssetTerminalDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_keys: TArray<
        crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    >,
    pub attribute_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FGroomAssetTerminalDataflowNode_v2 {
    pub strands_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub guides_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_keys: TArray<
        crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    >,
    pub attribute_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FResampleGuidesPointsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub points_count: EGroomNumPoints,
}
#[repr(C, align(8))]
pub struct FResampleCurvePointsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub points_count: EGroomNumPoints,
    pub num_points: i32,
}
#[repr(C, align(8))]
pub struct FSmoothGuidesCurvesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub smoothing_factor: f32,
}
#[repr(C, align(8))]
pub struct FSmoothCurvePointsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
    pub smoothing_factor: f32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomCollectionType(pub u8);
impl EGroomCollectionType {
    pub const STRANDS: EGroomCollectionType = EGroomCollectionType(0);
    pub const GUIDES: EGroomCollectionType = EGroomCollectionType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomAttributeType(pub u8);
impl EGroomAttributeType {
    pub const KINEMATIC_WEIGHTS: EGroomAttributeType = EGroomAttributeType(0);
    pub const BONE_INDICES: EGroomAttributeType = EGroomAttributeType(1);
    pub const BONE_WEIGHTS: EGroomAttributeType = EGroomAttributeType(2);
    pub const CURVE_PARENTS: EGroomAttributeType = EGroomAttributeType(3);
    pub const CURVE_LODS: EGroomAttributeType = EGroomAttributeType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGroomNumPoints(pub u8);
impl EGroomNumPoints {
    pub const DEFAULT: EGroomNumPoints = EGroomNumPoints(0);
    pub const SIZE4: EGroomNumPoints = EGroomNumPoints(4);
    pub const SIZE8: EGroomNumPoints = EGroomNumPoints(8);
    pub const SIZE16: EGroomNumPoints = EGroomNumPoints(16);
    pub const SIZE32: EGroomNumPoints = EGroomNumPoints(32);
    pub const SIZE64: EGroomNumPoints = EGroomNumPoints(64);
}
