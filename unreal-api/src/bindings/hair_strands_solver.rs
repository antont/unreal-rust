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
pub static mut U_GROOM_SOLVER_COMPONENT_SET_DEFORMER_SOLVER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_RESET_GROOM_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_RESET_COLLISION_COMPONENTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_REMOVE_GROOM_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_REMOVE_COLLISION_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_ADD_GROOM_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_GROOM_SOLVER_COMPONENT_ADD_COLLISION_COMPONENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UGroomSolverComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDeformerSolver"),
            &raw mut U_GROOM_SOLVER_COMPONENT_SET_DEFORMER_SOLVER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetGroomComponents"),
            &raw mut U_GROOM_SOLVER_COMPONENT_RESET_GROOM_COMPONENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetCollisionComponents"),
            &raw mut U_GROOM_SOLVER_COMPONENT_RESET_COLLISION_COMPONENTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGroomComponent"),
            &raw mut U_GROOM_SOLVER_COMPONENT_REMOVE_GROOM_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCollisionComponent"),
            &raw mut U_GROOM_SOLVER_COMPONENT_REMOVE_COLLISION_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGroomComponent"),
            &raw mut U_GROOM_SOLVER_COMPONENT_ADD_GROOM_COMPONENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCollisionComponent"),
            &raw mut U_GROOM_SOLVER_COMPONENT_ADD_COLLISION_COMPONENT,
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
}
