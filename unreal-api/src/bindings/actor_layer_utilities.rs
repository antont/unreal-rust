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
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULayersBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromLayer"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActors"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorToLayer"),
            &raw mut U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER,
        );
    }
}
#[repr(C, align(4))]
pub struct FActorLayer {
    pub name: FName,
}
impl FActorLayer {}
#[repr(C, align(8))]
pub struct ULayersBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl ULayersBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayersBlueprintLibrary")
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
    pub fn remove_actor_from_layer(
        in_actor: UPtr<crate::bindings::engine::AActor>,
        layer: &FActorLayer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer,
                __buffer.add(8).cast::<FActorLayer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::actor_layer_utilities::ULayersBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_REMOVE_ACTOR_FROM_LAYER,
                __buffer,
            )
        };
    }
    pub fn get_actors(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        actor_layer: &FActorLayer,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS,
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
                actor_layer,
                __buffer.add(8).cast::<FActorLayer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::actor_layer_utilities::ULayersBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_GET_ACTORS,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn add_actor_to_layer(
        in_actor: UPtr<crate::bindings::engine::AActor>,
        layer: &FActorLayer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer,
                __buffer.add(8).cast::<FActorLayer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::actor_layer_utilities::ULayersBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::actor_layer_utilities::U_LAYERS_BLUEPRINT_LIBRARY_ADD_ACTOR_TO_LAYER,
                __buffer,
            )
        };
    }
}
