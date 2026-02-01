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
    pub a_instanced_foliage_actor_remove_all_instances: *mut crate::ffi::UFunctionOpague,
    pub a_instanced_foliage_actor_add_instances: *mut crate::ffi::UFunctionOpague,
    pub u_foliage_statistics_foliage_overlapping_sphere_count: *mut crate::ffi::UFunctionOpague,
    pub u_foliage_statistics_foliage_overlapping_box_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_foliage_statistics_foliage_overlapping_box_count: *mut crate::ffi::UFunctionOpague,
    pub a_interactive_foliage_actor_capsule_touched: *mut crate::ffi::UFunctionOpague,
    pub u_procedural_foliage_spawner_simulate: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            a_instanced_foliage_actor_remove_all_instances: std::ptr::null_mut(),
            a_instanced_foliage_actor_add_instances: std::ptr::null_mut(),
            u_foliage_statistics_foliage_overlapping_sphere_count: std::ptr::null_mut(),
            u_foliage_statistics_foliage_overlapping_box_transforms: std::ptr::null_mut(),
            u_foliage_statistics_foliage_overlapping_box_count: std::ptr::null_mut(),
            a_interactive_foliage_actor_capsule_touched: std::ptr::null_mut(),
            u_procedural_foliage_spawner_simulate: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AInstancedFoliageActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllInstances"),
                &raw mut __FUNCTION_PTRS.a_instanced_foliage_actor_remove_all_instances,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInstances"),
                &raw mut __FUNCTION_PTRS.a_instanced_foliage_actor_add_instances,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFoliageStatistics::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FoliageOverlappingSphereCount"),
                &raw mut __FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_sphere_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FoliageOverlappingBoxTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FoliageOverlappingBoxCount"),
                &raw mut __FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_count,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AInteractiveFoliageActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CapsuleTouched"),
                &raw mut __FUNCTION_PTRS.a_interactive_foliage_actor_capsule_touched,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UProceduralFoliageSpawner::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Simulate"),
                &raw mut __FUNCTION_PTRS.u_procedural_foliage_spawner_simulate,
            );
        }
    }
}
#[repr(C, align(16))]
pub struct FProceduralFoliageInstance {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub location: crate::bindings::core_u_object::FVector,
    pub age: f32,
    pub normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 40],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageInstancedStaticMeshComponent")
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
pub struct UFoliageType {
    #[doc(hidden)]
    pub(crate) __padding_172: [u8; 172],
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
    pub(crate) __padding_272: [u8; 12],
    pub flags_272: u8,
    pub collision_scale: crate::bindings::core_u_object::FVector,
    pub average_normal_sample_count: i32,
    #[doc(hidden)]
    pub(crate) __padding_392: [u8; 84],
    pub mobility: crate::bindings::engine::EComponentMobility,
    #[doc(hidden)]
    pub(crate) __padding_404: [u8; 11],
    pub flags_404: u8,
    #[doc(hidden)]
    pub(crate) __padding_408: [u8; 3],
    pub flags_408: u8,
    #[doc(hidden)]
    pub(crate) __padding_412: [u8; 3],
    pub flags_412: u8,
    #[doc(hidden)]
    pub(crate) __padding_416: [u8; 3],
    pub shadow_cache_invalidation_behavior: crate::bindings::engine::EShadowCacheInvalidationBehavior,
    pub overridden_light_map_res: i32,
    pub lightmap_type: crate::bindings::engine::ELightmapType,
    #[doc(hidden)]
    pub(crate) __padding_428: [u8; 3],
    pub flags_428: u8,
    #[doc(hidden)]
    pub(crate) __padding_432: [u8; 3],
    pub flags_432: u8,
    pub world_position_offset_disable_distance: i32,
    #[doc(hidden)]
    pub(crate) __padding_873: [u8; 433],
    pub lighting_channels: crate::bindings::engine::FLightingChannels,
    #[doc(hidden)]
    pub(crate) __padding_876: [u8; 2],
    pub flags_876: u8,
    #[doc(hidden)]
    pub(crate) __padding_880: [u8; 3],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    #[doc(hidden)]
    pub(crate) __padding_1256: [u8; 360],
    pub runtime_virtual_textures: TArray<
        UPtr<crate::bindings::engine::URuntimeVirtualTexture>,
    >,
    pub virtual_texture_cull_mips: i32,
    pub virtual_texture_render_pass_type: crate::bindings::engine::ERuntimeVirtualTextureMainPassType,
    #[doc(hidden)]
    pub(crate) __padding_1280: [u8; 3],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_Actor")
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
pub struct UFoliageType_InstancedStaticMesh {
    #[doc(hidden)]
    pub(crate) __padding_1352: [u8; 1352],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageType_InstancedStaticMesh")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AInstancedFoliageActor")
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
    pub fn remove_all_instances(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_foliage_type: UPtr<UFoliageType>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .a_instanced_foliage_actor_remove_all_instances,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_foliage_type,
                __buffer.add(8).cast::<UPtr<UFoliageType>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage::AInstancedFoliageActor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .a_instanced_foliage_actor_remove_all_instances,
                __buffer,
            )
        };
    }
    pub fn add_instances(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_foliage_type: UPtr<UFoliageType>,
        in_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .a_instanced_foliage_actor_add_instances,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_foliage_type,
                __buffer.add(8).cast::<UPtr<UFoliageType>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_transforms,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage::AInstancedFoliageActor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .a_instanced_foliage_actor_add_instances,
                __buffer,
            )
        };
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteractiveFoliageComponent")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFoliageStatistics")
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
    pub fn foliage_overlapping_sphere_count(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        center_position: crate::bindings::core_u_object::FVector,
        radius: f32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_sphere_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &center_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(40).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::foliage::UFoliageStatistics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_sphere_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<i32>().read() }
    }
    pub fn foliage_overlapping_box_transforms(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        box_: crate::bindings::core_u_object::FBox,
        out_transforms: &mut TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &box_,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_transforms,
                __buffer
                    .add(72)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage::UFoliageStatistics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .swap(out_transforms);
        }
    }
    pub fn foliage_overlapping_box_count(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
        box_: crate::bindings::core_u_object::FBox,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &box_,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::foliage::UFoliageStatistics::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_foliage_statistics_foliage_overlapping_box_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<i32>().read() }
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGrassInstancedStaticMeshComponent")
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
pub struct AInteractiveFoliageActor {
    #[doc(hidden)]
    pub(crate) __padding_1256: [u8; 1256],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AInteractiveFoliageActor")
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AProceduralFoliageBlockingVolume")
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
pub struct UProceduralFoliageComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageComponent")
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
pub struct UProceduralFoliageSpawner {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageSpawner")
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
    pub fn simulate(&mut self, num_steps: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_procedural_foliage_spawner_simulate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&num_steps, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::foliage::__FUNCTION_PTRS
                    .u_procedural_foliage_spawner_simulate,
                __buffer,
            )
        };
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UProceduralFoliageTile")
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
pub struct AProceduralFoliageVolume {
    #[doc(hidden)]
    pub(crate) __padding_1216: [u8; 1216],
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
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AProceduralFoliageVolume")
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
