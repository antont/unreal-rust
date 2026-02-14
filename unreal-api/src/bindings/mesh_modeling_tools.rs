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
    pub u_mesh_constraint_properties_is_prevent_tiny_triangles_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_constraint_properties_is_prevent_normal_flips_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_convert_to_polygons_tool_properties_get_group_options_list: *mut crate::ffi::UFunctionOpague,
    pub u_displace_mesh_common_properties_get_weight_maps_func: *mut crate::ffi::UFunctionOpague,
    pub u_selective_tessellation_properties_get_material_i_ds_func: *mut crate::ffi::UFunctionOpague,
    pub u_dynamic_sculpt_tool_actions_discard_attributes: *mut crate::ffi::UFunctionOpague,
    pub u_poly_edit_topology_properties_regenerate_extra_corners: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_simplify_by_groups: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_retriangulate: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_recalc_normals: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_push_pull: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_outset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_offset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_merge: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_inset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_insert_edge_loop: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_insert_edge: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_flip: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_extrude: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_duplicate: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_disconnect: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_delete: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_decompose: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_cut_faces: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_bevel: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_recalc_normals: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_push_pull: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_poke: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_outset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_offset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_inset: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_flip: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_extrude: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_duplicate: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_disconnect: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_delete: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_actions_triangles_cut_faces: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_uv_actions_planar_projection: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_weld_centered: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_weld: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_straighten: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_simplify: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_fill_hole: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_extrude: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_delete_edge: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_collapse: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_bridge: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_bevel: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_weld_centered: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_weld: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_split: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_flip: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_fill_hole: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_extrude: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_collapse: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_polygons_tool_edge_actions_triangles_bridge: *mut crate::ffi::UFunctionOpague,
    pub u_hole_fill_tool_actions_select_all: *mut crate::ffi::UFunctionOpague,
    pub u_hole_fill_tool_actions_clear: *mut crate::ffi::UFunctionOpague,
    pub u_lattice_deformer_tool_properties_constrain: *mut crate::ffi::UFunctionOpague,
    pub u_lattice_deformer_tool_properties_clear_constraints: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_attribute_paint_tool_properties_get_attribute_names: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_unfreeze_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_shrink_current: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_grow_current: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_freeze_others: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_freeze_current: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_flood_fill_current: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_clear_current: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_group_paint_tool_freeze_actions_clear_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_space_deformer_tool_action_property_set_shift_to_center: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_quick_actions_paint_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_quick_actions_fill_white: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_quick_actions_fill_black: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_quick_actions_erase_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_utility_actions_get_weight_maps_func: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_utility_actions_get_lod_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_vertex_paint_tool_utility_actions_apply_selected_operation: *mut crate::ffi::UFunctionOpague,
    pub u_existing_mesh_material_properties_get_uv_channel_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_set_layer_weight: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_set_layer_name: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_set_active_layer: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_remove_layer_at_index: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_move_layer: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_sculpt_layer_properties_get_layer_name: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_uv_channel_properties_get_uv_channel_names_func: *mut crate::ffi::UFunctionOpague,
    pub uuv_projection_tool_edit_actions_reset: *mut crate::ffi::UFunctionOpague,
    pub uuv_projection_tool_edit_actions_auto_fit_align: *mut crate::ffi::UFunctionOpague,
    pub uuv_projection_tool_edit_actions_auto_fit: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mesh_constraint_properties_is_prevent_tiny_triangles_enabled: std::ptr::null_mut(),
            u_mesh_constraint_properties_is_prevent_normal_flips_enabled: std::ptr::null_mut(),
            u_convert_to_polygons_tool_properties_get_group_options_list: std::ptr::null_mut(),
            u_displace_mesh_common_properties_get_weight_maps_func: std::ptr::null_mut(),
            u_selective_tessellation_properties_get_material_i_ds_func: std::ptr::null_mut(),
            u_dynamic_sculpt_tool_actions_discard_attributes: std::ptr::null_mut(),
            u_poly_edit_topology_properties_regenerate_extra_corners: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_simplify_by_groups: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_retriangulate: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_recalc_normals: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_push_pull: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_outset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_offset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_merge: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_inset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_insert_edge_loop: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_insert_edge: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_flip: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_extrude: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_duplicate: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_disconnect: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_delete: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_decompose: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_cut_faces: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_bevel: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_recalc_normals: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_push_pull: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_poke: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_outset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_offset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_inset: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_flip: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_extrude: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_duplicate: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_disconnect: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_delete: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_actions_triangles_cut_faces: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_uv_actions_planar_projection: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_weld_centered: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_weld: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_straighten: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_simplify: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_fill_hole: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_extrude: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_delete_edge: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_collapse: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_bridge: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_bevel: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_weld_centered: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_weld: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_split: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_flip: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_fill_hole: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_extrude: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_collapse: std::ptr::null_mut(),
            u_edit_mesh_polygons_tool_edge_actions_triangles_bridge: std::ptr::null_mut(),
            u_hole_fill_tool_actions_select_all: std::ptr::null_mut(),
            u_hole_fill_tool_actions_clear: std::ptr::null_mut(),
            u_lattice_deformer_tool_properties_constrain: std::ptr::null_mut(),
            u_lattice_deformer_tool_properties_clear_constraints: std::ptr::null_mut(),
            u_mesh_attribute_paint_tool_properties_get_attribute_names: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_unfreeze_all: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_shrink_current: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_grow_current: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_freeze_others: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_freeze_current: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_flood_fill_current: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_clear_current: std::ptr::null_mut(),
            u_mesh_group_paint_tool_freeze_actions_clear_all: std::ptr::null_mut(),
            u_mesh_space_deformer_tool_action_property_set_shift_to_center: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_quick_actions_paint_all: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_quick_actions_fill_white: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_quick_actions_fill_black: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_quick_actions_erase_all: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_utility_actions_get_weight_maps_func: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_utility_actions_get_lod_names_func: std::ptr::null_mut(),
            u_mesh_vertex_paint_tool_utility_actions_apply_selected_operation: std::ptr::null_mut(),
            u_existing_mesh_material_properties_get_uv_channel_names_func: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_set_layer_weight: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_set_layer_name: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_set_active_layer: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_remove_layer_at_index: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_move_layer: std::ptr::null_mut(),
            u_mesh_sculpt_layer_properties_get_layer_name: std::ptr::null_mut(),
            u_mesh_uv_channel_properties_get_uv_channel_names_func: std::ptr::null_mut(),
            uuv_projection_tool_edit_actions_reset: std::ptr::null_mut(),
            uuv_projection_tool_edit_actions_auto_fit_align: std::ptr::null_mut(),
            uuv_projection_tool_edit_actions_auto_fit: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshConstraintProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPreventTinyTrianglesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_constraint_properties_is_prevent_tiny_triangles_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPreventNormalFlipsEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_constraint_properties_is_prevent_normal_flips_enabled,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UConvertToPolygonsToolProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGroupOptionsList"),
                &raw mut __FUNCTION_PTRS
                    .u_convert_to_polygons_tool_properties_get_group_options_list,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDisplaceMeshCommonProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWeightMapsFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_displace_mesh_common_properties_get_weight_maps_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USelectiveTessellationProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaterialIDsFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_selective_tessellation_properties_get_material_i_ds_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDynamicSculptToolActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DiscardAttributes"),
                &raw mut __FUNCTION_PTRS.u_dynamic_sculpt_tool_actions_discard_attributes,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPolyEditTopologyProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegenerateExtraCorners"),
                &raw mut __FUNCTION_PTRS
                    .u_poly_edit_topology_properties_regenerate_extra_corners,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditMeshPolygonsToolActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SimplifyByGroups"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_simplify_by_groups,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Retriangulate"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_retriangulate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RecalcNormals"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_recalc_normals,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PushPull"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_push_pull,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Outset"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_outset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Offset"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Merge"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_merge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Inset"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_inset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InsertEdgeLoop"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_insert_edge_loop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InsertEdge"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_insert_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Flip"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_flip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Extrude"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_extrude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Duplicate"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_duplicate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Disconnect"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_disconnect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Delete"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_delete,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Decompose"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_decompose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CutFaces"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_cut_faces,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Bevel"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_bevel,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditMeshPolygonsToolActions_Triangles::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RecalcNormals"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_recalc_normals,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PushPull"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_push_pull,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Poke"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_triangles_poke,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Outset"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_outset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Offset"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Inset"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_inset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Flip"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_actions_triangles_flip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Extrude"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_extrude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Duplicate"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_duplicate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Disconnect"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_disconnect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Delete"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_delete,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CutFaces"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_actions_triangles_cut_faces,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditMeshPolygonsToolUVActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PlanarProjection"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_uv_actions_planar_projection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditMeshPolygonsToolEdgeActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WeldCentered"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_weld_centered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Weld"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_weld,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Straighten"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_straighten,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Simplify"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_simplify,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FillHole"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_fill_hole,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Extrude"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_extrude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteEdge"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_delete_edge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Collapse"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_collapse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Bridge"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_bridge,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Bevel"),
                &raw mut __FUNCTION_PTRS.u_edit_mesh_polygons_tool_edge_actions_bevel,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEditMeshPolygonsToolEdgeActions_Triangles::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WeldCentered"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_weld_centered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Weld"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_weld,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Split"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_split,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Flip"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_flip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FillHole"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_fill_hole,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Extrude"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_extrude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Collapse"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_collapse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Bridge"),
                &raw mut __FUNCTION_PTRS
                    .u_edit_mesh_polygons_tool_edge_actions_triangles_bridge,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UHoleFillToolActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectAll"),
                &raw mut __FUNCTION_PTRS.u_hole_fill_tool_actions_select_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clear"),
                &raw mut __FUNCTION_PTRS.u_hole_fill_tool_actions_clear,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULatticeDeformerToolProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Constrain"),
                &raw mut __FUNCTION_PTRS.u_lattice_deformer_tool_properties_constrain,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearConstraints"),
                &raw mut __FUNCTION_PTRS
                    .u_lattice_deformer_tool_properties_clear_constraints,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshAttributePaintToolProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAttributeNames"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_attribute_paint_tool_properties_get_attribute_names,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshGroupPaintToolFreezeActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnfreezeAll"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_unfreeze_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShrinkCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_shrink_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GrowCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_grow_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FreezeOthers"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_freeze_others,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FreezeCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_freeze_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FloodFillCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_flood_fill_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_group_paint_tool_freeze_actions_clear_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAll"),
                &raw mut __FUNCTION_PTRS.u_mesh_group_paint_tool_freeze_actions_clear_all,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshSpaceDeformerToolActionPropertySet::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShiftToCenter"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_space_deformer_tool_action_property_set_shift_to_center,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshVertexPaintToolQuickActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PaintAll"),
                &raw mut __FUNCTION_PTRS.u_mesh_vertex_paint_tool_quick_actions_paint_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FillWhite"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_vertex_paint_tool_quick_actions_fill_white,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FillBlack"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_vertex_paint_tool_quick_actions_fill_black,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EraseAll"),
                &raw mut __FUNCTION_PTRS.u_mesh_vertex_paint_tool_quick_actions_erase_all,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshVertexPaintToolUtilityActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWeightMapsFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_vertex_paint_tool_utility_actions_get_weight_maps_func,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLODNamesFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_vertex_paint_tool_utility_actions_get_lod_names_func,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplySelectedOperation"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_vertex_paint_tool_utility_actions_apply_selected_operation,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UExistingMeshMaterialProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUVChannelNamesFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_existing_mesh_material_properties_get_uv_channel_names_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshSculptLayerProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLayerWeight"),
                &raw mut __FUNCTION_PTRS.u_mesh_sculpt_layer_properties_set_layer_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLayerName"),
                &raw mut __FUNCTION_PTRS.u_mesh_sculpt_layer_properties_set_layer_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActiveLayer"),
                &raw mut __FUNCTION_PTRS.u_mesh_sculpt_layer_properties_set_active_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLayerAtIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_sculpt_layer_properties_remove_layer_at_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveLayer"),
                &raw mut __FUNCTION_PTRS.u_mesh_sculpt_layer_properties_move_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLayerName"),
                &raw mut __FUNCTION_PTRS.u_mesh_sculpt_layer_properties_get_layer_name,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshUVChannelProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUVChannelNamesFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_uv_channel_properties_get_uv_channel_names_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UUVProjectionToolEditActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reset"),
                &raw mut __FUNCTION_PTRS.uuv_projection_tool_edit_actions_reset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AutoFitAlign"),
                &raw mut __FUNCTION_PTRS.uuv_projection_tool_edit_actions_auto_fit_align,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AutoFit"),
                &raw mut __FUNCTION_PTRS.uuv_projection_tool_edit_actions_auto_fit,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UMeshSculptBrushOpProps {
    __padding_end: [u8; 192],
}
impl UMeshSculptBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptBrushOpProps")
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
pub struct UBaseKelvinletBrushOpProps {
    __padding_end: [u8; 208],
}
impl UBaseKelvinletBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseKelvinletBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseKelvinletBrushOpProps")
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
pub struct UScaleKelvinletBrushOpProps {
    __padding_end: [u8; 216],
}
impl UScaleKelvinletBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleKelvinletBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UScaleKelvinletBrushOpProps")
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
pub struct UPullKelvinletBrushOpProps {
    __padding_end: [u8; 216],
}
impl UPullKelvinletBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPullKelvinletBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPullKelvinletBrushOpProps")
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
pub struct USharpPullKelvinletBrushOpProps {
    __padding_end: [u8; 216],
}
impl USharpPullKelvinletBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharpPullKelvinletBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharpPullKelvinletBrushOpProps")
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
pub struct UTwistKelvinletBrushOpProps {
    __padding_end: [u8; 216],
}
impl UTwistKelvinletBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTwistKelvinletBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTwistKelvinletBrushOpProps")
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
pub struct UEraseSculptLayerBrushOpProps {
    __padding_end: [u8; 200],
}
impl UEraseSculptLayerBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEraseSculptLayerBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEraseSculptLayerBrushOpProps")
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
#[repr(C, align(16))]
pub struct UGroupEraseBrushOpProps {
    __padding_end: [u8; 256],
}
impl UGroupEraseBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupEraseBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupEraseBrushOpProps")
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
pub struct UGroupPaintBrushOpProps {
    __padding_end: [u8; 200],
}
impl UGroupPaintBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupPaintBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupPaintBrushOpProps")
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
pub struct UInflateBrushOpProps {
    __padding_end: [u8; 200],
}
impl UInflateBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInflateBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInflateBrushOpProps")
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
pub struct UMoveBrushOpProps {
    __padding_end: [u8; 208],
}
impl UMoveBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoveBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoveBrushOpProps")
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
pub struct UPinchBrushOpProps {
    __padding_end: [u8; 208],
}
impl UPinchBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPinchBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPinchBrushOpProps")
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
pub struct UBasePlaneBrushOpProps {
    __padding_end: [u8; 192],
}
impl UBasePlaneBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBasePlaneBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBasePlaneBrushOpProps")
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
pub struct UPlaneBrushOpProps {
    __padding_end: [u8; 208],
}
impl UPlaneBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneBrushOpProps")
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
pub struct UViewAlignedPlaneBrushOpProps {
    __padding_end: [u8; 208],
}
impl UViewAlignedPlaneBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewAlignedPlaneBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewAlignedPlaneBrushOpProps")
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
pub struct UFixedPlaneBrushOpProps {
    __padding_end: [u8; 208],
}
impl UFixedPlaneBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedPlaneBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedPlaneBrushOpProps")
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
pub struct UStandardSculptBrushOpProps {
    __padding_end: [u8; 200],
}
impl UStandardSculptBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStandardSculptBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStandardSculptBrushOpProps")
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
pub struct UViewAlignedSculptBrushOpProps {
    __padding_end: [u8; 200],
}
impl UViewAlignedSculptBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewAlignedSculptBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewAlignedSculptBrushOpProps")
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
pub struct USculptMaxBrushOpProps {
    __padding_end: [u8; 216],
}
impl USculptMaxBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptMaxBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptMaxBrushOpProps")
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
pub struct UBaseSmoothBrushOpProps {
    __padding_end: [u8; 192],
}
impl UBaseSmoothBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseSmoothBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseSmoothBrushOpProps")
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
pub struct USmoothBrushOpProps {
    __padding_end: [u8; 208],
}
impl USmoothBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothBrushOpProps")
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
pub struct USecondarySmoothBrushOpProps {
    __padding_end: [u8; 208],
}
impl USecondarySmoothBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USecondarySmoothBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USecondarySmoothBrushOpProps")
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
pub struct USmoothFillBrushOpProps {
    __padding_end: [u8; 208],
}
impl USmoothFillBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothFillBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothFillBrushOpProps")
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
pub struct UFlattenBrushOpProps {
    __padding_end: [u8; 208],
}
impl UFlattenBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFlattenBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFlattenBrushOpProps")
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
pub struct UEraseBrushOpProps {
    __padding_end: [u8; 200],
}
impl UEraseBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEraseBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEraseBrushOpProps")
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
pub struct UVertexColorBaseBrushOpProps {
    __padding_end: [u8; 208],
}
impl UVertexColorBaseBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorBaseBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorBaseBrushOpProps")
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
pub struct UVertexColorPaintBrushOpProps {
    __padding_end: [u8; 224],
}
impl UVertexColorPaintBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorPaintBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorPaintBrushOpProps")
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
pub struct UVertexColorSoftenBrushOpProps {
    __padding_end: [u8; 208],
}
impl UVertexColorSoftenBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorSoftenBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorSoftenBrushOpProps")
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
pub struct UVertexColorSmoothBrushOpProps {
    __padding_end: [u8; 208],
}
impl UVertexColorSmoothBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorSmoothBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexColorSmoothBrushOpProps")
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
#[repr(C, align(16))]
pub struct UPolyEditActivityContext {
    __padding_end: [u8; 256],
}
impl UPolyEditActivityContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditActivityContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditActivityContext")
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
pub struct UMeshConstraintProperties {
    __padding_end: [u8; 192],
}
impl UMeshConstraintProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshConstraintProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshConstraintProperties")
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
pub struct URemeshProperties {
    __padding_end: [u8; 200],
}
impl URemeshProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshProperties")
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
pub struct UAddPrimitiveToolBuilder {
    __padding_end: [u8; 56],
}
impl UAddPrimitiveToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPrimitiveToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPrimitiveToolBuilder")
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
pub struct UProceduralShapeToolProperties {
    __padding_end: [u8; 200],
}
impl UProceduralShapeToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralShapeToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralShapeToolProperties")
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
pub struct UProceduralBoxToolProperties {
    __padding_end: [u8; 224],
}
impl UProceduralBoxToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralBoxToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralBoxToolProperties")
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
pub struct UProceduralRectangleToolProperties {
    __padding_end: [u8; 232],
}
impl UProceduralRectangleToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralRectangleToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralRectangleToolProperties")
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
pub struct UProceduralDiscToolProperties {
    __padding_end: [u8; 224],
}
impl UProceduralDiscToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralDiscToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralDiscToolProperties")
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
pub struct UProceduralTorusToolProperties {
    __padding_end: [u8; 216],
}
impl UProceduralTorusToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralTorusToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralTorusToolProperties")
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
pub struct UProceduralCylinderToolProperties {
    __padding_end: [u8; 216],
}
impl UProceduralCylinderToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralCylinderToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralCylinderToolProperties")
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
pub struct UProceduralConeToolProperties {
    __padding_end: [u8; 216],
}
impl UProceduralConeToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralConeToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralConeToolProperties")
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
pub struct UProceduralArrowToolProperties {
    __padding_end: [u8; 224],
}
impl UProceduralArrowToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralArrowToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralArrowToolProperties")
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
pub struct UProceduralSphereToolProperties {
    __padding_end: [u8; 224],
}
impl UProceduralSphereToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralSphereToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralSphereToolProperties")
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
pub struct UProceduralCapsuleToolProperties {
    __padding_end: [u8; 224],
}
impl UProceduralCapsuleToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralCapsuleToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralCapsuleToolProperties")
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
pub struct UProceduralStairsToolProperties {
    __padding_end: [u8; 232],
}
impl UProceduralStairsToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralStairsToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralStairsToolProperties")
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
pub struct UAddPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPrimitiveTool")
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
pub struct UAddBoxPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddBoxPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddBoxPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddBoxPrimitiveTool")
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
pub struct UAddCylinderPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddCylinderPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddCylinderPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddCylinderPrimitiveTool")
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
pub struct UAddCapsulePrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddCapsulePrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddCapsulePrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddCapsulePrimitiveTool")
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
pub struct UAddConePrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddConePrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddConePrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddConePrimitiveTool")
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
pub struct UAddRectanglePrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddRectanglePrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddRectanglePrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddRectanglePrimitiveTool")
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
pub struct UAddDiscPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddDiscPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddDiscPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddDiscPrimitiveTool")
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
pub struct UAddTorusPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddTorusPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddTorusPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddTorusPrimitiveTool")
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
pub struct UAddArrowPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddArrowPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddArrowPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddArrowPrimitiveTool")
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
pub struct UAddSpherePrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddSpherePrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddSpherePrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddSpherePrimitiveTool")
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
pub struct UAddStairsPrimitiveTool {
    __padding_end: [u8; 360],
}
impl UAddStairsPrimitiveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddStairsPrimitiveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddStairsPrimitiveTool")
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
pub struct UCombineMeshesToolBuilder {
    __padding_end: [u8; 56],
}
impl UCombineMeshesToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesToolBuilder")
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
pub struct UCombineMeshesToolProperties {
    __padding_end: [u8; 224],
}
impl UCombineMeshesToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesToolProperties")
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
pub struct UCombineMeshesTool {
    __padding_end: [u8; 272],
}
impl UCombineMeshesTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombineMeshesTool")
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
pub struct UDeleteGeometrySelectionCommand {
    __padding_end: [u8; 48],
}
impl UDeleteGeometrySelectionCommand {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeleteGeometrySelectionCommand")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeleteGeometrySelectionCommand")
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
pub struct UDisconnectGeometrySelectionCommand {
    __padding_end: [u8; 48],
}
impl UDisconnectGeometrySelectionCommand {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisconnectGeometrySelectionCommand")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisconnectGeometrySelectionCommand")
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
pub struct UModifyGeometrySelectionCommand {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand")
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
pub struct UModifyGeometrySelectionCommand_Invert {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand_Invert {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Invert")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Invert")
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
pub struct UModifyGeometrySelectionCommand_ExpandToConnected {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand_ExpandToConnected {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_ExpandToConnected")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_ExpandToConnected")
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
pub struct UModifyGeometrySelectionCommand_InvertConnected {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand_InvertConnected {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_InvertConnected")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_InvertConnected")
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
pub struct UModifyGeometrySelectionCommand_Expand {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand_Expand {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Expand")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Expand")
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
pub struct UModifyGeometrySelectionCommand_Contract {
    __padding_end: [u8; 48],
}
impl UModifyGeometrySelectionCommand_Contract {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Contract")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModifyGeometrySelectionCommand_Contract")
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
pub struct URetriangulateGeometrySelectionCommand {
    __padding_end: [u8; 48],
}
impl URetriangulateGeometrySelectionCommand {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetriangulateGeometrySelectionCommand")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URetriangulateGeometrySelectionCommand")
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
pub struct UConvertToPolygonsToolBuilder {
    __padding_end: [u8; 48],
}
impl UConvertToPolygonsToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsToolBuilder")
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
pub struct UConvertToPolygonsToolProperties {
    __padding_end: [u8; 288],
}
impl UConvertToPolygonsToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsToolProperties")
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
pub struct UConvertToPolygonsOperatorFactory {
    __padding_end: [u8; 64],
}
impl UConvertToPolygonsOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsOperatorFactory")
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
pub struct UConvertToPolygonsTool {
    __padding_end: [u8; 480],
}
impl UConvertToPolygonsTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertToPolygonsTool")
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
pub struct UCSGMeshesToolProperties {
    __padding_end: [u8; 224],
}
impl UCSGMeshesToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesToolProperties")
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
pub struct UTrimMeshesToolProperties {
    __padding_end: [u8; 216],
}
impl UTrimMeshesToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTrimMeshesToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTrimMeshesToolProperties")
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
pub struct UCSGMeshesTool {
    __padding_end: [u8; 400],
}
impl UCSGMeshesTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesTool")
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
pub struct UCSGMeshesToolBuilder {
    __padding_end: [u8; 56],
}
impl UCSGMeshesToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSGMeshesToolBuilder")
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
pub struct UCutMeshWithMeshToolProperties {
    __padding_end: [u8; 200],
}
impl UCutMeshWithMeshToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshToolProperties")
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
pub struct UCutMeshWithMeshTool {
    __padding_end: [u8; 960],
}
impl UCutMeshWithMeshTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshTool")
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
pub struct UCutMeshWithMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UCutMeshWithMeshToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCutMeshWithMeshToolBuilder")
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
pub struct UDeformMeshPolygonsToolBuilder {
    __padding_end: [u8; 56],
}
impl UDeformMeshPolygonsToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsToolBuilder")
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
pub struct UDeformMeshPolygonsTransformProperties {
    __padding_end: [u8; 216],
}
impl UDeformMeshPolygonsTransformProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsTransformProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsTransformProperties")
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
#[repr(C, align(16))]
pub struct UDeformMeshPolygonsTool {
    __padding_end: [u8; 6208],
}
impl UDeformMeshPolygonsTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDeformMeshPolygonsTool")
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
pub struct UDisplaceMeshCommonProperties {
    __padding_end: [u8; 240],
}
impl UDisplaceMeshCommonProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshCommonProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshCommonProperties")
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
pub struct USelectiveTessellationProperties {
    __padding_end: [u8; 216],
}
impl USelectiveTessellationProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelectiveTessellationProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelectiveTessellationProperties")
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
pub struct UDisplaceMeshTextureMapProperties {
    __padding_end: [u8; 256],
}
impl UDisplaceMeshTextureMapProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshTextureMapProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshTextureMapProperties")
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
pub struct UDisplaceMeshDirectionalFilterProperties {
    __padding_end: [u8; 224],
}
impl UDisplaceMeshDirectionalFilterProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshDirectionalFilterProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshDirectionalFilterProperties")
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
pub struct UDisplaceMeshPerlinNoiseProperties {
    __padding_end: [u8; 200],
}
impl UDisplaceMeshPerlinNoiseProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshPerlinNoiseProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshPerlinNoiseProperties")
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
pub struct UDisplaceMeshSineWaveProperties {
    __padding_end: [u8; 216],
}
impl UDisplaceMeshSineWaveProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshSineWaveProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshSineWaveProperties")
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
pub struct UDisplaceMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UDisplaceMeshToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshToolBuilder")
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
#[repr(C, align(16))]
pub struct UDisplaceMeshTool {
    __padding_end: [u8; 1296],
}
impl UDisplaceMeshTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDisplaceMeshTool")
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
pub struct UDrawAndRevolveToolBuilder {
    __padding_end: [u8; 48],
}
impl UDrawAndRevolveToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawAndRevolveToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawAndRevolveToolBuilder")
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
pub struct URevolveProperties {
    __padding_end: [u8; 296],
}
impl URevolveProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveProperties")
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
pub struct URevolveToolProperties {
    __padding_end: [u8; 360],
}
impl URevolveToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveToolProperties")
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
pub struct URevolveOperatorFactory {
    __padding_end: [u8; 64],
}
impl URevolveOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveOperatorFactory")
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
#[repr(C, align(16))]
pub struct UDrawAndRevolveTool {
    __padding_end: [u8; 464],
}
impl UDrawAndRevolveTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawAndRevolveTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawAndRevolveTool")
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
pub struct UDrawPolygonToolBuilder {
    __padding_end: [u8; 48],
}
impl UDrawPolygonToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolBuilder")
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
pub struct UDrawPolygonToolStandardProperties {
    __padding_end: [u8; 208],
}
impl UDrawPolygonToolStandardProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolStandardProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolStandardProperties")
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
pub struct UDrawPolygonToolSnapProperties {
    __padding_end: [u8; 200],
}
impl UDrawPolygonToolSnapProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolSnapProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonToolSnapProperties")
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
#[repr(C, align(16))]
pub struct UDrawPolygonTool {
    __padding_end: [u8; 1488],
}
impl UDrawPolygonTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolygonTool")
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
pub struct UDynamicMeshBrushTool {
    __padding_end: [u8; 992],
}
impl UDynamicMeshBrushTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushTool")
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
pub struct UDynamicMeshSculptToolBuilder {
    __padding_end: [u8; 64],
}
impl UDynamicMeshSculptToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshSculptToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshSculptToolBuilder")
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
pub struct UDynamicMeshBrushProperties {
    __padding_end: [u8; 224],
}
impl UDynamicMeshBrushProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushProperties")
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
pub struct UDynamicMeshBrushSculptProperties {
    __padding_end: [u8; 208],
}
impl UDynamicMeshBrushSculptProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushSculptProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshBrushSculptProperties")
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
pub struct UDynamicSculptToolActions {
    __padding_end: [u8; 192],
}
impl UDynamicSculptToolActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicSculptToolActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicSculptToolActions")
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
pub struct UBrushRemeshProperties {
    __padding_end: [u8; 216],
}
impl UBrushRemeshProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrushRemeshProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrushRemeshProperties")
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
#[repr(C, align(16))]
pub struct UFixedPlaneBrushProperties {
    __padding_end: [u8; 256],
}
impl UFixedPlaneBrushProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedPlaneBrushProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedPlaneBrushProperties")
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
#[repr(C, align(16))]
pub struct UDynamicMeshSculptTool {
    __padding_end: [u8; 3808],
}
impl UDynamicMeshSculptTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshSculptTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshSculptTool")
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
pub struct UEditMeshPolygonsToolBuilder {
    __padding_end: [u8; 56],
}
impl UEditMeshPolygonsToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolBuilder")
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
pub struct UPolyEditCommonProperties {
    __padding_end: [u8; 200],
}
impl UPolyEditCommonProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCommonProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCommonProperties")
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
pub struct UEditMeshPolygonsActionModeToolBuilder {
    __padding_end: [u8; 64],
}
impl UEditMeshPolygonsActionModeToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsActionModeToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsActionModeToolBuilder")
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
pub struct UEditMeshPolygonsSelectionModeToolBuilder {
    __padding_end: [u8; 64],
}
impl UEditMeshPolygonsSelectionModeToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsSelectionModeToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsSelectionModeToolBuilder")
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
pub struct UEditMeshPolygonsToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolActionPropertySet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActionPropertySet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActionPropertySet")
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
pub struct UPolyEditTopologyProperties {
    __padding_end: [u8; 208],
}
impl UPolyEditTopologyProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditTopologyProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditTopologyProperties")
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
pub struct UEditMeshPolygonsToolActions {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActions")
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
pub struct UEditMeshPolygonsToolActions_Triangles {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolActions_Triangles {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActions_Triangles")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolActions_Triangles")
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
pub struct UEditMeshPolygonsToolUVActions {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolUVActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolUVActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolUVActions")
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
pub struct UEditMeshPolygonsToolEdgeActions {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolEdgeActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolEdgeActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolEdgeActions")
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
pub struct UEditMeshPolygonsToolEdgeActions_Triangles {
    __padding_end: [u8; 192],
}
impl UEditMeshPolygonsToolEdgeActions_Triangles {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolEdgeActions_Triangles")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsToolEdgeActions_Triangles")
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
#[repr(C, align(16))]
pub struct UEditMeshPolygonsTool {
    __padding_end: [u8; 2336],
}
impl UEditMeshPolygonsTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshPolygonsTool")
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
pub struct UHoleFillToolBuilder {
    __padding_end: [u8; 48],
}
impl UHoleFillToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolBuilder")
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
pub struct USmoothHoleFillProperties {
    __padding_end: [u8; 224],
}
impl USmoothHoleFillProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothHoleFillProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothHoleFillProperties")
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
pub struct UHoleFillToolProperties {
    __padding_end: [u8; 192],
}
impl UHoleFillToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolProperties")
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
pub struct UHoleFillToolActions {
    __padding_end: [u8; 192],
}
impl UHoleFillToolActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillToolActions")
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
pub struct UHoleFillStatisticsProperties {
    __padding_end: [u8; 264],
}
impl UHoleFillStatisticsProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillStatisticsProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillStatisticsProperties")
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
pub struct UHoleFillOperatorFactory {
    __padding_end: [u8; 64],
}
impl UHoleFillOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillOperatorFactory")
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
#[repr(C, align(16))]
pub struct UHoleFillTool {
    __padding_end: [u8; 608],
}
impl UHoleFillTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHoleFillTool")
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
pub struct ULatticeDeformerToolBuilder {
    __padding_end: [u8; 48],
}
impl ULatticeDeformerToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerToolBuilder")
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
pub struct ULatticeDeformerToolProperties {
    __padding_end: [u8; 216],
}
impl ULatticeDeformerToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerToolProperties")
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
pub struct ULatticeDeformerOperatorFactory {
    __padding_end: [u8; 64],
}
impl ULatticeDeformerOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerOperatorFactory")
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
#[repr(C, align(16))]
pub struct ULatticeDeformerTool {
    __padding_end: [u8; 608],
}
impl ULatticeDeformerTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeDeformerTool")
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
#[repr(C, align(16))]
pub struct UMeshAttributePaintToolBuilder {
    __padding_end: [u8; 112],
}
impl UMeshAttributePaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolBuilder")
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
pub struct UMeshAttributePaintBrushOperationProperties {
    __padding_end: [u8; 200],
}
impl UMeshAttributePaintBrushOperationProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintBrushOperationProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintBrushOperationProperties")
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
pub struct UMeshAttributePaintToolVisualizationProperties {
    __padding_end: [u8; 192],
}
impl UMeshAttributePaintToolVisualizationProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolVisualizationProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolVisualizationProperties")
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
pub struct UMeshAttributePaintToolProperties {
    __padding_end: [u8; 216],
}
impl UMeshAttributePaintToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintToolProperties")
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
pub struct UMeshAttributePaintEditActions {
    __padding_end: [u8; 192],
}
impl UMeshAttributePaintEditActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintEditActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintEditActions")
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
#[repr(C, align(16))]
pub struct UMeshAttributePaintTool {
    __padding_end: [u8; 2592],
}
impl UMeshAttributePaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAttributePaintTool")
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
pub struct UMeshGroupPaintToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshGroupPaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolBuilder")
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
pub struct UGroupPaintBrushFilterProperties {
    __padding_end: [u8; 232],
}
impl UGroupPaintBrushFilterProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupPaintBrushFilterProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupPaintBrushFilterProperties")
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
pub struct UMeshGroupPaintToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UMeshGroupPaintToolActionPropertySet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolActionPropertySet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolActionPropertySet")
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
pub struct UMeshGroupPaintToolFreezeActions {
    __padding_end: [u8; 192],
}
impl UMeshGroupPaintToolFreezeActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolFreezeActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintToolFreezeActions")
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
#[repr(C, align(16))]
pub struct UMeshSculptToolBase {
    __padding_end: [u8; 3216],
}
impl UMeshSculptToolBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptToolBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptToolBase")
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
#[repr(C, align(16))]
pub struct UMeshGroupPaintTool {
    __padding_end: [u8; 4304],
}
impl UMeshGroupPaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshGroupPaintTool")
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
pub struct UMeshSpaceDeformerToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshSpaceDeformerToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolBuilder")
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
pub struct UMeshSpaceDeformerToolProperties {
    __padding_end: [u8; 224],
}
impl UMeshSpaceDeformerToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolProperties")
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
pub struct UMeshSpaceDeformerToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UMeshSpaceDeformerToolActionPropertySet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolActionPropertySet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerToolActionPropertySet")
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
pub struct USpaceDeformerOperatorFactory {
    __padding_end: [u8; 64],
}
impl USpaceDeformerOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceDeformerOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceDeformerOperatorFactory")
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
#[repr(C, align(16))]
pub struct UMeshSpaceDeformerTool {
    __padding_end: [u8; 768],
}
impl UMeshSpaceDeformerTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSpaceDeformerTool")
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
pub struct UMeshVertexPaintToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshVertexPaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolBuilder")
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
pub struct UVertexPaintBasicProperties {
    __padding_end: [u8; 256],
}
impl UVertexPaintBasicProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexPaintBasicProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexPaintBasicProperties")
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
pub struct UVertexPaintBrushFilterProperties {
    __padding_end: [u8; 208],
}
impl UVertexPaintBrushFilterProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexPaintBrushFilterProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexPaintBrushFilterProperties")
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
pub struct UMeshVertexPaintToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UMeshVertexPaintToolActionPropertySet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolActionPropertySet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolActionPropertySet")
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
pub struct UMeshVertexPaintToolQuickActions {
    __padding_end: [u8; 192],
}
impl UMeshVertexPaintToolQuickActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolQuickActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolQuickActions")
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
pub struct UMeshVertexPaintToolUtilityActions {
    __padding_end: [u8; 272],
}
impl UMeshVertexPaintToolUtilityActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolUtilityActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintToolUtilityActions")
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
#[repr(C, align(16))]
pub struct UMeshVertexPaintTool {
    __padding_end: [u8; 4944],
}
impl UMeshVertexPaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexPaintTool")
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
pub struct UMeshVertexSculptToolBuilder {
    __padding_end: [u8; 64],
}
impl UMeshVertexSculptToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexSculptToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexSculptToolBuilder")
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
pub struct UVertexBrushSculptProperties {
    __padding_end: [u8; 208],
}
impl UVertexBrushSculptProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexBrushSculptProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexBrushSculptProperties")
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
pub struct UVertexBrushAlphaProperties {
    __padding_end: [u8; 216],
}
impl UVertexBrushAlphaProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexBrushAlphaProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVertexBrushAlphaProperties")
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
pub struct UMeshSymmetryProperties {
    __padding_end: [u8; 192],
}
impl UMeshSymmetryProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSymmetryProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSymmetryProperties")
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
#[repr(C, align(16))]
pub struct UMeshVertexSculptTool {
    __padding_end: [u8; 6128],
}
impl UMeshVertexSculptTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexSculptTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshVertexSculptTool")
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
pub struct UOffsetMeshToolProperties {
    __padding_end: [u8; 200],
}
impl UOffsetMeshToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshToolProperties")
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
pub struct UOffsetWeightMapSetProperties {
    __padding_end: [u8; 232],
}
impl UOffsetWeightMapSetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetWeightMapSetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetWeightMapSetProperties")
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
pub struct UIterativeOffsetProperties {
    __padding_end: [u8; 200],
}
impl UIterativeOffsetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIterativeOffsetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIterativeOffsetProperties")
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
pub struct UImplicitOffsetProperties {
    __padding_end: [u8; 192],
}
impl UImplicitOffsetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImplicitOffsetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImplicitOffsetProperties")
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
#[repr(C, align(16))]
pub struct UOffsetMeshTool {
    __padding_end: [u8; 1216],
}
impl UOffsetMeshTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshTool")
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
pub struct UOffsetMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UOffsetMeshToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshToolBuilder")
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
pub struct UProjectToTargetToolBuilder {
    __padding_end: [u8; 48],
}
impl UProjectToTargetToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetToolBuilder")
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
pub struct URemeshMeshToolProperties {
    __padding_end: [u8; 240],
}
impl URemeshMeshToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshToolProperties")
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
pub struct UProjectToTargetToolProperties {
    __padding_end: [u8; 272],
}
impl UProjectToTargetToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetToolProperties")
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
pub struct URemeshMeshTool {
    __padding_end: [u8; 312],
}
impl URemeshMeshTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshTool")
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
pub struct UProjectToTargetTool {
    __padding_end: [u8; 328],
}
impl UProjectToTargetTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectToTargetTool")
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
pub struct UNewMeshMaterialProperties {
    __padding_end: [u8; 200],
}
impl UNewMeshMaterialProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNewMeshMaterialProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNewMeshMaterialProperties")
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
pub struct UExistingMeshMaterialProperties {
    __padding_end: [u8; 240],
}
impl UExistingMeshMaterialProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExistingMeshMaterialProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExistingMeshMaterialProperties")
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
pub struct UMeshEditingViewProperties {
    __padding_end: [u8; 264],
}
impl UMeshEditingViewProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshEditingViewProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshEditingViewProperties")
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
pub struct UMeshSculptLayerProperties {
    __padding_end: [u8; 112],
}
impl UMeshSculptLayerProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptLayerProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptLayerProperties")
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
pub struct UMeshStatisticsProperties {
    __padding_end: [u8; 232],
}
impl UMeshStatisticsProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshStatisticsProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshStatisticsProperties")
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
pub struct UMeshUVChannelProperties {
    __padding_end: [u8; 216],
}
impl UMeshUVChannelProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshUVChannelProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshUVChannelProperties")
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
pub struct URecomputeUVsToolBuilder {
    __padding_end: [u8; 48],
}
impl URecomputeUVsToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecomputeUVsToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecomputeUVsToolBuilder")
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
pub struct URecomputeUVsTool {
    __padding_end: [u8; 328],
}
impl URecomputeUVsTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecomputeUVsTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URecomputeUVsTool")
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
pub struct URemeshMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl URemeshMeshToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemeshMeshToolBuilder")
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
pub struct URemoveOccludedTrianglesToolBuilder {
    __padding_end: [u8; 48],
}
impl URemoveOccludedTrianglesToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesToolBuilder")
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
pub struct URemoveOccludedTrianglesToolProperties {
    __padding_end: [u8; 232],
}
impl URemoveOccludedTrianglesToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesToolProperties")
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
pub struct URemoveOccludedTrianglesAdvancedProperties {
    __padding_end: [u8; 192],
}
impl URemoveOccludedTrianglesAdvancedProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesAdvancedProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesAdvancedProperties")
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
pub struct URemoveOccludedTrianglesOperatorFactory {
    __padding_end: [u8; 72],
}
impl URemoveOccludedTrianglesOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesOperatorFactory")
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
#[repr(C, align(16))]
pub struct URemoveOccludedTrianglesTool {
    __padding_end: [u8; 528],
}
impl URemoveOccludedTrianglesTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOccludedTrianglesTool")
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
pub struct USculptBrushProperties {
    __padding_end: [u8; 248],
}
impl USculptBrushProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptBrushProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptBrushProperties")
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
pub struct UKelvinBrushProperties {
    __padding_end: [u8; 200],
}
impl UKelvinBrushProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKelvinBrushProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UKelvinBrushProperties")
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
#[repr(C, align(16))]
pub struct UWorkPlaneProperties {
    __padding_end: [u8; 256],
}
impl UWorkPlaneProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorkPlaneProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorkPlaneProperties")
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
pub struct USculptMaxBrushProperties {
    __padding_end: [u8; 192],
}
impl USculptMaxBrushProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptMaxBrushProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USculptMaxBrushProperties")
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
pub struct USmoothMeshToolProperties {
    __padding_end: [u8; 192],
}
impl USmoothMeshToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshToolProperties")
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
pub struct UIterativeSmoothProperties {
    __padding_end: [u8; 200],
}
impl UIterativeSmoothProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIterativeSmoothProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIterativeSmoothProperties")
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
pub struct UDiffusionSmoothProperties {
    __padding_end: [u8; 200],
}
impl UDiffusionSmoothProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffusionSmoothProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffusionSmoothProperties")
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
pub struct UImplicitSmoothProperties {
    __padding_end: [u8; 200],
}
impl UImplicitSmoothProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImplicitSmoothProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImplicitSmoothProperties")
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
pub struct USmoothWeightMapSetProperties {
    __padding_end: [u8; 232],
}
impl USmoothWeightMapSetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothWeightMapSetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothWeightMapSetProperties")
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
#[repr(C, align(16))]
pub struct USmoothMeshTool {
    __padding_end: [u8; 1232],
}
impl USmoothMeshTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshTool")
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
pub struct USmoothMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl USmoothMeshToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothMeshToolBuilder")
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
pub struct UPolyEditBevelEdgeProperties {
    __padding_end: [u8; 208],
}
impl UPolyEditBevelEdgeProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditBevelEdgeProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditBevelEdgeProperties")
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
pub struct UPolyEditBevelEdgeActivity {
    __padding_end: [u8; 328],
}
impl UPolyEditBevelEdgeActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditBevelEdgeActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditBevelEdgeActivity")
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
pub struct UPolyEditCutProperties {
    __padding_end: [u8; 192],
}
impl UPolyEditCutProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCutProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCutProperties")
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
#[repr(C, align(16))]
pub struct UPolyEditCutFacesActivity {
    __padding_end: [u8; 208],
}
impl UPolyEditCutFacesActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCutFacesActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditCutFacesActivity")
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
pub struct UPolyEditExtrudeProperties {
    __padding_end: [u8; 232],
}
impl UPolyEditExtrudeProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeProperties")
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
pub struct UPolyEditOffsetProperties {
    __padding_end: [u8; 232],
}
impl UPolyEditOffsetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditOffsetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditOffsetProperties")
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
pub struct UPolyEditPushPullProperties {
    __padding_end: [u8; 232],
}
impl UPolyEditPushPullProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPushPullProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPushPullProperties")
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
pub struct UPolyEditExtrudeActivity {
    __padding_end: [u8; 472],
}
impl UPolyEditExtrudeActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeActivity")
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
pub struct UPolyEditExtrudeEdgeActivityProperties {
    __padding_end: [u8; 208],
}
impl UPolyEditExtrudeEdgeActivityProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeEdgeActivityProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeEdgeActivityProperties")
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
#[repr(C, align(16))]
pub struct UPolyEditExtrudeEdgeActivity {
    __padding_end: [u8; 752],
}
impl UPolyEditExtrudeEdgeActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeEdgeActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditExtrudeEdgeActivity")
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
pub struct UGroupEdgeInsertionProperties {
    __padding_end: [u8; 200],
}
impl UGroupEdgeInsertionProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupEdgeInsertionProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupEdgeInsertionProperties")
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
#[repr(C, align(16))]
pub struct UPolyEditInsertEdgeActivity {
    __padding_end: [u8; 1200],
}
impl UPolyEditInsertEdgeActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsertEdgeActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsertEdgeActivity")
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
pub struct UEdgeLoopInsertionProperties {
    __padding_end: [u8; 232],
}
impl UEdgeLoopInsertionProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdgeLoopInsertionProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdgeLoopInsertionProperties")
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
#[repr(C, align(16))]
pub struct UPolyEditInsertEdgeLoopActivity {
    __padding_end: [u8; 1088],
}
impl UPolyEditInsertEdgeLoopActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsertEdgeLoopActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsertEdgeLoopActivity")
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
pub struct UPolyEditInsetOutsetProperties {
    __padding_end: [u8; 200],
}
impl UPolyEditInsetOutsetProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsetOutsetProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsetOutsetProperties")
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
pub struct UPolyEditInsetOutsetActivity {
    __padding_end: [u8; 120],
}
impl UPolyEditInsetOutsetActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsetOutsetActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditInsetOutsetActivity")
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
pub struct UPolyEditSetUVProperties {
    __padding_end: [u8; 192],
}
impl UPolyEditSetUVProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditSetUVProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditSetUVProperties")
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
#[repr(C, align(16))]
pub struct UPolyEditPlanarProjectionUVActivity {
    __padding_end: [u8; 256],
}
impl UPolyEditPlanarProjectionUVActivity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPlanarProjectionUVActivity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPlanarProjectionUVActivity")
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
pub struct UUVLayoutToolBuilder {
    __padding_end: [u8; 48],
}
impl UUVLayoutToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutToolBuilder")
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
#[repr(C, align(16))]
pub struct UUVLayoutTool {
    __padding_end: [u8; 432],
}
impl UUVLayoutTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutTool")
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
pub struct UUVProjectionToolBuilder {
    __padding_end: [u8; 48],
}
impl UUVProjectionToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolBuilder")
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
pub struct UUVProjectionToolEditActions {
    __padding_end: [u8; 192],
}
impl UUVProjectionToolEditActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolEditActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolEditActions")
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
#[repr(C, align(16))]
pub struct UUVProjectionToolProperties {
    __padding_end: [u8; 416],
}
impl UUVProjectionToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionToolProperties")
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
pub struct UUVProjectionOperatorFactory {
    __padding_end: [u8; 64],
}
impl UUVProjectionOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionOperatorFactory")
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
#[repr(C, align(16))]
pub struct UUVProjectionTool {
    __padding_end: [u8; 1312],
}
impl UUVProjectionTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVProjectionTool")
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
pub struct UWeldMeshEdgesToolBuilder {
    __padding_end: [u8; 48],
}
impl UWeldMeshEdgesToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesToolBuilder")
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
pub struct UWeldMeshEdgesToolProperties {
    __padding_end: [u8; 224],
}
impl UWeldMeshEdgesToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesToolProperties")
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
pub struct UWeldMeshEdgesOperatorFactory {
    __padding_end: [u8; 64],
}
impl UWeldMeshEdgesOperatorFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesOperatorFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesOperatorFactory")
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
pub struct UWeldMeshEdgesTool {
    __padding_end: [u8; 464],
}
impl UWeldMeshEdgesTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeldMeshEdgesTool")
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
#[repr(transparent)]
pub struct EBrushToolSizeType(pub u8);
impl EBrushToolSizeType {
    pub const ADAPTIVE: EBrushToolSizeType = EBrushToolSizeType(0);
    pub const WORLD: EBrushToolSizeType = EBrushToolSizeType(1);
}
#[repr(transparent)]
pub struct EMeshVertexPaintBrushAreaType(pub u8);
impl EMeshVertexPaintBrushAreaType {
    pub const CONNECTED: EMeshVertexPaintBrushAreaType = EMeshVertexPaintBrushAreaType(
        0,
    );
    pub const VOLUMETRIC: EMeshVertexPaintBrushAreaType = EMeshVertexPaintBrushAreaType(
        1,
    );
}
#[repr(transparent)]
pub struct EPlaneBrushSideMode(pub u8);
impl EPlaneBrushSideMode {
    pub const BOTH_SIDES: EPlaneBrushSideMode = EPlaneBrushSideMode(0);
    pub const PUSH_DOWN: EPlaneBrushSideMode = EPlaneBrushSideMode(1);
    pub const PULL_TOWARDS: EPlaneBrushSideMode = EPlaneBrushSideMode(2);
}
#[repr(transparent)]
pub struct EVertexColorPaintBrushOpBlendMode(pub i32);
impl EVertexColorPaintBrushOpBlendMode {
    pub const LERP: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        0,
    );
    pub const MIX: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        1,
    );
    pub const MULTIPLY: EVertexColorPaintBrushOpBlendMode = EVertexColorPaintBrushOpBlendMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshBoundaryConstraint(pub u8);
impl EMeshBoundaryConstraint {
    pub const FIXED: EMeshBoundaryConstraint = EMeshBoundaryConstraint(7);
    pub const REFINE: EMeshBoundaryConstraint = EMeshBoundaryConstraint(5);
    pub const FREE: EMeshBoundaryConstraint = EMeshBoundaryConstraint(1);
}
#[repr(transparent)]
pub struct EGroupBoundaryConstraint(pub u8);
impl EGroupBoundaryConstraint {
    pub const FIXED: EGroupBoundaryConstraint = EGroupBoundaryConstraint(7);
    pub const REFINE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(5);
    pub const FREE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(1);
    pub const IGNORE: EGroupBoundaryConstraint = EGroupBoundaryConstraint(0);
}
#[repr(transparent)]
pub struct EMaterialBoundaryConstraint(pub u8);
impl EMaterialBoundaryConstraint {
    pub const FIXED: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(7);
    pub const REFINE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(5);
    pub const FREE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(1);
    pub const IGNORE: EMaterialBoundaryConstraint = EMaterialBoundaryConstraint(0);
}
#[repr(transparent)]
pub struct EMakeMeshPolygroupMode(pub u8);
impl EMakeMeshPolygroupMode {
    pub const PER_SHAPE: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(0);
    pub const PER_FACE: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(1);
    pub const PER_QUAD: EMakeMeshPolygroupMode = EMakeMeshPolygroupMode(2);
}
#[repr(transparent)]
pub struct EMakeMeshPlacementType(pub u8);
impl EMakeMeshPlacementType {
    pub const GROUND_PLANE: EMakeMeshPlacementType = EMakeMeshPlacementType(0);
    pub const ON_SCENE: EMakeMeshPlacementType = EMakeMeshPlacementType(1);
    pub const AT_ORIGIN: EMakeMeshPlacementType = EMakeMeshPlacementType(2);
}
#[repr(transparent)]
pub struct EMakeMeshPivotLocation(pub u8);
impl EMakeMeshPivotLocation {
    pub const BASE: EMakeMeshPivotLocation = EMakeMeshPivotLocation(0);
    pub const CENTERED: EMakeMeshPivotLocation = EMakeMeshPivotLocation(1);
    pub const TOP: EMakeMeshPivotLocation = EMakeMeshPivotLocation(2);
}
#[repr(transparent)]
pub struct EProceduralRectType(pub i32);
impl EProceduralRectType {
    pub const RECTANGLE: EProceduralRectType = EProceduralRectType(0);
    pub const ROUNDED_RECTANGLE: EProceduralRectType = EProceduralRectType(1);
}
#[repr(transparent)]
pub struct EProceduralDiscType(pub i32);
impl EProceduralDiscType {
    pub const DISC: EProceduralDiscType = EProceduralDiscType(0);
    pub const PUNCTURED_DISC: EProceduralDiscType = EProceduralDiscType(1);
}
#[repr(transparent)]
pub struct EProceduralSphereType(pub i32);
impl EProceduralSphereType {
    pub const LAT_LONG: EProceduralSphereType = EProceduralSphereType(0);
    pub const BOX: EProceduralSphereType = EProceduralSphereType(1);
}
#[repr(transparent)]
pub struct EProceduralStairsType(pub i32);
impl EProceduralStairsType {
    pub const LINEAR: EProceduralStairsType = EProceduralStairsType(0);
    pub const FLOATING: EProceduralStairsType = EProceduralStairsType(1);
    pub const CURVED: EProceduralStairsType = EProceduralStairsType(2);
    pub const SPIRAL: EProceduralStairsType = EProceduralStairsType(3);
}
#[repr(transparent)]
pub struct EConvertToPolygonsMode(pub i32);
impl EConvertToPolygonsMode {
    pub const FACE_NORMAL_DEVIATION: EConvertToPolygonsMode = EConvertToPolygonsMode(0);
    pub const FIND_POLYGONS: EConvertToPolygonsMode = EConvertToPolygonsMode(1);
    pub const FROM_MATERIAL_I_DS: EConvertToPolygonsMode = EConvertToPolygonsMode(7);
    pub const FROM_UV_ISLANDS: EConvertToPolygonsMode = EConvertToPolygonsMode(2);
    pub const FROM_NORMAL_SEAMS: EConvertToPolygonsMode = EConvertToPolygonsMode(3);
    pub const FROM_CONNECTED_TRIS: EConvertToPolygonsMode = EConvertToPolygonsMode(4);
    pub const FROM_FURTHEST_POINT_SAMPLING: EConvertToPolygonsMode = EConvertToPolygonsMode(
        5,
    );
    pub const COPY_FROM_LAYER: EConvertToPolygonsMode = EConvertToPolygonsMode(6);
}
#[repr(transparent)]
pub struct EGroupTopologyDeformationStrategy(pub u8);
impl EGroupTopologyDeformationStrategy {
    pub const LINEAR: EGroupTopologyDeformationStrategy = EGroupTopologyDeformationStrategy(
        0,
    );
    pub const LAPLACIAN: EGroupTopologyDeformationStrategy = EGroupTopologyDeformationStrategy(
        1,
    );
}
#[repr(transparent)]
pub struct EQuickTransformerMode(pub u8);
impl EQuickTransformerMode {
    pub const AXIS_TRANSLATION: EQuickTransformerMode = EQuickTransformerMode(0);
    pub const AXIS_ROTATION: EQuickTransformerMode = EQuickTransformerMode(1);
}
#[repr(transparent)]
pub struct EWeightScheme(pub i32);
impl EWeightScheme {
    pub const UNIFORM: EWeightScheme = EWeightScheme(0);
    pub const UMBRELLA: EWeightScheme = EWeightScheme(1);
    pub const VALENCE: EWeightScheme = EWeightScheme(2);
    pub const MEAN_VALUE: EWeightScheme = EWeightScheme(3);
    pub const COTANGENT: EWeightScheme = EWeightScheme(4);
    pub const CLAMPED_COTANGENT: EWeightScheme = EWeightScheme(5);
    pub const IDT_COTANGENT: EWeightScheme = EWeightScheme(6);
}
#[repr(transparent)]
pub struct EDisplaceMeshToolDisplaceType(pub u8);
impl EDisplaceMeshToolDisplaceType {
    pub const CONSTANT: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(0);
    pub const DISPLACEMENT_MAP: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        1,
    );
    pub const RANDOM_NOISE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        2,
    );
    pub const PERLIN_NOISE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        3,
    );
    pub const SINE_WAVE: EDisplaceMeshToolDisplaceType = EDisplaceMeshToolDisplaceType(
        4,
    );
}
#[repr(transparent)]
pub struct EDisplaceMeshToolSubdivisionType(pub u8);
impl EDisplaceMeshToolSubdivisionType {
    pub const FLAT: EDisplaceMeshToolSubdivisionType = EDisplaceMeshToolSubdivisionType(
        0,
    );
    pub const PN_TRIANGLES: EDisplaceMeshToolSubdivisionType = EDisplaceMeshToolSubdivisionType(
        1,
    );
}
#[repr(transparent)]
pub struct EDisplaceMeshToolTriangleSelectionType(pub u8);
impl EDisplaceMeshToolTriangleSelectionType {
    pub const NONE: EDisplaceMeshToolTriangleSelectionType = EDisplaceMeshToolTriangleSelectionType(
        0,
    );
    pub const MATERIAL: EDisplaceMeshToolTriangleSelectionType = EDisplaceMeshToolTriangleSelectionType(
        1,
    );
}
#[repr(transparent)]
pub struct EDisplaceMeshToolChannelType(pub u8);
impl EDisplaceMeshToolChannelType {
    pub const RED: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(0);
    pub const GREEN: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(1);
    pub const BLUE: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(2);
    pub const ALPHA: EDisplaceMeshToolChannelType = EDisplaceMeshToolChannelType(3);
}
#[repr(transparent)]
pub struct ERevolvePropertiesPolygroupMode(pub u8);
impl ERevolvePropertiesPolygroupMode {
    pub const PER_SHAPE: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        0,
    );
    pub const PER_FACE: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        1,
    );
    pub const PER_REVOLVE_STEP: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        2,
    );
    pub const PER_PATH_SEGMENT: ERevolvePropertiesPolygroupMode = ERevolvePropertiesPolygroupMode(
        3,
    );
}
#[repr(transparent)]
pub struct ERevolvePropertiesQuadSplit(pub u8);
impl ERevolvePropertiesQuadSplit {
    pub const UNIFORM: ERevolvePropertiesQuadSplit = ERevolvePropertiesQuadSplit(0);
    pub const COMPACT: ERevolvePropertiesQuadSplit = ERevolvePropertiesQuadSplit(1);
}
#[repr(transparent)]
pub struct ERevolvePropertiesCapFillMode(pub u8);
impl ERevolvePropertiesCapFillMode {
    pub const NONE: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(0);
    pub const CENTER_FAN: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(
        1,
    );
    pub const DELAUNAY: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(2);
    pub const EAR_CLIPPING: ERevolvePropertiesCapFillMode = ERevolvePropertiesCapFillMode(
        3,
    );
}
#[repr(transparent)]
pub struct EDrawPolygonDrawMode(pub u8);
impl EDrawPolygonDrawMode {
    pub const FREEHAND: EDrawPolygonDrawMode = EDrawPolygonDrawMode(0);
    pub const CIRCLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(1);
    pub const SQUARE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(2);
    pub const RECTANGLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(3);
    pub const ROUNDED_RECTANGLE: EDrawPolygonDrawMode = EDrawPolygonDrawMode(4);
    pub const RING: EDrawPolygonDrawMode = EDrawPolygonDrawMode(5);
}
#[repr(transparent)]
pub struct EDrawPolygonExtrudeMode(pub u8);
impl EDrawPolygonExtrudeMode {
    pub const FLAT: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(0);
    pub const FIXED: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(1);
    pub const INTERACTIVE: EDrawPolygonExtrudeMode = EDrawPolygonExtrudeMode(2);
}
#[repr(transparent)]
pub struct EDynamicMeshSculptBrushType(pub u8);
impl EDynamicMeshSculptBrushType {
    pub const MOVE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(0);
    pub const PULL_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(1);
    pub const PULL_SHARP_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        2,
    );
    pub const SMOOTH: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(3);
    pub const OFFSET: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(4);
    pub const SCULPT_VIEW: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(5);
    pub const SCULPT_MAX: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(6);
    pub const INFLATE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(7);
    pub const SCALE_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(8);
    pub const PINCH: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(9);
    pub const TWIST_KELVIN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        10,
    );
    pub const FLATTEN: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(11);
    pub const PLANE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(12);
    pub const PLANE_VIEW_ALIGNED: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(
        13,
    );
    pub const FIXED_PLANE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(14);
    pub const RESAMPLE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(15);
    pub const LAST_VALUE: EDynamicMeshSculptBrushType = EDynamicMeshSculptBrushType(16);
}
#[repr(transparent)]
pub struct ELocalFrameMode(pub i32);
impl ELocalFrameMode {
    pub const FROM_OBJECT: ELocalFrameMode = ELocalFrameMode(0);
    pub const FROM_GEOMETRY: ELocalFrameMode = ELocalFrameMode(1);
}
#[repr(transparent)]
pub struct EBrushActionMode(pub i32);
impl EBrushActionMode {
    pub const PAINT: EBrushActionMode = EBrushActionMode(0);
    pub const FLOOD_FILL: EBrushActionMode = EBrushActionMode(1);
    pub const ERASE: EBrushActionMode = EBrushActionMode(2);
    pub const SMOOTH: EBrushActionMode = EBrushActionMode(3);
}
#[repr(transparent)]
pub struct EMeshAttributePaintMaterialMode(pub i32);
impl EMeshAttributePaintMaterialMode {
    pub const SHADED: EMeshAttributePaintMaterialMode = EMeshAttributePaintMaterialMode(
        0,
    );
    pub const COLOR_ONLY: EMeshAttributePaintMaterialMode = EMeshAttributePaintMaterialMode(
        1,
    );
}
#[repr(transparent)]
pub struct EMeshGroupPaintBrushType(pub u8);
impl EMeshGroupPaintBrushType {
    pub const PAINT: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(0);
    pub const ERASE: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(1);
    pub const LAST_VALUE: EMeshGroupPaintBrushType = EMeshGroupPaintBrushType(2);
}
#[repr(transparent)]
pub struct EMeshGroupPaintInteractionType(pub u8);
impl EMeshGroupPaintInteractionType {
    pub const BRUSH: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(0);
    pub const FILL: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(1);
    pub const GROUP_FILL: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        2,
    );
    pub const POLY_LASSO: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        3,
    );
    pub const LAST_VALUE: EMeshGroupPaintInteractionType = EMeshGroupPaintInteractionType(
        4,
    );
}
#[repr(transparent)]
pub struct EMeshGroupPaintBrushAreaType(pub u8);
impl EMeshGroupPaintBrushAreaType {
    pub const CONNECTED: EMeshGroupPaintBrushAreaType = EMeshGroupPaintBrushAreaType(0);
    pub const VOLUMETRIC: EMeshGroupPaintBrushAreaType = EMeshGroupPaintBrushAreaType(1);
}
#[repr(transparent)]
pub struct EMeshGroupPaintVisibilityType(pub u8);
impl EMeshGroupPaintVisibilityType {
    pub const NONE: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(0);
    pub const FRONT_FACING: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(
        1,
    );
    pub const UNOCCLUDED: EMeshGroupPaintVisibilityType = EMeshGroupPaintVisibilityType(
        2,
    );
}
#[repr(transparent)]
pub struct ENonlinearOperationType(pub i8);
impl ENonlinearOperationType {
    pub const BEND: ENonlinearOperationType = ENonlinearOperationType(0);
    pub const FLARE: ENonlinearOperationType = ENonlinearOperationType(1);
    pub const TWIST: ENonlinearOperationType = ENonlinearOperationType(2);
}
#[repr(transparent)]
pub struct EFlareProfileType(pub i8);
impl EFlareProfileType {
    pub const SIN_MODE: EFlareProfileType = EFlareProfileType(0);
    pub const SIN_SQUARED_MODE: EFlareProfileType = EFlareProfileType(1);
    pub const TRIANGLE_MODE: EFlareProfileType = EFlareProfileType(2);
}
#[repr(transparent)]
pub struct EMeshVertexPaintBrushType(pub u8);
impl EMeshVertexPaintBrushType {
    pub const PAINT: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(0);
    pub const ERASE: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(1);
    pub const SOFTEN: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(2);
    pub const SMOOTH: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(3);
    pub const LAST_VALUE: EMeshVertexPaintBrushType = EMeshVertexPaintBrushType(4);
}
#[repr(transparent)]
pub struct EMeshVertexPaintInteractionType(pub u8);
impl EMeshVertexPaintInteractionType {
    pub const BRUSH: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        0,
    );
    pub const TRI_FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        1,
    );
    pub const FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(2);
    pub const GROUP_FILL: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        3,
    );
    pub const POLY_LASSO: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        4,
    );
    pub const LAST_VALUE: EMeshVertexPaintInteractionType = EMeshVertexPaintInteractionType(
        5,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintColorBlendMode(pub u8);
impl EMeshVertexPaintColorBlendMode {
    pub const LERP: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(0);
    pub const MIX: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(1);
    pub const MULTIPLY: EMeshVertexPaintColorBlendMode = EMeshVertexPaintColorBlendMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintSecondaryActionType(pub u8);
impl EMeshVertexPaintSecondaryActionType {
    pub const ERASE: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        0,
    );
    pub const SOFTEN: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        1,
    );
    pub const SMOOTH: EMeshVertexPaintSecondaryActionType = EMeshVertexPaintSecondaryActionType(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintVisibilityType(pub u8);
impl EMeshVertexPaintVisibilityType {
    pub const NONE: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(0);
    pub const FRONT_FACING: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(
        1,
    );
    pub const UNOCCLUDED: EMeshVertexPaintVisibilityType = EMeshVertexPaintVisibilityType(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintMaterialMode(pub u8);
impl EMeshVertexPaintMaterialMode {
    pub const LIT_VERTEX_COLOR: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        0,
    );
    pub const UNLIT_VERTEX_COLOR: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        1,
    );
    pub const ORIGINAL_MATERIAL: EMeshVertexPaintMaterialMode = EMeshVertexPaintMaterialMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintToolUtilityOperations(pub i32);
impl EMeshVertexPaintToolUtilityOperations {
    pub const BLEND_ALL_SEAMS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        0,
    );
    pub const FILL_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        1,
    );
    pub const INVERT_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        2,
    );
    pub const COPY_CHANNEL_TO_CHANNEL: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        3,
    );
    pub const SWAP_CHANNELS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        4,
    );
    pub const COPY_FROM_WEIGHT_MAP: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        5,
    );
    pub const COPY_TO_OTHER_LO_DS: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        6,
    );
    pub const COPY_TO_SINGLE_LOD: EMeshVertexPaintToolUtilityOperations = EMeshVertexPaintToolUtilityOperations(
        7,
    );
}
#[repr(transparent)]
pub struct EMeshVertexPaintColorChannel(pub u8);
impl EMeshVertexPaintColorChannel {
    pub const RED: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(0);
    pub const GREEN: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(1);
    pub const BLUE: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(2);
    pub const ALPHA: EMeshVertexPaintColorChannel = EMeshVertexPaintColorChannel(3);
}
#[repr(transparent)]
pub struct EMeshVertexSculptBrushType(pub u8);
impl EMeshVertexSculptBrushType {
    pub const MOVE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(0);
    pub const PULL_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(1);
    pub const PULL_SHARP_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        2,
    );
    pub const SMOOTH: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(3);
    pub const SMOOTH_FILL: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(4);
    pub const OFFSET: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(5);
    pub const SCULPT_VIEW: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(6);
    pub const SCULPT_MAX: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(7);
    pub const INFLATE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(8);
    pub const INFLATE_STROKE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(9);
    pub const INFLATE_MAX: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(10);
    pub const SCALE_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(11);
    pub const PINCH: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(12);
    pub const TWIST_KELVIN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(13);
    pub const FLATTEN: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(14);
    pub const PLANE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(15);
    pub const PLANE_VIEW_ALIGNED: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        16,
    );
    pub const FIXED_PLANE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(17);
    pub const ERASE_SCULPT_LAYER: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(
        18,
    );
    pub const LAST_VALUE: EMeshVertexSculptBrushType = EMeshVertexSculptBrushType(19);
}
#[repr(transparent)]
pub struct EMeshSculptFalloffType(pub u8);
impl EMeshSculptFalloffType {
    pub const SMOOTH: EMeshSculptFalloffType = EMeshSculptFalloffType(0);
    pub const LINEAR: EMeshSculptFalloffType = EMeshSculptFalloffType(1);
    pub const INVERSE: EMeshSculptFalloffType = EMeshSculptFalloffType(2);
    pub const ROUND: EMeshSculptFalloffType = EMeshSculptFalloffType(3);
    pub const BOX_SMOOTH: EMeshSculptFalloffType = EMeshSculptFalloffType(4);
    pub const BOX_LINEAR: EMeshSculptFalloffType = EMeshSculptFalloffType(5);
    pub const BOX_INVERSE: EMeshSculptFalloffType = EMeshSculptFalloffType(6);
    pub const BOX_ROUND: EMeshSculptFalloffType = EMeshSculptFalloffType(7);
    pub const LAST_VALUE: EMeshSculptFalloffType = EMeshSculptFalloffType(8);
}
#[repr(transparent)]
pub struct EMeshVertexSculptBrushFilterType(pub u8);
impl EMeshVertexSculptBrushFilterType {
    pub const NONE: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        0,
    );
    pub const COMPONENT: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        1,
    );
    pub const POLY_GROUP: EMeshVertexSculptBrushFilterType = EMeshVertexSculptBrushFilterType(
        2,
    );
}
#[repr(transparent)]
pub struct EOffsetMeshToolOffsetType(pub u8);
impl EOffsetMeshToolOffsetType {
    pub const ITERATIVE: EOffsetMeshToolOffsetType = EOffsetMeshToolOffsetType(0);
    pub const IMPLICIT: EOffsetMeshToolOffsetType = EOffsetMeshToolOffsetType(1);
}
#[repr(transparent)]
pub struct ESetMeshMaterialMode(pub u8);
impl ESetMeshMaterialMode {
    pub const ORIGINAL: ESetMeshMaterialMode = ESetMeshMaterialMode(0);
    pub const CHECKERBOARD: ESetMeshMaterialMode = ESetMeshMaterialMode(1);
    pub const OVERRIDE: ESetMeshMaterialMode = ESetMeshMaterialMode(2);
}
#[repr(transparent)]
pub struct EMeshEditingMaterialModes(pub i32);
impl EMeshEditingMaterialModes {
    pub const EXISTING_MATERIAL: EMeshEditingMaterialModes = EMeshEditingMaterialModes(
        0,
    );
    pub const DIFFUSE: EMeshEditingMaterialModes = EMeshEditingMaterialModes(1);
    pub const GREY: EMeshEditingMaterialModes = EMeshEditingMaterialModes(2);
    pub const SOFT: EMeshEditingMaterialModes = EMeshEditingMaterialModes(3);
    pub const TRANSPARENT: EMeshEditingMaterialModes = EMeshEditingMaterialModes(4);
    pub const TANGENT_NORMAL: EMeshEditingMaterialModes = EMeshEditingMaterialModes(5);
    pub const VERTEX_COLOR: EMeshEditingMaterialModes = EMeshEditingMaterialModes(6);
    pub const CUSTOM_IMAGE: EMeshEditingMaterialModes = EMeshEditingMaterialModes(7);
    pub const CUSTOM: EMeshEditingMaterialModes = EMeshEditingMaterialModes(8);
}
#[repr(transparent)]
pub struct EOcclusionCalculationUIMode(pub u8);
impl EOcclusionCalculationUIMode {
    pub const GENERALIZED_WINDING_NUMBER: EOcclusionCalculationUIMode = EOcclusionCalculationUIMode(
        0,
    );
    pub const RAYCAST_OCCLUSION_SAMPLES: EOcclusionCalculationUIMode = EOcclusionCalculationUIMode(
        1,
    );
}
#[repr(transparent)]
pub struct EOcclusionTriangleSamplingUIMode(pub u8);
impl EOcclusionTriangleSamplingUIMode {
    pub const VERTICES: EOcclusionTriangleSamplingUIMode = EOcclusionTriangleSamplingUIMode(
        0,
    );
    pub const VERTICES_AND_CENTROIDS: EOcclusionTriangleSamplingUIMode = EOcclusionTriangleSamplingUIMode(
        1,
    );
}
#[repr(transparent)]
pub struct EOccludedAction(pub u8);
impl EOccludedAction {
    pub const REMOVE: EOccludedAction = EOccludedAction(0);
    pub const SET_NEW_GROUP: EOccludedAction = EOccludedAction(1);
}
#[repr(transparent)]
pub struct EMeshSculptStrokeType(pub u8);
impl EMeshSculptStrokeType {
    pub const SPACING: EMeshSculptStrokeType = EMeshSculptStrokeType(0);
    pub const AIRBRUSH: EMeshSculptStrokeType = EMeshSculptStrokeType(1);
    pub const DOTS: EMeshSculptStrokeType = EMeshSculptStrokeType(2);
    pub const LAST_VALUE: EMeshSculptStrokeType = EMeshSculptStrokeType(3);
}
#[repr(transparent)]
pub struct ESmoothMeshToolSmoothType(pub u8);
impl ESmoothMeshToolSmoothType {
    pub const ITERATIVE: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(0);
    pub const IMPLICIT: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(1);
    pub const DIFFUSION: ESmoothMeshToolSmoothType = ESmoothMeshToolSmoothType(2);
}
#[repr(transparent)]
pub struct EPolyEditCutPlaneOrientation(pub i32);
impl EPolyEditCutPlaneOrientation {
    pub const FACE_NORMALS: EPolyEditCutPlaneOrientation = EPolyEditCutPlaneOrientation(
        0,
    );
    pub const VIEW_DIRECTION: EPolyEditCutPlaneOrientation = EPolyEditCutPlaneOrientation(
        1,
    );
}
#[repr(transparent)]
pub struct EPolyEditExtrudeDistanceMode(pub i32);
impl EPolyEditExtrudeDistanceMode {
    pub const CLICK_IN_VIEWPORT: EPolyEditExtrudeDistanceMode = EPolyEditExtrudeDistanceMode(
        0,
    );
    pub const FIXED: EPolyEditExtrudeDistanceMode = EPolyEditExtrudeDistanceMode(1);
}
#[repr(transparent)]
pub struct EPolyEditExtrudeModeOptions(pub i32);
impl EPolyEditExtrudeModeOptions {
    pub const SINGLE_DIRECTION: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        3,
    );
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditExtrudeModeOptions = EPolyEditExtrudeModeOptions(
        1,
    );
}
#[repr(transparent)]
pub struct EPolyEditExtrudeDirection(pub i32);
impl EPolyEditExtrudeDirection {
    pub const SELECTION_NORMAL: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(0);
    pub const WORLD_X: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(1);
    pub const WORLD_Y: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(2);
    pub const WORLD_Z: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(3);
    pub const LOCAL_X: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(4);
    pub const LOCAL_Y: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(5);
    pub const LOCAL_Z: EPolyEditExtrudeDirection = EPolyEditExtrudeDirection(6);
}
#[repr(transparent)]
pub struct EPolyEditOffsetModeOptions(pub i32);
impl EPolyEditOffsetModeOptions {
    pub const VERTEX_NORMALS: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(2);
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditOffsetModeOptions = EPolyEditOffsetModeOptions(
        1,
    );
}
#[repr(transparent)]
pub struct EPolyEditPushPullModeOptions(pub i32);
impl EPolyEditPushPullModeOptions {
    pub const SELECTED_TRIANGLE_NORMALS: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        0,
    );
    pub const SELECTED_TRIANGLE_NORMALS_EVEN: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        1,
    );
    pub const SINGLE_DIRECTION: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        3,
    );
    pub const VERTEX_NORMALS: EPolyEditPushPullModeOptions = EPolyEditPushPullModeOptions(
        2,
    );
}
#[repr(transparent)]
pub struct EPolyEditExtrudeEdgeDirectionMode(pub i32);
impl EPolyEditExtrudeEdgeDirectionMode {
    pub const LOCAL_EXTRUDE_FRAMES: EPolyEditExtrudeEdgeDirectionMode = EPolyEditExtrudeEdgeDirectionMode(
        0,
    );
    pub const SINGLE_DIRECTION: EPolyEditExtrudeEdgeDirectionMode = EPolyEditExtrudeEdgeDirectionMode(
        1,
    );
}
#[repr(transparent)]
pub struct EPolyEditExtrudeEdgeDistanceMode(pub i32);
impl EPolyEditExtrudeEdgeDistanceMode {
    pub const FIXED: EPolyEditExtrudeEdgeDistanceMode = EPolyEditExtrudeEdgeDistanceMode(
        0,
    );
    pub const GIZMO: EPolyEditExtrudeEdgeDistanceMode = EPolyEditExtrudeEdgeDistanceMode(
        1,
    );
}
#[repr(transparent)]
pub struct EGroupEdgeInsertionMode(pub i32);
impl EGroupEdgeInsertionMode {
    pub const RETRIANGULATE: EGroupEdgeInsertionMode = EGroupEdgeInsertionMode(0);
    pub const PLANE_CUT: EGroupEdgeInsertionMode = EGroupEdgeInsertionMode(1);
}
#[repr(transparent)]
pub struct EEdgeLoopPositioningMode(pub i32);
impl EEdgeLoopPositioningMode {
    pub const EVEN: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(0);
    pub const PROPORTION_OFFSET: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(1);
    pub const DISTANCE_OFFSET: EEdgeLoopPositioningMode = EEdgeLoopPositioningMode(2);
}
#[repr(transparent)]
pub struct EEdgeLoopInsertionMode(pub i32);
impl EEdgeLoopInsertionMode {
    pub const RETRIANGULATE: EEdgeLoopInsertionMode = EEdgeLoopInsertionMode(0);
    pub const PLANE_CUT: EEdgeLoopInsertionMode = EEdgeLoopInsertionMode(1);
}
#[repr(transparent)]
pub struct EUVProjectionToolInitializationMode(pub i32);
impl EUVProjectionToolInitializationMode {
    pub const DEFAULT: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        0,
    );
    pub const USE_PREVIOUS: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        1,
    );
    pub const AUTO_FIT: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        2,
    );
    pub const AUTO_FIT_ALIGN: EUVProjectionToolInitializationMode = EUVProjectionToolInitializationMode(
        3,
    );
}
#[repr(transparent)]
pub struct EWeldMeshEdgesAttributeUIMode(pub u8);
impl EWeldMeshEdgesAttributeUIMode {
    pub const NONE: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(0);
    pub const ON_WELDED_MESH_EDGES_ONLY: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(
        1,
    );
    pub const ON_FULL_MESH: EWeldMeshEdgesAttributeUIMode = EWeldMeshEdgesAttributeUIMode(
        2,
    );
}
