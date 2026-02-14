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
    pub u_constraints_manager_on_constraint_removed_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_manager_on_constraint_added_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_remove_this_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_remove_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_get_constraints_array: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_create_transformable_handle: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_create_transformable_component_handle: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_create_from_type: *mut crate::ffi::UFunctionOpague,
    pub u_constraints_scripting_library_add_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_constraint_subsystem_on_constraint_removed_from_system_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_constraint_subsystem_on_constraint_added_to_system_delegate_signature: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_constraints_manager_on_constraint_removed_delegate_signature: std::ptr::null_mut(),
            u_constraints_manager_on_constraint_added_delegate_signature: std::ptr::null_mut(),
            u_constraints_scripting_library_remove_this_constraint: std::ptr::null_mut(),
            u_constraints_scripting_library_remove_constraint: std::ptr::null_mut(),
            u_constraints_scripting_library_get_constraints_array: std::ptr::null_mut(),
            u_constraints_scripting_library_create_transformable_handle: std::ptr::null_mut(),
            u_constraints_scripting_library_create_transformable_component_handle: std::ptr::null_mut(),
            u_constraints_scripting_library_create_from_type: std::ptr::null_mut(),
            u_constraints_scripting_library_add_constraint: std::ptr::null_mut(),
            u_constraint_subsystem_on_constraint_removed_from_system_delegate_signature: std::ptr::null_mut(),
            u_constraint_subsystem_on_constraint_added_to_system_delegate_signature: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UConstraintsManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnConstraintRemoved__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_manager_on_constraint_removed_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnConstraintAdded__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_manager_on_constraint_added_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UConstraintsScriptingLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveThisConstraint"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_this_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveConstraint"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConstraintsArray"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_scripting_library_get_constraints_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTransformableHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTransformableComponentHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_component_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateFromType"),
                &raw mut __FUNCTION_PTRS.u_constraints_scripting_library_create_from_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddConstraint"),
                &raw mut __FUNCTION_PTRS.u_constraints_scripting_library_add_constraint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UConstraintSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnConstraintRemovedFromSystem__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_constraint_subsystem_on_constraint_removed_from_system_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "OnConstraintAddedToSystem__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_constraint_subsystem_on_constraint_added_to_system_delegate_signature,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct AConstraintsActor {
    __padding_end: [u8; 1144],
}
impl AConstraintsActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AConstraintsActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AConstraintsActor")
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
pub struct UTickableConstraint {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub active: bool,
    __padding_end: [u8; 103],
}
impl UTickableConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableConstraint")
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
pub struct UConstraintsManager {
    __padding_end: [u8; 80],
}
impl UConstraintsManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsManager")
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
pub struct UConstraintsScriptingLibrary {
    __padding_end: [u8; 48],
}
impl UConstraintsScriptingLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsScriptingLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintsScriptingLibrary")
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_this_constraint,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_this_constraint,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_constraint,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_remove_constraint,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_get_constraints_array,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_get_constraints_array,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_handle,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_handle,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_component_handle,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_transformable_component_handle,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_from_type,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_create_from_type,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_add_constraint,
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
                crate::bindings::constraints::__FUNCTION_PTRS
                    .u_constraints_scripting_library_add_constraint,
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
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintSubsystem")
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
pub struct UTransformableHandle {
    #[doc(hidden)]
    pub(crate) __padding_52: [u8; 52],
    pub constraint_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    __padding_end: [u8; 32],
}
impl UTransformableHandle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableHandle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableHandle")
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
pub struct UTransformableComponentHandle {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub socket_name: FName,
}
impl UTransformableComponentHandle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableComponentHandle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableComponentHandle")
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
pub struct UTickableTransformConstraint {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub parent_trs_handle: UPtr<UTransformableHandle>,
    pub child_trs_handle: UPtr<UTransformableHandle>,
    pub b_maintain_offset: bool,
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 7],
    pub b_dynamic_offset: bool,
}
impl UTickableTransformConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTransformConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTransformConstraint")
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
pub struct UTickableTranslationConstraint {
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 192],
    pub offset_translation: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableTranslationConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTranslationConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableTranslationConstraint")
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
pub struct UTickableRotationConstraint {
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 192],
    pub offset_rotation: crate::bindings::core_u_object::FQuat,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableRotationConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableRotationConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableRotationConstraint")
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
pub struct UTickableScaleConstraint {
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 192],
    pub offset_scale: crate::bindings::core_u_object::FVector,
    pub axis_filter: crate::bindings::animation_core::FFilterOptionPerAxis,
}
impl UTickableScaleConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableScaleConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableScaleConstraint")
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
pub struct UTickableParentConstraint {
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 192],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub b_scaling: bool,
    pub transform_filter: crate::bindings::animation_core::FTransformFilter,
}
impl UTickableParentConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableParentConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableParentConstraint")
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
pub struct UTickableLookAtConstraint {
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 184],
    pub axis: crate::bindings::core_u_object::FVector,
}
impl UTickableLookAtConstraint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableLookAtConstraint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableLookAtConstraint")
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
