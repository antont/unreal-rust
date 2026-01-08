#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_groom_asset_set_rigged_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_interpolation_type: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_rendering: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_physics: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_meshes: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_materials: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_interpolation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_hair_groups_cards: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_enable_simulation_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_enable_global_interpolation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_set_deformed_group_sections: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_rigged_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_interpolation_type: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_rendering: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_physics: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_meshes: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_materials: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_interpolation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_hair_groups_cards: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_enable_simulation_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_enable_global_interpolation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_get_deformed_group_sections: *mut crate::ffi::UFunctionOpague,
    pub u_groom_asset_create_groom_dataflow: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_target_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_target_mesh_used_min_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_target_mesh_requested_min_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_target_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_target_binding_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_source_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_source_mesh_used_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_source_mesh_requested_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_source_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_num_interpolation_points: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_matching_section: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_group_infos: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_groom_binding_type: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_set_groom: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_target_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_target_mesh_used_min_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_target_mesh_requested_min_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_target_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_target_binding_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_source_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_source_mesh_used_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_source_mesh_requested_lod: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_source_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_num_interpolation_points: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_matching_section: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_group_infos: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_groom_binding_type: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_get_groom: *mut crate::ffi::UFunctionOpague,
    pub u_groom_binding_asset_build: *mut crate::ffi::UFunctionOpague,
    pub u_groom_blueprint_library_is_hair_strands_supported_in_world: *mut crate::ffi::UFunctionOpague,
    pub u_groom_blueprint_library_create_new_groom_binding_asset_with_path: *mut crate::ffi::UFunctionOpague,
    pub u_groom_blueprint_library_create_new_groom_binding_asset: *mut crate::ffi::UFunctionOpague,
    pub u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset_with_path: *mut crate::ffi::UFunctionOpague,
    pub u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_physics_asset: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_mesh_deformer: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_hair_length_scale_enable: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_hair_length_scale: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_groom_cache: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_groom_asset: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_enable_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_set_binding_asset: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_reset_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_reset_collision_components: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_get_niagara_component: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_get_is_hair_length_scale_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_groom_component_add_collision_component: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_groom_asset_set_rigged_skeletal_mesh: std::ptr::null_mut(),
            u_groom_asset_set_hair_interpolation_type: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_rendering: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_physics: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_meshes: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_materials: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_lod: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_interpolation: std::ptr::null_mut(),
            u_groom_asset_set_hair_groups_cards: std::ptr::null_mut(),
            u_groom_asset_set_enable_simulation_cache: std::ptr::null_mut(),
            u_groom_asset_set_enable_global_interpolation: std::ptr::null_mut(),
            u_groom_asset_set_deformed_group_sections: std::ptr::null_mut(),
            u_groom_asset_get_rigged_skeletal_mesh: std::ptr::null_mut(),
            u_groom_asset_get_hair_interpolation_type: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_rendering: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_physics: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_meshes: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_materials: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_lod: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_interpolation: std::ptr::null_mut(),
            u_groom_asset_get_hair_groups_cards: std::ptr::null_mut(),
            u_groom_asset_get_enable_simulation_cache: std::ptr::null_mut(),
            u_groom_asset_get_enable_global_interpolation: std::ptr::null_mut(),
            u_groom_asset_get_deformed_group_sections: std::ptr::null_mut(),
            u_groom_asset_create_groom_dataflow: std::ptr::null_mut(),
            u_groom_binding_asset_set_target_skeletal_mesh: std::ptr::null_mut(),
            u_groom_binding_asset_set_target_mesh_used_min_lod: std::ptr::null_mut(),
            u_groom_binding_asset_set_target_mesh_requested_min_lod: std::ptr::null_mut(),
            u_groom_binding_asset_set_target_geometry_cache: std::ptr::null_mut(),
            u_groom_binding_asset_set_target_binding_attribute: std::ptr::null_mut(),
            u_groom_binding_asset_set_source_skeletal_mesh: std::ptr::null_mut(),
            u_groom_binding_asset_set_source_mesh_used_lod: std::ptr::null_mut(),
            u_groom_binding_asset_set_source_mesh_requested_lod: std::ptr::null_mut(),
            u_groom_binding_asset_set_source_geometry_cache: std::ptr::null_mut(),
            u_groom_binding_asset_set_num_interpolation_points: std::ptr::null_mut(),
            u_groom_binding_asset_set_matching_section: std::ptr::null_mut(),
            u_groom_binding_asset_set_group_infos: std::ptr::null_mut(),
            u_groom_binding_asset_set_groom_binding_type: std::ptr::null_mut(),
            u_groom_binding_asset_set_groom: std::ptr::null_mut(),
            u_groom_binding_asset_get_target_skeletal_mesh: std::ptr::null_mut(),
            u_groom_binding_asset_get_target_mesh_used_min_lod: std::ptr::null_mut(),
            u_groom_binding_asset_get_target_mesh_requested_min_lod: std::ptr::null_mut(),
            u_groom_binding_asset_get_target_geometry_cache: std::ptr::null_mut(),
            u_groom_binding_asset_get_target_binding_attribute: std::ptr::null_mut(),
            u_groom_binding_asset_get_source_skeletal_mesh: std::ptr::null_mut(),
            u_groom_binding_asset_get_source_mesh_used_lod: std::ptr::null_mut(),
            u_groom_binding_asset_get_source_mesh_requested_lod: std::ptr::null_mut(),
            u_groom_binding_asset_get_source_geometry_cache: std::ptr::null_mut(),
            u_groom_binding_asset_get_num_interpolation_points: std::ptr::null_mut(),
            u_groom_binding_asset_get_matching_section: std::ptr::null_mut(),
            u_groom_binding_asset_get_group_infos: std::ptr::null_mut(),
            u_groom_binding_asset_get_groom_binding_type: std::ptr::null_mut(),
            u_groom_binding_asset_get_groom: std::ptr::null_mut(),
            u_groom_binding_asset_build: std::ptr::null_mut(),
            u_groom_blueprint_library_is_hair_strands_supported_in_world: std::ptr::null_mut(),
            u_groom_blueprint_library_create_new_groom_binding_asset_with_path: std::ptr::null_mut(),
            u_groom_blueprint_library_create_new_groom_binding_asset: std::ptr::null_mut(),
            u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset_with_path: std::ptr::null_mut(),
            u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset: std::ptr::null_mut(),
            u_groom_component_set_physics_asset: std::ptr::null_mut(),
            u_groom_component_set_mesh_deformer: std::ptr::null_mut(),
            u_groom_component_set_hair_length_scale_enable: std::ptr::null_mut(),
            u_groom_component_set_hair_length_scale: std::ptr::null_mut(),
            u_groom_component_set_groom_cache: std::ptr::null_mut(),
            u_groom_component_set_groom_asset: std::ptr::null_mut(),
            u_groom_component_set_enable_simulation: std::ptr::null_mut(),
            u_groom_component_set_binding_asset: std::ptr::null_mut(),
            u_groom_component_reset_simulation: std::ptr::null_mut(),
            u_groom_component_reset_collision_components: std::ptr::null_mut(),
            u_groom_component_get_niagara_component: std::ptr::null_mut(),
            u_groom_component_get_is_hair_length_scale_enabled: std::ptr::null_mut(),
            u_groom_component_add_collision_component: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomAsset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRiggedSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_rigged_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairInterpolationType"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_interpolation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsRendering"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_rendering,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsPhysics"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_physics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsMeshes"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_meshes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsMaterials"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_materials,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsInterpolation"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_interpolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairGroupsCards"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_hair_groups_cards,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableSimulationCache"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_enable_simulation_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableGlobalInterpolation"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_enable_global_interpolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDeformedGroupSections"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_set_deformed_group_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRiggedSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_rigged_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairInterpolationType"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_interpolation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsRendering"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_rendering,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsPhysics"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_physics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsMeshes"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_meshes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsMaterials"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_materials,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsInterpolation"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_interpolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHairGroupsCards"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_hair_groups_cards,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnableSimulationCache"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_enable_simulation_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEnableGlobalInterpolation"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_enable_global_interpolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDeformedGroupSections"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_get_deformed_group_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateGroomDataflow"),
            &raw mut __FUNCTION_PTRS.u_groom_asset_create_groom_dataflow,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomBindingAsset::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_target_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetMeshUsedMinLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_target_mesh_used_min_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetMeshRequestedMinLOD"),
            &raw mut __FUNCTION_PTRS
                .u_groom_binding_asset_set_target_mesh_requested_min_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetGeometryCache"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_target_geometry_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetBindingAttribute"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_target_binding_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_source_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceMeshUsedLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_source_mesh_used_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceMeshRequestedLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_source_mesh_requested_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceGeometryCache"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_source_geometry_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumInterpolationPoints"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_num_interpolation_points,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMatchingSection"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_matching_section,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroupInfos"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_group_infos,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroomBindingType"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_groom_binding_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroom"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_set_groom,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_target_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetMeshUsedMinLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_target_mesh_used_min_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetMeshRequestedMinLOD"),
            &raw mut __FUNCTION_PTRS
                .u_groom_binding_asset_get_target_mesh_requested_min_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetGeometryCache"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_target_geometry_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetBindingAttribute"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_target_binding_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceSkeletalMesh"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_source_skeletal_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceMeshUsedLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_source_mesh_used_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceMeshRequestedLOD"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_source_mesh_requested_lod,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceGeometryCache"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_source_geometry_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumInterpolationPoints"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_num_interpolation_points,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMatchingSection"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_matching_section,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGroupInfos"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_group_infos,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGroomBindingType"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_groom_binding_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGroom"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_get_groom,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Build"),
            &raw mut __FUNCTION_PTRS.u_groom_binding_asset_build,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsHairStrandsSupportedInWorld"),
            &raw mut __FUNCTION_PTRS
                .u_groom_blueprint_library_is_hair_strands_supported_in_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewGroomBindingAssetWithPath"),
            &raw mut __FUNCTION_PTRS
                .u_groom_blueprint_library_create_new_groom_binding_asset_with_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewGroomBindingAsset"),
            &raw mut __FUNCTION_PTRS
                .u_groom_blueprint_library_create_new_groom_binding_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewGeometryCacheGroomBindingAssetWithPath"),
            &raw mut __FUNCTION_PTRS
                .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset_with_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewGeometryCacheGroomBindingAsset"),
            &raw mut __FUNCTION_PTRS
                .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPhysicsAsset"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_physics_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeshDeformer"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_mesh_deformer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairLengthScaleEnable"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_hair_length_scale_enable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHairLengthScale"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_hair_length_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroomCache"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_groom_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroomAsset"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_groom_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEnableSimulation"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_enable_simulation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBindingAsset"),
            &raw mut __FUNCTION_PTRS.u_groom_component_set_binding_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetSimulation"),
            &raw mut __FUNCTION_PTRS.u_groom_component_reset_simulation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetCollisionComponents"),
            &raw mut __FUNCTION_PTRS.u_groom_component_reset_collision_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_component_get_niagara_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsHairLengthScaleEnabled"),
            &raw mut __FUNCTION_PTRS.u_groom_component_get_is_hair_length_scale_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCollisionComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_component_add_collision_component,
        );
    }
}
#[repr(C, align(8))]
pub struct FGroomCacheImportSettings {
    pub b_import_groom_cache: bool,
    pub import_type: EGroomCacheImportType,
    pub frame_start: i32,
    pub frame_end: i32,
    pub b_skip_empty_frames: bool,
    pub b_import_groom_asset: bool,
    pub groom_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_override_conversion_settings: bool,
    pub conversion_settings: FGroomConversionSettings,
}
impl FGroomCacheImportSettings {}
#[repr(C, align(8))]
pub struct FGroomConversionSettings {
    pub rotation: crate::bindings::core_u_object::FVector,
    pub scale: crate::bindings::core_u_object::FVector,
}
impl FGroomConversionSettings {}
#[repr(C, align(4))]
pub struct FHairGroupDesc {
    #[doc(hidden)]
    __padding_4: [u8; 4],
    pub hair_width: f32,
    pub hair_width_override: bool,
    pub hair_root_scale: f32,
    pub hair_root_scale_override: bool,
    pub hair_tip_scale: f32,
    pub hair_tip_scale_override: bool,
    pub hair_shadow_density: f32,
    pub hair_shadow_density_override: bool,
    pub hair_raytracing_radius_scale: f32,
    pub hair_raytracing_radius_scale_override: bool,
    pub b_use_hair_raytracing_geometry: bool,
    pub b_use_hair_raytracing_geometry_override: bool,
    pub lod_bias: f32,
    pub b_use_stable_rasterization: bool,
    pub b_use_stable_rasterization_override: bool,
    pub b_scatter_scene_lighting: bool,
    pub b_scatter_scene_lighting_override: bool,
    #[doc(hidden)]
    __padding_56: [u8; 4],
    pub hair_length_scale: f32,
    pub hair_length_scale_override: bool,
}
impl FHairGroupDesc {}
#[repr(C, align(4))]
pub struct FHairGroupLODInfo {
    __padding_end: [u8; 8],
}
impl FHairGroupLODInfo {}
#[repr(C, align(8))]
pub struct FHairGroupInfo {
    __padding_end: [u8; 64],
}
impl FHairGroupInfo {}
#[repr(C, align(8))]
pub struct FHairGroupsMaterial {
    __padding_end: [u8; 24],
}
impl FHairGroupsMaterial {}
#[repr(C, align(8))]
pub struct FHairGroupInfoWithVisibility {
    __padding_end: [u8; 72],
}
impl FHairGroupInfoWithVisibility {}
#[repr(C, align(4))]
pub struct FHairGroupCardsInfo {
    __padding_end: [u8; 8],
}
impl FHairGroupCardsInfo {}
#[repr(C, align(8))]
pub struct FHairGroupCardsTextures {
    __padding_end: [u8; 80],
}
impl FHairGroupCardsTextures {}
#[repr(C, align(8))]
pub struct FHairGroupsCardsSourceDescription {
    __padding_end: [u8; 168],
}
impl FHairGroupsCardsSourceDescription {}
#[repr(C, align(4))]
pub struct FHairLODSettings {
    __padding_end: [u8; 28],
}
impl FHairLODSettings {}
#[repr(C, align(4))]
pub struct FHairDecimationSettings {
    __padding_end: [u8; 8],
}
impl FHairDecimationSettings {}
#[repr(C, align(4))]
pub struct FHairInterpolationSettings {
    pub guide_type: EGroomGuideType,
    pub hair_to_guide_density: f32,
    pub rigged_guide_num_curves: i32,
    pub rigged_guide_num_points: i32,
    pub interpolation_quality: EHairInterpolationQuality,
    pub interpolation_distance: EHairInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
impl FHairInterpolationSettings {}
#[repr(C, align(4))]
pub struct FHairDeformationSettings {
    __padding_end: [u8; 12],
}
impl FHairDeformationSettings {}
#[repr(C, align(4))]
pub struct FHairGroupsInterpolation {
    __padding_end: [u8; 52],
}
impl FHairGroupsInterpolation {}
#[repr(C, align(8))]
pub struct FHairGroupsLOD {
    __padding_end: [u8; 24],
}
impl FHairGroupsLOD {}
#[repr(C, align(8))]
pub struct FHairGroupsMeshesSourceDescription {
    __padding_end: [u8; 136],
}
impl FHairGroupsMeshesSourceDescription {}
#[repr(C, align(8))]
pub struct FHairSolverSettings {
    __padding_end: [u8; 72],
}
impl FHairSolverSettings {}
#[repr(C, align(8))]
pub struct FHairExternalForces {
    __padding_end: [u8; 56],
}
impl FHairExternalForces {}
#[repr(C, align(8))]
pub struct FHairBendConstraint {
    __padding_end: [u8; 152],
}
impl FHairBendConstraint {}
#[repr(C, align(8))]
pub struct FHairStretchConstraint {
    __padding_end: [u8; 152],
}
impl FHairStretchConstraint {}
#[repr(C, align(8))]
pub struct FHairCollisionConstraint {
    __padding_end: [u8; 168],
}
impl FHairCollisionConstraint {}
#[repr(C, align(8))]
pub struct FHairMaterialConstraints {
    __padding_end: [u8; 472],
}
impl FHairMaterialConstraints {}
#[repr(C, align(8))]
pub struct FHairStrandsParameters {
    __padding_end: [u8; 152],
}
impl FHairStrandsParameters {}
#[repr(C, align(8))]
pub struct FHairGroupsPhysics {
    __padding_end: [u8; 768],
}
impl FHairGroupsPhysics {}
#[repr(C, align(1))]
pub struct FHairSimulationSolver {
    pub b_enable_simulation: bool,
}
impl FHairSimulationSolver {}
#[repr(C, align(8))]
pub struct FHairSimulationForces {
    pub gravity_vector: crate::bindings::core_u_object::FVector,
    pub air_drag: f32,
    pub air_velocity: crate::bindings::core_u_object::FVector,
}
impl FHairSimulationForces {}
#[repr(C, align(4))]
pub struct FHairSimulationConstraints {
    pub bend_damping: f32,
    pub bend_stiffness: f32,
    pub stretch_damping: f32,
    pub stretch_stiffness: f32,
    pub static_friction: f32,
    pub kinetic_friction: f32,
    pub strands_viscosity: f32,
    pub collision_radius: f32,
}
impl FHairSimulationConstraints {}
#[repr(C, align(8))]
pub struct FHairSimulationSetup {
    pub b_reset_simulation: bool,
    pub b_debug_simulation: bool,
    pub b_local_simulation: bool,
    pub linear_velocity_scale: f32,
    pub angular_velocity_scale: f32,
    pub local_bone: FString,
    pub teleport_distance: f32,
}
impl FHairSimulationSetup {}
#[repr(C, align(8))]
pub struct FHairSimulationSettings {
    pub b_override_settings: bool,
    pub simulation_setup: FHairSimulationSetup,
    pub solver_settings: FHairSimulationSolver,
    pub external_forces: FHairSimulationForces,
    pub material_constraints: FHairSimulationConstraints,
}
impl FHairSimulationSettings {}
#[repr(C, align(4))]
pub struct FHairGeometrySettings {
    pub hair_width: f32,
    pub hair_width_override: bool,
    pub hair_root_scale: f32,
    pub hair_tip_scale: f32,
}
impl FHairGeometrySettings {}
#[repr(C, align(4))]
pub struct FHairShadowSettings {
    pub hair_shadow_density: f32,
    pub hair_raytracing_radius_scale: f32,
    pub b_use_hair_raytracing_geometry: bool,
    pub b_voxelize: bool,
}
impl FHairShadowSettings {}
#[repr(C, align(1))]
pub struct FHairAdvancedRenderingSettings {
    pub b_use_stable_rasterization: bool,
    pub b_scatter_scene_lighting: bool,
}
impl FHairAdvancedRenderingSettings {}
#[repr(C, align(8))]
pub struct FHairGroupsRendering {
    __padding_end: [u8; 72],
}
impl FHairGroupsRendering {}
#[repr(C, align(4))]
pub struct FGoomBindingGroupInfo {
    __padding_end: [u8; 16],
}
impl FGoomBindingGroupInfo {}
#[repr(C, align(8))]
pub struct FFollicleMaskOptions {
    pub groom: UPtr<UGroomAsset>,
    pub channel: EFollicleMaskChannel,
}
impl FFollicleMaskOptions {}
#[repr(C, align(4))]
pub struct FGroomHairGroupPreview {
    pub group_index: i32,
    pub group_name: FName,
    pub group_id: i32,
    pub curve_count: i32,
    pub guide_count: i32,
    #[doc(hidden)]
    __padding_40: [u8; 12],
    pub interpolation_settings: FHairGroupsInterpolation,
}
impl FGroomHairGroupPreview {}
#[repr(C, align(4))]
pub struct FGroomBuildSettings {
    pub b_override_guides: bool,
    pub hair_to_guide_density: f32,
    pub interpolation_quality: EGroomInterpolationQuality,
    pub interpolation_distance: EGroomInterpolationWeight,
    pub b_randomize_guide: bool,
    pub b_use_unique_guide: bool,
}
impl FGroomBuildSettings {}
#[repr(C, align(8))]
pub struct UGroomCacheImportOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub import_settings: FGroomCacheImportSettings,
}
impl UGroomCacheImportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCacheImportOptions")
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
pub struct UGroomCacheImportData {
    __padding_end: [u8; 208],
}
impl UGroomCacheImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCacheImportData")
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
pub struct AGroomActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub groom_component: UPtr<UGroomComponent>,
    __padding_end: [u8; 8],
}
impl AGroomActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGroomActor")
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
#[repr(C, align(16))]
pub struct UGroomAsset {
    #[doc(hidden)]
    __padding_352: [u8; 352],
    pub hair_groups_rendering: TArray<FHairGroupsRendering>,
    pub hair_groups_physics: TArray<FHairGroupsPhysics>,
    pub hair_groups_interpolation: TArray<FHairGroupsInterpolation>,
    pub hair_groups_lod: TArray<FHairGroupsLOD>,
    pub hair_groups_cards: TArray<FHairGroupsCardsSourceDescription>,
    pub hair_groups_meshes: TArray<FHairGroupsMeshesSourceDescription>,
    pub hair_groups_materials: TArray<FHairGroupsMaterial>,
    pub enable_global_interpolation: bool,
    pub enable_simulation_cache: bool,
    pub hair_interpolation_type: EGroomInterpolationType,
    pub rigged_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 672],
}
impl UGroomAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomAsset")
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
    pub fn set_rigged_skeletal_mesh(
        &mut self,
        in_: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_rigged_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_rigged_skeletal_mesh,
                __buffer,
            )
        };
    }
    pub fn set_hair_interpolation_type(&mut self, in_: EGroomInterpolationType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_interpolation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_,
                __buffer.add(0).cast::<EGroomInterpolationType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_interpolation_type,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_rendering(&mut self, in_: &TArray<FHairGroupsRendering>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_rendering,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsRendering>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_rendering,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_physics(&mut self, in_: &TArray<FHairGroupsPhysics>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_physics,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsPhysics>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_physics,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_meshes(
        &mut self,
        in_: &TArray<FHairGroupsMeshesSourceDescription>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_meshes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsMeshesSourceDescription>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_meshes,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_materials(&mut self, in_: &TArray<FHairGroupsMaterial>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_materials,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsMaterial>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_materials,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_lod(&mut self, in_: &TArray<FHairGroupsLOD>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsLOD>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_lod,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_interpolation(
        &mut self,
        in_: &TArray<FHairGroupsInterpolation>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_interpolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsInterpolation>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_interpolation,
                __buffer,
            )
        };
    }
    pub fn set_hair_groups_cards(
        &mut self,
        in_: &TArray<FHairGroupsCardsSourceDescription>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_cards,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_,
                __buffer.add(0).cast::<TArray<FHairGroupsCardsSourceDescription>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_hair_groups_cards,
                __buffer,
            )
        };
    }
    pub fn set_enable_simulation_cache(&mut self, in_: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_enable_simulation_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_enable_simulation_cache,
                __buffer,
            )
        };
    }
    pub fn set_enable_global_interpolation(&mut self, in_: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_enable_global_interpolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_enable_global_interpolation,
                __buffer,
            )
        };
    }
    pub fn set_deformed_group_sections(&mut self, in_: &TArray<i32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_deformed_group_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_, __buffer.add(0).cast::<TArray<i32>>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_set_deformed_group_sections,
                __buffer,
            )
        };
    }
    pub fn get_rigged_skeletal_mesh(
        &self,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_rigged_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_rigged_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_hair_interpolation_type(&self) -> EGroomInterpolationType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_interpolation_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_interpolation_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EGroomInterpolationType>().read() }
    }
    pub fn get_hair_groups_rendering(&mut self) -> TArray<FHairGroupsRendering> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_rendering,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_rendering,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FHairGroupsRendering>>().read() }
    }
    pub fn get_hair_groups_physics(&mut self) -> TArray<FHairGroupsPhysics> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_physics,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_physics,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FHairGroupsPhysics>>().read() }
    }
    pub fn get_hair_groups_meshes(
        &mut self,
    ) -> TArray<FHairGroupsMeshesSourceDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_meshes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_meshes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FHairGroupsMeshesSourceDescription>>().read()
        }
    }
    pub fn get_hair_groups_materials(&mut self) -> TArray<FHairGroupsMaterial> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_materials,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_materials,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FHairGroupsMaterial>>().read() }
    }
    pub fn get_hair_groups_lod(&mut self) -> TArray<FHairGroupsLOD> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_lod,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FHairGroupsLOD>>().read() }
    }
    pub fn get_hair_groups_interpolation(&mut self) -> TArray<FHairGroupsInterpolation> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_interpolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_interpolation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FHairGroupsInterpolation>>().read() }
    }
    pub fn get_hair_groups_cards(
        &mut self,
    ) -> TArray<FHairGroupsCardsSourceDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_cards,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_hair_groups_cards,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FHairGroupsCardsSourceDescription>>().read()
        }
    }
    pub fn get_enable_simulation_cache(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_enable_simulation_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_enable_simulation_cache,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_enable_global_interpolation(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_enable_global_interpolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_enable_global_interpolation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_deformed_group_sections(&mut self) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_deformed_group_sections,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_get_deformed_group_sections,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<i32>>().read() }
    }
    pub fn create_groom_dataflow(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_create_groom_dataflow,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_asset_create_groom_dataflow,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UHairCardGenerationSettings {
    __padding_end: [u8; 48],
}
impl UHairCardGenerationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHairCardGenerationSettings")
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
pub struct UDataflowGroomContent {
    __padding_end: [u8; 240],
}
impl UDataflowGroomContent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowGroomContent")
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
pub struct UGroomAssetImportData {
    __padding_end: [u8; 112],
}
impl UGroomAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomAssetImportData")
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
#[repr(C, align(16))]
pub struct UGroomBindingAsset {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub groom_binding_type: EGroomBindingMeshType,
    pub groom: UPtr<UGroomAsset>,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_mesh_requested_lod: i32,
    pub source_mesh_used_lod: i32,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_mesh_requested_min_lod: i32,
    pub target_mesh_used_min_lod: i32,
    pub source_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub target_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub num_interpolation_points: i32,
    pub matching_section: i32,
    pub target_binding_attribute: FName,
    pub group_infos: TArray<FGoomBindingGroupInfo>,
    __padding_end: [u8; 176],
}
impl UGroomBindingAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomBindingAsset")
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
    pub fn set_target_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_skeletal_mesh,
                __buffer,
            )
        };
    }
    pub fn set_target_mesh_used_min_lod(&mut self, in_target_mesh_used_min_lod: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_mesh_used_min_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_mesh_used_min_lod,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_mesh_used_min_lod,
                __buffer,
            )
        };
    }
    pub fn set_target_mesh_requested_min_lod(
        &mut self,
        in_target_mesh_requested_min_lod: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_mesh_requested_min_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_target_mesh_requested_min_lod,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_mesh_requested_min_lod,
                __buffer,
            )
        };
    }
    pub fn set_target_geometry_cache(
        &mut self,
        in_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_geometry_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_geometry_cache,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_geometry_cache,
                __buffer,
            )
        };
    }
    pub fn set_target_binding_attribute(&mut self, in_attribute_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_binding_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attribute_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_target_binding_attribute,
                __buffer,
            )
        };
    }
    pub fn set_source_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_skeletal_mesh,
                __buffer,
            )
        };
    }
    pub fn set_source_mesh_used_lod(&mut self, in_source_mesh_used_lod: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_mesh_used_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_mesh_used_lod,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_mesh_used_lod,
                __buffer,
            )
        };
    }
    pub fn set_source_mesh_requested_lod(&mut self, in_source_mesh_requested_lod: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_mesh_requested_lod,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_mesh_requested_lod,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_mesh_requested_lod,
                __buffer,
            )
        };
    }
    pub fn set_source_geometry_cache(
        &mut self,
        in_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_geometry_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_geometry_cache,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_source_geometry_cache,
                __buffer,
            )
        };
    }
    pub fn set_num_interpolation_points(&mut self, in_num_interpolation_points: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_num_interpolation_points,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_interpolation_points,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_num_interpolation_points,
                __buffer,
            )
        };
    }
    pub fn set_matching_section(&mut self, in_matching_section: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_matching_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_matching_section,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_matching_section,
                __buffer,
            )
        };
    }
    pub fn set_group_infos(&mut self, in_group_infos: &TArray<FGoomBindingGroupInfo>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_group_infos,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_group_infos,
                __buffer.add(0).cast::<TArray<FGoomBindingGroupInfo>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_group_infos,
                __buffer,
            )
        };
    }
    pub fn set_groom_binding_type(
        &mut self,
        in_groom_binding_type: EGroomBindingMeshType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_groom_binding_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_groom_binding_type,
                __buffer.add(0).cast::<EGroomBindingMeshType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_groom_binding_type,
                __buffer,
            )
        };
    }
    pub fn set_groom(&mut self, in_groom: UPtr<UGroomAsset>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_groom,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_groom,
                __buffer.add(0).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_set_groom,
                __buffer,
            )
        };
    }
    pub fn get_target_skeletal_mesh(
        &self,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_target_mesh_used_min_lod(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_mesh_used_min_lod,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_mesh_used_min_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_target_mesh_requested_min_lod(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_mesh_requested_min_lod,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_mesh_requested_min_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_target_geometry_cache(
        &self,
    ) -> UPtr<crate::bindings::geometry_cache::UGeometryCache> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_geometry_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_geometry_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>()
                .read()
        }
    }
    pub fn get_target_binding_attribute(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_binding_attribute,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_target_binding_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_source_skeletal_mesh(
        &self,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_skeletal_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_source_mesh_used_lod(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_mesh_used_lod,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_mesh_used_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_source_mesh_requested_lod(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_mesh_requested_lod,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_mesh_requested_lod,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_source_geometry_cache(
        &self,
    ) -> UPtr<crate::bindings::geometry_cache::UGeometryCache> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_geometry_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_source_geometry_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>()
                .read()
        }
    }
    pub fn get_num_interpolation_points(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_num_interpolation_points,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_num_interpolation_points,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_matching_section(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_matching_section,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_matching_section,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_group_infos(&self) -> TArray<FGoomBindingGroupInfo> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_group_infos,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_group_infos,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FGoomBindingGroupInfo>>().read() }
    }
    pub fn get_groom_binding_type(&self) -> EGroomBindingMeshType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_groom_binding_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_groom_binding_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EGroomBindingMeshType>().read() }
    }
    pub fn get_groom(&self) -> UPtr<UGroomAsset> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_groom,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_get_groom,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGroomAsset>>().read() }
    }
    pub fn build(&mut self, completion_delegate: FBuild_CompletionDelegate) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_build,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &completion_delegate,
                __buffer.add(0).cast::<FBuild_CompletionDelegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_binding_asset_build,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGroomBindingAssetList {
    __padding_end: [u8; 64],
}
impl UGroomBindingAssetList {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomBindingAssetList")
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
pub struct UGroomBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UGroomBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomBlueprintLibrary")
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
    pub fn is_hair_strands_supported_in_world(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_is_hair_strands_supported_in_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::hair_strands_core::UGroomBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_is_hair_strands_supported_in_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn create_new_groom_binding_asset_with_path(
        in_desired_package_path: FString,
        in_groom_asset: UPtr<UGroomAsset>,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        in_num_interpolation_points: i32,
        in_source_skeletal_mesh_for_transfer: UPtr<
            crate::bindings::engine::USkeletalMesh,
        >,
        in_matching_section: i32,
    ) -> UPtr<UGroomBindingAsset> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_groom_binding_asset_with_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_desired_package_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_groom_asset,
                __buffer.add(16).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_interpolation_points,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_skeletal_mesh_for_transfer,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_matching_section,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::hair_strands_core::UGroomBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_groom_binding_asset_with_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UGroomBindingAsset>>().read() }
    }
    pub fn create_new_groom_binding_asset(
        in_groom_asset: UPtr<UGroomAsset>,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        in_num_interpolation_points: i32,
        in_source_skeletal_mesh_for_transfer: UPtr<
            crate::bindings::engine::USkeletalMesh,
        >,
        in_matching_section: i32,
    ) -> UPtr<UGroomBindingAsset> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_groom_binding_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_groom_asset,
                __buffer.add(0).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_interpolation_points,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_skeletal_mesh_for_transfer,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_matching_section,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::hair_strands_core::UGroomBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_groom_binding_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UGroomBindingAsset>>().read() }
    }
    pub fn create_new_geometry_cache_groom_binding_asset_with_path(
        desired_package_path: FString,
        groom_asset: UPtr<UGroomAsset>,
        geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
        num_interpolation_points: i32,
        source_geometry_cache_for_transfer: UPtr<
            crate::bindings::geometry_cache::UGeometryCache,
        >,
        matching_section: i32,
    ) -> UPtr<UGroomBindingAsset> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset_with_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &desired_package_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &groom_asset,
                __buffer.add(16).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_cache,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_interpolation_points,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_geometry_cache_for_transfer,
                __buffer
                    .add(40)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &matching_section,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::hair_strands_core::UGroomBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset_with_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UGroomBindingAsset>>().read() }
    }
    pub fn create_new_geometry_cache_groom_binding_asset(
        groom_asset: UPtr<UGroomAsset>,
        geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
        num_interpolation_points: i32,
        source_geometry_cache_for_transfer: UPtr<
            crate::bindings::geometry_cache::UGeometryCache,
        >,
        matching_section: i32,
    ) -> UPtr<UGroomBindingAsset> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &groom_asset,
                __buffer.add(0).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &geometry_cache,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_interpolation_points,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_geometry_cache_for_transfer,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::geometry_cache::UGeometryCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &matching_section,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::hair_strands_core::UGroomBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_blueprint_library_create_new_geometry_cache_groom_binding_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UGroomBindingAsset>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGroomCache {
    __padding_end: [u8; 160],
}
impl UGroomCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCache")
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
#[repr(C, align(16))]
pub struct UGroomComponent {
    #[doc(hidden)]
    __padding_1592: [u8; 1592],
    pub groom_asset: UPtr<UGroomAsset>,
    pub groom_cache: UPtr<UGroomCache>,
    #[doc(hidden)]
    __padding_1632: [u8; 24],
    pub binding_asset: UPtr<UGroomBindingAsset>,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    #[doc(hidden)]
    __padding_1664: [u8; 16],
    pub simulation_settings: FHairSimulationSettings,
    pub mesh_deformer: UPtr<crate::bindings::engine::UMeshDeformer>,
    pub mesh_deformer_instance: UPtr<crate::bindings::engine::UMeshDeformerInstance>,
    pub mesh_deformer_instance_settings: UPtr<
        crate::bindings::engine::UMeshDeformerInstanceSettings,
    >,
    #[doc(hidden)]
    __padding_1880: [u8; 48],
    pub attachment_name: FString,
    #[doc(hidden)]
    __padding_2032: [u8; 136],
    pub groom_groups_desc: TArray<FHairGroupDesc>,
    pub b_use_cards: bool,
    __padding_end: [u8; 143],
}
impl UGroomComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomComponent")
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
    pub fn set_physics_asset(
        &mut self,
        in_physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_physics_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_physics_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UPhysicsAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_physics_asset,
                __buffer,
            )
        };
    }
    pub fn set_mesh_deformer(
        &mut self,
        in_mesh_deformer: UPtr<crate::bindings::engine::UMeshDeformer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_mesh_deformer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mesh_deformer,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshDeformer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_mesh_deformer,
                __buffer,
            )
        };
    }
    pub fn set_hair_length_scale_enable(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_hair_length_scale_enable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_hair_length_scale_enable,
                __buffer,
            )
        };
    }
    pub fn set_hair_length_scale(&mut self, scale: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_hair_length_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_hair_length_scale,
                __buffer,
            )
        };
    }
    pub fn set_groom_cache(&mut self, in_groom_cache: UPtr<UGroomCache>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_groom_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_groom_cache,
                __buffer.add(0).cast::<UPtr<UGroomCache>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_groom_cache,
                __buffer,
            )
        };
    }
    pub fn set_groom_asset(&mut self, asset: UPtr<UGroomAsset>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_groom_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
                __buffer.add(0).cast::<UPtr<UGroomAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_groom_asset,
                __buffer,
            )
        };
    }
    pub fn set_enable_simulation(&mut self, b_in_enable_simulation: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_enable_simulation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enable_simulation,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_enable_simulation,
                __buffer,
            )
        };
    }
    pub fn set_binding_asset(&mut self, in_binding: UPtr<UGroomBindingAsset>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_binding_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_binding,
                __buffer.add(0).cast::<UPtr<UGroomBindingAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_set_binding_asset,
                __buffer,
            )
        };
    }
    pub fn reset_simulation(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_reset_simulation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_reset_simulation,
                __buffer,
            )
        };
    }
    pub fn reset_collision_components(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_reset_collision_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_reset_collision_components,
                __buffer,
            )
        };
    }
    pub fn get_niagara_component(
        &mut self,
        group_index: i32,
    ) -> UPtr<crate::bindings::niagara::UNiagaraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_get_niagara_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &group_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_get_niagara_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::niagara::UNiagaraComponent>>()
                .read()
        }
    }
    pub fn get_is_hair_length_scale_enabled(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_get_is_hair_length_scale_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_get_is_hair_length_scale_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_collision_component(
        &mut self,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_add_collision_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_core::__FUNCTION_PTRS
                    .u_groom_component_add_collision_component,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGroomCreateBindingOptions {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub groom_binding_type: EGroomBindingMeshType,
    pub source_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub source_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub target_geometry_cache: UPtr<crate::bindings::geometry_cache::UGeometryCache>,
    pub num_interpolation_points: i32,
    pub matching_section: i32,
    pub target_binding_attribute: FName,
}
impl UGroomCreateBindingOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCreateBindingOptions")
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
pub struct UGroomCreateFollicleMaskOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub resolution: i32,
    pub root_radius: i32,
    pub grooms: TArray<FFollicleMaskOptions>,
}
impl UGroomCreateFollicleMaskOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCreateFollicleMaskOptions")
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
pub struct UGroomCreateStrandsTexturesOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub layout: EHairTextureLayout,
    pub resolution: i32,
    pub trace_type: EStrandsTexturesTraceType,
    pub trace_distance: f32,
    pub mesh_type: EStrandsTexturesMeshType,
    pub static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub lod_index: i32,
    pub section_index: i32,
    pub uv_channel_index: i32,
    pub group_index: TArray<i32>,
    pub dilation: i32,
    pub generated_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
}
impl UGroomCreateStrandsTexturesOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomCreateStrandsTexturesOptions")
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
pub struct UGroomImportOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub conversion_settings: FGroomConversionSettings,
    pub interpolation_settings: TArray<FHairGroupsInterpolation>,
}
impl UGroomImportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomImportOptions")
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
pub struct UGroomHairGroupsPreview {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub groups: TArray<FGroomHairGroupPreview>,
}
impl UGroomHairGroupsPreview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomHairGroupsPreview")
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
pub struct UGroomHairGroupsMapping {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub old_group_names: TArray<FName>,
    pub new_group_names: TArray<FName>,
    pub old_to_new_group_index_mapping: TArray<i32>,
    pub new_to_old_group_index_mapping: TArray<i32>,
    __padding_end: [u8; 16],
}
impl UGroomHairGroupsMapping {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomHairGroupsMapping")
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
pub struct UGroomPluginSettings {
    __padding_end: [u8; 56],
}
impl UGroomPluginSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomPluginSettings")
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
pub struct UMovieSceneGroomCacheSection {
    __padding_end: [u8; 400],
}
impl UMovieSceneGroomCacheSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGroomCacheSection")
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
pub struct UMovieSceneGroomCacheTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneGroomCacheTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGroomCacheTrack")
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
pub struct UNiagaraDataInterfaceHairStrands {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceHairStrands {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceHairStrands")
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
pub struct UNiagaraDataInterfaceVelocityGrid {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceVelocityGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVelocityGrid")
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
pub struct UNiagaraDataInterfacePressureGrid {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfacePressureGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfacePressureGrid")
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
pub struct FBuild_CompletionDelegate {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EGroomCacheImportType(pub u8);
impl EGroomCacheImportType {
    pub const NONE: EGroomCacheImportType = EGroomCacheImportType(0);
    pub const STRANDS: EGroomCacheImportType = EGroomCacheImportType(1);
    pub const GUIDES: EGroomCacheImportType = EGroomCacheImportType(2);
    pub const ALL: EGroomCacheImportType = EGroomCacheImportType(3);
}
#[repr(transparent)]
pub struct EHairTextureLayout(pub u8);
impl EHairTextureLayout {
    pub const LAYOUT0: EHairTextureLayout = EHairTextureLayout(0);
    pub const LAYOUT1: EHairTextureLayout = EHairTextureLayout(1);
    pub const LAYOUT2: EHairTextureLayout = EHairTextureLayout(2);
    pub const LAYOUT3: EHairTextureLayout = EHairTextureLayout(3);
}
#[repr(transparent)]
pub struct EHairCardsSourceType(pub u8);
impl EHairCardsSourceType {
    pub const PROCEDURAL: EHairCardsSourceType = EHairCardsSourceType(0);
    pub const IMPORTED: EHairCardsSourceType = EHairCardsSourceType(1);
}
#[repr(transparent)]
pub struct EHairCardsGuideType(pub u8);
impl EHairCardsGuideType {
    pub const GENERATED: EHairCardsGuideType = EHairCardsGuideType(0);
    pub const GUIDE_BASED: EHairCardsGuideType = EHairCardsGuideType(1);
}
#[repr(transparent)]
pub struct EGroomGeometryType(pub u8);
impl EGroomGeometryType {
    pub const STRANDS: EGroomGeometryType = EGroomGeometryType(0);
    pub const CARDS: EGroomGeometryType = EGroomGeometryType(1);
    pub const MESHES: EGroomGeometryType = EGroomGeometryType(2);
}
#[repr(transparent)]
pub struct EGroomBindingType(pub u8);
impl EGroomBindingType {
    pub const NONE_BINDING: EGroomBindingType = EGroomBindingType(0);
    pub const RIGID: EGroomBindingType = EGroomBindingType(1);
    pub const SKINNING: EGroomBindingType = EGroomBindingType(2);
}
#[repr(transparent)]
pub struct EGroomOverrideType(pub u8);
impl EGroomOverrideType {
    pub const AUTO: EGroomOverrideType = EGroomOverrideType(0);
    pub const ENABLE: EGroomOverrideType = EGroomOverrideType(1);
    pub const DISABLE: EGroomOverrideType = EGroomOverrideType(2);
}
#[repr(transparent)]
pub struct EGroomGuideType(pub u8);
impl EGroomGuideType {
    pub const IMPORTED: EGroomGuideType = EGroomGuideType(0);
    pub const GENERATED: EGroomGuideType = EGroomGuideType(1);
    pub const RIGGED: EGroomGuideType = EGroomGuideType(2);
}
#[repr(transparent)]
pub struct EHairInterpolationQuality(pub u8);
impl EHairInterpolationQuality {
    pub const LOW: EHairInterpolationQuality = EHairInterpolationQuality(0);
    pub const MEDIUM: EHairInterpolationQuality = EHairInterpolationQuality(1);
    pub const HIGH: EHairInterpolationQuality = EHairInterpolationQuality(2);
    pub const UNKNOWN: EHairInterpolationQuality = EHairInterpolationQuality(3);
}
#[repr(transparent)]
pub struct EHairInterpolationWeight(pub u8);
impl EHairInterpolationWeight {
    pub const PARAMETRIC: EHairInterpolationWeight = EHairInterpolationWeight(0);
    pub const ROOT: EHairInterpolationWeight = EHairInterpolationWeight(1);
    pub const INDEX: EHairInterpolationWeight = EHairInterpolationWeight(2);
    pub const DISTANCE: EHairInterpolationWeight = EHairInterpolationWeight(3);
    pub const UNKNOWN: EHairInterpolationWeight = EHairInterpolationWeight(4);
}
#[repr(transparent)]
pub struct EGroomNiagaraSolvers(pub u8);
impl EGroomNiagaraSolvers {
    pub const NONE: EGroomNiagaraSolvers = EGroomNiagaraSolvers(0);
    pub const COSSERAT_RODS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(2);
    pub const ANGULAR_SPRINGS: EGroomNiagaraSolvers = EGroomNiagaraSolvers(4);
    pub const CUSTOM_SOLVER: EGroomNiagaraSolvers = EGroomNiagaraSolvers(8);
}
#[repr(transparent)]
pub struct EGroomStrandsSize(pub u8);
impl EGroomStrandsSize {
    pub const NONE: EGroomStrandsSize = EGroomStrandsSize(0);
    pub const SIZE2: EGroomStrandsSize = EGroomStrandsSize(2);
    pub const SIZE4: EGroomStrandsSize = EGroomStrandsSize(4);
    pub const SIZE8: EGroomStrandsSize = EGroomStrandsSize(8);
    pub const SIZE16: EGroomStrandsSize = EGroomStrandsSize(16);
    pub const SIZE32: EGroomStrandsSize = EGroomStrandsSize(32);
}
#[repr(transparent)]
pub struct EGroomCacheAttributes(pub u8);
impl EGroomCacheAttributes {
    pub const NONE: EGroomCacheAttributes = EGroomCacheAttributes(0);
    pub const POSITION: EGroomCacheAttributes = EGroomCacheAttributes(1);
    pub const WIDTH: EGroomCacheAttributes = EGroomCacheAttributes(2);
    pub const COLOR: EGroomCacheAttributes = EGroomCacheAttributes(4);
    pub const POSITION_WIDTH: EGroomCacheAttributes = EGroomCacheAttributes(3);
    pub const POSITION_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(5);
    pub const WIDTH_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(6);
    pub const POSITION_WIDTH_COLOR: EGroomCacheAttributes = EGroomCacheAttributes(7);
}
#[repr(transparent)]
pub struct EGroomCacheType(pub u8);
impl EGroomCacheType {
    pub const NONE: EGroomCacheType = EGroomCacheType(0);
    pub const STRANDS: EGroomCacheType = EGroomCacheType(1);
    pub const GUIDES: EGroomCacheType = EGroomCacheType(2);
}
#[repr(transparent)]
pub struct EFollicleMaskChannel(pub u8);
impl EFollicleMaskChannel {
    pub const R: EFollicleMaskChannel = EFollicleMaskChannel(0);
    pub const G: EFollicleMaskChannel = EFollicleMaskChannel(1);
    pub const B: EFollicleMaskChannel = EFollicleMaskChannel(2);
    pub const A: EFollicleMaskChannel = EFollicleMaskChannel(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationQuality(pub u8);
impl EGroomInterpolationQuality {
    pub const LOW: EGroomInterpolationQuality = EGroomInterpolationQuality(0);
    pub const MEDIUM: EGroomInterpolationQuality = EGroomInterpolationQuality(1);
    pub const HIGH: EGroomInterpolationQuality = EGroomInterpolationQuality(2);
    pub const UNKNOWN: EGroomInterpolationQuality = EGroomInterpolationQuality(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationWeight(pub u8);
impl EGroomInterpolationWeight {
    pub const PARAMETRIC: EGroomInterpolationWeight = EGroomInterpolationWeight(0);
    pub const ROOT: EGroomInterpolationWeight = EGroomInterpolationWeight(1);
    pub const INDEX: EGroomInterpolationWeight = EGroomInterpolationWeight(2);
    pub const UNKNOWN: EGroomInterpolationWeight = EGroomInterpolationWeight(3);
}
#[repr(transparent)]
pub struct EGroomInterpolationType(pub u8);
impl EGroomInterpolationType {
    pub const NONE: EGroomInterpolationType = EGroomInterpolationType(0);
    pub const RIGID_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(2);
    pub const OFFSET_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(4);
    pub const SMOOTH_TRANSFORM: EGroomInterpolationType = EGroomInterpolationType(8);
}
#[repr(transparent)]
pub struct EGroomBindingAssetBuildResult(pub u8);
impl EGroomBindingAssetBuildResult {
    pub const SUCCEEDED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(
        0,
    );
    pub const FAILED: EGroomBindingAssetBuildResult = EGroomBindingAssetBuildResult(1);
}
#[repr(transparent)]
pub struct EGroomBindingMeshType(pub u8);
impl EGroomBindingMeshType {
    pub const SKELETAL_MESH: EGroomBindingMeshType = EGroomBindingMeshType(0);
    pub const GEOMETRY_CACHE: EGroomBindingMeshType = EGroomBindingMeshType(1);
}
#[repr(transparent)]
pub struct EGroomLODMode(pub u8);
impl EGroomLODMode {
    pub const DEFAULT: EGroomLODMode = EGroomLODMode(0);
    pub const MANUAL: EGroomLODMode = EGroomLODMode(1);
    pub const AUTO: EGroomLODMode = EGroomLODMode(2);
}
#[repr(transparent)]
pub struct EHairDescriptionType(pub u8);
impl EHairDescriptionType {
    pub const SOURCE: EHairDescriptionType = EHairDescriptionType(0);
    pub const EDIT: EHairDescriptionType = EHairDescriptionType(1);
    pub const COUNT: EHairDescriptionType = EHairDescriptionType(2);
}
#[repr(transparent)]
pub struct EStrandsTexturesTraceType(pub u8);
impl EStrandsTexturesTraceType {
    pub const TRACE_INSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(0);
    pub const TRACE_OUSIDE: EStrandsTexturesTraceType = EStrandsTexturesTraceType(1);
    pub const TRACE_BIDIRECTIONAL: EStrandsTexturesTraceType = EStrandsTexturesTraceType(
        2,
    );
}
#[repr(transparent)]
pub struct EStrandsTexturesMeshType(pub u8);
impl EStrandsTexturesMeshType {
    pub const STATIC: EStrandsTexturesMeshType = EStrandsTexturesMeshType(0);
    pub const SKELETAL: EStrandsTexturesMeshType = EStrandsTexturesMeshType(1);
}
