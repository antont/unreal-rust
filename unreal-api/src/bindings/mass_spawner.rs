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
    pub u_mass_entity_config_asset_validate_entity_config: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_unload_config: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_scale_spawning_count: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_get_spawning_count_scale: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_get_count: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_do_spawning: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_do_despawning: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_debug_spawn: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_debug_clear: *mut crate::ffi::UFunctionOpague,
    pub a_mass_spawner_clear_templates: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mass_entity_config_asset_validate_entity_config: std::ptr::null_mut(),
            a_mass_spawner_unload_config: std::ptr::null_mut(),
            a_mass_spawner_scale_spawning_count: std::ptr::null_mut(),
            a_mass_spawner_get_spawning_count_scale: std::ptr::null_mut(),
            a_mass_spawner_get_count: std::ptr::null_mut(),
            a_mass_spawner_do_spawning: std::ptr::null_mut(),
            a_mass_spawner_do_despawning: std::ptr::null_mut(),
            a_mass_spawner_debug_spawn: std::ptr::null_mut(),
            a_mass_spawner_debug_clear: std::ptr::null_mut(),
            a_mass_spawner_clear_templates: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMassEntityConfigAsset::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ValidateEntityConfig"),
                &raw mut __FUNCTION_PTRS
                    .u_mass_entity_config_asset_validate_entity_config,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AMassSpawner::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnloadConfig"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_unload_config,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScaleSpawningCount"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_scale_spawning_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpawningCountScale"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_get_spawning_count_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCount"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_get_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoSpawning"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_do_spawning,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoDespawning"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_do_despawning,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DEBUG_Spawn"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_debug_spawn,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DEBUG_Clear"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_debug_clear,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearTemplates"),
                &raw mut __FUNCTION_PTRS.a_mass_spawner_clear_templates,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMassSpawnedEntityType {
    pub entity_config: TSoftObjectPtr<UMassEntityConfigAsset>,
    pub proportion: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FMassSpawnedEntityType {}
#[repr(C, align(8))]
pub struct FMassSpawnDataGenerator {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub generator_instance: UPtr<UMassEntitySpawnDataGeneratorBase>,
    pub proportion: f32,
}
impl FMassSpawnDataGenerator {}
#[repr(C, align(8))]
pub struct UMassEntityTraitBase {
    __padding_end: [u8; 48],
}
impl UMassEntityTraitBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityTraitBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityTraitBase")
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
pub struct UMassAssortedFragmentsTrait {
    __padding_end: [u8; 80],
}
impl UMassAssortedFragmentsTrait {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassAssortedFragmentsTrait")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassAssortedFragmentsTrait")
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
pub struct UMassEntityConfigAsset {
    __padding_end: [u8; 104],
}
impl UMassEntityConfigAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityConfigAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityConfigAsset")
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
pub struct UMassEntitySpawnDataGeneratorBase {
    __padding_end: [u8; 56],
}
impl UMassEntitySpawnDataGeneratorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntitySpawnDataGeneratorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntitySpawnDataGeneratorBase")
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
pub struct UMassEntityEQSSpawnPointsGenerator {
    __padding_end: [u8; 136],
}
impl UMassEntityEQSSpawnPointsGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityEQSSpawnPointsGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityEQSSpawnPointsGenerator")
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
pub struct UMassEntityZoneGraphSpawnPointsGenerator {
    __padding_end: [u8; 80],
}
impl UMassEntityZoneGraphSpawnPointsGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityZoneGraphSpawnPointsGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassEntityZoneGraphSpawnPointsGenerator")
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
pub struct AMassSpawner {
    #[doc(hidden)]
    pub(crate) __padding_1192: [u8; 1192],
    pub entity_types: TArray<FMassSpawnedEntityType>,
    __padding_end: [u8; 120],
}
impl AMassSpawner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMassSpawner")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AMassSpawner")
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
    pub fn unload_config(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_unload_config,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_unload_config,
                __buffer,
            )
        };
    }
    pub fn scale_spawning_count(&mut self, scale: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_scale_spawning_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_scale_spawning_count,
                __buffer,
            )
        };
    }
    pub fn get_spawning_count_scale(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_get_spawning_count_scale,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_get_spawning_count_scale,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS.a_mass_spawner_get_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS.a_mass_spawner_get_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn do_spawning(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_do_spawning,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_do_spawning,
                __buffer,
            )
        };
    }
    pub fn do_despawning(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_do_despawning,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_do_despawning,
                __buffer,
            )
        };
    }
    pub fn debug_spawn(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_debug_spawn,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_debug_spawn,
                __buffer,
            )
        };
    }
    pub fn debug_clear(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_debug_clear,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_debug_clear,
                __buffer,
            )
        };
    }
    pub fn clear_templates(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_clear_templates,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mass_spawner::__FUNCTION_PTRS
                    .a_mass_spawner_clear_templates,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMassSpawnerSubsystem {
    __padding_end: [u8; 208],
}
impl UMassSpawnerSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassSpawnerSubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassSpawnerSubsystem")
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
pub struct UMassSpawnLocationProcessor {
    __padding_end: [u8; 1136],
}
impl UMassSpawnLocationProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassSpawnLocationProcessor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassSpawnLocationProcessor")
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
pub struct UMassTranslator {
    __padding_end: [u8; 272],
}
impl UMassTranslator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassTranslator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMassTranslator")
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
pub struct FMassSpawner_OnSpawningFinishedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMassSpawner_OnDespawningFinishedEvent {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMassTranslationDirection(pub u8);
impl EMassTranslationDirection {
    pub const NONE: EMassTranslationDirection = EMassTranslationDirection(0);
    pub const INITIALIZATION_ONLY: EMassTranslationDirection = EMassTranslationDirection(
        1,
    );
    pub const ACTOR_TO_MASS: EMassTranslationDirection = EMassTranslationDirection(2);
    pub const MASS_TO_ACTOR: EMassTranslationDirection = EMassTranslationDirection(4);
    pub const BOTH_WAYS: EMassTranslationDirection = EMassTranslationDirection(6);
}
