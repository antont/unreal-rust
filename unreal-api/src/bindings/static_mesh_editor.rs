#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(forgetting_copy_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_static_mesh_editor_subsystem_update_nanite_source_filename: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_nanite_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lods_with_notification: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_screen_sizes: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lods: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_reduction_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_material_slot: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_group: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_from_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_lod_build_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_generate_lightmap_u_vs: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_convex_decomposition_collisions_with_notification: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_convex_decomposition_collisions: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_set_allow_cpu_access: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_replace_mesh_components_meshes_on_actors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_replace_mesh_components_meshes: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_replace_mesh_components_materials_on_actors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_replace_mesh_components_materials: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_remove_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_remove_lods: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_remove_collisions_with_notification: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_remove_collisions: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_reimport_all_custom_lo_ds: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_merge_static_mesh_actors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_join_static_mesh_actors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_is_section_visible_in_ray_tracing_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_is_section_force_opaque_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_is_section_collision_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_is_section_cast_shadow_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_is_section_affect_distance_field_lighting_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_insert_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_import_nanite_hi_res_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_import_lod: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_has_vertex_colors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_has_instance_vertex_colors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_simple_collision_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_num_uv_channels: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_number_verts: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_number_materials: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_nanite_source_filename: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_nanite_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_screen_sizes: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_reduction_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_material_slot: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_group: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_lod_build_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_convex_collision_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_get_collision_complexity: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_generate_planar_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_generate_cylindrical_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_generate_box_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_enable_section_visible_in_ray_tracing: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_enable_section_force_opaque: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_enable_section_collision: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_enable_section_cast_shadow: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_enable_section_affect_distance_field_lighting: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_create_proxy_mesh_actor: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions_with_notification: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_add_uv_channel: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_add_simple_collisions_with_notification: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_editor_subsystem_add_simple_collisions: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_static_mesh_editor_subsystem_update_nanite_source_filename: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_nanite_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lods_with_notification: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_screen_sizes: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lods: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_reduction_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_material_slot: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_group: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_from_static_mesh: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_lod_build_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_generate_lightmap_u_vs: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_convex_decomposition_collisions_with_notification: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_convex_decomposition_collisions: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_set_allow_cpu_access: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_replace_mesh_components_meshes_on_actors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_replace_mesh_components_meshes: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_replace_mesh_components_materials_on_actors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_replace_mesh_components_materials: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_remove_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_remove_lods: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_remove_collisions_with_notification: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_remove_collisions: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_reimport_all_custom_lo_ds: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_merge_static_mesh_actors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_join_static_mesh_actors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_is_section_visible_in_ray_tracing_enabled: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_is_section_force_opaque_enabled: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_is_section_collision_enabled: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_is_section_cast_shadow_enabled: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_is_section_affect_distance_field_lighting_enabled: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_insert_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_import_nanite_hi_res_mesh: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_import_lod: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_has_vertex_colors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_has_instance_vertex_colors: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_simple_collision_count: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_num_uv_channels: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_number_verts: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_number_materials: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_nanite_source_filename: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_nanite_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_screen_sizes: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_reduction_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_material_slot: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_group: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_count: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_lod_build_settings: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_convex_collision_count: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_get_collision_complexity: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_generate_planar_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_generate_cylindrical_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_generate_box_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_enable_section_visible_in_ray_tracing: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_enable_section_force_opaque: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_enable_section_collision: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_enable_section_cast_shadow: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_enable_section_affect_distance_field_lighting: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_create_proxy_mesh_actor: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions_with_notification: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_add_uv_channel: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_add_simple_collisions_with_notification: std::ptr::null_mut(),
            u_static_mesh_editor_subsystem_add_simple_collisions: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UStaticMeshEditorSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateNaniteSourceFilename"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_update_nanite_source_filename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNaniteSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_nanite_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLodsWithNotification"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lods_with_notification,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLodScreenSizes"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_screen_sizes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLods"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_set_lods,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLodReductionSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_reduction_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLODMaterialSlot"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_material_slot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLODGroup"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_set_lod_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLodFromStaticMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_from_static_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLodBuildSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_build_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGenerateLightmapUVs"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_generate_lightmap_u_vs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "SetConvexDecompositionCollisionsWithNotification",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions_with_notification,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConvexDecompositionCollisions"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAllowCPUAccess"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_allow_cpu_access,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMeshesOnActors"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes_on_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMeshes"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMaterialsOnActors"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials_on_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceMeshComponentsMaterials"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveUVChannel"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_remove_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLods"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_remove_lods,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveCollisionsWithNotification"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_collisions_with_notification,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveCollisions"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_remove_collisions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReimportAllCustomLODs"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_reimport_all_custom_lo_ds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MergeStaticMeshActors"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_merge_static_mesh_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JoinStaticMeshActors"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_join_static_mesh_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSectionVisibleInRayTracingEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_visible_in_ray_tracing_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSectionForceOpaqueEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_force_opaque_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSectionCollisionEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_collision_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSectionCastShadowEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_cast_shadow_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSectionAffectDistanceFieldLightingEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_affect_distance_field_lighting_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InsertUVChannel"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_insert_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportNaniteHiResMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_import_nanite_hi_res_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportLOD"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_import_lod,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasVertexColors"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_has_vertex_colors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasInstanceVertexColors"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_has_instance_vertex_colors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSimpleCollisionCount"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_simple_collision_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumUVChannels"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_num_uv_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumberVerts"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_get_number_verts,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumberMaterials"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_number_materials,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNaniteSourceFilename"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_source_filename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNaniteSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLodScreenSizes"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_screen_sizes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLodReductionSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_reduction_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLODMaterialSlot"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_material_slot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLODGroup"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_get_lod_group,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLodCount"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_get_lod_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLodBuildSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_build_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConvexCollisionCount"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_convex_collision_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCollisionComplexity"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_collision_complexity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GeneratePlanarUVChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_planar_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateCylindricalUVChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_cylindrical_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateBoxUVChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_box_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableSectionVisibleInRayTracing"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_visible_in_ray_tracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableSectionForceOpaque"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_force_opaque,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableSectionCollision"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_collision,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableSectionCastShadow"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_cast_shadow,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableSectionAffectDistanceFieldLighting"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_affect_distance_field_lighting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyMeshActor"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_create_proxy_mesh_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "BulkSetConvexDecompositionCollisionsWithNotification",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions_with_notification,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BulkSetConvexDecompositionCollisions"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddUVChannel"),
                &raw mut __FUNCTION_PTRS.u_static_mesh_editor_subsystem_add_uv_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSimpleCollisionsWithNotification"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions_with_notification,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSimpleCollisions"),
                &raw mut __FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FStaticMeshReductionSettings {
    pub percent_triangles: f32,
    pub screen_size: f32,
}
impl FStaticMeshReductionSettings {}
#[repr(C, align(8))]
pub struct FStaticMeshReductionOptions {
    pub b_auto_compute_lod_screen_size: bool,
    pub reduction_settings: TArray<FStaticMeshReductionSettings>,
}
impl FStaticMeshReductionOptions {}
#[repr(C, align(8))]
pub struct FJoinStaticMeshActorsOptions {
    pub b_destroy_source_actors: bool,
    pub new_actor_label: FString,
    pub b_rename_components_from_source: bool,
}
impl FJoinStaticMeshActorsOptions {}
#[repr(C, align(8))]
pub struct FMergeStaticMeshActorsOptions {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_merging_settings: crate::bindings::engine::FMeshMergingSettings,
}
impl FMergeStaticMeshActorsOptions {}
#[repr(C, align(8))]
pub struct FCreateProxyMeshActorOptions {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub b_spawn_merged_actor: bool,
    pub base_package_name: FString,
    pub mesh_proxy_settings: crate::bindings::engine::FMeshProxySettings,
}
impl FCreateProxyMeshActorOptions {}
#[repr(C, align(8))]
pub struct UStaticMeshEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UStaticMeshEditorUISubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshEditorUISubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshEditorUISubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStaticMeshEditorSubsystem {
    __padding_end: [u8; 56],
}
impl UStaticMeshEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshEditorSubsystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn update_nanite_source_filename(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        new_source_filename: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_update_nanite_source_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_source_filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_update_nanite_source_filename,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(new_source_filename);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_nanite_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        nanite_settings: crate::bindings::engine::FMeshNaniteSettings,
        b_apply_changes: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_nanite_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &nanite_settings,
                __buffer.add(8).cast::<crate::bindings::engine::FMeshNaniteSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(120).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_nanite_settings,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(nanite_settings);
        std::mem::forget(b_apply_changes);
    }
    pub fn set_lods_with_notification(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        reduction_options: &FStaticMeshReductionOptions,
        b_apply_changes: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lods_with_notification,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer.add(8).cast::<FStaticMeshReductionOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lods_with_notification,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_apply_changes);
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn set_lod_screen_sizes(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        screen_sizes: &TArray<f32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_screen_sizes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                screen_sizes,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_screen_sizes,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_lods(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        reduction_options: &FStaticMeshReductionOptions,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lods,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer.add(8).cast::<FStaticMeshReductionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lods,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn set_lod_reduction_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        reduction_options: &crate::bindings::engine::FMeshReductionSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_reduction_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduction_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FMeshReductionSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_reduction_settings,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
    }
    pub fn set_lod_material_slot(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        material_slot_index: i32,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_material_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_slot_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_material_slot,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(material_slot_index);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn set_lod_group(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_group: FName,
        b_rebuild_immediately: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_group,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lod_group,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_immediately,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_group,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_group);
        std::mem::forget(b_rebuild_immediately);
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn set_lod_from_static_mesh(
        &mut self,
        destination_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        destination_lod_index: i32,
        source_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        source_lod_index: i32,
        b_reuse_existing_material_slots: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_from_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_lod_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_static_mesh,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_lod_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reuse_existing_material_slots,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_from_static_mesh,
                __buffer,
            )
        };
        std::mem::forget(destination_static_mesh);
        std::mem::forget(destination_lod_index);
        std::mem::forget(source_static_mesh);
        std::mem::forget(source_lod_index);
        std::mem::forget(b_reuse_existing_material_slots);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn set_lod_build_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        build_options: &crate::bindings::engine::FMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                build_options,
                __buffer.add(16).cast::<crate::bindings::engine::FMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_lod_build_settings,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
    }
    pub fn set_generate_lightmap_uv(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_generate_lightmap_u_vs: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_generate_lightmap_u_vs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_generate_lightmap_u_vs,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_generate_lightmap_u_vs,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_generate_lightmap_u_vs);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_convex_decomposition_collisions_with_notification(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions_with_notification,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&hull_count, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions_with_notification,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(hull_count);
        std::mem::forget(max_hull_verts);
        std::mem::forget(hull_precision);
        std::mem::forget(b_apply_changes);
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn set_convex_decomposition_collisions(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&hull_count, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_convex_decomposition_collisions,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(hull_count);
        std::mem::forget(max_hull_verts);
        std::mem::forget(hull_precision);
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_allow_cpu_access(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_allow_cpu_access: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_allow_cpu_access,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_cpu_access,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_set_allow_cpu_access,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_allow_cpu_access);
    }
    pub fn replace_mesh_components_meshes_on_actors(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        mesh_to_be_replaced: UPtr<crate::bindings::engine::UStaticMesh>,
        new_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes_on_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_to_be_replaced,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes_on_actors,
                __buffer,
            )
        };
        std::mem::forget(mesh_to_be_replaced);
        std::mem::forget(new_mesh);
    }
    pub fn replace_mesh_components_meshes(
        &mut self,
        mesh_components: &TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
        mesh_to_be_replaced: UPtr<crate::bindings::engine::UStaticMesh>,
        new_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mesh_components,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<UPtr<crate::bindings::engine::UStaticMeshComponent>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_to_be_replaced,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_meshes,
                __buffer,
            )
        };
        std::mem::forget(mesh_to_be_replaced);
        std::mem::forget(new_mesh);
    }
    pub fn replace_mesh_components_materials_on_actors(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        material_to_be_replaced: UPtr<crate::bindings::engine::UMaterialInterface>,
        new_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials_on_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_to_be_replaced,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_material,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials_on_actors,
                __buffer,
            )
        };
        std::mem::forget(material_to_be_replaced);
        std::mem::forget(new_material);
    }
    pub fn replace_mesh_components_materials(
        &mut self,
        mesh_components: &TArray<UPtr<crate::bindings::engine::UMeshComponent>>,
        material_to_be_replaced: UPtr<crate::bindings::engine::UMaterialInterface>,
        new_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                mesh_components,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UMeshComponent>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_to_be_replaced,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_material,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_replace_mesh_components_materials,
                __buffer,
            )
        };
        std::mem::forget(material_to_be_replaced);
        std::mem::forget(new_material);
    }
    pub fn remove_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(uv_channel_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_lods(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_lods,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_lods,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn remove_collisions_with_notification(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_collisions_with_notification,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_collisions_with_notification,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_apply_changes);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn remove_collisions(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_collisions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_remove_collisions,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn reimport_all_custom_lods_reimport_all_custom_lo_ds(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_reimport_all_custom_lo_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_reimport_all_custom_lo_ds,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn merge_static_mesh_actors(
        &mut self,
        actors_to_merge: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        merge_options: &FMergeStaticMeshActorsOptions,
        out_merged_actor: &mut UPtr<crate::bindings::engine::AStaticMeshActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<409>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_merge_static_mesh_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_merge,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_options,
                __buffer.add(16).cast::<FMergeStaticMeshActorsOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_merged_actor,
                __buffer
                    .add(400)
                    .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_merge_static_mesh_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(400)
                .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>()
                .swap(out_merged_actor);
        }
        unsafe { __buffer.add(408).cast::<bool>().read() }
    }
    pub fn join_static_mesh_actors(
        &mut self,
        actors_to_join: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        join_options: &FJoinStaticMeshActorsOptions,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_join_static_mesh_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_join,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                join_options,
                __buffer.add(16).cast::<FJoinStaticMeshActorsOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_join_static_mesh_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn is_section_visible_in_ray_tracing_enabled(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_visible_in_ray_tracing_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_visible_in_ray_tracing_enabled,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_section_force_opaque_enabled(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_force_opaque_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_force_opaque_enabled,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_section_collision_enabled(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_collision_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_collision_enabled,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_section_cast_shadow_enabled(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_cast_shadow_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_cast_shadow_enabled,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_section_affect_distance_field_lighting_enabled(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_affect_distance_field_lighting_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_is_section_affect_distance_field_lighting_enabled,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn insert_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_insert_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_insert_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(uv_channel_index);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn import_nanite_hi_res_mesh(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        source_filename: FString,
        b_show_dialog_when_file_missing: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_import_nanite_hi_res_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_dialog_when_file_missing,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_import_nanite_hi_res_mesh,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(source_filename);
        std::mem::forget(b_show_dialog_when_file_missing);
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn import_lod(
        &mut self,
        base_static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        source_filename: FString,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_import_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_import_lod,
                __buffer,
            )
        };
        std::mem::forget(base_static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(source_filename);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn has_vertex_colors(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_has_vertex_colors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_has_vertex_colors,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_instance_vertex_colors(
        &mut self,
        static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_has_instance_vertex_colors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UStaticMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_has_instance_vertex_colors,
                __buffer,
            )
        };
        std::mem::forget(static_mesh_component);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_simple_collision_count(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_simple_collision_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_simple_collision_count,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_num_uv_channels(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_num_uv_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_num_uv_channels,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_number_verts(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_number_verts,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_number_verts,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_number_materials(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_number_materials,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_number_materials,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_nanite_source_filename(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_source_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_source_filename,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_nanite_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> crate::bindings::engine::FMeshNaniteSettings {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_nanite_settings,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe {
            __buffer.add(8).cast::<crate::bindings::engine::FMeshNaniteSettings>().read()
        }
    }
    pub fn get_lod_screen_sizes(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_screen_sizes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_screen_sizes,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<TArray<f32>>().read() }
    }
    pub fn get_lod_reduction_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        out_reduction_options: &mut crate::bindings::engine::FMeshReductionSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_reduction_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_reduction_options,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::FMeshReductionSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_reduction_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::engine::FMeshReductionSettings>()
                .swap(out_reduction_options);
        }
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
    }
    pub fn get_lod_material_slot(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        section_index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_material_slot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_material_slot,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_lod_group(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_group,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_group,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn get_lod_count(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_count,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_lod_build_settings(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        out_build_options: &mut crate::bindings::engine::FMeshBuildSettings,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_build_options,
                __buffer.add(16).cast::<crate::bindings::engine::FMeshBuildSettings>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_lod_build_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::FMeshBuildSettings>()
                .swap(out_build_options);
        }
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
    }
    pub fn get_convex_collision_count(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_convex_collision_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_convex_collision_count,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_collision_complexity(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) -> crate::bindings::physics_core::ECollisionTraceFlag {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_collision_complexity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_get_collision_complexity,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::physics_core::ECollisionTraceFlag>()
                .read()
        }
    }
    pub fn generate_planar_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        tiling: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_planar_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tiling,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_planar_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(uv_channel_index);
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn generate_cylindrical_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        tiling: &crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_cylindrical_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tiling,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_cylindrical_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(uv_channel_index);
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn generate_box_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
        uv_channel_index: i32,
        position: &crate::bindings::core_u_object::FVector,
        orientation: &crate::bindings::core_u_object::FRotator,
        size: &crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_box_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uv_channel_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                orientation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                size,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_generate_box_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        std::mem::forget(uv_channel_index);
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn enable_section_visible_in_ray_tracing(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_visible_in_ray_tracing: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_in_ray_tracing,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_visible_in_ray_tracing,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_visible_in_ray_tracing);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn enable_section_force_opaque(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_force_opaque: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_force_opaque,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_opaque,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_force_opaque,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_force_opaque);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn enable_section_collision(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_collision_enabled: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_collision_enabled,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_collision,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_collision_enabled);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn enable_section_cast_shadow(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_cast_shadow: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_cast_shadow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_cast_shadow,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_cast_shadow,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_cast_shadow);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn enable_section_affect_distance_field_lighting(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        b_affect_distance_field_lighting: bool,
        lod_index: i32,
        section_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_affect_distance_field_lighting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_distance_field_lighting,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_index,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_enable_section_affect_distance_field_lighting,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(b_affect_distance_field_lighting);
        std::mem::forget(lod_index);
        std::mem::forget(section_index);
    }
    pub fn create_proxy_mesh_actor(
        &mut self,
        actors_to_merge: &TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>,
        merge_options: &FCreateProxyMeshActorOptions,
        out_merged_actor: &mut UPtr<crate::bindings::engine::AStaticMeshActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<401>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_create_proxy_mesh_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_merge,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::AStaticMeshActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_options,
                __buffer.add(16).cast::<FCreateProxyMeshActorOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_merged_actor,
                __buffer
                    .add(392)
                    .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_create_proxy_mesh_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(392)
                .cast::<UPtr<crate::bindings::engine::AStaticMeshActor>>()
                .swap(out_merged_actor);
        }
        unsafe { __buffer.add(400).cast::<bool>().read() }
    }
    pub fn bulk_set_convex_decomposition_collisions_with_notification(
        &mut self,
        static_meshes: &TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
        b_apply_changes: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions_with_notification,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                static_meshes,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UStaticMesh>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_count,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions_with_notification,
                __buffer,
            )
        };
        std::mem::forget(hull_count);
        std::mem::forget(max_hull_verts);
        std::mem::forget(hull_precision);
        std::mem::forget(b_apply_changes);
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn bulk_set_convex_decomposition_collisions(
        &mut self,
        static_meshes: &TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
        hull_count: i32,
        max_hull_verts: i32,
        hull_precision: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                static_meshes,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UStaticMesh>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_count,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_hull_verts,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hull_precision,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_bulk_set_convex_decomposition_collisions,
                __buffer,
            )
        };
        std::mem::forget(hull_count);
        std::mem::forget(max_hull_verts);
        std::mem::forget(hull_precision);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn add_uv_channel(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        lod_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_uv_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_uv_channel,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(lod_index);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn add_simple_collisions_with_notification(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        shape_type: EScriptCollisionShapeType,
        b_apply_changes: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions_with_notification,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shape_type,
                __buffer.add(8).cast::<EScriptCollisionShapeType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_changes,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions_with_notification,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(shape_type);
        std::mem::forget(b_apply_changes);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn add_simple_collisions(
        &mut self,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        shape_type: EScriptCollisionShapeType,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shape_type,
                __buffer.add(8).cast::<EScriptCollisionShapeType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::static_mesh_editor::__FUNCTION_PTRS
                    .u_static_mesh_editor_subsystem_add_simple_collisions,
                __buffer,
            )
        };
        std::mem::forget(static_mesh);
        std::mem::forget(shape_type);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
}
#[repr(transparent)]
pub struct EScriptCollisionShapeType(pub u8);
impl EScriptCollisionShapeType {
    pub const BOX: EScriptCollisionShapeType = EScriptCollisionShapeType(0);
    pub const SPHERE: EScriptCollisionShapeType = EScriptCollisionShapeType(1);
    pub const CAPSULE: EScriptCollisionShapeType = EScriptCollisionShapeType(2);
    pub const NDOP10_X: EScriptCollisionShapeType = EScriptCollisionShapeType(3);
    pub const NDOP10_Y: EScriptCollisionShapeType = EScriptCollisionShapeType(4);
    pub const NDOP10_Z: EScriptCollisionShapeType = EScriptCollisionShapeType(5);
    pub const NDOP18: EScriptCollisionShapeType = EScriptCollisionShapeType(6);
    pub const NDOP26: EScriptCollisionShapeType = EScriptCollisionShapeType(7);
}
