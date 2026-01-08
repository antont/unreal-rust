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
    pub u_generate_static_mesh_lod_asset_tool_preset_properties_write_to_preset: *mut crate::ffi::UFunctionOpague,
    pub u_generate_static_mesh_lod_asset_tool_preset_properties_read_from_preset: *mut crate::ffi::UFunctionOpague,
    pub u_generate_static_mesh_lod_asset_tool_properties_get_group_layers_func: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_preview_lod_properties_get_lod_names_func: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_hi_res_source_model_actions_move_to_lod0: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_hi_res_source_model_actions_delete: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_material_actions_clean_materials: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_tool_remove_unreferenced_materials: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_tool_move_hi_res_to_lod0: *mut crate::ffi::UFunctionOpague,
    pub ulod_manager_tool_delete_hi_res_source_model: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_generate_static_mesh_lod_asset_tool_preset_properties_write_to_preset: std::ptr::null_mut(),
            u_generate_static_mesh_lod_asset_tool_preset_properties_read_from_preset: std::ptr::null_mut(),
            u_generate_static_mesh_lod_asset_tool_properties_get_group_layers_func: std::ptr::null_mut(),
            ulod_manager_preview_lod_properties_get_lod_names_func: std::ptr::null_mut(),
            ulod_manager_hi_res_source_model_actions_move_to_lod0: std::ptr::null_mut(),
            ulod_manager_hi_res_source_model_actions_delete: std::ptr::null_mut(),
            ulod_manager_material_actions_clean_materials: std::ptr::null_mut(),
            ulod_manager_tool_remove_unreferenced_materials: std::ptr::null_mut(),
            ulod_manager_tool_move_hi_res_to_lod0: std::ptr::null_mut(),
            ulod_manager_tool_delete_hi_res_source_model: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGenerateStaticMeshLODAssetToolPresetProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteToPreset"),
            &raw mut __FUNCTION_PTRS
                .u_generate_static_mesh_lod_asset_tool_preset_properties_write_to_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFromPreset"),
            &raw mut __FUNCTION_PTRS
                .u_generate_static_mesh_lod_asset_tool_preset_properties_read_from_preset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGenerateStaticMeshLODAssetToolProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGroupLayersFunc"),
            &raw mut __FUNCTION_PTRS
                .u_generate_static_mesh_lod_asset_tool_properties_get_group_layers_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULODManagerPreviewLODProperties::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODNamesFunc"),
            &raw mut __FUNCTION_PTRS
                .ulod_manager_preview_lod_properties_get_lod_names_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULODManagerHiResSourceModelActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveToLOD0"),
            &raw mut __FUNCTION_PTRS
                .ulod_manager_hi_res_source_model_actions_move_to_lod0,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Delete"),
            &raw mut __FUNCTION_PTRS.ulod_manager_hi_res_source_model_actions_delete,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULODManagerMaterialActions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CleanMaterials"),
            &raw mut __FUNCTION_PTRS.ulod_manager_material_actions_clean_materials,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULODManagerTool::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUnreferencedMaterials"),
            &raw mut __FUNCTION_PTRS.ulod_manager_tool_remove_unreferenced_materials,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveHiResToLOD0"),
            &raw mut __FUNCTION_PTRS.ulod_manager_tool_move_hi_res_to_lod0,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteHiResSourceModel"),
            &raw mut __FUNCTION_PTRS.ulod_manager_tool_delete_hi_res_source_model,
        );
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_StaticMeshLODGenerationSettings {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_StaticMeshLODGenerationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_StaticMeshLODGenerationSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODProcess {
    __padding_end: [u8; 2096],
}
impl UGenerateStaticMeshLODProcess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODProcess")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolBuilder {
    __padding_end: [u8; 56],
}
impl UGenerateStaticMeshLODAssetToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetToolBuilder")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolOutputProperties {
    __padding_end: [u8; 240],
}
impl UGenerateStaticMeshLODAssetToolOutputProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetToolOutputProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolPresetProperties {
    __padding_end: [u8; 200],
}
impl UGenerateStaticMeshLODAssetToolPresetProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetToolPresetProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolProperties {
    __padding_end: [u8; 376],
}
impl UGenerateStaticMeshLODAssetToolProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetToolProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetToolTextureProperties {
    __padding_end: [u8; 232],
}
impl UGenerateStaticMeshLODAssetToolTextureProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetToolTextureProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UGenerateStaticMeshLODAssetTool {
    __padding_end: [u8; 352],
}
impl UGenerateStaticMeshLODAssetTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateStaticMeshLODAssetTool")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStaticMeshLODGenerationSettings {
    __padding_end: [u8; 216],
}
impl UStaticMeshLODGenerationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshLODGenerationSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStaticMeshLODGenerationSettingsFactory {
    __padding_end: [u8; 136],
}
impl UStaticMeshLODGenerationSettingsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshLODGenerationSettingsFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerToolBuilder {
    __padding_end: [u8; 48],
}
impl ULODManagerToolBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerToolBuilder")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerLODProperties {
    __padding_end: [u8; 256],
}
impl ULODManagerLODProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerLODProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerPreviewLODProperties {
    __padding_end: [u8; 224],
}
impl ULODManagerPreviewLODProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerPreviewLODProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerActionPropertySet {
    __padding_end: [u8; 192],
}
impl ULODManagerActionPropertySet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerActionPropertySet")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerHiResSourceModelActions {
    __padding_end: [u8; 192],
}
impl ULODManagerHiResSourceModelActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerHiResSourceModelActions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerMaterialActions {
    __padding_end: [u8; 192],
}
impl ULODManagerMaterialActions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerMaterialActions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ILODManagerToolChangeTarget {}
#[repr(C, align(8))]
pub struct ULODManagerToolChangeTarget {
    __padding_end: [u8; 48],
}
impl ULODManagerToolChangeTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerToolChangeTarget")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULODManagerTool {
    __padding_end: [u8; 408],
}
impl ULODManagerTool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULODManagerTool")
            .unwrap()
    }
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
pub struct EGenerateStaticMeshLODProcess_MeshGeneratorModes(pub u8);
impl EGenerateStaticMeshLODProcess_MeshGeneratorModes {
    pub const SOLIDIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        0,
    );
    pub const SOLIDIFY_AND_CLOSE: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        1,
    );
    pub const CLEAN_AND_SIMPLIFY: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        2,
    );
    pub const CONVEX_HULL: EGenerateStaticMeshLODProcess_MeshGeneratorModes = EGenerateStaticMeshLODProcess_MeshGeneratorModes(
        3,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_SimplifyMethod(pub u8);
impl EGenerateStaticMeshLODProcess_SimplifyMethod {
    pub const TRIANGLE_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        0,
    );
    pub const VERTEX_COUNT: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        1,
    );
    pub const TRIANGLE_PERCENTAGE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        2,
    );
    pub const GEOMETRIC_TOLERANCE: EGenerateStaticMeshLODProcess_SimplifyMethod = EGenerateStaticMeshLODProcess_SimplifyMethod(
        3,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_NormalsMethod(pub u8);
impl EGenerateStaticMeshLODProcess_NormalsMethod {
    pub const FROM_ANGLE_THRESHOLD: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        0,
    );
    pub const PER_VERTEX: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        1,
    );
    pub const PER_TRIANGLE: EGenerateStaticMeshLODProcess_NormalsMethod = EGenerateStaticMeshLODProcess_NormalsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProcess_AutoUVMethod(pub u8);
