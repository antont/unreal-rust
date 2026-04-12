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
    pub udna_mesh_vertex_color_data_asset_get_color_by_mesh_and_index: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            udna_mesh_vertex_color_data_asset_get_color_by_mesh_and_index: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDNAMeshVertexColorDataAsset::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetColorByMeshAndIndex"),
                &raw mut __FUNCTION_PTRS
                    .udna_mesh_vertex_color_data_asset_get_color_by_mesh_and_index,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMeshVertexColorData {
    pub mesh_name: FString,
    pub colors: TArray<crate::bindings::core_u_object::FLinearColor>,
}
impl FMeshVertexColorData {}
#[repr(C, align(8))]
pub struct UDNAMeshVertexColorDataAsset {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub mesh_color_entries: TArray<FMeshVertexColorData>,
}
impl UDNAMeshVertexColorDataAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAMeshVertexColorDataAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAMeshVertexColorDataAsset")
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
    pub fn get_color_by_mesh_and_index(
        &self,
        in_mesh_name: FString,
        in_vertex_id: i32,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_dna::__FUNCTION_PTRS
                    .udna_mesh_vertex_color_data_asset_get_color_by_mesh_and_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mesh_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_vertex_id,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_dna::__FUNCTION_PTRS
                    .udna_mesh_vertex_color_data_asset_get_color_by_mesh_and_index,
                __buffer,
            )
        };
        std::mem::forget(in_mesh_name);
        std::mem::forget(in_vertex_id);
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMetaHumanInterchangeDnaTranslator {
    __padding_end: [u8; 184],
}
impl UMetaHumanInterchangeDnaTranslator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanInterchangeDnaTranslator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaHumanInterchangeDnaTranslator")
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
