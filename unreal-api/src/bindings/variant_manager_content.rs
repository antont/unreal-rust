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
    pub u_level_variant_sets_get_variant_set_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_get_variant_set: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_get_num_variant_sets: *mut crate::ffi::UFunctionOpague,
    pub a_level_variant_sets_actor_switch_on_variant_by_name: *mut crate::ffi::UFunctionOpague,
    pub a_level_variant_sets_actor_switch_on_variant_by_index: *mut crate::ffi::UFunctionOpague,
    pub a_level_variant_sets_actor_set_level_variant_sets: *mut crate::ffi::UFunctionOpague,
    pub a_level_variant_sets_actor_get_level_variant_sets: *mut crate::ffi::UFunctionOpague,
    pub u_property_value_has_recorded_data: *mut crate::ffi::UFunctionOpague,
    pub u_property_value_get_property_tooltip: *mut crate::ffi::UFunctionOpague,
    pub u_property_value_get_full_display_string: *mut crate::ffi::UFunctionOpague,
    pub a_switch_actor_select_option: *mut crate::ffi::UFunctionOpague,
    pub a_switch_actor_get_selected_option: *mut crate::ffi::UFunctionOpague,
    pub a_switch_actor_get_options: *mut crate::ffi::UFunctionOpague,
    pub u_variant_switch_on: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_thumbnail_from_texture: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_thumbnail_from_file: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_thumbnail_from_editor_viewport: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_thumbnail_from_camera: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_display_text: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_is_active: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_thumbnail: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_parent: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_num_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_num_actors: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_display_text: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_dependents: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_get_actor: *mut crate::ffi::UFunctionOpague,
    pub u_variant_delete_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_add_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_set_thumbnail_from_texture: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_set_thumbnail_from_file: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_set_thumbnail_from_editor_viewport: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_set_thumbnail_from_camera: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_set_display_text: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_variant_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_variant: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_thumbnail: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_parent: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_num_variants: *mut crate::ffi::UFunctionOpague,
    pub u_variant_set_get_display_text: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_level_variant_sets_get_variant_set_by_name: std::ptr::null_mut(),
            u_level_variant_sets_get_variant_set: std::ptr::null_mut(),
            u_level_variant_sets_get_num_variant_sets: std::ptr::null_mut(),
            a_level_variant_sets_actor_switch_on_variant_by_name: std::ptr::null_mut(),
            a_level_variant_sets_actor_switch_on_variant_by_index: std::ptr::null_mut(),
            a_level_variant_sets_actor_set_level_variant_sets: std::ptr::null_mut(),
            a_level_variant_sets_actor_get_level_variant_sets: std::ptr::null_mut(),
            u_property_value_has_recorded_data: std::ptr::null_mut(),
            u_property_value_get_property_tooltip: std::ptr::null_mut(),
            u_property_value_get_full_display_string: std::ptr::null_mut(),
            a_switch_actor_select_option: std::ptr::null_mut(),
            a_switch_actor_get_selected_option: std::ptr::null_mut(),
            a_switch_actor_get_options: std::ptr::null_mut(),
            u_variant_switch_on: std::ptr::null_mut(),
            u_variant_set_thumbnail_from_texture: std::ptr::null_mut(),
            u_variant_set_thumbnail_from_file: std::ptr::null_mut(),
            u_variant_set_thumbnail_from_editor_viewport: std::ptr::null_mut(),
            u_variant_set_thumbnail_from_camera: std::ptr::null_mut(),
            u_variant_set_display_text: std::ptr::null_mut(),
            u_variant_set_dependency: std::ptr::null_mut(),
            u_variant_is_active: std::ptr::null_mut(),
            u_variant_get_thumbnail: std::ptr::null_mut(),
            u_variant_get_parent: std::ptr::null_mut(),
            u_variant_get_num_dependencies: std::ptr::null_mut(),
            u_variant_get_num_actors: std::ptr::null_mut(),
            u_variant_get_display_text: std::ptr::null_mut(),
            u_variant_get_dependents: std::ptr::null_mut(),
            u_variant_get_dependency: std::ptr::null_mut(),
            u_variant_get_actor: std::ptr::null_mut(),
            u_variant_delete_dependency: std::ptr::null_mut(),
            u_variant_add_dependency: std::ptr::null_mut(),
            u_variant_set_set_thumbnail_from_texture: std::ptr::null_mut(),
            u_variant_set_set_thumbnail_from_file: std::ptr::null_mut(),
            u_variant_set_set_thumbnail_from_editor_viewport: std::ptr::null_mut(),
            u_variant_set_set_thumbnail_from_camera: std::ptr::null_mut(),
            u_variant_set_set_display_text: std::ptr::null_mut(),
            u_variant_set_get_variant_by_name: std::ptr::null_mut(),
            u_variant_set_get_variant: std::ptr::null_mut(),
            u_variant_set_get_thumbnail: std::ptr::null_mut(),
            u_variant_set_get_parent: std::ptr::null_mut(),
            u_variant_set_get_num_variants: std::ptr::null_mut(),
            u_variant_set_get_display_text: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULevelVariantSets::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariantSetByName"),
                &raw mut __FUNCTION_PTRS.u_level_variant_sets_get_variant_set_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariantSet"),
                &raw mut __FUNCTION_PTRS.u_level_variant_sets_get_variant_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVariantSets"),
                &raw mut __FUNCTION_PTRS.u_level_variant_sets_get_num_variant_sets,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ALevelVariantSetsActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchOnVariantByName"),
                &raw mut __FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchOnVariantByIndex"),
                &raw mut __FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLevelVariantSets"),
                &raw mut __FUNCTION_PTRS
                    .a_level_variant_sets_actor_set_level_variant_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLevelVariantSets"),
                &raw mut __FUNCTION_PTRS
                    .a_level_variant_sets_actor_get_level_variant_sets,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPropertyValue::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasRecordedData"),
                &raw mut __FUNCTION_PTRS.u_property_value_has_recorded_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPropertyTooltip"),
                &raw mut __FUNCTION_PTRS.u_property_value_get_property_tooltip,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFullDisplayString"),
                &raw mut __FUNCTION_PTRS.u_property_value_get_full_display_string,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ASwitchActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectOption"),
                &raw mut __FUNCTION_PTRS.a_switch_actor_select_option,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedOption"),
                &raw mut __FUNCTION_PTRS.a_switch_actor_get_selected_option,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOptions"),
                &raw mut __FUNCTION_PTRS.a_switch_actor_get_options,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVariant::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchOn"),
                &raw mut __FUNCTION_PTRS.u_variant_switch_on,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromTexture"),
                &raw mut __FUNCTION_PTRS.u_variant_set_thumbnail_from_texture,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromFile"),
                &raw mut __FUNCTION_PTRS.u_variant_set_thumbnail_from_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromEditorViewport"),
                &raw mut __FUNCTION_PTRS.u_variant_set_thumbnail_from_editor_viewport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromCamera"),
                &raw mut __FUNCTION_PTRS.u_variant_set_thumbnail_from_camera,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayText"),
                &raw mut __FUNCTION_PTRS.u_variant_set_display_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDependency"),
                &raw mut __FUNCTION_PTRS.u_variant_set_dependency,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActive"),
                &raw mut __FUNCTION_PTRS.u_variant_is_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetThumbnail"),
                &raw mut __FUNCTION_PTRS.u_variant_get_thumbnail,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParent"),
                &raw mut __FUNCTION_PTRS.u_variant_get_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumDependencies"),
                &raw mut __FUNCTION_PTRS.u_variant_get_num_dependencies,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumActors"),
                &raw mut __FUNCTION_PTRS.u_variant_get_num_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayText"),
                &raw mut __FUNCTION_PTRS.u_variant_get_display_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDependents"),
                &raw mut __FUNCTION_PTRS.u_variant_get_dependents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDependency"),
                &raw mut __FUNCTION_PTRS.u_variant_get_dependency,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActor"),
                &raw mut __FUNCTION_PTRS.u_variant_get_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteDependency"),
                &raw mut __FUNCTION_PTRS.u_variant_delete_dependency,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDependency"),
                &raw mut __FUNCTION_PTRS.u_variant_add_dependency,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UVariantSet::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromTexture"),
                &raw mut __FUNCTION_PTRS.u_variant_set_set_thumbnail_from_texture,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromFile"),
                &raw mut __FUNCTION_PTRS.u_variant_set_set_thumbnail_from_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromEditorViewport"),
                &raw mut __FUNCTION_PTRS.u_variant_set_set_thumbnail_from_editor_viewport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetThumbnailFromCamera"),
                &raw mut __FUNCTION_PTRS.u_variant_set_set_thumbnail_from_camera,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayText"),
                &raw mut __FUNCTION_PTRS.u_variant_set_set_display_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariantByName"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_variant_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariant"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetThumbnail"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_thumbnail,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParent"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumVariants"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_num_variants,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayText"),
                &raw mut __FUNCTION_PTRS.u_variant_set_get_display_text,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FVariantDependency {
    pub variant_set: TSoftObjectPtr<UVariantSet>,
    pub variant: TSoftObjectPtr<UVariant>,
    pub b_enabled: bool,
}
impl FVariantDependency {}
#[repr(C, align(8))]
pub struct ULevelVariantSets {
    __padding_end: [u8; 192],
}
impl ULevelVariantSets {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSets")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSets")
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
    pub fn get_variant_set_by_name(
        &mut self,
        variant_set_name: FString,
    ) -> UPtr<UVariantSet> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_variant_set_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_variant_set_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UVariantSet>>().read() }
    }
    pub fn get_variant_set(&mut self, variant_set_index: i32) -> UPtr<UVariantSet> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_variant_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_variant_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UVariantSet>>().read() }
    }
    pub fn get_num_variant_sets(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_num_variant_sets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_level_variant_sets_get_num_variant_sets,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct ALevelVariantSetsActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub level_variant_sets: crate::bindings::core_u_object::FSoftObjectPath,
    __padding_end: [u8; 80],
}
impl ALevelVariantSetsActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALevelVariantSetsActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ALevelVariantSetsActor")
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
    pub fn switch_on_variant_by_name(
        &mut self,
        variant_set_name: FString,
        variant_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn switch_on_variant_by_index(
        &mut self,
        variant_set_index: i32,
        variant_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_set_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_index,
                __buffer.add(4).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_switch_on_variant_by_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_level_variant_sets(&mut self, in_variant_sets: UPtr<ULevelVariantSets>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_set_level_variant_sets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variant_sets,
                __buffer.add(0).cast::<UPtr<ULevelVariantSets>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_set_level_variant_sets,
                __buffer,
            )
        };
    }
    pub fn get_level_variant_sets(&mut self, b_load: bool) -> UPtr<ULevelVariantSets> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_get_level_variant_sets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_load, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_level_variant_sets_actor_get_level_variant_sets,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<ULevelVariantSets>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULevelVariantSetsFunctionDirector {
    __padding_end: [u8; 80],
}
impl ULevelVariantSetsFunctionDirector {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSetsFunctionDirector")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSetsFunctionDirector")
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
pub struct UPropertyValue {
    __padding_end: [u8; 488],
}
impl UPropertyValue {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValue")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValue")
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
    pub fn has_recorded_data(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_has_recorded_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_has_recorded_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_property_tooltip(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_get_property_tooltip,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_get_property_tooltip,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_full_display_string(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_get_full_display_string,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_property_value_get_full_display_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPropertyValueTransform {
    __padding_end: [u8; 488],
}
impl UPropertyValueTransform {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueTransform")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueTransform")
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
pub struct UPropertyValueVisibility {
    __padding_end: [u8; 488],
}
impl UPropertyValueVisibility {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueVisibility")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueVisibility")
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
pub struct UPropertyValueColor {
    __padding_end: [u8; 488],
}
impl UPropertyValueColor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueColor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueColor")
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
pub struct UPropertyValueMaterial {
    __padding_end: [u8; 488],
}
impl UPropertyValueMaterial {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueMaterial")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueMaterial")
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
pub struct UPropertyValueOption {
    __padding_end: [u8; 488],
}
impl UPropertyValueOption {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueOption")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueOption")
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
pub struct UPropertyValueSoftObject {
    __padding_end: [u8; 488],
}
impl UPropertyValueSoftObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueSoftObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyValueSoftObject")
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
pub struct ASwitchActor {
    __padding_end: [u8; 1184],
}
impl ASwitchActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASwitchActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ASwitchActor")
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
    pub fn select_option(&mut self, option_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_select_option,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &option_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_select_option,
                __buffer,
            )
        };
    }
    pub fn get_selected_option(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_get_selected_option,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_get_selected_option,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_options(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_get_options,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .a_switch_actor_get_options,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UVariant {
    __padding_end: [u8; 128],
}
impl UVariant {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariant")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UVariant").copied()
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
    pub fn switch_on(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_switch_on,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_switch_on,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_texture(
        &mut self,
        new_thumbnail: UPtr<crate::bindings::engine::UTexture2D>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_thumbnail,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_texture,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_file(&mut self, file_path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_file,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_editor_viewport(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_editor_viewport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_editor_viewport,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_camera(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        camera_transform: &crate::bindings::core_u_object::FTransform,
        fov_degrees: f32,
        min_z: f32,
        gamma: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<124>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_camera,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fov_degrees,
                __buffer.add(112).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&min_z, __buffer.add(116).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&gamma, __buffer.add(120).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_thumbnail_from_camera,
                __buffer,
            )
        };
    }
    pub fn set_display_text(&mut self, new_display_text: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_display_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_display_text,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_display_text,
                __buffer,
            )
        };
    }
    pub fn is_active(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_is_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_is_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_thumbnail(&mut self) -> UPtr<crate::bindings::engine::UTexture2D> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_thumbnail,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_thumbnail,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>().read()
        }
    }
    pub fn get_parent(&mut self) -> UPtr<UVariantSet> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_parent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_parent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UVariantSet>>().read() }
    }
    pub fn get_num_dependencies(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_num_dependencies,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_num_dependencies,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_actors(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_num_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_num_actors,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_display_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_display_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_display_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_dependents(
        &mut self,
        level_variant_sets: UPtr<ULevelVariantSets>,
        b_only_enabled_dependencies: bool,
    ) -> TArray<UPtr<UVariant>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_dependents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_variant_sets,
                __buffer.add(0).cast::<UPtr<ULevelVariantSets>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_enabled_dependencies,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_dependents,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<UVariant>>>().read() }
    }
    pub fn get_dependency(&mut self, index: i32) -> FVariantDependency {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_dependency,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FVariantDependency>().read() }
    }
    pub fn get_actor(
        &mut self,
        actor_index: i32,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_get_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVariantObjectBinding {
    __padding_end: [u8; 160],
}
impl UVariantObjectBinding {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantObjectBinding")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantObjectBinding")
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
pub struct UVariantSet {
    __padding_end: [u8; 112],
}
impl UVariantSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariantSet")
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
    pub fn set_thumbnail_from_texture(
        &mut self,
        new_thumbnail: UPtr<crate::bindings::engine::UTexture2D>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_thumbnail,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_texture,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_file(&mut self, file_path: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &file_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_file,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_editor_viewport(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_editor_viewport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_editor_viewport,
                __buffer,
            )
        };
    }
    pub fn set_thumbnail_from_camera(
        &mut self,
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        camera_transform: &crate::bindings::core_u_object::FTransform,
        fov_degrees: f32,
        min_z: f32,
        gamma: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<124>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_camera,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_context_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fov_degrees,
                __buffer.add(112).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&min_z, __buffer.add(116).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&gamma, __buffer.add(120).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_thumbnail_from_camera,
                __buffer,
            )
        };
    }
    pub fn set_display_text(&mut self, new_display_text: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_display_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_display_text,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_set_display_text,
                __buffer,
            )
        };
    }
    pub fn get_variant_by_name(&mut self, variant_name: FString) -> UPtr<UVariant> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_variant_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_variant_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UVariant>>().read() }
    }
    pub fn get_variant(&mut self, variant_index: i32) -> UPtr<UVariant> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UVariant>>().read() }
    }
    pub fn get_thumbnail(&mut self) -> UPtr<crate::bindings::engine::UTexture2D> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_thumbnail,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_thumbnail,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture2D>>().read()
        }
    }
    pub fn get_parent(&mut self) -> UPtr<ULevelVariantSets> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_parent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_parent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<ULevelVariantSets>>().read() }
    }
    pub fn get_num_variants(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_num_variants,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_num_variants,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_display_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_display_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::variant_manager_content::__FUNCTION_PTRS
                    .u_variant_set_get_display_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
}
#[repr(transparent)]
pub struct EPropertyValueCategory(pub u8);
impl EPropertyValueCategory {
    pub const UNDEFINED: EPropertyValueCategory = EPropertyValueCategory(0);
    pub const GENERIC: EPropertyValueCategory = EPropertyValueCategory(1);
    pub const RELATIVE_LOCATION: EPropertyValueCategory = EPropertyValueCategory(2);
    pub const RELATIVE_ROTATION: EPropertyValueCategory = EPropertyValueCategory(4);
    pub const RELATIVE_SCALE3_D: EPropertyValueCategory = EPropertyValueCategory(8);
    pub const VISIBILITY: EPropertyValueCategory = EPropertyValueCategory(16);
    pub const MATERIAL: EPropertyValueCategory = EPropertyValueCategory(32);
    pub const COLOR: EPropertyValueCategory = EPropertyValueCategory(64);
    pub const OPTION: EPropertyValueCategory = EPropertyValueCategory(128);
}
