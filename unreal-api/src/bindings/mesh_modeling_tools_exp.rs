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
    pub u_bake_input_mesh_properties_get_target_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_input_mesh_properties_get_source_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_texture2_d_properties_get_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_multi_texture2_d_properties_get_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_mesh_attribute_maps_tool_properties_get_map_preview_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_multi_mesh_attribute_maps_tool_properties_get_map_preview_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_multi_mesh_input_tool_properties_get_target_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_slide_forward: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_slide_back: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_reset_grid_from_actor: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_push: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_pull: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_flip: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_corner_mode: *mut crate::ffi::UFunctionOpague,
    pub u_cube_grid_tool_actions_accept_and_start_new: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_world_origin: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_top: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_right: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_left: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_front: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_center: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_bottom: *mut crate::ffi::UFunctionOpague,
    pub u_edit_pivot_tool_action_property_set_back: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_inspector_material_properties_get_uv_channel_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_shrink: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_select_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_optimize_border: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_largest_tri_count_part: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_largest_area_part: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_invert: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_grow: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_flood_fill: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_expand_to_materials: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_edit_actions_clear: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_smooth_border: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_separate: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_flip_normals: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_duplicate: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_disconnect: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_delete: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_selection_mesh_edit_actions_create_polygroup: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_up: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_shift_to_center: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_right: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_left: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_forward: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_down: *mut crate::ffi::UFunctionOpague,
    pub u_mirror_tool_action_property_set_backward: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_duplicate: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_delete_all: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_delete: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_add_sphere: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_add_capsule: *mut crate::ffi::UFunctionOpague,
    pub u_simple_collision_editor_tool_action_properties_add_box: *mut crate::ffi::UFunctionOpague,
    pub u_plane_cut_tool_flip_plane: *mut crate::ffi::UFunctionOpague,
    pub u_plane_cut_tool_cut: *mut crate::ffi::UFunctionOpague,
    pub u_revolve_spline_tool_action_property_set_reset_axis: *mut crate::ffi::UFunctionOpague,
    pub u_transfer_mesh_tool_properties_get_target_lod_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_transfer_mesh_tool_properties_get_source_lod_names_func: *mut crate::ffi::UFunctionOpague,
    pub uuv_transfer_tool_properties_get_source_uv_channel_names: *mut crate::ffi::UFunctionOpague,
    pub uuv_transfer_tool_properties_get_dest_uv_channel_names: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_bake_input_mesh_properties_get_target_uv_layer_names_func: std::ptr::null_mut(),
            u_bake_input_mesh_properties_get_source_uv_layer_names_func: std::ptr::null_mut(),
            u_bake_texture2_d_properties_get_uv_layer_names_func: std::ptr::null_mut(),
            u_bake_multi_texture2_d_properties_get_uv_layer_names_func: std::ptr::null_mut(),
            u_bake_mesh_attribute_maps_tool_properties_get_map_preview_names_func: std::ptr::null_mut(),
            u_bake_multi_mesh_attribute_maps_tool_properties_get_map_preview_names_func: std::ptr::null_mut(),
            u_bake_multi_mesh_input_tool_properties_get_target_uv_layer_names_func: std::ptr::null_mut(),
            u_cube_grid_tool_actions_slide_forward: std::ptr::null_mut(),
            u_cube_grid_tool_actions_slide_back: std::ptr::null_mut(),
            u_cube_grid_tool_actions_reset_grid_from_actor: std::ptr::null_mut(),
            u_cube_grid_tool_actions_push: std::ptr::null_mut(),
            u_cube_grid_tool_actions_pull: std::ptr::null_mut(),
            u_cube_grid_tool_actions_flip: std::ptr::null_mut(),
            u_cube_grid_tool_actions_corner_mode: std::ptr::null_mut(),
            u_cube_grid_tool_actions_accept_and_start_new: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_world_origin: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_top: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_right: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_left: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_front: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_center: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_bottom: std::ptr::null_mut(),
            u_edit_pivot_tool_action_property_set_back: std::ptr::null_mut(),
            u_mesh_inspector_material_properties_get_uv_channel_names_func: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_shrink: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_select_all: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_optimize_border: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_largest_tri_count_part: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_largest_area_part: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_invert: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_grow: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_flood_fill: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_expand_to_materials: std::ptr::null_mut(),
            u_mesh_selection_edit_actions_clear: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_smooth_border: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_separate: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_flip_normals: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_duplicate: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_disconnect: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_delete: std::ptr::null_mut(),
            u_mesh_selection_mesh_edit_actions_create_polygroup: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_up: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_shift_to_center: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_right: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_left: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_forward: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_down: std::ptr::null_mut(),
            u_mirror_tool_action_property_set_backward: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_duplicate: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_delete_all: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_delete: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_add_sphere: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_add_capsule: std::ptr::null_mut(),
            u_simple_collision_editor_tool_action_properties_add_box: std::ptr::null_mut(),
            u_plane_cut_tool_flip_plane: std::ptr::null_mut(),
            u_plane_cut_tool_cut: std::ptr::null_mut(),
            u_revolve_spline_tool_action_property_set_reset_axis: std::ptr::null_mut(),
            u_transfer_mesh_tool_properties_get_target_lod_names_func: std::ptr::null_mut(),
            u_transfer_mesh_tool_properties_get_source_lod_names_func: std::ptr::null_mut(),
            uuv_transfer_tool_properties_get_source_uv_channel_names: std::ptr::null_mut(),
            uuv_transfer_tool_properties_get_dest_uv_channel_names: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeInputMeshProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_input_mesh_properties_get_target_uv_layer_names_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_input_mesh_properties_get_source_uv_layer_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeTexture2DProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS.u_bake_texture2_d_properties_get_uv_layer_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeMultiTexture2DProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_multi_texture2_d_properties_get_uv_layer_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeMeshAttributeMapsToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMapPreviewNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_mesh_attribute_maps_tool_properties_get_map_preview_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeMultiMeshAttributeMapsToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMapPreviewNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_multi_mesh_attribute_maps_tool_properties_get_map_preview_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeMultiMeshInputToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_multi_mesh_input_tool_properties_get_target_uv_layer_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCubeGridToolActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlideForward"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_slide_forward,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SlideBack"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_slide_back,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetGridFromActor"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_reset_grid_from_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Push"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_push,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Pull"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_pull,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Flip"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_flip,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CornerMode"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_corner_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AcceptAndStartNew"),
            &raw mut __FUNCTION_PTRS.u_cube_grid_tool_actions_accept_and_start_new,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditPivotToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WorldOrigin"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_world_origin,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Top"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_top,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Right"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_right,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Left"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_left,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Front"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_front,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Center"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_center,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Bottom"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_bottom,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Back"),
            &raw mut __FUNCTION_PTRS.u_edit_pivot_tool_action_property_set_back,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshInspectorMaterialProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUVChannelNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_mesh_inspector_material_properties_get_uv_channel_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshSelectionEditActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Shrink"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_shrink,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectAll"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_select_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OptimizeBorder"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_optimize_border,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LargestTriCountPart"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_largest_tri_count_part,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LargestAreaPart"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_largest_area_part,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Invert"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_invert,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Grow"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_grow,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FloodFill"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_flood_fill,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExpandToMaterials"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_expand_to_materials,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Clear"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_edit_actions_clear,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshSelectionMeshEditActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SmoothBorder"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_smooth_border,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Separate"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_separate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FlipNormals"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_flip_normals,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Duplicate"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_duplicate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Disconnect"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_disconnect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Delete"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_delete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePolygroup"),
            &raw mut __FUNCTION_PTRS.u_mesh_selection_mesh_edit_actions_create_polygroup,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMirrorToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Up"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_up,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShiftToCenter"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_shift_to_center,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Right"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_right,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Left"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_left,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Forward"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_forward,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Down"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_down,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Backward"),
            &raw mut __FUNCTION_PTRS.u_mirror_tool_action_property_set_backward,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USimpleCollisionEditorToolActionProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Duplicate"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_duplicate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAll"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_delete_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Delete"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_delete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSphere"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_add_sphere,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCapsule"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_add_capsule,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBox"),
            &raw mut __FUNCTION_PTRS
                .u_simple_collision_editor_tool_action_properties_add_box,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPlaneCutTool::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FlipPlane"),
            &raw mut __FUNCTION_PTRS.u_plane_cut_tool_flip_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Cut"),
            &raw mut __FUNCTION_PTRS.u_plane_cut_tool_cut,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URevolveSplineToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetAxis"),
            &raw mut __FUNCTION_PTRS.u_revolve_spline_tool_action_property_set_reset_axis,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTransferMeshToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetLODNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_transfer_mesh_tool_properties_get_target_lod_names_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceLODNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_transfer_mesh_tool_properties_get_source_lod_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUVTransferToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceUVChannelNames"),
            &raw mut __FUNCTION_PTRS
                .uuv_transfer_tool_properties_get_source_uv_channel_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDestUVChannelNames"),
            &raw mut __FUNCTION_PTRS
                .uuv_transfer_tool_properties_get_dest_uv_channel_names,
        );
    }
}
#[repr(C, align(8))]
pub struct UBakeInputMeshProperties {
    __padding_end: [u8; 336],
}
impl UBakeInputMeshProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeInputMeshProperties")
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
pub struct UBakeNormalMapToolProperties {
    __padding_end: [u8; 184],
}
impl UBakeNormalMapToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeNormalMapToolProperties")
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
pub struct UBakeOcclusionMapToolProperties {
    __padding_end: [u8; 208],
}
impl UBakeOcclusionMapToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeOcclusionMapToolProperties")
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
pub struct UBakeCurvatureMapToolProperties {
    __padding_end: [u8; 208],
}
impl UBakeCurvatureMapToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeCurvatureMapToolProperties")
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
pub struct UBakeUVShellMapToolProperties {
    __padding_end: [u8; 240],
}
impl UBakeUVShellMapToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeUVShellMapToolProperties")
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
pub struct UBakeHeightMapToolProperties {
    __padding_end: [u8; 208],
}
impl UBakeHeightMapToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeHeightMapToolProperties")
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
pub struct UBakeTexture2DProperties {
    __padding_end: [u8; 224],
}
impl UBakeTexture2DProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeTexture2DProperties")
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
pub struct UBakeMultiTexture2DProperties {
    #[doc(hidden)]
    __padding_232: [u8; 232],
    pub all_source_textures: TArray<UPtr<crate::bindings::engine::UTexture2D>>,
}
impl UBakeMultiTexture2DProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMultiTexture2DProperties")
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
pub struct UBakeVisualizationProperties {
    __padding_end: [u8; 200],
}
impl UBakeVisualizationProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeVisualizationProperties")
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
pub struct UAddPatchToolBuilder {
    __padding_end: [u8; 48],
}
impl UAddPatchToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPatchToolBuilder")
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
pub struct UAddPatchToolProperties {
    __padding_end: [u8; 200],
}
impl UAddPatchToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPatchToolProperties")
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
pub struct UAddPatchTool {
    __padding_end: [u8; 344],
}
impl UAddPatchTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPatchTool")
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
pub struct UAlignObjectsToolBuilder {
    __padding_end: [u8; 48],
}
impl UAlignObjectsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAlignObjectsToolBuilder")
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
pub struct UAlignObjectsToolProperties {
    __padding_end: [u8; 200],
}
impl UAlignObjectsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAlignObjectsToolProperties")
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
pub struct UAlignObjectsTool {
    __padding_end: [u8; 392],
}
impl UAlignObjectsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAlignObjectsTool")
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
pub struct UBakeMeshAttributeMapsToolBuilder {
    __padding_end: [u8; 48],
}
impl UBakeMeshAttributeMapsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeMapsToolBuilder")
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
pub struct UBakeMeshAttributeMapsToolProperties {
    __padding_end: [u8; 328],
}
impl UBakeMeshAttributeMapsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeMapsToolProperties")
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
pub struct UBakeMeshAttributeTool {
    __padding_end: [u8; 512],
}
impl UBakeMeshAttributeTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeTool")
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
pub struct UBakeMeshAttributeMapsToolBase {
    __padding_end: [u8; 888],
}
impl UBakeMeshAttributeMapsToolBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeMapsToolBase")
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
pub struct UBakeMeshAttributeMapsTool {
    __padding_end: [u8; 1072],
}
impl UBakeMeshAttributeMapsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeMapsTool")
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
pub struct UBakeMeshAttributeMapsResultToolProperties {
    __padding_end: [u8; 264],
}
impl UBakeMeshAttributeMapsResultToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeMapsResultToolProperties")
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
pub struct UBakeMeshAttributeVertexToolBuilder {
    __padding_end: [u8; 48],
}
impl UBakeMeshAttributeVertexToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeVertexToolBuilder")
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
pub struct UBakeMeshAttributeVertexToolProperties {
    __padding_end: [u8; 224],
}
impl UBakeMeshAttributeVertexToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeVertexToolProperties")
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
pub struct UBakeMeshAttributeVertexTool {
    __padding_end: [u8; 784],
}
impl UBakeMeshAttributeVertexTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMeshAttributeVertexTool")
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
pub struct UBakeMultiMeshAttributeMapsToolBuilder {
    __padding_end: [u8; 48],
}
impl UBakeMultiMeshAttributeMapsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMultiMeshAttributeMapsToolBuilder")
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
pub struct UBakeMultiMeshAttributeMapsToolProperties {
    __padding_end: [u8; 328],
}
impl UBakeMultiMeshAttributeMapsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMultiMeshAttributeMapsToolProperties")
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
pub struct UBakeMultiMeshInputToolProperties {
    __padding_end: [u8; 264],
}
impl UBakeMultiMeshInputToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMultiMeshInputToolProperties")
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
pub struct UBakeMultiMeshAttributeMapsTool {
    __padding_end: [u8; 1056],
}
impl UBakeMultiMeshAttributeMapsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeMultiMeshAttributeMapsTool")
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
pub struct UBakeTransformToolBuilder {
    __padding_end: [u8; 48],
}
impl UBakeTransformToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeTransformToolBuilder")
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
pub struct UBakeTransformToolProperties {
    __padding_end: [u8; 192],
}
impl UBakeTransformToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeTransformToolProperties")
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
pub struct UBakeTransformTool {
    __padding_end: [u8; 264],
}
impl UBakeTransformTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeTransformTool")
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
pub struct UConvertMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UConvertMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertMeshesToolBuilder")
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
pub struct UConvertMeshesToolProperties {
    __padding_end: [u8; 192],
}
impl UConvertMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertMeshesToolProperties")
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
pub struct UConvertMeshesTool {
    __padding_end: [u8; 248],
}
impl UConvertMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertMeshesTool")
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
pub struct UCubeGridToolBuilder {
    __padding_end: [u8; 56],
}
impl UCubeGridToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCubeGridToolBuilder")
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
pub struct UCubeGridToolProperties {
    __padding_end: [u8; 440],
}
impl UCubeGridToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCubeGridToolProperties")
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
pub struct UCubeGridToolActions {
    __padding_end: [u8; 200],
}
impl UCubeGridToolActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCubeGridToolActions")
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
pub struct UCubeGridTool {
    __padding_end: [u8; 1664],
}
impl UCubeGridTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCubeGridTool")
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
pub struct UDrawPolyPathToolBuilder {
    __padding_end: [u8; 48],
}
impl UDrawPolyPathToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolyPathToolBuilder")
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
pub struct UDrawPolyPathProperties {
    __padding_end: [u8; 224],
}
impl UDrawPolyPathProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolyPathProperties")
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
pub struct UDrawPolyPathExtrudeProperties {
    __padding_end: [u8; 192],
}
impl UDrawPolyPathExtrudeProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolyPathExtrudeProperties")
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
pub struct UDrawPolyPathTool {
    __padding_end: [u8; 720],
}
impl UDrawPolyPathTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawPolyPathTool")
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
pub struct UEditNormalsToolBuilder {
    __padding_end: [u8; 48],
}
impl UEditNormalsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditNormalsToolBuilder")
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
pub struct UEditNormalsToolProperties {
    __padding_end: [u8; 200],
}
impl UEditNormalsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditNormalsToolProperties")
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
pub struct UEditNormalsOperatorFactory {
    __padding_end: [u8; 72],
}
impl UEditNormalsOperatorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditNormalsOperatorFactory")
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
pub struct UEditNormalsTool {
    __padding_end: [u8; 768],
}
impl UEditNormalsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditNormalsTool")
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
pub struct UEditPivotToolBuilder {
    __padding_end: [u8; 48],
}
impl UEditPivotToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditPivotToolBuilder")
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
pub struct UEditPivotToolProperties {
    __padding_end: [u8; 192],
}
impl UEditPivotToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditPivotToolProperties")
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
pub struct UEditPivotToolActionPropertySet {
    __padding_end: [u8; 200],
}
impl UEditPivotToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditPivotToolActionPropertySet")
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
pub struct UEditPivotTool {
    __padding_end: [u8; 736],
}
impl UEditPivotTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditPivotTool")
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
pub struct UEditUVIslandsToolBuilder {
    __padding_end: [u8; 56],
}
impl UEditUVIslandsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditUVIslandsToolBuilder")
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
pub struct UEditUVIslandsTool {
    __padding_end: [u8; 1280],
}
impl UEditUVIslandsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditUVIslandsTool")
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
pub struct UExtractSplineToolProperties {
    __padding_end: [u8; 192],
}
impl UExtractSplineToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractSplineToolProperties")
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
pub struct UExtractSplineTool {
    __padding_end: [u8; 400],
}
impl UExtractSplineTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractSplineTool")
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
pub struct UExtractSplineToolBuilder {
    __padding_end: [u8; 48],
}
impl UExtractSplineToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractSplineToolBuilder")
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
pub struct UMeshBoundaryToolBase {
    __padding_end: [u8; 448],
}
impl UMeshBoundaryToolBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshBoundaryToolBase")
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
pub struct UMeshInspectorToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshInspectorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshInspectorToolBuilder")
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
pub struct UMeshInspectorProperties {
    __padding_end: [u8; 208],
}
impl UMeshInspectorProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshInspectorProperties")
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
pub struct UMeshInspectorMaterialProperties {
    __padding_end: [u8; 304],
}
impl UMeshInspectorMaterialProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshInspectorMaterialProperties")
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
pub struct UMeshInspectorTool {
    __padding_end: [u8; 656],
}
impl UMeshInspectorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshInspectorTool")
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
pub struct UMeshSelectionToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshSelectionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionToolBuilder")
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
pub struct UMeshSelectionToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UMeshSelectionToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionToolActionPropertySet")
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
pub struct UMeshSelectionEditActions {
    __padding_end: [u8; 192],
}
impl UMeshSelectionEditActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionEditActions")
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
pub struct UMeshSelectionMeshEditActions {
    __padding_end: [u8; 192],
}
impl UMeshSelectionMeshEditActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionMeshEditActions")
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
pub struct UMeshSelectionToolProperties {
    __padding_end: [u8; 200],
}
impl UMeshSelectionToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionToolProperties")
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
pub struct UMeshSelectionTool {
    __padding_end: [u8; 2120],
}
impl UMeshSelectionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSelectionTool")
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
pub struct UMirrorToolBuilder {
    __padding_end: [u8; 48],
}
impl UMirrorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorToolBuilder")
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
pub struct UMirrorToolProperties {
    __padding_end: [u8; 208],
}
impl UMirrorToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorToolProperties")
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
pub struct UMirrorOperatorFactory {
    __padding_end: [u8; 72],
}
impl UMirrorOperatorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorOperatorFactory")
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
pub struct UMirrorToolActionPropertySet {
    __padding_end: [u8; 200],
}
impl UMirrorToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorToolActionPropertySet")
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
pub struct UMirrorTool {
    __padding_end: [u8; 424],
}
impl UMirrorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorTool")
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
pub struct UPatternToolBuilder {
    __padding_end: [u8; 56],
}
impl UPatternToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternToolBuilder")
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
pub struct UPatternToolSettings {
    __padding_end: [u8; 208],
}
impl UPatternToolSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternToolSettings")
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
pub struct UPatternTool_BoundingBoxSettings {
    __padding_end: [u8; 200],
}
impl UPatternTool_BoundingBoxSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_BoundingBoxSettings")
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
pub struct UPatternTool_LinearSettings {
    __padding_end: [u8; 216],
}
impl UPatternTool_LinearSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_LinearSettings")
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
pub struct UPatternTool_GridSettings {
    __padding_end: [u8; 240],
}
impl UPatternTool_GridSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_GridSettings")
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
pub struct UPatternTool_RadialSettings {
    __padding_end: [u8; 240],
}
impl UPatternTool_RadialSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_RadialSettings")
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
pub struct UPatternTool_RotationSettings {
    __padding_end: [u8; 264],
}
impl UPatternTool_RotationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_RotationSettings")
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
pub struct UPatternTool_TranslationSettings {
    __padding_end: [u8; 264],
}
impl UPatternTool_TranslationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_TranslationSettings")
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
pub struct UPatternTool_ScaleSettings {
    __padding_end: [u8; 264],
}
impl UPatternTool_ScaleSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_ScaleSettings")
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
pub struct UPatternTool_OutputSettings {
    __padding_end: [u8; 192],
}
impl UPatternTool_OutputSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool_OutputSettings")
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
pub struct UPatternTool {
    __padding_end: [u8; 1200],
}
impl UPatternTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPatternTool")
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
pub struct UPhysicsObjectToolPropertySet {
    __padding_end: [u8; 288],
}
impl UPhysicsObjectToolPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsObjectToolPropertySet")
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
pub struct UCollisionGeometryVisualizationProperties {
    __padding_end: [u8; 232],
}
impl UCollisionGeometryVisualizationProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionGeometryVisualizationProperties")
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
pub struct UExtractCollisionGeometryToolBuilder {
    __padding_end: [u8; 48],
}
impl UExtractCollisionGeometryToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractCollisionGeometryToolBuilder")
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
pub struct UExtractCollisionToolProperties {
    __padding_end: [u8; 192],
}
impl UExtractCollisionToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractCollisionToolProperties")
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
pub struct UExtractCollisionGeometryTool {
    __padding_end: [u8; 872],
}
impl UExtractCollisionGeometryTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractCollisionGeometryTool")
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
pub struct UPhysicsInspectorToolBuilder {
    __padding_end: [u8; 48],
}
impl UPhysicsInspectorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsInspectorToolBuilder")
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
pub struct UPhysicsInspectorToolProperties {
    __padding_end: [u8; 192],
}
impl UPhysicsInspectorToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsInspectorToolProperties")
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
pub struct UPhysicsInspectorTool {
    __padding_end: [u8; 304],
}
impl UPhysicsInspectorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsInspectorTool")
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
pub struct USetCollisionGeometryToolBuilder {
    __padding_end: [u8; 48],
}
impl USetCollisionGeometryToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetCollisionGeometryToolBuilder")
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
pub struct USetCollisionGeometryToolProperties {
    __padding_end: [u8; 304],
}
impl USetCollisionGeometryToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetCollisionGeometryToolProperties")
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
pub struct USetCollisionGeometryTool {
    __padding_end: [u8; 944],
}
impl USetCollisionGeometryTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetCollisionGeometryTool")
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
pub struct USimpleCollisionEditorToolBuilder {
    __padding_end: [u8; 48],
}
impl USimpleCollisionEditorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleCollisionEditorToolBuilder")
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
pub struct USimpleCollisionEditorToolActionProperties {
    __padding_end: [u8; 192],
}
impl USimpleCollisionEditorToolActionProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleCollisionEditorToolActionProperties")
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
pub struct USimpleCollisionEditorTool {
    __padding_end: [u8; 272],
}
impl USimpleCollisionEditorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleCollisionEditorTool")
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
pub struct UPlaneCutToolBuilder {
    __padding_end: [u8; 48],
}
impl UPlaneCutToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneCutToolBuilder")
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
pub struct UPlaneCutToolProperties {
    __padding_end: [u8; 200],
}
impl UPlaneCutToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneCutToolProperties")
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
pub struct UPlaneCutOperatorFactory {
    __padding_end: [u8; 72],
}
impl UPlaneCutOperatorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneCutOperatorFactory")
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
pub struct UPlaneCutTool {
    __padding_end: [u8; 464],
}
impl UPlaneCutTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneCutTool")
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
pub struct UExtrudeMeshSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl UExtrudeMeshSelectionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtrudeMeshSelectionToolBuilder")
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
pub struct UExtrudeMeshSelectionToolProperties {
    __padding_end: [u8; 256],
}
impl UExtrudeMeshSelectionToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtrudeMeshSelectionToolProperties")
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
pub struct UExtrudeMeshSelectionTool {
    __padding_end: [u8; 2232],
}
impl UExtrudeMeshSelectionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtrudeMeshSelectionTool")
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
pub struct UOffsetMeshSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl UOffsetMeshSelectionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshSelectionToolBuilder")
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
pub struct UOffsetMeshSelectionToolProperties {
    __padding_end: [u8; 240],
}
impl UOffsetMeshSelectionToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshSelectionToolProperties")
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
pub struct UOffsetMeshSelectionTool {
    __padding_end: [u8; 2216],
}
impl UOffsetMeshSelectionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetMeshSelectionTool")
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
pub struct UMeshAnalysisProperties {
    __padding_end: [u8; 216],
}
impl UMeshAnalysisProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshAnalysisProperties")
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
pub struct URevolveBoundaryToolBuilder {
    __padding_end: [u8; 48],
}
impl URevolveBoundaryToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveBoundaryToolBuilder")
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
pub struct URevolveBoundaryOperatorFactory {
    __padding_end: [u8; 64],
}
impl URevolveBoundaryOperatorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveBoundaryOperatorFactory")
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
pub struct URevolveBoundaryToolProperties {
    __padding_end: [u8; 344],
}
impl URevolveBoundaryToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveBoundaryToolProperties")
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
pub struct URevolveBoundaryTool {
    __padding_end: [u8; 560],
}
impl URevolveBoundaryTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveBoundaryTool")
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
pub struct URevolveSplineToolProperties {
    __padding_end: [u8; 376],
}
impl URevolveSplineToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveSplineToolProperties")
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
pub struct URevolveSplineToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl URevolveSplineToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveSplineToolActionPropertySet")
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
pub struct UBaseMeshFromSplinesTool {
    __padding_end: [u8; 304],
}
impl UBaseMeshFromSplinesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshFromSplinesTool")
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
pub struct URevolveSplineTool {
    __padding_end: [u8; 448],
}
impl URevolveSplineTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveSplineTool")
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
pub struct UBaseMeshFromSplinesToolBuilder {
    __padding_end: [u8; 48],
}
impl UBaseMeshFromSplinesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshFromSplinesToolBuilder")
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
pub struct URevolveSplineToolBuilder {
    __padding_end: [u8; 48],
}
impl URevolveSplineToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URevolveSplineToolBuilder")
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
pub struct USeamSculptToolBuilder {
    __padding_end: [u8; 56],
}
impl USeamSculptToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USeamSculptToolBuilder")
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
pub struct USeamSculptToolProperties {
    __padding_end: [u8; 200],
}
impl USeamSculptToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USeamSculptToolProperties")
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
pub struct USeamSculptTool {
    __padding_end: [u8; 1264],
}
impl USeamSculptTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USeamSculptTool")
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
pub struct USelfUnionMeshesToolProperties {
    __padding_end: [u8; 200],
}
impl USelfUnionMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelfUnionMeshesToolProperties")
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
pub struct USelfUnionMeshesTool {
    __padding_end: [u8; 384],
}
impl USelfUnionMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelfUnionMeshesTool")
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
pub struct USelfUnionMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl USelfUnionMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelfUnionMeshesToolBuilder")
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
pub struct USplitMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl USplitMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplitMeshesToolBuilder")
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
pub struct USplitMeshesToolProperties {
    __padding_end: [u8; 208],
}
impl USplitMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplitMeshesToolProperties")
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
pub struct USplitMeshesTool {
    __padding_end: [u8; 360],
}
impl USplitMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplitMeshesTool")
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
pub struct UTransferMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UTransferMeshToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransferMeshToolBuilder")
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
pub struct UTransferMeshToolProperties {
    __padding_end: [u8; 304],
}
impl UTransferMeshToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransferMeshToolProperties")
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
pub struct UTransferMeshTool {
    __padding_end: [u8; 248],
}
impl UTransferMeshTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransferMeshTool")
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
pub struct UTransformMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UTransformMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformMeshesToolBuilder")
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
pub struct UTransformMeshesToolProperties {
    __padding_end: [u8; 192],
}
impl UTransformMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformMeshesToolProperties")
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
pub struct UTransformMeshesTool {
    __padding_end: [u8; 464],
}
impl UTransformMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformMeshesTool")
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
pub struct UTriangulateSplinesToolProperties {
    __padding_end: [u8; 240],
}
impl UTriangulateSplinesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTriangulateSplinesToolProperties")
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
pub struct UTriangulateSplinesTool {
    __padding_end: [u8; 328],
}
impl UTriangulateSplinesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTriangulateSplinesTool")
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
pub struct UTriangulateSplinesToolBuilder {
    __padding_end: [u8; 48],
}
impl UTriangulateSplinesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTriangulateSplinesToolBuilder")
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
pub struct UUVTransferToolBuilder {
    __padding_end: [u8; 48],
}
impl UUVTransferToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVTransferToolBuilder")
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
pub struct UUVTransferToolProperties {
    __padding_end: [u8; 288],
}
impl UUVTransferToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVTransferToolProperties")
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
pub struct UUVTransferTool {
    __padding_end: [u8; 536],
}
impl UUVTransferTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVTransferTool")
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
pub struct UVolumeToMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UVolumeToMeshToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeToMeshToolBuilder")
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
pub struct UVolumeToMeshToolProperties {
    __padding_end: [u8; 192],
}
impl UVolumeToMeshToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeToMeshToolProperties")
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
pub struct UVolumeToMeshTool {
    __padding_end: [u8; 832],
}
impl UVolumeToMeshTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeToMeshTool")
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
pub struct UVoxelBlendMeshesToolProperties {
    __padding_end: [u8; 216],
}
impl UVoxelBlendMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelBlendMeshesToolProperties")
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
pub struct UVoxelBlendMeshesTool {
    __padding_end: [u8; 344],
}
impl UVoxelBlendMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelBlendMeshesTool")
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
pub struct UVoxelBlendMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UVoxelBlendMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelBlendMeshesToolBuilder")
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
pub struct UVoxelMorphologyMeshesToolProperties {
    __padding_end: [u8; 216],
}
impl UVoxelMorphologyMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelMorphologyMeshesToolProperties")
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
pub struct UVoxelMorphologyMeshesTool {
    __padding_end: [u8; 344],
}
impl UVoxelMorphologyMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelMorphologyMeshesTool")
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
pub struct UVoxelMorphologyMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UVoxelMorphologyMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelMorphologyMeshesToolBuilder")
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
pub struct UVoxelSolidifyMeshesToolProperties {
    __padding_end: [u8; 216],
}
impl UVoxelSolidifyMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelSolidifyMeshesToolProperties")
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
pub struct UVoxelSolidifyMeshesTool {
    __padding_end: [u8; 344],
}
impl UVoxelSolidifyMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelSolidifyMeshesTool")
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
pub struct UVoxelSolidifyMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UVoxelSolidifyMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelSolidifyMeshesToolBuilder")
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
pub struct EBakeNormalSpace(pub i32);
impl EBakeNormalSpace {
    pub const TANGENT: EBakeNormalSpace = EBakeNormalSpace(0);
    pub const OBJECT: EBakeNormalSpace = EBakeNormalSpace(1);
}
#[repr(transparent)]
pub struct EBakeCurvatureTypeMode(pub i32);
impl EBakeCurvatureTypeMode {
    pub const MEAN_AVERAGE: EBakeCurvatureTypeMode = EBakeCurvatureTypeMode(0);
    pub const MAX: EBakeCurvatureTypeMode = EBakeCurvatureTypeMode(1);
    pub const MIN: EBakeCurvatureTypeMode = EBakeCurvatureTypeMode(2);
    pub const GAUSSIAN: EBakeCurvatureTypeMode = EBakeCurvatureTypeMode(3);
}
#[repr(transparent)]
pub struct EBakeCurvatureColorMode(pub i32);
impl EBakeCurvatureColorMode {
    pub const GRAYSCALE: EBakeCurvatureColorMode = EBakeCurvatureColorMode(0);
    pub const RED_BLUE: EBakeCurvatureColorMode = EBakeCurvatureColorMode(1);
    pub const RED_GREEN_BLUE: EBakeCurvatureColorMode = EBakeCurvatureColorMode(2);
}
#[repr(transparent)]
pub struct EBakeCurvatureClampMode(pub i32);
impl EBakeCurvatureClampMode {
    pub const NONE: EBakeCurvatureClampMode = EBakeCurvatureClampMode(0);
    pub const ONLY_POSITIVE: EBakeCurvatureClampMode = EBakeCurvatureClampMode(1);
    pub const ONLY_NEGATIVE: EBakeCurvatureClampMode = EBakeCurvatureClampMode(2);
}
#[repr(transparent)]
pub struct EBakeHeightRangeMode(pub i32);
impl EBakeHeightRangeMode {
    pub const ABSOLUTE: EBakeHeightRangeMode = EBakeHeightRangeMode(0);
    pub const RELATIVE_BOUNDS: EBakeHeightRangeMode = EBakeHeightRangeMode(1);
}
#[repr(transparent)]
pub struct EAlignObjectsAlignTypes(pub i32);
impl EAlignObjectsAlignTypes {
    pub const PIVOTS: EAlignObjectsAlignTypes = EAlignObjectsAlignTypes(0);
    pub const BOUNDING_BOXES: EAlignObjectsAlignTypes = EAlignObjectsAlignTypes(1);
}
#[repr(transparent)]
pub struct EAlignObjectsAlignToOptions(pub i32);
impl EAlignObjectsAlignToOptions {
    pub const FIRST_SELECTED: EAlignObjectsAlignToOptions = EAlignObjectsAlignToOptions(
        0,
    );
    pub const LAST_SELECTED: EAlignObjectsAlignToOptions = EAlignObjectsAlignToOptions(
        1,
    );
    pub const COMBINED: EAlignObjectsAlignToOptions = EAlignObjectsAlignToOptions(2);
}
#[repr(transparent)]
pub struct EAlignObjectsBoxPoint(pub i32);
impl EAlignObjectsBoxPoint {
    pub const CENTER: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(0);
    pub const BOTTOM: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(1);
    pub const TOP: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(2);
    pub const LEFT: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(3);
    pub const RIGHT: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(4);
    pub const FRONT: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(5);
    pub const BACK: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(6);
    pub const MIN: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(7);
    pub const MAX: EAlignObjectsBoxPoint = EAlignObjectsBoxPoint(8);
}
#[repr(transparent)]
pub struct EBakeMapType(pub i32);
impl EBakeMapType {
    pub const NONE: EBakeMapType = EBakeMapType(0);
    pub const TANGENT_SPACE_NORMAL: EBakeMapType = EBakeMapType(1);
    pub const OBJECT_SPACE_NORMAL: EBakeMapType = EBakeMapType(2);
    pub const FACE_NORMAL: EBakeMapType = EBakeMapType(4);
    pub const BENT_NORMAL: EBakeMapType = EBakeMapType(8);
    pub const POSITION: EBakeMapType = EBakeMapType(16);
    pub const CURVATURE: EBakeMapType = EBakeMapType(32);
    pub const AMBIENT_OCCLUSION: EBakeMapType = EBakeMapType(64);
    pub const TEXTURE: EBakeMapType = EBakeMapType(128);
    pub const MULTI_TEXTURE: EBakeMapType = EBakeMapType(256);
    pub const VERTEX_COLOR: EBakeMapType = EBakeMapType(512);
    pub const MATERIAL_ID: EBakeMapType = EBakeMapType(1024);
    pub const POLY_GROUP_ID: EBakeMapType = EBakeMapType(2048);
    pub const ONE: EBakeMapType = EBakeMapType(4096);
    pub const ZERO: EBakeMapType = EBakeMapType(8192);
    pub const UV_SHELL: EBakeMapType = EBakeMapType(16384);
    pub const HEIGHT: EBakeMapType = EBakeMapType(32768);
    pub const ALL: EBakeMapType = EBakeMapType(32767);
}
#[repr(transparent)]
pub struct EBakeVertexOutput(pub i32);
impl EBakeVertexOutput {
    pub const RGBA: EBakeVertexOutput = EBakeVertexOutput(0);
    pub const PER_CHANNEL: EBakeVertexOutput = EBakeVertexOutput(1);
}
#[repr(transparent)]
pub struct EBakeVertexChannel(pub i32);
impl EBakeVertexChannel {
    pub const R: EBakeVertexChannel = EBakeVertexChannel(0);
    pub const G: EBakeVertexChannel = EBakeVertexChannel(1);
    pub const B: EBakeVertexChannel = EBakeVertexChannel(2);
    pub const A: EBakeVertexChannel = EBakeVertexChannel(3);
    pub const RGBA: EBakeVertexChannel = EBakeVertexChannel(4);
}
#[repr(transparent)]
pub struct EBakeVertexTopology(pub i32);
impl EBakeVertexTopology {
    pub const CREATE_NEW: EBakeVertexTopology = EBakeVertexTopology(0);
    pub const USE_EXISTING: EBakeVertexTopology = EBakeVertexTopology(1);
}
#[repr(transparent)]
pub struct EBakeScaleMethod(pub u8);
impl EBakeScaleMethod {
    pub const BAKE_FULL_SCALE: EBakeScaleMethod = EBakeScaleMethod(0);
    pub const BAKE_NONUNIFORM_SCALE: EBakeScaleMethod = EBakeScaleMethod(1);
    pub const DO_NOT_BAKE_SCALE: EBakeScaleMethod = EBakeScaleMethod(2);
}
#[repr(transparent)]
pub struct ECubeGridToolFaceSelectionMode(pub i32);
impl ECubeGridToolFaceSelectionMode {
    pub const OUTSIDE_BASED_ON_NORMAL: ECubeGridToolFaceSelectionMode = ECubeGridToolFaceSelectionMode(
        0,
    );
    pub const INSIDE_BASED_ON_NORMAL: ECubeGridToolFaceSelectionMode = ECubeGridToolFaceSelectionMode(
        1,
    );
    pub const OUTSIDE_BASED_ON_VIEW_RAY: ECubeGridToolFaceSelectionMode = ECubeGridToolFaceSelectionMode(
        2,
    );
    pub const INSIDE_BASED_ON_VIEW_RAY: ECubeGridToolFaceSelectionMode = ECubeGridToolFaceSelectionMode(
        3,
    );
}
#[repr(transparent)]
pub struct EDrawPolyPathWidthMode(pub i32);
impl EDrawPolyPathWidthMode {
    pub const FIXED: EDrawPolyPathWidthMode = EDrawPolyPathWidthMode(0);
    pub const INTERACTIVE: EDrawPolyPathWidthMode = EDrawPolyPathWidthMode(1);
}
#[repr(transparent)]
pub struct EDrawPolyPathRadiusMode(pub i32);
impl EDrawPolyPathRadiusMode {
    pub const FIXED: EDrawPolyPathRadiusMode = EDrawPolyPathRadiusMode(0);
    pub const INTERACTIVE: EDrawPolyPathRadiusMode = EDrawPolyPathRadiusMode(1);
}
#[repr(transparent)]
pub struct EDrawPolyPathExtrudeMode(pub i32);
impl EDrawPolyPathExtrudeMode {
    pub const FLAT: EDrawPolyPathExtrudeMode = EDrawPolyPathExtrudeMode(0);
    pub const FIXED: EDrawPolyPathExtrudeMode = EDrawPolyPathExtrudeMode(1);
    pub const INTERACTIVE: EDrawPolyPathExtrudeMode = EDrawPolyPathExtrudeMode(2);
    pub const RAMP_FIXED: EDrawPolyPathExtrudeMode = EDrawPolyPathExtrudeMode(3);
    pub const RAMP_INTERACTIVE: EDrawPolyPathExtrudeMode = EDrawPolyPathExtrudeMode(4);
}
#[repr(transparent)]
pub struct EDrawPolyPathExtrudeDirection(pub i32);
impl EDrawPolyPathExtrudeDirection {
    pub const SELECTION_NORMAL: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(
        0,
    );
    pub const WORLD_X: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(1);
    pub const WORLD_Y: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(2);
    pub const WORLD_Z: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(3);
    pub const LOCAL_X: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(4);
    pub const LOCAL_Y: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(5);
    pub const LOCAL_Z: EDrawPolyPathExtrudeDirection = EDrawPolyPathExtrudeDirection(6);
}
#[repr(transparent)]
pub struct EEditPivotSnapDragRotationMode(pub u8);
impl EEditPivotSnapDragRotationMode {
    pub const ALIGN: EEditPivotSnapDragRotationMode = EEditPivotSnapDragRotationMode(1);
    pub const ALIGN_FLIPPED: EEditPivotSnapDragRotationMode = EEditPivotSnapDragRotationMode(
        2,
    );
    pub const LAST_VALUE: EEditPivotSnapDragRotationMode = EEditPivotSnapDragRotationMode(
        3,
    );
}
#[repr(transparent)]
pub struct EExtractSplineMode(pub u8);
impl EExtractSplineMode {
    pub const PLANE_CUT: EExtractSplineMode = EExtractSplineMode(1);
    pub const OPEN_BOUNDARY: EExtractSplineMode = EExtractSplineMode(2);
    pub const POLYGROUP_LOOPS: EExtractSplineMode = EExtractSplineMode(3);
}
#[repr(transparent)]
pub struct EMeshInspectorToolDrawIndexMode(pub u8);
impl EMeshInspectorToolDrawIndexMode {
    pub const NONE: EMeshInspectorToolDrawIndexMode = EMeshInspectorToolDrawIndexMode(0);
    pub const VERTEX_ID: EMeshInspectorToolDrawIndexMode = EMeshInspectorToolDrawIndexMode(
        1,
    );
    pub const TRIANGLE_ID: EMeshInspectorToolDrawIndexMode = EMeshInspectorToolDrawIndexMode(
        2,
    );
    pub const GROUP_ID: EMeshInspectorToolDrawIndexMode = EMeshInspectorToolDrawIndexMode(
        3,
    );
    pub const EDGE_ID: EMeshInspectorToolDrawIndexMode = EMeshInspectorToolDrawIndexMode(
        4,
    );
}
#[repr(transparent)]
pub struct EMeshInspectorMaterialMode(pub u8);
impl EMeshInspectorMaterialMode {
    pub const ORIGINAL: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(0);
    pub const FLAT_SHADED: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(1);
    pub const GREY: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(2);
    pub const TRANSPARENT: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(3);
    pub const TANGENT_NORMAL: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(4);
    pub const VERTEX_COLOR: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(5);
    pub const GROUP_COLOR: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(6);
    pub const CHECKERBOARD: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(7);
    pub const OVERRIDE: EMeshInspectorMaterialMode = EMeshInspectorMaterialMode(8);
}
#[repr(transparent)]
pub struct EMeshSelectionToolPrimaryMode(pub i32);
impl EMeshSelectionToolPrimaryMode {
    pub const BRUSH: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(0);
    pub const VOLUMETRIC_BRUSH: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        1,
    );
    pub const ANGLE_FILTERED: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        2,
    );
    pub const VISIBLE: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(3);
    pub const ALL_CONNECTED: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        4,
    );
    pub const ALL_IN_GROUP: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        5,
    );
    pub const BY_MATERIAL: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        6,
    );
    pub const BY_MATERIAL_ALL: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        7,
    );
    pub const BY_UV_ISLAND: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        8,
    );
    pub const ALL_WITHIN_ANGLE: EMeshSelectionToolPrimaryMode = EMeshSelectionToolPrimaryMode(
        9,
    );
}
#[repr(transparent)]
pub struct EMeshFacesColorMode(pub i32);
impl EMeshFacesColorMode {
    pub const NONE: EMeshFacesColorMode = EMeshFacesColorMode(0);
    pub const BY_GROUP: EMeshFacesColorMode = EMeshFacesColorMode(1);
    pub const BY_MATERIAL_ID: EMeshFacesColorMode = EMeshFacesColorMode(2);
    pub const BY_UV_ISLAND: EMeshFacesColorMode = EMeshFacesColorMode(3);
}
#[repr(transparent)]
pub struct EMirrorOperationMode(pub u8);
impl EMirrorOperationMode {
    pub const MIRROR_AND_APPEND: EMirrorOperationMode = EMirrorOperationMode(0);
    pub const MIRROR_EXISTING: EMirrorOperationMode = EMirrorOperationMode(1);
}
#[repr(transparent)]
pub struct EMeshMirrorWeldNormalMode(pub u8);
impl EMeshMirrorWeldNormalMode {
    pub const MIRROR_NORMALS: EMeshMirrorWeldNormalMode = EMeshMirrorWeldNormalMode(0);
    pub const AVERAGE_MIRROR_NORMALS: EMeshMirrorWeldNormalMode = EMeshMirrorWeldNormalMode(
        1,
    );
}
#[repr(transparent)]
pub struct EMirrorSaveMode(pub u8);
impl EMirrorSaveMode {
    pub const INPUT_OBJECTS: EMirrorSaveMode = EMirrorSaveMode(0);
    pub const NEW_OBJECTS: EMirrorSaveMode = EMirrorSaveMode(1);
}
#[repr(transparent)]
pub struct EPatternToolShape(pub u8);
impl EPatternToolShape {
    pub const LINE: EPatternToolShape = EPatternToolShape(0);
    pub const GRID: EPatternToolShape = EPatternToolShape(1);
    pub const CIRCLE: EPatternToolShape = EPatternToolShape(2);
}
#[repr(transparent)]
pub struct EPatternToolSingleAxis(pub u8);
impl EPatternToolSingleAxis {
    pub const X_AXIS: EPatternToolSingleAxis = EPatternToolSingleAxis(0);
    pub const Y_AXIS: EPatternToolSingleAxis = EPatternToolSingleAxis(1);
    pub const Z_AXIS: EPatternToolSingleAxis = EPatternToolSingleAxis(2);
}
#[repr(transparent)]
pub struct EPatternToolSinglePlane(pub u8);
impl EPatternToolSinglePlane {
    pub const XY_PLANE: EPatternToolSinglePlane = EPatternToolSinglePlane(0);
    pub const XZ_PLANE: EPatternToolSinglePlane = EPatternToolSinglePlane(1);
    pub const YZ_PLANE: EPatternToolSinglePlane = EPatternToolSinglePlane(2);
}
#[repr(transparent)]
pub struct EPatternToolAxisSpacingMode(pub u8);
impl EPatternToolAxisSpacingMode {
    pub const BY_COUNT: EPatternToolAxisSpacingMode = EPatternToolAxisSpacingMode(0);
    pub const STEP_SIZE: EPatternToolAxisSpacingMode = EPatternToolAxisSpacingMode(1);
    pub const PACKED: EPatternToolAxisSpacingMode = EPatternToolAxisSpacingMode(2);
}
#[repr(transparent)]
pub struct ECollisionGeometryMode(pub i32);
impl ECollisionGeometryMode {
    pub const DEFAULT: ECollisionGeometryMode = ECollisionGeometryMode(0);
    pub const SIMPLE_AND_COMPLEX: ECollisionGeometryMode = ECollisionGeometryMode(1);
    pub const USE_SIMPLE_AS_COMPLEX: ECollisionGeometryMode = ECollisionGeometryMode(2);
    pub const USE_COMPLEX_AS_SIMPLE: ECollisionGeometryMode = ECollisionGeometryMode(3);
}
#[repr(transparent)]
pub struct EExtractCollisionOutputType(pub u8);
impl EExtractCollisionOutputType {
    pub const SIMPLE: EExtractCollisionOutputType = EExtractCollisionOutputType(0);
    pub const COMPLEX: EExtractCollisionOutputType = EExtractCollisionOutputType(1);
}
#[repr(transparent)]
pub struct ECollisionGeometryType(pub i32);
impl ECollisionGeometryType {
    pub const COPY_FROM_INPUTS: ECollisionGeometryType = ECollisionGeometryType(0);
    pub const ALIGNED_BOXES: ECollisionGeometryType = ECollisionGeometryType(1);
    pub const ORIENTED_BOXES: ECollisionGeometryType = ECollisionGeometryType(2);
    pub const MINIMAL_SPHERES: ECollisionGeometryType = ECollisionGeometryType(3);
    pub const CAPSULES: ECollisionGeometryType = ECollisionGeometryType(4);
    pub const CONVEX_HULLS: ECollisionGeometryType = ECollisionGeometryType(5);
    pub const CONVEX_DECOMPOSITIONS: ECollisionGeometryType = ECollisionGeometryType(8);
    pub const SWEPT_HULLS: ECollisionGeometryType = ECollisionGeometryType(6);
    pub const LEVEL_SETS: ECollisionGeometryType = ECollisionGeometryType(7);
    pub const MIN_VOLUME: ECollisionGeometryType = ECollisionGeometryType(10);
    pub const EMPTY: ECollisionGeometryType = ECollisionGeometryType(11);
}
#[repr(transparent)]
pub struct ESetCollisionGeometryInputMode(pub i32);
impl ESetCollisionGeometryInputMode {
    pub const COMBINE_ALL: ESetCollisionGeometryInputMode = ESetCollisionGeometryInputMode(
        0,
    );
    pub const PER_INPUT_OBJECT: ESetCollisionGeometryInputMode = ESetCollisionGeometryInputMode(
        1,
    );
    pub const PER_MESH_COMPONENT: ESetCollisionGeometryInputMode = ESetCollisionGeometryInputMode(
        2,
    );
    pub const PER_MESH_GROUP: ESetCollisionGeometryInputMode = ESetCollisionGeometryInputMode(
        3,
    );
}
#[repr(transparent)]
pub struct EConvexDecompositionMethod(pub i32);
impl EConvexDecompositionMethod {
    pub const NAVIGATION_DRIVEN: EConvexDecompositionMethod = EConvexDecompositionMethod(
        0,
    );
    pub const VOLUMETRIC_ERROR: EConvexDecompositionMethod = EConvexDecompositionMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EProjectedHullAxis(pub i32);
impl EProjectedHullAxis {
    pub const X: EProjectedHullAxis = EProjectedHullAxis(0);
    pub const Y: EProjectedHullAxis = EProjectedHullAxis(1);
    pub const Z: EProjectedHullAxis = EProjectedHullAxis(2);
    pub const SMALLEST_BOX_DIMENSION: EProjectedHullAxis = EProjectedHullAxis(3);
    pub const SMALLEST_VOLUME: EProjectedHullAxis = EProjectedHullAxis(4);
}
#[repr(transparent)]
pub struct EExtrudeMeshSelectionInteractionMode(pub u8);
impl EExtrudeMeshSelectionInteractionMode {
    pub const INTERACTIVE: EExtrudeMeshSelectionInteractionMode = EExtrudeMeshSelectionInteractionMode(
        0,
    );
    pub const FIXED: EExtrudeMeshSelectionInteractionMode = EExtrudeMeshSelectionInteractionMode(
        1,
    );
}
#[repr(transparent)]
pub struct EExtrudeMeshSelectionRegionModifierMode(pub u8);
impl EExtrudeMeshSelectionRegionModifierMode {
    pub const ORIGINAL_SHAPE: EExtrudeMeshSelectionRegionModifierMode = EExtrudeMeshSelectionRegionModifierMode(
        0,
    );
    pub const FLATTEN_TO_PLANE: EExtrudeMeshSelectionRegionModifierMode = EExtrudeMeshSelectionRegionModifierMode(
        1,
    );
    pub const RAYCAST_TO_PLANE: EExtrudeMeshSelectionRegionModifierMode = EExtrudeMeshSelectionRegionModifierMode(
        2,
    );
}
#[repr(transparent)]
pub struct EOffsetMeshSelectionDirectionMode(pub u8);
impl EOffsetMeshSelectionDirectionMode {
    pub const VERTEX_NORMALS: EOffsetMeshSelectionDirectionMode = EOffsetMeshSelectionDirectionMode(
        0,
    );
    pub const FACE_NORMALS: EOffsetMeshSelectionDirectionMode = EOffsetMeshSelectionDirectionMode(
        1,
    );
    pub const CONSTANT_WIDTH: EOffsetMeshSelectionDirectionMode = EOffsetMeshSelectionDirectionMode(
        2,
    );
}
#[repr(transparent)]
pub struct ERevolveSplineSampleMode(pub u8);
impl ERevolveSplineSampleMode {
    pub const CONTROL_POINTS_ONLY: ERevolveSplineSampleMode = ERevolveSplineSampleMode(
        0,
    );
    pub const POLY_LINE_MAX_ERROR: ERevolveSplineSampleMode = ERevolveSplineSampleMode(
        1,
    );
    pub const UNIFORM_SPACING_ALONG_CURVE: ERevolveSplineSampleMode = ERevolveSplineSampleMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESplitMeshesMethod(pub u8);
impl ESplitMeshesMethod {
    pub const BY_MESH_TOPOLOGY: ESplitMeshesMethod = ESplitMeshesMethod(0);
    pub const BY_VERTEX_OVERLAP: ESplitMeshesMethod = ESplitMeshesMethod(1);
    pub const BY_MATERIAL_ID: ESplitMeshesMethod = ESplitMeshesMethod(2);
    pub const BY_POLY_GROUP: ESplitMeshesMethod = ESplitMeshesMethod(3);
}
#[repr(transparent)]
pub struct ETransformMeshesTransformMode(pub u8);
impl ETransformMeshesTransformMode {
    pub const SHARED_GIZMO: ETransformMeshesTransformMode = ETransformMeshesTransformMode(
        0,
    );
    pub const SHARED_GIZMO_LOCAL: ETransformMeshesTransformMode = ETransformMeshesTransformMode(
        1,
    );
    pub const PER_OBJECT_GIZMO: ETransformMeshesTransformMode = ETransformMeshesTransformMode(
        2,
    );
    pub const LAST_VALUE: ETransformMeshesTransformMode = ETransformMeshesTransformMode(
        3,
    );
}
#[repr(transparent)]
pub struct ETransformMeshesSnapDragSource(pub u8);
impl ETransformMeshesSnapDragSource {
    pub const CLICK_POINT: ETransformMeshesSnapDragSource = ETransformMeshesSnapDragSource(
        0,
    );
    pub const PIVOT: ETransformMeshesSnapDragSource = ETransformMeshesSnapDragSource(1);
    pub const LAST_VALUE: ETransformMeshesSnapDragSource = ETransformMeshesSnapDragSource(
        2,
    );
}
#[repr(transparent)]
pub struct ETransformMeshesSnapDragRotationMode(pub u8);
impl ETransformMeshesSnapDragRotationMode {
    pub const IGNORE: ETransformMeshesSnapDragRotationMode = ETransformMeshesSnapDragRotationMode(
        0,
    );
    pub const ALIGN: ETransformMeshesSnapDragRotationMode = ETransformMeshesSnapDragRotationMode(
        1,
    );
    pub const ALIGN_FLIPPED: ETransformMeshesSnapDragRotationMode = ETransformMeshesSnapDragRotationMode(
        2,
    );
    pub const LAST_VALUE: ETransformMeshesSnapDragRotationMode = ETransformMeshesSnapDragRotationMode(
        3,
    );
}
#[repr(transparent)]
pub struct EVoxelBlendOperation(pub u8);
impl EVoxelBlendOperation {
    pub const UNION: EVoxelBlendOperation = EVoxelBlendOperation(0);
    pub const SUBTRACT: EVoxelBlendOperation = EVoxelBlendOperation(1);
}
