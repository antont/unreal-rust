#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCreateColorArrayFromFloatArrayDataflowNode {
    pub float_array: TArray<f32>,
    pub color_array: TArray<FLinearColor>,
    pub b_normalize_input: bool,
    pub color: FLinearColor,
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
    pub points: TArray<FVector>,
    pub point: FVector,
}
#[repr(C, align(8))]
pub struct FGetNumArrayElementsDataflowNode {
    pub float_array: TArray<f32>,
    pub int_array: TArray<i32>,
    pub points: TArray<FVector>,
    pub vector3f_array: TArray<FVector3f>,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FBoolArrayToFaceSelectionDataflowNode {
    pub bool_attribute_data: TArray<bool>,
    pub face_selection: FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FFloatArrayToVertexSelectionDataflowNode {
    pub float_array: TArray<f32>,
    pub operation: ECompareOperation1Enum,
    pub threshold: f32,
    pub vertex_selection: FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FFloatArrayNormalizeDataflowNode {
    pub in_float_array: TArray<f32>,
    pub selection: FDataflowVertexSelection,
    pub min_range: f32,
    pub max_range: f32,
    pub out_float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FVectorArrayNormalizeDataflowNode {
    pub in_vector_array: TArray<FVector>,
    pub selection: FDataflowVertexSelection,
    pub magnitude: f32,
    pub out_vector_array: TArray<FVector>,
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
    pub transform_selection: FDataflowTransformSelection,
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
    pub array: FDataflowArrayTypes,
    pub size: i32,
}
#[repr(C, align(8))]
pub struct FDataflowGetArrayElementNode {
    pub array: FDataflowArrayTypes,
    pub index: i32,
    pub element: FDataflowAllTypes,
}
#[repr(C, align(8))]
pub struct FDataflowConvertToArrayNode {
    pub array: FDataflowArrayTypes,
    pub element: FDataflowAllTypes,
}
#[repr(C, align(8))]
pub struct FDataflowMakeManagedArrayCollectionArrayNode {
    pub array: TArray<FManagedArrayCollection>,
    pub element: FManagedArrayCollection,
}
#[repr(C, align(16))]
pub struct FDataflowRootProxyMesh {
    pub mesh: UPtr<UStaticMesh>,
    pub transform: FTransform,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(16))]
pub struct FMakeRootProxyMeshDataflowNode {
    pub mesh: UPtr<UStaticMesh>,
    pub transform: FTransform,
    pub override_materials: TArray<UPtr<UMaterialInterface>>,
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
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionTerminalDataflowNode {
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub material_instances: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
}
#[repr(C, align(8))]
pub struct FGetGeometryCollectionAssetDataflowNode {
    pub asset: UPtr<UGeometryCollection>,
}
#[repr(C, align(8))]
pub struct FGetGeometryCollectionSourcesDataflowNode {
    pub asset: UPtr<UGeometryCollection>,
    pub sources: TArray<FGeometryCollectionSource>,
}
#[repr(C, align(8))]
pub struct FCreateGeometryCollectionFromSourcesDataflowNode {
    pub sources: TArray<FGeometryCollectionSource>,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub material_instances: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
}
#[repr(C, align(8))]
pub struct FCreateGeometryCollectionFromSourcesDataflowNode_v2 {
    pub sources: TArray<FGeometryCollectionSource>,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionToCollectionDataflowNode {
    pub geometry_collection: UPtr<UGeometryCollection>,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub material_instances: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionToCollectionDataflowNode_v2 {
    pub geometry_collection: UPtr<UGeometryCollection>,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
}
#[repr(C, align(8))]
pub struct FBlueprintToCollectionDataflowNode {
    pub blueprint: UPtr<UBlueprint>,
    pub b_split_components: bool,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub material_instances: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
}
#[repr(C, align(8))]
pub struct FBlueprintToCollectionDataflowNode_v2 {
    pub blueprint: UPtr<UBlueprint>,
    pub b_split_components: bool,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
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
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub color: FLinearColor,
    pub line_width_multiplier: f32,
    pub center_color: FLinearColor,
    pub center_size: f32,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FClusterFlattenDataflowNode {
    pub collection: FManagedArrayCollection,
    pub optional_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterUnclusterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterMergeToNeighborsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub neighbor_selection_method: EClusterNeighborSelectionMethodEnum,
    pub min_volume_cube_root: f32,
    pub b_only_to_connected: bool,
    pub b_only_same_parent: bool,
}
#[repr(C, align(8))]
pub struct FClusterMergeDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FClusterIsolatedRootsDataflowNode {
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FClusterMagnetDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub iterations: i32,
}
#[repr(C, align(8))]
pub struct FVectorToStringDataflowNode {
    pub vector: FVector,
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
    pub collection: FManagedArrayCollection,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub b_use_robust_hulls: bool,
    pub mesh: UPtr<UDynamicMesh>,
    pub meshes: TArray<UPtr<UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FSphereCoveringToMeshDataflowNode {
    pub sphere_covering: FDataflowSphereCovering,
    pub vertices_along_each_side: i32,
    pub mesh: UPtr<UDynamicMesh>,
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
    pub mesh: UPtr<UDynamicMesh>,
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
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FSetVisibilityInCollectionDataflowNode {
    pub visibility: EVisibiltyOptionsEnum,
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub face_selection: FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FMergeInCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FRadialFalloffFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub sphere: FSphere,
    pub translation: FVector,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(16))]
pub struct FBoxFalloffFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub box_: FBox,
    pub transform: FTransform,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FPlaneFalloffFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub position: FVector,
    pub normal: FVector,
    pub distance: f32,
    pub translation: FVector,
    pub magnitude: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub default: f32,
    pub falloff_type: EDataflowFieldFalloffType,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub field_selection_mask: FDataflowVertexSelection,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRadialIntMaskFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub sphere: FSphere,
    pub translation: FVector,
    pub interior_value: i32,
    pub exterior_value: i32,
    pub set_mask_condition_type: EDataflowSetMaskConditionType,
    pub field_int_result: TArray<i32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformScalarFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: f32,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformVectorFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: f32,
    pub direction: FVector,
    pub field_vector_result: TArray<FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRadialVectorFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: f32,
    pub position: FVector,
    pub field_vector_result: TArray<FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FRandomVectorFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: f32,
    pub field_vector_result: TArray<FVector>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(16))]
pub struct FNoiseFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub min_range: f32,
    pub max_range: f32,
    pub transform: FTransform,
    pub field_float_result: TArray<f32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FUniformIntegerFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: i32,
    pub field_int_result: TArray<i32>,
    pub field_remap: TArray<i32>,
    pub num_sample_positions: i32,
}
#[repr(C, align(8))]
pub struct FWaveScalarFieldDataflowNode {
    pub sample_positions: TArray<FVector3f>,
    pub sample_indices: FDataflowVertexSelection,
    pub magnitude: f32,
    pub position: FVector,
    pub translation: FVector,
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
    pub field_vector_left: TArray<FVector>,
    pub field_remap_left: TArray<i32>,
    pub field_vector_right: TArray<FVector>,
    pub field_remap_right: TArray<i32>,
    pub magnitude: f32,
    pub operation: EDataflowVectorFieldOperationType,
    pub b_swap_vector_inputs: bool,
    pub field_vector_result: TArray<FVector>,
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
    pub bounding_box: FBox,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FUniformScatterPointsDataflowNode_v2 {
    pub min_number_of_points: i32,
    pub max_number_of_points: i32,
    pub random_seed: i32,
    pub bounding_box: FBox,
    pub points: TArray<FVector>,
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
    pub bounding_box: FBox,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FRadialScatterPointsDataflowNode {
    pub center: FVector,
    pub normal: FVector,
    pub radius: f32,
    pub angular_steps: i32,
    pub radial_steps: i32,
    pub angle_offset: f32,
    pub variability: f32,
    pub random_seed: f32,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FRadialScatterPointsDataflowNode_v2 {
    pub bounding_box: FBox,
    pub center: FVector,
    pub normal: FVector,
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
    pub points: TArray<FVector>,
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
    pub bounding_box: FBox,
    pub points: TArray<FVector>,
}
#[repr(C, align(16))]
pub struct FTransformPointsDataflowNode {
    pub points: TArray<FVector>,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FAppendPointsDataflowNode {
    pub points_a: TArray<FVector>,
    pub points_b: TArray<FVector>,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FVoronoiFractureDataflowNode {
    pub collection: FManagedArrayCollection,
    pub points: TArray<FVector>,
    pub transform_selection: FDataflowTransformSelection,
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
    pub collection: FManagedArrayCollection,
    pub points: TArray<FVector>,
    pub transform_selection: FDataflowTransformSelection,
    pub transform: FTransform,
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
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FPlaneCutterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub transform_selection: FDataflowTransformSelection,
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
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub transform_selection: FDataflowTransformSelection,
    pub transform: FTransform,
    pub cut_planes: TArray<FTransform>,
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
    pub render_type: EDataflowDebugDrawRenderType,
    pub plane_size_multiplier: f32,
    pub b_translucent: bool,
    pub b_randomize_colors: bool,
    pub color_random_seed: i32,
    pub line_width_multiplier: f32,
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FExplodedViewDataflowNode {
    pub collection: FManagedArrayCollection,
    pub uniform_scale: f32,
    pub scale: FVector,
    pub offset: FVector,
}
#[repr(C, align(8))]
pub struct FSliceCutterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub transform_selection: FDataflowTransformSelection,
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
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FBrickCutterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub transform_selection: FDataflowTransformSelection,
    pub transform: FTransform,
    pub bond: EFractureBrickBondEnum,
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
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FMeshCutterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub transform_selection: FDataflowTransformSelection,
    pub transform: FTransform,
    pub cutting_dynamic_meshes: TArray<UPtr<UDynamicMesh>>,
    pub cutting_static_mesh: UPtr<UStaticMesh>,
    pub b_use_hi_res: bool,
    pub lod_level: i32,
    pub cut_distribution: EMeshCutterCutDistribution,
    pub per_cut_mesh_selection: EMeshCutterPerCutMeshSelection,
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
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FUniformFractureDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub transform: FTransform,
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
    pub new_geometry_transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(4))]
pub struct FMinSettings {
    pub min_attr_value: f32,
    pub min_color: FLinearColor,
}
#[repr(C, align(4))]
pub struct FMaxSettings {
    pub max_attr_value: f32,
    pub max_color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FVisualizeFractureDataflowNode {
    pub collection: FManagedArrayCollection,
    pub level: i32,
    pub random_seed: i32,
    pub b_apply_exploded_view: bool,
    pub explode_amount: f32,
    pub scale: FVector,
    pub b_apply_color: bool,
    pub coloring_type: EDataflowVisualizeFractureColoringType,
    pub random_color_range_min: i32,
    pub random_color_range_max: i32,
    pub attribute: FString,
    pub min: FMinSettings,
    pub max: FMaxSettings,
    pub offset: FVector,
}
#[repr(C, align(8))]
pub struct FSetFloatAttributeDataflowNode {
    pub collection: FManagedArrayCollection,
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
    pub point: TArray<FVector>,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FMakeBoxDataflowNode {
    pub data_type: EMakeBoxDataTypeEnum,
    pub min: FVector,
    pub max: FVector,
    pub center: FVector,
    pub size: FVector,
    pub box_: FBox,
}
#[repr(C, align(8))]
pub struct FMakeSphereDataflowNode {
    pub center: FVector,
    pub radius: f32,
    pub sphere: FSphere,
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
    pub vector: FVector,
}
#[repr(C, align(16))]
pub struct FMakeTransformDataflowNode {
    pub in_translation: FVector,
    pub in_rotation: FVector,
    pub in_scale: FVector,
    pub out_transform: FTransform,
}
#[repr(C, align(16))]
pub struct FMakeTransformDataflowNode_v2 {
    pub translation: FVector,
    pub rotation: FVector,
    pub rotator: FRotator,
    pub quat: FQuat,
    pub scale: FVector,
    pub transform: FTransform,
}
#[repr(C, align(16))]
pub struct FMakeQuaternionDataflowNode {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub quaternion: FQuat,
}
#[repr(C, align(8))]
pub struct FMakeFloatArrayDataflowNode {
    pub num_elements: i32,
    pub value: f32,
    pub float_array: TArray<f32>,
}
#[repr(C, align(8))]
pub struct FMakeCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub b_add_root_transform: bool,
}
#[repr(C, align(8))]
pub struct FMakeRotatorDataflowNode {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub rotator: FRotator,
}
#[repr(C, align(16))]
pub struct FBreakTransformDataflowNode {
    pub transform: FTransform,
    pub translation: FDataflowVectorTypes,
    pub rotation: FVector,
    pub rotator: FRotator,
    pub quat: FQuat,
    pub scale: FDataflowVectorTypes,
}
#[repr(C, align(8))]
pub struct FMakeSphereMeshDataflowNode {
    pub radius: f32,
    pub num_phi: i32,
    pub num_theta: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeCapsuleMeshDataflowNode {
    pub radius: f32,
    pub segment_length: f32,
    pub num_hemisphere_arc_steps: i32,
    pub num_circle_steps: i32,
    pub num_segment_steps: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeCylinderMeshDataflowNode {
    pub radius1: f32,
    pub radius2: f32,
    pub height: f32,
    pub length_samples: i32,
    pub angle_samples: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeBoxMeshDataflowNode {
    pub center: FVector,
    pub size: FVector,
    pub subdivisions_x: i32,
    pub subdivisions_y: i32,
    pub subdivisions_z: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(16))]
pub struct FMakePlaneDataflowNode {
    pub base_point: FVector,
    pub normal: FVector,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub plane_size_multiplier: f32,
    pub plane: FPlane,
}
#[repr(C, align(8))]
pub struct FMakeDiscMeshDataflowNode {
    pub radius: f32,
    pub normal: FVector,
    pub angle_samples: i32,
    pub radial_samples: i32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub mesh: UPtr<UDynamicMesh>,
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
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeRectangleMeshDataflowNode {
    pub origin: FVector,
    pub normal: FVector,
    pub width: f32,
    pub height: f32,
    pub width_vertex_count: i32,
    pub height_vertex_count: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeTorusMeshDataflowNode {
    pub origin: FVector,
    pub radius1: f32,
    pub profile_vertex_count: i32,
    pub radius2: f32,
    pub sweep_vertex_count: i32,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FMakeMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FGetMaterialInterfaceArraySizeDataflowNode {
    pub material_array: TArray<UPtr<UMaterialInterface>>,
    pub size: i32,
}
#[repr(C, align(8))]
pub struct FGetMaterialInterfaceAssetDataflowNode {
    pub material: UPtr<UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FGetFromMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<UMaterialInterface>>,
    pub material: UPtr<UMaterialInterface>,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FSetIntoMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<UMaterialInterface>>,
    pub material: UPtr<UMaterialInterface>,
    pub index: i32,
}
#[repr(C, align(8))]
pub struct FAddToMaterialInterfaceArrayDataflowNode {
    pub material_array: TArray<UPtr<UMaterialInterface>>,
    pub materials_to_add: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FAssignMaterialInterfaceToCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_selection: FDataflowFaceSelection,
    pub material_array: TArray<UPtr<UMaterialInterface>>,
    pub material: UPtr<UMaterialInterface>,
    pub material_index: i32,
    pub b_merge_duplicate_materials: bool,
}
#[repr(C, align(8))]
pub struct FMaterialInterfaceTextureOverrideDataflowNode {
    pub material: UPtr<UMaterialInterface>,
    pub target_texture: UPtr<UTexture2D>,
    pub override_texture: UPtr<UTexture2D>,
}
#[repr(C, align(8))]
pub struct FAddMaterialToCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_selection: FDataflowFaceSelection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub outside_material: UPtr<UMaterial>,
    pub inside_material: UPtr<UMaterial>,
    pub b_assign_outside_material: bool,
    pub b_assign_inside_material: bool,
}
#[repr(C, align(8))]
pub struct FReAssignMaterialInCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_selection: FDataflowFaceSelection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub outside_material_idx: i32,
    pub inside_material_idx: i32,
    pub b_assign_outside_material: bool,
    pub b_assign_inside_material: bool,
}
#[repr(C, align(8))]
pub struct FMaterialsInfoDataflowNode {
    pub materials: TArray<UPtr<UMaterial>>,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FGetMaterialFromMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<UMaterial>>,
    pub material: UPtr<UMaterial>,
    pub material_idx: i32,
}
#[repr(C, align(8))]
pub struct FSetMaterialInMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<UMaterial>>,
    pub material: UPtr<UMaterial>,
    pub operation: ESetMaterialOperationTypeEnum,
    pub material_idx: i32,
}
#[repr(C, align(8))]
pub struct FMakeMaterialDataflowNode {
    pub in_material: UPtr<UMaterial>,
    pub material: UPtr<UMaterial>,
}
#[repr(C, align(8))]
pub struct FMakeMaterialsArrayDataflowNode {
    pub materials: TArray<UPtr<UMaterial>>,
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
    pub vector: FVector,
    pub scale: f32,
    pub scaled_vector: FVector,
}
#[repr(C, align(8))]
pub struct FDotProductDataflowNode {
    pub vector_a: FVector,
    pub vector_b: FVector,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FCrossProductDataflowNode {
    pub vector_a: FVector,
    pub vector_b: FVector,
    pub return_value: FVector,
}
#[repr(C, align(8))]
pub struct FNormalizeDataflowNode {
    pub vector_a: FVector,
    pub tolerance: f32,
    pub return_value: FVector,
}
#[repr(C, align(8))]
pub struct FLengthDataflowNode {
    pub vector: FVector,
    pub return_value: f32,
}
#[repr(C, align(8))]
pub struct FDistanceDataflowNode {
    pub point_a: FVector,
    pub point_b: FVector,
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
    pub return_value: FVector,
}
#[repr(C, align(8))]
pub struct FRandomUnitVectorInConeDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub cone_direction: FVector,
    pub cone_half_angle: f32,
    pub return_value: FVector,
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
    pub a: FDataflowNumericTypes,
    pub b: FDataflowNumericTypes,
    pub c: FDataflowNumericTypes,
    pub d: FDataflowNumericTypes,
    pub expression: FString,
    pub return_value: FDataflowNumericTypes,
}
#[repr(C, align(8))]
pub struct FPointsToMeshDataflowNode {
    pub points: TArray<FVector>,
    pub mesh: UPtr<UDynamicMesh>,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FMeshProcessorDataflowNodeBase {
    pub mesh_processor: TSubclassOf<UDynamicMeshProcessorBlueprint>,
    pub mesh_processor_instance: UPtr<UDynamicMeshProcessorBlueprint>,
    pub owning_object: UPtr<UObject>,
    pub dynamic_connections: FDataflowDynamicConnections,
    pub property_bag: FInstancedPropertyBag,
}
#[repr(C, align(8))]
pub struct FApplyMeshProcessorToMeshDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FApplyMeshProcessorToGeometryCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionToMeshesDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub b_convert_selection_to_leaves: bool,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
    pub meshes: TArray<UPtr<UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FAppendMeshesToCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub added_selection: FDataflowTransformSelection,
    pub meshes: TArray<UPtr<UDynamicMesh>>,
    pub parent_index: i32,
}
#[repr(C, align(8))]
pub struct FBoxToMeshDataflowNode {
    pub box_: FBox,
    pub mesh: UPtr<UDynamicMesh>,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FMeshInfoDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub info_string: FString,
}
#[repr(C, align(8))]
pub struct FMeshToCollectionDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub collection: FManagedArrayCollection,
    pub b_split_islands: bool,
    pub b_connect_islands_by_vertex_overlap: bool,
    pub connect_vertices_threshold: f32,
    pub b_add_cluster_root_for_single_mesh: bool,
}
#[repr(C, align(8))]
pub struct FCollectionToMeshDataflowNode {
    pub collection: FManagedArrayCollection,
    pub b_center_pivot: bool,
    pub mesh: UPtr<UDynamicMesh>,
    pub transform_selection: FDataflowTransformSelection,
    pub b_weld_vertices: bool,
    pub b_preserve_isolated_vertices: bool,
}
#[repr(C, align(8))]
pub struct FStaticMeshToMeshDataflowNode {
    pub static_mesh: UPtr<UStaticMesh>,
    pub b_use_hi_res: bool,
    pub lod_level: i32,
    pub mesh: UPtr<UDynamicMesh>,
    pub material_array: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FGetMeshBoundingBoxDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub bounding_box: FBox,
    pub center: FVector,
    pub dimensions: FVector,
}
#[repr(C, align(8))]
pub struct FMeshAppendDataflowNode {
    pub mesh1: UPtr<UDynamicMesh>,
    pub mesh2: UPtr<UDynamicMesh>,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FDataflowMeshAppendDataflowNode {
    pub mesh: UPtr<UDataflowMesh>,
    pub append_mesh: UPtr<UDataflowMesh>,
}
#[repr(C, align(8))]
pub struct FMakeDataflowMeshDataflowNode {
    pub in_mesh: UPtr<UDynamicMesh>,
    pub in_materials: TArray<UPtr<UMaterialInterface>>,
    pub mesh: UPtr<UDataflowMesh>,
}
#[repr(C, align(8))]
pub struct FDuplicateMeshUVChannelNode {
    pub mesh: UPtr<UDataflowMesh>,
    pub source_uv_channel: i32,
    pub new_uv_channel: i32,
}
#[repr(C, align(8))]
pub struct FSplitMeshIslandsDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub meshes: TArray<UPtr<UDynamicMesh>>,
    pub split_method: EDataflowMeshSplitIslandsMethod,
    pub connect_vertices_threshold: f32,
}
#[repr(C, align(8))]
pub struct FSplitDataflowMeshDataflowNode {
    pub in_mesh: UPtr<UDataflowMesh>,
    pub mesh: UPtr<UDynamicMesh>,
    pub material_array: TArray<UPtr<UMaterialInterface>>,
}
#[repr(C, align(8))]
pub struct FMeshCopyToPointsDataflowNode {
    pub points: TArray<FVector>,
    pub mesh_to_copy: UPtr<UDynamicMesh>,
    pub scale: f32,
    pub mesh: UPtr<UDynamicMesh>,
    pub meshes: TArray<UPtr<UDynamicMesh>>,
}
#[repr(C, align(8))]
pub struct FGetMeshDataDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub vertex_count: i32,
    pub edge_count: i32,
    pub triangle_count: i32,
}
#[repr(C, align(8))]
pub struct FGetCollectionFromAssetDataflowNode {
    pub collection_asset: UPtr<UGeometryCollection>,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FAppendCollectionAssetsDataflowNode {
    pub collection1: FManagedArrayCollection,
    pub collection2: FManagedArrayCollection,
    pub geometry_group_guids_out1: TArray<FString>,
    pub geometry_group_guids_out2: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FPrintStringDataflowNode {
    pub b_print_to_screen: bool,
    pub b_print_to_log: bool,
    pub color: FColor,
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
    pub collection: FManagedArrayCollection,
    pub bounding_box: FBox,
    pub center: FVector,
    pub dimensions: FVector,
}
#[repr(C, align(8))]
pub struct FBoundingSphereDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bounding_sphere: FSphere,
}
#[repr(C, align(8))]
pub struct FGetBoxLengthsDataflowNode {
    pub boxes: TArray<FBox>,
    pub lengths: TArray<f32>,
    pub measurement_method: EBoxLengthMeasurementMethod,
}
#[repr(C, align(8))]
pub struct FExpandBoundingBoxDataflowNode {
    pub bounding_box: FBox,
    pub min: FVector,
    pub max: FVector,
    pub center: FVector,
    pub half_extents: FVector,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FExpandBoundingSphereDataflowNode {
    pub bounding_sphere: FSphere,
    pub center: FVector,
    pub radius: f32,
    pub volume: f32,
}
#[repr(C, align(8))]
pub struct FExpandVectorDataflowNode {
    pub vector: FVector,
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
    pub inputs: TArray<FDataflowStringConvertibleTypes>,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FHashStringDataflowNode {
    pub string: FString,
    pub hash: i32,
}
#[repr(C, align(8))]
pub struct FHashVectorDataflowNode {
    pub vector: FVector,
    pub hash: i32,
}
#[repr(C, align(8))]
pub struct FGetBoundingBoxesFromCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub bounding_boxes: TArray<FBox>,
}
#[repr(C, align(8))]
pub struct FGetRootIndexFromCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub root_index: i32,
}
#[repr(C, align(8))]
pub struct FGetCentroidsFromCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub b_color_by_level: bool,
    pub color: FLinearColor,
    pub b_size_by_level: bool,
    pub size: f32,
    pub point_size: FRuntimeFloatCurve,
    pub centroids: TArray<FVector>,
    pub levels: TArray<i32>,
}
#[repr(C, align(8))]
pub struct FTransformCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub translate: FVector,
    pub rotation_order: ERotationOrderEnum,
    pub rotate: FVector,
    pub scale: FVector,
    pub uniform_scale: f32,
    pub rotate_pivot: FVector,
    pub scale_pivot: FVector,
    pub b_invert_transformation: bool,
}
#[repr(C, align(8))]
pub struct FBakeTransformsInCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FTransformMeshDataflowNode {
    pub mesh: UPtr<UDynamicMesh>,
    pub translate: FVector,
    pub rotation_order: ERotationOrderEnum,
    pub rotate: FVector,
    pub scale: FVector,
    pub uniform_scale: f32,
    pub rotate_pivot: FVector,
    pub scale_pivot: FVector,
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
    pub mesh_a: UPtr<UDynamicMesh>,
    pub mesh_b: UPtr<UDynamicMesh>,
    pub b_condition: bool,
    pub mesh: UPtr<UDynamicMesh>,
}
#[repr(C, align(8))]
pub struct FBranchCollectionDataflowNode {
    pub true_collection: FManagedArrayCollection,
    pub false_collection: FManagedArrayCollection,
    pub b_condition: bool,
    pub chosen_collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetSchemaDataflowNode {
    pub collection: FManagedArrayCollection,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FRemoveOnBreakDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub b_enabled_removal: bool,
    pub post_break_timer: FVector2f,
    pub removal_timer: FVector2f,
    pub b_cluster_crumbling: bool,
}
#[repr(C, align(8))]
pub struct FSetAnchorStateDataflowNode {
    pub anchor_state: EAnchorStateEnum,
    pub b_set_not_selected_bones_to_opposite_state: bool,
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FSetDynamicStateDataflowNode {
    pub dynamic_state: EDataflowGeometryCollectionDynamicState,
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
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
    pub collection: FManagedArrayCollection,
    pub color: FLinearColor,
    pub line_width_multiplier: f32,
    pub center_color: FLinearColor,
    pub center_size: f32,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(16))]
pub struct FCollectionSetPivotDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform: FTransform,
}
#[repr(C, align(8))]
pub struct FAddCustomCollectionAttributeDataflowNode {
    pub collection: FManagedArrayCollection,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub custom_attribute_type: ECustomAttributeTypeEnum,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FGetNumElementsInCollectionGroupDataflowNode {
    pub collection: FManagedArrayCollection,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub num_elements: i32,
}
#[repr(C, align(8))]
pub struct FGetCollectionAttributeDataTypedDataflowNode {
    pub collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub float_attribute_data: TArray<f32>,
    pub double_attribute_data: TArray<f64>,
    pub int32_attribute_data: TArray<i32>,
    pub string_attribute_data: TArray<FString>,
    pub vector3f_attribute_data: TArray<FVector3f>,
    pub vector3d_attribute_data: TArray<FVector3d>,
    pub linear_color_attribute_data: TArray<FLinearColor>,
}
#[repr(C, align(8))]
pub struct FGetCollectionAttributeDataTypedDataflowNode_v2 {
    pub collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub numeric_array: FDataflowNumericArrayTypes,
    pub vector_array: FDataflowVectorArrayTypes,
    pub string_array: FDataflowStringArrayTypes,
}
#[repr(C, align(8))]
pub struct FSetCollectionAttributeDataTypedDataflowNode {
    pub collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub group_name: EStandardGroupNameEnum,
    pub custom_group_name: FString,
    pub attr_name: FString,
    pub bool_attribute_data: TArray<bool>,
    pub float_attribute_data: TArray<f32>,
    pub double_attribute_data: TArray<f64>,
    pub int32_attribute_data: TArray<i32>,
    pub string_attribute_data: TArray<FString>,
    pub vector3f_attribute_data: TArray<FVector3f>,
    pub vector3d_attribute_data: TArray<FVector3d>,
    pub linear_color_attribute_data: TArray<FLinearColor>,
}
#[repr(C, align(8))]
pub struct FSelectionToVertexListDataflowNode {
    pub vertex_selection: FDataflowVertexSelection,
    pub vertex_list: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FMultiplyTransformDataflowNode {
    pub in_left_transform: FTransform,
    pub in_right_transform: FTransform,
    pub out_transform: FTransform,
}
#[repr(C, align(16))]
pub struct FInvertTransformDataflowNode {
    pub in_transform: FTransform,
    pub out_transform: FTransform,
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
    pub collection: FManagedArrayCollection,
    pub vertices: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FPointsToCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FCollectionToPointsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FSpheresToPointsDataflowNode {
    pub spheres: TArray<FSphere>,
    pub points: TArray<FVector>,
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
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FFilterPointSetWithMeshDataflowNode {
    pub target_mesh: UPtr<UDynamicMesh>,
    pub filter_method: u8,
    pub b_keep_inside: bool,
    pub winding_threshold: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub b_use_signed_distance: bool,
    pub sample_points: TArray<FVector>,
}
#[repr(C, align(8))]
pub struct FUniformPointSamplingDataflowNode {
    pub target_mesh: UPtr<UDynamicMesh>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub sample_points: TArray<FVector>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FNonUniformPointSamplingDataflowNode {
    pub target_mesh: UPtr<UDynamicMesh>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub max_sampling_radius: f32,
    pub size_distribution: ENonUniformSamplingDistributionMode,
    pub size_distribution_power: f32,
    pub sample_points: TArray<FVector>,
    pub sample_radii: TArray<f32>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FVertexWeightedPointSamplingDataflowNode {
    pub target_mesh: UPtr<UDynamicMesh>,
    pub vertex_weights: TArray<f32>,
    pub sampling_radius: f32,
    pub max_num_samples: i32,
    pub sub_sample_density: f32,
    pub random_seed: i32,
    pub max_sampling_radius: f32,
    pub size_distribution: ENonUniformSamplingDistributionMode,
    pub size_distribution_power: f32,
    pub weight_mode: ENonUniformSamplingWeightMode,
    pub b_invert_weights: bool,
    pub sample_points: TArray<FVector>,
    pub sample_radii: TArray<f32>,
    pub sample_triangle_i_ds: TArray<i32>,
    pub sample_barycentric_coords: TArray<FVector>,
    pub num_sample_points: i32,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionAllDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub transform_selection_a: FDataflowTransformSelection,
    pub transform_selection_b: FDataflowTransformSelection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionInfoDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
    pub string: FString,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionNoneDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionInvertDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionRandomDataflowNode {
    pub b_deterministic: bool,
    pub random_seed: f32,
    pub random_threshold: f32,
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionRootDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionCustomDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bone_indicies: FString,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionCustomDataflowNode_v2 {
    pub collection: FManagedArrayCollection,
    pub bone_indices: FString,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionFromIndexArrayDataflowNode {
    pub collection: FManagedArrayCollection,
    pub bone_indices: TArray<i32>,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionParentDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByPercentageDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub percentage: i32,
    pub b_deterministic: bool,
    pub random_seed: f32,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionChildrenDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionSiblingsDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionLevelDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionTargetLevelDataflowNode {
    pub collection: FManagedArrayCollection,
    pub target_level: i32,
    pub b_skip_embedded: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionContactDataflowNode {
    pub transform_selection: FDataflowTransformSelection,
    pub collection: FManagedArrayCollection,
    pub b_allow_contact_in_parent_levels: bool,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionLeafDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionClusterDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionClusterDataflowNode_v2 {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
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
    pub collection: FManagedArrayCollection,
    pub size_min: f32,
    pub size_max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub b_use_relative_size: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByVolumeDataflowNode {
    pub collection: FManagedArrayCollection,
    pub volume_min: f32,
    pub volume_max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FCollectionTransformSelectionInBoxDataflowNode {
    pub collection: FManagedArrayCollection,
    pub box_: FBox,
    pub transform: FTransform,
    pub ty: ESelectSubjectTypeEnum,
    pub b_all_vertices_must_contained_in_box: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(16))]
pub struct FCollectionTransformSelectionInSphereDataflowNode {
    pub collection: FManagedArrayCollection,
    pub sphere: FSphere,
    pub transform: FTransform,
    pub ty: ESelectSubjectTypeEnum,
    pub b_all_vertices_must_contained_in_sphere: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByFloatAttrDataflowNode {
    pub collection: FManagedArrayCollection,
    pub group_name: FString,
    pub attr_name: FString,
    pub min: f32,
    pub max: f32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionTransformSelectionByIntAttrDataflowNode {
    pub collection: FManagedArrayCollection,
    pub group_name: FString,
    pub attr_name: FString,
    pub min: i32,
    pub max: i32,
    pub range_setting: ERangeSettingEnum,
    pub b_inclusive: bool,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionCustomDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_indicies: FString,
    pub vertex_selection: FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionFaceSelectionCustomDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_indicies: FString,
    pub face_selection: FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionConvertDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub face_selection: FDataflowFaceSelection,
    pub vertex_selection: FDataflowVertexSelection,
    pub b_all_elements_must_be_selected: bool,
}
#[repr(C, align(8))]
pub struct FCollectionFaceSelectionInvertDataflowNode {
    pub face_selection: FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionByPercentageDataflowNode {
    pub vertex_selection: FDataflowVertexSelection,
    pub percentage: i32,
    pub b_deterministic: bool,
    pub random_seed: f32,
}
#[repr(C, align(8))]
pub struct FCollectionVertexSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub vertex_selection_a: FDataflowVertexSelection,
    pub vertex_selection_b: FDataflowVertexSelection,
    pub vertex_selection: FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionByAttrDataflowNode {
    pub collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub group: ESelectionByAttrGroup,
    pub attribute: FString,
    pub operation: ESelectionByAttrOperation,
    pub value: FString,
    pub vertex_selection: FDataflowVertexSelection,
    pub face_selection: FDataflowFaceSelection,
    pub transform_selection: FDataflowTransformSelection,
    pub geometry_selection: FDataflowGeometrySelection,
    pub material_selection: FDataflowMaterialSelection,
    pub curve_selection: FDataflowCurveSelection,
}
#[repr(C, align(8))]
pub struct FGeometrySelectionToVertexSelectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub geometry_indices: FString,
    pub geometry_selection: FDataflowGeometrySelection,
    pub vertex_selection: FDataflowVertexSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionSetOperationDataflowNode {
    pub operation: ESetOperationEnum,
    pub selection_a: FDataflowSelectionTypes,
    pub selection_b: FDataflowSelectionTypes,
    pub selection: FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FCollectionSelectionInvertDataflowNode {
    pub selection: FDataflowSelectionTypes,
}
#[repr(C, align(8))]
pub struct FCollectionSelectInternalFacesDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub face_selection: FDataflowFaceSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSelectTransformStringDataflowNode {
    pub collection: FManagedArrayCollection,
    pub attribute: FString,
    pub search_text: FString,
    pub method: EDataflowCollectionSelectionByNameMethod,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCollectionSetTransformStringValueDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub attribute: FString,
    pub text_to_set: FString,
}
#[repr(C, align(8))]
pub struct FSkeletalMeshToCollectionDataflowNode {
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub collection: FManagedArrayCollection,
    pub b_import_transform_only: bool,
}
#[repr(C, align(8))]
pub struct FCollectionToSkeletalMeshDataflowNode {
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub skeletal_mesh: UPtr<USkeletalMesh>,
    pub skeleton: UPtr<USkeleton>,
}
#[repr(C, align(8))]
pub struct FSkeletonToCollectionDataflowNode {
    pub skeleton: UPtr<USkeleton>,
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(16))]
pub struct FStaticMeshToCollectionDataflowNode_v2 {
    pub static_mesh: UPtr<UStaticMesh>,
    pub mesh_transform: FTransform,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
    pub root_proxy_meshes: TArray<FDataflowRootProxyMesh>,
    pub b_set_internal_from_material_index: bool,
    pub b_split_components: bool,
}
#[repr(C, align(16))]
pub struct FStaticMeshToCollectionDataflowNode {
    pub static_mesh: UPtr<UStaticMesh>,
    pub mesh_transform: FTransform,
    pub b_set_internal_from_material_index: bool,
    pub b_split_components: bool,
    pub collection: FManagedArrayCollection,
    pub materials: TArray<UPtr<UMaterial>>,
    pub material_instances: TArray<UPtr<UMaterialInterface>>,
    pub instanced_meshes: TArray<FGeometryCollectionAutoInstanceMesh>,
}
#[repr(C, align(8))]
pub struct FBakeTextureFromCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub image: FDataflowImage,
    pub face_selection: FDataflowFaceSelection,
    pub resolution: EDataflowImageResolution,
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
    pub collection: FManagedArrayCollection,
    pub from_collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
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
    pub collection: FManagedArrayCollection,
    pub from_collection: FManagedArrayCollection,
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
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub kinematic_value: ESetKinematicVertexSelectionKinematicValue,
}
#[repr(C, align(8))]
pub struct FTriangleBoundaryIndicesNode {
    pub collection: FManagedArrayCollection,
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
    pub collection: FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub generate_method: EGenerateConvexMethod,
    pub intersect_if_computed_is_smaller_by_factor: f32,
    pub min_external_volume_to_intersect: f32,
    pub b_compute_intersections_before_hull: bool,
    pub simplification_distance_threshold: f32,
    pub convex_decomposition_settings: FDataflowConvexDecompositionSettings,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FSimplifyConvexHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub simplify_method: EConvexHullSimplifyMethod,
    pub simplification_angle_threshold: f32,
    pub simplification_distance_threshold: f32,
    pub min_target_triangle_count: i32,
    pub b_use_existing_vertices: bool,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FCreateNonOverlappingConvexHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub can_exceed_fraction: f32,
    pub simplification_distance_threshold: f32,
    pub overlap_removal_method: EConvexOverlapRemovalMethodEnum,
    pub overlap_removal_shrink_percent: f32,
    pub can_remove_fraction: f32,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FGenerateClusterConvexHullsFromLeafHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub convex_count: i32,
    pub error_tolerance: f64,
    pub b_prefer_external_collision_shapes: bool,
    pub allow_merges: EAllowConvexMergeMethod,
    pub merge_proximity_filter: EConvexHullProximityFilter,
    pub merge_proximity_distance_threshold: f32,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub b_allow_merging_leaf_hulls: bool,
    pub b_protect_negative_space: bool,
    pub sample_method: ENegativeSpaceSampleMethodDataflowEnum,
    pub b_require_search_sample_coverage: bool,
    pub b_only_connected_to_hull: bool,
    pub target_num_samples: i32,
    pub min_sample_spacing: f64,
    pub negative_space_tolerance: f64,
    pub min_radius: f64,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FGenerateClusterConvexHullsFromChildrenHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub convex_count: i32,
    pub error_tolerance: f64,
    pub b_prefer_external_collision_shapes: bool,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub merge_proximity_filter: EConvexHullProximityFilter,
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
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FClearConvexHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
}
#[repr(C, align(8))]
pub struct FCopyConvexHullsFromRootDataflowNode {
    pub collection: FManagedArrayCollection,
    pub from_collection: FManagedArrayCollection,
    pub b_skip_if_empty: bool,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FMergeConvexHullsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub sphere_covering: FDataflowSphereCovering,
    pub max_convex_count: i32,
    pub error_tolerance: f64,
    pub optional_selection_filter: FDataflowTransformSelection,
    pub merge_proximity_filter: EConvexHullProximityFilter,
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
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
    pub sphere_covering_debug_draw_render_settings: FDataflowNodeSphereCoveringDebugDrawSettings,
}
#[repr(C, align(8))]
pub struct FUpdateVolumeAttributesDataflowNode {
    pub collection: FManagedArrayCollection,
}
#[repr(C, align(8))]
pub struct FGetConvexHullVolumeDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub volume: f32,
    pub b_sum_children_for_clusters_without_hulls: bool,
    pub b_volume_of_union: bool,
    pub debug_draw_render_settings: FDataflowNodeDebugDrawSettings,
    pub b_randomize_color: bool,
    pub color_random_seed: i32,
}
#[repr(C, align(8))]
pub struct FFixTinyGeoDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub merge_type: EFixTinyGeoMergeType,
    pub b_on_fracture_level: bool,
    pub b_only_clusters: bool,
    pub b_only_same_parent: bool,
    pub b_geometry_only_same_parent: bool,
    pub b_fracture_level_is_all: bool,
    pub neighbor_selection: EFixTinyGeoNeighborSelectionMethod,
    pub b_only_to_connected: bool,
    pub b_use_collection_proximity_for_connections: bool,
    pub use_bone_selection: EFixTinyGeoUseBoneSelection,
    pub selection_method: EFixTinyGeoGeometrySelectionMethod,
    pub min_volume_cube_root: f32,
    pub relative_volume: f32,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(8))]
pub struct FRecomputeNormalsInGeometryCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub b_only_tangents: bool,
    pub b_recompute_sharp_edges: bool,
    pub sharp_edge_angle_threshold: f32,
    pub b_only_internal_surfaces: bool,
}
#[repr(C, align(8))]
pub struct FResampleGeometryCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_selection: FDataflowTransformSelection,
    pub add_samples_for_collision: bool,
    pub collision_sample_spacing: f32,
}
#[repr(C, align(8))]
pub struct FValidateGeometryCollectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub b_remove_unreferenced_geometry: bool,
    pub b_remove_clusters_of_one: bool,
    pub b_remove_dangling_clusters: bool,
}
#[repr(C, align(8))]
pub struct FAddUVChannelDataflowNode {
    pub collection: FManagedArrayCollection,
    pub uv_channel: i32,
    pub default_value: FVector2f,
}
#[repr(C, align(8))]
pub struct FAutoUnwrapUVDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_selection: FDataflowFaceSelection,
    pub uv_channel: i32,
    pub gutter_size: i32,
}
#[repr(C, align(8))]
pub struct FMergeUVIslandsDataflowNode {
    pub collection: FManagedArrayCollection,
    pub face_selection: FDataflowFaceSelection,
    pub uv_channel: i32,
    pub area_distortion_threshold: f64,
    pub max_normal_deviation_deg: f64,
    pub normal_smoothing_rounds: i32,
    pub normal_smoothing_alpha: f64,
}
#[repr(C, align(8))]
pub struct FBoxProjectUVDataflowNode {
    pub collection: FManagedArrayCollection,
    pub uv_channel: i32,
    pub gutter_size: i32,
    pub projection_scale: FVector,
    pub uv_offset: FVector2f,
    pub b_auto_fit_to_bounds: bool,
    pub b_center_box_at_pivot: bool,
    pub b_uniform_projection_scale: bool,
}
#[repr(C, align(8))]
pub struct FGeometryCollectionVertexScalarToVertexIndicesNode {
    pub collection: FManagedArrayCollection,
    pub attribute_key: FCollectionAttributeKey,
    pub selection_threshold: f32,
    pub vertex_indices: TArray<i32>,
}
#[repr(C, align(16))]
pub struct FTransformCollectionAttributeDataflowNode {
    pub collection: FManagedArrayCollection,
    pub transform_in: FTransform,
    pub local_transform: FTransform,
    pub group_name: FString,
    pub attribute_name: FString,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromFloatArrayDataflowNode {
    pub collection: FManagedArrayCollection,
    pub float_array: TArray<f32>,
    pub b_normalize_input: bool,
    pub color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromVertexIndicesDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_indices_in: TArray<i32>,
    pub selected_color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FSetVertexColorFromVertexSelectionDataflowNode {
    pub collection: FManagedArrayCollection,
    pub vertex_selection: FDataflowVertexSelection,
    pub selected_color: FLinearColor,
}
