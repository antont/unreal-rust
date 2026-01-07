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
pub static mut UDNA_ASSET_IMPORT_UI_RESET_TO_DEFAULT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut UDNA_IMPORTER_LIBRARY_IMPORT_SKELETAL_MESH_DNA: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDNAAssetImportUI::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToDefault"),
            &raw mut UDNA_ASSET_IMPORT_UI_RESET_TO_DEFAULT,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UDNAImporterLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportSkeletalMeshDNA"),
            &raw mut UDNA_IMPORTER_LIBRARY_IMPORT_SKELETAL_MESH_DNA,
        );
    }
}
#[repr(C, align(8))]
pub struct UDNAAssetImportUI {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 32],
}
impl UDNAAssetImportUI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAAssetImportUI")
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
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_logic_editor::UDNA_ASSET_IMPORT_UI_RESET_TO_DEFAULT,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_logic_editor::UDNA_ASSET_IMPORT_UI_RESET_TO_DEFAULT,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UDNAAssetImportFactory {
    __padding_end: [u8; 184],
}
impl UDNAAssetImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAAssetImportFactory")
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
pub struct UDNAImporterLibrary {
    __padding_end: [u8; 48],
}
impl UDNAImporterLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDNAImporterLibrary")
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
    pub fn import_skeletal_mesh_dna(
        file_name: FString,
        mesh: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_logic_editor::UDNA_IMPORTER_LIBRARY_IMPORT_SKELETAL_MESH_DNA,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_logic_editor::UDNAImporterLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_logic_editor::UDNA_IMPORTER_LIBRARY_IMPORT_SKELETAL_MESH_DNA,
                __buffer,
            )
        };
    }
}
