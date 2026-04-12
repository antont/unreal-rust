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
    pub u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current_surface: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_mesh_weight_map_paint_tool_actions_flood_fill_current: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_mesh_weight_map_paint_tool_actions_clear_all: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_blueprint_library_set_dataflow_node_property: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_blueprint_library_connect_dataflow_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_blueprint_library_add_dataflow_node: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_editor_blueprint_library_add_dataflow_from_clipboard_content: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_simulation_scene_description_new_geometry_cache: *mut crate::ffi::UFunctionOpague,
    pub u_dataflow_simulation_scene_description_generate_geometry_cache: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current_surface: std::ptr::null_mut(),
            u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current: std::ptr::null_mut(),
            u_dataflow_editor_mesh_weight_map_paint_tool_actions_flood_fill_current: std::ptr::null_mut(),
            u_dataflow_editor_mesh_weight_map_paint_tool_actions_clear_all: std::ptr::null_mut(),
            u_dataflow_editor_blueprint_library_set_dataflow_node_property: std::ptr::null_mut(),
            u_dataflow_editor_blueprint_library_connect_dataflow_nodes: std::ptr::null_mut(),
            u_dataflow_editor_blueprint_library_add_dataflow_node: std::ptr::null_mut(),
            u_dataflow_editor_blueprint_library_add_dataflow_from_clipboard_content: std::ptr::null_mut(),
            u_dataflow_simulation_scene_description_new_geometry_cache: std::ptr::null_mut(),
            u_dataflow_simulation_scene_description_generate_geometry_cache: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataflowEditorMeshWeightMapPaintToolActions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InvertCurrentSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InvertCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_mesh_weight_map_paint_tool_actions_invert_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FloodFillCurrent"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_mesh_weight_map_paint_tool_actions_flood_fill_current,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAll"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_mesh_weight_map_paint_tool_actions_clear_all,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataflowEditorBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDataflowNodeProperty"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_set_dataflow_node_property,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectDataflowNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_connect_dataflow_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDataflowNode"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDataflowFromClipboardContent"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_from_clipboard_content,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDataflowSimulationSceneDescription::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NewGeometryCache"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_simulation_scene_description_new_geometry_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateGeometryCache"),
                &raw mut __FUNCTION_PTRS
                    .u_dataflow_simulation_scene_description_generate_geometry_cache,
            );
        }
    }
}
pub struct IDataflowEditorToolBuilder {}
#[repr(C, align(8))]
pub struct UDataflowEditorToolBuilder {
    __padding_end: [u8; 48],
}
impl UDataflowEditorToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorToolBuilder")
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
pub struct UDataflowEditorSettings {
    __padding_end: [u8; 48],
}
impl UDataflowEditorSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSettings")
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
pub struct UDataflowEvaluationSettings {
    __padding_end: [u8; 56],
}
impl UDataflowEvaluationSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEvaluationSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEvaluationSettings")
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
pub struct UDataflowBoneManipulator {
    __padding_end: [u8; 224],
}
impl UDataflowBoneManipulator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBoneManipulator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowBoneManipulator")
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
pub struct UDataflowEditorEditSkeletonBonesToolBuilder {
    __padding_end: [u8; 56],
}
impl UDataflowEditorEditSkeletonBonesToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorEditSkeletonBonesToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorEditSkeletonBonesToolBuilder")
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
pub struct UDataflowEditorEditSkeletonBonesTool {
    __padding_end: [u8; 736],
}
impl UDataflowEditorEditSkeletonBonesTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorEditSkeletonBonesTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorEditSkeletonBonesTool")
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
pub struct UDataflowTransformGizmoSource {
    __padding_end: [u8; 72],
}
impl UDataflowTransformGizmoSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowTransformGizmoSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowTransformGizmoSource")
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
pub struct UDataflowEditorSkinWeightsPaintToolBuilder {
    __padding_end: [u8; 64],
}
impl UDataflowEditorSkinWeightsPaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSkinWeightsPaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSkinWeightsPaintToolBuilder")
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
pub struct UDataflowEditorSkinWeightsPaintTool {
    __padding_end: [u8; 3456],
}
impl UDataflowEditorSkinWeightsPaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSkinWeightsPaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorSkinWeightsPaintTool")
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
pub struct UDataflowEditorVertexAttributePaintToolBuilder {
    __padding_end: [u8; 80],
}
impl UDataflowEditorVertexAttributePaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintToolBuilder")
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
pub struct UDataflowEditorVertexAttributePaintToolProperties {
    __padding_end: [u8; 608],
}
impl UDataflowEditorVertexAttributePaintToolProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintToolProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintToolProperties")
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
pub struct UDataflowEditorVertexAttributePaintTool {
    __padding_end: [u8; 5888],
}
impl UDataflowEditorVertexAttributePaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorVertexAttributePaintTool")
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
pub struct UDataflowWeightMapEraseBrushOpProps {
    __padding_end: [u8; 200],
}
impl UDataflowWeightMapEraseBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapEraseBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapEraseBrushOpProps")
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
pub struct UDataflowWeightMapPaintBrushOpProps {
    __padding_end: [u8; 208],
}
impl UDataflowWeightMapPaintBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapPaintBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapPaintBrushOpProps")
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
pub struct UDataflowWeightMapSmoothBrushOpProps {
    __padding_end: [u8; 200],
}
impl UDataflowWeightMapSmoothBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapSmoothBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowWeightMapSmoothBrushOpProps")
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
pub struct UDataflowVertexAttributePaintBrushOpProps {
    __padding_end: [u8; 208],
}
impl UDataflowVertexAttributePaintBrushOpProps {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowVertexAttributePaintBrushOpProps")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowVertexAttributePaintBrushOpProps")
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
pub struct UDataflowEditorWeightMapPaintToolBuilder {
    __padding_end: [u8; 72],
}
impl UDataflowEditorWeightMapPaintToolBuilder {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintToolBuilder")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintToolBuilder")
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
pub struct UDataflowEditorWeightMapPaintBrushFilterProperties {
    __padding_end: [u8; 248],
}
impl UDataflowEditorWeightMapPaintBrushFilterProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintBrushFilterProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintBrushFilterProperties")
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
pub struct UDataflowEditorMeshWeightMapPaintToolActions {
    __padding_end: [u8; 192],
}
impl UDataflowEditorMeshWeightMapPaintToolActions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorMeshWeightMapPaintToolActions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorMeshWeightMapPaintToolActions")
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
pub struct UDataflowEditorUpdateWeightMapProperties {
    __padding_end: [u8; 200],
}
impl UDataflowEditorUpdateWeightMapProperties {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorUpdateWeightMapProperties")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorUpdateWeightMapProperties")
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
pub struct UDataflowEditorWeightMapPaintTool {
    __padding_end: [u8; 5184],
}
impl UDataflowEditorWeightMapPaintTool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintTool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorWeightMapPaintTool")
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
pub struct UAssetDefinition_DataflowAsset {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_DataflowAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_DataflowAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_DataflowAsset")
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
pub struct UAssetDefinition_DataflowContext {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_DataflowContext {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_DataflowContext")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_DataflowContext")
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
pub struct UDataflowAssetFactory {
    __padding_end: [u8; 136],
}
impl UDataflowAssetFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowAssetFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowAssetFactory")
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
pub struct UDataflowComponentReadOnlyToolTarget {
    __padding_end: [u8; 96],
}
impl UDataflowComponentReadOnlyToolTarget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentReadOnlyToolTarget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentReadOnlyToolTarget")
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
pub struct UDataflowComponentToolTarget {
    __padding_end: [u8; 112],
}
impl UDataflowComponentToolTarget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentToolTarget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentToolTarget")
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
pub struct UDataflowComponentReadOnlyToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UDataflowComponentReadOnlyToolTargetFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentReadOnlyToolTargetFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentReadOnlyToolTargetFactory")
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
pub struct UDataflowComponentToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UDataflowComponentToolTargetFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentToolTargetFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowComponentToolTargetFactory")
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
pub struct UDataflowEditor {
    __padding_end: [u8; 136],
}
impl UDataflowEditor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditor")
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
pub struct UDataflowEditorBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UDataflowEditorBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorBlueprintLibrary")
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
    pub fn set_dataflow_node_property(
        dataflow: UPtr<crate::bindings::dataflow_engine::UDataflow>,
        node_name: FName,
        property_name: FName,
        propertyvalue: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_set_dataflow_node_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dataflow,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &propertyvalue,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_editor::UDataflowEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_set_dataflow_node_property,
                __buffer,
            )
        };
        std::mem::forget(dataflow);
        std::mem::forget(node_name);
        std::mem::forget(property_name);
        std::mem::forget(propertyvalue);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn connect_dataflow_nodes(
        dataflow: UPtr<crate::bindings::dataflow_engine::UDataflow>,
        from_node_name: FName,
        output_name: FName,
        to_node_name: FName,
        input_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_connect_dataflow_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dataflow,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_node_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_node_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(44).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_editor::UDataflowEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_connect_dataflow_nodes,
                __buffer,
            )
        };
        std::mem::forget(dataflow);
        std::mem::forget(from_node_name);
        std::mem::forget(output_name);
        std::mem::forget(to_node_name);
        std::mem::forget(input_name);
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn add_dataflow_node(
        dataflow: UPtr<crate::bindings::dataflow_engine::UDataflow>,
        node_type_name: FName,
        base_name: FName,
        location: crate::bindings::core_u_object::FVector2D,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dataflow,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_type_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_editor::UDataflowEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_node,
                __buffer,
            )
        };
        std::mem::forget(dataflow);
        std::mem::forget(node_type_name);
        std::mem::forget(base_name);
        std::mem::forget(location);
        unsafe { __buffer.add(48).cast::<FName>().read() }
    }
    pub fn add_dataflow_from_clipboard_content(
        dataflow: UPtr<crate::bindings::dataflow_engine::UDataflow>,
        clipboard_content: FString,
        location: crate::bindings::core_u_object::FVector2D,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_from_clipboard_content,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dataflow,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::dataflow_engine::UDataflow>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clipboard_content,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::dataflow_editor::UDataflowEditorBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::dataflow_editor::__FUNCTION_PTRS
                    .u_dataflow_editor_blueprint_library_add_dataflow_from_clipboard_content,
                __buffer,
            )
        };
        std::mem::forget(dataflow);
        std::mem::forget(clipboard_content);
        std::mem::forget(location);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UDataflowEditorCollectionComponent {
    __padding_end: [u8; 2560],
}
impl UDataflowEditorCollectionComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorCollectionComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorCollectionComponent")
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
pub struct UDataflowEditorMode {
    __padding_end: [u8; 744],
}
impl UDataflowEditorMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorMode")
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
pub struct UDataflowEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl UDataflowEditorUISubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorUISubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorUISubsystem")
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
pub struct UDataflowEditorOptions {
    __padding_end: [u8; 160],
}
impl UDataflowEditorOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowEditorOptions")
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
pub struct UDataflowObjectFactory {
    __padding_end: [u8; 48],
}
impl UDataflowObjectFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowObjectFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowObjectFactory")
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
pub struct UDataflowSchema {
    __padding_end: [u8; 56],
}
impl UDataflowSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSchema")
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
pub struct UDataflowSimulationSettings {
    __padding_end: [u8; 56],
}
impl UDataflowSimulationSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationSettings")
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
pub struct UDataflowSimulationSceneDescription {
    __padding_end: [u8; 304],
}
impl UDataflowSimulationSceneDescription {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationSceneDescription")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowSimulationSceneDescription")
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
pub struct UDataflowReadOnlyToolTarget {
    __padding_end: [u8; 112],
}
impl UDataflowReadOnlyToolTarget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowReadOnlyToolTarget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowReadOnlyToolTarget")
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
pub struct UDataflowToolTarget {
    __padding_end: [u8; 128],
}
impl UDataflowToolTarget {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowToolTarget")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowToolTarget")
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
pub struct UDataflowReadOnlyToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UDataflowReadOnlyToolTargetFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowReadOnlyToolTargetFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowReadOnlyToolTargetFactory")
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
pub struct UDataflowToolTargetFactory {
    __padding_end: [u8; 48],
}
impl UDataflowToolTargetFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowToolTargetFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataflowToolTargetFactory")
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
pub struct ESkinWeightsCorrectionType(pub u8);
impl ESkinWeightsCorrectionType {
    pub const RELAX: ESkinWeightsCorrectionType = ESkinWeightsCorrectionType(0);
    pub const PRUNE: ESkinWeightsCorrectionType = ESkinWeightsCorrectionType(1);
    pub const HAMMER: ESkinWeightsCorrectionType = ESkinWeightsCorrectionType(2);
    pub const CLAMP: ESkinWeightsCorrectionType = ESkinWeightsCorrectionType(3);
    pub const NORMALIZE: ESkinWeightsCorrectionType = ESkinWeightsCorrectionType(4);
}
#[repr(transparent)]
pub struct EDataflowEditorToolEditOperation(pub u8);
impl EDataflowEditorToolEditOperation {
    pub const ADD: EDataflowEditorToolEditOperation = EDataflowEditorToolEditOperation(
        0,
    );
    pub const REPLACE: EDataflowEditorToolEditOperation = EDataflowEditorToolEditOperation(
        1,
    );
    pub const MULTIPLY: EDataflowEditorToolEditOperation = EDataflowEditorToolEditOperation(
        2,
    );
    pub const INVERT: EDataflowEditorToolEditOperation = EDataflowEditorToolEditOperation(
        3,
    );
    pub const RELAX: EDataflowEditorToolEditOperation = EDataflowEditorToolEditOperation(
        4,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorToolVisibilityType(pub u8);
impl EDataflowEditorToolVisibilityType {
    pub const NONE: EDataflowEditorToolVisibilityType = EDataflowEditorToolVisibilityType(
        0,
    );
    pub const UNOCCLUDED: EDataflowEditorToolVisibilityType = EDataflowEditorToolVisibilityType(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorToolColorMode(pub u8);
impl EDataflowEditorToolColorMode {
    pub const GREYSCALE: EDataflowEditorToolColorMode = EDataflowEditorToolColorMode(0);
    pub const RAMP: EDataflowEditorToolColorMode = EDataflowEditorToolColorMode(1);
    pub const FULL_MATERIAL: EDataflowEditorToolColorMode = EDataflowEditorToolColorMode(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorToolMirrorDirection(pub u8);
impl EDataflowEditorToolMirrorDirection {
    pub const POSITIVE_TO_NEGATIVE: EDataflowEditorToolMirrorDirection = EDataflowEditorToolMirrorDirection(
        0,
    );
    pub const NEGATIVE_TO_POSITIVE: EDataflowEditorToolMirrorDirection = EDataflowEditorToolMirrorDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorToolEditMode(pub u8);
impl EDataflowEditorToolEditMode {
    pub const BRUSH: EDataflowEditorToolEditMode = EDataflowEditorToolEditMode(0);
    pub const MESH: EDataflowEditorToolEditMode = EDataflowEditorToolEditMode(1);
}
#[repr(transparent)]
pub struct EDataflowEditorWeightMapPaintInteractionType(pub u8);
impl EDataflowEditorWeightMapPaintInteractionType {
    pub const BRUSH: EDataflowEditorWeightMapPaintInteractionType = EDataflowEditorWeightMapPaintInteractionType(
        0,
    );
    pub const FILL: EDataflowEditorWeightMapPaintInteractionType = EDataflowEditorWeightMapPaintInteractionType(
        1,
    );
    pub const POLY_LASSO: EDataflowEditorWeightMapPaintInteractionType = EDataflowEditorWeightMapPaintInteractionType(
        2,
    );
    pub const GRADIENT: EDataflowEditorWeightMapPaintInteractionType = EDataflowEditorWeightMapPaintInteractionType(
        3,
    );
    pub const LAST_VALUE: EDataflowEditorWeightMapPaintInteractionType = EDataflowEditorWeightMapPaintInteractionType(
        4,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorWeightMapPaintBrushType(pub u8);
impl EDataflowEditorWeightMapPaintBrushType {
    pub const PAINT: EDataflowEditorWeightMapPaintBrushType = EDataflowEditorWeightMapPaintBrushType(
        0,
    );
    pub const SMOOTH: EDataflowEditorWeightMapPaintBrushType = EDataflowEditorWeightMapPaintBrushType(
        1,
    );
    pub const ERASE: EDataflowEditorWeightMapPaintBrushType = EDataflowEditorWeightMapPaintBrushType(
        2,
    );
    pub const LAST_VALUE: EDataflowEditorWeightMapPaintBrushType = EDataflowEditorWeightMapPaintBrushType(
        3,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorWeightMapPaintVisibilityType(pub u8);
impl EDataflowEditorWeightMapPaintVisibilityType {
    pub const NONE: EDataflowEditorWeightMapPaintVisibilityType = EDataflowEditorWeightMapPaintVisibilityType(
        0,
    );
    pub const UNOCCLUDED: EDataflowEditorWeightMapPaintVisibilityType = EDataflowEditorWeightMapPaintVisibilityType(
        1,
    );
}
#[repr(transparent)]
pub struct EDataflowConstructionViewportMousePanButton(pub u8);
impl EDataflowConstructionViewportMousePanButton {
    pub const RIGHT: EDataflowConstructionViewportMousePanButton = EDataflowConstructionViewportMousePanButton(
        0,
    );
    pub const MIDDLE: EDataflowConstructionViewportMousePanButton = EDataflowConstructionViewportMousePanButton(
        1,
    );
    pub const RIGHT_OR_MIDDLE: EDataflowConstructionViewportMousePanButton = EDataflowConstructionViewportMousePanButton(
        2,
    );
}
#[repr(transparent)]
pub struct EDataflowEditorEvaluationMode(pub u8);
impl EDataflowEditorEvaluationMode {
    pub const AUTOMATIC: EDataflowEditorEvaluationMode = EDataflowEditorEvaluationMode(
        0,
    );
    pub const MANUAL: EDataflowEditorEvaluationMode = EDataflowEditorEvaluationMode(1);
}
