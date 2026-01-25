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
    pub u_entity_component_get_entity: *mut crate::ffi::UFunctionOpague,
    pub a_rust_actor_get_entity_component: *mut crate::ffi::UFunctionOpague,
    pub a_rust_actor_get_entity: *mut crate::ffi::UFunctionOpague,
    pub u_rust_extension_f_hit_result_new: *mut crate::ffi::UFunctionOpague,
    pub a_rust_game_mode_base_on_actor_hit: *mut crate::ffi::UFunctionOpague,
    pub a_rust_game_mode_base_on_actor_end_overlap: *mut crate::ffi::UFunctionOpague,
    pub a_rust_game_mode_base_on_actor_destroyed: *mut crate::ffi::UFunctionOpague,
    pub a_rust_game_mode_base_on_actor_begin_overlap: *mut crate::ffi::UFunctionOpague,
    pub u_rust_reflection_library_k2_has_component: *mut crate::ffi::UFunctionOpague,
    pub u_rust_reflection_library_k2_get_reflection_vector3: *mut crate::ffi::UFunctionOpague,
    pub u_rust_reflection_library_k2_get_reflection_quat: *mut crate::ffi::UFunctionOpague,
    pub u_rust_reflection_library_k2_get_reflection_float: *mut crate::ffi::UFunctionOpague,
    pub u_rust_reflection_library_k2_get_reflection_bool: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_entity_component_get_entity: std::ptr::null_mut(),
            a_rust_actor_get_entity_component: std::ptr::null_mut(),
            a_rust_actor_get_entity: std::ptr::null_mut(),
            u_rust_extension_f_hit_result_new: std::ptr::null_mut(),
            a_rust_game_mode_base_on_actor_hit: std::ptr::null_mut(),
            a_rust_game_mode_base_on_actor_end_overlap: std::ptr::null_mut(),
            a_rust_game_mode_base_on_actor_destroyed: std::ptr::null_mut(),
            a_rust_game_mode_base_on_actor_begin_overlap: std::ptr::null_mut(),
            u_rust_reflection_library_k2_has_component: std::ptr::null_mut(),
            u_rust_reflection_library_k2_get_reflection_vector3: std::ptr::null_mut(),
            u_rust_reflection_library_k2_get_reflection_quat: std::ptr::null_mut(),
            u_rust_reflection_library_k2_get_reflection_float: std::ptr::null_mut(),
            u_rust_reflection_library_k2_get_reflection_bool: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEntityComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntity"),
            &raw mut __FUNCTION_PTRS.u_entity_component_get_entity,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ARustActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntityComponent"),
            &raw mut __FUNCTION_PTRS.a_rust_actor_get_entity_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntity"),
            &raw mut __FUNCTION_PTRS.a_rust_actor_get_entity,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URustExtension_FHitResult::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("New"),
            &raw mut __FUNCTION_PTRS.u_rust_extension_f_hit_result_new,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ARustGameModeBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorHit"),
            &raw mut __FUNCTION_PTRS.a_rust_game_mode_base_on_actor_hit,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorEndOverlap"),
            &raw mut __FUNCTION_PTRS.a_rust_game_mode_base_on_actor_end_overlap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorDestroyed"),
            &raw mut __FUNCTION_PTRS.a_rust_game_mode_base_on_actor_destroyed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorBeginOverlap"),
            &raw mut __FUNCTION_PTRS.a_rust_game_mode_base_on_actor_begin_overlap,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = URustReflectionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_HasComponent"),
            &raw mut __FUNCTION_PTRS.u_rust_reflection_library_k2_has_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionVector3"),
            &raw mut __FUNCTION_PTRS.u_rust_reflection_library_k2_get_reflection_vector3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionQuat"),
            &raw mut __FUNCTION_PTRS.u_rust_reflection_library_k2_get_reflection_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionFloat"),
            &raw mut __FUNCTION_PTRS.u_rust_reflection_library_k2_get_reflection_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionBool"),
            &raw mut __FUNCTION_PTRS.u_rust_reflection_library_k2_get_reflection_bool,
        );
    }
}
#[repr(C, align(8))]
pub struct FEntity {
    pub(crate) __padding_end: [u8; 8],
}
impl FEntity {}
#[repr(C, align(8))]
pub struct UAnimNotify_RustEvent {
    __padding_end: [u8; 176],
}
impl UAnimNotify_RustEvent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_RustEvent")
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
pub struct UUuid {
    __padding_end: [u8; 64],
}
impl UUuid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UUuid").unwrap()
    }
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
pub struct UEntityComponent {
    __padding_end: [u8; 328],
}
impl UEntityComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEntityComponent")
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
    pub fn get_entity(&mut self) -> FEntity {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_entity_component_get_entity,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_entity_component_get_entity,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FEntity>().read() }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_GetComponentRust {
    __padding_end: [u8; 240],
}
impl UK2Node_GetComponentRust {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_GetComponentRust")
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
pub struct ARustActor {
    __padding_end: [u8; 1152],
}
impl ARustActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARustActor")
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
    pub fn get_entity_component(&mut self) -> UPtr<UEntityComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .a_rust_actor_get_entity_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .a_rust_actor_get_entity_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UEntityComponent>>().read() }
    }
    pub fn get_entity(&mut self) -> FEntity {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS.a_rust_actor_get_entity,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS.a_rust_actor_get_entity,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FEntity>().read() }
    }
}
#[repr(C, align(8))]
pub struct ARustBindingsActor {
    __padding_end: [u8; 1136],
}
impl ARustBindingsActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARustBindingsActor")
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
pub struct URustExtension {
    __padding_end: [u8; 48],
}
impl URustExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension")
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
pub struct URustExtension_FHitResult {
    __padding_end: [u8; 48],
}
impl URustExtension_FHitResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustExtension_FHitResult")
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
    pub fn new() -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_f_hit_result_new,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_FHitResult::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_extension_f_hit_result_new,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::engine::FHitResult>().read() }
    }
}
#[repr(C, align(8))]
pub struct ARustGameModeBase {
    __padding_end: [u8; 1328],
}
impl ARustGameModeBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ARustGameModeBase")
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
pub struct UUEdGraphSchema_Rust {
    __padding_end: [u8; 48],
}
impl UUEdGraphSchema_Rust {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUEdGraphSchema_Rust")
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
pub struct URustReflectionLibrary {
    __padding_end: [u8; 48],
}
impl URustReflectionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URustReflectionLibrary")
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
    pub fn k2_has_component(id: UPtr<UUuid>, entity_id: FEntity) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_has_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&id, __buffer.add(0).cast::<UPtr<UUuid>>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entity_id,
                __buffer.add(8).cast::<FEntity>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustReflectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_has_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn k2_get_reflection_vector3(
        id: UPtr<UUuid>,
        entity_id: FEntity,
        index: i32,
        out: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_vector3,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&id, __buffer.add(0).cast::<UPtr<UUuid>>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entity_id,
                __buffer.add(8).cast::<FEntity>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustReflectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_vector3,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().swap(out);
        }
    }
    pub fn k2_get_reflection_quat(
        id: UPtr<UUuid>,
        entity_id: FEntity,
        index: i32,
        out: &mut crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&id, __buffer.add(0).cast::<UPtr<UUuid>>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entity_id,
                __buffer.add(8).cast::<FEntity>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rust_plugin::URustReflectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>().swap(out);
        }
    }
    pub fn k2_get_reflection_float(
        id: UPtr<UUuid>,
        entity_id: FEntity,
        index: i32,
        out: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&id, __buffer.add(0).cast::<UPtr<UUuid>>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entity_id,
                __buffer.add(8).cast::<FEntity>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustReflectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(20).cast::<f32>().swap(out);
        }
    }
    pub fn k2_get_reflection_bool(
        id: UPtr<UUuid>,
        entity_id: FEntity,
        index: i32,
        out: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&id, __buffer.add(0).cast::<UPtr<UUuid>>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &entity_id,
                __buffer.add(8).cast::<FEntity>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out, __buffer.add(20).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::rust_plugin::URustReflectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::__FUNCTION_PTRS
                    .u_rust_reflection_library_k2_get_reflection_bool,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(20).cast::<bool>().swap(out);
        }
    }
}
