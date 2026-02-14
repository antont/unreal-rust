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
    pub u_custom_mesh_component_set_custom_mesh_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_custom_mesh_component_clear_custom_mesh_triangles: *mut crate::ffi::UFunctionOpague,
    pub u_custom_mesh_component_add_custom_mesh_triangles: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_custom_mesh_component_set_custom_mesh_triangles: std::ptr::null_mut(),
            u_custom_mesh_component_clear_custom_mesh_triangles: std::ptr::null_mut(),
            u_custom_mesh_component_add_custom_mesh_triangles: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCustomMeshComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomMeshTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_custom_mesh_component_set_custom_mesh_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearCustomMeshTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_custom_mesh_component_clear_custom_mesh_triangles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddCustomMeshTriangles"),
                &raw mut __FUNCTION_PTRS
                    .u_custom_mesh_component_add_custom_mesh_triangles,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FCustomMeshTriangle {
    pub vertex0: crate::bindings::core_u_object::FVector,
    pub vertex1: crate::bindings::core_u_object::FVector,
    pub vertex2: crate::bindings::core_u_object::FVector,
}
impl FCustomMeshTriangle {}
#[repr(C, align(16))]
pub struct UCustomMeshComponent {
    __padding_end: [u8; 1600],
}
impl UCustomMeshComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomMeshComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomMeshComponent")
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
    pub fn set_custom_mesh_triangles(
        &mut self,
        triangles: &TArray<FCustomMeshTriangle>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_set_custom_mesh_triangles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(0).cast::<TArray<FCustomMeshTriangle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_set_custom_mesh_triangles,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn clear_custom_mesh_triangles(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_clear_custom_mesh_triangles,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_clear_custom_mesh_triangles,
                __buffer,
            )
        };
    }
    pub fn add_custom_mesh_triangles(
        &mut self,
        triangles: &TArray<FCustomMeshTriangle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_add_custom_mesh_triangles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                triangles,
                __buffer.add(0).cast::<TArray<FCustomMeshTriangle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::custom_mesh_component::__FUNCTION_PTRS
                    .u_custom_mesh_component_add_custom_mesh_triangles,
                __buffer,
            )
        };
    }
}
