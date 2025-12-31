#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCreateColorArrayFromFloatArrayDataflowNode {
    pub float_array: TArray<f32>,
    pub color_array: TArray<crate::bindings::core_u_object::FLinearColor>,
    pub b_normalize_input: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FGetFloatArrayElementDataflowNode {
    pub index: i32,
    pub float_array: TArray<f32>,
    pub float_value: f32,
}
#[repr(C, align(8))]
pub struct FFloatArrayToIntArrayDataflowNode {
    pub function: EFloatArrayToIntArrayFunctionEnum,
    pub float_array: TArray<f32>,
    pub int_array: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FGetArrayElementDataflowNode {
    pub index: i32,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub point: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FGetNumArrayElementsDataflowNode {
    pub float_array: TArray<f32>,
    pub int_array: TArray<i32>,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub vector3f_array: TArray<crate::bindings::core_u_object::FVector3f>,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FBoolArrayToFaceSelectionDataflowNode {
    pub bool_attribute_data: TArray<bool>,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FFloatArrayToVertexSelectionDataflowNode {
    pub float_array: TArray<f32>,
    pub operation: ECompareOperation1Enum,
    pub threshold: f32,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FFloatArrayNormalizeDataflowNode {
    pub in_float_array: TArray<f32>,
    pub selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub min_range: f32,
    pub max_range: f32,
    pub out_float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FVectorArrayNormalizeDataflowNode {
    pub in_vector_array: TArray<crate::bindings::core_u_object::FVector>,
    pub selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub out_vector_array: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FUnionIntArraysDataflowNode {
    pub in_array1: TArray<i32>,
    pub in_array2: TArray<i32>,
    pub out_array: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRemoveFloatArrayElementDataflowNode {
    pub index: i32,
    pub b_preserve_order: bool,
    pub float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FFloatArrayComputeStatisticsDataflowNode {
    pub float_array: TArray<f32>,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub operation_name: EStatisticsOperationEnum,
    pub value: f32,
    pub indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FRandomizeFloatArrayDataflowNode {
    pub float_array: TArray<f32>,
    pub random_range_min: f32,
    pub random_range_max: f32,
    pub random_seed: i32,
}
#[repr(C, align(8))]
pub struct FDataflowGetArraySizeNode {
    pub array: crate::bindings::dataflow_core::FDataflowArrayTypes,
    pub size: i32,
}
#[repr(C, align(8))]
pub struct FDataflowGetArrayElementNode {
    pub array: crate::bindings::dataflow_core::FDataflowArrayTypes,
    pub index: i32,
    pub element: crate::bindings::dataflow_core::FDataflowAllTypes,
}
#[repr(C, align(8))]
pub struct FDataflowConvertToArrayNode {
    pub array: crate::bindings::dataflow_core::FDataflowArrayTypes,
    pub element: crate::bindings::dataflow_core::FDataflowAllTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMakeManagedArrayCollectionArrayNode {
    pub array: TArray<crate::bindings::chaos::FManagedArrayCollection>,
    pub element: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(16))]
pub struct FDataflowRootProxyMesh {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(16))]
pub struct FMakeRootProxyMeshDataflowNode {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub root_proxy_mesh: FDataflowRootProxyMesh,
}
#[repr(C, align(8))]
pub struct FMakeRootProxyMeshArrayDataflowNode {
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(16))]
pub struct FAddRootProxyMeshToArrayDataflowNode {
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
    pub root_proxy_mesh: FDataflowRootProxyMesh,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionTerminalDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionTerminalDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material_instances: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
}
#[repr(C, align(8))]
pub struct FGetGeometryCollectionAssetDataflowNode {
    pub asset: UPtr<crate::bindings::geometry_collection_engine::UGeometryCollection>,
}
#[repr(C, align(8))]
pub struct FGetGeometryCollectionSourcesDataflowNode {
    pub asset: UPtr<crate::bindings::geometry_collection_engine::UGeometryCollection>,
    pub sources: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionSource,
    >,
}
#[repr(C, align(8))]
pub struct FCreateGeometryCollectionFromSourcesDataflowNode {
    pub sources: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionSource,
    >,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material_instances: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
}
#[repr(C, align(8))]
pub struct FCreateGeometryCollectionFromSourcesDataflowNode_v2 {
    pub sources: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionSource,
    >,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionToCollectionDataflowNode {
    pub geometry_collection: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material_instances: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionToCollectionDataflowNode_v2 {
    pub geometry_collection: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FBlueprintToCollectionDataflowNode {
    pub blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    pub b_split_components: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material_instances: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
}
#[repr(C, align(8))]
pub struct FBlueprintToCollectionDataflowNode_v2 {
    pub blueprint: UPtr<crate::bindings::engine::UBlueprint>,
    pub b_split_components: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FAutoClusterDataflowNode {
    pub cluster_size_method: EClusterSizeMethodEnum,
    pub cluster_sites: i32,
    pub cluster_fraction: f32,
    pub site_size: f32,
    pub cluster_grid_width: i32,
    pub cluster_grid_depth: i32,
    pub cluster_grid_height: i32,
    pub drift_iterations: i32,
    pub minimum_size: f32,
    pub b_prefer_convexity: bool,
    pub concavity_tolerance: f32,
    pub auto_cluster: bool,
    pub enforce_site_parameters: bool,
    pub avoid_isolated: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub line_width_multiplier: f32,
    pub center_color: crate::bindings::core_u_object::FLinearColor,
    pub center_size: f32,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FClusterFlattenDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub optional_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterUnclusterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterMergeToNeighborsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub neighbor_selection_method: EClusterNeighborSelectionMethodEnum,
    pub min_volume_cube_root: f32,
    pub b_only_to_connected: bool,
    pub b_only_same_parent: bool,
}
#[repr(C, align(8))]
pub struct FClusterMergeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterIsolatedRootsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FClusterMagnetDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub iterations: i32,
}
#[repr(C, align(8))]
pub struct FVectorToStringDataflowNode {
    pub vector: crate::bindings::core_u_object::FVector,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FFloatToStringDataflowNode {
    pub float: f32,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FIntToStringDataflowNode {
    pub int: i32,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FBoolToStringDataflowNode {
    pub bool: bool,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FIntToFloatDataflowNode {
    pub int: i32,
    pub float: f32,
}
#[repr(C, align(8))]
pub struct FIntToDoubleDataflowNode {
    pub int: i32,
    pub double: f64,
}
#[repr(C, align(8))]
pub struct FFloatToDoubleDataflowNode {
    pub float: f32,
    pub double: f64,
}
#[repr(C, align(8))]
pub struct FFloatToIntDataflowNode {
    pub function: EFloatToIntFunctionEnum,
    pub float: f32,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FIntToBoolDataflowNode {
    pub int: i32,
    pub bool: bool,
}
#[repr(C, align(8))]
pub struct FBoolToIntDataflowNode {
    pub bool: bool,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FConvexHullToMeshDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_use_robust_hulls: bool,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FSphereCoveringToMeshDataflowNode {
    pub sphere_covering: FDataflowSphereCovering,
    pub vertices_along_each_side: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FDataflowSphereCovering {}
#[repr(C, align(8))]
pub struct FSphereCoveringCountSpheresNode {
    pub sphere_covering: FDataflowSphereCovering,
    pub num_spheres: i32,
}
#[repr(C, align(8))]
pub struct FMeshToOBJStringDebugDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub b_invert_faces: bool,
    pub string_obj: FString,
}
#[repr(C, align(8))]
pub struct FWriteStringToFile {
    pub file_path: FString,
    pub file_contents: FString,
}
#[repr(C, align(8))]
pub struct FPruneInCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FSetVisibilityInCollectionDataflowNode {
    pub visibility: EVisibiltyOptionsEnum,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FMergeInCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FRadialFalloffFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub sphere: crate::bindings::core_u_object::FSphere,
    pub translation: crate::bindings::core_u_object::FVector,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(16))]
pub struct FBoxFalloffFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FPlaneFalloffFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub position: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub distance: f32,
    pub translation: crate::bindings::core_u_object::FVector,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRadialIntMaskFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub sphere: crate::bindings::core_u_object::FSphere,
    pub translation: crate::bindings::core_u_object::FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition_type: EDataflowSetMaskConditionType,
    pub field_int_result: TArray<i32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformScalarFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformVectorFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub direction: crate::bindings::core_u_object::FVector,
    pub field_vector_result: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRadialVectorFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub field_vector_result: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRandomVectorFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub field_vector_result: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(16))]
pub struct FNoiseFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub min_range: f32,
    pub max_range: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformIntegerFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: i32,
    pub field_int_result: TArray<i32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FWaveScalarFieldDataflowNode {
    pub sample_positions: TArray<crate::bindings::core_u_object::FVector3f>,
    pub sample_indices: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub magnitude: f32,
    pub position: crate::bindings::core_u_object::FVector,
    pub translation: crate::bindings::core_u_object::FVector,
    pub wavelength: f32,
    pub period: f32,
    pub function_type: EDataflowWaveFunctionType,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FSumScalarFieldDataflowNode {
    pub field_float_left: TArray<f32>,
    pub field_remap_left: TArray<i32>,
    pub field_float_right: TArray<f32>,
    pub field_remap_right: TArray<i32>,
    pub magnitude: f32,
    pub operation: EDataflowFloatFieldOperationType,
    pub b_swap_inputs: bool,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FSumVectorFieldDataflowNode {
    pub field_float: TArray<f32>,
    pub field_float_remap: TArray<i32>,
    pub field_vector_left: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap_left: TArray<i32>,
    pub field_vector_right: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap_right: TArray<i32>,
    pub magnitude: f32,
    pub operation: EDataflowVectorFieldOperationType,
    pub b_swap_vector_inputs: bool,
    pub field_vector_result: TArray<crate::bindings::core_u_object::FVector>,
    pub field_remap: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FFieldMakeDenseFloatArrayDataflowNode {
    pub field_float_input: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
    pub default: f32,
    pub field_float_result: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FUniformScatterPointsDataflowNode {
    pub min_number_of_points: i32,
    pub max_number_of_points: i32,
    pub random_seed: f32,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FUniformScatterPointsDataflowNode_v2 {
    pub min_number_of_points: i32,
    pub max_number_of_points: i32,
    pub random_seed: i32,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FClusterScatterPointsDataflowNode {
    pub number_clusters_min: i32,
    pub number_clusters_max: i32,
    pub points_per_cluster_min: i32,
    pub points_per_cluster_max: i32,
    pub cluster_radius_fraction_min: f32,
    pub cluster_radius_fraction_max: f32,
    pub cluster_radius_offset: f32,
    pub random_seed: i32,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FRadialScatterPointsDataflowNode {
    pub center: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub angular_steps: i32,
    pub radial_steps: i32,
    pub angle_offset: f32,
    pub variability: f32,
    pub random_seed: f32,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FRadialScatterPointsDataflowNode_v2 {
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub random_seed: i32,
    pub angular_steps: i32,
    pub angle_offset: f32,
    pub angular_noise: f32,
    pub radius: f32,
    pub radial_steps: i32,
    pub radial_step_exponent: f32,
    pub radial_min_step: f32,
    pub radial_noise: f32,
    pub radial_variability: f32,
    pub angular_variability: f32,
    pub axial_variability: f32,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FGridScatterPointsDataflowNode {
    pub number_of_points_in_x: i32,
    pub number_of_points_in_y: i32,
    pub number_of_points_in_z: i32,
    pub random_seed: i32,
    pub max_random_displacement_x: f32,
    pub max_random_displacement_y: f32,
    pub max_random_displacement_z: f32,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(16))]
pub struct FTransformPointsDataflowNode {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FAppendPointsDataflowNode {
    pub points_a: TArray<crate::bindings::core_u_object::FVector>,
    pub points_b: TArray<crate::bindings::core_u_object::FVector>,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FVoronoiFractureDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub random_seed: f32,
    pub chance_to_fracture: f32,
    pub group_fracture: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(16))]
pub struct FVoronoiFractureDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub split_islands: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FPlaneCutterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub num_planes: i32,
    pub random_seed: f32,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(16))]
pub struct FPlaneCutterDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cut_planes: TArray<crate::bindings::core_u_object::FTransform>,
    pub num_planes: i32,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub split_islands: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub render_type: crate::bindings::dataflow_engine::EDataflowDebugDrawRenderType,
    pub plane_size_multiplier: f32,
    pub b_translucent: bool,
    pub b_randomize_colors: bool,
    pub color_random_seed: i32,
    pub line_width_multiplier: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FExplodedViewDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub uniform_scale: f32,
    pub scale: crate::bindings::core_u_object::FVector,
    pub offset: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FSliceCutterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub slices_x: i32,
    pub slices_y: i32,
    pub slices_z: i32,
    pub slice_angle_variation: f32,
    pub slice_offset_variation: f32,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub split_islands: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FBrickCutterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub bond: crate::bindings::fracture_engine::EFractureBrickBondEnum,
    pub brick_length: f32,
    pub brick_height: f32,
    pub brick_depth: f32,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub split_islands: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FMeshCutterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub cutting_dynamic_meshes: TArray<
        UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    >,
    pub cutting_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub b_use_hi_res: bool,
    pub lod_level: i32,
    pub cut_distribution: crate::bindings::fracture_engine::EMeshCutterCutDistribution,
    pub per_cut_mesh_selection: crate::bindings::fracture_engine::EMeshCutterPerCutMeshSelection,
    pub number_to_scatter: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_z: i32,
    pub variability: f32,
    pub min_scale_factor: f32,
    pub max_scale_factor: f32,
    pub b_random_orientation: bool,
    pub roll_range: f32,
    pub pitch_range: f32,
    pub yaw_range: f32,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub split_islands: bool,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FUniformFractureDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub min_voronoi_sites: i32,
    pub max_voronoi_sites: i32,
    pub internal_material_id: i32,
    pub random_seed: i32,
    pub chance_to_fracture: f32,
    pub group_fracture: bool,
    pub split_islands: bool,
    pub grout: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub octave_number: i32,
    pub point_spacing: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
    pub new_geometry_transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(4))]
pub struct FMinSettings {
    pub min_attr_value: f32,
    pub min_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FMaxSettings {
    pub max_attr_value: f32,
    pub max_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FVisualizeFractureDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub level: i32,
    pub random_seed: i32,
    pub b_apply_exploded_view: bool,
    pub explode_amount: f32,
    pub scale: crate::bindings::core_u_object::FVector,
    pub b_apply_color: bool,
    pub coloring_type: EDataflowVisualizeFractureColoringType,
    pub random_color_range_min: i32,
    pub random_color_range_max: i32,
    pub attribute: FString,
    pub min: FMinSettings,
    pub max: FMaxSettings,
    pub offset: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FSetFloatAttributeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute: FString,
    pub method: EDataflowSetFloatArrayMethod,
    pub random_seed: i32,
    pub noise_scale: f32,
    pub float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FMakeLiteralStringDataflowNode {
    pub value: FString,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FMakeLiteralStringDataflowNode_v2 {
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FMakePointsDataflowNode {
    pub point: TArray<crate::bindings::core_u_object::FVector>,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FMakeBoxDataflowNode {
    pub data_type: EMakeBoxDataTypeEnum,
    pub min: crate::bindings::core_u_object::FVector,
    pub max: crate::bindings::core_u_object::FVector,
    pub center: crate::bindings::core_u_object::FVector,
    pub size: crate::bindings::core_u_object::FVector,
    pub box_: crate::bindings::core_u_object::FBox,
}
#[repr(C, align(8))]
pub struct FMakeSphereDataflowNode {
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub sphere: crate::bindings::core_u_object::FSphere,
}
#[repr(C, align(8))]
pub struct FMakeLiteralFloatDataflowNode {
    pub value: f32,
    pub float: f32,
}
#[repr(C, align(8))]
pub struct FMakeLiteralFloatDataflowNode_v2 {
    pub float: f32,
}
#[repr(C, align(8))]
pub struct FMakeLiteralDoubleDataflowNode {
    pub double: f64,
}
#[repr(C, align(8))]
pub struct FMakeLiteralIntDataflowNode {
    pub value: i32,
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FMakeLiteralIntDataflowNode_v2 {
    pub int: i32,
}
#[repr(C, align(8))]
pub struct FMakeLiteralBoolDataflowNode {
    pub value: bool,
    pub bool: bool,
}
#[repr(C, align(8))]
pub struct FMakeLiteralBoolDataflowNode_v2 {
    pub bool: bool,
}
#[repr(C, align(8))]
pub struct FMakeLiteralVectorDataflowNode {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vector: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(16))]
pub struct FMakeTransformDataflowNode {
    pub in_translation: crate::bindings::core_u_object::FVector,
    pub in_rotation: crate::bindings::core_u_object::FVector,
    pub in_scale: crate::bindings::core_u_object::FVector,
    pub out_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FMakeTransformDataflowNode_v2 {
    pub translation: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FVector,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub quat: crate::bindings::core_u_object::FQuat,
    pub scale: crate::bindings::core_u_object::FVector,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FMakeQuaternionDataflowNode {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub quaternion: crate::bindings::core_u_object::FQuat,
}
#[repr(C, align(8))]
pub struct FMakeFloatArrayDataflowNode {
    pub num_elements: i32,
    pub value: f32,
    pub float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FMakeCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_add_root_transform: bool,
}
#[repr(C, align(8))]
pub struct FMakeRotatorDataflowNode {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub rotator: crate::bindings::core_u_object::FRotator,
}
#[repr(C, align(16))]
pub struct FBreakTransformDataflowNode {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub translation: crate::bindings::dataflow_core::FDataflowVectorTypes,
    pub rotation: crate::bindings::core_u_object::FVector,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub quat: crate::bindings::core_u_object::FQuat,
    pub scale: crate::bindings::dataflow_core::FDataflowVectorTypes,
}
#[repr(C, align(8))]
pub struct FMakeSphereMeshDataflowNode {
    pub radius: f32,
    pub num_phi: i32,
    pub num_theta: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeCapsuleMeshDataflowNode {
    pub radius: f32,
    pub segment_length: f32,
    pub num_hemisphere_arc_steps: i32,
    pub num_circle_steps: i32,
    pub num_segment_steps: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeCylinderMeshDataflowNode {
    pub radius1: f32,
    pub radius2: f32,
    pub height: f32,
    pub length_samples: i32,
    pub angle_samples: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeBoxMeshDataflowNode {
    pub center: crate::bindings::core_u_object::FVector,
    pub size: crate::bindings::core_u_object::FVector,
    pub subdivisions_x: i32,
    pub subdivisions_y: i32,
    pub subdivisions_z: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(16))]
pub struct FMakePlaneDataflowNode {
    pub base_point: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub plane_size_multiplier: f32,
    pub plane: crate::bindings::core_u_object::FPlane,
}
#[repr(C, align(8))]
pub struct FMakeDiscMeshDataflowNode {
    pub radius: f32,
    pub normal: crate::bindings::core_u_object::FVector,
    pub angle_samples: i32,
    pub radial_samples: i32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeStairMeshDataflowNode {
    pub stair_type: EDataflowStairTypeEnum,
    pub num_steps: i32,
    pub step_width: f32,
    pub step_height: f32,
    pub step_depth: f32,
    pub curve_angle: f32,
    pub inner_radius: f32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeRectangleMeshDataflowNode {
    pub origin: crate::bindings::core_u_object::FVector,
    pub normal: crate::bindings::core_u_object::FVector,
    pub width: f32,
    pub height: f32,
    pub width_vertex_count: i32,
    pub height_vertex_count: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeTorusMeshDataflowNode {
    pub origin: crate::bindings::core_u_object::FVector,
    pub radius1: f32,
    pub profile_vertex_count: i32,
    pub radius2: f32,
    pub sweep_vertex_count: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FGetMaterialInterfaceArraySizeDataflowNode {
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub size: i32,
}
#[repr(C, align(8))]
pub struct FGetMaterialInterfaceAssetDataflowNode {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FGetFromMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FSetIntoMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FAddToMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub materials_to_add: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FAssignMaterialInterfaceToCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub material_index: i32,
    pub b_merge_duplicate_materials: bool,
}
#[repr(C, align(8))]
pub struct FMaterialInterfaceTextureOverrideDataflowNode {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub target_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub override_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FAddMaterialToCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub outside_material: UPtr<crate::bindings::engine::UMaterial>,
    pub inside_material: UPtr<crate::bindings::engine::UMaterial>,
    pub b_assign_outside_material: bool,
    pub b_assign_inside_material: bool,
}
#[repr(C, align(8))]
pub struct FReAssignMaterialInCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub outside_material_idx: i32,
    pub inside_material_idx: i32,
    pub b_assign_outside_material: bool,
    pub b_assign_inside_material: bool,
}
#[repr(C, align(8))]
pub struct FMaterialsInfoDataflowNode {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FGetMaterialFromMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material: UPtr<crate::bindings::engine::UMaterial>,
    pub material_idx: i32,
}
#[repr(C, align(8))]
pub struct FSetMaterialInMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material: UPtr<crate::bindings::engine::UMaterial>,
    pub operation: ESetMaterialOperationTypeEnum,
    pub material_idx: i32,
}
#[repr(C, align(8))]
pub struct FMakeMaterialDataflowNode {
    pub in_material: UPtr<crate::bindings::engine::UMaterial>,
    pub material: UPtr<crate::bindings::engine::UMaterial>,
}
#[repr(C, align(8))]
pub struct FMakeMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
}
#[repr(C, align(8))]
pub struct FAddDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSubtractDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMultiplyDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSafeDivideDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FDivideDataflowNode {}
#[repr(C, align(8))]
pub struct FDivisionDataflowNode {
    pub dividend: f32,
    pub divisor: f32,
    pub remainder: f32,
    pub return_value: i32,
}
#[repr(C, align(8))]
pub struct FSafeReciprocalDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSquareDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSquareRootDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FInverseSqrtDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FCubeDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FNegateDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FAbsDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FFloorDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FCeilDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FRoundDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FTruncDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FFracDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMinDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMaxDataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMin3DataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub float_c: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMax3DataflowNode {
    pub float_a: f32,
    pub float_b: f32,
    pub float_c: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSignDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FClampDataflowNode {
    pub float: f32,
    pub min: f32,
    pub max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FFitDataflowNode {
    pub float: f32,
    pub old_min: f32,
    pub old_max: f32,
    pub new_min: f32,
    pub new_max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FEFitDataflowNode {
    pub float: f32,
    pub old_min: f32,
    pub old_max: f32,
    pub new_min: f32,
    pub new_max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FPowDataflowNode {
    pub base: f32,
    pub exp: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FLogDataflowNode {
    pub base: f32,
    pub a: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FLogeDataflowNode {
    pub a: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FLerpDataflowNode {
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FWrapDataflowNode {
    pub float: f32,
    pub min: f32,
    pub max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FExpDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FSinDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FArcSinDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FCosDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FArcCosDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FTanDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FArcTanDataflowNode {
    pub float: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FArcTan2DataflowNode {
    pub y: f32,
    pub x: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FNormalizeToRangeDataflowNode {
    pub float: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FScaleVectorDataflowNode {
    pub vector: crate::bindings::core_u_object::FVector,
    pub scale: f32,
    pub scaled_vector: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FDotProductDataflowNode {
    pub vector_a: crate::bindings::core_u_object::FVector,
    pub vector_b: crate::bindings::core_u_object::FVector,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FCrossProductDataflowNode {
    pub vector_a: crate::bindings::core_u_object::FVector,
    pub vector_b: crate::bindings::core_u_object::FVector,
    pub return_value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FNormalizeDataflowNode {
    pub vector_a: crate::bindings::core_u_object::FVector,
    pub tolerance: f32,
    pub return_value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FLengthDataflowNode {
    pub vector: crate::bindings::core_u_object::FVector,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FDistanceDataflowNode {
    pub point_a: crate::bindings::core_u_object::FVector,
    pub point_b: crate::bindings::core_u_object::FVector,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FIsNearlyZeroDataflowNode {
    pub float: f32,
    pub return_value: bool,
}
#[repr(C, align(8))]
pub struct FRandomFloatDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FRandomFloatInRangeDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub min: f32,
    pub max: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FRandomUnitVectorDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub return_value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRandomUnitVectorInConeDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub cone_direction: crate::bindings::core_u_object::FVector,
    pub cone_half_angle: f32,
    pub return_value: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FRadiansToDegreesDataflowNode {
    pub radians: f32,
    pub degrees: f32,
}
#[repr(C, align(8))]
pub struct FDegreesToRadiansDataflowNode {
    pub degrees: f32,
    pub radians: f32,
}
#[repr(C, align(8))]
pub struct FMathConstantsDataflowNode {
    pub constant: EMathConstantsEnum,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FOneMinusDataflowNode {
    pub a: f32,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FFloatMathExpressionDataflowNode {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub expression: FString,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FMathExpressionDataflowNode {
    pub a: crate::bindings::dataflow_core::FDataflowNumericTypes,
    pub b: crate::bindings::dataflow_core::FDataflowNumericTypes,
    pub c: crate::bindings::dataflow_core::FDataflowNumericTypes,
    pub d: crate::bindings::dataflow_core::FDataflowNumericTypes,
    pub expression: FString,
    pub return_value: crate::bindings::dataflow_core::FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FPointsToMeshDataflowNode {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FMeshProcessorDataflowNodeBase {
    pub mesh_processor: TSubclassOf<
        crate::bindings::geometry_framework::UDynamicMeshProcessorBlueprint,
    >,
    pub mesh_processor_instance: UPtr<
        crate::bindings::geometry_framework::UDynamicMeshProcessorBlueprint,
    >,
    pub owning_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub dynamic_connections: crate::bindings::dataflow_engine::FDataflowDynamicConnections,
    pub property_bag: crate::bindings::core_u_object::FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FApplyMeshProcessorToMeshDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FApplyMeshProcessorToGeometryCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionToMeshesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_convert_selection_to_leaves: bool,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
    pub meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FAppendMeshesToCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub added_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
    pub parent_index: i32,
}
#[repr(C, align(8))]
pub struct FBoxToMeshDataflowNode {
    pub box_: crate::bindings::core_u_object::FBox,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FMeshInfoDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub info_string: FString,
}
#[repr(C, align(8))]
pub struct FMeshToCollectionDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_split_islands: bool,
    pub b_connect_islands_by_vertex_overlap: bool,
    pub connect_vertices_threshold: f32,
    pub b_add_cluster_root_for_single_mesh: bool,
}
#[repr(C, align(8))]
pub struct FCollectionToMeshDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_center_pivot: bool,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
}
#[repr(C, align(8))]
pub struct FStaticMeshToMeshDataflowNode {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub b_use_hi_res: bool,
    pub lod_level: i32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FGetMeshBoundingBoxDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub dimensions: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FMeshAppendDataflowNode {
    pub mesh1: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub mesh2: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FDataflowMeshAppendDataflowNode {
    pub mesh: UPtr<crate::bindings::dataflow_engine::UDataflowMesh>,
    pub append_mesh: UPtr<crate::bindings::dataflow_engine::UDataflowMesh>,
}
#[repr(C, align(8))]
pub struct FMakeDataflowMeshDataflowNode {
    pub in_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub in_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub mesh: UPtr<crate::bindings::dataflow_engine::UDataflowMesh>,
}
#[repr(C, align(8))]
pub struct FDuplicateMeshUVChannelNode {
    pub mesh: UPtr<crate::bindings::dataflow_engine::UDataflowMesh>,
    pub source_uv_channel: i32,
    pub new_uv_channel: i32,
}
#[repr(C, align(8))]
pub struct FSplitMeshIslandsDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
    pub split_method: EDataflowMeshSplitIslandsMethod,
    pub connect_vertices_threshold: f32,
}
#[repr(C, align(8))]
pub struct FSplitDataflowMeshDataflowNode {
    pub in_mesh: UPtr<crate::bindings::dataflow_engine::UDataflowMesh>,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub material_array: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FMeshCopyToPointsDataflowNode {
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub mesh_to_copy: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub scale: f32,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub meshes: TArray<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FGetMeshDataDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub vertex_count: i32,
    pub edge_count: i32,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FGetCollectionFromAssetDataflowNode {
    pub collection_asset: UPtr<
        crate::bindings::geometry_collection_engine::UGeometryCollection,
    >,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FAppendCollectionAssetsDataflowNode {
    pub collection1: crate::bindings::chaos::FManagedArrayCollection,
    pub collection2: crate::bindings::chaos::FManagedArrayCollection,
    pub geometry_group_guids_out1: TArray<FString>,
    pub geometry_group_guids_out2: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FPrintStringDataflowNode {
    pub b_print_to_screen: bool,
    pub b_print_to_log: bool,
    pub color: crate::bindings::core_u_object::FColor,
    pub duration: f32,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FLogStringDataflowNode {
    pub b_print_to_log: bool,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FBoundingBoxDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub center: crate::bindings::core_u_object::FVector,
    pub dimensions: crate::bindings::core_u_object::FVector,
}
#[repr(C, align(8))]
pub struct FBoundingSphereDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bounding_sphere: crate::bindings::core_u_object::FSphere,
}
#[repr(C, align(8))]
pub struct FGetBoxLengthsDataflowNode {
    pub boxes: TArray<crate::bindings::core_u_object::FBox>,
    pub lengths: TArray<f32>,
    pub measurement_method: EBoxLengthMeasurementMethod,
}
#[repr(C, align(8))]
pub struct FExpandBoundingBoxDataflowNode {
    pub bounding_box: crate::bindings::core_u_object::FBox,
    pub min: crate::bindings::core_u_object::FVector,
    pub max: crate::bindings::core_u_object::FVector,
    pub center: crate::bindings::core_u_object::FVector,
    pub half_extents: crate::bindings::core_u_object::FVector,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FExpandBoundingSphereDataflowNode {
    pub bounding_sphere: crate::bindings::core_u_object::FSphere,
    pub center: crate::bindings::core_u_object::FVector,
    pub radius: f32,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FExpandVectorDataflowNode {
    pub vector: crate::bindings::core_u_object::FVector,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C, align(8))]
pub struct FStringAppendDataflowNode {
    pub string1: FString,
    pub string2: FString,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FStringAppendDataflowNode_v2 {
    pub inputs: TArray<crate::bindings::dataflow_core::FDataflowStringConvertibleTypes>,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FHashStringDataflowNode {
    pub string: FString,
    pub hash: i32,
}
#[repr(C, align(8))]
pub struct FHashVectorDataflowNode {
    pub vector: crate::bindings::core_u_object::FVector,
    pub hash: i32,
}
#[repr(C, align(8))]
pub struct FGetBoundingBoxesFromCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub bounding_boxes: TArray<crate::bindings::core_u_object::FBox>,
}
#[repr(C, align(8))]
pub struct FGetRootIndexFromCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub root_index: i32,
}
#[repr(C, align(8))]
pub struct FGetCentroidsFromCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_color_by_level: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub b_size_by_level: bool,
    pub size: f32,
    pub point_size: crate::bindings::engine::FRuntimeFloatCurve,
    pub centroids: TArray<crate::bindings::core_u_object::FVector>,
    pub levels: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FTransformCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub translate: crate::bindings::core_u_object::FVector,
    pub rotation_order: ERotationOrderEnum,
    pub rotate: crate::bindings::core_u_object::FVector,
    pub scale: crate::bindings::core_u_object::FVector,
    pub uniform_scale: f32,
    pub rotate_pivot: crate::bindings::core_u_object::FVector,
    pub scale_pivot: crate::bindings::core_u_object::FVector,
    pub b_invert_transformation: bool,
}
#[repr(C, align(8))]
pub struct FBakeTransformsInCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FTransformMeshDataflowNode {
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub translate: crate::bindings::core_u_object::FVector,
    pub rotation_order: ERotationOrderEnum,
    pub rotate: crate::bindings::core_u_object::FVector,
    pub scale: crate::bindings::core_u_object::FVector,
    pub uniform_scale: f32,
    pub rotate_pivot: crate::bindings::core_u_object::FVector,
    pub scale_pivot: crate::bindings::core_u_object::FVector,
    pub b_invert_transformation: bool,
}
#[repr(C, align(8))]
pub struct FCompareIntDataflowNode {
    pub operation: ECompareOperationEnum,
    pub int_a: i32,
    pub int_b: i32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FCompareFloatDataflowNode {
    pub operation: ECompareOperationEnum,
    pub float_a: f32,
    pub float_b: f32,
    pub result: bool,
}
#[repr(C, align(8))]
pub struct FBooleanOperationDataflowNode {
    pub operation: EBooleanOperationEnum,
    pub b_bool_a: bool,
    pub b_bool_b: bool,
    pub b_result: bool,
}
#[repr(C, align(8))]
pub struct FBranchMeshDataflowNode {
    pub mesh_a: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub mesh_b: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub b_condition: bool,
    pub mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FBranchCollectionDataflowNode {
    pub true_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub false_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_condition: bool,
    pub chosen_collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetSchemaDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FRemoveOnBreakDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_enabled_removal: bool,
    pub post_break_timer: crate::bindings::core_u_object::FVector2f,
    pub removal_timer: crate::bindings::core_u_object::FVector2f,
    pub b_cluster_crumbling: bool,
}
#[repr(C, align(8))]
pub struct FSetAnchorStateDataflowNode {
    pub anchor_state: EAnchorStateEnum,
    pub b_set_not_selected_bones_to_opposite_state: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FSetDynamicStateDataflowNode {
    pub dynamic_state: EDataflowGeometryCollectionDynamicState,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FProximityDataflowNode {
    pub proximity_method: EProximityMethodEnum,
    pub distance_threshold: f32,
    pub contact_threshold: f32,
    pub filter_contact_method: EProximityContactFilteringMethodEnum,
    pub b_use_as_connection_graph: bool,
    pub contact_area_method: EConnectionContactAreaMethodEnum,
    pub b_recompute_convex_hulls: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub line_width_multiplier: f32,
    pub center_color: crate::bindings::core_u_object::FLinearColor,
    pub center_size: f32,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(16))]
pub struct FCollectionSetPivotDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FAddCustomCollectionAttributeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub custom_attribute_type: ECustomAttributeTypeEnum,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FGetNumElementsInCollectionGroupDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FGetCollectionAttributeDataTypedDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub float_attribute_data: TArray<f32>,
    pub double_attribute_data: TArray<f64>,
    pub int32_attribute_data: TArray<i32>,
    pub string_attribute_data: TArray<FString>,
    pub vector3f_attribute_data: TArray<crate::bindings::core_u_object::FVector3f>,
    pub vector3d_attribute_data: TArray<crate::bindings::core_u_object::FVector3d>,
    pub linear_color_attribute_data: TArray<
        crate::bindings::core_u_object::FLinearColor,
    >,
}
#[repr(C, align(8))]
pub struct FGetCollectionAttributeDataTypedDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub numeric_array: crate::bindings::dataflow_core::FDataflowNumericArrayTypes,
    pub vector_array: crate::bindings::dataflow_core::FDataflowVectorArrayTypes,
    pub string_array: crate::bindings::dataflow_core::FDataflowStringArrayTypes,
}
#[repr(C, align(8))]
pub struct FSetCollectionAttributeDataTypedDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub float_attribute_data: TArray<f32>,
    pub double_attribute_data: TArray<f64>,
    pub int32_attribute_data: TArray<i32>,
    pub string_attribute_data: TArray<FString>,
    pub vector3f_attribute_data: TArray<crate::bindings::core_u_object::FVector3f>,
    pub vector3d_attribute_data: TArray<crate::bindings::core_u_object::FVector3d>,
    pub linear_color_attribute_data: TArray<
        crate::bindings::core_u_object::FLinearColor,
    >,
}
#[repr(C, align(8))]
pub struct FSelectionToVertexListDataflowNode {
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub vertex_list: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FMultiplyTransformDataflowNode {
    pub in_left_transform: crate::bindings::core_u_object::FTransform,
    pub in_right_transform: crate::bindings::core_u_object::FTransform,
    pub out_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(16))]
pub struct FInvertTransformDataflowNode {
    pub in_transform: crate::bindings::core_u_object::FTransform,
    pub out_transform: crate::bindings::core_u_object::FTransform,
}
#[repr(C, align(8))]
pub struct FBranchFloatDataflowNode {
    pub a: f32,
    pub b: f32,
    pub b_condition: bool,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FBranchIntDataflowNode {
    pub a: i32,
    pub b: i32,
    pub b_condition: bool,
    pub return_value: i32,
}
#[repr(C, align(8))]
pub struct FVisualizeTetrahedronsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertices: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FPointsToCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FCollectionToPointsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FSpheresToPointsDataflowNode {
    pub spheres: TArray<crate::bindings::core_u_object::FSphere>,
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub radii: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FGetFloatOverrideFromAssetDataflowNode {
    pub float: f32,
    pub float_default: f32,
}
#[repr(C, align(8))]
pub struct FGetIntOverrideFromAssetDataflowNode {
    pub int: i32,
    pub int_default: i32,
}
#[repr(C, align(8))]
pub struct FGetBoolOverrideFromAssetDataflowNode {
    pub bool: bool,
    pub bool_default: bool,
}
#[repr(C, align(8))]
pub struct FGetStringOverrideFromAssetDataflowNode {
    pub string: FString,
    pub string_default: FString,
}
#[repr(C, align(8))]
pub struct FCloseGeometryOnCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FFilterPointSetWithMeshDataflowNode {
    pub target_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub filter_method: u8,
    pub b_keep_inside: bool,
    pub winding_threshold: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub b_use_signed_distance: bool,
    pub sample_points: TArray<crate::bindings::core_u_object::FVector>,
}
#[repr(C, align(8))]
pub struct FUniformPointSamplingDataflowNode {
    pub target_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub sample_points: TArray<crate::bindings::core_u_object::FVector>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<crate::bindings::core_u_object::FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FNonUniformPointSamplingDataflowNode {
    pub target_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub max_sampling_radius: f32,
    pub size_distribution: crate::bindings::fracture_engine::ENonUniformSamplingDistributionMode,
    pub size_distribution_power: f32,
    pub sample_points: TArray<crate::bindings::core_u_object::FVector>,
    pub sample_radii: TArray<f32>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<crate::bindings::core_u_object::FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FVertexWeightedPointSamplingDataflowNode {
    pub target_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    pub vertex_weights: TArray<f32>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub max_sampling_radius: f32,
    pub size_distribution: crate::bindings::fracture_engine::ENonUniformSamplingDistributionMode,
    pub size_distribution_power: f32,
    pub weight_mode: crate::bindings::fracture_engine::ENonUniformSamplingWeightMode,
    pub b_invert_weights: bool,
    pub sample_points: TArray<crate::bindings::core_u_object::FVector>,
    pub sample_radii: TArray<f32>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<crate::bindings::core_u_object::FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionAllDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub transform_selection_a: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform_selection_b: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionInfoDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionNoneDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionInvertDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionRandomDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub random_threshold: f32,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionRootDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionCustomDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bone_indicies: FString,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionCustomDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bone_indices: FString,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionFromIndexArrayDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub bone_indices: TArray<i32>,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionParentDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByPercentageDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub percentage: i32,
    pub b_deterministic: bool,
    pub random_seed: f32,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionChildrenDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionSiblingsDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionLevelDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionTargetLevelDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub target_level: i32,
    pub b_skip_embedded: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionContactDataflowNode {
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_allow_contact_in_parent_levels: bool,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionLeafDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionClusterDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionClusterDataflowNode_v2 {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FSelectFloatArrayIndicesInRangeDataflowNode {
    pub values: TArray<f32>,
    pub min: f32,
    pub max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub indices: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionBySizeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub size_min: f32,
    pub size_max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub b_use_relative_size: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByVolumeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub volume_min: f32,
    pub volume_max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FCollectionTransformSelectionInBoxDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub box_: crate::bindings::core_u_object::FBox,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub ty: ESelectSubjectTypeEnum,
    pub b_all_vertices_must_contained_in_box: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FCollectionTransformSelectionInSphereDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub sphere: crate::bindings::core_u_object::FSphere,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub ty: ESelectSubjectTypeEnum,
    pub b_all_vertices_must_contained_in_sphere: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByFloatAttrDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_name: FString,
    pub attr_name: FString,
    pub min: f32,
    pub max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByIntAttrDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub group_name: FString,
    pub attr_name: FString,
    pub min: i32,
    pub max: i32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionCustomDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_indicies: FString,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionFaceSelectionCustomDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_indicies: FString,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionConvertDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub b_all_elements_must_be_selected: bool,
}
#[repr(C, align(8))]
pub struct FCollectionFaceSelectionInvertDataflowNode {
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionByPercentageDataflowNode {
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub percentage: i32,
    pub b_deterministic: bool,
    pub random_seed: f32,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub vertex_selection_a: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub vertex_selection_b: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionByAttrDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub group: ESelectionByAttrGroup,
    pub attribute: FString,
    pub operation: ESelectionByAttrOperation,
    pub value: FString,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub geometry_selection: crate::bindings::dataflow_core::FDataflowGeometrySelection,
    pub material_selection: crate::bindings::dataflow_core::FDataflowMaterialSelection,
    pub curve_selection: crate::bindings::dataflow_core::FDataflowCurveSelection,
}
#[repr(C, align(8))]
pub struct FGeometrySelectionToVertexSelectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub geometry_indices: FString,
    pub geometry_selection: crate::bindings::dataflow_core::FDataflowGeometrySelection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub selection_a: crate::bindings::dataflow_core::FDataflowSelectionTypes,
    pub selection_b: crate::bindings::dataflow_core::FDataflowSelectionTypes,
    pub selection: crate::bindings::dataflow_core::FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionInvertDataflowNode {
    pub selection: crate::bindings::dataflow_core::FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FCollectionSelectInternalFacesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectTransformStringDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute: FString,
    pub search_text: FString,
    pub method: EDataflowCollectionSelectionByNameMethod,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSetTransformStringValueDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub attribute: FString,
    pub text_to_set: FString,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshToCollectionDataflowNode {
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_import_transform_only: bool,
}
#[repr(C, align(8))]
pub struct FCollectionToSkeletalMeshDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
}
#[repr(C, align(8))]
pub struct FSkeletonToCollectionDataflowNode {
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(16))]
pub struct FStaticMeshToCollectionDataflowNode_v2 {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub mesh_transform: crate::bindings::core_u_object::FTransform,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
    pub b_set_internal_from_material_index: bool,
    pub b_split_components: bool,
}
#[repr(C, align(16))]
pub struct FStaticMeshToCollectionDataflowNode {
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub mesh_transform: crate::bindings::core_u_object::FTransform,
    pub b_set_internal_from_material_index: bool,
    pub b_split_components: bool,
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub materials: TArray<UPtr<crate::bindings::engine::UMaterial>>,
    pub material_instances: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub instanced_meshes: TArray<
        crate::bindings::geometry_collection_engine::FGeometryCollectionAutoInstanceMesh,
    >,
}
#[repr(C, align(8))]
pub struct FBakeTextureFromCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub image: crate::bindings::dataflow_core::FDataflowImage,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub resolution: crate::bindings::dataflow_core::EDataflowImageResolution,
    pub gutter_size: i32,
    pub uv_channel: i32,
    pub red_channel: ECollectionBakeTextureAttribute,
    pub green_channel: ECollectionBakeTextureAttribute,
    pub blue_channel: ECollectionBakeTextureAttribute,
    pub alpha_channel: ECollectionBakeTextureAttribute,
    pub max_distance: f32,
    pub occlusion_rays: i32,
    pub occlusion_blur_radius: f32,
    pub curvature_blur_radius: f32,
    pub voxel_resolution: i32,
    pub smoothing_iterations: i32,
    pub thickness_factor: f32,
    pub max_curvature: f32,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionTransferVertexAttributeNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub from_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub transfer_method: EDataflowTransferVertexAttributeNodeTransferMethod,
    pub bounding_volume_type: EDataflowTransferVertexAttributeNodeBoundingVolume,
    pub source_scale: EDataflowTransferVertexAttributeNodeSourceScale,
    pub falloff: EDataflowTransferVertexAttributeNodeFalloff,
    pub falloff_threshold: f32,
    pub edge_multiplier: f32,
    pub bound_multiplier: f32,
    pub transform_name_suffix: FString,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionTransferVertexSkinWeightsNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub from_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transfer_method: EDataflowTransferVertexAttributeNodeTransferMethod,
    pub bounding_volume_type: EDataflowTransferVertexAttributeNodeBoundingVolume,
    pub source_scale: EDataflowTransferVertexAttributeNodeSourceScale,
    pub falloff: EDataflowTransferVertexAttributeNodeFalloff,
    pub falloff_threshold: f32,
    pub edge_multiplier: f32,
    pub bound_multiplier: f32,
    pub transform_name_suffix: FString,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionSetKinematicVertexSelectionNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub kinematic_value: ESetKinematicVertexSelectionKinematicValue,
}
#[repr(C, align(8))]
pub struct FTriangleBoundaryIndicesNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub boundary_indices_out: TArray<i32>,
}
#[repr(C, align(4))]
pub struct FDataflowConvexDecompositionSettings {
    pub min_size_to_decompose: f32,
    pub max_geo_to_hull_volume_ratio_to_decompose: f32,
    pub error_tolerance: f32,
    pub max_hulls_per_geometry: i32,
    pub min_thickness_tolerance: f32,
    pub num_additional_splits: i32,
    pub b_protect_negative_space: bool,
    pub b_only_connected_to_hull: bool,
    pub negative_space_tolerance: f32,
    pub negative_space_min_radius: f32,
}
#[repr(C, align(8))]
pub struct FMakeDataflowConvexDecompositionSettingsNode {
    pub min_size_to_decompose: f32,
    pub max_geo_to_hull_volume_ratio_to_decompose: f32,
    pub error_tolerance: f32,
    pub max_hulls_per_geometry: i32,
    pub min_thickness_tolerance: f32,
    pub num_additional_splits: i32,
    pub b_protect_negative_space: bool,
    pub b_only_connected_to_hull: bool,
    pub negative_space_tolerance: f32,
    pub negative_space_min_radius: f32,
    pub decomposition_settings: FDataflowConvexDecompositionSettings,
}
#[repr(C, align(8))]
pub struct FCreateLeafConvexHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub generate_method: crate::bindings::chaos::EGenerateConvexMethod,
    pub intersect_if_computed_is_smaller_by_factor: f32,
    pub min_external_volume_to_intersect: f32,
    pub b_compute_intersections_before_hull: bool,
    pub simplification_distance_threshold: f32,
    pub convex_decomposition_settings: FDataflowConvexDecompositionSettings,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FSimplifyConvexHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub simplify_method: crate::bindings::fracture_engine::EConvexHullSimplifyMethod,
    pub simplification_angle_threshold: f32,
    pub simplification_distance_threshold: f32,
    pub min_target_triangle_count: i32,
    pub b_use_existing_vertices: bool,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FCreateNonOverlappingConvexHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub can_exceed_fraction: f32,
    pub simplification_distance_threshold: f32,
    pub overlap_removal_method: EConvexOverlapRemovalMethodEnum,
    pub overlap_removal_shrink_percent: f32,
    pub can_remove_fraction: f32,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FGenerateClusterConvexHullsFromLeafHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub convex_count: i32,
    pub error_tolerance: f64,
    pub b_prefer_external_collision_shapes: bool,
    pub allow_merges: crate::bindings::chaos::EAllowConvexMergeMethod,
    pub merge_proximity_filter: crate::bindings::chaos::EConvexHullProximityFilter,
    pub merge_proximity_distance_threshold: f32,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_allow_merging_leaf_hulls: bool,
    pub b_protect_negative_space: bool,
    pub sample_method: ENegativeSpaceSampleMethodDataflowEnum,
    pub b_require_search_sample_coverage: bool,
    pub b_only_connected_to_hull: bool,
    pub target_num_samples: i32,
    pub min_sample_spacing: f64,
    pub negative_space_tolerance: f64,
    pub min_radius: f64,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FGenerateClusterConvexHullsFromChildrenHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub convex_count: i32,
    pub error_tolerance: f64,
    pub b_prefer_external_collision_shapes: bool,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub merge_proximity_filter: crate::bindings::chaos::EConvexHullProximityFilter,
    pub merge_proximity_distance_threshold: f32,
    pub b_allow_merging_leaf_hulls: bool,
    pub b_protect_negative_space: bool,
    pub sample_method: ENegativeSpaceSampleMethodDataflowEnum,
    pub b_require_search_sample_coverage: bool,
    pub b_only_connected_to_hull: bool,
    pub target_num_samples: i32,
    pub min_sample_spacing: f64,
    pub negative_space_tolerance: f64,
    pub min_radius: f64,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FClearConvexHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCopyConvexHullsFromRootDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub from_collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_skip_if_empty: bool,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FMergeConvexHullsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub max_convex_count: i32,
    pub error_tolerance: f64,
    pub optional_selection_filter: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub merge_proximity_filter: crate::bindings::chaos::EConvexHullProximityFilter,
    pub merge_proximity_distance_threshold: f32,
    pub b_protect_negative_space: bool,
    pub b_compute_negative_space_per_bone: bool,
    pub sample_method: ENegativeSpaceSampleMethodDataflowEnum,
    pub b_require_search_sample_coverage: bool,
    pub b_only_connected_to_hull: bool,
    pub target_num_samples: i32,
    pub min_sample_spacing: f64,
    pub negative_space_tolerance: f64,
    pub min_radius: f64,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FUpdateVolumeAttributesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetConvexHullVolumeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub volume: f32,
    pub b_sum_children_for_clusters_without_hulls: bool,
    pub b_volume_of_union: bool,
    pub debug_draw_render_settings: crate::bindings::dataflow_engine::FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FFixTinyGeoDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub merge_type: crate::bindings::fracture_engine::EFixTinyGeoMergeType,
    pub b_on_fracture_level: bool,
    pub b_only_clusters: bool,
    pub b_only_same_parent: bool,
    pub b_geometry_only_same_parent: bool,
    pub b_fracture_level_is_all: bool,
    pub neighbor_selection: crate::bindings::fracture_engine::EFixTinyGeoNeighborSelectionMethod,
    pub b_only_to_connected: bool,
    pub b_use_collection_proximity_for_connections: bool,
    pub use_bone_selection: crate::bindings::fracture_engine::EFixTinyGeoUseBoneSelection,
    pub selection_method: crate::bindings::fracture_engine::EFixTinyGeoGeometrySelectionMethod,
    pub min_volume_cube_root: f32,
    pub relative_volume: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(8))]
pub struct FRecomputeNormalsInGeometryCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub b_only_tangents: bool,
    pub b_recompute_sharp_edges: bool,
    pub sharp_edge_angle_threshold: f32,
    pub b_only_internal_surfaces: bool,
}
#[repr(C, align(8))]
pub struct FResampleGeometryCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_selection: crate::bindings::dataflow_core::FDataflowTransformSelection,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(8))]
pub struct FValidateGeometryCollectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub b_remove_unreferenced_geometry: bool,
    pub b_remove_clusters_of_one: bool,
    pub b_remove_dangling_clusters: bool,
}
#[repr(C, align(8))]
pub struct FAddUVChannelDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub uv_channel: i32,
    pub default_value: crate::bindings::core_u_object::FVector2f,
}
#[repr(C, align(8))]
pub struct FAutoUnwrapUVDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub uv_channel: i32,
    pub gutter_size: i32,
}
#[repr(C, align(8))]
pub struct FMergeUVIslandsDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub face_selection: crate::bindings::dataflow_core::FDataflowFaceSelection,
    pub uv_channel: i32,
    pub area_distortion_threshold: f64,
    pub max_normal_deviation_deg: f64,
    pub normal_smoothing_rounds: i32,
    pub normal_smoothing_alpha: f64,
}
#[repr(C, align(8))]
pub struct FBoxProjectUVDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub uv_channel: i32,
    pub gutter_size: i32,
    pub projection_scale: crate::bindings::core_u_object::FVector,
    pub uv_offset: crate::bindings::core_u_object::FVector2f,
    pub b_auto_fit_to_bounds: bool,
    pub b_center_box_at_pivot: bool,
    pub b_uniform_projection_scale: bool,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionVertexScalarToVertexIndicesNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub attribute_key: crate::bindings::dataflow_engine_plugin::FCollectionAttributeKey,
    pub selection_threshold: f32,
    pub vertex_indices: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FTransformCollectionAttributeDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub transform_in: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub group_name: FString,
    pub attribute_name: FString,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromFloatArrayDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub float_array: TArray<f32>,
    pub b_normalize_input: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromVertexIndicesDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_indices_in: TArray<i32>,
    pub selected_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromVertexSelectionDataflowNode {
    pub collection: crate::bindings::chaos::FManagedArrayCollection,
    pub vertex_selection: crate::bindings::dataflow_core::FDataflowVertexSelection,
    pub selected_color: crate::bindings::core_u_object::FLinearColor,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFloatArrayToIntArrayFunctionEnum(pub u8);
impl EFloatArrayToIntArrayFunctionEnum {
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_FLOOR: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        0,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_CEIL: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        1,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ROUND: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        2,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_TRUNCATE: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        3,
    );
    pub const DATAFLOW_FLOAT_TO_INT_NON_ZERO_TO_INDEX: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        4,
    );
    pub const DATAFLOW_FLOAT_TO_INT_ZERO_TO_INDEX: EFloatArrayToIntArrayFunctionEnum = EFloatArrayToIntArrayFunctionEnum(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECompareOperation1Enum(pub u8);
impl ECompareOperation1Enum {
    pub const DATAFLOW_COMPARE_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(0);
    pub const DATAFLOW_COMPARE_SMALLER: ECompareOperation1Enum = ECompareOperation1Enum(
        1,
    );
    pub const DATAFLOW_COMPARE_SMALLER_OR_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(
        2,
    );
    pub const DATAFLOW_COMPARE_GREATER: ECompareOperation1Enum = ECompareOperation1Enum(
        3,
    );
    pub const DATAFLOW_COMPARE_GREATER_OR_EQUAL: ECompareOperation1Enum = ECompareOperation1Enum(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStatisticsOperationEnum(pub u8);
impl EStatisticsOperationEnum {
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MIN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        0,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MEAN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        2,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MEDIAN: EStatisticsOperationEnum = EStatisticsOperationEnum(
        3,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_MODE: EStatisticsOperationEnum = EStatisticsOperationEnum(
        4,
    );
    pub const DATAFLOW_E_STATISTICS_OPERATION_ENUM_SUM: EStatisticsOperationEnum = EStatisticsOperationEnum(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClusterSizeMethodEnum(pub u8);
impl EClusterSizeMethodEnum {
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_NUMBER: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        0,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_FRACTION_OF_INPUT: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        1,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_SIZE: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        2,
    );
    pub const DATAFLOW_CLUSTER_SIZE_METHOD_BY_GRID: EClusterSizeMethodEnum = EClusterSizeMethodEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EClusterNeighborSelectionMethodEnum(pub u8);
impl EClusterNeighborSelectionMethodEnum {
    pub const DATAFLOW_CLUSTER_NEIGHBOR_SELECTION_METHOD_LARGEST_NEIGHBOR: EClusterNeighborSelectionMethodEnum = EClusterNeighborSelectionMethodEnum(
        0,
    );
    pub const DATAFLOW_CLUSTER_NEIGHBOR_SELECTION_METHOD_NEAREST_CENTER: EClusterNeighborSelectionMethodEnum = EClusterNeighborSelectionMethodEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFloatToIntFunctionEnum(pub u8);
impl EFloatToIntFunctionEnum {
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_FLOOR: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        0,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_CEIL: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        1,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ROUND: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        2,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_TRUNCATE: EFloatToIntFunctionEnum = EFloatToIntFunctionEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVisibiltyOptionsEnum(pub u8);
impl EVisibiltyOptionsEnum {
    pub const DATAFLOW_VISIBILITY_OPTIONS_VISIBLE: EVisibiltyOptionsEnum = EVisibiltyOptionsEnum(
        0,
    );
    pub const DATAFLOW_VISIBILITY_OPTIONS_INVISIBLE: EVisibiltyOptionsEnum = EVisibiltyOptionsEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowFieldFalloffType(pub u8);
impl EDataflowFieldFalloffType {
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_NONE: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        0,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_LINEAR: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        1,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_INVERSE: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        2,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_SQUARED: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        3,
    );
    pub const DATAFLOW_FIELD_FALLOFF_TYPE_LOGARITHMIC: EDataflowFieldFalloffType = EDataflowFieldFalloffType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowSetMaskConditionType(pub u8);
impl EDataflowSetMaskConditionType {
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_ALWAYS: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        0,
    );
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_IFF_NOT_INTERIOR: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        1,
    );
    pub const DATAFLOW_SET_MASK_CONDITION_TYPE_IFF_NOT_EXTERIOR: EDataflowSetMaskConditionType = EDataflowSetMaskConditionType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowWaveFunctionType(pub u8);
impl EDataflowWaveFunctionType {
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_COSINE: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        0,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_GAUSSIAN: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        1,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_FALLOFF: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        2,
    );
    pub const DATAFLOW_WAVE_FUNCTION_TYPE_DECAY: EDataflowWaveFunctionType = EDataflowWaveFunctionType(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowFloatFieldOperationType(pub u8);
impl EDataflowFloatFieldOperationType {
    pub const DATAFLOW_FLOAT_FIELD_OPERATION_TYPE_MULTIPLY: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        0,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_DIVIDE: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        1,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_ADD: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        2,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_SUBSTRACT: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        3,
    );
    pub const DATAFLOW_FLOAT_FIELD_FALLOFF_TYPE_MIN: EDataflowFloatFieldOperationType = EDataflowFloatFieldOperationType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowVectorFieldOperationType(pub u8);
impl EDataflowVectorFieldOperationType {
    pub const DATAFLOW_VECTOR_FIELD_OPERATION_TYPE_MULTIPLY: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        0,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_DIVIDE: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        1,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_ADD: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        2,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_SUBSTRACT: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        3,
    );
    pub const DATAFLOW_VECTOR_FIELD_FALLOFF_TYPE_CROSS_PRODUCT: EDataflowVectorFieldOperationType = EDataflowVectorFieldOperationType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowVisualizeFractureColoringType(pub u8);
impl EDataflowVisualizeFractureColoringType {
    pub const COLOR_BY_PARENT: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        0,
    );
    pub const COLOR_BY_LEVEL: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        1,
    );
    pub const COLOR_BY_CLUSTER: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        2,
    );
    pub const COLOR_BY_LEAF_LEVEL: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        3,
    );
    pub const COLOR_BY_LEAF: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        4,
    );
    pub const COLOR_BY_ATTR: EDataflowVisualizeFractureColoringType = EDataflowVisualizeFractureColoringType(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowSetFloatArrayMethod(pub u8);
impl EDataflowSetFloatArrayMethod {
    pub const RANDOM: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(0);
    pub const NOISE: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(1);
    pub const BY_BOUNDING_BOX: EDataflowSetFloatArrayMethod = EDataflowSetFloatArrayMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMakeBoxDataTypeEnum(pub u8);
impl EMakeBoxDataTypeEnum {
    pub const DATAFLOW_MAKE_BOX_DATA_TYPE_MIN_MAX: EMakeBoxDataTypeEnum = EMakeBoxDataTypeEnum(
        0,
    );
    pub const DATAFLOW_MAKE_BOX_DATA_TYPE_CENTER_SIZE: EMakeBoxDataTypeEnum = EMakeBoxDataTypeEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowStairTypeEnum(pub u8);
impl EDataflowStairTypeEnum {
    pub const LINEAR: EDataflowStairTypeEnum = EDataflowStairTypeEnum(0);
    pub const FLOATING: EDataflowStairTypeEnum = EDataflowStairTypeEnum(1);
    pub const CURVED: EDataflowStairTypeEnum = EDataflowStairTypeEnum(2);
    pub const SPIRAL: EDataflowStairTypeEnum = EDataflowStairTypeEnum(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESetMaterialOperationTypeEnum(pub u8);
impl ESetMaterialOperationTypeEnum {
    pub const DATAFLOW_SET_MATERIAL_OPERATION_TYPE_ADD: ESetMaterialOperationTypeEnum = ESetMaterialOperationTypeEnum(
        0,
    );
    pub const DATAFLOW_SET_MATERIAL_OPERATION_TYPE_INSERT: ESetMaterialOperationTypeEnum = ESetMaterialOperationTypeEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMathConstantsEnum(pub u8);
impl EMathConstantsEnum {
    pub const DATAFLOW_MATH_CONSTANTS_PI: EMathConstantsEnum = EMathConstantsEnum(0);
    pub const DATAFLOW_MATH_CONSTANTS_HALF_PI: EMathConstantsEnum = EMathConstantsEnum(
        1,
    );
    pub const DATAFLOW_MATH_CONSTANTS_TWO_PI: EMathConstantsEnum = EMathConstantsEnum(2);
    pub const DATAFLOW_MATH_CONSTANTS_FOUR_PI: EMathConstantsEnum = EMathConstantsEnum(
        3,
    );
    pub const DATAFLOW_MATH_CONSTANTS_INV_PI: EMathConstantsEnum = EMathConstantsEnum(4);
    pub const DATAFLOW_MATH_CONSTANTS_INV_TWO_PI: EMathConstantsEnum = EMathConstantsEnum(
        5,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT2: EMathConstantsEnum = EMathConstantsEnum(6);
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT2: EMathConstantsEnum = EMathConstantsEnum(
        7,
    );
    pub const DATAFLOW_MATH_CONSTANTS_SQRT3: EMathConstantsEnum = EMathConstantsEnum(8);
    pub const DATAFLOW_MATH_CONSTANTS_INV_SQRT3: EMathConstantsEnum = EMathConstantsEnum(
        9,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_E: EMathConstantsEnum = EMathConstantsEnum(
        10,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_GAMMA: EMathConstantsEnum = EMathConstantsEnum(
        11,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_GOLDEN_RATIO: EMathConstantsEnum = EMathConstantsEnum(
        12,
    );
    pub const DATAFLOW_FLOAT_TO_INT_FUNCTION_ZERO_TOLERANCE: EMathConstantsEnum = EMathConstantsEnum(
        13,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowMeshSplitIslandsMethod(pub u8);
impl EDataflowMeshSplitIslandsMethod {
    pub const NO_SPLIT: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        0,
    );
    pub const BY_MESH_TOPOLOGY: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        1,
    );
    pub const BY_VERTEX_OVERLAP: EDataflowMeshSplitIslandsMethod = EDataflowMeshSplitIslandsMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBoxLengthMeasurementMethod(pub u8);
impl EBoxLengthMeasurementMethod {
    pub const X_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(0);
    pub const Y_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(1);
    pub const Z_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(2);
    pub const SHORTEST_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(
        3,
    );
    pub const LONGEST_AXIS: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(4);
    pub const DIAGONAL: EBoxLengthMeasurementMethod = EBoxLengthMeasurementMethod(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERotationOrderEnum(pub u8);
impl ERotationOrderEnum {
    pub const DATAFLOW_ROTATION_ORDER_XYZ: ERotationOrderEnum = ERotationOrderEnum(0);
    pub const DATAFLOW_ROTATION_ORDER_YZX: ERotationOrderEnum = ERotationOrderEnum(1);
    pub const DATAFLOW_ROTATION_ORDER_ZXY: ERotationOrderEnum = ERotationOrderEnum(2);
    pub const DATAFLOW_ROTATION_ORDER_XZY: ERotationOrderEnum = ERotationOrderEnum(3);
    pub const DATAFLOW_ROTATION_ORDER_YXZ: ERotationOrderEnum = ERotationOrderEnum(4);
    pub const DATAFLOW_ROTATION_ORDER_ZYX: ERotationOrderEnum = ERotationOrderEnum(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECompareOperationEnum(pub u8);
impl ECompareOperationEnum {
    pub const DATAFLOW_COMPARE_EQUAL: ECompareOperationEnum = ECompareOperationEnum(0);
    pub const DATAFLOW_COMPARE_SMALLER: ECompareOperationEnum = ECompareOperationEnum(1);
    pub const DATAFLOW_COMPARE_SMALLER_OR_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        2,
    );
    pub const DATAFLOW_COMPARE_GREATER: ECompareOperationEnum = ECompareOperationEnum(3);
    pub const DATAFLOW_COMPARE_GREATER_OR_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        4,
    );
    pub const DATAFLOW_COMPARE_NOT_EQUAL: ECompareOperationEnum = ECompareOperationEnum(
        5,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBooleanOperationEnum(pub u8);
impl EBooleanOperationEnum {
    pub const DATAFLOW_AND: EBooleanOperationEnum = EBooleanOperationEnum(0);
    pub const DATAFLOW_OR: EBooleanOperationEnum = EBooleanOperationEnum(1);
    pub const DATAFLOW_NOT: EBooleanOperationEnum = EBooleanOperationEnum(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAnchorStateEnum(pub u8);
impl EAnchorStateEnum {
    pub const DATAFLOW_ANCHOR_STATE_ANCHORED: EAnchorStateEnum = EAnchorStateEnum(0);
    pub const DATAFLOW_ANCHOR_STATE_NOT_ANCHORED: EAnchorStateEnum = EAnchorStateEnum(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowGeometryCollectionDynamicState(pub u8);
impl EDataflowGeometryCollectionDynamicState {
    pub const NONE: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        0,
    );
    pub const DYNAMIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        1,
    );
    pub const KINEMATIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        2,
    );
    pub const STATIC: EDataflowGeometryCollectionDynamicState = EDataflowGeometryCollectionDynamicState(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProximityMethodEnum(pub u8);
impl EProximityMethodEnum {
    pub const DATAFLOW_PROXIMITY_METHOD_PRECISE: EProximityMethodEnum = EProximityMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_METHOD_CONVEX_HULL: EProximityMethodEnum = EProximityMethodEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EProximityContactFilteringMethodEnum(pub u8);
impl EProximityContactFilteringMethodEnum {
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_PROJECTED_BOUNDS_OVERLAP: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_SHARP: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        1,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_AREA: EProximityContactFilteringMethodEnum = EProximityContactFilteringMethodEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConnectionContactAreaMethodEnum(pub u8);
impl EConnectionContactAreaMethodEnum {
    pub const DATAFLOW_CONNECTION_CONTACT_AREA_METHOD_NONE: EConnectionContactAreaMethodEnum = EConnectionContactAreaMethodEnum(
        0,
    );
    pub const DATAFLOW_PROXIMITY_CONTACT_FILTERING_METHOD_CONVEX_HULL_AREA: EConnectionContactAreaMethodEnum = EConnectionContactAreaMethodEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EStandardGroupNameEnum(pub u8);
impl EStandardGroupNameEnum {
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_TRANSFORM: EStandardGroupNameEnum = EStandardGroupNameEnum(
        0,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_GEOMETRY: EStandardGroupNameEnum = EStandardGroupNameEnum(
        1,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_FACES: EStandardGroupNameEnum = EStandardGroupNameEnum(
        2,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_VERTICES: EStandardGroupNameEnum = EStandardGroupNameEnum(
        3,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_MATERIAL: EStandardGroupNameEnum = EStandardGroupNameEnum(
        4,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_BREAKING: EStandardGroupNameEnum = EStandardGroupNameEnum(
        5,
    );
    pub const DATAFLOW_E_STANDARD_GROUP_NAME_ENUM_CUSTOM: EStandardGroupNameEnum = EStandardGroupNameEnum(
        6,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECustomAttributeTypeEnum(pub u8);
impl ECustomAttributeTypeEnum {
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_U_INT8: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        0,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        1,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_FLOAT: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        2,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_DOUBLE: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        3,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_BOOL: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        4,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_STRING: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        5,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR2F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        6,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR3F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        7,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR3D: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        8,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR4F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        9,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_LINEAR_COLOR: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        10,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_TRANSFORM: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        11,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_QUAT4F: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        12,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_BOX: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        13,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_GUID: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        14,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32_SET: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        15,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT32_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        16,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        17,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR2: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        18,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR4: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        19,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_INT_VECTOR2_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        20,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_FLOAT_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        21,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_VECTOR2F_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        22,
    );
    pub const DATAFLOW_CUSTOM_ATTRIBUTE_TYPE_F_VECTOR3F_ARRAY: ECustomAttributeTypeEnum = ECustomAttributeTypeEnum(
        23,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESetOperationEnum(pub u8);
impl ESetOperationEnum {
    pub const DATAFLOW_SET_OPERATION_AND: ESetOperationEnum = ESetOperationEnum(0);
    pub const DATAFLOW_SET_OPERATION_OR: ESetOperationEnum = ESetOperationEnum(1);
    pub const DATAFLOW_SET_OPERATION_XOR: ESetOperationEnum = ESetOperationEnum(2);
    pub const DATAFLOW_SET_OPERATION_SUBTRACT: ESetOperationEnum = ESetOperationEnum(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ERangeSettingEnum(pub u8);
impl ERangeSettingEnum {
    pub const DATAFLOW_RANGE_SETTING_INSIDE_RANGE: ERangeSettingEnum = ERangeSettingEnum(
        0,
    );
    pub const DATAFLOW_RANGE_SETTING_OUTSIDE_RANGE: ERangeSettingEnum = ERangeSettingEnum(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESelectSubjectTypeEnum(pub u8);
impl ESelectSubjectTypeEnum {
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_VERTICES: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        0,
    );
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_BOUNDING_BOX: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        1,
    );
    pub const DATAFLOW_SELECT_SUBJECT_TYPE_CENTROID: ESelectSubjectTypeEnum = ESelectSubjectTypeEnum(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESelectionByAttrGroup(pub u8);
impl ESelectionByAttrGroup {
    pub const VERTICES: ESelectionByAttrGroup = ESelectionByAttrGroup(0);
    pub const FACES: ESelectionByAttrGroup = ESelectionByAttrGroup(1);
    pub const TRANSFORM: ESelectionByAttrGroup = ESelectionByAttrGroup(2);
    pub const GEOMETRY: ESelectionByAttrGroup = ESelectionByAttrGroup(3);
    pub const MATERIAL: ESelectionByAttrGroup = ESelectionByAttrGroup(4);
    pub const CURVES: ESelectionByAttrGroup = ESelectionByAttrGroup(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESelectionByAttrOperation(pub u8);
impl ESelectionByAttrOperation {
    pub const EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(0);
    pub const NOT_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(1);
    pub const GREATER: ESelectionByAttrOperation = ESelectionByAttrOperation(2);
    pub const GREATER_OR_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(3);
    pub const SMALLER: ESelectionByAttrOperation = ESelectionByAttrOperation(4);
    pub const SMALLER_OR_EQUAL: ESelectionByAttrOperation = ESelectionByAttrOperation(5);
    pub const MAXIMUM: ESelectionByAttrOperation = ESelectionByAttrOperation(6);
    pub const MINIMUM: ESelectionByAttrOperation = ESelectionByAttrOperation(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowCollectionSelectionByNameMethod(pub u8);
impl EDataflowCollectionSelectionByNameMethod {
    pub const EXACT: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        0,
    );
    pub const STARTS_WITH: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        1,
    );
    pub const ENDS_WITH: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        2,
    );
    pub const CONTAINS: EDataflowCollectionSelectionByNameMethod = EDataflowCollectionSelectionByNameMethod(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECollectionBakeTextureAttribute(pub i32);
impl ECollectionBakeTextureAttribute {
    pub const NONE: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(0);
    pub const DISTANCE_TO_EXTERNAL: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        1,
    );
    pub const AMBIENT_OCCLUSION: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        2,
    );
    pub const CURVATURE: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        3,
    );
    pub const NORMAL_X: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        4,
    );
    pub const NORMAL_Y: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        5,
    );
    pub const NORMAL_Z: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        6,
    );
    pub const POSITION_X: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        7,
    );
    pub const POSITION_Y: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        8,
    );
    pub const POSITION_Z: ECollectionBakeTextureAttribute = ECollectionBakeTextureAttribute(
        9,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeTransferMethod(pub u8);
impl EDataflowTransferVertexAttributeNodeTransferMethod {
    pub const COMPONENT: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        0,
    );
    pub const GLOBAL: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        1,
    );
    pub const NONE: EDataflowTransferVertexAttributeNodeTransferMethod = EDataflowTransferVertexAttributeNodeTransferMethod(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeBoundingVolume(pub u8);
impl EDataflowTransferVertexAttributeNodeBoundingVolume {
    pub const VERTEX: EDataflowTransferVertexAttributeNodeBoundingVolume = EDataflowTransferVertexAttributeNodeBoundingVolume(
        0,
    );
    pub const TRIANGLE: EDataflowTransferVertexAttributeNodeBoundingVolume = EDataflowTransferVertexAttributeNodeBoundingVolume(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeSourceScale(pub u8);
impl EDataflowTransferVertexAttributeNodeSourceScale {
    pub const COMPONENT_EDGE: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        0,
    );
    pub const ASSET_EDGE: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        1,
    );
    pub const ASSET_BOUND: EDataflowTransferVertexAttributeNodeSourceScale = EDataflowTransferVertexAttributeNodeSourceScale(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDataflowTransferVertexAttributeNodeFalloff(pub u8);
impl EDataflowTransferVertexAttributeNodeFalloff {
    pub const SQUARED: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        0,
    );
    pub const LINEAR: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        1,
    );
    pub const NONE: EDataflowTransferVertexAttributeNodeFalloff = EDataflowTransferVertexAttributeNodeFalloff(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESetKinematicVertexSelectionKinematicValue(pub u8);
impl ESetKinematicVertexSelectionKinematicValue {
    pub const SET_KINEMATIC: ESetKinematicVertexSelectionKinematicValue = ESetKinematicVertexSelectionKinematicValue(
        0,
    );
    pub const SET_NON_KINEMATIC: ESetKinematicVertexSelectionKinematicValue = ESetKinematicVertexSelectionKinematicValue(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConvexOverlapRemovalMethodEnum(pub u8);
impl EConvexOverlapRemovalMethodEnum {
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_NONE: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        0,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ALL: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        1,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ONLY_CLUSTERS: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        2,
    );
    pub const DATAFLOW_E_CONVEX_OVERLAP_REMOVAL_METHOD_ONLY_CLUSTERS_VS_CLUSTERS: EConvexOverlapRemovalMethodEnum = EConvexOverlapRemovalMethodEnum(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENegativeSpaceSampleMethodDataflowEnum(pub u8);
impl ENegativeSpaceSampleMethodDataflowEnum {
    pub const UNIFORM: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        0,
    );
    pub const VOXEL_SEARCH: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        1,
    );
    pub const NAVIGABLE_VOXEL_SEARCH: ENegativeSpaceSampleMethodDataflowEnum = ENegativeSpaceSampleMethodDataflowEnum(
        2,
    );
}
