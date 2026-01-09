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
    pub u_variant_manager_blueprint_library_set_value_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_vector: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_string: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_quat: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_object: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_int_point: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_int: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_float: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_color: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_value_bool: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_set_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_variant_set_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_variant_set: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_variant_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_variant: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_captured_property_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_captured_property: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_actor_binding_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_remove_actor_binding: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_record: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_vector: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_string: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_quat: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_object: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_int_point: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_int: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_float: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_color: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_value_bool: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_property_type_string: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_captured_properties: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_get_capturable_properties: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_delete_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_create_level_variant_sets_asset: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_create_level_variant_sets_actor: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_capture_property: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_apply: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_add_variant_set: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_add_variant: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_add_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_manager_blueprint_library_add_actor_binding: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_variant_manager_blueprint_library_set_value_vector4: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_vector2_d: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_vector: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_string: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_rotator: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_quat: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_object: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_linear_color: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_int_point: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_int: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_float: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_color: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_value_bool: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_set_dependency: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_variant_set_by_name: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_variant_set: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_variant_by_name: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_variant: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_captured_property_by_name: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_captured_property: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_actor_binding_by_name: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_remove_actor_binding: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_record: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_vector4: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_vector2_d: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_vector: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_string: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_rotator: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_quat: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_object: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_linear_color: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_int_point: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_int: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_float: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_color: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_value_bool: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_property_type_string: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_captured_properties: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_get_capturable_properties: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_delete_dependency: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_create_level_variant_sets_asset: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_create_level_variant_sets_actor: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_capture_property: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_apply: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_add_variant_set: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_add_variant: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_add_dependency: std::ptr::null_mut(),
            u_variant_manager_blueprint_library_add_actor_binding: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UVariantManagerBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueVector4"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_set_value_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueVector2D"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_set_value_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueVector"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueString"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueRotator"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_set_value_rotator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueQuat"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueObject"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueLinearColor"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_set_value_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueIntPoint"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_set_value_int_point,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueInt"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueFloat"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueColor"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueBool"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_value_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDependency"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_set_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVariantSetByName"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_variant_set_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVariantSet"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_variant_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVariantByName"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_variant_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVariant"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_remove_variant,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCapturedPropertyByName"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_captured_property_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCapturedProperty"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_captured_property,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorBindingByName"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_actor_binding_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorBinding"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_remove_actor_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Record"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_record,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueVector4"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_value_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueVector2D"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_value_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueVector"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueString"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueRotator"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_value_rotator,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueQuat"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueObject"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueLinearColor"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_value_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueIntPoint"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_value_int_point,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueInt"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueFloat"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueColor"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValueBool"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_get_value_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyTypeString"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_property_type_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCapturedProperties"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_captured_properties,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCapturableProperties"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_get_capturable_properties,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteDependency"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_delete_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLevelVariantSetsAsset"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_create_level_variant_sets_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLevelVariantSetsActor"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_create_level_variant_sets_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureProperty"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_capture_property,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Apply"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_apply,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariantSet"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_add_variant_set,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVariant"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_add_variant,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddDependency"),
            &raw mut __FUNCTION_PTRS.u_variant_manager_blueprint_library_add_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorBinding"),
            &raw mut __FUNCTION_PTRS
                .u_variant_manager_blueprint_library_add_actor_binding,
        );
    }
}
#[repr(C, align(8))]
pub struct FCapturableProperty {
    pub display_name: FString,
    pub(crate) __padding_end: [u8; 56],
}
impl FCapturableProperty {}
#[repr(C, align(8))]
pub struct UPropertyTemplateObject {
    __padding_end: [u8; 248],
}
impl UPropertyTemplateObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyTemplateObject")
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
pub struct UVariantManagerBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UVariantManagerBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantManagerBlueprintLibrary")
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
    pub fn set_value_vector4(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector4,
                __buffer,
            )
        };
    }
    pub fn set_value_vector2_d(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector2_d,
                __buffer,
            )
        };
    }
    pub fn set_value_vector(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_vector,
                __buffer,
            )
        };
    }
    pub fn set_value_string(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_string,
                __buffer,
            )
        };
    }
    pub fn set_value_rotator(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_rotator,
                __buffer,
            )
        };
    }
    pub fn set_value_quat(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_quat,
                __buffer,
            )
        };
    }
    pub fn set_value_object(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_object,
                __buffer,
            )
        };
    }
    pub fn set_value_linear_color(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_linear_color,
                __buffer,
            )
        };
    }
    pub fn set_value_int_point(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FIntPoint,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_int_point,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FIntPoint>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_int_point,
                __buffer,
            )
        };
    }
    pub fn set_value_int(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_int,
                __buffer,
            )
        };
    }
    pub fn set_value_float(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_float,
                __buffer,
            )
        };
    }
    pub fn set_value_color(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: crate::bindings::core_u_object::FColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_color,
                __buffer,
            )
        };
    }
    pub fn set_value_bool(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
        in_value: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_value_bool,
                __buffer,
            )
        };
    }
    pub fn set_dependency(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        index: i32,
        dependency: &mut crate::bindings::variant_manager_content::FVariantDependency,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                dependency,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::variant_manager_content::FVariantDependency,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_set_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::variant_manager_content::FVariantDependency>()
                .swap(dependency);
        }
    }
    pub fn remove_variant_set_by_name(
        level_variant_sets: UPtr<
            crate::bindings::variant_manager_content::ULevelVariantSets,
        >,
        variant_set_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_set_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_variant_sets,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::ULevelVariantSets>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_set_by_name,
                __buffer,
            )
        };
    }
    pub fn remove_variant_set(
        level_variant_sets: UPtr<
            crate::bindings::variant_manager_content::ULevelVariantSets,
        >,
        variant_set: UPtr<crate::bindings::variant_manager_content::UVariantSet>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_variant_sets,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::ULevelVariantSets>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UVariantSet>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_set,
                __buffer,
            )
        };
    }
    pub fn remove_variant_by_name(
        variant_set: UPtr<crate::bindings::variant_manager_content::UVariantSet>,
        variant_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UVariantSet>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant_by_name,
                __buffer,
            )
        };
    }
    pub fn remove_variant(
        variant_set: UPtr<crate::bindings::variant_manager_content::UVariantSet>,
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UVariantSet>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_variant,
                __buffer,
            )
        };
    }
    pub fn remove_captured_property_by_name(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
        property_path: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_captured_property_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_captured_property_by_name,
                __buffer,
            )
        };
    }
    pub fn remove_captured_property(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_captured_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_captured_property,
                __buffer,
            )
        };
    }
    pub fn remove_actor_binding_by_name(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_actor_binding_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_actor_binding_by_name,
                __buffer,
            )
        };
    }
    pub fn remove_actor_binding(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_actor_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_remove_actor_binding,
                __buffer,
            )
        };
    }
    pub fn record(
        prop_val: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_record,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prop_val,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_record,
                __buffer,
            )
        };
    }
    pub fn get_value_vector4(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector4,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_value_vector2_d(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_value_vector(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_value_string(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_value_rotator(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_value_quat(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_value_object(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_value_linear_color(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_linear_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_value_int_point(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FIntPoint {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_int_point,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_int_point,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FIntPoint>().read()
        }
    }
    pub fn get_value_int(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_value_float(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_value_color(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> crate::bindings::core_u_object::FColor {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>().read()
        }
    }
    pub fn get_value_bool(
        property: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_value_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_property_type_string(
        prop_val: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_property_type_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prop_val,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_property_type_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_captured_properties(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> TArray<UPtr<crate::bindings::variant_manager_content::UPropertyValue>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_captured_properties,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_captured_properties,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >,
                >()
                .read()
        }
    }
    pub fn get_capturable_properties(
        actor_or_class: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_capturable_properties,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_or_class,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_get_capturable_properties,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FString>>().read() }
    }
    pub fn delete_dependency(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_delete_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_delete_dependency,
                __buffer,
            )
        };
    }
    pub fn create_level_variant_sets_asset(
        asset_name: FString,
        asset_path: FString,
    ) -> UPtr<crate::bindings::variant_manager_content::ULevelVariantSets> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_create_level_variant_sets_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_create_level_variant_sets_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<
                    UPtr<crate::bindings::variant_manager_content::ULevelVariantSets>,
                >()
                .read()
        }
    }
    pub fn create_level_variant_sets_actor(
        level_variant_sets_asset: UPtr<
            crate::bindings::variant_manager_content::ULevelVariantSets,
        >,
    ) -> UPtr<crate::bindings::variant_manager_content::ALevelVariantSetsActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_create_level_variant_sets_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_variant_sets_asset,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::ULevelVariantSets>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_create_level_variant_sets_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<
                        crate::bindings::variant_manager_content::ALevelVariantSetsActor,
                    >,
                >()
                .read()
        }
    }
    pub fn capture_property(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
        property_path: FString,
    ) -> UPtr<crate::bindings::variant_manager_content::UPropertyValue> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_capture_property,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_capture_property,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::variant_manager_content::UPropertyValue>>()
                .read()
        }
    }
    pub fn apply(
        prop_val: UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_apply,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prop_val,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UPropertyValue>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_apply,
                __buffer,
            )
        };
    }
    pub fn add_variant_set(
        level_variant_sets: UPtr<
            crate::bindings::variant_manager_content::ULevelVariantSets,
        >,
        variant_set: UPtr<crate::bindings::variant_manager_content::UVariantSet>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_variant_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_variant_sets,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::ULevelVariantSets>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UVariantSet>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_variant_set,
                __buffer,
            )
        };
    }
    pub fn add_variant(
        variant_set: UPtr<crate::bindings::variant_manager_content::UVariantSet>,
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::variant_manager_content::UVariantSet>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_variant,
                __buffer,
            )
        };
    }
    pub fn add_dependency(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        dependency: &mut crate::bindings::variant_manager_content::FVariantDependency,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                dependency,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::variant_manager_content::FVariantDependency,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::variant_manager_content::FVariantDependency>()
                .swap(dependency);
        }
        unsafe { __buffer.add(112).cast::<i32>().read() }
    }
    pub fn add_actor_binding(
        variant: UPtr<crate::bindings::variant_manager_content::UVariant>,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_actor_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::variant_manager_content::UVariant>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::variant_manager::UVariantManagerBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager::__FUNCTION_PTRS
                    .u_variant_manager_blueprint_library_add_actor_binding,
                __buffer,
            )
        };
    }
}
