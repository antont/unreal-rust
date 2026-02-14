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
    pub u_meta_sound_preset_widget_interface_on_constructed: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_preset_widget_interface_on_audition_state_changed: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_preset_widget_interface_get_supported_meta_sounds: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_builder_listener_remove_all_delegates: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_editor_settings_get_audition_platform_names: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_editor_settings_get_audition_page_names: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_set_node_location: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_set_focused_page: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_find_or_create_graph_input_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_find_or_begin_building: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_build_to_asset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_editor_subsystem_add_builder_delegate_listener: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_meta_sound_preset_widget_interface_on_constructed: std::ptr::null_mut(),
            u_meta_sound_preset_widget_interface_on_audition_state_changed: std::ptr::null_mut(),
            u_meta_sound_preset_widget_interface_get_supported_meta_sounds: std::ptr::null_mut(),
            u_meta_sound_editor_builder_listener_remove_all_delegates: std::ptr::null_mut(),
            u_metasound_editor_settings_get_audition_platform_names: std::ptr::null_mut(),
            u_metasound_editor_settings_get_audition_page_names: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_set_node_location: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_set_focused_page: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_find_or_create_graph_input_metadata: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_find_or_begin_building: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_build_to_asset: std::ptr::null_mut(),
            u_meta_sound_editor_subsystem_add_builder_delegate_listener: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMetaSoundPresetWidgetInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnConstructed"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_constructed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnAuditionStateChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_audition_state_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedMetaSounds"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_get_supported_meta_sounds,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMetaSoundEditorBuilderListener::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllDelegates"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_editor_builder_listener_remove_all_delegates,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMetasoundEditorSettings::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAuditionPlatformNames"),
                &raw mut __FUNCTION_PTRS
                    .u_metasound_editor_settings_get_audition_platform_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAuditionPageNames"),
                &raw mut __FUNCTION_PTRS
                    .u_metasound_editor_settings_get_audition_page_names,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMetaSoundEditorSubsystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeLocation"),
                &raw mut __FUNCTION_PTRS.u_meta_sound_editor_subsystem_set_node_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFocusedPage"),
                &raw mut __FUNCTION_PTRS.u_meta_sound_editor_subsystem_set_focused_page,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindOrCreateGraphInputMetadata"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_create_graph_input_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindOrBeginBuilding"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_begin_building,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BuildToAsset"),
                &raw mut __FUNCTION_PTRS.u_meta_sound_editor_subsystem_build_to_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBuilderDelegateListener"),
                &raw mut __FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_add_builder_delegate_listener,
            );
        }
    }
}
pub struct IMetaSoundPresetWidgetInterface {}
#[repr(C, align(8))]
pub struct UMetaSoundPresetWidgetInterface {
    __padding_end: [u8; 48],
}
impl UMetaSoundPresetWidgetInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundPresetWidgetInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundPresetWidgetInterface")
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
    pub fn on_constructed(
        &mut self,
        builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_constructed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_constructed,
                __buffer,
            )
        };
    }
    pub fn on_audition_state_changed(
        &mut self,
        audio_component: UPtr<crate::bindings::engine::UAudioComponent>,
        b_is_auditioning: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_audition_state_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAudioComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_auditioning,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_on_audition_state_changed,
                __buffer,
            )
        };
    }
    pub fn get_supported_meta_sounds(
        &self,
        b_support_all_presets: &mut bool,
        excluded_meta_sounds: &mut TArray<
            TScriptInterface<
                crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
            >,
        >,
        included_meta_sounds: &mut TArray<
            TScriptInterface<
                crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
            >,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_get_supported_meta_sounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_support_all_presets,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                excluded_meta_sounds,
                __buffer
                    .add(8)
                    .cast::<
                        TArray<
                            TScriptInterface<
                                crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                            >,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                included_meta_sounds,
                __buffer
                    .add(24)
                    .cast::<
                        TArray<
                            TScriptInterface<
                                crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                            >,
                        >,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_preset_widget_interface_get_supported_meta_sounds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_support_all_presets);
        }
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >,
                >()
                .swap(excluded_meta_sounds);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TArray<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >,
                >()
                .swap(included_meta_sounds);
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_MetaSoundPatch {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundPatch {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundPatch")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundPatch")
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
pub struct UAssetDefinition_MetaSoundSource {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_MetaSoundSource {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundSource")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_MetaSoundSource")
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
pub struct UMetasoundEditorViewBase {
    __padding_end: [u8; 56],
}
impl UMetasoundEditorViewBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorViewBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorViewBase")
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
pub struct UMetasoundInterfacesView {
    __padding_end: [u8; 56],
}
impl UMetasoundInterfacesView {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundInterfacesView")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundInterfacesView")
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
pub struct UMetasoundPagesView {
    __padding_end: [u8; 56],
}
impl UMetasoundPagesView {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundPagesView")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundPagesView")
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
pub struct UMetaSoundEditorBuilderListener {
    __padding_end: [u8; 904],
}
impl UMetaSoundEditorBuilderListener {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorBuilderListener")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorBuilderListener")
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
    pub fn remove_all_delegates(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_builder_listener_remove_all_delegates,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_builder_listener_remove_all_delegates,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphMemberDefaultLiteral {
    __padding_end: [u8; 64],
}
impl UMetasoundEditorGraphMemberDefaultLiteral {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultLiteral")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultLiteral")
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
pub struct UMetasoundEditorGraphMember {
    __padding_end: [u8; 104],
}
impl UMetasoundEditorGraphMember {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMember")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMember")
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
pub struct UMetasoundEditorGraphVertex {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphVertex {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVertex")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVertex")
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
pub struct UMetasoundEditorGraphInput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphInput {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInput")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInput")
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
pub struct UMetasoundEditorGraphOutput {
    __padding_end: [u8; 288],
}
impl UMetasoundEditorGraphOutput {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutput")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutput")
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
pub struct UMetasoundEditorGraphVariable {
    __padding_end: [u8; 240],
}
impl UMetasoundEditorGraphVariable {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariable")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariable")
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
pub struct UMetasoundEditorGraph {
    __padding_end: [u8; 248],
}
impl UMetasoundEditorGraph {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraph")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraph")
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
pub struct UMetasoundEditorGraphCommentNode {
    __padding_end: [u8; 312],
}
impl UMetasoundEditorGraphCommentNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphCommentNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphCommentNode")
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
pub struct UMetasoundEditorGraphNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphNode")
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
pub struct UMetasoundEditorGraphMemberNode {
    __padding_end: [u8; 256],
}
impl UMetasoundEditorGraphMemberNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberNode")
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
pub struct UMetasoundEditorGraphInputNode {
    __padding_end: [u8; 616],
}
impl UMetasoundEditorGraphInputNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInputNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphInputNode")
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
pub struct UMetasoundEditorGraphMemberDefaultBool {
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultBool {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBool")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBool")
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
pub struct UMetasoundEditorGraphMemberDefaultBoolArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultBoolArray {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBoolArray")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultBoolArray")
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
pub struct UMetasoundEditorGraphMemberDefaultInt {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultInt {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultInt")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultInt")
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
pub struct UMetasoundEditorGraphMemberDefaultIntArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultIntArray {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultIntArray")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultIntArray")
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
pub struct UMetasoundEditorGraphMemberDefaultFloat {
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 96],
    pub range: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 120],
}
impl UMetasoundEditorGraphMemberDefaultFloat {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloat")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloat")
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
pub struct UMetasoundEditorGraphMemberDefaultFloatArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultFloatArray {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloatArray")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultFloatArray")
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
pub struct UMetasoundEditorGraphMemberDefaultString {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultString {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultString")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultString")
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
pub struct UMetasoundEditorGraphMemberDefaultStringArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultStringArray {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultStringArray")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultStringArray")
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
pub struct UMetasoundEditorGraphMemberDefaultObject {
    __padding_end: [u8; 88],
}
impl UMetasoundEditorGraphMemberDefaultObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObject")
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
pub struct UMetasoundEditorGraphMemberDefaultObjectArray {
    __padding_end: [u8; 96],
}
impl UMetasoundEditorGraphMemberDefaultObjectArray {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObjectArray")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphMemberDefaultObjectArray")
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
pub struct UMetasoundEditorGraphOutputNode {
    __padding_end: [u8; 600],
}
impl UMetasoundEditorGraphOutputNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutputNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphOutputNode")
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
pub struct UMetasoundEditorGraphExternalNode {
    __padding_end: [u8; 416],
}
impl UMetasoundEditorGraphExternalNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphExternalNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphExternalNode")
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
pub struct UMetasoundEditorGraphVariableNode {
    __padding_end: [u8; 648],
}
impl UMetasoundEditorGraphVariableNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariableNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphVariableNode")
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
pub struct UMetasoundEditorGraphSchema {
    __padding_end: [u8; 48],
}
impl UMetasoundEditorGraphSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphSchema")
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
pub struct UMetasoundEditorSettings {
    __padding_end: [u8; 640],
}
impl UMetasoundEditorSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorSettings")
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
pub struct UMetaSoundEditorSubsystem {
    __padding_end: [u8; 72],
}
impl UMetaSoundEditorSubsystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorSubsystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundEditorSubsystem")
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
    pub fn set_node_location(
        &mut self,
        in_builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
        in_node: &crate::bindings::metasound_engine::FMetaSoundNodeHandle,
        in_location: &crate::bindings::core_u_object::FVector2D,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_set_node_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::metasound_engine::FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(40)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_set_node_location,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
    }
    pub fn set_focused_page(
        &self,
        builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
        page_name: FName,
        b_open_editor: bool,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_set_focused_page,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &page_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_open_editor,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(21)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_set_focused_page,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(21)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
    }
    pub fn find_or_create_graph_input_metadata(
        &mut self,
        in_builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
        input_name: FName,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
    ) -> UPtr<crate::bindings::metasound_frontend::UMetaSoundFrontendMemberMetadata> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_create_graph_input_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(20)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_create_graph_input_metadata,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    UPtr<
                        crate::bindings::metasound_frontend::UMetaSoundFrontendMemberMetadata,
                    >,
                >()
                .read()
        }
    }
    pub fn find_or_begin_building(
        &self,
        meta_sound: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
    ) -> UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_begin_building,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_sound,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_find_or_begin_building,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>>()
                .read()
        }
    }
    pub fn build_to_asset(
        &mut self,
        in_builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
        author: FString,
        asset_name: FString,
        package_path: FString,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
        template_sound_wave: UPtr<crate::bindings::engine::USoundWave>,
    ) -> TScriptInterface<
        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
    > {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_build_to_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&author, __buffer.add(8).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_path,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(56)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &template_sound_wave,
                __buffer.add(64).cast::<UPtr<crate::bindings::engine::USoundWave>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_build_to_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer
                .add(72)
                .cast::<
                    TScriptInterface<
                        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn add_builder_delegate_listener(
        &mut self,
        in_builder: UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
        out_result: &mut crate::bindings::metasound_engine::EMetaSoundBuilderResult,
    ) -> UPtr<UMetaSoundEditorBuilderListener> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_add_builder_delegate_listener,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_builder,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::metasound_engine::UMetaSoundBuilderBase>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::metasound_engine::EMetaSoundBuilderResult,
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
                crate::bindings::metasound_editor::__FUNCTION_PTRS
                    .u_meta_sound_editor_subsystem_add_builder_delegate_listener,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_engine::EMetaSoundBuilderResult>()
                .swap(out_result);
        }
        unsafe {
            __buffer.add(16).cast::<UPtr<UMetaSoundEditorBuilderListener>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundBaseFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundBaseFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBaseFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBaseFactory")
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
pub struct UMetaSoundFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundFactory")
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
pub struct UMetaSoundSourceFactory {
    __padding_end: [u8; 144],
}
impl UMetaSoundSourceFactory {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSourceFactory")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSourceFactory")
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
pub struct FMetaSoundEditorBuilderListener_OnDocumentDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentAuthorChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentKeywordsChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentCategoryHierarchyChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnDocumentIsDeprecatedChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphInputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphInputInheritsDefaultChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputAddedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnRemovingGraphOutputDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDisplayNameChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDataTypeChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputDescriptionChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputSortOrderIndexChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsConstructorPinChangedDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMetaSoundEditorBuilderListener_OnGraphOutputIsAdvancedDisplayChangedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMetasoundActiveAnalyzerEnvelopeDirection(pub u8);
impl EMetasoundActiveAnalyzerEnvelopeDirection {
    pub const FROM_SOURCE_OUTPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        0,
    );
    pub const FROM_DESTINATION_INPUT: EMetasoundActiveAnalyzerEnvelopeDirection = EMetasoundActiveAnalyzerEnvelopeDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundBoolMemberDefaultWidget(pub u8);
impl EMetasoundBoolMemberDefaultWidget {
    pub const NONE: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        0,
    );
    pub const BUTTON: EMetasoundBoolMemberDefaultWidget = EMetasoundBoolMemberDefaultWidget(
        1,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidget(pub u8);
impl EMetasoundMemberDefaultWidget {
    pub const NONE: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(0);
    pub const SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(1);
    pub const RADIAL_SLIDER: EMetasoundMemberDefaultWidget = EMetasoundMemberDefaultWidget(
        2,
    );
}
#[repr(transparent)]
pub struct EMetasoundMemberDefaultWidgetValueType(pub u8);
impl EMetasoundMemberDefaultWidgetValueType {
    pub const LINEAR: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        0,
    );
    pub const FREQUENCY: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        1,
    );
    pub const VOLUME: EMetasoundMemberDefaultWidgetValueType = EMetasoundMemberDefaultWidgetValueType(
        2,
    );
}
#[repr(transparent)]
pub struct EAuditionPageMode(pub u8);
impl EAuditionPageMode {
    pub const FOCUSED: EAuditionPageMode = EAuditionPageMode(0);
    pub const USER: EAuditionPageMode = EAuditionPageMode(1);
}
#[repr(transparent)]
pub struct EMetasoundActiveDetailView(pub u8);
impl EMetasoundActiveDetailView {
    pub const METASOUND: EMetasoundActiveDetailView = EMetasoundActiveDetailView(0);
    pub const GENERAL: EMetasoundActiveDetailView = EMetasoundActiveDetailView(1);
}
