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
    pub u_animation_sharing_state_processor_process_actor_state: *mut crate::ffi::UFunctionOpague,
    pub u_animation_sharing_state_processor_get_animation_state_enum: *mut crate::ffi::UFunctionOpague,
    pub u_anim_sharing_state_instance_get_instanced_actors: *mut crate::ffi::UFunctionOpague,
    pub u_animation_sharing_manager_register_actor_with_skeleton_bp: *mut crate::ffi::UFunctionOpague,
    pub u_animation_sharing_manager_get_animation_sharing_manager: *mut crate::ffi::UFunctionOpague,
    pub u_animation_sharing_manager_create_animation_sharing_manager: *mut crate::ffi::UFunctionOpague,
    pub u_animation_sharing_manager_animation_sharing_enabled: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_sharing_state_processor_process_actor_state: std::ptr::null_mut(),
            u_animation_sharing_state_processor_get_animation_state_enum: std::ptr::null_mut(),
            u_anim_sharing_state_instance_get_instanced_actors: std::ptr::null_mut(),
            u_animation_sharing_manager_register_actor_with_skeleton_bp: std::ptr::null_mut(),
            u_animation_sharing_manager_get_animation_sharing_manager: std::ptr::null_mut(),
            u_animation_sharing_manager_create_animation_sharing_manager: std::ptr::null_mut(),
            u_animation_sharing_manager_animation_sharing_enabled: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimationSharingStateProcessor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProcessActorState"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_state_processor_process_actor_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimationStateEnum"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_state_processor_get_animation_state_enum,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimSharingStateInstance::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstancedActors"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_sharing_state_instance_get_instanced_actors,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimationSharingManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterActorWithSkeletonBP"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_manager_register_actor_with_skeleton_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimationSharingManager"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_manager_get_animation_sharing_manager,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateAnimationSharingManager"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_manager_create_animation_sharing_manager,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AnimationSharingEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_animation_sharing_manager_animation_sharing_enabled,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimationSharingStateProcessor {
    __padding_end: [u8; 96],
}
impl UAnimationSharingStateProcessor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingStateProcessor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingStateProcessor")
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
    pub fn process_actor_state(
        &mut self,
        out_state: &mut i32,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        current_state: u8,
        on_demand_state: u8,
        b_should_process: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_state_processor_process_actor_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(out_state, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &current_state,
                __buffer.add(16).cast::<u8>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_demand_state,
                __buffer.add(17).cast::<u8>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_should_process,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_state_processor_process_actor_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(out_state);
        }
        unsafe {
            __buffer.add(18).cast::<bool>().swap(b_should_process);
        }
        std::mem::forget(in_actor);
        std::mem::forget(current_state);
        std::mem::forget(on_demand_state);
    }
    pub fn get_animation_state_enum(
        &mut self,
    ) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_state_processor_get_animation_state_enum,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_state_processor_get_animation_state_enum,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
}
#[repr(C, align(16))]
pub struct UAnimSharingStateInstance {
    #[doc(hidden)]
    pub(crate) __padding_1128: [u8; 1128],
    pub animation_to_play: UPtr<crate::bindings::engine::UAnimSequence>,
    pub permutation_time_offset: f32,
    pub play_rate: f32,
    pub b_state_bool: bool,
    __padding_end: [u8; 23],
}
impl UAnimSharingStateInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingStateInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingStateInstance")
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
    pub fn get_instanced_actors(
        &mut self,
        actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_anim_sharing_state_instance_get_instanced_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_anim_sharing_state_instance_get_instanced_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(actors);
        }
    }
}
#[repr(C, align(16))]
pub struct UAnimSharingTransitionInstance {
    #[doc(hidden)]
    pub(crate) __padding_1128: [u8; 1128],
    pub from_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub to_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub blend_time: f32,
    pub b_blend_bool: bool,
}
impl UAnimSharingTransitionInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingTransitionInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingTransitionInstance")
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
#[repr(C, align(16))]
pub struct UAnimSharingAdditiveInstance {
    #[doc(hidden)]
    pub(crate) __padding_1128: [u8; 1128],
    pub base_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub additive_animation: TWeakObjectPtr<crate::bindings::engine::UAnimSequence>,
    pub alpha: f32,
    pub b_state_bool: bool,
}
impl UAnimSharingAdditiveInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingAdditiveInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingAdditiveInstance")
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
pub struct UAnimSharingInstance {
    __padding_end: [u8; 648],
}
impl UAnimSharingInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingInstance")
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
pub struct UAnimationSharingManager {
    __padding_end: [u8; 584],
}
impl UAnimationSharingManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingManager")
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
    pub fn register_actor_with_skeleton_bp(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        sharing_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_register_actor_with_skeleton_bp,
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
                &sharing_skeleton,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_register_actor_with_skeleton_bp,
                __buffer,
            )
        };
        std::mem::forget(in_actor);
        std::mem::forget(sharing_skeleton);
    }
    pub fn get_animation_sharing_manager(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UAnimationSharingManager> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_get_animation_sharing_manager,
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
        let __object_ptr = crate::bindings::animation_sharing::UAnimationSharingManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_get_animation_sharing_manager,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        unsafe { __buffer.add(8).cast::<UPtr<UAnimationSharingManager>>().read() }
    }
    pub fn create_animation_sharing_manager(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        setup: UPtr<UAnimationSharingSetup>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_create_animation_sharing_manager,
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
                &setup,
                __buffer.add(8).cast::<UPtr<UAnimationSharingSetup>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_sharing::UAnimationSharingManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_create_animation_sharing_manager,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        std::mem::forget(setup);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn animation_sharing_enabled() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_animation_sharing_enabled,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::animation_sharing::UAnimationSharingManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::__FUNCTION_PTRS
                    .u_animation_sharing_manager_animation_sharing_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimationSharingSetup {
    __padding_end: [u8; 416],
}
impl UAnimationSharingSetup {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingSetup")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingSetup")
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
