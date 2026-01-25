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
    pub u_attribute_editor_normals_actions_reset_hard_normals: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_normals_actions_discard_tangents: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_uv_actions_get_uv_layer_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_uv_actions_duplicate_selected: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_uv_actions_delete_selected: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_uv_actions_clear_all: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_uv_actions_add_new: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_lightmap_uv_actions_reset: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_lightmap_uv_actions_enable: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_lightmap_uv_actions_disable: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_new_attribute_actions_add_weight_map_layer: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_new_attribute_actions_add_poly_group_layer: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_modify_attribute_actions_get_attribute_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_modify_attribute_actions_delete_selected: *mut crate::ffi::UFunctionOpague,
    pub u_attribute_editor_copy_attribute_actions_copy_from_to: *mut crate::ffi::UFunctionOpague,
    pub u_polygon_on_mesh_tool_action_property_set_draw_polygon: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weights_paint_tool_properties_get_target_skin_weight_profiles_func: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weights_paint_tool_properties_get_source_skin_weight_profiles_func: *mut crate::ffi::UFunctionOpague,
    pub u_skin_weights_paint_tool_properties_get_source_lo_ds_func: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_attribute_editor_normals_actions_reset_hard_normals: std::ptr::null_mut(),
            u_attribute_editor_normals_actions_discard_tangents: std::ptr::null_mut(),
            u_attribute_editor_uv_actions_get_uv_layer_names_func: std::ptr::null_mut(),
            u_attribute_editor_uv_actions_duplicate_selected: std::ptr::null_mut(),
            u_attribute_editor_uv_actions_delete_selected: std::ptr::null_mut(),
            u_attribute_editor_uv_actions_clear_all: std::ptr::null_mut(),
            u_attribute_editor_uv_actions_add_new: std::ptr::null_mut(),
            u_attribute_editor_lightmap_uv_actions_reset: std::ptr::null_mut(),
            u_attribute_editor_lightmap_uv_actions_enable: std::ptr::null_mut(),
            u_attribute_editor_lightmap_uv_actions_disable: std::ptr::null_mut(),
            u_attribute_editor_new_attribute_actions_add_weight_map_layer: std::ptr::null_mut(),
            u_attribute_editor_new_attribute_actions_add_poly_group_layer: std::ptr::null_mut(),
            u_attribute_editor_modify_attribute_actions_get_attribute_names_func: std::ptr::null_mut(),
            u_attribute_editor_modify_attribute_actions_delete_selected: std::ptr::null_mut(),
            u_attribute_editor_copy_attribute_actions_copy_from_to: std::ptr::null_mut(),
            u_polygon_on_mesh_tool_action_property_set_draw_polygon: std::ptr::null_mut(),
            u_skin_weights_paint_tool_properties_get_target_skin_weight_profiles_func: std::ptr::null_mut(),
            u_skin_weights_paint_tool_properties_get_source_skin_weight_profiles_func: std::ptr::null_mut(),
            u_skin_weights_paint_tool_properties_get_source_lo_ds_func: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorNormalsActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetHardNormals"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_normals_actions_reset_hard_normals,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DiscardTangents"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_normals_actions_discard_tangents,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorUVActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUVLayerNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_uv_actions_get_uv_layer_names_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateSelected"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_uv_actions_duplicate_selected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteSelected"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_uv_actions_delete_selected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearAll"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_uv_actions_clear_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNew"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_uv_actions_add_new,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorLightmapUVActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Reset"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_lightmap_uv_actions_reset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Enable"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_lightmap_uv_actions_enable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Disable"),
            &raw mut __FUNCTION_PTRS.u_attribute_editor_lightmap_uv_actions_disable,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorNewAttributeActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddWeightMapLayer"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_new_attribute_actions_add_weight_map_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPolyGroupLayer"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_new_attribute_actions_add_poly_group_layer,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorModifyAttributeActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttributeNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_modify_attribute_actions_get_attribute_names_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteSelected"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_modify_attribute_actions_delete_selected,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAttributeEditorCopyAttributeActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyFromTo"),
            &raw mut __FUNCTION_PTRS
                .u_attribute_editor_copy_attribute_actions_copy_from_to,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPolygonOnMeshToolActionPropertySet::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DrawPolygon"),
            &raw mut __FUNCTION_PTRS
                .u_polygon_on_mesh_tool_action_property_set_draw_polygon,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkinWeightsPaintToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetSkinWeightProfilesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_skin_weights_paint_tool_properties_get_target_skin_weight_profiles_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceSkinWeightProfilesFunc"),
            &raw mut __FUNCTION_PTRS
                .u_skin_weights_paint_tool_properties_get_source_skin_weight_profiles_func,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceLODsFunc"),
            &raw mut __FUNCTION_PTRS
                .u_skin_weights_paint_tool_properties_get_source_lo_ds_func,
        );
    }
}
#[repr(C, align(8))]
pub struct UToolMeshSelector {
    __padding_end: [u8; 112],
}
impl UToolMeshSelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolMeshSelector")
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
pub struct UAttributeEditorToolBuilder {
    __padding_end: [u8; 48],
}
impl UAttributeEditorToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorToolBuilder")
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
pub struct UAttributeEditorAttribProperties {
    __padding_end: [u8; 280],
}
impl UAttributeEditorAttribProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorAttribProperties")
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
pub struct UAttributeEditorActionPropertySet {
    __padding_end: [u8; 192],
}
impl UAttributeEditorActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorActionPropertySet")
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
pub struct UAttributeEditorNormalsActions {
    __padding_end: [u8; 192],
}
impl UAttributeEditorNormalsActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorNormalsActions")
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
pub struct UAttributeEditorUVActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorUVActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorUVActions")
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
pub struct UAttributeEditorLightmapUVActions {
    __padding_end: [u8; 208],
}
impl UAttributeEditorLightmapUVActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorLightmapUVActions")
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
pub struct UAttributeEditorNewAttributeActions {
    __padding_end: [u8; 216],
}
impl UAttributeEditorNewAttributeActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorNewAttributeActions")
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
pub struct UAttributeEditorModifyAttributeActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorModifyAttributeActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorModifyAttributeActions")
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
pub struct UAttributeEditorCopyAttributeActions {
    __padding_end: [u8; 224],
}
impl UAttributeEditorCopyAttributeActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorCopyAttributeActions")
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
pub struct UAttributeEditorTool {
    __padding_end: [u8; 304],
}
impl UAttributeEditorTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeEditorTool")
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
pub struct UParameterizeMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UParameterizeMeshToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshToolBuilder")
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
pub struct UParameterizeMeshTool {
    __padding_end: [u8; 336],
}
impl UParameterizeMeshTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParameterizeMeshTool")
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
pub struct UPolygonOnMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl UPolygonOnMeshToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonOnMeshToolBuilder")
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
pub struct UPolygonOnMeshToolProperties {
    __padding_end: [u8; 224],
}
impl UPolygonOnMeshToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonOnMeshToolProperties")
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
pub struct UPolygonOnMeshToolActionPropertySet {
    __padding_end: [u8; 192],
}
impl UPolygonOnMeshToolActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonOnMeshToolActionPropertySet")
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
pub struct UPolygonOnMeshTool {
    __padding_end: [u8; 656],
}
impl UPolygonOnMeshTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonOnMeshTool")
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
pub struct USimplifyMeshToolBuilder {
    __padding_end: [u8; 48],
}
impl USimplifyMeshToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimplifyMeshToolBuilder")
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
pub struct USimplifyMeshToolProperties {
    __padding_end: [u8; 240],
}
impl USimplifyMeshToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimplifyMeshToolProperties")
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
pub struct USimplifyMeshTool {
    __padding_end: [u8; 312],
}
impl USimplifyMeshTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimplifyMeshTool")
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
pub struct URefSkeletonPoser {
    __padding_end: [u8; 488],
}
impl URefSkeletonPoser {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URefSkeletonPoser")
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
pub struct ISkeletalMeshEditingInterface {}
#[repr(C, align(8))]
pub struct USkeletalMeshEditingInterface {
    __padding_end: [u8; 48],
}
impl USkeletalMeshEditingInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditingInterface")
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
pub struct USkeletalMeshGizmoContextObjectBase {
    __padding_end: [u8; 48],
}
impl USkeletalMeshGizmoContextObjectBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshGizmoContextObjectBase")
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
pub struct USkeletalMeshGizmoWrapperBase {
    __padding_end: [u8; 56],
}
impl USkeletalMeshGizmoWrapperBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshGizmoWrapperBase")
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
pub struct USkeletalMeshEditorContextObjectBase {
    __padding_end: [u8; 48],
}
impl USkeletalMeshEditorContextObjectBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorContextObjectBase")
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
pub struct USkeletonEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl USkeletonEditingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonEditingToolBuilder")
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
pub struct USkeletonEditingTool {
    __padding_end: [u8; 720],
}
impl USkeletonEditingTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonEditingTool")
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
pub struct USkeletonEditingProperties {
    __padding_end: [u8; 336],
}
impl USkeletonEditingProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonEditingProperties")
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
pub struct UProjectionProperties {
    __padding_end: [u8; 352],
}
impl UProjectionProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProjectionProperties")
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
pub struct UMirroringProperties {
    __padding_end: [u8; 240],
}
impl UMirroringProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirroringProperties")
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
pub struct UOrientingProperties {
    __padding_end: [u8; 240],
}
impl UOrientingProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrientingProperties")
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
pub struct USkeletonTransformProxy {
    __padding_end: [u8; 512],
}
impl USkeletonTransformProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonTransformProxy")
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
pub struct USkinWeightsBindingToolBuilder {
    __padding_end: [u8; 48],
}
impl USkinWeightsBindingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsBindingToolBuilder")
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
pub struct USkinWeightsBindingToolProperties {
    __padding_end: [u8; 216],
}
impl USkinWeightsBindingToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsBindingToolProperties")
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
pub struct USkinWeightsBindingTool {
    __padding_end: [u8; 656],
}
impl USkinWeightsBindingTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsBindingTool")
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
pub struct USkinWeightsPaintToolBuilder {
    __padding_end: [u8; 56],
}
impl USkinWeightsPaintToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsPaintToolBuilder")
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
pub struct USkinWeightsPaintToolProperties {
    __padding_end: [u8; 608],
}
impl USkinWeightsPaintToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsPaintToolProperties")
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
pub struct UDEPRECATED_WeightToolMeshSelector {
    __padding_end: [u8; 112],
}
impl UDEPRECATED_WeightToolMeshSelector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_WeightToolMeshSelector")
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
pub struct UWeightToolTransferManager {
    __padding_end: [u8; 88],
}
impl UWeightToolTransferManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeightToolTransferManager")
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
pub struct UWeightToolSelectionIsolator {
    __padding_end: [u8; 1864],
}
impl UWeightToolSelectionIsolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeightToolSelectionIsolator")
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
pub struct USkinWeightsPaintTool {
    __padding_end: [u8; 3416],
}
impl USkinWeightsPaintTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkinWeightsPaintTool")
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
pub struct EWeightBrushFalloffMode(pub u8);
impl EWeightBrushFalloffMode {
    pub const SURFACE: EWeightBrushFalloffMode = EWeightBrushFalloffMode(0);
    pub const VOLUME: EWeightBrushFalloffMode = EWeightBrushFalloffMode(1);
}
#[repr(transparent)]
pub struct EComponentSelectionMode(pub u8);
impl EComponentSelectionMode {
    pub const VERTICES: EComponentSelectionMode = EComponentSelectionMode(0);
    pub const EDGES: EComponentSelectionMode = EComponentSelectionMode(1);
    pub const FACES: EComponentSelectionMode = EComponentSelectionMode(2);
}
#[repr(transparent)]
pub struct EAttributeEditorElementType(pub u8);
impl EAttributeEditorElementType {
    pub const VERTEX: EAttributeEditorElementType = EAttributeEditorElementType(0);
    pub const VERTEX_INSTANCE: EAttributeEditorElementType = EAttributeEditorElementType(
        1,
    );
    pub const TRIANGLE: EAttributeEditorElementType = EAttributeEditorElementType(2);
    pub const POLYGON: EAttributeEditorElementType = EAttributeEditorElementType(3);
    pub const EDGE: EAttributeEditorElementType = EAttributeEditorElementType(4);
    pub const POLYGON_GROUP: EAttributeEditorElementType = EAttributeEditorElementType(
        5,
    );
}
#[repr(transparent)]
pub struct EAttributeEditorAttribType(pub u8);
impl EAttributeEditorAttribType {
    pub const INT32: EAttributeEditorAttribType = EAttributeEditorAttribType(0);
    pub const BOOLEAN: EAttributeEditorAttribType = EAttributeEditorAttribType(1);
    pub const FLOAT: EAttributeEditorAttribType = EAttributeEditorAttribType(2);
    pub const VECTOR2: EAttributeEditorAttribType = EAttributeEditorAttribType(3);
    pub const VECTOR3: EAttributeEditorAttribType = EAttributeEditorAttribType(4);
    pub const VECTOR4: EAttributeEditorAttribType = EAttributeEditorAttribType(5);
    pub const STRING: EAttributeEditorAttribType = EAttributeEditorAttribType(6);
    pub const UNKNOWN: EAttributeEditorAttribType = EAttributeEditorAttribType(7);
}
#[repr(transparent)]
pub struct EPolygonType(pub i32);
impl EPolygonType {
    pub const CIRCLE: EPolygonType = EPolygonType(0);
    pub const SQUARE: EPolygonType = EPolygonType(1);
    pub const RECTANGLE: EPolygonType = EPolygonType(2);
    pub const ROUND_RECT: EPolygonType = EPolygonType(3);
    pub const CUSTOM: EPolygonType = EPolygonType(4);
}
#[repr(transparent)]
pub struct EEditingOperation(pub u8);
impl EEditingOperation {
    pub const SELECT: EEditingOperation = EEditingOperation(0);
    pub const CREATE: EEditingOperation = EEditingOperation(1);
    pub const REMOVE: EEditingOperation = EEditingOperation(2);
    pub const TRANSFORM: EEditingOperation = EEditingOperation(3);
    pub const PARENT: EEditingOperation = EEditingOperation(4);
    pub const RENAME: EEditingOperation = EEditingOperation(5);
    pub const MIRROR: EEditingOperation = EEditingOperation(6);
}
#[repr(transparent)]
pub struct EProjectionType(pub u8);
impl EProjectionType {
    pub const CAMERA_PLANE: EProjectionType = EProjectionType(0);
    pub const ON_MESH: EProjectionType = EProjectionType(1);
    pub const WITHIN_MESH: EProjectionType = EProjectionType(2);
}
#[repr(transparent)]
pub struct ESkinWeightsBindType(pub u8);
impl ESkinWeightsBindType {
    pub const DIRECT_DISTANCE: ESkinWeightsBindType = ESkinWeightsBindType(0);
    pub const GEODESIC_VOXEL: ESkinWeightsBindType = ESkinWeightsBindType(1);
}
#[repr(transparent)]
pub struct EWeightEditMode(pub u8);
impl EWeightEditMode {
    pub const BRUSH: EWeightEditMode = EWeightEditMode(0);
    pub const MESH: EWeightEditMode = EWeightEditMode(1);
    pub const BONES: EWeightEditMode = EWeightEditMode(2);
}
#[repr(transparent)]
pub struct EWeightEditOperation(pub u8);
impl EWeightEditOperation {
    pub const ADD: EWeightEditOperation = EWeightEditOperation(0);
    pub const REPLACE: EWeightEditOperation = EWeightEditOperation(1);
    pub const MULTIPLY: EWeightEditOperation = EWeightEditOperation(2);
    pub const RELAX: EWeightEditOperation = EWeightEditOperation(3);
    pub const RELATIVE_SCALE: EWeightEditOperation = EWeightEditOperation(4);
}
#[repr(transparent)]
pub struct EWeightColorMode(pub u8);
impl EWeightColorMode {
    pub const GREYSCALE: EWeightColorMode = EWeightColorMode(0);
    pub const RAMP: EWeightColorMode = EWeightColorMode(1);
    pub const BONE_COLORS: EWeightColorMode = EWeightColorMode(2);
    pub const FULL_MATERIAL: EWeightColorMode = EWeightColorMode(3);
}
#[repr(transparent)]
pub struct EMirrorDirection(pub u8);
impl EMirrorDirection {
    pub const POSITIVE_TO_NEGATIVE: EMirrorDirection = EMirrorDirection(0);
    pub const NEGATIVE_TO_POSITIVE: EMirrorDirection = EMirrorDirection(1);
}
#[repr(transparent)]
pub struct EMeshTransferOption(pub u8);
impl EMeshTransferOption {
    pub const SOURCE: EMeshTransferOption = EMeshTransferOption(0);
    pub const TARGET: EMeshTransferOption = EMeshTransferOption(1);
}
