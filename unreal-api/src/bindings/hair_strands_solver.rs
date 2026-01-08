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
    pub u_groom_solver_component_set_deformer_solver: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_reset_groom_components: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_reset_collision_components: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_remove_groom_component: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_remove_collision_component: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_add_groom_component: *mut crate::ffi::UFunctionOpague,
    pub u_groom_solver_component_add_collision_component: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_groom_solver_component_set_deformer_solver: std::ptr::null_mut(),
            u_groom_solver_component_reset_groom_components: std::ptr::null_mut(),
            u_groom_solver_component_reset_collision_components: std::ptr::null_mut(),
            u_groom_solver_component_remove_groom_component: std::ptr::null_mut(),
            u_groom_solver_component_remove_collision_component: std::ptr::null_mut(),
            u_groom_solver_component_add_groom_component: std::ptr::null_mut(),
            u_groom_solver_component_add_collision_component: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomSolverComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDeformerSolver"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_set_deformer_solver,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetGroomComponents"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_reset_groom_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetCollisionComponents"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_reset_collision_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGroomComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_remove_groom_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCollisionComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_remove_collision_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGroomComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_add_groom_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCollisionComponent"),
            &raw mut __FUNCTION_PTRS.u_groom_solver_component_add_collision_component,
        );
    }
}
#[repr(C, align(8))]
pub struct FDataflowGroomSolverProxy {
    __padding_end: [u8; 344],
}
impl FDataflowGroomSolverProxy {}
#[repr(C, align(16))]
pub struct UGroomSolverComponent {
    __padding_end: [u8; 2304],
}
impl UGroomSolverComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroomSolverComponent")
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
    pub fn set_deformer_solver(
        &mut self,
        deformer_solver: UPtr<crate::bindings::engine::UMeshDeformer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_set_deformer_solver,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &deformer_solver,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshDeformer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_set_deformer_solver,
                __buffer,
            )
        };
    }
    pub fn reset_groom_components(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_reset_groom_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_reset_groom_components,
                __buffer,
            )
        };
    }
    pub fn reset_collision_components(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_reset_collision_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_reset_collision_components,
                __buffer,
            )
        };
    }
    pub fn remove_groom_component(
        &mut self,
        groom_component: UPtr<crate::bindings::hair_strands_core::UGroomComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_remove_groom_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &groom_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::hair_strands_core::UGroomComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_remove_groom_component,
                __buffer,
            )
        };
    }
    pub fn remove_collision_component(
        &mut self,
        collision_component: UPtr<crate::bindings::engine::UMeshComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_remove_collision_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_remove_collision_component,
                __buffer,
            )
        };
    }
    pub fn add_groom_component(
        &mut self,
        groom_component: UPtr<crate::bindings::hair_strands_core::UGroomComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_add_groom_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &groom_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::hair_strands_core::UGroomComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_add_groom_component,
                __buffer,
            )
        };
    }
    pub fn add_collision_component(
        &mut self,
        collision_component: UPtr<crate::bindings::engine::UMeshComponent>,
        lod_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_add_collision_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&lod_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::hair_strands_solver::__FUNCTION_PTRS
                    .u_groom_solver_component_add_collision_component,
                __buffer,
            )
        };
    }
}
