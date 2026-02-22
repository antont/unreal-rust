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
    pub ugltf_exporter_export_to_gltf: *mut crate::ffi::UFunctionOpague,
    pub ugltf_export_options_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub ugltf_proxy_options_reset_to_default: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            ugltf_exporter_export_to_gltf: std::ptr::null_mut(),
            ugltf_export_options_reset_to_default: std::ptr::null_mut(),
            ugltf_proxy_options_reset_to_default: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGLTFExporter::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportToGLTF"),
                &raw mut __FUNCTION_PTRS.ugltf_exporter_export_to_gltf,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGLTFExportOptions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetToDefault"),
                &raw mut __FUNCTION_PTRS.ugltf_export_options_reset_to_default,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGLTFProxyOptions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetToDefault"),
                &raw mut __FUNCTION_PTRS.ugltf_proxy_options_reset_to_default,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FGLTFExportMessages {
    pub suggestions: TArray<FString>,
    pub warnings: TArray<FString>,
    pub errors: TArray<FString>,
}
impl FGLTFExportMessages {}
#[repr(C, align(4))]
pub struct FGLTFMaterialBakeSize {
    pub x: i32,
    pub y: i32,
    pub b_auto_detect: bool,
}
impl FGLTFMaterialBakeSize {}
#[repr(C, align(4))]
pub struct FGLTFOverrideMaterialBakeSettings {
    pub b_override_size: bool,
    pub size: FGLTFMaterialBakeSize,
    pub b_override_filter: bool,
    pub filter: crate::bindings::engine::TextureFilter,
    pub b_override_tiling: bool,
    pub tiling: crate::bindings::engine::TextureAddress,
}
impl FGLTFOverrideMaterialBakeSettings {}
#[repr(C, align(8))]
pub struct UGLTFExporter {
    __padding_end: [u8; 128],
}
impl UGLTFExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFExporter")
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
    pub fn export_to_gltf(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        file_path: FString,
        options: UPtr<UGLTFExportOptions>,
        selected_actors: &TSet<UPtr<crate::bindings::engine::AActor>>,
        out_messages: &mut FGLTFExportMessages,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<161>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_exporter_export_to_gltf,
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
                &file_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &options,
                __buffer.add(24).cast::<UPtr<UGLTFExportOptions>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                selected_actors,
                __buffer.add(32).cast::<TSet<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_messages,
                __buffer.add(112).cast::<FGLTFExportMessages>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gltf_exporter::UGLTFExporter::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_exporter_export_to_gltf,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<FGLTFExportMessages>().swap(out_messages);
        }
        std::mem::forget(object);
        std::mem::forget(file_path);
        std::mem::forget(options);
        unsafe { __buffer.add(160).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGLTFAnimSequenceExporter {
    __padding_end: [u8; 128],
}
impl UGLTFAnimSequenceExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFAnimSequenceExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFAnimSequenceExporter")
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
pub struct UGLTFLevelExporter {
    __padding_end: [u8; 128],
}
impl UGLTFLevelExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelExporter")
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
pub struct UGLTFLevelSequenceExporter {
    __padding_end: [u8; 128],
}
impl UGLTFLevelSequenceExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelSequenceExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelSequenceExporter")
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
pub struct UGLTFLevelVariantSetsExporter {
    __padding_end: [u8; 128],
}
impl UGLTFLevelVariantSetsExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelVariantSetsExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFLevelVariantSetsExporter")
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
pub struct UGLTFMaterialExporter {
    __padding_end: [u8; 128],
}
impl UGLTFMaterialExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFMaterialExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFMaterialExporter")
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
pub struct UGLTFSkeletalMeshExporter {
    __padding_end: [u8; 128],
}
impl UGLTFSkeletalMeshExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFSkeletalMeshExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFSkeletalMeshExporter")
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
pub struct UGLTFStaticMeshExporter {
    __padding_end: [u8; 128],
}
impl UGLTFStaticMeshExporter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFStaticMeshExporter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFStaticMeshExporter")
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
pub struct UGLTFExportOptions {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub export_uniform_scale: f32,
    pub b_export_preview_mesh: bool,
    pub b_skip_near_default_values: bool,
    pub b_include_copyright_notice: bool,
    pub b_export_proxy_materials: bool,
    pub b_use_importer_material_mapping: bool,
    pub b_export_unlit_materials: bool,
    pub b_export_clear_coat_materials: bool,
    pub b_export_cloth_materials: bool,
    pub b_export_thin_translucent_materials: bool,
    pub b_export_specular_glossiness_materials: bool,
    pub b_export_emissive_strength: bool,
    pub bake_material_inputs: EGLTFMaterialBakeMode,
    pub default_material_bake_size: FGLTFMaterialBakeSize,
    pub default_material_bake_filter: crate::bindings::engine::TextureFilter,
    pub default_material_bake_tiling: crate::bindings::engine::TextureAddress,
    pub default_input_bake_settings: TMap<
        EGLTFMaterialPropertyGroup,
        FGLTFOverrideMaterialBakeSettings,
    >,
    pub default_level_of_detail: i32,
    pub b_export_source_model: bool,
    pub b_export_vertex_colors: bool,
    pub b_export_vertex_skin_weights: bool,
    pub b_make_skinned_meshes_root: bool,
    pub b_use_mesh_quantization: bool,
    pub b_export_morph_targets: bool,
    pub b_export_level_sequences: bool,
    pub b_export_animation_sequences: bool,
    pub texture_image_format: EGLTFTextureImageFormat,
    pub texture_image_quality: i32,
    pub b_export_texture_transforms: bool,
    pub b_export_lightmaps: bool,
    pub b_adjust_normalmaps: bool,
    pub b_export_hidden_in_game: bool,
    pub b_export_lights: bool,
    pub b_export_cameras: bool,
    pub export_material_variants: EGLTFMaterialVariantMode,
}
impl UGLTFExportOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFExportOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFExportOptions")
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
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_export_options_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_export_options_reset_to_default,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGLTFProxyOptions {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_bake_material_inputs: bool,
    pub b_use_thin_translucent_shading_model: bool,
    pub default_material_bake_size: FGLTFMaterialBakeSize,
    pub default_material_bake_filter: crate::bindings::engine::TextureFilter,
    pub default_material_bake_tiling: crate::bindings::engine::TextureAddress,
    pub default_input_bake_settings: TMap<
        EGLTFMaterialPropertyGroup,
        FGLTFOverrideMaterialBakeSettings,
    >,
}
impl UGLTFProxyOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFProxyOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFProxyOptions")
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
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_proxy_options_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gltf_exporter::__FUNCTION_PTRS
                    .ugltf_proxy_options_reset_to_default,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGLTFMaterialExportOptions {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub proxy: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub default: FGLTFOverrideMaterialBakeSettings,
    pub inputs: TMap<EGLTFMaterialPropertyGroup, FGLTFOverrideMaterialBakeSettings>,
}
impl UGLTFMaterialExportOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFMaterialExportOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGLTFMaterialExportOptions")
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
pub struct EGLTFMaterialBakeMode(pub u8);
impl EGLTFMaterialBakeMode {
    pub const DISABLED: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(0);
    pub const SIMPLE: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(1);
    pub const USE_MESH_DATA: EGLTFMaterialBakeMode = EGLTFMaterialBakeMode(2);
}
#[repr(transparent)]
pub struct EGLTFMaterialPropertyGroup(pub u8);
impl EGLTFMaterialPropertyGroup {
    pub const NONE: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(0);
    pub const BASE_COLOR_OPACITY: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        1,
    );
    pub const METALLIC_ROUGHNESS: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        2,
    );
    pub const EMISSIVE_COLOR: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(3);
    pub const NORMAL: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(4);
    pub const AMBIENT_OCCLUSION: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        5,
    );
    pub const CLEAR_COAT_ROUGHNESS: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        6,
    );
    pub const CLEAR_COAT_BOTTOM_NORMAL: EGLTFMaterialPropertyGroup = EGLTFMaterialPropertyGroup(
        7,
    );
}
#[repr(transparent)]
pub struct EGLTFTextureImageFormat(pub u8);
impl EGLTFTextureImageFormat {
    pub const NONE: EGLTFTextureImageFormat = EGLTFTextureImageFormat(0);
    pub const PNG: EGLTFTextureImageFormat = EGLTFTextureImageFormat(1);
    pub const JPEG: EGLTFTextureImageFormat = EGLTFTextureImageFormat(2);
}
#[repr(transparent)]
pub struct EGLTFMaterialVariantMode(pub u8);
impl EGLTFMaterialVariantMode {
    pub const NONE: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(0);
    pub const SIMPLE: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(1);
    pub const USE_MESH_DATA: EGLTFMaterialVariantMode = EGLTFMaterialVariantMode(2);
}
