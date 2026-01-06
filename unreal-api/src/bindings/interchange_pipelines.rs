#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_INTERCHANGE_GENERIC_AUDIO_PIPELINE_CREATE_SOUND_WAVE_FACTORY_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_GENERIC_MESH_PIPELINE_SET_COMBINE_SKELETAL_MESHES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_GENERIC_MESH_PIPELINE_GET_COMBINE_SKELETAL_MESHES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_SET_CONTEXT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_IS_VALID_MESH_INSTANCE_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_IS_VALID_MESH_GEOMETRY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_INSTANCE_SKELETON_ROOT_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_INSTANCE_BY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_GEOMETRY_SKELETON_ROOT_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_GEOMETRY_BY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_STATIC_MESH_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_STATIC_MESH_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_SKINNED_MESH_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_SKINNED_MESH_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_INSTANCE_UIDS_USING_MESH_GEOMETRY_UID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_INSTANCE_UIDS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_GEOMETRY_NOT_INSTANCED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_GEOMETRY_CACHE_INSTANCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_GEOMETRY_CACHE_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_CREATE_INTERCHANGE_PIPELINE_MESHES_UTILITIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeGenericAudioPipeline::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateSoundWaveFactoryNode"),
            &raw mut U_INTERCHANGE_GENERIC_AUDIO_PIPELINE_CREATE_SOUND_WAVE_FACTORY_NODE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeGenericMeshPipeline::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCombineSkeletalMeshes"),
            &raw mut U_INTERCHANGE_GENERIC_MESH_PIPELINE_SET_COMBINE_SKELETAL_MESHES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCombineSkeletalMeshes"),
            &raw mut U_INTERCHANGE_GENERIC_MESH_PIPELINE_GET_COMBINE_SKELETAL_MESHES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePipelineMeshesUtilities::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetContext"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_SET_CONTEXT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidMeshInstanceUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_IS_VALID_MESH_INSTANCE_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidMeshGeometryUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_IS_VALID_MESH_GEOMETRY_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshInstanceSkeletonRootUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_INSTANCE_SKELETON_ROOT_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshInstanceByUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_INSTANCE_BY_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshGeometrySkeletonRootUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_GEOMETRY_SKELETON_ROOT_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshGeometryByUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_MESH_GEOMETRY_BY_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllStaticMeshInstance"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_STATIC_MESH_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllStaticMeshGeometry"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_STATIC_MESH_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllSkinnedMeshInstance"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_SKINNED_MESH_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllSkinnedMeshGeometry"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_SKINNED_MESH_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllMeshInstanceUidsUsingMeshGeometryUid"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_INSTANCE_UIDS_USING_MESH_GEOMETRY_UID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllMeshInstanceUids"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_INSTANCE_UIDS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllMeshGeometryNotInstanced"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_GEOMETRY_NOT_INSTANCED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllMeshGeometry"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_MESH_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllGeometryCacheInstance"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_GEOMETRY_CACHE_INSTANCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllGeometryCacheGeometry"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_GET_ALL_GEOMETRY_CACHE_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateInterchangePipelineMeshesUtilities"),
            &raw mut U_INTERCHANGE_PIPELINE_MESHES_UTILITIES_CREATE_INTERCHANGE_PIPELINE_MESHES_UTILITIES,
        );
    }
}
#[repr(C, align(8))]
pub struct FInterchangeLodSceneNodeContainer {
    __padding_end: [u8; 16],
}
impl FInterchangeLodSceneNodeContainer {}
#[repr(C, align(8))]
pub struct FInterchangeMeshInstance {
    __padding_end: [u8; 128],
}
impl FInterchangeMeshInstance {}
#[repr(C, align(8))]
pub struct FInterchangeMeshGeometry {
    __padding_end: [u8; 56],
}
impl FInterchangeMeshGeometry {}
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
impl FInterchangePipelineMeshesUtilitiesContext {}
#[repr(C, align(8))]
pub struct UInterchangeGenericCommonMeshesProperties {
    #[doc(hidden)]
    __padding_344: [u8; 344],
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
    __padding_end: [u8; 3],
}
impl UInterchangeGenericCommonMeshesProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericCommonMeshesProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub b_import_only_animations: bool,
    pub skeleton: TWeakObjectPtr<crate::bindings::engine::USkeleton>,
    pub b_import_meshes_in_bone_hierarchy: bool,
    pub b_use_t0_as_ref_pose: bool,
    pub b_add_curve_metadata_to_skeleton: bool,
    pub b_convert_statics_with_morph_targets_to_skeletals: bool,
}
impl UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericCommonSkeletalMeshesAndAnimationsProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericAnimationPipeline {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub b_import_animations: bool,
    pub b_import_bone_tracks: bool,
    pub animation_range: EInterchangeAnimationRange,
    pub frame_import_range: crate::bindings::core_u_object::FInt32Interval,
    pub b_use30_hz_to_bake_bone_animation: bool,
    pub custom_bone_animation_sample_rate: i32,
    pub b_snap_to_closest_frame_boundary: bool,
    pub b_import_custom_attribute: bool,
    #[doc(hidden)]
    __padding_383: [u8; 1],
    pub b_set_material_drive_parameter_on_custom_attribute: bool,
    pub material_curve_suffixes: TArray<FString>,
    pub b_remove_curve_redundant_keys: bool,
    pub b_do_not_import_curve_with_zero: bool,
    pub b_delete_existing_non_curve_custom_attributes: bool,
    pub b_delete_existing_custom_attribute_curves: bool,
    pub b_delete_existing_morph_target_curves: bool,
    __padding_end: [u8; 51],
}
impl UInterchangeGenericAnimationPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericAnimationPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericAssetsPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
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
    __padding_end: [u8; 280],
}
impl UInterchangeGenericAssetsPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericAssetsPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericAudioPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub pipeline_display_name: FString,
    pub b_import_sounds: bool,
    __padding_end: [u8; 63],
}
impl UInterchangeGenericAudioPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericAudioPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericGroomPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
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
    __padding_end: [u8; 4],
}
impl UInterchangeGenericGroomPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericGroomPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericMaterialPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
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
    __padding_end: [u8; 56],
}
impl UInterchangeGenericMaterialPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericMaterialPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericMeshPipeline {
    #[doc(hidden)]
    __padding_360: [u8; 360],
    pub b_import_static_meshes: bool,
    pub b_combine_static_meshes: bool,
    pub lod_group: FName,
    pub b_auto_compute_lod_screen_sizes: bool,
    pub lod_screen_sizes: TArray<f32>,
    #[doc(hidden)]
    __padding_401: [u8; 1],
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
    #[doc(hidden)]
    __padding_475: [u8; 1],
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
    __padding_end: [u8; 176],
}
impl UInterchangeGenericMeshPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericMeshPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericLevelPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub pipeline_display_name: FString,
    pub reimport_property_strategy: crate::bindings::interchange_core::EReimportStrategyFlags,
    pub scene_hierarchy_type: EInterchangeSceneHierarchyType,
    pub b_delete_missing_actors: bool,
    pub b_force_reimport_deleted_actors: bool,
    pub b_force_reimport_deleted_assets: bool,
    pub b_delete_missing_assets: bool,
    pub b_use_hierarchical_ism_components: bool,
    pub b_use_physical_instead_of_standard_perspective_camera: bool,
    __padding_end: [u8; 216],
}
impl UInterchangeGenericLevelPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericLevelPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGenericTexturePipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub pipeline_display_name: FString,
    pub b_import_textures: bool,
    pub asset_name: FString,
    pub b_detect_normal_map_texture: bool,
    pub b_flip_normal_map_green_channel: bool,
    pub b_import_udi_ms: bool,
    pub file_extensions_to_import_as_long_lat_cubemap: TSet<FString>,
    pub b_prefer_compressed_source_data: bool,
    pub b_allow_non_power_of_two: bool,
    __padding_end: [u8; 62],
}
impl UInterchangeGenericTexturePipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGenericTexturePipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGLTFPipelineSettings {
    __padding_end: [u8; 192],
}
impl UGLTFPipelineSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFPipelineSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGLTFPipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub pipeline_display_name: FString,
    __padding_end: [u8; 8],
}
impl UInterchangeGLTFPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGLTFPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialXPipelineSettings {
    __padding_end: [u8; 432],
}
impl UMaterialXPipelineSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialXPipelineSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMaterialXPipeline {
    __padding_end: [u8; 360],
}
impl UInterchangeMaterialXPipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMaterialXPipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangePipelineMeshesUtilities {
    __padding_end: [u8; 384],
}
impl UInterchangePipelineMeshesUtilities {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineMeshesUtilities")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSparseVolumeTexturePipeline {
    #[doc(hidden)]
    __padding_344: [u8; 344],
    pub pipeline_display_name: FString,
    pub b_import_sparse_volume_textures: bool,
    pub b_import_animated_sparse_volume_textures: bool,
    pub asset_name: FString,
    __padding_end: [u8; 8],
}
impl UInterchangeSparseVolumeTexturePipeline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSparseVolumeTexturePipeline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(transparent)]
pub struct EInterchangeForceMeshType(pub u8);
impl EInterchangeForceMeshType {
    pub const IFMT_NONE: EInterchangeForceMeshType = EInterchangeForceMeshType(0);
    pub const IFMT_STATIC_MESH: EInterchangeForceMeshType = EInterchangeForceMeshType(1);
    pub const IFMT_SKELETAL_MESH: EInterchangeForceMeshType = EInterchangeForceMeshType(
        2,
    );
}
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
#[repr(transparent)]
pub struct EInterchangeAnimationRange(pub u8);
impl EInterchangeAnimationRange {
    pub const TIMELINE: EInterchangeAnimationRange = EInterchangeAnimationRange(0);
    pub const ANIMATED: EInterchangeAnimationRange = EInterchangeAnimationRange(1);
    pub const SET_RANGE: EInterchangeAnimationRange = EInterchangeAnimationRange(2);
    pub const MAX: EInterchangeAnimationRange = EInterchangeAnimationRange(3);
}
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
