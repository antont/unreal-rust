#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_dynamic_mesh_processor_blueprint_process_dynamic_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_view_mode_overrides_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_vertex_color_space_transform_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_shadows_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_secondary_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_secondary_buffers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_override_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_override_secondary_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_override_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_mesh_draw_path: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_enable_wireframe_render_pass: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_enable_raytracing: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_enable_flat_shading: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_distance_field_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_constant_override_color: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_set_color_override_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_has_override_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_view_mode_overrides_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_vertex_color_space_transform_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_shadows_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_secondary_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_secondary_buffers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_override_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_override_secondary_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_override_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_mesh_draw_path: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_flat_shading_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_enable_wireframe_render_pass: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_enable_raytracing: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_dynamic_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_distance_field_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_constant_override_color: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_get_color_override_mode: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_clear_secondary_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_clear_override_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_clear_override_secondary_wireframe_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_base_dynamic_mesh_component_clear_override_render_material: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_validate_material_slots: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_update_collision: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_set_tangents_type: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_set_dynamic_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_set_deferred_collision_updates_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_set_complex_as_simple_collision_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_set_allows_geometry_selection: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_notify_mesh_vertex_attributes_modified: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_notify_mesh_modified: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_get_tangents_type_pure: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_get_tangents_type: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_enable_complex_as_simple_collision: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_configure_material_set: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_component_allows_geometry_selection: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_release_compute_mesh: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_release_all_compute_meshes: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_get_dynamic_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_get_compute_mesh_pool: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_free_all_compute_meshes: *mut crate::ffi::UFunctionOpague,
    pub a_dynamic_mesh_actor_allocate_compute_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_reset_to_cube: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_reset: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_is_empty: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_get_triangle_count: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_pool_return_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_pool_return_all_meshes: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_pool_request_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_mesh_pool_free_all_meshes: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_dynamic_mesh_processor_blueprint_process_dynamic_mesh: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_view_mode_overrides_enabled: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_vertex_color_space_transform_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_shadows_enabled: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_secondary_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_secondary_buffers_visibility: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_override_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_override_secondary_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_override_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_mesh_draw_path: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_enable_wireframe_render_pass: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_enable_raytracing: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_enable_flat_shading: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_distance_field_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_constant_override_color: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_set_color_override_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_has_override_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_view_mode_overrides_enabled: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_vertex_color_space_transform_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_shadows_enabled: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_secondary_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_secondary_buffers_visibility: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_override_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_override_secondary_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_override_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_mesh_draw_path: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_flat_shading_enabled: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_enable_wireframe_render_pass: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_enable_raytracing: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_dynamic_mesh: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_distance_field_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_constant_override_color: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_get_color_override_mode: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_clear_secondary_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_clear_override_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_clear_override_secondary_wireframe_render_material: std::ptr::null_mut(),
            u_base_dynamic_mesh_component_clear_override_render_material: std::ptr::null_mut(),
            u_dynamic_mesh_component_validate_material_slots: std::ptr::null_mut(),
            u_dynamic_mesh_component_update_collision: std::ptr::null_mut(),
            u_dynamic_mesh_component_set_tangents_type: std::ptr::null_mut(),
            u_dynamic_mesh_component_set_dynamic_mesh: std::ptr::null_mut(),
            u_dynamic_mesh_component_set_deferred_collision_updates_enabled: std::ptr::null_mut(),
            u_dynamic_mesh_component_set_complex_as_simple_collision_enabled: std::ptr::null_mut(),
            u_dynamic_mesh_component_set_allows_geometry_selection: std::ptr::null_mut(),
            u_dynamic_mesh_component_notify_mesh_vertex_attributes_modified: std::ptr::null_mut(),
            u_dynamic_mesh_component_notify_mesh_modified: std::ptr::null_mut(),
            u_dynamic_mesh_component_get_tangents_type_pure: std::ptr::null_mut(),
            u_dynamic_mesh_component_get_tangents_type: std::ptr::null_mut(),
            u_dynamic_mesh_component_enable_complex_as_simple_collision: std::ptr::null_mut(),
            u_dynamic_mesh_component_configure_material_set: std::ptr::null_mut(),
            u_dynamic_mesh_component_allows_geometry_selection: std::ptr::null_mut(),
            a_dynamic_mesh_actor_release_compute_mesh: std::ptr::null_mut(),
            a_dynamic_mesh_actor_release_all_compute_meshes: std::ptr::null_mut(),
            a_dynamic_mesh_actor_get_dynamic_mesh_component: std::ptr::null_mut(),
            a_dynamic_mesh_actor_get_compute_mesh_pool: std::ptr::null_mut(),
            a_dynamic_mesh_actor_free_all_compute_meshes: std::ptr::null_mut(),
            a_dynamic_mesh_actor_allocate_compute_mesh: std::ptr::null_mut(),
            u_dynamic_mesh_reset_to_cube: std::ptr::null_mut(),
            u_dynamic_mesh_reset: std::ptr::null_mut(),
            u_dynamic_mesh_is_empty: std::ptr::null_mut(),
            u_dynamic_mesh_get_triangle_count: std::ptr::null_mut(),
            u_dynamic_mesh_pool_return_mesh: std::ptr::null_mut(),
            u_dynamic_mesh_pool_return_all_meshes: std::ptr::null_mut(),
            u_dynamic_mesh_pool_request_mesh: std::ptr::null_mut(),
            u_dynamic_mesh_pool_free_all_meshes: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDynamicMeshProcessorBlueprint::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProcessDynamicMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_processor_blueprint_process_dynamic_mesh,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBaseDynamicMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetViewModeOverridesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_view_mode_overrides_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVertexColorSpaceTransformMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_vertex_color_space_transform_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShadowsEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_shadows_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSecondaryRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSecondaryBuffersVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_buffers_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOverrideWireframeRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOverrideSecondaryWireframeRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_secondary_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetOverrideRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMeshDrawPath"),
                &raw mut __FUNCTION_PTRS.u_base_dynamic_mesh_component_set_mesh_draw_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableWireframeRenderPass"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_wireframe_render_pass,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableRaytracing"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_raytracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableFlatShading"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_flat_shading,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDistanceFieldMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_distance_field_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConstantOverrideColor"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_constant_override_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetColorOverrideMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_color_override_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasOverrideRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_has_override_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetViewModeOverridesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_view_mode_overrides_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVertexColorSpaceTransformMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_vertex_color_space_transform_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShadowsEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_shadows_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSecondaryRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSecondaryBuffersVisibility"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_buffers_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOverrideWireframeRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOverrideSecondaryWireframeRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_secondary_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOverrideRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMeshDrawPath"),
                &raw mut __FUNCTION_PTRS.u_base_dynamic_mesh_component_get_mesh_draw_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFlatShadingEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_flat_shading_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnableWireframeRenderPass"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_wireframe_render_pass,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnableRaytracing"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_raytracing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDynamicMesh"),
                &raw mut __FUNCTION_PTRS.u_base_dynamic_mesh_component_get_dynamic_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDistanceFieldMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_distance_field_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConstantOverrideColor"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_constant_override_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetColorOverrideMode"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_color_override_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearSecondaryRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_secondary_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideWireframeRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "ClearOverrideSecondaryWireframeRenderMaterial",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_secondary_wireframe_render_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideRenderMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_render_material,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDynamicMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ValidateMaterialSlots"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_validate_material_slots,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateCollision"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_update_collision,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTangentsType"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_set_tangents_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDynamicMesh"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_set_dynamic_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDeferredCollisionUpdatesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_deferred_collision_updates_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetComplexAsSimpleCollisionEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_complex_as_simple_collision_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAllowsGeometrySelection"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_allows_geometry_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotifyMeshVertexAttributesModified"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_notify_mesh_vertex_attributes_modified,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotifyMeshModified"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_notify_mesh_modified,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTangentsTypePure"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_get_tangents_type_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTangentsType"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_get_tangents_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableComplexAsSimpleCollision"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_enable_complex_as_simple_collision,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConfigureMaterialSet"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_component_configure_material_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AllowsGeometrySelection"),
                &raw mut __FUNCTION_PTRS
                    .u_dynamic_mesh_component_allows_geometry_selection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ADynamicMeshActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReleaseComputeMesh"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_release_compute_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReleaseAllComputeMeshes"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_release_all_compute_meshes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDynamicMeshComponent"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_get_dynamic_mesh_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComputeMeshPool"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_get_compute_mesh_pool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FreeAllComputeMeshes"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_free_all_compute_meshes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AllocateComputeMesh"),
                &raw mut __FUNCTION_PTRS.a_dynamic_mesh_actor_allocate_compute_mesh,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDynamicMesh::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetToCube"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_reset_to_cube,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reset"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_reset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEmpty"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_is_empty,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTriangleCount"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_get_triangle_count,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDynamicMeshPool::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReturnMesh"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_pool_return_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReturnAllMeshes"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_pool_return_all_meshes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestMesh"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_pool_request_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FreeAllMeshes"),
                &raw mut __FUNCTION_PTRS.u_dynamic_mesh_pool_free_all_meshes,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FDynamicMeshChangeInfo {
    pub ty: EDynamicMeshChangeType,
    pub flags: EDynamicMeshAttributeChangeFlags,
    pub b_is_revert_change: bool,
    pub(crate) __padding_end: [u8; 29],
}
impl FDynamicMeshChangeInfo {}
#[repr(C, align(8))]
pub struct UDynamicMeshProcessorBlueprint {
    __padding_end: [u8; 56],
}
impl UDynamicMeshProcessorBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshProcessorBlueprint")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshProcessorBlueprint")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn process_dynamic_mesh(
        &mut self,
        target_mesh: UPtr<UDynamicMesh>,
        b_failed: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_processor_blueprint_process_dynamic_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_mesh,
                __buffer.add(0).cast::<UPtr<UDynamicMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_failed, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_processor_blueprint_process_dynamic_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_failed);
        }
    }
}
pub struct IMeshCommandChangeTarget {}
#[repr(C, align(8))]
pub struct UMeshCommandChangeTarget {
    __padding_end: [u8; 48],
}
impl UMeshCommandChangeTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshCommandChangeTarget")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshCommandChangeTarget")
            .copied()
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
pub struct IMeshReplacementCommandChangeTarget {}
#[repr(C, align(8))]
pub struct UMeshReplacementCommandChangeTarget {
    __padding_end: [u8; 48],
}
impl UMeshReplacementCommandChangeTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshReplacementCommandChangeTarget")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshReplacementCommandChangeTarget")
            .copied()
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
pub struct IMeshVertexCommandChangeTarget {}
#[repr(C, align(8))]
pub struct UMeshVertexCommandChangeTarget {
    __padding_end: [u8; 48],
}
impl UMeshVertexCommandChangeTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexCommandChangeTarget")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexCommandChangeTarget")
            .copied()
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
pub struct UBaseDynamicMeshComponent {
    #[doc(hidden)]
    pub(crate) __padding_1608: [u8; 1608],
    pub b_explicit_show_wireframe: bool,
    pub wireframe_color: crate::bindings::core_u_object::FLinearColor,
    pub color_mode: EDynamicMeshComponentColorOverrideMode,
    pub constant_color: crate::bindings::core_u_object::FColor,
    pub color_space_mode: EDynamicMeshVertexColorTransformMode,
    #[doc(hidden)]
    pub(crate) __padding_1638: [u8; 1],
    pub b_enable_flat_shading: bool,
    pub b_enable_view_mode_overrides: bool,
    #[doc(hidden)]
    pub(crate) __padding_1680: [u8; 40],
    pub b_enable_raytracing: bool,
    pub draw_path: EDynamicMeshDrawPath,
    __padding_end: [u8; 30],
}
impl UBaseDynamicMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseDynamicMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseDynamicMeshComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_view_mode_overrides_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_view_mode_overrides_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_view_mode_overrides_enabled,
                __buffer,
            )
        };
    }
    pub fn set_vertex_color_space_transform_mode(
        &mut self,
        new_mode: EDynamicMeshVertexColorTransformMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_vertex_color_space_transform_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mode,
                __buffer.add(0).cast::<EDynamicMeshVertexColorTransformMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_vertex_color_space_transform_mode,
                __buffer,
            )
        };
    }
    pub fn set_shadows_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_shadows_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_shadows_enabled,
                __buffer,
            )
        };
    }
    pub fn set_secondary_render_material(
        &mut self,
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_render_material,
                __buffer,
            )
        };
    }
    pub fn set_secondary_buffers_visibility(&mut self, b_set_visible: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_buffers_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_visible,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_secondary_buffers_visibility,
                __buffer,
            )
        };
    }
    pub fn set_override_wireframe_render_material(
        &mut self,
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_wireframe_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_wireframe_render_material,
                __buffer,
            )
        };
    }
    pub fn set_override_secondary_wireframe_render_material(
        &mut self,
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
    }
    pub fn set_override_render_material(
        &mut self,
        material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material,
                __buffer
                    .add(0)
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_override_render_material,
                __buffer,
            )
        };
    }
    pub fn set_mesh_draw_path(&mut self, new_draw_path: EDynamicMeshDrawPath) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_mesh_draw_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_draw_path,
                __buffer.add(0).cast::<EDynamicMeshDrawPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_mesh_draw_path,
                __buffer,
            )
        };
    }
    pub fn set_enable_wireframe_render_pass(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_wireframe_render_pass,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_wireframe_render_pass,
                __buffer,
            )
        };
    }
    pub fn set_enable_raytracing(&mut self, b_set_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_raytracing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_enabled,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_raytracing,
                __buffer,
            )
        };
    }
    pub fn set_enable_flat_shading(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_flat_shading,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_enable_flat_shading,
                __buffer,
            )
        };
    }
    pub fn set_constant_override_color(
        &mut self,
        new_color: crate::bindings::core_u_object::FColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_constant_override_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_color,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_constant_override_color,
                __buffer,
            )
        };
    }
    pub fn set_color_override_mode(
        &mut self,
        new_mode: EDynamicMeshComponentColorOverrideMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_color_override_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mode,
                __buffer.add(0).cast::<EDynamicMeshComponentColorOverrideMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_set_color_override_mode,
                __buffer,
            )
        };
    }
    pub fn has_override_render_material(&self, k: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_has_override_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&k, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_has_override_render_material,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_view_mode_overrides_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_view_mode_overrides_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_view_mode_overrides_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_vertex_color_space_transform_mode(
        &self,
    ) -> EDynamicMeshVertexColorTransformMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_vertex_color_space_transform_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_vertex_color_space_transform_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EDynamicMeshVertexColorTransformMode>().read() }
    }
    pub fn get_shadows_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_shadows_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_shadows_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_secondary_render_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_render_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_secondary_buffers_visibility(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_buffers_visibility,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_secondary_buffers_visibility,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_override_wireframe_render_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_wireframe_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_wireframe_render_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_override_secondary_wireframe_render_material(
        &self,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_override_render_material(
        &self,
        material_index: i32,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_render_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_index,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_override_render_material,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn get_mesh_draw_path(&self) -> EDynamicMeshDrawPath {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_mesh_draw_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_mesh_draw_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EDynamicMeshDrawPath>().read() }
    }
    pub fn get_flat_shading_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_flat_shading_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_flat_shading_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_enable_wireframe_render_pass(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_wireframe_render_pass,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_wireframe_render_pass,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_enable_raytracing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_raytracing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_enable_raytracing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_dynamic_mesh(&mut self) -> UPtr<UDynamicMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_dynamic_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_dynamic_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMesh>>().read() }
    }
    pub fn get_constant_override_color(&self) -> crate::bindings::core_u_object::FColor {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_constant_override_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_constant_override_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FColor>().read()
        }
    }
    pub fn get_color_override_mode(&self) -> EDynamicMeshComponentColorOverrideMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_color_override_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_get_color_override_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EDynamicMeshComponentColorOverrideMode>().read()
        }
    }
    pub fn clear_secondary_render_material(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_secondary_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_secondary_render_material,
                __buffer,
            )
        };
    }
    pub fn clear_override_wireframe_render_material(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_wireframe_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_wireframe_render_material,
                __buffer,
            )
        };
    }
    pub fn clear_override_secondary_wireframe_render_material(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_secondary_wireframe_render_material,
                __buffer,
            )
        };
    }
    pub fn clear_override_render_material(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_render_material,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_base_dynamic_mesh_component_clear_override_render_material,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UDynamicMeshComponent {
    #[doc(hidden)]
    pub(crate) __padding_2040: [u8; 2040],
    pub tangents_type: EDynamicMeshComponentTangentsMode,
    #[doc(hidden)]
    pub(crate) __padding_2272: [u8; 231],
    pub collision_type: crate::bindings::physics_core::ECollisionTraceFlag,
    pub b_use_async_cooking: bool,
    pub b_enable_complex_collision: bool,
    pub b_defer_collision_updates: bool,
    pub b_disable_mesh_uv_hit_results: bool,
    __padding_end: [u8; 251],
}
impl UDynamicMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn validate_material_slots(
        &mut self,
        b_create_if_missing: bool,
        b_delete_extra_slots: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_validate_material_slots,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_if_missing,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_delete_extra_slots,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_validate_material_slots,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn update_collision(&mut self, b_only_if_pending: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_update_collision,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_pending,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_update_collision,
                __buffer,
            )
        };
    }
    pub fn set_tangents_type(
        &mut self,
        new_tangents_type: EDynamicMeshComponentTangentsMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_tangents_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tangents_type,
                __buffer.add(0).cast::<EDynamicMeshComponentTangentsMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_tangents_type,
                __buffer,
            )
        };
    }
    pub fn set_dynamic_mesh(&mut self, new_mesh: UPtr<UDynamicMesh>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_dynamic_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer.add(0).cast::<UPtr<UDynamicMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_dynamic_mesh,
                __buffer,
            )
        };
    }
    pub fn set_deferred_collision_updates_enabled(
        &mut self,
        b_enabled: bool,
        b_immediate_update: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_deferred_collision_updates_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate_update,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_deferred_collision_updates_enabled,
                __buffer,
            )
        };
    }
    pub fn set_complex_as_simple_collision_enabled(
        &mut self,
        b_enabled: bool,
        b_immediate_update: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_complex_as_simple_collision_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediate_update,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_complex_as_simple_collision_enabled,
                __buffer,
            )
        };
    }
    pub fn set_allows_geometry_selection(
        &mut self,
        b_in_allows_geometry_selection: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_allows_geometry_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_allows_geometry_selection,
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
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_set_allows_geometry_selection,
                __buffer,
            )
        };
    }
    pub fn notify_mesh_vertex_attributes_modified(
        &mut self,
        b_positions: bool,
        b_normals: bool,
        b_u_vs: bool,
        b_colors: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_notify_mesh_vertex_attributes_modified,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_positions,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_normals, __buffer.add(1).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_u_vs, __buffer.add(2).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_colors, __buffer.add(3).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_notify_mesh_vertex_attributes_modified,
                __buffer,
            )
        };
    }
    pub fn notify_mesh_modified(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_notify_mesh_modified,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_notify_mesh_modified,
                __buffer,
            )
        };
    }
    pub fn get_tangents_type_pure(&self) -> EDynamicMeshComponentTangentsMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_get_tangents_type_pure,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_get_tangents_type_pure,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EDynamicMeshComponentTangentsMode>().read() }
    }
    pub fn get_tangents_type(&self) -> EDynamicMeshComponentTangentsMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_get_tangents_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_get_tangents_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EDynamicMeshComponentTangentsMode>().read() }
    }
    pub fn enable_complex_as_simple_collision(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_enable_complex_as_simple_collision,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_enable_complex_as_simple_collision,
                __buffer,
            )
        };
    }
    pub fn configure_material_set(
        &mut self,
        new_material_set: &TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
        b_delete_extra_slots: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_configure_material_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_material_set,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::engine::UMaterialInterface>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_delete_extra_slots,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_configure_material_set,
                __buffer,
            )
        };
    }
    pub fn allows_geometry_selection(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_allows_geometry_selection,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_component_allows_geometry_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ADynamicMeshActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub dynamic_mesh_component: UPtr<UDynamicMeshComponent>,
    pub b_enable_compute_mesh_pool: bool,
    __padding_end: [u8; 15],
}
impl ADynamicMeshActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADynamicMeshActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADynamicMeshActor")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn release_compute_mesh(&mut self, mesh: UPtr<UDynamicMesh>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_release_compute_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh,
                __buffer.add(0).cast::<UPtr<UDynamicMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_release_compute_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn release_all_compute_meshes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_release_all_compute_meshes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_release_all_compute_meshes,
                __buffer,
            )
        };
    }
    pub fn get_dynamic_mesh_component(&self) -> UPtr<UDynamicMeshComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_get_dynamic_mesh_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_get_dynamic_mesh_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMeshComponent>>().read() }
    }
    pub fn get_compute_mesh_pool(&mut self) -> UPtr<UDynamicMeshPool> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_get_compute_mesh_pool,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_get_compute_mesh_pool,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMeshPool>>().read() }
    }
    pub fn free_all_compute_meshes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_free_all_compute_meshes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_free_all_compute_meshes,
                __buffer,
            )
        };
    }
    pub fn allocate_compute_mesh(&mut self) -> UPtr<UDynamicMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_allocate_compute_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .a_dynamic_mesh_actor_allocate_compute_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMesh>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDynamicMeshGenerator {
    __padding_end: [u8; 48],
}
impl UDynamicMeshGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshGenerator")
            .copied()
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
pub struct UDynamicMesh {
    __padding_end: [u8; 216],
}
impl UDynamicMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMesh")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMesh")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn reset_to_cube(&mut self) -> UPtr<UDynamicMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_reset_to_cube,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_reset_to_cube,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMesh>>().read() }
    }
    pub fn reset(&mut self) -> UPtr<UDynamicMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_reset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_reset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMesh>>().read() }
    }
    pub fn is_empty(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_is_empty,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_is_empty,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_triangle_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_get_triangle_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_get_triangle_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDynamicMeshPool {
    __padding_end: [u8; 120],
}
impl UDynamicMeshPool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshPool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshPool")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn return_mesh(&mut self, mesh: UPtr<UDynamicMesh>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_return_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh,
                __buffer.add(0).cast::<UPtr<UDynamicMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_return_mesh,
                __buffer,
            )
        };
    }
    pub fn return_all_meshes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_return_all_meshes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_return_all_meshes,
                __buffer,
            )
        };
    }
    pub fn request_mesh(&mut self) -> UPtr<UDynamicMesh> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_request_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_request_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UDynamicMesh>>().read() }
    }
    pub fn free_all_meshes(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_free_all_meshes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::geometry_framework::__FUNCTION_PTRS
                    .u_dynamic_mesh_pool_free_all_meshes,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FDynamicMesh_MeshModifiedBPEvent {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EDynamicMeshChangeType(pub u8);
impl EDynamicMeshChangeType {
    pub const GENERAL_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(0);
    pub const MESH_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(1);
    pub const MESH_REPLACEMENT_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(
        2,
    );
    pub const MESH_VERTEX_CHANGE: EDynamicMeshChangeType = EDynamicMeshChangeType(3);
    pub const DEFORMATION_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(4);
    pub const ATTRIBUTE_EDIT: EDynamicMeshChangeType = EDynamicMeshChangeType(5);
}
#[repr(transparent)]
pub struct EDynamicMeshAttributeChangeFlags(pub u8);
impl EDynamicMeshAttributeChangeFlags {
    pub const UNKNOWN: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        0,
    );
    pub const MESH_TOPOLOGY: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        1,
    );
    pub const VERTEX_POSITIONS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        2,
    );
    pub const NORMALS_TANGENTS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        4,
    );
    pub const VERTEX_COLORS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        8,
    );
    pub const U_VS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        16,
    );
    pub const TRIANGLE_GROUPS: EDynamicMeshAttributeChangeFlags = EDynamicMeshAttributeChangeFlags(
        32,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentColorOverrideMode(pub u8);
impl EDynamicMeshComponentColorOverrideMode {
    pub const NONE: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        0,
    );
    pub const VERTEX_COLORS: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        1,
    );
    pub const POLYGROUPS: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        2,
    );
    pub const CONSTANT: EDynamicMeshComponentColorOverrideMode = EDynamicMeshComponentColorOverrideMode(
        3,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentDistanceFieldMode(pub u8);
impl EDynamicMeshComponentDistanceFieldMode {
    pub const NO_DISTANCE_FIELD: EDynamicMeshComponentDistanceFieldMode = EDynamicMeshComponentDistanceFieldMode(
        0,
    );
    pub const ASYNC_CPU_DISTANCE_FIELD: EDynamicMeshComponentDistanceFieldMode = EDynamicMeshComponentDistanceFieldMode(
        1,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshDrawPath(pub u8);
impl EDynamicMeshDrawPath {
    pub const DYNAMIC_DRAW: EDynamicMeshDrawPath = EDynamicMeshDrawPath(0);
    pub const STATIC_DRAW: EDynamicMeshDrawPath = EDynamicMeshDrawPath(1);
}
#[repr(transparent)]
pub struct EDynamicMeshVertexColorTransformMode(pub u8);
impl EDynamicMeshVertexColorTransformMode {
    pub const NO_TRANSFORM: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        0,
    );
    pub const LINEAR_TO_SRGB: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        1,
    );
    pub const SRGB_TO_LINEAR: EDynamicMeshVertexColorTransformMode = EDynamicMeshVertexColorTransformMode(
        2,
    );
}
#[repr(transparent)]
pub struct EDynamicMeshComponentTangentsMode(pub u8);
impl EDynamicMeshComponentTangentsMode {
    pub const NO_TANGENTS: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        0,
    );
    pub const AUTO_CALCULATED: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        1,
    );
    pub const EXTERNALLY_PROVIDED: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        2,
    );
    pub const DEFAULT: EDynamicMeshComponentTangentsMode = EDynamicMeshComponentTangentsMode(
        255,
    );
}
