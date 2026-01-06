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
pub static mut A_INSTANCED_FOLIAGE_ACTOR_REMOVE_ALL_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_INSTANCED_FOLIAGE_ACTOR_ADD_INSTANCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_SPHERE_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_BOX_TRANSFORMS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_BOX_COUNT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_INTERACTIVE_FOLIAGE_ACTOR_CAPSULE_TOUCHED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_PROCEDURAL_FOLIAGE_SPAWNER_SIMULATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AInstancedFoliageActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllInstances"),
            &raw mut A_INSTANCED_FOLIAGE_ACTOR_REMOVE_ALL_INSTANCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInstances"),
            &raw mut A_INSTANCED_FOLIAGE_ACTOR_ADD_INSTANCES,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFoliageStatistics::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FoliageOverlappingSphereCount"),
            &raw mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_SPHERE_COUNT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FoliageOverlappingBoxTransforms"),
            &raw mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_BOX_TRANSFORMS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FoliageOverlappingBoxCount"),
            &raw mut U_FOLIAGE_STATISTICS_FOLIAGE_OVERLAPPING_BOX_COUNT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = AInteractiveFoliageActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CapsuleTouched"),
            &raw mut A_INTERACTIVE_FOLIAGE_ACTOR_CAPSULE_TOUCHED,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UProceduralFoliageSpawner::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Simulate"),
            &raw mut U_PROCEDURAL_FOLIAGE_SPAWNER_SIMULATE,
        );
    }
}
#[repr(C, align(16))]
pub struct FProceduralFoliageInstance {
    #[doc(hidden)]
    __padding_32: [u8; 32],
    pub location: crate::bindings::core_u_object::FVector,
    pub age: f32,
    pub normal: crate::bindings::core_u_object::FVector,
    __padding_end: [u8; 40],
}
impl FProceduralFoliageInstance {}
#[repr(C, align(16))]
pub struct UFoliageInstancedStaticMeshComponent {
    __padding_end: [u8; 3360],
}
impl UFoliageInstancedStaticMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageInstancedStaticMeshComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFoliageType {
    #[doc(hidden)]
    __padding_172: [u8; 172],
    pub z_offset: crate::bindings::core_u_object::FFloatInterval,
    pub flags_180: u8,
    pub align_max_angle: f32,
    pub flags_188: u8,
    pub random_pitch_angle: f32,
    pub ground_slope_angle: crate::bindings::core_u_object::FFloatInterval,
    pub height: crate::bindings::core_u_object::FFloatInterval,
    pub landscape_layers: TArray<FName>,
    pub minimum_layer_weight: f32,
    pub exclusion_landscape_layers: TArray<FName>,
    pub minimum_exclusion_layer_weight: f32,
    #[doc(hidden)]
    __padding_272: [u8; 12],
    pub flags_272: u8,
    pub collision_scale: crate::bindings::core_u_object::FVector,
    pub average_normal_sample_count: i32,
    #[doc(hidden)]
    __padding_392: [u8; 84],
    pub mobility: crate::bindings::engine::EComponentMobility,
    #[doc(hidden)]
    __padding_404: [u8; 11],
    pub flags_404: u8,
    #[doc(hidden)]
    __padding_408: [u8; 3],
    pub flags_408: u8,
    #[doc(hidden)]
    __padding_412: [u8; 3],
    pub flags_412: u8,
    #[doc(hidden)]
    __padding_416: [u8; 3],
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub overridden_light_map_res: i32,
    pub lightmap_type: crate::bindings::engine::ELightmapType,
    #[doc(hidden)]
    __padding_428: [u8; 3],
    pub flags_428: u8,
    #[doc(hidden)]
    __padding_432: [u8; 3],
    pub flags_432: u8,
    pub world_position_offset_disable_distance: i32,
    #[doc(hidden)]
    __padding_873: [u8; 433],
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    #[doc(hidden)]
    __padding_876: [u8; 2],
    pub flags_876: u8,
    #[doc(hidden)]
    __padding_880: [u8; 3],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    #[doc(hidden)]
    __padding_1256: [u8; 360],
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub virtual_texture_cull_mips: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    #[doc(hidden)]
    __padding_1280: [u8; 3],
    pub flags_1280: u8,
    __padding_end: [u8; 71],
}
impl UFoliageType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFoliageType_Actor {
    __padding_end: [u8; 1376],
}
impl UFoliageType_Actor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_Actor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFoliageType_InstancedStaticMesh {
    #[doc(hidden)]
    __padding_1352: [u8; 1352],
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub override_materials: TArray<UPtr<crate::bindings::engine::UMaterialInterface>>,
    pub nanite_override_materials: TArray<
        UPtr<crate::bindings::engine::UMaterialInterface>,
    >,
    __padding_end: [u8; 8],
}
impl UFoliageType_InstancedStaticMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_InstancedStaticMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AInstancedFoliageActor {
    __padding_end: [u8; 1896],
}
impl AInstancedFoliageActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AInstancedFoliageActor")
            .unwrap()
    }
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
pub struct UInteractiveFoliageComponent {
    __padding_end: [u8; 1904],
}
impl UInteractiveFoliageComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteractiveFoliageComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UFoliageStatistics {
    __padding_end: [u8; 48],
}
impl UFoliageStatistics {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageStatistics")
            .unwrap()
    }
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
pub struct UGrassInstancedStaticMeshComponent {
    __padding_end: [u8; 3280],
}
impl UGrassInstancedStaticMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGrassInstancedStaticMeshComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AInteractiveFoliageActor {
    #[doc(hidden)]
    __padding_1256: [u8; 1256],
    pub foliage_damage_impulse_scale: f32,
    pub foliage_touch_impulse_scale: f32,
    pub foliage_stiffness: f32,
    pub foliage_stiffness_quadratic: f32,
    pub foliage_damping: f32,
    pub max_damage_impulse: f32,
    pub max_touch_impulse: f32,
    pub max_force: f32,
    __padding_end: [u8; 8],
}
impl AInteractiveFoliageActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AInteractiveFoliageActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AProceduralFoliageBlockingVolume {
    __padding_end: [u8; 1360],
}
impl AProceduralFoliageBlockingVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AProceduralFoliageBlockingVolume")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UProceduralFoliageComponent {
    #[doc(hidden)]
    __padding_240: [u8; 240],
    pub foliage_spawner: UPtr<UProceduralFoliageSpawner>,
    pub tile_overlap: f32,
    pub b_allow_landscape: bool,
    pub b_allow_bsp: bool,
    pub b_allow_static_mesh: bool,
    pub b_allow_translucent: bool,
    pub b_allow_foliage: bool,
    pub b_show_debug_tiles: bool,
    __padding_end: [u8; 30],
}
impl UProceduralFoliageComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UProceduralFoliageSpawner {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub random_seed: i32,
    pub tile_size: f32,
    pub num_unique_tiles: i32,
    pub minimum_quad_tree_size: f32,
    __padding_end: [u8; 72],
}
impl UProceduralFoliageSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageSpawner")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UProceduralFoliageTile {
    __padding_end: [u8; 376],
}
impl UProceduralFoliageTile {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageTile")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct AProceduralFoliageVolume {
    #[doc(hidden)]
    __padding_1216: [u8; 1216],
    pub procedural_component: UPtr<UProceduralFoliageComponent>,
    __padding_end: [u8; 8],
}
impl AProceduralFoliageVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AProceduralFoliageVolume")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FFoliageInstancedStaticMeshComponent_OnInstanceTakePointDamage {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFoliageInstancedStaticMeshComponent_OnInstanceTakeRadialDamage {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EFoliageScaling(pub u8);
impl EFoliageScaling {
    pub const UNIFORM: EFoliageScaling = EFoliageScaling(0);
    pub const FREE: EFoliageScaling = EFoliageScaling(1);
    pub const LOCK_XY: EFoliageScaling = EFoliageScaling(2);
    pub const LOCK_XZ: EFoliageScaling = EFoliageScaling(3);
    pub const LOCK_YZ: EFoliageScaling = EFoliageScaling(4);
}
#[repr(transparent)]
pub struct FoliageVertexColorMask(pub u8);
impl FoliageVertexColorMask {
    pub const FOLIAGEVERTEXCOLORMASK_DISABLED: FoliageVertexColorMask = FoliageVertexColorMask(
        0,
    );
    pub const FOLIAGEVERTEXCOLORMASK_RED: FoliageVertexColorMask = FoliageVertexColorMask(
        1,
    );
    pub const FOLIAGEVERTEXCOLORMASK_GREEN: FoliageVertexColorMask = FoliageVertexColorMask(
        2,
    );
    pub const FOLIAGEVERTEXCOLORMASK_BLUE: FoliageVertexColorMask = FoliageVertexColorMask(
        3,
    );
    pub const FOLIAGEVERTEXCOLORMASK_ALPHA: FoliageVertexColorMask = FoliageVertexColorMask(
        4,
    );
}
