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
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USharedImageConstRefBlueprintFns::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidth"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSize"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelValue"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelLinearColor"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHeight"),
            &raw mut U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT,
        );
    }
}
#[repr(C, align(8))]
pub struct FSharedImageConstRefBlueprint {
    __padding_end: [u8; 8],
}
impl FSharedImageConstRefBlueprint {}
#[repr(C, align(8))]
pub struct USharedImageConstRefBlueprintFns {
    __padding_end: [u8; 48],
}
impl USharedImageConstRefBlueprintFns {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USharedImageConstRefBlueprintFns")
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
    pub fn is_valid(image: &FSharedImageConstRefBlueprint) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_IS_VALID,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_width(image: &FSharedImageConstRefBlueprint) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_WIDTH,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_size(
        image: &FSharedImageConstRefBlueprint,
    ) -> crate::bindings::core_u_object::FVector2f {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_SIZE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2f>().read()
        }
    }
    pub fn get_pixel_value(
        image: &FSharedImageConstRefBlueprint,
        x: i32,
        y: i32,
        b_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector4f {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_VALUE,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_valid);
        }
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4f>().read()
        }
    }
    pub fn get_pixel_linear_color(
        image: &FSharedImageConstRefBlueprint,
        x: i32,
        y: i32,
        b_valid: &mut bool,
        failure_color: crate::bindings::core_u_object::FLinearColor,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&y, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &failure_color,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_PIXEL_LINEAR_COLOR,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_valid);
        }
        unsafe {
            __buffer
                .add(36)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_height(image: &FSharedImageConstRefBlueprint) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                image,
                __buffer.add(0).cast::<FSharedImageConstRefBlueprint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::image_core::USharedImageConstRefBlueprintFns::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::image_core::U_SHARED_IMAGE_CONST_REF_BLUEPRINT_FNS_GET_HEIGHT,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
