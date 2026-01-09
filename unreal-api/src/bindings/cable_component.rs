#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[doc(hidden)]
pub static mut __FUNCTION_PTRS: FunctionPtrs = FunctionPtrs::empty();
pub struct FunctionPtrs {
    pub u_cable_component_set_attach_end_to_component: *mut crate::ffi::UFunctionOpague,
    pub u_cable_component_set_attach_end_to: *mut crate::ffi::UFunctionOpague,
    pub u_cable_component_get_cable_particle_locations: *mut crate::ffi::UFunctionOpague,
    pub u_cable_component_get_attached_component: *mut crate::ffi::UFunctionOpague,
    pub u_cable_component_get_attached_actor: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_cable_component_set_attach_end_to_component: std::ptr::null_mut(),
            u_cable_component_set_attach_end_to: std::ptr::null_mut(),
            u_cable_component_get_cable_particle_locations: std::ptr::null_mut(),
            u_cable_component_get_attached_component: std::ptr::null_mut(),
            u_cable_component_get_attached_actor: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCableComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttachEndToComponent"),
            &raw mut __FUNCTION_PTRS.u_cable_component_set_attach_end_to_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttachEndTo"),
            &raw mut __FUNCTION_PTRS.u_cable_component_set_attach_end_to,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCableParticleLocations"),
            &raw mut __FUNCTION_PTRS.u_cable_component_get_cable_particle_locations,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttachedComponent"),
            &raw mut __FUNCTION_PTRS.u_cable_component_get_attached_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttachedActor"),
            &raw mut __FUNCTION_PTRS.u_cable_component_get_attached_actor,
        );
    }
}
#[repr(C, align(8))]
pub struct ACableActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub cable_component: UPtr<UCableComponent>,
}
impl ACableActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ACableActor")
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
pub struct UCableComponent {
    #[doc(hidden)]
    pub(crate) __padding_1576: [u8; 1576],
    pub b_attach_start: bool,
    pub b_attach_end: bool,
    #[doc(hidden)]
    pub(crate) __padding_1648: [u8; 64],
    pub end_location: crate::bindings::core_u_object::FVector,
    pub cable_length: f32,
    pub num_segments: i32,
    pub substep_time: f32,
    pub solver_iterations: i32,
    pub b_enable_stiffness: bool,
    pub b_use_substepping: bool,
    pub b_skip_cable_update_when_not_visible: bool,
    pub b_skip_cable_update_when_not_owner_recently_rendered: bool,
    pub b_enable_collision: bool,
    pub collision_friction: f32,
    pub cable_force: crate::bindings::core_u_object::FVector,
    pub cable_gravity_scale: f32,
    pub cable_width: f32,
    pub num_sides: i32,
    pub tile_material: f32,
    pub b_reset_after_teleport: bool,
    pub teleport_distance_threshold: f32,
    pub teleport_rotation_threshold: f32,
    pub b_teleport_after_reattach: bool,
    __padding_end: [u8; 307],
}
impl UCableComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCableComponent")
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
    pub fn set_attach_end_to_component(
        &mut self,
        component: UPtr<crate::bindings::engine::USceneComponent>,
        socket_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_set_attach_end_to_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_set_attach_end_to_component,
                __buffer,
            )
        };
    }
    pub fn set_attach_end_to(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        component_property: FName,
        socket_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_set_attach_end_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_property,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_set_attach_end_to,
                __buffer,
            )
        };
    }
    pub fn get_cable_particle_locations(
        &self,
        locations: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_cable_particle_locations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                locations,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_cable_particle_locations,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(locations);
        }
    }
    pub fn get_attached_component(
        &self,
    ) -> UPtr<crate::bindings::engine::USceneComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_attached_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_attached_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .read()
        }
    }
    pub fn get_attached_actor(&self) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_attached_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::cable_component::__FUNCTION_PTRS
                    .u_cable_component_get_attached_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
}
