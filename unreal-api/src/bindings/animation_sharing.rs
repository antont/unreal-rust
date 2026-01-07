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
pub static mut U_ANIMATION_SHARING_STATE_PROCESSOR_PROCESS_ACTOR_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_SHARING_STATE_PROCESSOR_GET_ANIMATION_STATE_ENUM: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIM_SHARING_STATE_INSTANCE_GET_INSTANCED_ACTORS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_SHARING_MANAGER_REGISTER_ACTOR_WITH_SKELETON_BP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_SHARING_MANAGER_GET_ANIMATION_SHARING_MANAGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_SHARING_MANAGER_CREATE_ANIMATION_SHARING_MANAGER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_ANIMATION_SHARING_MANAGER_ANIMATION_SHARING_ENABLED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationSharingStateProcessor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ProcessActorState"),
            &raw mut U_ANIMATION_SHARING_STATE_PROCESSOR_PROCESS_ACTOR_STATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationStateEnum"),
            &raw mut U_ANIMATION_SHARING_STATE_PROCESSOR_GET_ANIMATION_STATE_ENUM,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimSharingStateInstance::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstancedActors"),
            &raw mut U_ANIM_SHARING_STATE_INSTANCE_GET_INSTANCED_ACTORS,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationSharingManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterActorWithSkeletonBP"),
            &raw mut U_ANIMATION_SHARING_MANAGER_REGISTER_ACTOR_WITH_SKELETON_BP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationSharingManager"),
            &raw mut U_ANIMATION_SHARING_MANAGER_GET_ANIMATION_SHARING_MANAGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateAnimationSharingManager"),
            &raw mut U_ANIMATION_SHARING_MANAGER_CREATE_ANIMATION_SHARING_MANAGER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AnimationSharingEnabled"),
            &raw mut U_ANIMATION_SHARING_MANAGER_ANIMATION_SHARING_ENABLED,
        );
    }
}
#[repr(C, align(8))]
pub struct UAnimationSharingStateProcessor {
    __padding_end: [u8; 96],
}
impl UAnimationSharingStateProcessor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingStateProcessor")
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
pub struct UAnimSharingStateInstance {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub animation_to_play: UPtr<crate::bindings::engine::UAnimSequence>,
    pub permutation_time_offset: f32,
    pub play_rate: f32,
    pub b_state_bool: bool,
    __padding_end: [u8; 23],
}
impl UAnimSharingStateInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingStateInstance")
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
                crate::bindings::animation_sharing::U_ANIM_SHARING_STATE_INSTANCE_GET_INSTANCED_ACTORS,
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
                crate::bindings::animation_sharing::U_ANIM_SHARING_STATE_INSTANCE_GET_INSTANCED_ACTORS,
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
    __padding_1128: [u8; 1128],
    pub from_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub to_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub blend_time: f32,
    pub b_blend_bool: bool,
}
impl UAnimSharingTransitionInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingTransitionInstance")
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
pub struct UAnimSharingAdditiveInstance {
    #[doc(hidden)]
    __padding_1128: [u8; 1128],
    pub base_component: TWeakObjectPtr<crate::bindings::engine::USkeletalMeshComponent>,
    pub additive_animation: TWeakObjectPtr<crate::bindings::engine::UAnimSequence>,
    pub alpha: f32,
    pub b_state_bool: bool,
}
impl UAnimSharingAdditiveInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingAdditiveInstance")
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
pub struct UAnimSharingInstance {
    __padding_end: [u8; 648],
}
impl UAnimSharingInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSharingInstance")
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
pub struct UAnimationSharingManager {
    __padding_end: [u8; 584],
}
impl UAnimationSharingManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingManager")
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_REGISTER_ACTOR_WITH_SKELETON_BP,
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_REGISTER_ACTOR_WITH_SKELETON_BP,
                __buffer,
            )
        };
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_GET_ANIMATION_SHARING_MANAGER,
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_GET_ANIMATION_SHARING_MANAGER,
                __buffer,
            )
        };
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_CREATE_ANIMATION_SHARING_MANAGER,
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_CREATE_ANIMATION_SHARING_MANAGER,
                __buffer,
            )
        };
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
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_ANIMATION_SHARING_ENABLED,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::animation_sharing::UAnimationSharingManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_sharing::U_ANIMATION_SHARING_MANAGER_ANIMATION_SHARING_ENABLED,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationSharingSetup")
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
