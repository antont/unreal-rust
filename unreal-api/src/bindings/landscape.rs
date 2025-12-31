#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FLandscapeEditLayerMergeRenderBlackboardItem {}
#[repr(C, align(8))]
pub struct FGizmoSelectData {
    pub ratio: f32,
    pub height_data: f32,
}
#[repr(C, align(8))]
pub struct FGrassVariety {
    pub grass_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub grass_density: crate::bindings::core_u_object::FPerPlatformFloat,
    pub grass_density_quality: crate::bindings::engine::FPerQualityLevelFloat,
    pub b_use_grid: bool,
    pub placement_jitter: f32,
    pub start_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub start_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub end_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub end_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub min_lod: i32,
    pub allowed_density_range: crate::bindings::core_u_object::FFloatInterval,
    pub scaling: EGrassScaling,
    pub scale_x: crate::bindings::core_u_object::FFloatInterval,
    pub scale_y: crate::bindings::core_u_object::FFloatInterval,
    pub scale_z: crate::bindings::core_u_object::FFloatInterval,
    pub b_weight_attenuates_max_scale: bool,
    pub max_scale_weight_attenuation: f32,
    pub random_rotation: bool,
    pub align_to_surface: bool,
    pub b_align_to_triangle_normals: bool,
    pub b_use_landscape_lightmap: bool,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    pub b_receives_decals: bool,
    pub b_affect_distance_field_lighting: bool,
    pub b_cast_dynamic_shadow: bool,
    pub b_cast_contact_shadow: bool,
    pub b_keep_instance_buffer_cpu_copy: bool,
    pub instance_world_position_offset_disable_distance: u32,
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
}
#[repr(C, align(4))]
pub struct FLandscapeMaterialTextureStreamingInfo {
    pub texture_name: FName,
    pub texel_factor: f32,
}
#[repr(C, align(8))]
pub struct FLandscapeSplineConnection {
    pub segment: UPtr<ULandscapeSplineSegment>,
    pub flags_8: u8,
}
#[repr(C, align(8))]
pub struct FForeignControlPointData {
    pub modification_key: crate::bindings::core_u_object::FGuid,
    pub mesh_component: UPtr<UControlPointMeshComponent>,
    pub identifier: TLazyObjectPtr<ULandscapeSplineControlPoint>,
}
#[repr(C, align(8))]
pub struct FForeignSplineSegmentData {
    pub modification_key: crate::bindings::core_u_object::FGuid,
    pub mesh_components: TArray<UPtr<crate::bindings::engine::USplineMeshComponent>>,
    pub identifier: TLazyObjectPtr<ULandscapeSplineSegment>,
}
#[repr(C, align(8))]
pub struct FForeignWorldSplineData {
    pub foreign_control_point_data_map_deprecated: TMap<
        TLazyObjectPtr<ULandscapeSplineControlPoint>,
        FForeignControlPointData,
    >,
    pub foreign_control_point_data: TArray<FForeignControlPointData>,
    pub foreign_spline_segment_data_map_deprecated: TMap<
        TLazyObjectPtr<ULandscapeSplineSegment>,
        FForeignSplineSegmentData,
    >,
    pub foreign_spline_segment_data: TArray<FForeignSplineSegmentData>,
}
#[repr(C, align(8))]
pub struct FLandscapeSplineInterpPoint {
    pub center: crate::bindings::core_u_object::FVector,
    pub left: crate::bindings::core_u_object::FVector,
    pub right: crate::bindings::core_u_object::FVector,
    pub falloff_left: crate::bindings::core_u_object::FVector,
    pub falloff_right: crate::bindings::core_u_object::FVector,
    pub layer_left: crate::bindings::core_u_object::FVector,
    pub layer_right: crate::bindings::core_u_object::FVector,
    pub layer_falloff_left: crate::bindings::core_u_object::FVector,
    pub layer_falloff_right: crate::bindings::core_u_object::FVector,
    pub start_end_falloff: f32,
}
#[repr(C, align(8))]
pub struct FLandscapeSplineSegmentConnection {
    pub control_point: UPtr<ULandscapeSplineControlPoint>,
    pub tangent_len: f32,
    pub socket_name: FName,
}
#[repr(C, align(8))]
pub struct FLandscapeSplineMeshEntry {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub material_overrides: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub flags_24: u8,
    pub center_adjust: crate::bindings::core_u_object::FVector2D,
    pub flags_48: u8,
    pub scale: crate::bindings::core_u_object::FVector,
    pub orientation_deprecated: LandscapeSplineMeshOrientation,
    pub forward_axis: crate::bindings::engine::ESplineMeshAxis,
    pub up_axis: crate::bindings::engine::ESplineMeshAxis,
}
#[repr(C, align(8))]
pub struct FGrassInput {
    pub name: FName,
    pub grass_type: UPtr<ULandscapeGrassType>,
    pub input: crate::bindings::engine::FExpressionInput,
}
#[repr(C, align(8))]
pub struct FLandscapeLayerBrush {
    pub blueprint_brush: UPtr<ALandscapeBlueprintBrushBase>,
}
#[repr(C, align(8))]
pub struct FLandscapeLayer {
    pub guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub name_deprecated: FName,
    pub b_visible_deprecated: bool,
    pub b_locked_deprecated: bool,
    pub heightmap_alpha_deprecated: f32,
    pub weightmap_alpha_deprecated: f32,
    pub blend_mode_deprecated: ELandscapeBlendMode,
    pub brushes: TArray<FLandscapeLayerBrush>,
    pub weightmap_layer_allocation_blend_deprecated: TMap<
        UPtr<ULandscapeLayerInfoObject>,
        bool,
    >,
    pub edit_layer: UPtr<ULandscapeEditLayerBase>,
}
#[repr(C, align(16))]
pub struct FLandscapeBrushParameters {
    pub render_area_world_transform: crate::bindings::core_u_object::FTransform,
    pub render_area_size: crate::bindings::core_u_object::FIntPoint,
    pub combined_result: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub layer_type: ELandscapeToolTargetType,
    pub weightmap_layer_name: FName,
}
#[repr(C, align(8))]
pub struct FLandscapeEditToolRenderData {
    pub tool_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub gizmo_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub selected_type: i32,
    pub debug_channel_r: i32,
    pub debug_channel_g: i32,
    pub debug_channel_b: i32,
    pub data_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub layer_contribution_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub dirty_texture: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FWeightmapLayerAllocationInfo {
    pub layer_info: UPtr<ULandscapeLayerInfoObject>,
    pub weightmap_texture_index: u8,
    pub weightmap_texture_channel: u8,
}
#[repr(C, align(8))]
pub struct FLandscapeComponentMaterialOverride {
    pub lod_index: crate::bindings::core_u_object::FPerPlatformInt,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FLandscapePerLODMaterialOverride {
    pub lod_index: i32,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FWeightmapData {
    pub textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub layer_allocations: TArray<FWeightmapLayerAllocationInfo>,
    pub texture_usages: TArray<UPtr<ULandscapeWeightmapUsage>>,
}
#[repr(C, align(8))]
pub struct FHeightmapData {
    pub texture: UPtr<crate::bindings::engine::UTexture2D>,
}
#[repr(C, align(8))]
pub struct FLandscapeLayerComponentData {
    pub debug_name: FName,
    pub heightmap_data: FHeightmapData,
    pub weightmap_data: FWeightmapData,
}
#[repr(C, align(8))]
pub struct FHeightmapTextureEdgeSnapshot {}
#[repr(C, align(8))]
pub struct FLandscapeInfoLayerSettings {
    pub layer_info_obj: UPtr<ULandscapeLayerInfoObject>,
    pub layer_name: FName,
    pub thumbnail_mic: UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    pub owner: TWeakObjectPtr<ALandscapeProxy>,
    pub debug_color_channel: i32,
}
#[repr(C, align(8))]
pub struct FLandscapeTargetLayerSettings {
    pub layer_info_obj: UPtr<ULandscapeLayerInfoObject>,
    pub reimport_layer_file_path: FString,
}
#[repr(C, align(8))]
pub struct FLandscapeEditorLayerSettings {
    pub layer_info_obj: UPtr<ULandscapeLayerInfoObject>,
    pub reimport_layer_file_path: FString,
}
#[repr(C, align(8))]
pub struct FLandscapeImportLayerInfo {
    pub layer_name: FName,
    pub layer_info: UPtr<ULandscapeLayerInfoObject>,
    pub source_file_path: FString,
}
#[repr(C, align(8))]
pub struct FLandscapeProxyMaterialOverride {
    pub lod_index: crate::bindings::core_u_object::FPerPlatformInt,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
#[repr(C, align(8))]
pub struct FLandscapeTexture2DMipMap {
    pub size_x: i32,
    pub size_y: i32,
    pub b_compressed: bool,
}
#[repr(C, align(8))]
pub struct FLayerBlendInput {
    pub layer_name: FName,
    pub blend_type: ELandscapeLayerBlendType,
    pub layer_input: crate::bindings::engine::FExpressionInput,
    pub height_input: crate::bindings::engine::FExpressionInput,
    pub preview_weight: f32,
    pub const_layer_input: crate::bindings::core_u_object::FVector,
    pub const_height_input: f32,
}
#[repr(C, align(8))]
pub struct FPhysicalMaterialInput {
    pub physical_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub input: crate::bindings::engine::FExpressionInput,
}
pub struct UControlPointMeshComponent {
    pub flags_1888: u8,
    pub virtual_texture_main_pass_max_draw_distance: f32,
}
pub struct ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {}
pub struct ILandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {}
pub struct ULandscapeSplineInterface {}
pub struct ILandscapeSplineInterface {}
pub struct ULandscapeEditLayerRenderer {}
pub struct ILandscapeEditLayerRenderer {}
pub struct ALandscapeGizmoActor {
    pub width: f32,
    pub height: f32,
    pub length_z: f32,
    pub margin_z: f32,
    pub min_relative_z: f32,
    pub relative_scale_z: f32,
    pub target_landscape_info: TWeakObjectPtr<ULandscapeInfo>,
    pub sprite_component: UPtr<crate::bindings::engine::UBillboardComponent>,
}
pub struct ALandscapeGizmoActiveActor {
    pub data_type: ELandscapeGizmoType,
    pub gizmo_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub texture_scale: crate::bindings::core_u_object::FVector2D,
    pub sampled_height: TArray<crate::bindings::core_u_object::FVector>,
    pub sampled_normal: TArray<crate::bindings::core_u_object::FVector>,
    pub sample_size_x: i32,
    pub sample_size_y: i32,
    pub cached_width: f32,
    pub cached_height: f32,
    pub cached_scale_xy: f32,
    pub frustum_verts: crate::bindings::core_u_object::FVector,
    pub gizmo_material: UPtr<crate::bindings::engine::UMaterial>,
    pub gizmo_data_material: UPtr<crate::bindings::engine::UMaterialInstance>,
    pub gizmo_mesh_material: UPtr<crate::bindings::engine::UMaterial>,
    pub layer_infos: TArray<UPtr<ULandscapeLayerInfoObject>>,
    pub snap_type: ELandscapeGizmoSnapType,
    pub b_follow_terrain_height: bool,
    pub unsnapped_rotation: crate::bindings::core_u_object::FRotator,
}
pub struct ULandscapeGizmoRenderComponent {}
pub struct ULandscapeGrassType {
    pub grass_varieties: TArray<FGrassVariety>,
    pub flags_64: u8,
    pub state_hash: u32,
    pub grass_mesh_deprecated: UPtr<crate::bindings::engine::UStaticMesh>,
    pub grass_density_deprecated: f32,
    pub placement_jitter_deprecated: f32,
    pub start_cull_distance_deprecated: i32,
    pub end_cull_distance_deprecated: i32,
    pub random_rotation_deprecated: bool,
    pub align_to_surface_deprecated: bool,
}
pub struct ALandscapeProxy {
    pub spline_component: UPtr<ULandscapeSplinesComponent>,
    pub landscape_guid: crate::bindings::core_u_object::FGuid,
    pub original_landscape_guid: crate::bindings::core_u_object::FGuid,
    pub b_enable_nanite: bool,
    pub per_lod_override_materials: TArray<FLandscapePerLODMaterialOverride>,
    pub pre_edit_per_lod_override_materials_deprecated: TArray<
        FLandscapePerLODMaterialOverride,
    >,
    pub nanite_lod_index: i32,
    pub b_nanite_skirt_enabled: bool,
    pub nanite_skirt_depth: f32,
    pub nanite_position_precision: i32,
    pub nanite_max_edge_length_factor: f32,
    pub b_disable_runtime_grass_map_generation: bool,
    pub target_layers_for_fixup: TMap<FName, UPtr<ULandscapeLayerInfoObject>>,
    pub max_lod_level: i32,
    pub lod_distance_factor_deprecated: f32,
    pub lod_falloff_deprecated: ELandscapeLODFalloff,
    pub lod0_screen_size: f32,
    pub lod_group_key: u32,
    pub lod0_distribution_setting: f32,
    pub lod_distribution_setting: f32,
    pub scalable_lod0_screen_size: crate::bindings::engine::FPerQualityLevelFloat,
    pub scalable_lod0_distribution_setting: crate::bindings::engine::FPerQualityLevelFloat,
    pub scalable_lod_distribution_setting: crate::bindings::engine::FPerQualityLevelFloat,
    pub b_use_scalable_lod_settings: bool,
    pub lod_blend_range: f32,
    pub export_lod: i32,
    pub static_lighting_lod: i32,
    pub default_phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub streaming_distance_multiplier: f32,
    pub landscape_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub landscape_hole_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub landscape_materials_override_deprecated: TArray<FLandscapeProxyMaterialOverride>,
    pub pre_edit_landscape_material_deprecated: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub pre_edit_landscape_hole_material_deprecated: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub b_is_performing_interactive_action_on_landscape_material_override_deprecated: bool,
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub b_set_create_runtime_virtual_texture_volumes: bool,
    pub b_virtual_texture_render_with_quad: bool,
    pub b_virtual_texture_render_with_quad_hq: bool,
    pub virtual_texture_num_lods: i32,
    pub virtual_texture_lod_bias: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    pub negative_z_bounds_extension: f32,
    pub positive_z_bounds_extension: f32,
    pub landscape_components: TArray<UPtr<ULandscapeComponent>>,
    pub collision_components: TArray<UPtr<ULandscapeHeightfieldCollisionComponent>>,
    pub foliage_components: TArray<
        UPtr<crate::bindings::engine::UHierarchicalInstancedStaticMeshComponent>,
    >,
    pub nanite_component_deprecated: UPtr<ULandscapeNaniteComponent>,
    pub nanite_components: TArray<UPtr<ULandscapeNaniteComponent>>,
    pub grass_types_max_discard_distance: f32,
    pub static_lighting_resolution: f32,
    pub flags_2164: u8,
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub flags_2166: u8,
    pub flags_2168: u8,
    pub flags_2172: u8,
    pub flags_2176: u8,
    pub flags_2180: u8,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    pub flags_2184: u8,
    pub non_nanite_virtual_shadow_map_constant_depth_bias: f32,
    pub non_nanite_virtual_shadow_map_invalidation_height_error_threshold: f32,
    pub non_nanite_virtual_shadow_map_invalidation_screen_size_limit: f32,
    pub flags_2200: u8,
    pub flags_2204: u8,
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub ld_max_draw_distance: f32,
    pub lightmass_settings: crate::bindings::engine::FLightmassPrimitiveSettings,
    pub collision_mip_level: i32,
    pub simple_collision_mip_level: i32,
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub flags_2688: u8,
    pub editor_cached_layer_infos_deprecated: TArray<UPtr<ULandscapeLayerInfoObject>>,
    pub reimport_heightmap_file_path: FString,
    pub reimport_destination_layer_guid: crate::bindings::core_u_object::FGuid,
    pub editor_layer_settings_deprecated: TArray<FLandscapeEditorLayerSettings>,
    pub weightmap_usage_map: TMap<
        UPtr<crate::bindings::engine::UTexture2D>,
        UPtr<ULandscapeWeightmapUsage>,
    >,
    pub registered_to_subsystem: UPtr<ULandscapeSubsystem>,
    pub component_size_quads: i32,
    pub subsection_size_quads: i32,
    pub num_subsections: i32,
    pub flags_3036: u8,
    pub navigation_geometry_gathering_mode: crate::bindings::engine::ENavDataGatheringMode,
    pub b_use_dynamic_material_instance: bool,
    pub max_painted_layers_per_component: i32,
    pub b_use_landscape_for_culling_invisible_hlod_vertices: bool,
    pub hlod_texture_size_policy: ELandscapeHLODTextureSizePolicy,
    pub hlod_texture_size: i32,
    pub hlod_material_override: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub hlod_mesh_source_lod_policy: ELandscapeHLODMeshSourceLODPolicy,
    pub hlod_mesh_source_lod: i32,
    pub b_has_layers_content_deprecated: bool,
    pub b_use_compressed_heightmap_storage_deprecated: bool,
    pub b_strip_physics_when_cooked_client: bool,
    pub b_strip_physics_when_cooked_server: bool,
    pub b_strip_grass_when_cooked_client: bool,
    pub b_strip_grass_when_cooked_server: bool,
    pub target_layers: TMap<FName, FLandscapeTargetLayerSettings>,
    pub section_base: crate::bindings::core_u_object::FIntPoint,
}
pub struct ULandscapeHeightfieldCollisionComponent {
    pub component_layer_infos: TArray<UPtr<ULandscapeLayerInfoObject>>,
    pub section_base_x: i32,
    pub section_base_y: i32,
    pub collision_size_quads: i32,
    pub collision_scale: f32,
    pub simple_collision_size_quads: i32,
    pub collision_quad_flags: TArray<u8>,
    pub heightfield_guid: crate::bindings::core_u_object::FGuid,
    pub cached_local_box: crate::bindings::core_u_object::FBox,
    pub render_component_deprecated: TLazyObjectPtr<ULandscapeComponent>,
    pub render_component_ref: UPtr<ULandscapeComponent>,
    pub collision_hash: u32,
    pub physical_material_render_objects: TArray<
        UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    >,
    pub cooked_physical_materials: TArray<
        UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    >,
}
pub struct ULandscapeMaterialInstanceConstant {
    pub texture_streaming_info: TArray<FLandscapeMaterialTextureStreamingInfo>,
    pub flags_1864: u8,
}
pub struct ULandscapeMeshCollisionComponent_DEPRECATED {
    pub mesh_guid: crate::bindings::core_u_object::FGuid,
}
pub struct ALandscapeMeshProxyActor {
    pub landscape_mesh_proxy_component: UPtr<ULandscapeMeshProxyComponent>,
}
pub struct ULandscapeMeshProxyComponent {
    pub landscape_guid: crate::bindings::core_u_object::FGuid,
    pub proxy_component_bases: TArray<crate::bindings::core_u_object::FIntPoint>,
    pub proxy_component_centers_object_space: TArray<
        crate::bindings::core_u_object::FVector,
    >,
    pub component_x_vector_object_space: crate::bindings::core_u_object::FVector,
    pub component_y_vector_object_space: crate::bindings::core_u_object::FVector,
    pub component_resolution: i32,
    pub proxy_lod: i8,
    pub lod_group_key: u32,
}
pub struct ULandscapeSplinesComponent {
    pub spline_resolution: f32,
    pub spline_color: crate::bindings::core_u_object::FColor,
    pub control_point_sprite: UPtr<crate::bindings::engine::UTexture2D>,
    pub spline_editor_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub flags_1528: u8,
    pub control_points: TArray<UPtr<ULandscapeSplineControlPoint>>,
    pub segments: TArray<UPtr<ULandscapeSplineSegment>>,
    pub foreign_world_spline_data_map: TMap<
        TSoftObjectPtr<crate::bindings::engine::UWorld>,
        FForeignWorldSplineData,
    >,
    pub cooked_foreign_mesh_components: TArray<
        UPtr<crate::bindings::engine::UMeshComponent>,
    >,
}
pub struct ULandscapeSplineControlPoint {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub width: f32,
    pub layer_width_ratio: f32,
    pub side_falloff: f32,
    pub left_side_falloff_factor: f32,
    pub right_side_falloff_factor: f32,
    pub left_side_layer_falloff_factor: f32,
    pub right_side_layer_falloff_factor: f32,
    pub end_falloff: f32,
    pub segment_mesh_offset: f32,
    pub layer_name: FName,
    pub flags_144: u8,
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub material_overrides: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub mesh_scale: crate::bindings::core_u_object::FVector,
    pub flags_200: u8,
    pub collision_profile_name_deprecated: FName,
    pub flags_216: u8,
    pub flags_220: u8,
    pub flags_224: u8,
    pub ld_max_draw_distance: f32,
    pub translucency_sort_priority: i32,
    pub flags_236: u8,
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub virtual_texture_lod_bias: i32,
    pub virtual_texture_cull_mips: i32,
    pub virtual_texture_main_pass_max_draw_distance: f32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub flags_712: u8,
    pub connected_segments: TArray<FLandscapeSplineConnection>,
    pub points: TArray<FLandscapeSplineInterpPoint>,
    pub bounds: crate::bindings::core_u_object::FBox,
    pub local_mesh_component: UPtr<UControlPointMeshComponent>,
    pub foreign_world: TSoftObjectPtr<crate::bindings::engine::UWorld>,
    pub modification_key: crate::bindings::core_u_object::FGuid,
}
pub struct ULandscapeSplineSegment {
    pub connections: FLandscapeSplineSegmentConnection,
    pub layer_name: FName,
    pub flags_108: u8,
    pub spline_meshes: TArray<FLandscapeSplineMeshEntry>,
    pub flags_128: u8,
    pub collision_profile_name_deprecated: FName,
    pub flags_144: u8,
    pub random_seed: i32,
    pub ld_max_draw_distance: f32,
    pub translucency_sort_priority: i32,
    pub flags_160: u8,
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub virtual_texture_lod_bias: i32,
    pub virtual_texture_cull_mips: i32,
    pub virtual_texture_main_pass_max_draw_distance: f32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub flags_632: u8,
    pub spline_info: crate::bindings::core_u_object::FInterpCurveVector,
    pub points: TArray<FLandscapeSplineInterpPoint>,
    pub bounds: crate::bindings::core_u_object::FBox,
    pub local_mesh_components: TArray<
        UPtr<crate::bindings::engine::USplineMeshComponent>,
    >,
    pub foreign_worlds: TArray<TSoftObjectPtr<crate::bindings::engine::UWorld>>,
    pub modification_key: crate::bindings::core_u_object::FGuid,
}
pub struct ALandscapeStreamingProxy {
    pub landscape_actor_deprecated: TLazyObjectPtr<ALandscape>,
    pub landscape_actor_ref: TSoftObjectPtr<ALandscape>,
    pub overridden_shared_properties: TSet<FName>,
}
pub struct ULandscapeWeightmapUsage {
    pub channel_usage: UPtr<ULandscapeComponent>,
    pub layer_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UMaterialExpressionLandscapeGrassOutput {
    pub grass_types: TArray<FGrassInput>,
}
pub struct AControlPointMeshActor {
    pub control_point_mesh_component: UPtr<UControlPointMeshComponent>,
}
pub struct ALandscape {
    pub b_are_new_landscape_actors_spatially_loaded: bool,
    pub b_include_grid_size_in_name_for_landscape_actors: bool,
    pub b_can_have_layers_content_deprecated: bool,
    pub b_use_generated_landscape_spline_meshes_actors: bool,
    pub landscape_splines_target_layer_guid_deprecated: crate::bindings::core_u_object::FGuid,
    pub selected_edit_layer_index: i32,
    pub b_enable_editor_layers_tick: bool,
    pub b_warned_global_merge_dimensions_exceeded: bool,
    pub landscape_layers_deprecated: TArray<FLandscapeLayer>,
    pub heightmap_rt_list_deprecated: TArray<
        UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    >,
    pub weightmap_rt_list_deprecated: TArray<
        UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    >,
    pub tracked_streaming_in_textures: TArray<
        TWeakObjectPtr<crate::bindings::engine::UTexture2D>,
    >,
    pub target_display_order_list: TArray<FName>,
    pub target_display_order: ELandscapeLayerDisplayMode,
    pub landscape_edit_layers: TArray<FLandscapeLayer>,
    pub landscape_splines_affected_components: TSet<UPtr<ULandscapeComponent>>,
    pub b_landscape_layers_are_initialized: bool,
    pub was_compiling_shaders: bool,
    pub layer_content_update_modes: u32,
    pub b_spline_layer_update_requested: bool,
}
pub struct ALandscapeBlueprintBrushBase {
    pub update_on_property_change: bool,
    pub affect_heightmap: bool,
    pub affect_weightmap: bool,
    pub affect_visibility_layer: bool,
    pub affected_weightmap_layers: TArray<FName>,
    pub b_use_power_of_two_render_target: bool,
    pub owning_landscape: UPtr<ALandscape>,
    pub b_is_visible: bool,
}
pub struct ULandscapeLODStreamingProxy_DEPRECATED {}
pub struct ULandscapeComponent {
    pub section_base_x: i32,
    pub section_base_y: i32,
    pub component_size_quads: i32,
    pub subsection_size_quads: i32,
    pub num_subsections: i32,
    pub override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub override_hole_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub override_materials_deprecated: TArray<FLandscapeComponentMaterialOverride>,
    pub material_instance_deprecated: UPtr<
        crate::bindings::engine::UMaterialInstanceConstant,
    >,
    pub material_instances: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    >,
    pub material_instances_dynamic: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    >,
    pub lod_index_to_material_index: TArray<i8>,
    pub xy_offsetmap_texture_deprecated: UPtr<crate::bindings::engine::UTexture2D>,
    pub weightmap_scale_bias: crate::bindings::core_u_object::FVector4,
    pub weightmap_subsection_offset: f32,
    pub heightmap_scale_bias: crate::bindings::core_u_object::FVector4,
    pub cached_local_box: crate::bindings::core_u_object::FBox,
    pub mip_to_mip_max_deltas: TArray<f64>,
    pub collision_component_deprecated: TLazyObjectPtr<
        ULandscapeHeightfieldCollisionComponent,
    >,
    pub collision_component_ref: UPtr<ULandscapeHeightfieldCollisionComponent>,
    pub b_user_triggered_change_requested: bool,
    pub b_nanite_active: bool,
    pub lighting_guid: crate::bindings::core_u_object::FGuid,
    pub layers_data: TMap<
        crate::bindings::core_u_object::FGuid,
        FLandscapeLayerComponentData,
    >,
    pub obsolete_edit_layer_data: TMap<
        crate::bindings::core_u_object::FGuid,
        FLandscapeLayerComponentData,
    >,
    pub weightmap_textures_usage: TArray<UPtr<ULandscapeWeightmapUsage>>,
    pub layer_update_flag_per_mode: u32,
    pub b_pending_collision_data_update: bool,
    pub b_pending_layer_collision_data_update: bool,
    pub heightmap_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub weightmap_layer_allocations: TArray<FWeightmapLayerAllocationInfo>,
    pub weightmap_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub per_lod_override_materials: TArray<FLandscapePerLODMaterialOverride>,
    pub grass_types: TArray<UPtr<ULandscapeGrassType>>,
    pub map_build_data_id: crate::bindings::core_u_object::FGuid,
    pub collision_mip_level: i32,
    pub simple_collision_mip_level: i32,
    pub negative_z_bounds_extension: f32,
    pub positive_z_bounds_extension: f32,
    pub static_lighting_resolution: f32,
    pub forced_lod: i32,
    pub lod_bias: i32,
    pub registered_edge_fixup: UPtr<ULandscapeHeightmapTextureEdgeFixup>,
    pub irrelevant_lights_deprecated: TArray<crate::bindings::core_u_object::FGuid>,
    pub lighting_lod_bias: i32,
    pub layer_allow_list: TArray<UPtr<ULandscapeLayerInfoObject>>,
    pub edit_tool_render_data: FLandscapeEditToolRenderData,
    pub mobile_data_source_hash: crate::bindings::core_u_object::FGuid,
    pub material_per_lod: TMap<UPtr<crate::bindings::engine::UMaterialInterface>, i8>,
    pub spline_hash: u32,
    pub physical_material_hash: u32,
    pub last_saved_physical_material_hash: u32,
    pub mobile_material_interface_deprecated: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub mobile_combination_material_instance_deprecated: UPtr<
        crate::bindings::engine::UMaterialInstanceConstant,
    >,
    pub mobile_material_interfaces: TArray<
        UPtr<crate::bindings::engine::UMaterialInterface>,
    >,
    pub mobile_weightmap_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
    pub mobile_weightmap_texture_array: UPtr<crate::bindings::engine::UTexture2DArray>,
    pub mobile_weightmap_layer_allocations: TArray<FWeightmapLayerAllocationInfo>,
    pub mobile_combination_material_instances: TArray<
        UPtr<crate::bindings::engine::UMaterialInstanceConstant>,
    >,
}
pub struct ULandscapeHeightmapTextureEdgeFixup {}
pub struct ULandscapeEditLayerBase {
    pub guid: crate::bindings::core_u_object::FGuid,
    pub layer_name: FName,
    pub b_visible: bool,
    pub b_locked: bool,
    pub heightmap_alpha: f32,
    pub weightmap_alpha: f32,
    pub owning_landscape: TWeakObjectPtr<ALandscape>,
    pub weightmap_layer_allocation_blend: TMap<UPtr<ULandscapeLayerInfoObject>, bool>,
}
pub struct ULandscapeEditLayerPersistent {}
pub struct ULandscapeEditLayer {}
pub struct ULandscapeEditLayerProcedural {}
pub struct ULandscapeEditLayerSplines {}
pub struct ULandscapeDefaultEditLayerRenderer {}
pub struct ULandscapeHeightmapNormalsEditLayerRenderer {}
pub struct ULandscapeWeightmapWeightBlendedLayersRenderer {}
pub struct ULandscapeScratchRenderTarget {
    pub render_target: UPtr<crate::bindings::engine::UTextureRenderTarget>,
}
pub struct ULandscapeEditResourcesSubsystem {
    pub scratch_render_targets: TArray<UPtr<ULandscapeScratchRenderTarget>>,
    pub layer_debug_color_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub selection_color_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub selection_region_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub mask_region_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub color_mask_region_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub landscape_black_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub landscape_layer_usage_material: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub landscape_dirty_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
pub struct ULandscapeHLODBuilder {}
pub struct ULandscapeInfo {
    pub landscape_actor: TWeakObjectPtr<ALandscape>,
    pub landscape_guid: crate::bindings::core_u_object::FGuid,
    pub component_size_quads: i32,
    pub subsection_size_quads: i32,
    pub component_num_subsections: i32,
    pub b_draw_scale_set_by_actor: bool,
    pub draw_scale: crate::bindings::core_u_object::FVector,
    pub layers: TArray<FLandscapeInfoLayerSettings>,
    pub region_size_in_components: i32,
    pub streaming_proxies_deprecated: TArray<TWeakObjectPtr<ALandscapeStreamingProxy>>,
    pub spline_actors: TArray<TScriptInterface<ILandscapeSplineInterface>>,
    pub sorted_streaming_proxies: TArray<TWeakObjectPtr<ALandscapeStreamingProxy>>,
}
pub struct ULandscapeInfoMap {}
pub struct ULandscapeLayerInfoObject {
    pub layer_name: FName,
    pub phys_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
    pub hardness: f32,
    pub minimum_collision_relevance_weight: f32,
    pub flags_80: u8,
    pub spline_falloff_modulation_texture: UPtr<crate::bindings::engine::UTexture2D>,
    pub spline_falloff_modulation_color_mask: ESplineModulationColorMask,
    pub spline_falloff_modulation_tiling: f32,
    pub spline_falloff_modulation_bias: f32,
    pub spline_falloff_modulation_scale: f32,
    pub is_referenced_from_loaded_data: bool,
    pub layer_usage_debug_color: crate::bindings::core_u_object::FLinearColor,
    pub blend_method: ELandscapeTargetLayerBlendMethod,
    pub blend_group: FName,
}
pub struct ULandscapeNaniteComponent {
    pub proxy_content_id: crate::bindings::core_u_object::FGuid,
    pub b_enabled: bool,
    pub source_landscape_components: TArray<UPtr<ULandscapeComponent>>,
}
pub struct ULandscapeSettings {
    pub max_number_of_layers: i32,
    pub b_show_dialog_for_automatic_layer_creation: bool,
    pub max_components: i32,
    pub max_image_import_cache_size_mega_bytes: u32,
    pub paint_strength_gamma: f32,
    pub b_disable_painting_startup_slowdown: bool,
    pub landscape_dirtying_mode: ELandscapeDirtyingMode,
    pub side_resolution_limit: i32,
    pub default_landscape_material: TSoftObjectPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub default_layer_info_object: TSoftObjectPtr<ULandscapeLayerInfoObject>,
    pub brush_size_ui_max: f32,
    pub brush_size_clamp_max: f32,
    pub hlod_max_texture_size: i32,
    pub b_should_update_edit_layers_during_interactive_changes: bool,
    pub b_restrictive_mode: bool,
    pub spline_icon_world_z_offset: f32,
    pub spline_icon_scale: f32,
    pub b_display_target_layer_thumbnails: bool,
    pub b_disable_temporal_anti_aliasing_in_landscape_mode: bool,
    pub target_layer_default_blend_method: ELandscapeTargetLayerBlendMethod,
}
pub struct ALandscapeSplineActor {
    pub landscape_guid: crate::bindings::core_u_object::FGuid,
    pub landscape_actor: UPtr<ALandscape>,
}
pub struct ALandscapeSplineMeshesActor {
    pub static_mesh_components: TArray<
        UPtr<crate::bindings::engine::UStaticMeshComponent>,
    >,
    pub grid_guid: crate::bindings::core_u_object::FGuid,
}
pub struct ULandscapeSubsystem {
    pub streaming_proxies_needing_reregister: TSet<UPtr<ALandscapeStreamingProxy>>,
    pub landscape_actors: TArray<UPtr<ALandscape>>,
    pub proxies: TArray<UPtr<ALandscapeProxy>>,
}
pub struct ULandscapeTextureHash {
    pub texture_hash_guid: crate::bindings::core_u_object::FGuid,
    pub last_source_id: crate::bindings::core_u_object::FGuid,
    pub texture_type: ELandscapeTextureType,
    pub texture_usage: ELandscapeTextureUsage,
}
pub struct ULandscapeTextureMipEdgeOverrideFactory {}
pub struct ULandscapeTextureStorageProviderFactory {}
pub struct UMaterialExpressionLandscapeLayerBlend {
    pub layers: TArray<FLayerBlendInput>,
}
pub struct UMaterialExpressionLandscapeLayerCoords {
    pub mapping_type: ETerrainCoordMappingType,
    pub custom_uv_type: ELandscapeCustomizedCoordType,
    pub mapping_scale: f32,
    pub mapping_rotation: f32,
    pub mapping_pan_u: f32,
    pub mapping_pan_v: f32,
}
pub struct UMaterialExpressionLandscapeLayerSample {
    pub parameter_name: FName,
    pub preview_weight: f32,
}
pub struct UMaterialExpressionLandscapeLayerSwitch {
    pub layer_used: crate::bindings::engine::FExpressionInput,
    pub layer_not_used: crate::bindings::engine::FExpressionInput,
    pub parameter_name: FName,
    pub flags_308: u8,
}
pub struct UMaterialExpressionLandscapeLayerWeight {
    pub base: crate::bindings::engine::FExpressionInput,
    pub layer: crate::bindings::engine::FExpressionInput,
    pub parameter_name: FName,
    pub preview_weight: f32,
    pub const_base: crate::bindings::core_u_object::FVector,
}
pub struct UMaterialExpressionLandscapePhysicalMaterialOutput {
    pub inputs: TArray<FPhysicalMaterialInput>,
}
pub struct UMaterialExpressionLandscapeVisibilityMask {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EGrassScaling(pub u8);
impl EGrassScaling {
    pub const UNIFORM: EGrassScaling = EGrassScaling(0);
    pub const FREE: EGrassScaling = EGrassScaling(1);
    pub const LOCK_XY: EGrassScaling = EGrassScaling(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct LandscapeSplineMeshOrientation(pub u8);
impl LandscapeSplineMeshOrientation {
    pub const LSMO_X_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        0,
    );
    pub const LSMO_Y_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeBlendMode(pub u8);
impl ELandscapeBlendMode {
    pub const LSBM_ADDITIVE_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(0);
    pub const LSBM_ALPHA_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeToolTargetType(pub u8);
impl ELandscapeToolTargetType {
    pub const HEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(0);
    pub const WEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(1);
    pub const VISIBILITY: ELandscapeToolTargetType = ELandscapeToolTargetType(2);
    pub const INVALID: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
    pub const COUNT: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeLayerBlendType(pub u8);
impl ELandscapeLayerBlendType {
    pub const LB_WEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(0);
    pub const LB_ALPHA_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(1);
    pub const LB_HEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeGizmoType(pub u8);
impl ELandscapeGizmoType {
    pub const LGT_NONE: ELandscapeGizmoType = ELandscapeGizmoType(0);
    pub const LGT_HEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(1);
    pub const LGT_WEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeGizmoSnapType(pub i32);
impl ELandscapeGizmoSnapType {
    pub const NONE: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(0);
    pub const COMPONENT: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(1);
    pub const TEXEL: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeLODFalloff(pub u8);
impl ELandscapeLODFalloff {
    pub const LINEAR: ELandscapeLODFalloff = ELandscapeLODFalloff(0);
    pub const SQUARE_ROOT: ELandscapeLODFalloff = ELandscapeLODFalloff(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeHLODTextureSizePolicy(pub u8);
impl ELandscapeHLODTextureSizePolicy {
    pub const AUTOMATIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        0,
    );
    pub const SPECIFIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        1,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeHLODMeshSourceLODPolicy(pub u8);
impl ELandscapeHLODMeshSourceLODPolicy {
    pub const AUTOMATIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        0,
    );
    pub const SPECIFIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        1,
    );
    pub const LOWEST_DETAIL_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeLayerDisplayMode(pub u8);
impl ELandscapeLayerDisplayMode {
    pub const DEFAULT: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(0);
    pub const ALPHABETICAL: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(1);
    pub const USER_SPECIFIC: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(2);
    pub const BY_BLEND_METHOD: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESplineModulationColorMask(pub u8);
impl ESplineModulationColorMask {
    pub const RED: ESplineModulationColorMask = ESplineModulationColorMask(0);
    pub const GREEN: ESplineModulationColorMask = ESplineModulationColorMask(1);
    pub const BLUE: ESplineModulationColorMask = ESplineModulationColorMask(2);
    pub const ALPHA: ESplineModulationColorMask = ESplineModulationColorMask(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeTargetLayerBlendMethod(pub u8);
impl ELandscapeTargetLayerBlendMethod {
    pub const NONE: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        0,
    );
    pub const FINAL_WEIGHT_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        1,
    );
    pub const PREMULTIPLIED_ALPHA_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        2,
    );
    pub const COUNT: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        3,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeDirtyingMode(pub u8);
impl ELandscapeDirtyingMode {
    pub const AUTO: ELandscapeDirtyingMode = ELandscapeDirtyingMode(0);
    pub const IN_LANDSCAPE_MODE_ONLY: ELandscapeDirtyingMode = ELandscapeDirtyingMode(1);
    pub const IN_LANDSCAPE_MODE_AND_USER_TRIGGERED_CHANGES: ELandscapeDirtyingMode = ELandscapeDirtyingMode(
        2,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeTextureType(pub u8);
impl ELandscapeTextureType {
    pub const UNKNOWN: ELandscapeTextureType = ELandscapeTextureType(0);
    pub const HEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(1);
    pub const WEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeTextureUsage(pub u8);
impl ELandscapeTextureUsage {
    pub const UNKNOWN: ELandscapeTextureUsage = ELandscapeTextureUsage(0);
    pub const EDIT_LAYER_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(1);
    pub const FINAL_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETerrainCoordMappingType(pub u8);
impl ETerrainCoordMappingType {
    pub const TCMT_AUTO: ETerrainCoordMappingType = ETerrainCoordMappingType(0);
    pub const TCMT_XY: ETerrainCoordMappingType = ETerrainCoordMappingType(1);
    pub const TCMT_XZ: ETerrainCoordMappingType = ETerrainCoordMappingType(2);
    pub const TCMT_YZ: ETerrainCoordMappingType = ETerrainCoordMappingType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeCustomizedCoordType(pub u8);
impl ELandscapeCustomizedCoordType {
    pub const LCCT_NONE: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        0,
    );
    pub const LCCT_CUSTOM_UV0: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        1,
    );
    pub const LCCT_CUSTOM_UV1: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        2,
    );
    pub const LCCT_CUSTOM_UV2: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        3,
    );
    pub const LCCT_WEIGHT_MAP_UV: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        4,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeImportAlphamapType(pub u8);
impl ELandscapeImportAlphamapType {
    pub const ADDITIVE: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(0);
    pub const LAYERED: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ELandscapeLayerPaintingRestriction(pub u8);
impl ELandscapeLayerPaintingRestriction {
    pub const NONE: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        0,
    );
    pub const USE_MAX_LAYERS: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        1,
    );
    pub const EXISTING_ONLY: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        2,
    );
    pub const USE_COMPONENT_ALLOW_LIST: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        3,
    );
}
