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
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_VALUES_FOR_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_VALUE_FOR_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_KEYS_AND_VALUES_FOR_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_ALL_OBJECTS_AND_VALUES_FOR_KEY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_ALL_DATASMITH_USER_DATA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_DATASMITH_IMPORTED_SEQUENCES_ACTOR_PLAY_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDatasmithContentBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataValuesForKey"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_VALUES_FOR_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataValueForKey"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_VALUE_FOR_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataKeysAndValuesForValue"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA_KEYS_AND_VALUES_FOR_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserData"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_DATASMITH_USER_DATA,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllObjectsAndValuesForKey"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_ALL_OBJECTS_AND_VALUES_FOR_KEY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllDatasmithUserData"),
            &raw mut U_DATASMITH_CONTENT_BLUEPRINT_LIBRARY_GET_ALL_DATASMITH_USER_DATA,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ADatasmithImportedSequencesActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PlayLevelSequence"),
            &raw mut A_DATASMITH_IMPORTED_SEQUENCES_ACTOR_PLAY_LEVEL_SEQUENCE,
        );
    }
}
#[repr(C, align(4))]
pub struct FDatasmithAssetImportOptions {
    __padding_end: [u8; 12],
}
impl FDatasmithAssetImportOptions {}
#[repr(C, align(1))]
pub struct FDatasmithStaticMeshImportOptions {
    pub min_lightmap_resolution: EDatasmithImportLightmapMin,
    pub max_lightmap_resolution: EDatasmithImportLightmapMax,
    pub b_generate_lightmap_u_vs: bool,
    pub b_remove_degenerates: bool,
}
impl FDatasmithStaticMeshImportOptions {}
#[repr(C, align(1))]
pub struct FDatasmithReimportOptions {
    pub b_update_actors: bool,
    pub b_respawn_deleted_actors: bool,
}
impl FDatasmithReimportOptions {}
#[repr(C, align(4))]
pub struct FDatasmithImportBaseOptions {
    pub scene_handling: EDatasmithImportScene,
    pub b_include_geometry: bool,
    pub b_include_material: bool,
    pub b_include_light: bool,
    pub b_include_camera: bool,
    pub b_include_animation: bool,
    pub asset_options: FDatasmithAssetImportOptions,
    pub static_mesh_options: FDatasmithStaticMeshImportOptions,
}
impl FDatasmithImportBaseOptions {}
#[repr(C, align(8))]
pub struct FDatasmithTessellationOptions {
    pub chord_tolerance: f32,
    pub max_edge_length: f32,
    pub normal_tolerance: f32,
    pub stitching_technique: EDatasmithCADStitchingTechnique,
    pub geometric_tolerance: f64,
    pub stitching_tolerance: f64,
}
impl FDatasmithTessellationOptions {}
#[repr(C, align(8))]
pub struct FDatasmithRetessellationOptions {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub retessellation_rule: EDatasmithCADRetessellationRule,
    __padding_end: [u8; 7],
}
impl FDatasmithRetessellationOptions {}
#[repr(C, align(8))]
pub struct UDatasmithAdditionalData {
    __padding_end: [u8; 48],
}
impl UDatasmithAdditionalData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithAdditionalData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ADatasmithAreaLightActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub mobility: crate::bindings::engine::EComponentMobility,
    pub light_type: EDatasmithAreaLightActorType,
    pub light_shape: EDatasmithAreaLightActorShape,
    pub dimensions: crate::bindings::core_u_object::FVector2D,
    pub intensity: f32,
    pub intensity_units: crate::bindings::engine::ELightUnits,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub temperature: f32,
    pub ies_texture: UPtr<crate::bindings::engine::UTextureLightProfile>,
    pub b_use_ies_brightness: bool,
    pub ies_brightness_scale: f32,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub source_radius: f32,
    pub source_length: f32,
    pub attenuation_radius: f32,
    pub spotlight_inner_angle: f32,
    pub spotlight_outer_angle: f32,
    __padding_end: [u8; 4],
}
impl ADatasmithAreaLightActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADatasmithAreaLightActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithAssetImportData {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub asset_import_options: FDatasmithAssetImportOptions,
    __padding_end: [u8; 52],
}
impl UDatasmithAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithAssetImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithStaticMeshImportData {
    __padding_end: [u8; 168],
}
impl UDatasmithStaticMeshImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithStaticMeshImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithStaticMeshCADImportData {
    __padding_end: [u8; 264],
}
impl UDatasmithStaticMeshCADImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithStaticMeshCADImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithSceneImportData {
    __padding_end: [u8; 152],
}
impl UDatasmithSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithTranslatedSceneImportData {
    __padding_end: [u8; 168],
}
impl UDatasmithTranslatedSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithTranslatedSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithCADImportSceneData {
    __padding_end: [u8; 184],
}
impl UDatasmithCADImportSceneData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithCADImportSceneData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithMDLSceneImportData {
    __padding_end: [u8; 152],
}
impl UDatasmithMDLSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithMDLSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithGLTFSceneImportData {
    __padding_end: [u8; 224],
}
impl UDatasmithGLTFSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithGLTFSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithStaticMeshGLTFImportData {
    __padding_end: [u8; 184],
}
impl UDatasmithStaticMeshGLTFImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithStaticMeshGLTFImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithFBXSceneImportData {
    __padding_end: [u8; 184],
}
impl UDatasmithFBXSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithFBXSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithDeltaGenAssetImportData {
    __padding_end: [u8; 160],
}
impl UDatasmithDeltaGenAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithDeltaGenAssetImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithDeltaGenSceneImportData {
    __padding_end: [u8; 256],
}
impl UDatasmithDeltaGenSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithDeltaGenSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithVREDAssetImportData {
    __padding_end: [u8; 160],
}
impl UDatasmithVREDAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithVREDAssetImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithVREDSceneImportData {
    __padding_end: [u8; 280],
}
impl UDatasmithVREDSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithVREDSceneImportData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithAssetUserData {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub meta_data: TMap<FName, FString>,
    __padding_end: [u8; 80],
}
impl UDatasmithAssetUserData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithAssetUserData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithContentBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UDatasmithContentBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithContentBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithCustomActionBase {
    __padding_end: [u8; 56],
}
impl UDatasmithCustomActionBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithCustomActionBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ADatasmithImportedSequencesActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub imported_sequences: TArray<
        UPtr<crate::bindings::level_sequence::ULevelSequence>,
    >,
}
impl ADatasmithImportedSequencesActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADatasmithImportedSequencesActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithOptionsBase {
    __padding_end: [u8; 48],
}
impl UDatasmithOptionsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithOptionsBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithCommonTessellationOptions {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub options: FDatasmithTessellationOptions,
}
impl UDatasmithCommonTessellationOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithCommonTessellationOptions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithImportOptions {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub base_options: FDatasmithImportBaseOptions,
    pub reimport_options: FDatasmithReimportOptions,
    pub file_name: FString,
    pub file_path: FString,
    pub source_uri: FString,
    __padding_end: [u8; 24],
}
impl UDatasmithImportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithImportOptions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithScene {
    __padding_end: [u8; 736],
}
impl UDatasmithScene {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithScene")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ADatasmithSceneActor {
    __padding_end: [u8; 1232],
}
impl ADatasmithSceneActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADatasmithSceneActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithObjectTemplate {
    __padding_end: [u8; 56],
}
impl UDatasmithObjectTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithObjectTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithActorTemplate {
    __padding_end: [u8; 216],
}
impl UDatasmithActorTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithActorTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithAreaLightActorTemplate {
    __padding_end: [u8; 208],
}
impl UDatasmithAreaLightActorTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithAreaLightActorTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithCineCameraActorTemplate {
    __padding_end: [u8; 112],
}
impl UDatasmithCineCameraActorTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithCineCameraActorTemplate")
            .unwrap()
    }
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
pub struct UDatasmithCineCameraComponentTemplate {
    __padding_end: [u8; 160],
}
impl UDatasmithCineCameraComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithCineCameraComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithDecalComponentTemplate {
    __padding_end: [u8; 96],
}
impl UDatasmithDecalComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithDecalComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithLandscapeTemplate {
    __padding_end: [u8; 72],
}
impl UDatasmithLandscapeTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithLandscapeTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithLightComponentTemplate {
    __padding_end: [u8; 112],
}
impl UDatasmithLightComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithLightComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithMaterialInstanceTemplate {
    __padding_end: [u8; 424],
}
impl UDatasmithMaterialInstanceTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithMaterialInstanceTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithPointLightComponentTemplate {
    __padding_end: [u8; 72],
}
impl UDatasmithPointLightComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithPointLightComponentTemplate")
            .unwrap()
    }
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
pub struct UDatasmithPostProcessVolumeTemplate {
    __padding_end: [u8; 144],
}
impl UDatasmithPostProcessVolumeTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithPostProcessVolumeTemplate")
            .unwrap()
    }
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
pub struct UDatasmithSceneComponentTemplate {
    __padding_end: [u8; 304],
}
impl UDatasmithSceneComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithSceneComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithSkyLightComponentTemplate {
    __padding_end: [u8; 72],
}
impl UDatasmithSkyLightComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithSkyLightComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithSpotLightComponentTemplate {
    __padding_end: [u8; 64],
}
impl UDatasmithSpotLightComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithSpotLightComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithStaticMeshComponentTemplate {
    __padding_end: [u8; 80],
}
impl UDatasmithStaticMeshComponentTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithStaticMeshComponentTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UDatasmithStaticMeshTemplate {
    __padding_end: [u8; 176],
}
impl UDatasmithStaticMeshTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDatasmithStaticMeshTemplate")
            .unwrap()
    }
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
pub struct EDatasmithImportLightmapMin(pub u8);
impl EDatasmithImportLightmapMin {
    pub const LIGHTMAP_16: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(0);
    pub const LIGHTMAP_32: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(1);
    pub const LIGHTMAP_64: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(2);
    pub const LIGHTMAP_128: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(3);
    pub const LIGHTMAP_256: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(4);
    pub const LIGHTMAP_512: EDatasmithImportLightmapMin = EDatasmithImportLightmapMin(5);
}
#[repr(transparent)]
pub struct EDatasmithImportLightmapMax(pub u8);
impl EDatasmithImportLightmapMax {
    pub const LIGHTMAP_64: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(0);
    pub const LIGHTMAP_128: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(1);
    pub const LIGHTMAP_256: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(2);
    pub const LIGHTMAP_512: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(3);
    pub const LIGHTMAP_1024: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(
        4,
    );
    pub const LIGHTMAP_2048: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(
        5,
    );
    pub const LIGHTMAP_4096: EDatasmithImportLightmapMax = EDatasmithImportLightmapMax(
        6,
    );
}
#[repr(transparent)]
pub struct EDatasmithImportScene(pub u8);
impl EDatasmithImportScene {
    pub const NEW_LEVEL: EDatasmithImportScene = EDatasmithImportScene(0);
    pub const CURRENT_LEVEL: EDatasmithImportScene = EDatasmithImportScene(1);
    pub const ASSETS_ONLY: EDatasmithImportScene = EDatasmithImportScene(2);
}
#[repr(transparent)]
pub struct EDatasmithCADStitchingTechnique(pub u8);
impl EDatasmithCADStitchingTechnique {
    pub const STITCHING_NONE: EDatasmithCADStitchingTechnique = EDatasmithCADStitchingTechnique(
        0,
    );
    pub const STITCHING_HEAL: EDatasmithCADStitchingTechnique = EDatasmithCADStitchingTechnique(
        1,
    );
    pub const STITCHING_SEW: EDatasmithCADStitchingTechnique = EDatasmithCADStitchingTechnique(
        2,
    );
}
#[repr(transparent)]
pub struct EDatasmithCADRetessellationRule(pub u8);
impl EDatasmithCADRetessellationRule {
    pub const ALL: EDatasmithCADRetessellationRule = EDatasmithCADRetessellationRule(0);
    pub const SKIP_DELETED_SURFACES: EDatasmithCADRetessellationRule = EDatasmithCADRetessellationRule(
        1,
    );
}
#[repr(transparent)]
pub struct EDatasmithAreaLightActorType(pub u8);
impl EDatasmithAreaLightActorType {
    pub const POINT: EDatasmithAreaLightActorType = EDatasmithAreaLightActorType(0);
    pub const SPOT: EDatasmithAreaLightActorType = EDatasmithAreaLightActorType(1);
    pub const IES_DEPRECATED: EDatasmithAreaLightActorType = EDatasmithAreaLightActorType(
        2,
    );
    pub const RECT: EDatasmithAreaLightActorType = EDatasmithAreaLightActorType(3);
}
#[repr(transparent)]
pub struct EDatasmithAreaLightActorShape(pub u8);
impl EDatasmithAreaLightActorShape {
    pub const RECTANGLE: EDatasmithAreaLightActorShape = EDatasmithAreaLightActorShape(
        0,
    );
    pub const DISC: EDatasmithAreaLightActorShape = EDatasmithAreaLightActorShape(1);
    pub const SPHERE: EDatasmithAreaLightActorShape = EDatasmithAreaLightActorShape(2);
    pub const CYLINDER: EDatasmithAreaLightActorShape = EDatasmithAreaLightActorShape(3);
    pub const NONE: EDatasmithAreaLightActorShape = EDatasmithAreaLightActorShape(4);
}
#[repr(transparent)]
pub struct EDatasmithImportSearchPackagePolicy(pub u8);
impl EDatasmithImportSearchPackagePolicy {
    pub const CURRENT: EDatasmithImportSearchPackagePolicy = EDatasmithImportSearchPackagePolicy(
        0,
    );
    pub const ALL: EDatasmithImportSearchPackagePolicy = EDatasmithImportSearchPackagePolicy(
        1,
    );
}
#[repr(transparent)]
pub struct EDatasmithImportAssetConflictPolicy(pub u8);
impl EDatasmithImportAssetConflictPolicy {
    pub const REPLACE: EDatasmithImportAssetConflictPolicy = EDatasmithImportAssetConflictPolicy(
        0,
    );
    pub const UPDATE: EDatasmithImportAssetConflictPolicy = EDatasmithImportAssetConflictPolicy(
        1,
    );
    pub const USE: EDatasmithImportAssetConflictPolicy = EDatasmithImportAssetConflictPolicy(
        2,
    );
    pub const IGNORE: EDatasmithImportAssetConflictPolicy = EDatasmithImportAssetConflictPolicy(
        3,
    );
}
#[repr(transparent)]
pub struct EDatasmithImportActorPolicy(pub u8);
impl EDatasmithImportActorPolicy {
    pub const UPDATE: EDatasmithImportActorPolicy = EDatasmithImportActorPolicy(0);
    pub const FULL: EDatasmithImportActorPolicy = EDatasmithImportActorPolicy(1);
    pub const IGNORE: EDatasmithImportActorPolicy = EDatasmithImportActorPolicy(2);
}
#[repr(transparent)]
pub struct EDatasmithImportMaterialQuality(pub u8);
impl EDatasmithImportMaterialQuality {
    pub const USE_NO_FRESNEL_CURVES: EDatasmithImportMaterialQuality = EDatasmithImportMaterialQuality(
        0,
    );
    pub const USE_SIMPLIFIER_FRESNEL_CURVES: EDatasmithImportMaterialQuality = EDatasmithImportMaterialQuality(
        1,
    );
    pub const USE_REAL_FRESNEL_CURVES: EDatasmithImportMaterialQuality = EDatasmithImportMaterialQuality(
        2,
    );
}
