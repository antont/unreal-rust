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
    pub a_landmass_actor_update_brush_extents: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_set_mesh_exents_material: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_set_editor_tick_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_restore_landscape_editing: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_render_layer_native: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_render_layer: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_move_to_top: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_move_to_bottom: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_move_brush_up: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_move_brush_down: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_fast_preview_mode: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_draw_brush_material: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_custom_tick: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_actor_actor_selection_changed: *mut crate::ffi::UFunctionOpague,
    pub u_landmass_blueprint_function_library_world_extents_to_canvas_coordinates: *mut crate::ffi::UFunctionOpague,
    pub u_landmass_blueprint_function_library_get_cursor_world_ray: *mut crate::ffi::UFunctionOpague,
    pub u_landmass_blueprint_function_library_force_update_texture: *mut crate::ffi::UFunctionOpague,
    pub u_landmass_blueprint_function_library_combine_world_extents: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_erosion_brush_base_set_target_landscape: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_erosion_brush_base_get_landscape: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_erosion_brush_base_actor_selection_changed: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_update_child_data_counts: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_toggle_preview_mode: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_sort_brushes: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_set_target_landscape: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_set_capture_boundary_normals: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_request_update_from_brush: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_remove_brush_from_tree: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_populate_node_tree: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_launch_landmass_editor: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_get_nodes_within_extents: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_get_landscape: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_get_actors_within_modified_nodes: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_draw_brush_material: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_consolidate_nodes: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_add_brush_to_tree: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_add_brush_to_array: *mut crate::ffi::UFunctionOpague,
    pub a_landmass_manager_base_actor_selection_changed: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_landmass_actor_update_brush_extents: std::ptr::null_mut(),
            a_landmass_actor_set_mesh_exents_material: std::ptr::null_mut(),
            a_landmass_actor_set_editor_tick_enabled: std::ptr::null_mut(),
            a_landmass_actor_restore_landscape_editing: std::ptr::null_mut(),
            a_landmass_actor_render_layer_native: std::ptr::null_mut(),
            a_landmass_actor_render_layer: std::ptr::null_mut(),
            a_landmass_actor_move_to_top: std::ptr::null_mut(),
            a_landmass_actor_move_to_bottom: std::ptr::null_mut(),
            a_landmass_actor_move_brush_up: std::ptr::null_mut(),
            a_landmass_actor_move_brush_down: std::ptr::null_mut(),
            a_landmass_actor_fast_preview_mode: std::ptr::null_mut(),
            a_landmass_actor_draw_brush_material: std::ptr::null_mut(),
            a_landmass_actor_custom_tick: std::ptr::null_mut(),
            a_landmass_actor_actor_selection_changed: std::ptr::null_mut(),
            u_landmass_blueprint_function_library_world_extents_to_canvas_coordinates: std::ptr::null_mut(),
            u_landmass_blueprint_function_library_get_cursor_world_ray: std::ptr::null_mut(),
            u_landmass_blueprint_function_library_force_update_texture: std::ptr::null_mut(),
            u_landmass_blueprint_function_library_combine_world_extents: std::ptr::null_mut(),
            a_landmass_erosion_brush_base_set_target_landscape: std::ptr::null_mut(),
            a_landmass_erosion_brush_base_get_landscape: std::ptr::null_mut(),
            a_landmass_erosion_brush_base_actor_selection_changed: std::ptr::null_mut(),
            a_landmass_manager_base_update_child_data_counts: std::ptr::null_mut(),
            a_landmass_manager_base_toggle_preview_mode: std::ptr::null_mut(),
            a_landmass_manager_base_sort_brushes: std::ptr::null_mut(),
            a_landmass_manager_base_set_target_landscape: std::ptr::null_mut(),
            a_landmass_manager_base_set_capture_boundary_normals: std::ptr::null_mut(),
            a_landmass_manager_base_request_update_from_brush: std::ptr::null_mut(),
            a_landmass_manager_base_remove_brush_from_tree: std::ptr::null_mut(),
            a_landmass_manager_base_populate_node_tree: std::ptr::null_mut(),
            a_landmass_manager_base_launch_landmass_editor: std::ptr::null_mut(),
            a_landmass_manager_base_get_nodes_within_extents: std::ptr::null_mut(),
            a_landmass_manager_base_get_landscape: std::ptr::null_mut(),
            a_landmass_manager_base_get_actors_within_modified_nodes: std::ptr::null_mut(),
            a_landmass_manager_base_draw_brush_material: std::ptr::null_mut(),
            a_landmass_manager_base_consolidate_nodes: std::ptr::null_mut(),
            a_landmass_manager_base_add_brush_to_tree: std::ptr::null_mut(),
            a_landmass_manager_base_add_brush_to_array: std::ptr::null_mut(),
            a_landmass_manager_base_actor_selection_changed: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ALandmassActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateBrushExtents"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_update_brush_extents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMeshExentsMaterial"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_set_mesh_exents_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEditorTickEnabled"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_set_editor_tick_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestoreLandscapeEditing"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_restore_landscape_editing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenderLayer_Native"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_render_layer_native,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenderLayer"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_render_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveToTop"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_move_to_top,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveToBottom"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_move_to_bottom,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveBrushUp"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_move_brush_up,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveBrushDown"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_move_brush_down,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FastPreviewMode"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_fast_preview_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DrawBrushMaterial"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_draw_brush_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CustomTick"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_custom_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActorSelectionChanged"),
                &raw mut __FUNCTION_PTRS.a_landmass_actor_actor_selection_changed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULandmassBlueprintFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WorldExtentsToCanvasCoordinates"),
                &raw mut __FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_world_extents_to_canvas_coordinates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCursorWorldRay"),
                &raw mut __FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_get_cursor_world_ray,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForceUpdateTexture"),
                &raw mut __FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_force_update_texture,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CombineWorldExtents"),
                &raw mut __FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_combine_world_extents,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ALandmassErosionBrushBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTargetLandscape"),
                &raw mut __FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_set_target_landscape,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLandscape"),
                &raw mut __FUNCTION_PTRS.a_landmass_erosion_brush_base_get_landscape,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActorSelectionChanged"),
                &raw mut __FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_actor_selection_changed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ALandmassManagerBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateChildDataCounts"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_update_child_data_counts,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TogglePreviewMode"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_toggle_preview_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SortBrushes"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_sort_brushes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTargetLandscape"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_set_target_landscape,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCaptureBoundaryNormals"),
                &raw mut __FUNCTION_PTRS
                    .a_landmass_manager_base_set_capture_boundary_normals,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestUpdateFromBrush"),
                &raw mut __FUNCTION_PTRS
                    .a_landmass_manager_base_request_update_from_brush,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveBrushFromTree"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_remove_brush_from_tree,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PopulateNodeTree"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_populate_node_tree,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LaunchLandmassEditor"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_launch_landmass_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodesWithinExtents"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_get_nodes_within_extents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLandscape"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_get_landscape,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsWithinModifiedNodes"),
                &raw mut __FUNCTION_PTRS
                    .a_landmass_manager_base_get_actors_within_modified_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DrawBrushMaterial"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_draw_brush_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConsolidateNodes"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_consolidate_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBrushToTree"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_add_brush_to_tree,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBrushToArray"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_add_brush_to_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActorSelectionChanged"),
                &raw mut __FUNCTION_PTRS.a_landmass_manager_base_actor_selection_changed,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct FBrushDataTree {
    pub current_level: i32,
    pub parent_index: i32,
    pub brush_actors: TArray<UPtr<ALandmassActor>>,
    pub index_x0y0: i32,
    pub index_x1y0: i32,
    pub index_x0y1: i32,
    pub index_x1y1: i32,
    pub child_data_count: i32,
    pub node_extents: crate::bindings::core_u_object::FVector4,
}
impl FBrushDataTree {}
#[repr(C, align(16))]
pub struct FLandmassLandscapeInfo {
    pub landscape_transform: crate::bindings::core_u_object::FTransform,
    pub landscape_quads: crate::bindings::core_u_object::FIntPoint,
    pub render_target_resolution: crate::bindings::core_u_object::FIntPoint,
}
impl FLandmassLandscapeInfo {}
#[repr(C, align(16))]
pub struct ALandmassActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub brush_size: f32,
    pub draw_to_entire_landscape: bool,
    pub affects_heightmap: bool,
    pub affects_weightmaps: bool,
    pub affects_visibility: bool,
    pub height_blend_mode: EBrushBlendMode,
    pub height_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub weight_map_blend_mode: EBrushBlendMode,
    pub weightmap_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub brush_extents: crate::bindings::core_u_object::FVector4,
    pub weightmap_layers: TArray<FName>,
    pub brush_render_parameters: crate::bindings::landscape::FLandscapeBrushParameters,
    pub brush_manager: UPtr<ALandmassManagerBase>,
    #[doc(hidden)]
    pub(crate) __padding_1400: [u8; 32],
    pub extents_preview_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub heightmap_render_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    pub weightmap_render_mid: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
    __padding_end: [u8; 32],
}
impl ALandmassActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassActor")
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
    pub fn update_brush_extents(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_update_brush_extents,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_update_brush_extents,
                __buffer,
            )
        };
    }
    pub fn set_mesh_exents_material(
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_set_mesh_exents_material,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_set_mesh_exents_material,
                __buffer,
            )
        };
    }
    pub fn set_editor_tick_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_set_editor_tick_enabled,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_set_editor_tick_enabled,
                __buffer,
            )
        };
    }
    pub fn restore_landscape_editing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_restore_landscape_editing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_restore_landscape_editing,
                __buffer,
            )
        };
    }
    pub fn render_layer_native(
        &mut self,
        in_parameters: &crate::bindings::landscape::FLandscapeBrushParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_render_layer_native,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameters,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::landscape::FLandscapeBrushParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_render_layer_native,
                __buffer,
            )
        };
    }
    pub fn render_layer(
        &mut self,
        in_parameters: &crate::bindings::landscape::FLandscapeBrushParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_render_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameters,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::landscape::FLandscapeBrushParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_render_layer,
                __buffer,
            )
        };
    }
    pub fn move_to_top(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_to_top,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_to_top,
                __buffer,
            )
        };
    }
    pub fn move_to_bottom(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_to_bottom,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_to_bottom,
                __buffer,
            )
        };
    }
    pub fn move_brush_up(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_brush_up,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_brush_up,
                __buffer,
            )
        };
    }
    pub fn move_brush_down(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_brush_down,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_move_brush_down,
                __buffer,
            )
        };
    }
    pub fn fast_preview_mode(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_fast_preview_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_fast_preview_mode,
                __buffer,
            )
        };
    }
    pub fn draw_brush_material(
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_draw_brush_material,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_draw_brush_material,
                __buffer,
            )
        };
    }
    pub fn custom_tick(&mut self, delta_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_custom_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_custom_tick,
                __buffer,
            )
        };
    }
    pub fn actor_selection_changed(&mut self, b_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_actor_selection_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_selected,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_actor_actor_selection_changed,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ULandmassBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl ULandmassBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandmassBlueprintFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandmassBlueprintFunctionLibrary")
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
    pub fn world_extents_to_canvas_coordinates(
        world_extents: crate::bindings::core_u_object::FVector4,
        landscape_info: FLandmassLandscapeInfo,
        screen_position: &mut crate::bindings::core_u_object::FVector2D,
        screen_size: &mut crate::bindings::core_u_object::FVector2D,
        coordinate_position: &mut crate::bindings::core_u_object::FVector2D,
        coordinate_size: &mut crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_world_extents_to_canvas_coordinates,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_extents,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &landscape_info,
                __buffer.add(32).cast::<FLandmassLandscapeInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                screen_position,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                screen_size,
                __buffer.add(160).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                coordinate_position,
                __buffer.add(176).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                coordinate_size,
                __buffer.add(192).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::landmass_editor::ULandmassBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_world_extents_to_canvas_coordinates,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(144)
                .cast::<crate::bindings::core_u_object::FVector2D>()
                .swap(screen_position);
        }
        unsafe {
            __buffer
                .add(160)
                .cast::<crate::bindings::core_u_object::FVector2D>()
                .swap(screen_size);
        }
        unsafe {
            __buffer
                .add(176)
                .cast::<crate::bindings::core_u_object::FVector2D>()
                .swap(coordinate_position);
        }
        unsafe {
            __buffer
                .add(192)
                .cast::<crate::bindings::core_u_object::FVector2D>()
                .swap(coordinate_size);
        }
    }
    pub fn get_cursor_world_ray(
        camera_location: &mut crate::bindings::core_u_object::FVector,
        ray_origin: &mut crate::bindings::core_u_object::FVector,
        ray_direction: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_get_cursor_world_ray,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ray_origin,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ray_direction,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::landmass_editor::ULandmassBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_get_cursor_world_ray,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(camera_location);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(ray_origin);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(ray_direction);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn force_update_texture(in_texture: UPtr<crate::bindings::engine::UTexture>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_force_update_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_texture,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::landmass_editor::ULandmassBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_force_update_texture,
                __buffer,
            )
        };
    }
    pub fn combine_world_extents(
        extents_a: crate::bindings::core_u_object::FVector4,
        extents_b: crate::bindings::core_u_object::FVector4,
        combined_extents: &mut crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_combine_world_extents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &extents_a,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &extents_b,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                combined_extents,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::landmass_editor::ULandmassBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .u_landmass_blueprint_function_library_combine_world_extents,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::core_u_object::FVector4>()
                .swap(combined_extents);
        }
    }
}
#[repr(C, align(16))]
pub struct ALandmassErosionBrushBase {
    __padding_end: [u8; 1344],
}
impl ALandmassErosionBrushBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassErosionBrushBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassErosionBrushBase")
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
    pub fn set_target_landscape(
        &mut self,
        in_owning_landscape: UPtr<crate::bindings::landscape::ALandscape>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_set_target_landscape,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_owning_landscape,
                __buffer.add(0).cast::<UPtr<crate::bindings::landscape::ALandscape>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_set_target_landscape,
                __buffer,
            )
        };
    }
    pub fn get_landscape(&mut self) -> UPtr<crate::bindings::landscape::ALandscape> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_get_landscape,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_get_landscape,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::landscape::ALandscape>>().read()
        }
    }
    pub fn actor_selection_changed(&mut self, b_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_actor_selection_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_selected,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_erosion_brush_base_actor_selection_changed,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct ALandmassManagerBase {
    #[doc(hidden)]
    pub(crate) __padding_1312: [u8; 1312],
    pub brush_node_data: TArray<FBrushDataTree>,
    pub landscape_information: FLandmassLandscapeInfo,
    pub brush_tree_depth: i32,
    pub landmass_brushes: TArray<UPtr<ALandmassActor>>,
    __padding_end: [u8; 24],
}
impl ALandmassManagerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassManagerBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandmassManagerBase")
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
    pub fn update_child_data_counts(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_update_child_data_counts,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_update_child_data_counts,
                __buffer,
            )
        };
    }
    pub fn toggle_preview_mode(&mut self, b_enable_preview_mode: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_toggle_preview_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_preview_mode,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_toggle_preview_mode,
                __buffer,
            )
        };
    }
    pub fn sort_brushes(
        &mut self,
        brush_array_to_match: TArray<UPtr<ALandmassActor>>,
        actors_to_sort: TArray<UPtr<ALandmassActor>>,
    ) -> TArray<UPtr<ALandmassActor>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_sort_brushes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_array_to_match,
                __buffer.add(0).cast::<TArray<UPtr<ALandmassActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actors_to_sort,
                __buffer.add(16).cast::<TArray<UPtr<ALandmassActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_sort_brushes,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<UPtr<ALandmassActor>>>().read() }
    }
    pub fn set_target_landscape(
        &mut self,
        in_owning_landscape: UPtr<crate::bindings::landscape::ALandscape>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_set_target_landscape,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_owning_landscape,
                __buffer.add(0).cast::<UPtr<crate::bindings::landscape::ALandscape>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_set_target_landscape,
                __buffer,
            )
        };
    }
    pub fn set_capture_boundary_normals(&mut self, b_in_capture_boundary_normals: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_set_capture_boundary_normals,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_capture_boundary_normals,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_set_capture_boundary_normals,
                __buffer,
            )
        };
    }
    pub fn request_update_from_brush(
        &mut self,
        brush_requesting_update: UPtr<ALandmassActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_request_update_from_brush,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_requesting_update,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_request_update_from_brush,
                __buffer,
            )
        };
    }
    pub fn remove_brush_from_tree(
        &mut self,
        brush_to_remove: UPtr<ALandmassActor>,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_remove_brush_from_tree,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_to_remove,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_remove_brush_from_tree,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<i32>>().read() }
    }
    pub fn populate_node_tree(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_populate_node_tree,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_populate_node_tree,
                __buffer,
            )
        };
    }
    pub fn launch_landmass_editor(
        &mut self,
        brush_requesting_editor: UPtr<ALandmassActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_launch_landmass_editor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_requesting_editor,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_launch_landmass_editor,
                __buffer,
            )
        };
    }
    pub fn get_nodes_within_extents(
        &mut self,
        in_extents: &mut crate::bindings::core_u_object::FVector4,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_nodes_within_extents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_extents,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_nodes_within_extents,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector4>()
                .swap(in_extents);
        }
        unsafe { __buffer.add(32).cast::<TArray<i32>>().read() }
    }
    pub fn get_landscape(&mut self) -> UPtr<crate::bindings::landscape::ALandscape> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_landscape,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_landscape,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::landscape::ALandscape>>().read()
        }
    }
    pub fn get_actors_within_modified_nodes(
        &mut self,
        in_modified_nodes: &mut TArray<i32>,
    ) -> TArray<UPtr<ALandmassActor>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_actors_within_modified_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_modified_nodes,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_get_actors_within_modified_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(in_modified_nodes);
        }
        unsafe { __buffer.add(16).cast::<TArray<UPtr<ALandmassActor>>>().read() }
    }
    pub fn draw_brush_material(
        &mut self,
        brush: UPtr<ALandmassActor>,
        brush_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_draw_brush_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_material,
                __buffer
                    .add(8)
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_draw_brush_material,
                __buffer,
            )
        };
    }
    pub fn consolidate_nodes(&mut self, nodes_to_consolidate: &mut TArray<i32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_consolidate_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                nodes_to_consolidate,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_consolidate_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(nodes_to_consolidate);
        }
    }
    pub fn add_brush_to_tree(
        &mut self,
        brush_to_add: UPtr<ALandmassActor>,
        in_extents: crate::bindings::core_u_object::FVector4,
        in_map_to_whole_landscape: bool,
        modified_extents: &mut crate::bindings::core_u_object::FVector4,
        invalidated_brushes: &mut TArray<UPtr<ALandmassActor>>,
        modified_nodes: &mut TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_add_brush_to_tree,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_to_add,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extents,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_map_to_whole_landscape,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modified_extents,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                invalidated_brushes,
                __buffer.add(96).cast::<TArray<UPtr<ALandmassActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                modified_nodes,
                __buffer.add(112).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_add_brush_to_tree,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(64)
                .cast::<crate::bindings::core_u_object::FVector4>()
                .swap(modified_extents);
        }
        unsafe {
            __buffer
                .add(96)
                .cast::<TArray<UPtr<ALandmassActor>>>()
                .swap(invalidated_brushes);
        }
        unsafe {
            __buffer.add(112).cast::<TArray<i32>>().swap(modified_nodes);
        }
    }
    pub fn add_brush_to_array(&mut self, brush_to_add: UPtr<ALandmassActor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_add_brush_to_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &brush_to_add,
                __buffer.add(0).cast::<UPtr<ALandmassActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_add_brush_to_array,
                __buffer,
            )
        };
    }
    pub fn actor_selection_changed(&mut self, b_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_actor_selection_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_selected,
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
                crate::bindings::landmass_editor::__FUNCTION_PTRS
                    .a_landmass_manager_base_actor_selection_changed,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FLandmassActor_OnBrushUpdated {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBrushBlendMode(pub u8);
impl EBrushBlendMode {
    pub const ALPHA_BLEND: EBrushBlendMode = EBrushBlendMode(0);
    pub const MIN: EBrushBlendMode = EBrushBlendMode(1);
    pub const MAX: EBrushBlendMode = EBrushBlendMode(2);
    pub const ADDITIVE: EBrushBlendMode = EBrushBlendMode(3);
}
