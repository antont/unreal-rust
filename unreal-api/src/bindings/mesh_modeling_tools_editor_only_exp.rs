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
    pub u_bake_render_capture_tool_properties_get_map_preview_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_render_capture_input_tool_properties_get_target_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_bake_render_capture_input_tool_properties_get_target_uv_layer_index: *mut crate::ffi::UFunctionOpague,
    pub u_bsp_conversion_tool_action_property_set_select_all_valid_brushes: *mut crate::ffi::UFunctionOpague,
    pub u_bsp_conversion_tool_action_property_set_deselect_volumes: *mut crate::ffi::UFunctionOpague,
    pub u_bsp_conversion_tool_action_property_set_deselect_non_valid: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_materials_tool_properties_get_material_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_edit_mesh_materials_edit_actions_assign_active_material: *mut crate::ffi::UFunctionOpague,
    pub uism_editor_tool_action_property_set_duplicate: *mut crate::ffi::UFunctionOpague,
    pub uism_editor_tool_action_property_set_delete: *mut crate::ffi::UFunctionOpague,
    pub uism_editor_tool_action_property_set_clear_selection: *mut crate::ffi::UFunctionOpague,
    pub uism_editor_tool_replace_property_set_replace: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_bake_render_capture_tool_properties_get_map_preview_names_func: std::ptr::null_mut(),
            u_bake_render_capture_input_tool_properties_get_target_uv_layer_names_func: std::ptr::null_mut(),
            u_bake_render_capture_input_tool_properties_get_target_uv_layer_index: std::ptr::null_mut(),
            u_bsp_conversion_tool_action_property_set_select_all_valid_brushes: std::ptr::null_mut(),
            u_bsp_conversion_tool_action_property_set_deselect_volumes: std::ptr::null_mut(),
            u_bsp_conversion_tool_action_property_set_deselect_non_valid: std::ptr::null_mut(),
            u_edit_mesh_materials_tool_properties_get_material_names_func: std::ptr::null_mut(),
            u_edit_mesh_materials_edit_actions_assign_active_material: std::ptr::null_mut(),
            uism_editor_tool_action_property_set_duplicate: std::ptr::null_mut(),
            uism_editor_tool_action_property_set_delete: std::ptr::null_mut(),
            uism_editor_tool_action_property_set_clear_selection: std::ptr::null_mut(),
            uism_editor_tool_replace_property_set_replace: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeRenderCaptureToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMapPreviewNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_render_capture_tool_properties_get_map_preview_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBakeRenderCaptureInputToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_bake_render_capture_input_tool_properties_get_target_uv_layer_names_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetUVLayerIndex"),
            &raw mut __FUNCTION_PTRS
                .u_bake_render_capture_input_tool_properties_get_target_uv_layer_index,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UBspConversionToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectAllValidBrushes"),
            &raw mut __FUNCTION_PTRS
                .u_bsp_conversion_tool_action_property_set_select_all_valid_brushes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeselectVolumes"),
            &raw mut __FUNCTION_PTRS
                .u_bsp_conversion_tool_action_property_set_deselect_volumes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeselectNonValid"),
            &raw mut __FUNCTION_PTRS
                .u_bsp_conversion_tool_action_property_set_deselect_non_valid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditMeshMaterialsToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_edit_mesh_materials_tool_properties_get_material_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditMeshMaterialsEditActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssignActiveMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_edit_mesh_materials_edit_actions_assign_active_material,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UISMEditorToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Duplicate"),
            &raw mut __FUNCTION_PTRS.uism_editor_tool_action_property_set_duplicate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Delete"),
            &raw mut __FUNCTION_PTRS.uism_editor_tool_action_property_set_delete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSelection"),
            &raw mut __FUNCTION_PTRS.uism_editor_tool_action_property_set_clear_selection,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UISMEditorToolReplacePropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Replace"),
            &raw mut __FUNCTION_PTRS.uism_editor_tool_replace_property_set_replace,
        );
    }
}
#[repr(C, align(16))]
pub struct UPivotActorTransformProperties {
    __padding_end: [u8; 240],
}
impl UPivotActorTransformProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPivotActorTransformProperties")
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
pub struct UAddPivotActorToolBuilder {
    __padding_end: [u8; 48],
}
impl UAddPivotActorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPivotActorToolBuilder")
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
pub struct UAddPivotActorTool {
    __padding_end: [u8; 624],
}
impl UAddPivotActorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAddPivotActorTool")
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
pub struct UBakeRenderCaptureResults {
    __padding_end: [u8; 264],
}
impl UBakeRenderCaptureResults {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureResults")
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
pub struct UBakeRenderCaptureToolBuilder {
    __padding_end: [u8; 48],
}
impl UBakeRenderCaptureToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureToolBuilder")
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
pub struct URenderCaptureProperties {
    __padding_end: [u8; 208],
}
impl URenderCaptureProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URenderCaptureProperties")
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
pub struct UBakeRenderCaptureToolProperties {
    __padding_end: [u8; 232],
}
impl UBakeRenderCaptureToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureToolProperties")
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
pub struct UBakeRenderCaptureInputToolProperties {
    __padding_end: [u8; 224],
}
impl UBakeRenderCaptureInputToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureInputToolProperties")
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
pub struct UBakeRenderCaptureVisualizationProperties {
    __padding_end: [u8; 200],
}
impl UBakeRenderCaptureVisualizationProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureVisualizationProperties")
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
pub struct UBakeRenderCaptureTool {
    __padding_end: [u8; 728],
}
impl UBakeRenderCaptureTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeRenderCaptureTool")
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
pub struct UBspConversionToolBuilder {
    __padding_end: [u8; 48],
}
impl UBspConversionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBspConversionToolBuilder")
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
pub struct UBspConversionToolProperties {
    __padding_end: [u8; 192],
}
impl UBspConversionToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBspConversionToolProperties")
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
pub struct UBspConversionToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UBspConversionToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBspConversionToolActionPropertySet")
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
pub struct UBspConversionTool {
    __padding_end: [u8; 368],
}
impl UBspConversionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBspConversionTool")
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
pub struct UDrawSplineToolProperties {
    __padding_end: [u8; 280],
}
impl UDrawSplineToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawSplineToolProperties")
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
pub struct UDrawSplineTool {
    __padding_end: [u8; 432],
}
impl UDrawSplineTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawSplineTool")
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
pub struct UDrawSplineToolBuilder {
    __padding_end: [u8; 48],
}
impl UDrawSplineToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrawSplineToolBuilder")
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
pub struct UEditMeshMaterialsToolBuilder {
    __padding_end: [u8; 56],
}
impl UEditMeshMaterialsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshMaterialsToolBuilder")
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
pub struct UEditMeshMaterialsToolProperties {
    __padding_end: [u8; 232],
}
impl UEditMeshMaterialsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshMaterialsToolProperties")
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
pub struct UEditMeshMaterialsEditActions {
    __padding_end: [u8; 192],
}
impl UEditMeshMaterialsEditActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshMaterialsEditActions")
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
pub struct UEditMeshMaterialsTool {
    __padding_end: [u8; 2176],
}
impl UEditMeshMaterialsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditMeshMaterialsTool")
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
pub struct UHarvestInstancesToolBuilder {
    __padding_end: [u8; 48],
}
impl UHarvestInstancesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHarvestInstancesToolBuilder")
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
pub struct UHarvestInstancesToolSettings {
    __padding_end: [u8; 192],
}
impl UHarvestInstancesToolSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHarvestInstancesToolSettings")
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
pub struct UHarvestInstancesTool_OutputSettings {
    __padding_end: [u8; 216],
}
impl UHarvestInstancesTool_OutputSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHarvestInstancesTool_OutputSettings")
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
pub struct UHarvestInstancesTool {
    __padding_end: [u8; 320],
}
impl UHarvestInstancesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHarvestInstancesTool")
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
pub struct UISMEditorToolBuilder {
    __padding_end: [u8; 48],
}
impl UISMEditorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorToolBuilder")
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
pub struct UISMEditorToolProperties {
    __padding_end: [u8; 208],
}
impl UISMEditorToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorToolProperties")
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
pub struct UISMEditorToolActionPropertySetBase {
    __padding_end: [u8; 192],
}
impl UISMEditorToolActionPropertySetBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorToolActionPropertySetBase")
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
pub struct UISMEditorToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UISMEditorToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorToolActionPropertySet")
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
pub struct UISMEditorToolReplacePropertySet {
    __padding_end: [u8; 200],
}
impl UISMEditorToolReplacePropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorToolReplacePropertySet")
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
pub struct UISMEditorTool {
    __padding_end: [u8; 360],
}
impl UISMEditorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UISMEditorTool")
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
pub struct UMergeMeshesToolProperties {
    __padding_end: [u8; 200],
}
impl UMergeMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMergeMeshesToolProperties")
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
pub struct UMergeMeshesTool {
    __padding_end: [u8; 344],
}
impl UMergeMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMergeMeshesTool")
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
pub struct UMergeMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UMergeMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMergeMeshesToolBuilder")
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
pub struct UMeshTangentsToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshTangentsToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTangentsToolBuilder")
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
pub struct UMeshTangentsToolProperties {
    __padding_end: [u8; 208],
}
impl UMeshTangentsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTangentsToolProperties")
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
pub struct UMeshTangentsTool {
    __padding_end: [u8; 720],
}
impl UMeshTangentsTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTangentsTool")
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
pub struct UMeshToVolumeToolBuilder {
    __padding_end: [u8; 48],
}
impl UMeshToVolumeToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshToVolumeToolBuilder")
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
pub struct UMeshToVolumeToolProperties {
    __padding_end: [u8; 232],
}
impl UMeshToVolumeToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshToVolumeToolProperties")
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
pub struct UMeshToVolumeTool {
    __padding_end: [u8; 288],
}
impl UMeshToVolumeTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshToVolumeTool")
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
pub struct UShapeSprayToolBuilder {
    __padding_end: [u8; 56],
}
impl UShapeSprayToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShapeSprayToolBuilder")
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
pub struct UShapeSprayToolProperties {
    __padding_end: [u8; 224],
}
impl UShapeSprayToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShapeSprayToolProperties")
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
pub struct UShapeSprayTool {
    __padding_end: [u8; 1600],
}
impl UShapeSprayTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShapeSprayTool")
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
pub struct USubdividePolyToolBuilder {
    __padding_end: [u8; 48],
}
impl USubdividePolyToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubdividePolyToolBuilder")
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
pub struct USubdividePolyToolProperties {
    __padding_end: [u8; 216],
}
impl USubdividePolyToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubdividePolyToolProperties")
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
pub struct USubdividePolyTool {
    __padding_end: [u8; 328],
}
impl USubdividePolyTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubdividePolyTool")
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
pub struct UVoxelCSGMeshesToolProperties {
    __padding_end: [u8; 208],
}
impl UVoxelCSGMeshesToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelCSGMeshesToolProperties")
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
pub struct UVoxelCSGMeshesTool {
    __padding_end: [u8; 344],
}
impl UVoxelCSGMeshesTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelCSGMeshesTool")
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
pub struct UVoxelCSGMeshesToolBuilder {
    __padding_end: [u8; 48],
}
impl UVoxelCSGMeshesToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelCSGMeshesToolBuilder")
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
pub struct EBspConversionMode(pub u8);
impl EBspConversionMode {
    pub const CONVERT_FIRST: EBspConversionMode = EBspConversionMode(0);
    pub const COMBINE_FIRST: EBspConversionMode = EBspConversionMode(1);
}
#[repr(transparent)]
pub struct EDrawSplineOutputMode(pub u8);
impl EDrawSplineOutputMode {
    pub const EMPTY_ACTOR: EDrawSplineOutputMode = EDrawSplineOutputMode(0);
    pub const EXISTING_ACTOR: EDrawSplineOutputMode = EDrawSplineOutputMode(1);
    pub const CREATE_BLUEPRINT: EDrawSplineOutputMode = EDrawSplineOutputMode(2);
}
#[repr(transparent)]
pub struct EDrawSplineDrawMode(pub u8);
impl EDrawSplineDrawMode {
    pub const TANGENT_DRAG: EDrawSplineDrawMode = EDrawSplineDrawMode(0);
    pub const CLICK_AUTO_TANGENT: EDrawSplineDrawMode = EDrawSplineDrawMode(1);
    pub const FREE_DRAW: EDrawSplineDrawMode = EDrawSplineDrawMode(2);
}
#[repr(transparent)]
pub struct ESplineOffsetMethod(pub u8);
impl ESplineOffsetMethod {
    pub const HIT_NORMAL: ESplineOffsetMethod = ESplineOffsetMethod(0);
    pub const CUSTOM: ESplineOffsetMethod = ESplineOffsetMethod(1);
}
#[repr(transparent)]
pub struct EDrawSplineUpVectorMode(pub u8);
impl EDrawSplineUpVectorMode {
    pub const ALIGN_TO_PREVIOUS: EDrawSplineUpVectorMode = EDrawSplineUpVectorMode(0);
    pub const USE_HIT_NORMAL: EDrawSplineUpVectorMode = EDrawSplineUpVectorMode(1);
}
#[repr(transparent)]
pub struct EHarvestInstancesToolOutputType(pub i32);
impl EHarvestInstancesToolOutputType {
    pub const HISMC: EHarvestInstancesToolOutputType = EHarvestInstancesToolOutputType(
        0,
    );
    pub const ISMC: EHarvestInstancesToolOutputType = EHarvestInstancesToolOutputType(1);
}
#[repr(transparent)]
pub struct EISMEditorTransformMode(pub u8);
impl EISMEditorTransformMode {
    pub const SHARED_GIZMO: EISMEditorTransformMode = EISMEditorTransformMode(0);
    pub const SHARED_GIZMO_LOCAL: EISMEditorTransformMode = EISMEditorTransformMode(1);
    pub const PER_OBJECT_GIZMO: EISMEditorTransformMode = EISMEditorTransformMode(2);
    pub const LAST_VALUE: EISMEditorTransformMode = EISMEditorTransformMode(3);
}
#[repr(transparent)]
pub struct EMeshToVolumeMode(pub i32);
impl EMeshToVolumeMode {
    pub const TRIANGULATE_POLYGONS: EMeshToVolumeMode = EMeshToVolumeMode(0);
    pub const MINIMAL_POLYGONS: EMeshToVolumeMode = EMeshToVolumeMode(1);
}
#[repr(transparent)]
pub struct EVoxelCSGOperation(pub u8);
impl EVoxelCSGOperation {
    pub const DIFFERENCE_AB: EVoxelCSGOperation = EVoxelCSGOperation(0);
    pub const DIFFERENCE_BA: EVoxelCSGOperation = EVoxelCSGOperation(1);
    pub const INTERSECT: EVoxelCSGOperation = EVoxelCSGOperation(2);
    pub const UNION: EVoxelCSGOperation = EVoxelCSGOperation(3);
}
