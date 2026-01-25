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
    pub u_datasmith_content_blueprint_library_get_datasmith_user_data_values_for_key: *mut crate::ffi::UFunctionOpague,
    pub u_datasmith_content_blueprint_library_get_datasmith_user_data_value_for_key: *mut crate::ffi::UFunctionOpague,
    pub u_datasmith_content_blueprint_library_get_datasmith_user_data_keys_and_values_for_value: *mut crate::ffi::UFunctionOpague,
    pub u_datasmith_content_blueprint_library_get_datasmith_user_data: *mut crate::ffi::UFunctionOpague,
    pub u_datasmith_content_blueprint_library_get_all_objects_and_values_for_key: *mut crate::ffi::UFunctionOpague,
    pub u_datasmith_content_blueprint_library_get_all_datasmith_user_data: *mut crate::ffi::UFunctionOpague,
    pub a_datasmith_imported_sequences_actor_play_level_sequence: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_datasmith_content_blueprint_library_get_datasmith_user_data_values_for_key: std::ptr::null_mut(),
            u_datasmith_content_blueprint_library_get_datasmith_user_data_value_for_key: std::ptr::null_mut(),
            u_datasmith_content_blueprint_library_get_datasmith_user_data_keys_and_values_for_value: std::ptr::null_mut(),
            u_datasmith_content_blueprint_library_get_datasmith_user_data: std::ptr::null_mut(),
            u_datasmith_content_blueprint_library_get_all_objects_and_values_for_key: std::ptr::null_mut(),
            u_datasmith_content_blueprint_library_get_all_datasmith_user_data: std::ptr::null_mut(),
            a_datasmith_imported_sequences_actor_play_level_sequence: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDatasmithContentBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataValuesForKey"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_datasmith_user_data_values_for_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataValueForKey"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_datasmith_user_data_value_for_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserDataKeysAndValuesForValue"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_datasmith_user_data_keys_and_values_for_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDatasmithUserData"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_datasmith_user_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllObjectsAndValuesForKey"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_all_objects_and_values_for_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllDatasmithUserData"),
            &raw mut __FUNCTION_PTRS
                .u_datasmith_content_blueprint_library_get_all_datasmith_user_data,
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
            &raw mut __FUNCTION_PTRS
                .a_datasmith_imported_sequences_actor_play_level_sequence,
        );
    }
}
#[repr(C, align(4))]
pub struct FDatasmithAssetImportOptions {
    pub(crate) __padding_end: [u8; 12],
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
    pub(crate) __padding_32: [u8; 32],
    pub retessellation_rule: EDatasmithCADRetessellationRule,
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
    pub(crate) __padding_1136: [u8; 1136],
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
    pub(crate) __padding_96: [u8; 96],
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
    pub(crate) __padding_48: [u8; 48],
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
    pub fn get_datasmith_user_data_values_for_key(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        key: FName,
        b_partial_match_key: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_values_for_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&key, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_partial_match_key,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_values_for_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_datasmith_user_data_value_for_key(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        key: FName,
        b_partial_match_key: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_value_for_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&key, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_partial_match_key,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_value_for_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_datasmith_user_data_keys_and_values_for_value(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        string_to_match: FString,
        out_keys: &mut TArray<FName>,
        out_values: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_keys_and_values_for_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string_to_match,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_keys,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(40).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data_keys_and_values_for_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FName>>().swap(out_keys);
        }
        unsafe {
            __buffer.add(40).cast::<TArray<FString>>().swap(out_values);
        }
    }
    pub fn get_datasmith_user_data(
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UDatasmithAssetUserData> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_datasmith_user_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UDatasmithAssetUserData>>().read() }
    }
    pub fn get_all_objects_and_values_for_key(
        key: FName,
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        out_objects: &mut TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        out_values: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_all_objects_and_values_for_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&key, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_objects,
                __buffer
                    .add(24)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(40).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_all_objects_and_values_for_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .swap(out_objects);
        }
        unsafe {
            __buffer.add(40).cast::<TArray<FString>>().swap(out_values);
        }
    }
    pub fn get_all_datasmith_user_data(
        object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        out_user_data: &mut TArray<UPtr<UDatasmithAssetUserData>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_all_datasmith_user_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_user_data,
                __buffer.add(8).cast::<TArray<UPtr<UDatasmithAssetUserData>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::datasmith_content::UDatasmithContentBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .u_datasmith_content_blueprint_library_get_all_datasmith_user_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<UDatasmithAssetUserData>>>()
                .swap(out_user_data);
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
    pub(crate) __padding_1136: [u8; 1136],
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
    pub fn play_level_sequence(
        &mut self,
        sequence_to_play: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .a_datasmith_imported_sequences_actor_play_level_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence_to_play,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::datasmith_content::__FUNCTION_PTRS
                    .a_datasmith_imported_sequences_actor_play_level_sequence,
                __buffer,
            )
        };
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
    pub(crate) __padding_48: [u8; 48],
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
    pub(crate) __padding_56: [u8; 56],
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