impl EGenerateStaticMeshLODProcess_AutoUVMethod {
    pub const PATCH_BUILDER: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        0,
    );
    pub const UV_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        1,
    );
    pub const X_ATLAS: EGenerateStaticMeshLODProcess_AutoUVMethod = EGenerateStaticMeshLODProcess_AutoUVMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODBakeResolution(pub i32);
impl EGenerateStaticMeshLODBakeResolution {
    pub const RESOLUTION16: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        16,
    );
    pub const RESOLUTION32: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        32,
    );
    pub const RESOLUTION64: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        64,
    );
    pub const RESOLUTION128: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        128,
    );
    pub const RESOLUTION256: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        256,
    );
    pub const RESOLUTION512: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        512,
    );
    pub const RESOLUTION1024: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        1024,
    );
    pub const RESOLUTION2048: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        2048,
    );
    pub const RESOLUTION4096: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        4096,
    );
    pub const RESOLUTION8192: EGenerateStaticMeshLODBakeResolution = EGenerateStaticMeshLODBakeResolution(
        8192,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODSimpleCollisionGeometryType(pub u8);
impl EGenerateStaticMeshLODSimpleCollisionGeometryType {
    pub const ALIGNED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        0,
    );
    pub const ORIENTED_BOXES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        1,
    );
    pub const MINIMAL_SPHERES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        2,
    );
    pub const CAPSULES: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        3,
    );
    pub const CONVEX_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        4,
    );
    pub const SWEPT_HULLS: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        5,
    );
    pub const MIN_VOLUME: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        6,
    );
    pub const NONE: EGenerateStaticMeshLODSimpleCollisionGeometryType = EGenerateStaticMeshLODSimpleCollisionGeometryType(
        7,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLODProjectedHullAxisMode(pub u8);
impl EGenerateStaticMeshLODProjectedHullAxisMode {
    pub const X: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        0,
    );
    pub const Y: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        1,
    );
    pub const Z: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        2,
    );
    pub const SMALLEST_BOX_DIMENSION: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        3,
    );
    pub const SMALLEST_VOLUME: EGenerateStaticMeshLODProjectedHullAxisMode = EGenerateStaticMeshLODProjectedHullAxisMode(
        4,
    );
}
#[repr(transparent)]
pub struct EGenerateStaticMeshLOD_BakeConstraint(pub i32);
impl EGenerateStaticMeshLOD_BakeConstraint {
    pub const NO_CONSTRAINT: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        0,
    );
    pub const DO_NOT_BAKE: EGenerateStaticMeshLOD_BakeConstraint = EGenerateStaticMeshLOD_BakeConstraint(
        1,
    );
}
#[repr(transparent)]
pub struct EGenerateLODAssetOutputMode(pub u8);
impl EGenerateLODAssetOutputMode {
    pub const CREATE_NEW_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        0,
    );
    pub const UPDATE_EXISTING_ASSET: EGenerateLODAssetOutputMode = EGenerateLODAssetOutputMode(
        1,
    );
}
