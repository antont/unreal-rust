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
    pub u_typed_element_selection_set_library_set_selection_from_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_library_select_elements_from_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_library_get_normalized_selection: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_library_get_normalized_element_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_library_deselect_elements_from_list: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_set_selection: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_select_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_select_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_restore_selection_state: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_on_pre_change_dynamic_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_on_change_dynamic_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_k2_get_selected_element_handles: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_is_element_selected: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_has_selected_objects: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_has_selected_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_top_selected_object: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_selection_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_selected_objects: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_num_selected_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_current_selection_state: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_get_bottom_selected_object: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_deselect_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_deselect_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_count_selected_objects: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_count_selected_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_clear_selection: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_can_select_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_can_deselect_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_set_allow_selection_modifiers: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_asset_data_interface_get_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_asset_data_interface_get_all_referenced_asset_datas: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_hierarchy_interface_get_parent_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_hierarchy_interface_get_child_elements: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_object_interface_get_object_class: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_object_interface_get_object: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_primitive_custom_data_interface_set_custom_data_value: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_primitive_custom_data_interface_set_custom_data: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_select_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_is_element_selected: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_get_selection_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_deselect_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_can_select_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_can_deselect_element: *mut crate::ffi::UFunctionOpague,
    pub u_typed_element_selection_interface_allow_selection_modifiers: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_typed_element_selection_set_library_set_selection_from_list: std::ptr::null_mut(),
            u_typed_element_selection_set_library_select_elements_from_list: std::ptr::null_mut(),
            u_typed_element_selection_set_library_get_normalized_selection: std::ptr::null_mut(),
            u_typed_element_selection_set_library_get_normalized_element_list: std::ptr::null_mut(),
            u_typed_element_selection_set_library_deselect_elements_from_list: std::ptr::null_mut(),
            u_typed_element_selection_set_set_selection: std::ptr::null_mut(),
            u_typed_element_selection_set_select_elements: std::ptr::null_mut(),
            u_typed_element_selection_set_select_element: std::ptr::null_mut(),
            u_typed_element_selection_set_restore_selection_state: std::ptr::null_mut(),
            u_typed_element_selection_set_on_pre_change_dynamic_delegate_signature: std::ptr::null_mut(),
            u_typed_element_selection_set_on_change_dynamic_delegate_signature: std::ptr::null_mut(),
            u_typed_element_selection_set_k2_get_selected_element_handles: std::ptr::null_mut(),
            u_typed_element_selection_set_is_element_selected: std::ptr::null_mut(),
            u_typed_element_selection_set_has_selected_objects: std::ptr::null_mut(),
            u_typed_element_selection_set_has_selected_elements: std::ptr::null_mut(),
            u_typed_element_selection_set_get_top_selected_object: std::ptr::null_mut(),
            u_typed_element_selection_set_get_selection_element: std::ptr::null_mut(),
            u_typed_element_selection_set_get_selected_objects: std::ptr::null_mut(),
            u_typed_element_selection_set_get_num_selected_elements: std::ptr::null_mut(),
            u_typed_element_selection_set_get_current_selection_state: std::ptr::null_mut(),
            u_typed_element_selection_set_get_bottom_selected_object: std::ptr::null_mut(),
            u_typed_element_selection_set_deselect_elements: std::ptr::null_mut(),
            u_typed_element_selection_set_deselect_element: std::ptr::null_mut(),
            u_typed_element_selection_set_count_selected_objects: std::ptr::null_mut(),
            u_typed_element_selection_set_count_selected_elements: std::ptr::null_mut(),
            u_typed_element_selection_set_clear_selection: std::ptr::null_mut(),
            u_typed_element_selection_set_can_select_element: std::ptr::null_mut(),
            u_typed_element_selection_set_can_deselect_element: std::ptr::null_mut(),
            u_typed_element_selection_set_allow_selection_modifiers: std::ptr::null_mut(),
            u_typed_element_asset_data_interface_get_asset_data: std::ptr::null_mut(),
            u_typed_element_asset_data_interface_get_all_referenced_asset_datas: std::ptr::null_mut(),
            u_typed_element_hierarchy_interface_get_parent_element: std::ptr::null_mut(),
            u_typed_element_hierarchy_interface_get_child_elements: std::ptr::null_mut(),
            u_typed_element_object_interface_get_object_class: std::ptr::null_mut(),
            u_typed_element_object_interface_get_object: std::ptr::null_mut(),
            u_typed_element_primitive_custom_data_interface_set_custom_data_value: std::ptr::null_mut(),
            u_typed_element_primitive_custom_data_interface_set_custom_data: std::ptr::null_mut(),
            u_typed_element_selection_interface_select_element: std::ptr::null_mut(),
            u_typed_element_selection_interface_is_element_selected: std::ptr::null_mut(),
            u_typed_element_selection_interface_get_selection_element: std::ptr::null_mut(),
            u_typed_element_selection_interface_deselect_element: std::ptr::null_mut(),
            u_typed_element_selection_interface_can_select_element: std::ptr::null_mut(),
            u_typed_element_selection_interface_can_deselect_element: std::ptr::null_mut(),
            u_typed_element_selection_interface_allow_selection_modifiers: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementSelectionSetLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelectionFromList"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_library_set_selection_from_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectElementsFromList"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_library_select_elements_from_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNormalizedSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNormalizedElementList"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_element_list,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectElementsFromList"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_library_deselect_elements_from_list,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementSelectionSet::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelection"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_set_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectElements"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_select_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectElement"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_select_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestoreSelectionState"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_restore_selection_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPreChangeDynamic__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_on_pre_change_dynamic_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnChangeDynamic__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_on_change_dynamic_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetSelectedElementHandles"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_k2_get_selected_element_handles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsElementSelected"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_is_element_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasSelectedObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasSelectedElements"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTopSelectedObject"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_top_selected_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selection_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selected_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumSelectedElements"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_num_selected_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentSelectionState"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_current_selection_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBottomSelectedObject"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_get_bottom_selected_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectElements"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_deselect_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectElement"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_deselect_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CountSelectedObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CountSelectedElements"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearSelection"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_clear_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanSelectElement"),
                &raw mut __FUNCTION_PTRS.u_typed_element_selection_set_can_select_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanDeselectElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_can_deselect_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AllowSelectionModifiers"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_set_allow_selection_modifiers,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementAssetDataInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetData"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_asset_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllReferencedAssetDatas"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_all_referenced_asset_datas,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementHierarchyInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_parent_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChildElements"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_child_elements,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementObjectInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectClass"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_object_interface_get_object_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObject"),
                &raw mut __FUNCTION_PTRS.u_typed_element_object_interface_get_object,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementPrimitiveCustomDataInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomDataValue"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomData"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTypedElementSelectionInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_select_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsElementSelected"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_is_element_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_get_selection_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_deselect_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanSelectElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_select_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanDeselectElement"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_deselect_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AllowSelectionModifiers"),
                &raw mut __FUNCTION_PTRS
                    .u_typed_element_selection_interface_allow_selection_modifiers,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FTypedElementSelectionOptions {
    pub b_allow_hidden: bool,
    pub b_allow_groups: bool,
    pub b_allow_legacy_notifications: bool,
    pub b_warn_if_locked: bool,
    pub b_allow_sub_root_selection: bool,
    pub child_element_inclusion_method: ETypedElementChildInclusionMethod,
    pub(crate) __padding_end: [u8; 14],
}
impl FTypedElementSelectionOptions {}
#[repr(C, align(8))]
pub struct FTypedElementSelectionSetState {
    pub(crate) __padding_end: [u8; 24],
}
impl FTypedElementSelectionSetState {}
#[repr(C, align(4))]
pub struct FTypedElementIsSelectedOptions {
    pub b_allow_indirect: bool,
    pub(crate) __padding_end: [u8; 15],
}
impl FTypedElementIsSelectedOptions {}
#[repr(C, align(4))]
pub struct FTypedElementSelectionNormalizationOptions {
    pub b_expand_groups: bool,
    pub b_follow_attachment: bool,
    pub(crate) __padding_end: [u8; 14],
}
impl FTypedElementSelectionNormalizationOptions {}
#[repr(C, align(1))]
pub struct FTypedElementAssetDataReferencedOptions {
    pub b_only_top_level_asset: bool,
}
impl FTypedElementAssetDataReferencedOptions {}
#[repr(C, align(8))]
pub struct UTypedElementSelectionSetLibrary {
    __padding_end: [u8; 48],
}
impl UTypedElementSelectionSetLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionSetLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionSetLibrary")
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
    pub fn set_selection_from_list(
        selection_set: UPtr<UTypedElementSelectionSet>,
        element_list: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_set_selection_from_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_set,
                __buffer.add(0).cast::<UPtr<UTypedElementSelectionSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_options,
                __buffer.add(24).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_runtime::UTypedElementSelectionSetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_set_selection_from_list,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn select_elements_from_list(
        selection_set: UPtr<UTypedElementSelectionSet>,
        element_list: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_select_elements_from_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_set,
                __buffer.add(0).cast::<UPtr<UTypedElementSelectionSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_options,
                __buffer.add(24).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_runtime::UTypedElementSelectionSetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_select_elements_from_list,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn get_normalized_selection(
        selection_set: UPtr<UTypedElementSelectionSet>,
        normalization_options: FTypedElementSelectionNormalizationOptions,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementListProxy {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_set,
                __buffer.add(0).cast::<UPtr<UTypedElementSelectionSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normalization_options,
                __buffer.add(8).cast::<FTypedElementSelectionNormalizationOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_runtime::UTypedElementSelectionSetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_selection,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                >()
                .read()
        }
    }
    pub fn get_normalized_element_list(
        selection_set: UPtr<UTypedElementSelectionSet>,
        element_list: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        normalization_options: FTypedElementSelectionNormalizationOptions,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementListProxy {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_element_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_set,
                __buffer.add(0).cast::<UPtr<UTypedElementSelectionSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normalization_options,
                __buffer.add(24).cast::<FTypedElementSelectionNormalizationOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_runtime::UTypedElementSelectionSetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_get_normalized_element_list,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                >()
                .read()
        }
    }
    pub fn deselect_elements_from_list(
        selection_set: UPtr<UTypedElementSelectionSet>,
        element_list: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_deselect_elements_from_list,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_set,
                __buffer.add(0).cast::<UPtr<UTypedElementSelectionSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_list,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &selection_options,
                __buffer.add(24).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::typed_element_runtime::UTypedElementSelectionSetLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_library_deselect_elements_from_list,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UTypedElementSelectionSet {
    __padding_end: [u8; 2256],
}
impl UTypedElementSelectionSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionSet")
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
    pub fn set_selection(
        &mut self,
        in_element_handles: &TArray<
            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        >,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_set_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handles,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(16).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_set_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn select_elements(
        &mut self,
        in_element_handles: &TArray<
            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        >,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_select_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handles,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(16).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_select_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn select_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_select_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_select_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn restore_selection_state(
        &mut self,
        in_selection_state: &FTypedElementSelectionSetState,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_restore_selection_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_state,
                __buffer.add(0).cast::<FTypedElementSelectionSetState>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_restore_selection_state,
                __buffer,
            )
        };
    }
    pub fn get_selected_element_handles(
        &self,
        in_base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> TArray<crate::bindings::typed_element_framework::FScriptTypedElementHandle> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_k2_get_selected_element_handles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_interface_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_k2_get_selected_element_handles,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >,
                >()
                .read()
        }
    }
    pub fn is_element_selected(
        &self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: FTypedElementIsSelectedOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_is_element_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(8).cast::<FTypedElementIsSelectedOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_is_element_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn has_selected_objects(
        &self,
        in_required_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_required_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_selected_elements(
        &self,
        in_base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_interface_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_has_selected_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_top_selected_object(
        &self,
        in_required_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_top_selected_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_required_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_top_selected_object,
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
    pub fn get_selection_element(
        &self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_method: ETypedElementSelectionMethod,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selection_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_method,
                __buffer.add(8).cast::<ETypedElementSelectionMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selection_element,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                >()
                .read()
        }
    }
    pub fn get_selected_objects(
        &self,
        in_required_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selected_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_required_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_selected_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_num_selected_elements(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_num_selected_elements,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_num_selected_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_current_selection_state(&self) -> FTypedElementSelectionSetState {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_current_selection_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_current_selection_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FTypedElementSelectionSetState>().read() }
    }
    pub fn get_bottom_selected_object(
        &self,
        in_required_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_bottom_selected_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_required_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_get_bottom_selected_object,
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
    pub fn deselect_elements(
        &mut self,
        in_element_handles: &TArray<
            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        >,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_deselect_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handles,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(16).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_deselect_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn deselect_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_deselect_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_deselect_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn count_selected_objects(
        &self,
        in_required_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_required_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn count_selected_elements(
        &self,
        in_base_interface_type: TSubclassOf<crate::bindings::core_u_object::UInterface>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_base_interface_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_count_selected_elements,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn clear_selection(
        &mut self,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_clear_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(0).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_clear_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn can_select_element(
        &self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_can_select_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_can_select_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn can_deselect_element(
        &self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_can_deselect_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_can_deselect_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn allow_selection_modifiers(
        &self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_allow_selection_modifiers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_set_allow_selection_modifiers,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
pub struct ITypedElementAssetDataInterface {}
#[repr(C, align(8))]
pub struct UTypedElementAssetDataInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementAssetDataInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementAssetDataInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementAssetDataInterface")
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
    pub fn get_asset_data(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_all_referenced_asset_datas(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_all_referenced_asset_datas,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_asset_data_interface_get_all_referenced_asset_datas,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
}
pub struct ITypedElementHierarchyInterface {}
#[repr(C, align(8))]
pub struct UTypedElementHierarchyInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementHierarchyInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementHierarchyInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementHierarchyInterface")
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
    pub fn get_parent_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        b_allow_create: bool,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_parent_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_create,
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
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_parent_element,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                >()
                .read()
        }
    }
    pub fn get_child_elements(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        out_element_handles: &mut TArray<
            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        >,
        b_allow_create: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_child_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_element_handles,
                __buffer
                    .add(8)
                    .cast::<
                        TArray<
                            crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_create,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_hierarchy_interface_get_child_elements,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >,
                >()
                .swap(out_element_handles);
        }
    }
}
pub struct ITypedElementObjectInterface {}
#[repr(C, align(8))]
pub struct UTypedElementObjectInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementObjectInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementObjectInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementObjectInterface")
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
    pub fn get_object_class(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_object_interface_get_object_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_object_interface_get_object_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_object(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_object_interface_get_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_object_interface_get_object,
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
}
pub struct ITypedElementPrimitiveCustomDataInterface {}
#[repr(C, align(8))]
pub struct UTypedElementPrimitiveCustomDataInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementPrimitiveCustomDataInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementPrimitiveCustomDataInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementPrimitiveCustomDataInterface")
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
    pub fn set_custom_data_value(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        custom_data_index: i32,
        custom_data_value: f32,
        b_mark_render_state_dirty: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &custom_data_value,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_render_state_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data_value,
                __buffer,
            )
        };
    }
    pub fn set_custom_data(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        custom_data_floats: &TArray<f32>,
        b_mark_render_state_dirty: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                custom_data_floats,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_render_state_dirty,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_primitive_custom_data_interface_set_custom_data,
                __buffer,
            )
        };
    }
}
pub struct ITypedElementSelectionInterface {}
#[repr(C, align(8))]
pub struct UTypedElementSelectionInterface {
    __padding_end: [u8; 48],
}
impl UTypedElementSelectionInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTypedElementSelectionInterface")
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
    pub fn select_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_set: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        in_selection_options: &FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_select_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_set,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_options,
                __buffer.add(24).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_select_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn is_element_selected(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_set: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        in_selection_options: &FTypedElementIsSelectedOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_is_element_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_set,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_options,
                __buffer.add(24).cast::<FTypedElementIsSelectedOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_is_element_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_selection_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_current_selection: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        in_selection_method: ETypedElementSelectionMethod,
    ) -> crate::bindings::typed_element_framework::FScriptTypedElementHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_get_selection_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_current_selection,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_method,
                __buffer.add(24).cast::<ETypedElementSelectionMethod>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_get_selection_element,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<
                    crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                >()
                .read()
        }
    }
    pub fn deselect_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_set: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
        in_selection_options: &FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_deselect_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_set,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_options,
                __buffer.add(24).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_deselect_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn can_select_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: &FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_select_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_select_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn can_deselect_element(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_options: &FTypedElementSelectionOptions,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_deselect_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection_options,
                __buffer.add(8).cast::<FTypedElementSelectionOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_can_deselect_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn allow_selection_modifiers(
        &mut self,
        in_element_handle: &crate::bindings::typed_element_framework::FScriptTypedElementHandle,
        in_selection_set: crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_allow_selection_modifiers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_element_handle,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementHandle,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selection_set,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::typed_element_framework::FScriptTypedElementListProxy,
                    >(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::typed_element_runtime::__FUNCTION_PTRS
                    .u_typed_element_selection_interface_allow_selection_modifiers,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct FTypedElementSelectionSet_OnPreSelectionChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTypedElementSelectionSet_OnSelectionChange {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ETypedElementChildInclusionMethod(pub u8);
impl ETypedElementChildInclusionMethod {
    pub const NONE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        0,
    );
    pub const IMMEDIATE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        1,
    );
    pub const RECURSIVE: ETypedElementChildInclusionMethod = ETypedElementChildInclusionMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ETypedElementSelectionMethod(pub u8);
impl ETypedElementSelectionMethod {
    pub const PRIMARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(0);
    pub const SECONDARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(1);
    pub const FROM_SECONDARY: ETypedElementSelectionMethod = ETypedElementSelectionMethod(
        2,
    );
}
