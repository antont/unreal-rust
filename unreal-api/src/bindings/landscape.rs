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
pub static mut A_LANDSCAPE_PROXY_SET_VIRTUAL_TEXTURE_RENDER_PASS_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_VECTOR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_TEXTURE_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_SCALAR_PARAMETER_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_WEIGHTMAP_FROM_RENDER_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_HEIGHTMAP_FROM_RENDER_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_WEIGHTMAP_TO_RENDER_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_HEIGHTMAP_TO_RENDER_TARGET: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_GET_LANDSCAPE_ACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_EDITOR_SET_LANDSCAPE_MATERIAL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_EDITOR_APPLY_SPLINE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_DELETE_UNUSED_LAYERS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_CHANGE_LOD_DISTANCE_FACTOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_PROXY_CHANGE_COMPONENT_SCREEN_SIZE_TO_USE_SUB_SECTIONS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_HEIGHTFIELD_COLLISION_COMPONENT_GET_RENDER_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_SPLINES_COMPONENT_GET_SPLINE_MESH_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_RENDER_WEIGHTMAPS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_RENDER_WEIGHTMAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_RENDER_HEIGHTMAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_GET_TARGET_LAYER_NAMES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_FORCE_LAYERS_FULL_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_REQUEST_LANDSCAPE_UPDATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_RENDER_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_RENDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_INITIALIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_GET_BLUEPRINT_RENDER_DEPENDENCIES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_SET_LOD_BIAS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_SET_FORCED_LOD: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_GET_MATERIAL_INSTANCE_DYNAMIC: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_GET_GRASS_TYPES_BP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_BY_NAME_AT_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_AT_LOCATION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ALandscapeProxy::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVirtualTextureRenderPassType"),
            &raw mut A_LANDSCAPE_PROXY_SET_VIRTUAL_TEXTURE_RENDER_PASS_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLandscapeMaterialVectorParameterValue"),
            &raw mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_VECTOR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLandscapeMaterialTextureParameterValue"),
            &raw mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_TEXTURE_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLandscapeMaterialScalarParameterValue"),
            &raw mut A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_SCALAR_PARAMETER_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LandscapeImportWeightmapFromRenderTarget"),
            &raw mut A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_WEIGHTMAP_FROM_RENDER_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LandscapeImportHeightmapFromRenderTarget"),
            &raw mut A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_HEIGHTMAP_FROM_RENDER_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LandscapeExportWeightmapToRenderTarget"),
            &raw mut A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_WEIGHTMAP_TO_RENDER_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LandscapeExportHeightmapToRenderTarget"),
            &raw mut A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_HEIGHTMAP_TO_RENDER_TARGET,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLandscapeActor"),
            &raw mut A_LANDSCAPE_PROXY_GET_LANDSCAPE_ACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorSetLandscapeMaterial"),
            &raw mut A_LANDSCAPE_PROXY_EDITOR_SET_LANDSCAPE_MATERIAL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorApplySpline"),
            &raw mut A_LANDSCAPE_PROXY_EDITOR_APPLY_SPLINE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteUnusedLayers"),
            &raw mut A_LANDSCAPE_PROXY_DELETE_UNUSED_LAYERS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangeLODDistanceFactor"),
            &raw mut A_LANDSCAPE_PROXY_CHANGE_LOD_DISTANCE_FACTOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ChangeComponentScreenSizeToUseSubSections"),
            &raw mut A_LANDSCAPE_PROXY_CHANGE_COMPONENT_SCREEN_SIZE_TO_USE_SUB_SECTIONS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULandscapeHeightfieldCollisionComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRenderComponent"),
            &raw mut U_LANDSCAPE_HEIGHTFIELD_COLLISION_COMPONENT_GET_RENDER_COMPONENT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULandscapeSplinesComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSplineMeshComponents"),
            &raw mut U_LANDSCAPE_SPLINES_COMPONENT_GET_SPLINE_MESH_COMPONENTS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ALandscape::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenderWeightmaps"),
            &raw mut A_LANDSCAPE_RENDER_WEIGHTMAPS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenderWeightmap"),
            &raw mut A_LANDSCAPE_RENDER_WEIGHTMAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenderHeightmap"),
            &raw mut A_LANDSCAPE_RENDER_HEIGHTMAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTargetLayerNames"),
            &raw mut A_LANDSCAPE_GET_TARGET_LAYER_NAMES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ForceLayersFullUpdate"),
            &raw mut A_LANDSCAPE_FORCE_LAYERS_FULL_UPDATE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ALandscapeBlueprintBrushBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RequestLandscapeUpdate"),
            &raw mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_REQUEST_LANDSCAPE_UPDATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenderLayer"),
            &raw mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_RENDER_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Render"),
            &raw mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_RENDER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Initialize"),
            &raw mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_INITIALIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBlueprintRenderDependencies"),
            &raw mut A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_GET_BLUEPRINT_RENDER_DEPENDENCIES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULandscapeComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLODBias"),
            &raw mut U_LANDSCAPE_COMPONENT_SET_LOD_BIAS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForcedLOD"),
            &raw mut U_LANDSCAPE_COMPONENT_SET_FORCED_LOD,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInstanceDynamic"),
            &raw mut U_LANDSCAPE_COMPONENT_GET_MATERIAL_INSTANCE_DYNAMIC,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGrassTypesBP"),
            &raw mut U_LANDSCAPE_COMPONENT_GET_GRASS_TYPES_BP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorGetPaintLayerWeightByNameAtLocation"),
            &raw mut U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_BY_NAME_AT_LOCATION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorGetPaintLayerWeightAtLocation"),
            &raw mut U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_AT_LOCATION,
        );
    }
}
#[repr(C, align(8))]
pub struct FGrassVariety {
    pub grass_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub grass_density: crate::bindings::core_u_object::FPerPlatformFloat,
    pub grass_density_quality: crate::bindings::engine::FPerQualityLevelFloat,
    pub b_use_grid: bool,
    pub placement_jitter: f32,
    pub start_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub start_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub end_cull_distance: crate::bindings::core_u_object::FPerPlatformInt,
    pub end_cull_distance_quality: crate::bindings::engine::FPerQualityLevelInt,
    pub min_lod: i32,
    pub allowed_density_range: crate::bindings::core_u_object::FFloatInterval,
    pub scaling: EGrassScaling,
    pub scale_x: crate::bindings::core_u_object::FFloatInterval,
    pub scale_y: crate::bindings::core_u_object::FFloatInterval,
    pub scale_z: crate::bindings::core_u_object::FFloatInterval,
    pub b_weight_attenuates_max_scale: bool,
    pub max_scale_weight_attenuation: f32,
    pub random_rotation: bool,
    pub align_to_surface: bool,
    pub b_align_to_triangle_normals: bool,
    pub b_use_landscape_lightmap: bool,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    pub b_receives_decals: bool,
    pub b_affect_distance_field_lighting: bool,
    pub b_cast_dynamic_shadow: bool,
    pub b_cast_contact_shadow: bool,
    pub b_keep_instance_buffer_cpu_copy: bool,
    #[doc(hidden)]
    __padding_744: [u8; 6],
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
}
impl FGrassVariety {}
#[repr(C, align(16))]
pub struct FLandscapeBrushParameters {
    pub render_area_world_transform: crate::bindings::core_u_object::FTransform,
    pub render_area_size: crate::bindings::core_u_object::FIntPoint,
    pub combined_result: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub layer_type: ELandscapeToolTargetType,
    pub weightmap_layer_name: FName,
}
impl FLandscapeBrushParameters {}
#[repr(C, align(16))]
pub struct UControlPointMeshComponent {
    __padding_end: [u8; 1904],
}
impl UControlPointMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlPointMeshComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ILandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {}
#[repr(C, align(8))]
pub struct ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {
    __padding_end: [u8; 48],
}
impl ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeBrushRenderCallAdapter_GlobalMergeLegacySupport_DEPRECATED")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ILandscapeSplineInterface {}
#[repr(C, align(8))]
pub struct ULandscapeSplineInterface {
    __padding_end: [u8; 48],
}
impl ULandscapeSplineInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSplineInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct ILandscapeEditLayerRenderer {}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerRenderer {
    __padding_end: [u8; 48],
}
impl ULandscapeEditLayerRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayerRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeGizmoActor {
    __padding_end: [u8; 1176],
}
impl ALandscapeGizmoActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeGizmoActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeGizmoActiveActor {
    __padding_end: [u8; 1608],
}
impl ALandscapeGizmoActiveActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeGizmoActiveActor")
            .unwrap()
    }
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
pub struct ULandscapeGizmoRenderComponent {
    __padding_end: [u8; 1504],
}
impl ULandscapeGizmoRenderComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeGizmoRenderComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeGrassType {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub grass_varieties: TArray<FGrassVariety>,
    pub flags_64: u8,
    __padding_end: [u8; 39],
}
impl ULandscapeGrassType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeGrassType")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeProxy {
    #[doc(hidden)]
    __padding_1272: [u8; 1272],
    pub b_enable_nanite: bool,
    #[doc(hidden)]
    __padding_1872: [u8; 592],
    pub landscape_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    #[doc(hidden)]
    __padding_1928: [u8; 48],
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    #[doc(hidden)]
    __padding_1945: [u8; 1],
    pub b_virtual_texture_render_with_quad: bool,
    pub b_virtual_texture_render_with_quad_hq: bool,
    pub virtual_texture_num_lods: i32,
    pub virtual_texture_lod_bias: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    #[doc(hidden)]
    __padding_2164: [u8; 207],
    pub flags_2164: u8,
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub flags_2166: u8,
    #[doc(hidden)]
    __padding_2168: [u8; 1],
    pub flags_2168: u8,
    #[doc(hidden)]
    __padding_2172: [u8; 3],
    pub flags_2172: u8,
    #[doc(hidden)]
    __padding_2176: [u8; 3],
    pub flags_2176: u8,
    #[doc(hidden)]
    __padding_2180: [u8; 3],
    pub flags_2180: u8,
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    #[doc(hidden)]
    __padding_2200: [u8; 18],
    pub flags_2200: u8,
    #[doc(hidden)]
    __padding_2204: [u8; 3],
    pub flags_2204: u8,
    #[doc(hidden)]
    __padding_2208: [u8; 3],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub ld_max_draw_distance: f32,
    #[doc(hidden)]
    __padding_2256: [u8; 32],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    pub flags_2688: u8,
    __padding_end: [u8; 655],
}
impl ALandscapeProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_virtual_texture_render_pass_type(
        &mut self,
        in_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_VIRTUAL_TEXTURE_RENDER_PASS_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_VIRTUAL_TEXTURE_RENDER_PASS_TYPE,
                __buffer,
            )
        };
    }
    pub fn set_landscape_material_vector_parameter_value(
        &mut self,
        parameter_name: FName,
        value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_VECTOR_PARAMETER_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_VECTOR_PARAMETER_VALUE,
                __buffer,
            )
        };
    }
    pub fn set_landscape_material_texture_parameter_value(
        &mut self,
        parameter_name: FName,
        value: UPtr<crate::bindings::engine::UTexture>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_TEXTURE_PARAMETER_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_TEXTURE_PARAMETER_VALUE,
                __buffer,
            )
        };
    }
    pub fn set_landscape_material_scalar_parameter_value(
        &mut self,
        parameter_name: FName,
        value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_SCALAR_PARAMETER_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_SET_LANDSCAPE_MATERIAL_SCALAR_PARAMETER_VALUE,
                __buffer,
            )
        };
    }
    pub fn landscape_import_weightmap_from_render_target(
        &mut self,
        in_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        in_layer_name: FName,
        in_edit_layer_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_WEIGHTMAP_FROM_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_layer_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_edit_layer_index,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_WEIGHTMAP_FROM_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn landscape_import_heightmap_from_render_target(
        &mut self,
        in_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        in_import_height_from_rg_channel: bool,
        in_edit_layer_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_HEIGHTMAP_FROM_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_import_height_from_rg_channel,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_edit_layer_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_IMPORT_HEIGHTMAP_FROM_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn landscape_export_heightmap_to_render_target(
        &mut self,
        in_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        in_export_height_into_rg_channel: bool,
        in_export_landscape_proxies: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_HEIGHTMAP_TO_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_render_target,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_export_height_into_rg_channel,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_export_landscape_proxies,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_LANDSCAPE_EXPORT_HEIGHTMAP_TO_RENDER_TARGET,
                __buffer,
            )
        };
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn get_landscape_actor(&mut self) -> UPtr<ALandscape> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_GET_LANDSCAPE_ACTOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_GET_LANDSCAPE_ACTOR,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ALandscape>>().read() }
    }
    pub fn editor_set_landscape_material(
        &mut self,
        new_landscape_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_EDITOR_SET_LANDSCAPE_MATERIAL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_landscape_material,
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
                crate::bindings::landscape::A_LANDSCAPE_PROXY_EDITOR_SET_LANDSCAPE_MATERIAL,
                __buffer,
            )
        };
    }
    pub fn editor_apply_spline(
        &mut self,
        in_spline_component: UPtr<crate::bindings::engine::USplineComponent>,
        start_width: f32,
        end_width: f32,
        start_side_falloff: f32,
        end_side_falloff: f32,
        start_roll: f32,
        end_roll: f32,
        num_subdivisions: i32,
        b_raise_heights: bool,
        b_lower_heights: bool,
        paint_layer: UPtr<ULandscapeLayerInfoObject>,
        edit_layer_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_EDITOR_APPLY_SPLINE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_spline_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USplineComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_width,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_width, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_side_falloff,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_side_falloff,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_roll,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_roll, __buffer.add(28).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_subdivisions,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_raise_heights,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_lower_heights,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paint_layer,
                __buffer.add(40).cast::<UPtr<ULandscapeLayerInfoObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &edit_layer_name,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_EDITOR_APPLY_SPLINE,
                __buffer,
            )
        };
    }
    pub fn delete_unused_layers(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_DELETE_UNUSED_LAYERS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_DELETE_UNUSED_LAYERS,
                __buffer,
            )
        };
    }
    pub fn change_lod_distance_factor(&mut self, in_lod_distance_factor: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_CHANGE_LOD_DISTANCE_FACTOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lod_distance_factor,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_CHANGE_LOD_DISTANCE_FACTOR,
                __buffer,
            )
        };
    }
    pub fn change_component_screen_size_to_use_sub_sections(
        &mut self,
        in_component_screen_size_to_use_sub_sections: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_PROXY_CHANGE_COMPONENT_SCREEN_SIZE_TO_USE_SUB_SECTIONS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_screen_size_to_use_sub_sections,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_PROXY_CHANGE_COMPONENT_SCREEN_SIZE_TO_USE_SUB_SECTIONS,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct ULandscapeHeightfieldCollisionComponent {
    __padding_end: [u8; 2096],
}
impl ULandscapeHeightfieldCollisionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeHeightfieldCollisionComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_render_component(&self) -> UPtr<ULandscapeComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_HEIGHTFIELD_COLLISION_COMPONENT_GET_RENDER_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_HEIGHTFIELD_COLLISION_COMPONENT_GET_RENDER_COMPONENT,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ULandscapeComponent>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeMaterialInstanceConstant {
    __padding_end: [u8; 1872],
}
impl ULandscapeMaterialInstanceConstant {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeMaterialInstanceConstant")
            .unwrap()
    }
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
pub struct ULandscapeMeshCollisionComponent_DEPRECATED {
    __padding_end: [u8; 2208],
}
impl ULandscapeMeshCollisionComponent_DEPRECATED {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeMeshCollisionComponent_DEPRECATED")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeMeshProxyActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub landscape_mesh_proxy_component: UPtr<ULandscapeMeshProxyComponent>,
}
impl ALandscapeMeshProxyActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeMeshProxyActor")
            .unwrap()
    }
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
pub struct ULandscapeMeshProxyComponent {
    __padding_end: [u8; 2000],
}
impl ULandscapeMeshProxyComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeMeshProxyComponent")
            .unwrap()
    }
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
pub struct ULandscapeSplinesComponent {
    __padding_end: [u8; 1824],
}
impl ULandscapeSplinesComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSplinesComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_spline_mesh_components(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::USplineMeshComponent>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_SPLINES_COMPONENT_GET_SPLINE_MESH_COMPONENTS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_SPLINES_COMPONENT_GET_SPLINE_MESH_COMPONENTS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::USplineMeshComponent>>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeSplineControlPoint {
    #[doc(hidden)]
    __padding_280: [u8; 280],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    __padding_end: [u8; 168],
}
impl ULandscapeSplineControlPoint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSplineControlPoint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeSplineSegment {
    #[doc(hidden)]
    __padding_200: [u8; 200],
    pub body_instance: crate::bindings::engine::FBodyInstance,
    __padding_end: [u8; 152],
}
impl ULandscapeSplineSegment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSplineSegment")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeStreamingProxy {
    __padding_end: [u8; 3496],
}
impl ALandscapeStreamingProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeStreamingProxy")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeWeightmapUsage {
    __padding_end: [u8; 96],
}
impl ULandscapeWeightmapUsage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeWeightmapUsage")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeGrassOutput {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeGrassOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeGrassOutput")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AControlPointMeshActor {
    #[doc(hidden)]
    __padding_1136: [u8; 1136],
    pub control_point_mesh_component: UPtr<UControlPointMeshComponent>,
}
impl AControlPointMeshActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AControlPointMeshActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscape {
    __padding_end: [u8; 3768],
}
impl ALandscape {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscape")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn render_weightmaps(
        &mut self,
        in_world_transform: crate::bindings::core_u_object::FTransform,
        in_extents: crate::bindings::core_u_object::FBox2D,
        in_weightmap_layer_names: &TArray<FName>,
        out_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<161>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_RENDER_WEIGHTMAPS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extents,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_weightmap_layer_names,
                __buffer.add(136).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_render_target,
                __buffer
                    .add(152)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_RENDER_WEIGHTMAPS,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<bool>().read() }
    }
    pub fn render_weightmap(
        &mut self,
        in_world_transform: crate::bindings::core_u_object::FTransform,
        in_extents: crate::bindings::core_u_object::FBox2D,
        in_weightmap_layer_name: FName,
        out_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<161>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_RENDER_WEIGHTMAP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extents,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_weightmap_layer_name,
                __buffer.add(136).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_render_target,
                __buffer
                    .add(152)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_RENDER_WEIGHTMAP,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<bool>().read() }
    }
    pub fn render_heightmap(
        &mut self,
        in_world_transform: crate::bindings::core_u_object::FTransform,
        in_extents: crate::bindings::core_u_object::FBox2D,
        out_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_RENDER_HEIGHTMAP,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extents,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FBox2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_render_target,
                __buffer
                    .add(136)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_RENDER_HEIGHTMAP,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn get_target_layer_names(
        &self,
        b_in_include_visibility_layer: bool,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_GET_TARGET_LAYER_NAMES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_include_visibility_layer,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_GET_TARGET_LAYER_NAMES,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FName>>().read() }
    }
    pub fn force_layers_full_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_FORCE_LAYERS_FULL_UPDATE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_FORCE_LAYERS_FULL_UPDATE,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct ALandscapeBlueprintBrushBase {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub update_on_property_change: bool,
    pub affect_heightmap: bool,
    pub affect_weightmap: bool,
    pub affect_visibility_layer: bool,
    pub affected_weightmap_layers: TArray<FName>,
    pub b_use_power_of_two_render_target: bool,
    __padding_end: [u8; 135],
}
impl ALandscapeBlueprintBrushBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeBlueprintBrushBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn request_landscape_update(&mut self, b_in_user_triggered: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_REQUEST_LANDSCAPE_UPDATE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_user_triggered,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::A_LANDSCAPE_BLUEPRINT_BRUSH_BASE_REQUEST_LANDSCAPE_UPDATE,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct ULandscapeLODStreamingProxy_DEPRECATED {
    __padding_end: [u8; 256],
}
impl ULandscapeLODStreamingProxy_DEPRECATED {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeLODStreamingProxy_DEPRECATED")
            .unwrap()
    }
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
pub struct ULandscapeComponent {
    #[doc(hidden)]
    __padding_1504: [u8; 1504],
    pub section_base_x: i32,
    pub section_base_y: i32,
    #[doc(hidden)]
    __padding_1528: [u8; 16],
    pub override_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub override_hole_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    #[doc(hidden)]
    __padding_2172: [u8; 628],
    pub forced_lod: i32,
    pub lod_bias: i32,
    __padding_end: [u8; 364],
}
impl ULandscapeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_lod_bias(&mut self, in_lod_bias: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_SET_LOD_BIAS,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_lod_bias,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_SET_LOD_BIAS,
                __buffer,
            )
        };
    }
    pub fn set_forced_lod(&mut self, in_forced_lod: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_SET_FORCED_LOD,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_forced_lod,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_SET_FORCED_LOD,
                __buffer,
            )
        };
    }
    pub fn get_material_instance_dynamic(
        &self,
        in_index: i32,
    ) -> UPtr<crate::bindings::engine::UMaterialInstanceDynamic> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_GET_MATERIAL_INSTANCE_DYNAMIC,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_GET_MATERIAL_INSTANCE_DYNAMIC,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UMaterialInstanceDynamic>>()
                .read()
        }
    }
    pub fn get_grass_types_bp(&self) -> TArray<UPtr<ULandscapeGrassType>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_GET_GRASS_TYPES_BP,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_GET_GRASS_TYPES_BP,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<ULandscapeGrassType>>>().read() }
    }
    pub fn editor_get_paint_layer_weight_by_name_at_location(
        &mut self,
        in_location: &crate::bindings::core_u_object::FVector,
        in_paint_layer_name: FName,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_BY_NAME_AT_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_paint_layer_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_BY_NAME_AT_LOCATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<f32>().read() }
    }
    pub fn editor_get_paint_layer_weight_at_location(
        &mut self,
        in_location: &crate::bindings::core_u_object::FVector,
        paint_layer: UPtr<ULandscapeLayerInfoObject>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_AT_LOCATION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &paint_layer,
                __buffer.add(24).cast::<UPtr<ULandscapeLayerInfoObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::landscape::U_LANDSCAPE_COMPONENT_EDITOR_GET_PAINT_LAYER_WEIGHT_AT_LOCATION,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeHeightmapTextureEdgeFixup {
    __padding_end: [u8; 152],
}
impl ULandscapeHeightmapTextureEdgeFixup {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeHeightmapTextureEdgeFixup")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerBase {
    __padding_end: [u8; 216],
}
impl ULandscapeEditLayerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayerBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerPersistent {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayerPersistent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayerPersistent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditLayer {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerProcedural {
    __padding_end: [u8; 216],
}
impl ULandscapeEditLayerProcedural {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayerProcedural")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditLayerSplines {
    __padding_end: [u8; 224],
}
impl ULandscapeEditLayerSplines {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditLayerSplines")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeDefaultEditLayerRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeDefaultEditLayerRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeDefaultEditLayerRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeHeightmapNormalsEditLayerRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeHeightmapNormalsEditLayerRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeHeightmapNormalsEditLayerRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeWeightmapWeightBlendedLayersRenderer {
    __padding_end: [u8; 56],
}
impl ULandscapeWeightmapWeightBlendedLayersRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeWeightmapWeightBlendedLayersRenderer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeScratchRenderTarget {
    __padding_end: [u8; 128],
}
impl ULandscapeScratchRenderTarget {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeScratchRenderTarget")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeEditResourcesSubsystem {
    __padding_end: [u8; 136],
}
impl ULandscapeEditResourcesSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeEditResourcesSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeHLODBuilder {
    __padding_end: [u8; 72],
}
impl ULandscapeHLODBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeHLODBuilder")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeInfo {
    __padding_end: [u8; 848],
}
impl ULandscapeInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeInfo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeInfoMap {
    __padding_end: [u8; 136],
}
impl ULandscapeInfoMap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeInfoMap")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeLayerInfoObject {
    __padding_end: [u8; 184],
}
impl ULandscapeLayerInfoObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeLayerInfoObject")
            .unwrap()
    }
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
pub struct ULandscapeNaniteComponent {
    __padding_end: [u8; 1936],
}
impl ULandscapeNaniteComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeNaniteComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeSettings {
    __padding_end: [u8; 264],
}
impl ULandscapeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeSplineActor {
    __padding_end: [u8; 1168],
}
impl ALandscapeSplineActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeSplineActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ALandscapeSplineMeshesActor {
    #[doc(hidden)]
    __padding_1144: [u8; 1144],
    pub static_mesh_components: TArray<
        UPtr<crate::bindings::engine::UStaticMeshComponent>,
    >,
    __padding_end: [u8; 16],
}
impl ALandscapeSplineMeshesActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALandscapeSplineMeshesActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeSubsystem {
    __padding_end: [u8; 496],
}
impl ULandscapeSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeSubsystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeTextureHash {
    __padding_end: [u8; 168],
}
impl ULandscapeTextureHash {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeTextureHash")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeTextureMipEdgeOverrideFactory {
    __padding_end: [u8; 64],
}
impl ULandscapeTextureMipEdgeOverrideFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeTextureMipEdgeOverrideFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULandscapeTextureStorageProviderFactory {
    __padding_end: [u8; 112],
}
impl ULandscapeTextureStorageProviderFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeTextureStorageProviderFactory")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerBlend {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeLayerBlend {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeLayerBlend")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerCoords {
    __padding_end: [u8; 224],
}
impl UMaterialExpressionLandscapeLayerCoords {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeLayerCoords")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerSample {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapeLayerSample {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeLayerSample")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerSwitch {
    __padding_end: [u8; 312],
}
impl UMaterialExpressionLandscapeLayerSwitch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeLayerSwitch")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeLayerWeight {
    __padding_end: [u8; 336],
}
impl UMaterialExpressionLandscapeLayerWeight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeLayerWeight")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapePhysicalMaterialOutput {
    __padding_end: [u8; 216],
}
impl UMaterialExpressionLandscapePhysicalMaterialOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapePhysicalMaterialOutput")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialExpressionLandscapeVisibilityMask {
    __padding_end: [u8; 200],
}
impl UMaterialExpressionLandscapeVisibilityMask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialExpressionLandscapeVisibilityMask")
            .unwrap()
    }
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
pub struct EGrassScaling(pub u8);
impl EGrassScaling {
    pub const UNIFORM: EGrassScaling = EGrassScaling(0);
    pub const FREE: EGrassScaling = EGrassScaling(1);
    pub const LOCK_XY: EGrassScaling = EGrassScaling(2);
}
#[repr(transparent)]
pub struct LandscapeSplineMeshOrientation(pub u8);
impl LandscapeSplineMeshOrientation {
    pub const LSMO_X_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        0,
    );
    pub const LSMO_Y_UP: LandscapeSplineMeshOrientation = LandscapeSplineMeshOrientation(
        1,
    );
}
#[repr(transparent)]
pub struct ELandscapeBlendMode(pub u8);
impl ELandscapeBlendMode {
    pub const LSBM_ADDITIVE_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(0);
    pub const LSBM_ALPHA_BLEND: ELandscapeBlendMode = ELandscapeBlendMode(1);
}
#[repr(transparent)]
pub struct ELandscapeToolTargetType(pub u8);
impl ELandscapeToolTargetType {
    pub const HEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(0);
    pub const WEIGHTMAP: ELandscapeToolTargetType = ELandscapeToolTargetType(1);
    pub const VISIBILITY: ELandscapeToolTargetType = ELandscapeToolTargetType(2);
    pub const INVALID: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
    pub const COUNT: ELandscapeToolTargetType = ELandscapeToolTargetType(3);
}
#[repr(transparent)]
pub struct ELandscapeLayerBlendType(pub u8);
impl ELandscapeLayerBlendType {
    pub const LB_WEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(0);
    pub const LB_ALPHA_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(1);
    pub const LB_HEIGHT_BLEND: ELandscapeLayerBlendType = ELandscapeLayerBlendType(2);
}
#[repr(transparent)]
pub struct ELandscapeGizmoType(pub u8);
impl ELandscapeGizmoType {
    pub const LGT_NONE: ELandscapeGizmoType = ELandscapeGizmoType(0);
    pub const LGT_HEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(1);
    pub const LGT_WEIGHT: ELandscapeGizmoType = ELandscapeGizmoType(2);
}
#[repr(transparent)]
pub struct ELandscapeGizmoSnapType(pub i32);
impl ELandscapeGizmoSnapType {
    pub const NONE: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(0);
    pub const COMPONENT: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(1);
    pub const TEXEL: ELandscapeGizmoSnapType = ELandscapeGizmoSnapType(2);
}
#[repr(transparent)]
pub struct ELandscapeLODFalloff(pub u8);
impl ELandscapeLODFalloff {
    pub const LINEAR: ELandscapeLODFalloff = ELandscapeLODFalloff(0);
    pub const SQUARE_ROOT: ELandscapeLODFalloff = ELandscapeLODFalloff(1);
}
#[repr(transparent)]
pub struct ELandscapeHLODTextureSizePolicy(pub u8);
impl ELandscapeHLODTextureSizePolicy {
    pub const AUTOMATIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        0,
    );
    pub const SPECIFIC_SIZE: ELandscapeHLODTextureSizePolicy = ELandscapeHLODTextureSizePolicy(
        1,
    );
}
#[repr(transparent)]
pub struct ELandscapeHLODMeshSourceLODPolicy(pub u8);
impl ELandscapeHLODMeshSourceLODPolicy {
    pub const AUTOMATIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        0,
    );
    pub const SPECIFIC_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        1,
    );
    pub const LOWEST_DETAIL_LOD: ELandscapeHLODMeshSourceLODPolicy = ELandscapeHLODMeshSourceLODPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct ELandscapeLayerDisplayMode(pub u8);
