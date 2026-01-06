#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut U_OCTREE_DYNAMIC_MESH_COMPONENT_SET_DYNAMIC_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINE_SET_COMPONENT_SET_LINE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINE_SET_COMPONENT_CLEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LINE_SET_COMPONENT_ADD_LINES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_POINT_SET_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_POINT_SET_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_LINE_SET_VISIBILITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_LINE_SET_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_ALL_POINT_SETS_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_SET_ALL_LINE_SETS_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_TRIANGLE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_POINT_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_LINE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_ALL_TRIANGLE_SETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_ALL_POINT_SETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_REMOVE_ALL_LINE_SETS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_GET_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_FIND_TRIANGLE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_FIND_POINT_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_FIND_LINE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_DISCONNECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_CREATE_IN_WORLD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_ADD_TRIANGLE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_ADD_POINT_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PREVIEW_GEOMETRY_ADD_LINE_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_POINT_SET_COMPONENT_SET_POINT_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_POINT_SET_COMPONENT_CLEAR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_POINT_SET_COMPONENT_ADD_POINTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODELING_OBJECTS_CREATION_API_CREATE_TEXTURE_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODELING_OBJECTS_CREATION_API_CREATE_NEW_COMPONENT_ON_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODELING_OBJECTS_CREATION_API_CREATE_NEW_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODELING_OBJECTS_CREATION_API_CREATE_MESH_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MODELING_OBJECTS_CREATION_API_CREATE_MATERIAL_OBJECT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_SHOULD_SHOW_PROPERTY_SET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_GET_OUTPUT_TYPE_NAMES_FUNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_GET_CURRENT_CREATE_MESH_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_POLYGROUP_LAYERS_PROPERTIES_GET_GROUP_LAYERS_FUNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_WEIGHT_MAP_SET_PROPERTIES_GET_WEIGHT_MAPS_FUNC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_TOPOLOGY_SELECTION_MECHANIC_PROPERTIES_SELECT_ALL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MESH_TOPOLOGY_SELECTION_MECHANIC_PROPERTIES_INVERT_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOctreeDynamicMeshComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDynamicMesh"),
            &raw mut U_OCTREE_DYNAMIC_MESH_COMPONENT_SET_DYNAMIC_MESH,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULineSetComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLineMaterial"),
            &raw mut U_LINE_SET_COMPONENT_SET_LINE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Clear"),
            &raw mut U_LINE_SET_COMPONENT_CLEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLines"),
            &raw mut U_LINE_SET_COMPONENT_ADD_LINES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPreviewGeometry::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPointSetVisibility"),
            &raw mut U_PREVIEW_GEOMETRY_SET_POINT_SET_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPointSetMaterial"),
            &raw mut U_PREVIEW_GEOMETRY_SET_POINT_SET_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLineSetVisibility"),
            &raw mut U_PREVIEW_GEOMETRY_SET_LINE_SET_VISIBILITY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLineSetMaterial"),
            &raw mut U_PREVIEW_GEOMETRY_SET_LINE_SET_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllPointSetsMaterial"),
            &raw mut U_PREVIEW_GEOMETRY_SET_ALL_POINT_SETS_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllLineSetsMaterial"),
            &raw mut U_PREVIEW_GEOMETRY_SET_ALL_LINE_SETS_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTriangleSet"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_TRIANGLE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemovePointSet"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_POINT_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLineSet"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_LINE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllTriangleSets"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_ALL_TRIANGLE_SETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllPointSets"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_ALL_POINT_SETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllLineSets"),
            &raw mut U_PREVIEW_GEOMETRY_REMOVE_ALL_LINE_SETS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActor"),
            &raw mut U_PREVIEW_GEOMETRY_GET_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTriangleSet"),
            &raw mut U_PREVIEW_GEOMETRY_FIND_TRIANGLE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPointSet"),
            &raw mut U_PREVIEW_GEOMETRY_FIND_POINT_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindLineSet"),
            &raw mut U_PREVIEW_GEOMETRY_FIND_LINE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Disconnect"),
            &raw mut U_PREVIEW_GEOMETRY_DISCONNECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateInWorld"),
            &raw mut U_PREVIEW_GEOMETRY_CREATE_IN_WORLD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTriangleSet"),
            &raw mut U_PREVIEW_GEOMETRY_ADD_TRIANGLE_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPointSet"),
            &raw mut U_PREVIEW_GEOMETRY_ADD_POINT_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLineSet"),
            &raw mut U_PREVIEW_GEOMETRY_ADD_LINE_SET,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPointSetComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPointMaterial"),
            &raw mut U_POINT_SET_COMPONENT_SET_POINT_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Clear"),
            &raw mut U_POINT_SET_COMPONENT_CLEAR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPoints"),
            &raw mut U_POINT_SET_COMPONENT_ADD_POINTS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UModelingObjectsCreationAPI::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateTextureObject"),
            &raw mut U_MODELING_OBJECTS_CREATION_API_CREATE_TEXTURE_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewComponentOnActor"),
            &raw mut U_MODELING_OBJECTS_CREATION_API_CREATE_NEW_COMPONENT_ON_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewActor"),
            &raw mut U_MODELING_OBJECTS_CREATION_API_CREATE_NEW_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMeshObject"),
            &raw mut U_MODELING_OBJECTS_CREATION_API_CREATE_MESH_OBJECT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMaterialObject"),
            &raw mut U_MODELING_OBJECTS_CREATION_API_CREATE_MATERIAL_OBJECT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCreateMeshObjectTypeProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldShowPropertySet"),
            &raw mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_SHOULD_SHOW_PROPERTY_SET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutputTypeNamesFunc"),
            &raw mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_GET_OUTPUT_TYPE_NAMES_FUNC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentCreateMeshType"),
            &raw mut U_CREATE_MESH_OBJECT_TYPE_PROPERTIES_GET_CURRENT_CREATE_MESH_TYPE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPolygroupLayersProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGroupLayersFunc"),
            &raw mut U_POLYGROUP_LAYERS_PROPERTIES_GET_GROUP_LAYERS_FUNC,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UWeightMapSetProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWeightMapsFunc"),
            &raw mut U_WEIGHT_MAP_SET_PROPERTIES_GET_WEIGHT_MAPS_FUNC,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMeshTopologySelectionMechanicProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectAll"),
            &raw mut U_MESH_TOPOLOGY_SELECTION_MECHANIC_PROPERTIES_SELECT_ALL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InvertSelection"),
            &raw mut U_MESH_TOPOLOGY_SELECTION_MECHANIC_PROPERTIES_INVERT_SELECTION,
        );
    }
}
#[repr(C, align(1))]
pub struct FModelingToolsAxisFilter {
    __padding_end: [u8; 3],
}
impl FModelingToolsAxisFilter {}
#[repr(C, align(1))]
pub struct FModelingToolsColorChannelFilter {
    __padding_end: [u8; 4],
}
impl FModelingToolsColorChannelFilter {}
#[repr(C, align(16))]
pub struct FCreateMeshObjectParams {
    __padding_end: [u8; 1712],
}
impl FCreateMeshObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMeshObjectResult {
    __padding_end: [u8; 32],
}
impl FCreateMeshObjectResult {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectParams {
    __padding_end: [u8; 64],
}
impl FCreateTextureObjectParams {}
#[repr(C, align(8))]
pub struct FCreateTextureObjectResult {
    __padding_end: [u8; 16],
}
impl FCreateTextureObjectResult {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectParams {
    __padding_end: [u8; 40],
}
impl FCreateMaterialObjectParams {}
#[repr(C, align(8))]
pub struct FCreateMaterialObjectResult {
    __padding_end: [u8; 16],
}
impl FCreateMaterialObjectResult {}
#[repr(C, align(16))]
pub struct FCreateActorParams {
    __padding_end: [u8; 144],
}
impl FCreateActorParams {}
#[repr(C, align(8))]
pub struct FCreateActorResult {
    __padding_end: [u8; 16],
}
impl FCreateActorResult {}
#[repr(C, align(8))]
pub struct FCreateComponentParams {
    __padding_end: [u8; 40],
}
impl FCreateComponentParams {}
#[repr(C, align(8))]
pub struct FCreateComponentResult {
    __padding_end: [u8; 16],
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
