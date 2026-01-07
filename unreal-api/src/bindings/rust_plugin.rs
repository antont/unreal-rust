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
pub static mut U_ENTITY_COMPONENT_GET_ENTITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_ACTOR_GET_ENTITY_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_ACTOR_GET_ENTITY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_EXTENSION_F_HIT_RESULT_NEW: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_GAME_MODE_BASE_ON_ACTOR_HIT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_GAME_MODE_BASE_ON_ACTOR_END_OVERLAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_GAME_MODE_BASE_ON_ACTOR_DESTROYED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut A_RUST_GAME_MODE_BASE_ON_ACTOR_BEGIN_OVERLAP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_REFLECTION_LIBRARY_K2_HAS_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_VECTOR3: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_QUAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_FLOAT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_BOOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEntityComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntity"),
            &raw mut U_ENTITY_COMPONENT_GET_ENTITY,
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
            &raw mut A_RUST_ACTOR_GET_ENTITY_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEntity"),
            &raw mut A_RUST_ACTOR_GET_ENTITY,
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
            &raw mut U_RUST_EXTENSION_F_HIT_RESULT_NEW,
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
            &raw mut A_RUST_GAME_MODE_BASE_ON_ACTOR_HIT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorEndOverlap"),
            &raw mut A_RUST_GAME_MODE_BASE_ON_ACTOR_END_OVERLAP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorDestroyed"),
            &raw mut A_RUST_GAME_MODE_BASE_ON_ACTOR_DESTROYED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnActorBeginOverlap"),
            &raw mut A_RUST_GAME_MODE_BASE_ON_ACTOR_BEGIN_OVERLAP,
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
            &raw mut U_RUST_REFLECTION_LIBRARY_K2_HAS_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionVector3"),
            &raw mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_VECTOR3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionQuat"),
            &raw mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_QUAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionFloat"),
            &raw mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_FLOAT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_GetReflectionBool"),
            &raw mut U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_BOOL,
        );
    }
}
#[repr(C, align(8))]
pub struct FEntity {
    __padding_end: [u8; 8],
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
                crate::bindings::rust_plugin::U_ENTITY_COMPONENT_GET_ENTITY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::U_ENTITY_COMPONENT_GET_ENTITY,
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
                crate::bindings::rust_plugin::A_RUST_ACTOR_GET_ENTITY_COMPONENT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::A_RUST_ACTOR_GET_ENTITY_COMPONENT,
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
                crate::bindings::rust_plugin::A_RUST_ACTOR_GET_ENTITY,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::A_RUST_ACTOR_GET_ENTITY,
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
                crate::bindings::rust_plugin::U_RUST_EXTENSION_F_HIT_RESULT_NEW,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rust_plugin::URustExtension_FHitResult::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rust_plugin::U_RUST_EXTENSION_F_HIT_RESULT_NEW,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::engine::FHitResult>().read() }
    }
    pub fn verify_layout() {}
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_HAS_COMPONENT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_HAS_COMPONENT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_VECTOR3,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_VECTOR3,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_QUAT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_QUAT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_FLOAT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_FLOAT,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_BOOL,
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
                crate::bindings::rust_plugin::U_RUST_REFLECTION_LIBRARY_K2_GET_REFLECTION_BOOL,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(20).cast::<bool>().swap(out);
        }
    }
}
