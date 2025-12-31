#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FInterchangeLodSceneNodeContainer {
    pub base_nodes: TArray<
        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
    >,
}
#[repr(C, align(8))]
pub struct FInterchangeMeshInstance {
    pub mesh_instance_uid: FString,
    pub lod_group_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
    pub b_reference_skinned_mesh: bool,
    pub b_reference_morph_target: bool,
    pub b_has_morph_targets: bool,
    pub b_is_animated: bool,
    pub scene_node_per_lod_index: TMap<i32, FInterchangeLodSceneNodeContainer>,
    pub referencing_mesh_geometry_uids: TArray<FString>,
}
#[repr(C, align(8))]
pub struct FInterchangeMeshGeometry {
    pub mesh_uid: FString,
    pub mesh_node: UPtr<crate::bindings::interchange_nodes::UInterchangeMeshNode>,
    pub referencing_mesh_instance_uids: TArray<FString>,
    pub attached_socket_uids: TArray<FString>,
}
#[repr(C, align(1))]
pub struct FInterchangePipelineMeshesUtilitiesContext {
    pub b_convert_static_mesh_to_skeletal_mesh: bool,
    pub b_convert_skeletal_mesh_to_static_mesh: bool,
    pub b_convert_statics_with_morph_targets_to_skeletals: bool,
    pub b_import_meshes_in_bone_hierarchy: bool,
    pub b_query_geometry_only_if_no_instance: bool,
    pub b_ignore_static_meshes: bool,
    pub b_ignore_geometry_caches: bool,
}
pub struct UInterchangeGenericCommonMeshesProperties {
    pub force_all_mesh_as_type: EInterchangeForceMeshType,
    pub b_single_bone_skeleton: bool,
    pub b_auto_detect_mesh_type: bool,
    pub b_import_lods: bool,
    pub b_bake_meshes: bool,
    pub b_bake_pivot_meshes: bool,
    pub b_keep_sections_separate: bool,
    pub vertex_color_import_option: EInterchangeVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub b_import_sockets: bool,
    pub b_recompute_normals: bool,
    pub b_recompute_tangents: bool,
    pub b_use_mikk_t_space: bool,
    pub b_compute_weighted_normals: bool,
    pub b_use_high_precision_tangent_basis: bool,
    pub b_use_full_precision_u_vs: bool,
    pub b_use_backwards_compatible_f16_trunc_u_vs: bool,
    pub b_remove_degenerates: bool,
}
pub struct UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties {
    pub b_import_only_animations: bool,
    pub skeleton: TWeakObjectPtr<crate::bindings::engine::USkeleton>,
    pub b_import_meshes_in_bone_hierarchy: bool,
    pub b_use_t0_as_ref_pose: bool,
    pub b_add_curve_metadata_to_skeleton: bool,
    pub b_convert_statics_with_morph_targets_to_skeletals: bool,
}
pub struct UInterchangeGenericAnimationPipeline {
    pub common_skeletal_meshes_and_animations_properties: TWeakObjectPtr<
        UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties,
    >,
    pub common_meshes_properties: TWeakObjectPtr<
        UInterchangeGenericCommonMeshesProperties,
    >,
    pub b_import_animations: bool,
    pub b_import_bone_tracks: bool,
    pub animation_range: EInterchangeAnimationRange,
    pub frame_import_range: crate::bindings::core_u_object::FInt32Interval,
    pub b_use30_hz_to_bake_bone_animation: bool,
    pub custom_bone_animation_sample_rate: i32,
    pub b_snap_to_closest_frame_boundary: bool,
    pub b_import_custom_attribute: bool,
    pub b_add_curve_metadata_to_skeleton_deprecated: bool,
    pub b_set_material_drive_parameter_on_custom_attribute: bool,
    pub material_curve_suffixes: TArray<FString>,
    pub b_remove_curve_redundant_keys: bool,
    pub b_do_not_import_curve_with_zero: bool,
    pub b_delete_existing_non_curve_custom_attributes: bool,
    pub b_delete_existing_custom_attribute_curves: bool,
    pub b_delete_existing_morph_target_curves: bool,
    pub source_animation_name: FString,
    pub b_scene_import: bool,
}
pub struct UInterchangeGenericAssetsPipeline {
    pub pipeline_display_name: FString,
    pub reimport_strategy: crate::bindings::interchange_core::EReimportStrategyFlags,
    pub b_use_source_name_for_asset: bool,
    pub b_scene_name_sub_folder: bool,
    pub b_asset_type_sub_folders: bool,
    pub asset_name: FString,
    pub import_offset_translation: crate::bindings::core_u_object::FVector,
    pub import_offset_rotation: crate::bindings::core_u_object::FRotator,
    pub import_offset_uniform_scale: f32,
    pub common_meshes_properties: UPtr<UInterchangeGenericCommonMeshesProperties>,
    pub common_skeletal_meshes_and_animations_properties: UPtr<
        UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties,
    >,
    pub mesh_pipeline: UPtr<UInterchangeGenericMeshPipeline>,
    pub animation_pipeline: UPtr<UInterchangeGenericAnimationPipeline>,
    pub material_pipeline: UPtr<UInterchangeGenericMaterialPipeline>,
    pub groom_pipeline: UPtr<UInterchangeGenericGroomPipeline>,
    pub scene_name_folder_prefix: FString,
    pub content_path_existing_skeleton: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_import_only_animation_adjusted: bool,
}
pub struct UInterchangeGenericAudioPipeline {
    pub pipeline_display_name: FString,
    pub b_import_sounds: bool,
    pub base_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
    pub source_datas: TArray<
        UPtr<crate::bindings::interchange_core::UInterchangeSourceData>,
    >,
    pub sound_wave_nodes: TArray<
        UPtr<crate::bindings::interchange_nodes::UInterchangeAudioSoundWaveNode>,
    >,
    pub sound_wave_factory_nodes: TArray<
        UPtr<
            crate::bindings::interchange_factory_nodes::UInterchangeAudioSoundWaveFactoryNode,
        >,
    >,
}
pub struct UInterchangeGenericGroomPipeline {
    pub pipeline_display_name: FString,
    pub b_enable_groom_types_import: bool,
    pub b_import_grooms: bool,
    pub group_interpolation_settings: crate::bindings::hair_strands_core::FHairGroupsInterpolation,
    pub b_import_groom_caches: bool,
    pub groom_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub import_groom_cache_type: crate::bindings::interchange_factory_nodes::EInterchangeGroomCacheImportType,
    pub b_override_time_range: bool,
    pub frame_start: i32,
    pub frame_end: i32,
}
pub struct UInterchangeGenericMaterialPipeline {
    pub pipeline_display_name: FString,
    pub b_import_materials: bool,
    pub search_location: EInterchangeMaterialSearchLocation,
    pub asset_name: FString,
    pub material_import: EInterchangeMaterialImportOption,
    pub b_identify_duplicate_materials: bool,
    pub b_create_material_instance_for_parent: bool,
    pub parent_material: crate::bindings::core_u_object::FSoftObjectPath,
    pub texture_pipeline: UPtr<UInterchangeGenericTexturePipeline>,
    pub sparse_volume_texture_pipeline: UPtr<UInterchangeSparseVolumeTexturePipeline>,
    pub b_override_displacement: bool,
    pub override_displacement_center: f32,
    pub base_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
}
pub struct UInterchangeGenericMeshPipeline {
    pub common_meshes_properties: TWeakObjectPtr<
        UInterchangeGenericCommonMeshesProperties,
    >,
    pub common_skeletal_meshes_and_animations_properties: TWeakObjectPtr<
        UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties,
    >,
    pub b_import_static_meshes: bool,
    pub b_combine_static_meshes: bool,
    pub lod_group: FName,
    pub b_auto_compute_lod_screen_sizes: bool,
    pub lod_screen_sizes: TArray<f32>,
    pub b_import_collision_deprecated: bool,
    pub b_collision: bool,
    pub b_import_collision_according_to_mesh_name: bool,
    pub b_one_convex_hull_per_ucx: bool,
    pub collision: crate::bindings::interchange_nodes::EInterchangeMeshCollision,
    pub b_force_collision_primitive_generation: bool,
    pub b_build_nanite: bool,
    pub nanite_triangle_threshold: i64,
    pub b_build_reversed_index_buffer: bool,
    pub b_generate_lightmap_u_vs: bool,
    pub b_generate_distance_field_as_if_two_sided: bool,
    pub b_support_face_remap: bool,
    pub min_lightmap_resolution: i32,
    pub src_lightmap_index: i32,
    pub dst_lightmap_index: i32,
    pub build_scale3_d: crate::bindings::core_u_object::FVector,
    pub distance_field_resolution_scale: f32,
    pub distance_field_replacement_mesh: TWeakObjectPtr<
        crate::bindings::engine::UStaticMesh,
    >,
    pub max_lumen_mesh_cards: i32,
    pub b_import_skeletal_meshes: bool,
    pub skeletal_mesh_import_content_type: crate::bindings::interchange_factory_nodes::EInterchangeSkeletalMeshContentType,
    pub last_skeletal_mesh_import_content_type: crate::bindings::interchange_factory_nodes::EInterchangeSkeletalMeshContentType,
    pub b_combine_skeletal_meshes_deprecated: bool,
    pub b_import_morph_targets: bool,
    pub b_merge_morph_targets_with_same_name: bool,
    pub b_import_vertex_attributes: bool,
    pub b_update_skeleton_reference_pose: bool,
    pub b_create_physics_asset: bool,
    pub physics_asset: TWeakObjectPtr<crate::bindings::engine::UPhysicsAsset>,
    pub b_use_high_precision_skin_weights: bool,
    pub threshold_position: f32,
    pub threshold_tangent_normal: f32,
    pub threshold_uv: f32,
    pub morph_threshold_position: f32,
    pub bone_influence_limit: i32,
    pub b_import_geometry_caches: bool,
    pub b_flatten_tracks: bool,
    pub compressed_position_precision: f32,
    pub compressed_texture_coordinates_number_of_bits: i32,
    pub b_override_time_range: bool,
    pub frame_start: i32,
    pub frame_end: i32,
    pub motion_vectors: crate::bindings::interchange_nodes::EInterchangeMotionVectorsHandling,
    pub b_apply_constant_topology_optimizations: bool,
    pub b_store_imported_vertex_numbers: bool,
    pub b_optimize_index_buffers: bool,
}
pub struct UInterchangeGenericLevelPipeline {
    pub pipeline_display_name: FString,
    pub reimport_property_strategy: crate::bindings::interchange_core::EReimportStrategyFlags,
    pub scene_hierarchy_type: EInterchangeSceneHierarchyType,
    pub b_delete_missing_actors: bool,
    pub b_force_reimport_deleted_actors: bool,
    pub b_force_reimport_deleted_assets: bool,
    pub b_delete_missing_assets: bool,
    pub b_use_hierarchical_ism_components: bool,
    pub b_use_physical_instead_of_standard_perspective_camera: bool,
}
pub struct UInterchangeGenericTexturePipeline {
    pub pipeline_display_name: FString,
    pub b_import_textures: bool,
    pub asset_name: FString,
    pub b_detect_normal_map_texture: bool,
    pub b_flip_normal_map_green_channel: bool,
    pub b_import_udi_ms: bool,
    pub file_extensions_to_import_as_long_lat_cubemap: TSet<FString>,
    pub b_prefer_compressed_source_data: bool,
    pub b_allow_non_power_of_two: bool,
    pub base_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
}
pub struct UGLTFPipelineSettings {
    pub material_parents: TMap<FString, crate::bindings::core_u_object::FSoftObjectPath>,
}
pub struct UInterchangeGLTFPipeline {
    pub pipeline_display_name: FString,
}
pub struct UMaterialXPipelineSettings {
    pub predefined_surface_shaders: TMap<
        crate::bindings::interchange_common::EInterchangeMaterialXShaders,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub predefined_bsdf: TMap<
        crate::bindings::interchange_common::EInterchangeMaterialXBSDF,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub predefined_edf: TMap<
        crate::bindings::interchange_common::EInterchangeMaterialXEDF,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
    pub predefined_vdf: TMap<
        crate::bindings::interchange_common::EInterchangeMaterialXVDF,
        crate::bindings::core_u_object::FSoftObjectPath,
    >,
}
pub struct UInterchangeMaterialXPipeline {
    pub b_volumetric_material: bool,
}
pub struct UInterchangePipelineMeshesUtilities {}
pub struct UInterchangeSparseVolumeTexturePipeline {
    pub pipeline_display_name: FString,
    pub b_import_sparse_volume_textures: bool,
    pub b_import_animated_sparse_volume_textures: bool,
    pub asset_name: FString,
    pub base_node_container: UPtr<
        crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
    >,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeForceMeshType(pub u8);
impl EInterchangeForceMeshType {
    pub const IFMT_NONE: EInterchangeForceMeshType = EInterchangeForceMeshType(0);
    pub const IFMT_STATIC_MESH: EInterchangeForceMeshType = EInterchangeForceMeshType(1);
    pub const IFMT_SKELETAL_MESH: EInterchangeForceMeshType = EInterchangeForceMeshType(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeVertexColorImportOption(pub u8);
impl EInterchangeVertexColorImportOption {
    pub const IVCIO_REPLACE: EInterchangeVertexColorImportOption = EInterchangeVertexColorImportOption(
        0,
    );
    pub const IVCIO_IGNORE: EInterchangeVertexColorImportOption = EInterchangeVertexColorImportOption(
        1,
    );
    pub const IVCIO_OVERRIDE: EInterchangeVertexColorImportOption = EInterchangeVertexColorImportOption(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeAnimationRange(pub u8);
impl EInterchangeAnimationRange {
    pub const TIMELINE: EInterchangeAnimationRange = EInterchangeAnimationRange(0);
    pub const ANIMATED: EInterchangeAnimationRange = EInterchangeAnimationRange(1);
    pub const SET_RANGE: EInterchangeAnimationRange = EInterchangeAnimationRange(2);
    pub const MAX: EInterchangeAnimationRange = EInterchangeAnimationRange(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeMaterialSearchLocation(pub u8);
impl EInterchangeMaterialSearchLocation {
    pub const LOCAL: EInterchangeMaterialSearchLocation = EInterchangeMaterialSearchLocation(
        0,
    );
    pub const UNDER_PARENT: EInterchangeMaterialSearchLocation = EInterchangeMaterialSearchLocation(
        1,
    );
    pub const UNDER_ROOT: EInterchangeMaterialSearchLocation = EInterchangeMaterialSearchLocation(
        2,
    );
    pub const ALL_ASSETS: EInterchangeMaterialSearchLocation = EInterchangeMaterialSearchLocation(
        3,
    );
    pub const DO_NOT_SEARCH: EInterchangeMaterialSearchLocation = EInterchangeMaterialSearchLocation(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeMaterialImportOption(pub u8);
impl EInterchangeMaterialImportOption {
    pub const IMPORT_AS_MATERIALS: EInterchangeMaterialImportOption = EInterchangeMaterialImportOption(
        0,
    );
    pub const IMPORT_AS_MATERIAL_INSTANCES: EInterchangeMaterialImportOption = EInterchangeMaterialImportOption(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EInterchangeSceneHierarchyType(pub u8);
impl EInterchangeSceneHierarchyType {
    pub const CREATE_LEVEL_ACTORS: EInterchangeSceneHierarchyType = EInterchangeSceneHierarchyType(
        0,
    );
    pub const CREATE_LEVEL_INSTANCE_ACTOR: EInterchangeSceneHierarchyType = EInterchangeSceneHierarchyType(
        1,
    );
    pub const CREATE_PACKED_ACTOR: EInterchangeSceneHierarchyType = EInterchangeSceneHierarchyType(
        2,
    );
}
