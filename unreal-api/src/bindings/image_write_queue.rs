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
pub static mut U_IMAGE_WRITE_BLUEPRINT_LIBRARY_EXPORT_TO_DISK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UImageWriteBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportToDisk"),
            &raw mut U_IMAGE_WRITE_BLUEPRINT_LIBRARY_EXPORT_TO_DISK,
        );
    }
}
#[repr(C, align(16))]
pub struct FImageWriteOptions {
    pub format: EDesiredImageFormat,
    pub on_complete: FImageWriteOptions_OnComplete,
    pub compression_quality: i32,
    pub b_overwrite_file: bool,
    pub b_async: bool,
    __padding_end: [u8; 50],
}
impl FImageWriteOptions {}
#[repr(C, align(8))]
pub struct UImageWriteBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UImageWriteBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageWriteBlueprintLibrary")
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
    pub fn export_to_disk(
        texture: UPtr<crate::bindings::engine::UTexture>,
        filename: FString,
        options: &FImageWriteOptions,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_write_queue::U_IMAGE_WRITE_BLUEPRINT_LIBRARY_EXPORT_TO_DISK,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(32).cast::<FImageWriteOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_write_queue::UImageWriteBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_write_queue::U_IMAGE_WRITE_BLUEPRINT_LIBRARY_EXPORT_TO_DISK,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FImageWriteOptions_OnComplete {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FExportToDisk_OnComplete {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EDesiredImageFormat(pub u8);
impl EDesiredImageFormat {
    pub const PNG: EDesiredImageFormat = EDesiredImageFormat(0);
    pub const JPG: EDesiredImageFormat = EDesiredImageFormat(1);
    pub const BMP: EDesiredImageFormat = EDesiredImageFormat(2);
    pub const EXR: EDesiredImageFormat = EDesiredImageFormat(3);
}
