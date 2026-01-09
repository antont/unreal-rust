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
    pub u_shared_image_const_ref_blueprint_fns_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_shared_image_const_ref_blueprint_fns_get_width: *mut crate::ffi::UFunctionOpague,
    pub u_shared_image_const_ref_blueprint_fns_get_size: *mut crate::ffi::UFunctionOpague,
    pub u_shared_image_const_ref_blueprint_fns_get_pixel_value: *mut crate::ffi::UFunctionOpague,
    pub u_shared_image_const_ref_blueprint_fns_get_pixel_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_shared_image_const_ref_blueprint_fns_get_height: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_shared_image_const_ref_blueprint_fns_is_valid: std::ptr::null_mut(),
            u_shared_image_const_ref_blueprint_fns_get_width: std::ptr::null_mut(),
            u_shared_image_const_ref_blueprint_fns_get_size: std::ptr::null_mut(),
            u_shared_image_const_ref_blueprint_fns_get_pixel_value: std::ptr::null_mut(),
            u_shared_image_const_ref_blueprint_fns_get_pixel_linear_color: std::ptr::null_mut(),
            u_shared_image_const_ref_blueprint_fns_get_height: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USharedImageConstRefBlueprintFns::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut __FUNCTION_PTRS.u_shared_image_const_ref_blueprint_fns_is_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWidth"),
            &raw mut __FUNCTION_PTRS.u_shared_image_const_ref_blueprint_fns_get_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSize"),
            &raw mut __FUNCTION_PTRS.u_shared_image_const_ref_blueprint_fns_get_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelValue"),
            &raw mut __FUNCTION_PTRS
                .u_shared_image_const_ref_blueprint_fns_get_pixel_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPixelLinearColor"),
            &raw mut __FUNCTION_PTRS
                .u_shared_image_const_ref_blueprint_fns_get_pixel_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetHeight"),
            &raw mut __FUNCTION_PTRS.u_shared_image_const_ref_blueprint_fns_get_height,
        );
    }
}
#[repr(C, align(8))]
pub struct FSharedImageConstRefBlueprint {
    pub(crate) __padding_end: [u8; 8],
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_is_valid,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_is_valid,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_width,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_width,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_size,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_size,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_pixel_value,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_pixel_value,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_pixel_linear_color,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_pixel_linear_color,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_height,
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
                crate::bindings::image_core::__FUNCTION_PTRS
                    .u_shared_image_const_ref_blueprint_fns_get_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
