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
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_STRIP_LOD_GEOMETRY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SKELETAL_MESH_OVERLAY_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_VISIBLE_IN_RAY_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_RECOMPUTE_TANGENTS_VERTEX_MASK_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_RECOMPUTE_TANGENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_CAST_SHADOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MORPH_TARGETS_TO_GENERATED_BY_ENGINE_FOR_ALL_SKELETAL_MESH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MORPH_TARGETS_TO_GENERATED_BY_ENGINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MATERIAL_SLOT_OVERLAY_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_RENAME_SOCKET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REMOVE_LO_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REIMPORT_ALL_CUSTOM_LO_DS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REGENERATE_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_IS_PHYSICS_ASSET_COMPATIBLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_IMPORT_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SKELETON_CURVE_META_DATA_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SKELETAL_MESH_OVERLAY_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_VISIBLE_IN_RAY_TRACING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_RECOMPUTE_TANGENTS_VERTEX_MASK_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_RECOMPUTE_TANGENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_CAST_SHADOW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_NUM_VERTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_NUM_SECTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_MORPH_TARGETS_GENERATED_BY_ENGINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_MATERIAL_SLOT_OVERLAY_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_MATERIAL_SLOT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_BUILD_SETTINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_CREATE_PHYSICS_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_ASSIGN_PHYSICS_ASSET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkeletalMeshEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StripLODGeometry"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_STRIP_LOD_GEOMETRY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMeshOverlayMaterial"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SKELETAL_MESH_OVERLAY_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionVisibleInRayTracing"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_VISIBLE_IN_RAY_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionRecomputeTangentsVertexMaskChannel"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_RECOMPUTE_TANGENTS_VERTEX_MASK_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionRecomputeTangent"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_RECOMPUTE_TANGENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionCastShadow"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_SECTION_CAST_SHADOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "SetMorphTargetsToGeneratedByEngineForAllSkeletalMesh",
            ),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MORPH_TARGETS_TO_GENERATED_BY_ENGINE_FOR_ALL_SKELETAL_MESH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTargetsToGeneratedByEngine"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MORPH_TARGETS_TO_GENERATED_BY_ENGINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialSlotOverlayMaterial"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_MATERIAL_SLOT_OVERLAY_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLodBuildSettings"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_SET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameSocket"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_RENAME_SOCKET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLODs"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REMOVE_LO_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReimportAllCustomLODs"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REIMPORT_ALL_CUSTOM_LO_DS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegenerateLOD"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_REGENERATE_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPhysicsAssetCompatible"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_IS_PHYSICS_ASSET_COMPATIBLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportLOD"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_IMPORT_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletonCurveMetaDataNames"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SKELETON_CURVE_META_DATA_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletalMeshOverlayMaterial"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SKELETAL_MESH_OVERLAY_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionVisibleInRayTracing"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_VISIBLE_IN_RAY_TRACING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionRecomputeTangentsVertexMaskChannel"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_RECOMPUTE_TANGENTS_VERTEX_MASK_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionRecomputeTangent"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_RECOMPUTE_TANGENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionCastShadow"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_SECTION_CAST_SHADOW,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumVerts"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_NUM_VERTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumSections"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_NUM_SECTIONS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetsGeneratedByEngine"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_MORPH_TARGETS_GENERATED_BY_ENGINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialSlotOverlayMaterial"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_MATERIAL_SLOT_OVERLAY_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODMaterialSlot"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_MATERIAL_SLOT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLODCount"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLodBuildSettings"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_GET_LOD_BUILD_SETTINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePhysicsAsset"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_CREATE_PHYSICS_ASSET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AssignPhysicsAsset"),
            &raw mut U_SKELETAL_MESH_EDITOR_SUBSYSTEM_ASSIGN_PHYSICS_ASSET,
        );
    }
}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorContextMenuContext {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorContextMenuContext")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorUISubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorUISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorUISubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USkeletalMeshEditorSubsystem {
    __padding_end: [u8; 56],
}
impl USkeletalMeshEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorSubsystem")
            .unwrap()
    }
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
pub struct ESkelSubSysQueryCurvesMetatdataNamesFilter(pub u8);
impl ESkelSubSysQueryCurvesMetatdataNamesFilter {
    pub const ALL: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        0,
    );
    pub const MORPH_TARGET_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        1,
    );
    pub const MATERIAL_ONLY: ESkelSubSysQueryCurvesMetatdataNamesFilter = ESkelSubSysQueryCurvesMetatdataNamesFilter(
        2,
    );
}