impl ELandscapeLayerDisplayMode {
    pub const DEFAULT: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(0);
    pub const ALPHABETICAL: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(1);
    pub const USER_SPECIFIC: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(2);
    pub const BY_BLEND_METHOD: ELandscapeLayerDisplayMode = ELandscapeLayerDisplayMode(
        3,
    );
}
#[repr(transparent)]
pub struct ESplineModulationColorMask(pub u8);
impl ESplineModulationColorMask {
    pub const RED: ESplineModulationColorMask = ESplineModulationColorMask(0);
    pub const GREEN: ESplineModulationColorMask = ESplineModulationColorMask(1);
    pub const BLUE: ESplineModulationColorMask = ESplineModulationColorMask(2);
    pub const ALPHA: ESplineModulationColorMask = ESplineModulationColorMask(3);
}
#[repr(transparent)]
pub struct ELandscapeTargetLayerBlendMethod(pub u8);
impl ELandscapeTargetLayerBlendMethod {
    pub const NONE: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        0,
    );
    pub const FINAL_WEIGHT_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        1,
    );
    pub const PREMULTIPLIED_ALPHA_BLENDING: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        2,
    );
    pub const COUNT: ELandscapeTargetLayerBlendMethod = ELandscapeTargetLayerBlendMethod(
        3,
    );
}
#[repr(transparent)]
pub struct ELandscapeDirtyingMode(pub u8);
impl ELandscapeDirtyingMode {
    pub const AUTO: ELandscapeDirtyingMode = ELandscapeDirtyingMode(0);
    pub const IN_LANDSCAPE_MODE_ONLY: ELandscapeDirtyingMode = ELandscapeDirtyingMode(1);
    pub const IN_LANDSCAPE_MODE_AND_USER_TRIGGERED_CHANGES: ELandscapeDirtyingMode = ELandscapeDirtyingMode(
        2,
    );
}
#[repr(transparent)]
pub struct ELandscapeTextureType(pub u8);
impl ELandscapeTextureType {
    pub const UNKNOWN: ELandscapeTextureType = ELandscapeTextureType(0);
    pub const HEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(1);
    pub const WEIGHTMAP: ELandscapeTextureType = ELandscapeTextureType(2);
}
#[repr(transparent)]
pub struct ELandscapeTextureUsage(pub u8);
impl ELandscapeTextureUsage {
    pub const UNKNOWN: ELandscapeTextureUsage = ELandscapeTextureUsage(0);
    pub const EDIT_LAYER_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(1);
    pub const FINAL_DATA: ELandscapeTextureUsage = ELandscapeTextureUsage(2);
}
#[repr(transparent)]
pub struct ETerrainCoordMappingType(pub u8);
impl ETerrainCoordMappingType {
    pub const TCMT_AUTO: ETerrainCoordMappingType = ETerrainCoordMappingType(0);
    pub const TCMT_XY: ETerrainCoordMappingType = ETerrainCoordMappingType(1);
    pub const TCMT_XZ: ETerrainCoordMappingType = ETerrainCoordMappingType(2);
    pub const TCMT_YZ: ETerrainCoordMappingType = ETerrainCoordMappingType(3);
}
#[repr(transparent)]
pub struct ELandscapeCustomizedCoordType(pub u8);
impl ELandscapeCustomizedCoordType {
    pub const LCCT_NONE: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        0,
    );
    pub const LCCT_CUSTOM_UV0: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        1,
    );
    pub const LCCT_CUSTOM_UV1: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        2,
    );
    pub const LCCT_CUSTOM_UV2: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        3,
    );
    pub const LCCT_WEIGHT_MAP_UV: ELandscapeCustomizedCoordType = ELandscapeCustomizedCoordType(
        4,
    );
}
#[repr(transparent)]
pub struct ELandscapeImportAlphamapType(pub u8);
impl ELandscapeImportAlphamapType {
    pub const ADDITIVE: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(0);
    pub const LAYERED: ELandscapeImportAlphamapType = ELandscapeImportAlphamapType(1);
}
#[repr(transparent)]
pub struct ELandscapeLayerPaintingRestriction(pub u8);
impl ELandscapeLayerPaintingRestriction {
    pub const NONE: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        0,
    );
    pub const USE_MAX_LAYERS: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        1,
    );
    pub const EXISTING_ONLY: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        2,
    );
    pub const USE_COMPONENT_ALLOW_LIST: ELandscapeLayerPaintingRestriction = ELandscapeLayerPaintingRestriction(
        3,
    );
}
