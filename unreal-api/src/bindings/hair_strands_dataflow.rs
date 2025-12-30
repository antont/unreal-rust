#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FAttachGuidesRootsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub group_index: i32,
    pub kinematic_weights_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FAttachCurveRootsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub kinematic_weights_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildCurveWeightsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub curve_weights: FRuntimeFloatCurve,
    pub weights_attribute: FCollectionAttributeKey,
}
#[repr(C, align(16))]
pub struct FTransferSkinWeightsGroomNode {
    pub collection: FManagedArrayCollection,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub lod_index: i32,
    pub group_index: i32,
    pub relative_transform: FTransform,
    pub curves_type: EGroomCollectionType,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
}
#[repr(C, align(16))]
pub struct FTransferGeometrySkinWeightsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub lod_index: i32,
    pub relative_transform: FTransform,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildGroomSplineSkinWeightsNode {
    pub collection: FManagedArrayCollection,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bone_key: FCollectionAttributeKey,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
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
    pub collection: FManagedArrayCollection,
    pub spline_param_name: FString,
    pub spline_bones_name: FString,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bones_key: FCollectionAttributeKey,
    pub group_index: i32,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
    pub curves_type: EGroomCollectionType,
}
#[repr(C, align(8))]
pub struct FConvertLinearToSplineSkinWeightsNode {
    pub collection: FManagedArrayCollection,
    pub group_index: i32,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bone_key: FCollectionAttributeKey,
    pub spline_param_name: FString,
    pub spline_bones_name: FString,
    pub curves_type: EGroomCollectionType,
}
#[repr(C, align(8))]
pub struct FBuildSplineSkinWeightsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub lod_index: i32,
    pub root_bones: TArray<FString>,
    pub samples_per_segment: i32,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bones_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FSplineToLinearSkinWeightsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bones_key: FCollectionAttributeKey,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FLinearToSplineSkinWeightsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub bone_indices_key: FCollectionAttributeKey,
    pub bone_weights_key: FCollectionAttributeKey,
    pub spline_param_key: FCollectionAttributeKey,
    pub spline_bones_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildGuidesLODsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_parents_key: FCollectionAttributeKey,
    pub curve_lods_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FBuildCurveLODsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub curve_parents_key: FCollectionAttributeKey,
    pub curve_lods_key: FCollectionAttributeKey,
}
#[repr(C, align(8))]
pub struct FGenerateGuidesCurvesDataflowNode {
    pub collection: FManagedArrayCollection,
    pub guides_count: i32,
}
#[repr(C, align(8))]
pub struct FGenerateCurveGeometryDataflowNode {
    pub collection: FManagedArrayCollection,
    pub source_curves: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub curve_count: i32,
    pub b_merge_curves: bool,
}
#[repr(C, align(8))]
pub struct FGetGroomAssetDataflowNode {
    pub groom_asset: UPtr<UGroomAsset>,
    pub curves_type: EGroomCollectionType,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetGroomAssetDataflowNode_v2 {
    pub groom_asset: UPtr<UGroomAsset>,
}
#[repr(C, align(8))]
pub struct FGroomAssetToCollectionDataflowNode {
    pub groom_asset: UPtr<UGroomAsset>,
    pub curves_type: EGroomCollectionType,
    pub curves_thickness: f32,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetGroomAttributesDataflowNode {
    pub attribute_key: FCollectionAttributeKey,
    pub curves_type: EGroomCollectionType,
    pub attribute_type: EGroomAttributeType,
}
#[repr(C, align(8))]
pub struct FGetCurveAttributesDataflowNode {
    pub attribute_key: FCollectionAttributeKey,
    pub attribute_type: EGroomAttributeType,
}
#[repr(C, align(8))]
pub struct FGroomAssetTerminalDataflowNode {
    pub collection: FManagedArrayCollection,
    pub attribute_keys: TArray<FCollectionAttributeKey>,
    pub attribute_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FGroomAssetTerminalDataflowNode_v2 {
    pub strands_collection: FManagedArrayCollection,
    pub guides_collection: FManagedArrayCollection,
    pub attribute_keys: TArray<FCollectionAttributeKey>,
    pub attribute_names: TArray<FName>,
}
#[repr(C, align(8))]
pub struct FResampleGuidesPointsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub points_count: EGroomNumPoints,
}
#[repr(C, align(8))]
pub struct FResampleCurvePointsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub points_count: EGroomNumPoints,
    pub num_points: i32,
}
#[repr(C, align(8))]
pub struct FSmoothGuidesCurvesDataflowNode {
    pub collection: FManagedArrayCollection,
    pub smoothing_factor: f32,
}
#[repr(C, align(8))]
pub struct FSmoothCurvePointsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub curve_selection: FDataflowCurveSelection,
    pub smoothing_factor: f32,
}
