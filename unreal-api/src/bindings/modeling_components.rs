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
    pub u_octree_dynamic_mesh_component_set_dynamic_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_line_set_component_set_line_material: *mut crate::ffi::UFunctionOpague,
    pub u_line_set_component_clear: *mut crate::ffi::UFunctionOpague,
    pub u_line_set_component_add_lines: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_point_set_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_point_set_material: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_line_set_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_line_set_material: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_all_point_sets_material: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_set_all_line_sets_material: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_triangle_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_point_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_line_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_all_triangle_sets: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_all_point_sets: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_remove_all_line_sets: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_get_actor: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_find_triangle_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_find_point_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_find_line_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_disconnect: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_create_in_world: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_add_triangle_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_add_point_set: *mut crate::ffi::UFunctionOpague,
    pub u_preview_geometry_add_line_set: *mut crate::ffi::UFunctionOpague,
    pub u_point_set_component_set_point_material: *mut crate::ffi::UFunctionOpague,
    pub u_point_set_component_clear: *mut crate::ffi::UFunctionOpague,
    pub u_point_set_component_add_points: *mut crate::ffi::UFunctionOpague,
    pub u_modeling_objects_creation_api_create_texture_object: *mut crate::ffi::UFunctionOpague,
    pub u_modeling_objects_creation_api_create_new_component_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_modeling_objects_creation_api_create_new_actor: *mut crate::ffi::UFunctionOpague,
    pub u_modeling_objects_creation_api_create_mesh_object: *mut crate::ffi::UFunctionOpague,
    pub u_modeling_objects_creation_api_create_material_object: *mut crate::ffi::UFunctionOpague,
    pub u_create_mesh_object_type_properties_should_show_property_set: *mut crate::ffi::UFunctionOpague,
    pub u_create_mesh_object_type_properties_get_output_type_names_func: *mut crate::ffi::UFunctionOpague,
    pub u_create_mesh_object_type_properties_get_current_create_mesh_type: *mut crate::ffi::UFunctionOpague,
    pub u_polygroup_layers_properties_get_group_layers_func: *mut crate::ffi::UFunctionOpague,
    pub u_weight_map_set_properties_get_weight_maps_func: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_topology_selection_mechanic_properties_select_all: *mut crate::ffi::UFunctionOpague,
    pub u_mesh_topology_selection_mechanic_properties_invert_selection: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_octree_dynamic_mesh_component_set_dynamic_mesh: std::ptr::null_mut(),
            u_line_set_component_set_line_material: std::ptr::null_mut(),
            u_line_set_component_clear: std::ptr::null_mut(),
            u_line_set_component_add_lines: std::ptr::null_mut(),
            u_preview_geometry_set_point_set_visibility: std::ptr::null_mut(),
            u_preview_geometry_set_point_set_material: std::ptr::null_mut(),
            u_preview_geometry_set_line_set_visibility: std::ptr::null_mut(),
            u_preview_geometry_set_line_set_material: std::ptr::null_mut(),
            u_preview_geometry_set_all_point_sets_material: std::ptr::null_mut(),
            u_preview_geometry_set_all_line_sets_material: std::ptr::null_mut(),
            u_preview_geometry_remove_triangle_set: std::ptr::null_mut(),
            u_preview_geometry_remove_point_set: std::ptr::null_mut(),
            u_preview_geometry_remove_line_set: std::ptr::null_mut(),
            u_preview_geometry_remove_all_triangle_sets: std::ptr::null_mut(),
            u_preview_geometry_remove_all_point_sets: std::ptr::null_mut(),
            u_preview_geometry_remove_all_line_sets: std::ptr::null_mut(),
            u_preview_geometry_get_actor: std::ptr::null_mut(),
            u_preview_geometry_find_triangle_set: std::ptr::null_mut(),
            u_preview_geometry_find_point_set: std::ptr::null_mut(),
            u_preview_geometry_find_line_set: std::ptr::null_mut(),
            u_preview_geometry_disconnect: std::ptr::null_mut(),
            u_preview_geometry_create_in_world: std::ptr::null_mut(),
            u_preview_geometry_add_triangle_set: std::ptr::null_mut(),
            u_preview_geometry_add_point_set: std::ptr::null_mut(),
            u_preview_geometry_add_line_set: std::ptr::null_mut(),
            u_point_set_component_set_point_material: std::ptr::null_mut(),
            u_point_set_component_clear: std::ptr::null_mut(),
            u_point_set_component_add_points: std::ptr::null_mut(),
            u_modeling_objects_creation_api_create_texture_object: std::ptr::null_mut(),
            u_modeling_objects_creation_api_create_new_component_on_actor: std::ptr::null_mut(),
            u_modeling_objects_creation_api_create_new_actor: std::ptr::null_mut(),
            u_modeling_objects_creation_api_create_mesh_object: std::ptr::null_mut(),
            u_modeling_objects_creation_api_create_material_object: std::ptr::null_mut(),
            u_create_mesh_object_type_properties_should_show_property_set: std::ptr::null_mut(),
            u_create_mesh_object_type_properties_get_output_type_names_func: std::ptr::null_mut(),
            u_create_mesh_object_type_properties_get_current_create_mesh_type: std::ptr::null_mut(),
            u_polygroup_layers_properties_get_group_layers_func: std::ptr::null_mut(),
            u_weight_map_set_properties_get_weight_maps_func: std::ptr::null_mut(),
            u_mesh_topology_selection_mechanic_properties_select_all: std::ptr::null_mut(),
            u_mesh_topology_selection_mechanic_properties_invert_selection: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UOctreeDynamicMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDynamicMesh"),
                &raw mut __FUNCTION_PTRS.u_octree_dynamic_mesh_component_set_dynamic_mesh,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULineSetComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLineMaterial"),
                &raw mut __FUNCTION_PTRS.u_line_set_component_set_line_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clear"),
                &raw mut __FUNCTION_PTRS.u_line_set_component_clear,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLines"),
                &raw mut __FUNCTION_PTRS.u_line_set_component_add_lines,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPreviewGeometry::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPointSetVisibility"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_point_set_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPointSetMaterial"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_point_set_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLineSetVisibility"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_line_set_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLineSetMaterial"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_line_set_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAllPointSetsMaterial"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_all_point_sets_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAllLineSetsMaterial"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_set_all_line_sets_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTriangleSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_triangle_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemovePointSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_point_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLineSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_line_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllTriangleSets"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_all_triangle_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllPointSets"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_all_point_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllLineSets"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_remove_all_line_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActor"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_get_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindTriangleSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_find_triangle_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindPointSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_find_point_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindLineSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_find_line_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Disconnect"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_disconnect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateInWorld"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_create_in_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTriangleSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_add_triangle_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddPointSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_add_point_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLineSet"),
                &raw mut __FUNCTION_PTRS.u_preview_geometry_add_line_set,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPointSetComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPointMaterial"),
                &raw mut __FUNCTION_PTRS.u_point_set_component_set_point_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clear"),
                &raw mut __FUNCTION_PTRS.u_point_set_component_clear,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddPoints"),
                &raw mut __FUNCTION_PTRS.u_point_set_component_add_points,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UModelingObjectsCreationAPI::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTextureObject"),
                &raw mut __FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_texture_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewComponentOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_new_component_on_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewActor"),
                &raw mut __FUNCTION_PTRS.u_modeling_objects_creation_api_create_new_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMeshObject"),
                &raw mut __FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_mesh_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMaterialObject"),
                &raw mut __FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_material_object,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCreateMeshObjectTypeProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldShowPropertySet"),
                &raw mut __FUNCTION_PTRS
                    .u_create_mesh_object_type_properties_should_show_property_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOutputTypeNamesFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_create_mesh_object_type_properties_get_output_type_names_func,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentCreateMeshType"),
                &raw mut __FUNCTION_PTRS
                    .u_create_mesh_object_type_properties_get_current_create_mesh_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPolygroupLayersProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGroupLayersFunc"),
                &raw mut __FUNCTION_PTRS
                    .u_polygroup_layers_properties_get_group_layers_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWeightMapSetProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWeightMapsFunc"),
                &raw mut __FUNCTION_PTRS.u_weight_map_set_properties_get_weight_maps_func,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMeshTopologySelectionMechanicProperties::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectAll"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_topology_selection_mechanic_properties_select_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InvertSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_mesh_topology_selection_mechanic_properties_invert_selection,
            );
        }
    }
}
#[repr(C, align(1))]
pub struct FModelingToolsAxisFilter {
    pub(crate) __padding_end: [u8; 3],
}
impl FModelingToolsAxisFilter {}
#[repr(C, align(1))]
pub struct FModelingToolsColorChannelFilter {
    pub(crate) __padding_end: [u8; 4],
}
impl FModelingToolsColorChannelFilter {}
#[repr(C, align(16))]
pub struct FCreateMeshObjectParams {
    pub(crate) __padding_end: [u8; 1712],
}
impl FCreateMeshObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMeshObjectResult {
    pub(crate) __padding_end: [u8; 32],
}
impl FCreateMeshObjectResult {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectParams {
    pub(crate) __padding_end: [u8; 64],
}
impl FCreateTextureObjectParams {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectResult {
    pub(crate) __padding_end: [u8; 16],
}
impl FCreateTextureObjectResult {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectParams {
    pub(crate) __padding_end: [u8; 40],
}
impl FCreateMaterialObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectResult {
    pub(crate) __padding_end: [u8; 16],
}
impl FCreateMaterialObjectResult {}
#[repr(C, align(16))]
pub struct FCreateActorParams {
    pub(crate) __padding_end: [u8; 144],
}
impl FCreateActorParams {}
#[repr(C, align(8))]
pub struct FCreateActorResult {
    pub(crate) __padding_end: [u8; 16],
}
impl FCreateActorResult {}
#[repr(C, align(8))]
pub struct FCreateComponentParams {
    pub(crate) __padding_end: [u8; 40],
}
impl FCreateComponentParams {}
#[repr(C, align(8))]
pub struct FCreateComponentResult {
    pub(crate) __padding_end: [u8; 16],
}
impl FCreateComponentResult {}
pub struct IDynamicMeshProvider {}
#[repr(C, align(8))]
pub struct UDynamicMeshProvider {
    __padding_end: [u8; 48],
}
impl UDynamicMeshProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshProvider")
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
pub struct IDynamicMeshCommitter {}
#[repr(C, align(8))]
pub struct UDynamicMeshCommitter {
    __padding_end: [u8; 48],
}
impl UDynamicMeshCommitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshCommitter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshCommitter")
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
pub struct IPersistentDynamicMeshSource {}
#[repr(C, align(8))]
pub struct UPersistentDynamicMeshSource {
    __padding_end: [u8; 48],
}
impl UPersistentDynamicMeshSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersistentDynamicMeshSource")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersistentDynamicMeshSource")
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
pub struct UInteractiveToolActivity {
    __padding_end: [u8; 56],
}
impl UInteractiveToolActivity {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteractiveToolActivity")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteractiveToolActivity")
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
pub struct IToolActivityHost {}
#[repr(C, align(8))]
pub struct UToolActivityHost {
    __padding_end: [u8; 48],
}
impl UToolActivityHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolActivityHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolActivityHost")
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
pub struct ILatticeStateStorage {}
#[repr(C, align(8))]
pub struct ULatticeStateStorage {
    __padding_end: [u8; 48],
}
impl ULatticeStateStorage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeStateStorage")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeStateStorage")
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
pub struct IMeshSculptLayersManager {}
#[repr(C, align(8))]
pub struct UMeshSculptLayersManager {
    __padding_end: [u8; 48],
}
impl UMeshSculptLayersManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptLayersManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSculptLayersManager")
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
pub struct IModelingToolExternalDynamicMeshUpdateAPI {}
#[repr(C, align(8))]
pub struct UModelingToolExternalDynamicMeshUpdateAPI {
    __padding_end: [u8; 48],
}
impl UModelingToolExternalDynamicMeshUpdateAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingToolExternalDynamicMeshUpdateAPI")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingToolExternalDynamicMeshUpdateAPI")
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
pub struct UGeometrySelectionEditCommandArguments {
    __padding_end: [u8; 96],
}
impl UGeometrySelectionEditCommandArguments {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommandArguments")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommandArguments")
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
pub struct UGeometrySelectionEditCommandResult {
    __padding_end: [u8; 168],
}
impl UGeometrySelectionEditCommandResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommandResult")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommandResult")
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
pub struct UGeometrySelectionEditCommand {
    __padding_end: [u8; 48],
}
impl UGeometrySelectionEditCommand {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommand")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionEditCommand")
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
pub struct UVoxelProperties {
    __padding_end: [u8; 208],
}
impl UVoxelProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVoxelProperties")
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
pub struct UVolumetricBrushStampIndicatorBuilder {
    __padding_end: [u8; 48],
}
impl UVolumetricBrushStampIndicatorBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumetricBrushStampIndicatorBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumetricBrushStampIndicatorBuilder")
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
pub struct UVolumetricBrushStampIndicator {
    __padding_end: [u8; 232],
}
impl UVolumetricBrushStampIndicator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumetricBrushStampIndicator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumetricBrushStampIndicator")
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
pub struct UMultiSelectionMeshEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl UMultiSelectionMeshEditingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiSelectionMeshEditingToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiSelectionMeshEditingToolBuilder")
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
pub struct UBaseCreateFromSelectedToolBuilder {
    __padding_end: [u8; 48],
}
impl UBaseCreateFromSelectedToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedToolBuilder")
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
pub struct UOnAcceptHandleSourcesPropertiesBase {
    __padding_end: [u8; 184],
}
impl UOnAcceptHandleSourcesPropertiesBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesPropertiesBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesPropertiesBase")
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
pub struct UOnAcceptHandleSourcesProperties {
    __padding_end: [u8; 192],
}
impl UOnAcceptHandleSourcesProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesProperties")
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
pub struct UBaseCreateFromSelectedHandleSourceProperties {
    __padding_end: [u8; 232],
}
impl UBaseCreateFromSelectedHandleSourceProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedHandleSourceProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedHandleSourceProperties")
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
pub struct UBaseCreateFromSelectedCollisionProperties {
    __padding_end: [u8; 192],
}
impl UBaseCreateFromSelectedCollisionProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedCollisionProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedCollisionProperties")
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
pub struct UTransformInputsToolProperties {
    __padding_end: [u8; 192],
}
impl UTransformInputsToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformInputsToolProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformInputsToolProperties")
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
pub struct UMultiSelectionMeshEditingTool {
    __padding_end: [u8; 232],
}
impl UMultiSelectionMeshEditingTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiSelectionMeshEditingTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiSelectionMeshEditingTool")
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
pub struct UBaseCreateFromSelectedTool {
    __padding_end: [u8; 312],
}
impl UBaseCreateFromSelectedTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCreateFromSelectedTool")
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
pub struct USingleTargetWithSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl USingleTargetWithSelectionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleTargetWithSelectionToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleTargetWithSelectionToolBuilder")
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
pub struct UBaseMeshProcessingToolBuilder {
    __padding_end: [u8; 48],
}
impl UBaseMeshProcessingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshProcessingToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshProcessingToolBuilder")
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
pub struct USingleTargetWithSelectionTool {
    __padding_end: [u8; 336],
}
impl USingleTargetWithSelectionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleTargetWithSelectionTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleTargetWithSelectionTool")
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
pub struct UBaseMeshProcessingTool {
    __padding_end: [u8; 1184],
}
impl UBaseMeshProcessingTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshProcessingTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMeshProcessingTool")
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
pub struct UBaseVoxelTool {
    __padding_end: [u8; 336],
}
impl UBaseVoxelTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseVoxelTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseVoxelTool")
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
pub struct UMeshSurfacePointMeshEditingToolBuilder {
    __padding_end: [u8; 56],
}
impl UMeshSurfacePointMeshEditingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSurfacePointMeshEditingToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshSurfacePointMeshEditingToolBuilder")
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
pub struct UMultiTargetWithSelectionToolBuilder {
    __padding_end: [u8; 48],
}
impl UMultiTargetWithSelectionToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTargetWithSelectionToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTargetWithSelectionToolBuilder")
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
pub struct UMultiTargetWithSelectionTool {
    __padding_end: [u8; 280],
}
impl UMultiTargetWithSelectionTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTargetWithSelectionTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTargetWithSelectionTool")
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
pub struct USingleSelectionMeshEditingToolBuilder {
    __padding_end: [u8; 48],
}
impl USingleSelectionMeshEditingToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleSelectionMeshEditingToolBuilder")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleSelectionMeshEditingToolBuilder")
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
pub struct USingleSelectionMeshEditingTool {
    __padding_end: [u8; 224],
}
impl USingleSelectionMeshEditingTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleSelectionMeshEditingTool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleSelectionMeshEditingTool")
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
pub struct UDynamicMeshReplacementChangeTarget {
    __padding_end: [u8; 104],
}
impl UDynamicMeshReplacementChangeTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshReplacementChangeTarget")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDynamicMeshReplacementChangeTarget")
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
pub struct UOctreeDynamicMeshComponent {
    __padding_end: [u8; 1984],
}
impl UOctreeDynamicMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOctreeDynamicMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOctreeDynamicMeshComponent")
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
    pub fn set_dynamic_mesh(
        &mut self,
        new_mesh: UPtr<crate::bindings::geometry_framework::UDynamicMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_octree_dynamic_mesh_component_set_dynamic_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mesh,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::geometry_framework::UDynamicMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_octree_dynamic_mesh_component_set_dynamic_mesh,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct ULineSetComponent {
    __padding_end: [u8; 1712],
}
impl ULineSetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULineSetComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULineSetComponent")
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
    pub fn set_line_material(
        &mut self,
        in_line_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_set_line_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_line_material,
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
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_set_line_material,
                __buffer,
            )
        };
    }
    pub fn clear(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_clear,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_clear,
                __buffer,
            )
        };
    }
    pub fn add_lines(
        &mut self,
        in_start: &TArray<crate::bindings::core_u_object::FVector>,
        in_end: &TArray<crate::bindings::core_u_object::FVector>,
        in_color: &crate::bindings::core_u_object::FColor,
        in_thickness: f32,
        in_depth_bias: f32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_add_lines,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_start,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_end,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_thickness,
                __buffer.add(36).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_depth_bias,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_line_set_component_add_lines,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMeshElementsVisualizerProperties {
    __padding_end: [u8; 232],
}
impl UMeshElementsVisualizerProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshElementsVisualizerProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshElementsVisualizerProperties")
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
pub struct UPreviewGeometry {
    __padding_end: [u8; 296],
}
impl UPreviewGeometry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewGeometry")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewGeometry")
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
pub struct UMeshElementsVisualizer {
    __padding_end: [u8; 336],
}
impl UMeshElementsVisualizer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshElementsVisualizer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshElementsVisualizer")
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
pub struct UMeshWireframeComponent {
    __padding_end: [u8; 1744],
}
impl UMeshWireframeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshWireframeComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshWireframeComponent")
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
pub struct UPointSetComponent {
    __padding_end: [u8; 1712],
}
impl UPointSetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPointSetComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPointSetComponent")
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
    pub fn set_point_material(
        &mut self,
        in_point_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_set_point_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_point_material,
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
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_set_point_material,
                __buffer,
            )
        };
    }
    pub fn clear(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_clear,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_clear,
                __buffer,
            )
        };
    }
    pub fn add_points(
        &mut self,
        positions: &TArray<crate::bindings::core_u_object::FVector>,
        in_color: &crate::bindings::core_u_object::FColor,
        in_size: f32,
        in_depth_bias: f32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_add_points,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                positions,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_size, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_depth_bias,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_point_set_component_add_points,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<i32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UPreviewMesh {
    __padding_end: [u8; 304],
}
impl UPreviewMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewMesh")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewMesh")
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
pub struct UPolyEditPreviewMesh {
    __padding_end: [u8; 1152],
}
impl UPolyEditPreviewMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPreviewMesh")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyEditPreviewMesh")
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
pub struct APreviewGeometryActor {
    __padding_end: [u8; 1144],
}
impl APreviewGeometryActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APreviewGeometryActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APreviewGeometryActor")
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
pub struct UTriangleSetComponent {
    __padding_end: [u8; 1840],
}
impl UTriangleSetComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTriangleSetComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTriangleSetComponent")
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
pub struct UUVLayoutPreviewProperties {
    __padding_end: [u8; 224],
}
impl UUVLayoutPreviewProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutPreviewProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutPreviewProperties")
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
pub struct UUVLayoutPreview {
    __padding_end: [u8; 384],
}
impl UUVLayoutPreview {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutPreview")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUVLayoutPreview")
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
pub struct UCollectSurfacePathMechanic {
    __padding_end: [u8; 1472],
}
impl UCollectSurfacePathMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollectSurfacePathMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollectSurfacePathMechanic")
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
pub struct UCollisionPrimitivesMechanic {
    __padding_end: [u8; 1456],
}
impl UCollisionPrimitivesMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionPrimitivesMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionPrimitivesMechanic")
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
pub struct UConstructionPlaneMechanic {
    __padding_end: [u8; 256],
}
impl UConstructionPlaneMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstructionPlaneMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstructionPlaneMechanic")
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
pub struct UCurveControlPointsMechanic {
    __padding_end: [u8; 1664],
}
impl UCurveControlPointsMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveControlPointsMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveControlPointsMechanic")
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
pub struct UDragAlignmentMechanic {
    __padding_end: [u8; 640],
}
impl UDragAlignmentMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDragAlignmentMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDragAlignmentMechanic")
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
pub struct UDragAlignmentInteraction {
    __padding_end: [u8; 736],
}
impl UDragAlignmentInteraction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDragAlignmentInteraction")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDragAlignmentInteraction")
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
pub struct ULatticeControlPointsMechanic {
    __padding_end: [u8; 1296],
}
impl ULatticeControlPointsMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeControlPointsMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULatticeControlPointsMechanic")
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
pub struct UPlaneDistanceFromHitMechanic {
    __padding_end: [u8; 1312],
}
impl UPlaneDistanceFromHitMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneDistanceFromHitMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlaneDistanceFromHitMechanic")
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
pub struct UPolyLassoMarqueeMechanic {
    __padding_end: [u8; 512],
}
impl UPolyLassoMarqueeMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyLassoMarqueeMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolyLassoMarqueeMechanic")
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
pub struct URectangleMarqueeMechanic {
    __padding_end: [u8; 576],
}
impl URectangleMarqueeMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URectangleMarqueeMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URectangleMarqueeMechanic")
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
pub struct URectangleMarqueeInteraction {
    __padding_end: [u8; 592],
}
impl URectangleMarqueeInteraction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URectangleMarqueeInteraction")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URectangleMarqueeInteraction")
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
pub struct USpaceCurveDeformationMechanicPropertySet {
    __padding_end: [u8; 200],
}
impl USpaceCurveDeformationMechanicPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceCurveDeformationMechanicPropertySet")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceCurveDeformationMechanicPropertySet")
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
pub struct USpaceCurveDeformationMechanic {
    __padding_end: [u8; 736],
}
impl USpaceCurveDeformationMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceCurveDeformationMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpaceCurveDeformationMechanic")
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
pub struct USpatialCurveDistanceMechanic {
    __padding_end: [u8; 1040],
}
impl USpatialCurveDistanceMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpatialCurveDistanceMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpatialCurveDistanceMechanic")
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
pub struct UMeshOpPreviewWithBackgroundCompute {
    __padding_end: [u8; 216],
}
impl UMeshOpPreviewWithBackgroundCompute {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshOpPreviewWithBackgroundCompute")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshOpPreviewWithBackgroundCompute")
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
pub struct UModelingComponentsSettings {
    __padding_end: [u8; 112],
}
impl UModelingComponentsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingComponentsSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingComponentsSettings")
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
pub struct UModelingComponentsEditorSettings {
    __padding_end: [u8; 128],
}
impl UModelingComponentsEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingComponentsEditorSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingComponentsEditorSettings")
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
pub struct UModelingObjectsCreationAPI {
    __padding_end: [u8; 48],
}
impl UModelingObjectsCreationAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingObjectsCreationAPI")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingObjectsCreationAPI")
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
    pub fn create_texture_object(
        &mut self,
        create_tex_params: &FCreateTextureObjectParams,
    ) -> FCreateTextureObjectResult {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_texture_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                create_tex_params,
                __buffer.add(0).cast::<FCreateTextureObjectParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_texture_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<FCreateTextureObjectResult>().read() }
    }
    pub fn create_new_component_on_actor(
        &mut self,
        create_component_params: &FCreateComponentParams,
    ) -> FCreateComponentResult {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_new_component_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                create_component_params,
                __buffer.add(0).cast::<FCreateComponentParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_new_component_on_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FCreateComponentResult>().read() }
    }
    pub fn create_new_actor(
        &mut self,
        create_actor_params: &FCreateActorParams,
    ) -> FCreateActorResult {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_new_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                create_actor_params,
                __buffer.add(0).cast::<FCreateActorParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_new_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FCreateActorResult>().read() }
    }
    pub fn create_mesh_object(
        &mut self,
        create_mesh_params: &FCreateMeshObjectParams,
    ) -> FCreateMeshObjectResult {
        let mut __stack = crate::core_data::StackAlloc::<1744>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_mesh_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                create_mesh_params,
                __buffer.add(0).cast::<FCreateMeshObjectParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_mesh_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(1712).cast::<FCreateMeshObjectResult>().read() }
    }
    pub fn create_material_object(
        &mut self,
        create_material_params: &FCreateMaterialObjectParams,
    ) -> FCreateMaterialObjectResult {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_material_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                create_material_params,
                __buffer.add(0).cast::<FCreateMaterialObjectParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::modeling_components::__FUNCTION_PTRS
                    .u_modeling_objects_creation_api_create_material_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FCreateMaterialObjectResult>().read() }
    }
}
#[repr(C, align(8))]
pub struct APreviewMeshActor {
    __padding_end: [u8; 1144],
}
impl APreviewMeshActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APreviewMeshActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APreviewMeshActor")
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
pub struct UCreateMeshObjectTypeProperties {
    __padding_end: [u8; 232],
}
impl UCreateMeshObjectTypeProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateMeshObjectTypeProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateMeshObjectTypeProperties")
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
pub struct UGeometrySelectionVisualizationProperties {
    __padding_end: [u8; 280],
}
impl UGeometrySelectionVisualizationProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionVisualizationProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionVisualizationProperties")
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
pub struct UOnAcceptHandleSourcesPropertiesSingle {
    __padding_end: [u8; 192],
}
impl UOnAcceptHandleSourcesPropertiesSingle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesPropertiesSingle")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnAcceptHandleSourcesPropertiesSingle")
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
pub struct UPolygroupLayersProperties {
    __padding_end: [u8; 216],
}
impl UPolygroupLayersProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygroupLayersProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygroupLayersProperties")
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
pub struct UWeightMapSetProperties {
    __padding_end: [u8; 224],
}
impl UWeightMapSetProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeightMapSetProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWeightMapSetProperties")
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
pub struct UMeshTopologySelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UMeshTopologySelectionMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTopologySelectionMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTopologySelectionMechanic")
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
pub struct UBoundarySelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UBoundarySelectionMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoundarySelectionMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoundarySelectionMechanic")
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
pub struct UGeometrySelectionManager {
    __padding_end: [u8; 1240],
}
impl UGeometrySelectionManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeometrySelectionManager")
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
pub struct UMeshTopologySelectionMechanicProperties {
    __padding_end: [u8; 208],
}
impl UMeshTopologySelectionMechanicProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTopologySelectionMechanicProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshTopologySelectionMechanicProperties")
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
pub struct UDEPRECATED_PolygonSelectionMechanicProperties {
    __padding_end: [u8; 208],
}
impl UDEPRECATED_PolygonSelectionMechanicProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_PolygonSelectionMechanicProperties")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_PolygonSelectionMechanicProperties")
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
pub struct UPolygonSelectionMechanic {
    __padding_end: [u8; 2880],
}
impl UPolygonSelectionMechanic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonSelectionMechanic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolygonSelectionMechanic")
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
pub struct UModelingSceneSnappingManager {
    __padding_end: [u8; 344],
}
impl UModelingSceneSnappingManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingSceneSnappingManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelingSceneSnappingManager")
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
pub struct IToolHostCustomizationAPI {}
#[repr(C, align(8))]
pub struct UToolHostCustomizationAPI {
    __padding_end: [u8; 48],
}
impl UToolHostCustomizationAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolHostCustomizationAPI")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UToolHostCustomizationAPI")
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
pub struct UMultiTransformer {
    __padding_end: [u8; 432],
}
impl UMultiTransformer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTransformer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMultiTransformer")
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
#[repr(transparent)]
pub struct ECreateObjectTypeHint(pub u8);
impl ECreateObjectTypeHint {
    pub const UNDEFINED: ECreateObjectTypeHint = ECreateObjectTypeHint(0);
    pub const STATIC_MESH: ECreateObjectTypeHint = ECreateObjectTypeHint(1);
    pub const VOLUME: ECreateObjectTypeHint = ECreateObjectTypeHint(2);
    pub const DYNAMIC_MESH_ACTOR: ECreateObjectTypeHint = ECreateObjectTypeHint(3);
}
#[repr(transparent)]
pub struct ECreateModelingObjectResult(pub u8);
impl ECreateModelingObjectResult {
    pub const OK: ECreateModelingObjectResult = ECreateModelingObjectResult(0);
    pub const CANCELLED: ECreateModelingObjectResult = ECreateModelingObjectResult(1);
    pub const FAILED_UNKNOWN: ECreateModelingObjectResult = ECreateModelingObjectResult(
        2,
    );
    pub const FAILED_NO_API_FOUND: ECreateModelingObjectResult = ECreateModelingObjectResult(
        3,
    );
    pub const FAILED_INVALID_WORLD: ECreateModelingObjectResult = ECreateModelingObjectResult(
        4,
    );
    pub const FAILED_INVALID_MESH: ECreateModelingObjectResult = ECreateModelingObjectResult(
        5,
    );
    pub const FAILED_INVALID_TEXTURE: ECreateModelingObjectResult = ECreateModelingObjectResult(
        6,
    );
    pub const FAILED_ASSET_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        7,
    );
    pub const FAILED_ACTOR_CREATION_FAILED: ECreateModelingObjectResult = ECreateModelingObjectResult(
        8,
    );
    pub const FAILED_INVALID_MATERIAL: ECreateModelingObjectResult = ECreateModelingObjectResult(
        9,
    );
    pub const FAILED_INVALID_ACTOR: ECreateModelingObjectResult = ECreateModelingObjectResult(
        10,
    );
}
#[repr(transparent)]
pub struct EHandleSourcesMethod(pub u8);
impl EHandleSourcesMethod {
    pub const DELETE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(0);
    pub const HIDE_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(1);
    pub const KEEP_SOURCES: EHandleSourcesMethod = EHandleSourcesMethod(2);
    pub const KEEP_FIRST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(3);
    pub const KEEP_LAST_SOURCE: EHandleSourcesMethod = EHandleSourcesMethod(4);
}
#[repr(transparent)]
pub struct EBaseCreateFromSelectedTargetType(pub i32);
impl EBaseCreateFromSelectedTargetType {
    pub const NEW_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        0,
    );
    pub const FIRST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        1,
    );
    pub const LAST_INPUT_OBJECT: EBaseCreateFromSelectedTargetType = EBaseCreateFromSelectedTargetType(
        2,
    );
}
#[repr(transparent)]
pub struct EUVLayoutPreviewSide(pub i32);
impl EUVLayoutPreviewSide {
    pub const LEFT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(0);
    pub const RIGHT: EUVLayoutPreviewSide = EUVLayoutPreviewSide(1);
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointTransformMode(pub i32);
impl ESpaceCurveControlPointTransformMode {
    pub const SHARED: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        0,
    );
    pub const PER_VERTEX: ESpaceCurveControlPointTransformMode = ESpaceCurveControlPointTransformMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointOriginMode(pub i32);
impl ESpaceCurveControlPointOriginMode {
    pub const SHARED: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        0,
    );
    pub const FIRST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        1,
    );
    pub const LAST: ESpaceCurveControlPointOriginMode = ESpaceCurveControlPointOriginMode(
        2,
    );
}
#[repr(transparent)]
pub struct ESpaceCurveControlPointFalloffType(pub i32);
impl ESpaceCurveControlPointFalloffType {
    pub const LINEAR: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        0,
    );
    pub const SMOOTH: ESpaceCurveControlPointFalloffType = ESpaceCurveControlPointFalloffType(
        1,
    );
}
#[repr(transparent)]
pub struct EModelingComponentsPlaneVisualizationMode(pub u8);
impl EModelingComponentsPlaneVisualizationMode {
    pub const SIMPLE_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        0,
    );
    pub const HIERARCHICAL_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        1,
    );
    pub const FIXED_SCREEN_AREA_GRID: EModelingComponentsPlaneVisualizationMode = EModelingComponentsPlaneVisualizationMode(
        2,
    );
}
#[repr(transparent)]
pub struct EGeometrySelectionElementType(pub u8);
impl EGeometrySelectionElementType {
    pub const VERTEX: EGeometrySelectionElementType = EGeometrySelectionElementType(1);
    pub const EDGE: EGeometrySelectionElementType = EGeometrySelectionElementType(2);
    pub const FACE: EGeometrySelectionElementType = EGeometrySelectionElementType(4);
}
#[repr(transparent)]
pub struct EGeometrySelectionTopologyType(pub u8);
impl EGeometrySelectionTopologyType {
    pub const TRIANGLE: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        1,
    );
    pub const POLYGROUP: EGeometrySelectionTopologyType = EGeometrySelectionTopologyType(
        2,
    );
}
#[repr(transparent)]
pub struct EMarqueeSelectionUpdateType(pub i32);
impl EMarqueeSelectionUpdateType {
    pub const ON_DRAG: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(0);
    pub const ON_TICK_AND_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(
        1,
    );
    pub const ON_RELEASE: EMarqueeSelectionUpdateType = EMarqueeSelectionUpdateType(2);
}
#[repr(transparent)]
pub struct ELatticeInterpolationType(pub u8);
impl ELatticeInterpolationType {
    pub const LINEAR: ELatticeInterpolationType = ELatticeInterpolationType(0);
    pub const CUBIC: ELatticeInterpolationType = ELatticeInterpolationType(1);
}
#[repr(transparent)]
pub struct EBakeTextureResolution(pub i32);
impl EBakeTextureResolution {
    pub const RESOLUTION16: EBakeTextureResolution = EBakeTextureResolution(16);
    pub const RESOLUTION32: EBakeTextureResolution = EBakeTextureResolution(32);
    pub const RESOLUTION64: EBakeTextureResolution = EBakeTextureResolution(64);
    pub const RESOLUTION128: EBakeTextureResolution = EBakeTextureResolution(128);
    pub const RESOLUTION256: EBakeTextureResolution = EBakeTextureResolution(256);
    pub const RESOLUTION512: EBakeTextureResolution = EBakeTextureResolution(512);
    pub const RESOLUTION1024: EBakeTextureResolution = EBakeTextureResolution(1024);
    pub const RESOLUTION2048: EBakeTextureResolution = EBakeTextureResolution(2048);
    pub const RESOLUTION4096: EBakeTextureResolution = EBakeTextureResolution(4096);
    pub const RESOLUTION8192: EBakeTextureResolution = EBakeTextureResolution(8192);
}
#[repr(transparent)]
pub struct EBakeTextureBitDepth(pub i32);
impl EBakeTextureBitDepth {
    pub const CHANNEL_BITS8: EBakeTextureBitDepth = EBakeTextureBitDepth(0);
    pub const CHANNEL_BITS16: EBakeTextureBitDepth = EBakeTextureBitDepth(1);
}
#[repr(transparent)]
pub struct EBakeTextureSamplesPerPixel(pub i32);
impl EBakeTextureSamplesPerPixel {
    pub const SAMPLE1: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(1);
    pub const SAMPLE4: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(4);
    pub const SAMPLE16: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(16);
    pub const SAMPLE64: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(64);
    pub const SAMPLE256: EBakeTextureSamplesPerPixel = EBakeTextureSamplesPerPixel(256);
}
