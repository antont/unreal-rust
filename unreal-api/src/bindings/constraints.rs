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
pub static mut U_CONSTRAINTS_MANAGER_ON_CONSTRAINT_REMOVED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_MANAGER_ON_CONSTRAINT_ADDED_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_THIS_CONSTRAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_CONSTRAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_GET_CONSTRAINTS_ARRAY: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_HANDLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_COMPONENT_HANDLE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_FROM_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINTS_SCRIPTING_LIBRARY_ADD_CONSTRAINT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINT_SUBSYSTEM_ON_CONSTRAINT_REMOVED_FROM_SYSTEM_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_CONSTRAINT_SUBSYSTEM_ON_CONSTRAINT_ADDED_TO_SYSTEM_DELEGATE_SIGNATURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UConstraintsManager::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnConstraintRemoved__DelegateSignature"),
            &raw mut U_CONSTRAINTS_MANAGER_ON_CONSTRAINT_REMOVED_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnConstraintAdded__DelegateSignature"),
            &raw mut U_CONSTRAINTS_MANAGER_ON_CONSTRAINT_ADDED_DELEGATE_SIGNATURE,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UConstraintsScriptingLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveThisConstraint"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_THIS_CONSTRAINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveConstraint"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_CONSTRAINT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetConstraintsArray"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_GET_CONSTRAINTS_ARRAY,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateTransformableHandle"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_HANDLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateTransformableComponentHandle"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_COMPONENT_HANDLE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFromType"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_FROM_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddConstraint"),
            &raw mut U_CONSTRAINTS_SCRIPTING_LIBRARY_ADD_CONSTRAINT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UConstraintSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnConstraintRemovedFromSystem__DelegateSignature",
            ),
            &raw mut U_CONSTRAINT_SUBSYSTEM_ON_CONSTRAINT_REMOVED_FROM_SYSTEM_DELEGATE_SIGNATURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnConstraintAddedToSystem__DelegateSignature"),
            &raw mut U_CONSTRAINT_SUBSYSTEM_ON_CONSTRAINT_ADDED_TO_SYSTEM_DELEGATE_SIGNATURE,
        );
    }
}
#[repr(C, align(8))]
pub struct AConstraintsActor {
    __padding_end: [u8; 1144],
}
impl AConstraintsActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AConstraintsActor")
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
pub struct UTickableConstraint {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub active: bool,
    __padding_end: [u8; 103],
}
impl UTickableConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableConstraint")
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
pub struct UConstraintsManager {
    __padding_end: [u8; 80],
}
impl UConstraintsManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsManager")
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
pub struct UConstraintsScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UConstraintsScriptingLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsScriptingLibrary")
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
    pub fn remove_this_constraint(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_tickable_constraint: UPtr<UTickableConstraint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_THIS_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_tickable_constraint,
                __buffer.add(8).cast::<UPtr<UTickableConstraint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_THIS_CONSTRAINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_constraint(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_REMOVE_CONSTRAINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_constraints_array(
        in_world: UPtr<crate::bindings::engine::UWorld>,
    ) -> TArray<UPtr<UTickableConstraint>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_GET_CONSTRAINTS_ARRAY,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_GET_CONSTRAINTS_ARRAY,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<UTickableConstraint>>>().read() }
    }
    pub fn create_transformable_handle(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_attachment_name: &FName,
    ) -> UPtr<UTransformableHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_HANDLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_attachment_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_HANDLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UTransformableHandle>>().read() }
    }
    pub fn create_transformable_component_handle(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_scene_component: UPtr<crate::bindings::engine::USceneComponent>,
        in_socket_name: &FName,
    ) -> UPtr<UTransformableComponentHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_COMPONENT_HANDLE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_scene_component,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_socket_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_TRANSFORMABLE_COMPONENT_HANDLE,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UTransformableComponentHandle>>().read() }
    }
    pub fn create_from_type(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_type: crate::bindings::animation_core::ETransformConstraintType,
    ) -> UPtr<UTickableTransformConstraint> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_FROM_TYPE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::animation_core::ETransformConstraintType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_CREATE_FROM_TYPE,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UTickableTransformConstraint>>().read() }
    }
    pub fn add_constraint(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_parent_handle: UPtr<UTransformableHandle>,
        in_child_handle: UPtr<UTransformableHandle>,
        in_constraint: UPtr<UTickableTransformConstraint>,
        b_maintain_offset: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_ADD_CONSTRAINT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_handle,
                __buffer.add(8).cast::<UPtr<UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child_handle,
                __buffer.add(16).cast::<UPtr<UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer.add(24).cast::<UPtr<UTickableTransformConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_offset,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::constraints::UConstraintsScriptingLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::constraints::U_CONSTRAINTS_SCRIPTING_LIBRARY_ADD_CONSTRAINT,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UConstraintSubsystem {
    __padding_end: [u8; 168],
}
impl UConstraintSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintSubsystem")
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
pub struct UTransformableHandle {
    #[doc(hidden)]
    __padding_52: [u8; 52],
    pub constraint_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    __padding_end: [u8; 32],
}
impl UTransformableHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableHandle")
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
pub struct UTransformableComponentHandle {
    #[doc(hidden)]
    __padding_112: [u8; 112],
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
}
impl UTransformableComponentHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableComponentHandle")
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
pub struct UTickableTransformConstraint {
    #[doc(hidden)]
    __padding_152: [u8; 152],
    pub parent_trs_handle: UPtr<UTransformableHandle>,
    pub child_trs_handle: UPtr<UTransformableHandle>,
    pub b_maintain_offset: bool,
    #[doc(hidden)]
    __padding_176: [u8; 7],
    pub b_dynamic_offset: bool,
}
impl UTickableTransformConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTransformConstraint")
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
pub struct UTickableTranslationConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_translation: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableTranslationConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTranslationConstraint")
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
pub struct UTickableRotationConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_rotation: crate::bindings::core_u_object::FQuat,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableRotationConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableRotationConstraint")
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
pub struct UTickableScaleConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_scale: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableScaleConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableScaleConstraint")
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
pub struct UTickableParentConstraint {
    #[doc(hidden)]
    __padding_192: [u8; 192],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub b_scaling: bool,
    pub transform_filter: crate::bindings::animation_core::FTransformFilter,
}
impl UTickableParentConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableParentConstraint")
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
pub struct UTickableLookAtConstraint {
    #[doc(hidden)]
    __padding_184: [u8; 184],
    pub axis: crate::bindings::core_u_object::FVector,
}
impl UTickableLookAtConstraint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableLookAtConstraint")
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
#[repr(C, align(1))]
pub struct FConstraintsManager_OnConstraintAdded_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintsManager_OnConstraintRemoved_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintSubsystem_OnConstraintAddedToSystem_BP {
    _opague: [u8; 1],
}
#[repr(C, align(1))]
pub struct FConstraintSubsystem_OnConstraintRemovedFromSystem_BP {
    _opague: [u8; 1],
}
