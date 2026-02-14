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
    pub u_sequencer_curve_editor_object_show_curve: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_set_random_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_set_custom_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_set_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_select_keys: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_open_curve_editor: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_is_curve_shown: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_is_curve_editor_open: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_has_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_get_selected_keys: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_get_custom_color_for_channel: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_get_channels_with_selected_keys: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_empty_selection: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_delete_color_for_channels: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_close_curve_editor: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_curve_editor_object_apply_filter: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_render_movie: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_link_anim_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_is_rendering_movie: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_is_event_endpoint_valid: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_import_level_sequence_fbx: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_import_fbx_to_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_get_object_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_get_level_sequence_link_from_anim_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_get_bound_objects: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_get_anim_sequence_link_from_level_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_export_level_sequence_fbx: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_export_fbx_from_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_export_anim_sequence_wait_for_delegate: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_export_anim_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_create_quick_binding: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_create_event: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_clear_linked_anim_sequences: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_tools_function_library_cancel_movie_render: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_sequencer_curve_editor_object_show_curve: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_set_random_color_for_channels: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_set_custom_color_for_channels: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_set_custom_color_for_channel: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_select_keys: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_open_curve_editor: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_is_curve_shown: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_is_curve_editor_open: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_has_custom_color_for_channel: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_get_selected_keys: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_get_custom_color_for_channel: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_get_channels_with_selected_keys: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_empty_selection: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_delete_color_for_channels: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_close_curve_editor: std::ptr::null_mut(),
            u_sequencer_curve_editor_object_apply_filter: std::ptr::null_mut(),
            u_sequencer_tools_function_library_render_movie: std::ptr::null_mut(),
            u_sequencer_tools_function_library_link_anim_sequence: std::ptr::null_mut(),
            u_sequencer_tools_function_library_is_rendering_movie: std::ptr::null_mut(),
            u_sequencer_tools_function_library_is_event_endpoint_valid: std::ptr::null_mut(),
            u_sequencer_tools_function_library_import_level_sequence_fbx: std::ptr::null_mut(),
            u_sequencer_tools_function_library_import_fbx_to_control_rig: std::ptr::null_mut(),
            u_sequencer_tools_function_library_get_object_bindings: std::ptr::null_mut(),
            u_sequencer_tools_function_library_get_level_sequence_link_from_anim_sequence: std::ptr::null_mut(),
            u_sequencer_tools_function_library_get_bound_objects: std::ptr::null_mut(),
            u_sequencer_tools_function_library_get_anim_sequence_link_from_level_sequence: std::ptr::null_mut(),
            u_sequencer_tools_function_library_export_level_sequence_fbx: std::ptr::null_mut(),
            u_sequencer_tools_function_library_export_fbx_from_control_rig: std::ptr::null_mut(),
            u_sequencer_tools_function_library_export_anim_sequence_wait_for_delegate: std::ptr::null_mut(),
            u_sequencer_tools_function_library_export_anim_sequence: std::ptr::null_mut(),
            u_sequencer_tools_function_library_create_quick_binding: std::ptr::null_mut(),
            u_sequencer_tools_function_library_create_event: std::ptr::null_mut(),
            u_sequencer_tools_function_library_clear_linked_anim_sequences: std::ptr::null_mut(),
            u_sequencer_tools_function_library_cancel_movie_render: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencerCurveEditorObject::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowCurve"),
                &raw mut __FUNCTION_PTRS.u_sequencer_curve_editor_object_show_curve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRandomColorForChannels"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_random_color_for_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomColorForChannels"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomColorForChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectKeys"),
                &raw mut __FUNCTION_PTRS.u_sequencer_curve_editor_object_select_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenCurveEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_open_curve_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCurveShown"),
                &raw mut __FUNCTION_PTRS.u_sequencer_curve_editor_object_is_curve_shown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCurveEditorOpen"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_is_curve_editor_open,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasCustomColorForChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_has_custom_color_for_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedKeys"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_selected_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomColorForChannel"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_custom_color_for_channel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChannelsWithSelectedKeys"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_channels_with_selected_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EmptySelection"),
                &raw mut __FUNCTION_PTRS.u_sequencer_curve_editor_object_empty_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteColorForChannels"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_delete_color_for_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CloseCurveEditor"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_close_curve_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyFilter"),
                &raw mut __FUNCTION_PTRS.u_sequencer_curve_editor_object_apply_filter,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencerToolsFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenderMovie"),
                &raw mut __FUNCTION_PTRS.u_sequencer_tools_function_library_render_movie,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LinkAnimSequence"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_link_anim_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRenderingMovie"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_rendering_movie,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEventEndpointValid"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_event_endpoint_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportLevelSequenceFBX"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_level_sequence_fbx,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportFBXToControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_fbx_to_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectBindings"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_object_bindings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLevelSequenceLinkFromAnimSequence"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_level_sequence_link_from_anim_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoundObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_bound_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimSequenceLinkFromLevelSequence"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_anim_sequence_link_from_level_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportLevelSequenceFBX"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_level_sequence_fbx,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportFBXFromControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_fbx_from_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportAnimSequenceWaitForDelegate"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence_wait_for_delegate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportAnimSequence"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateQuickBinding"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_create_quick_binding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateEvent"),
                &raw mut __FUNCTION_PTRS.u_sequencer_tools_function_library_create_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearLinkedAnimSequences"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_clear_linked_anim_sequences,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelMovieRender"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_tools_function_library_cancel_movie_render,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FSequencerChannelProxy {
    pub channel_name: FName,
    pub section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
impl FSequencerChannelProxy {}
#[repr(C, align(8))]
pub struct FSequencerBoundObjects {
    pub binding_proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub bound_objects: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl FSequencerBoundObjects {}
#[repr(C, align(8))]
pub struct FSequencerQuickBindingResult {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub payload_names: TArray<FString>,
}
impl FSequencerQuickBindingResult {}
#[repr(C, align(8))]
pub struct FSequencerExportFBXParams {
    pub world: UPtr<crate::bindings::engine::UWorld>,
    pub sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub root_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub tracks: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    pub override_options: UPtr<crate::bindings::unreal_ed::UFbxExportOption>,
    pub fbx_file_name: FString,
}
impl FSequencerExportFBXParams {}
#[repr(C, align(8))]
pub struct USequencerCurveEditorObject {
    __padding_end: [u8; 64],
}
impl USequencerCurveEditorObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerCurveEditorObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerCurveEditorObject")
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
    pub fn show_curve(&mut self, channel: &FSequencerChannelProxy, b_show_curve: bool) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_show_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel,
                __buffer.add(0).cast::<FSequencerChannelProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_curve,
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
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_show_curve,
                __buffer,
            )
        };
    }
    pub fn set_random_color_for_channels(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifiers: &TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_random_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifiers,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_random_color_for_channels,
                __buffer,
            )
        };
    }
    pub fn set_custom_color_for_channels(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifiers: &TArray<FString>,
        new_colors: &TArray<crate::bindings::core_u_object::FLinearColor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifiers,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_colors,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channels,
                __buffer,
            )
        };
    }
    pub fn set_custom_color_for_channel(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
        new_color: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_color,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_set_custom_color_for_channel,
                __buffer,
            )
        };
    }
    pub fn select_keys(
        &mut self,
        channel: &FSequencerChannelProxy,
        indices: &TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_select_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel,
                __buffer.add(0).cast::<FSequencerChannelProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_select_keys,
                __buffer,
            )
        };
    }
    pub fn open_curve_editor(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_open_curve_editor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_open_curve_editor,
                __buffer,
            )
        };
    }
    pub fn is_curve_shown(&mut self, channel: &FSequencerChannelProxy) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_is_curve_shown,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel,
                __buffer.add(0).cast::<FSequencerChannelProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_is_curve_shown,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_curve_editor_open(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_is_curve_editor_open,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_is_curve_editor_open,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_custom_color_for_channel(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_has_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_has_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_selected_keys(
        &mut self,
        channel_proxy: &FSequencerChannelProxy,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_selected_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel_proxy,
                __buffer.add(0).cast::<FSequencerChannelProxy>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_selected_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<i32>>().read() }
    }
    pub fn get_custom_color_for_channel(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: FString,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_custom_color_for_channel,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_channels_with_selected_keys(&mut self) -> TArray<FSequencerChannelProxy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_channels_with_selected_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_get_channels_with_selected_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FSequencerChannelProxy>>().read() }
    }
    pub fn empty_selection(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_empty_selection,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_empty_selection,
                __buffer,
            )
        };
    }
    pub fn delete_color_for_channels(
        &mut self,
        class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        identifier: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_delete_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                identifier,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_delete_color_for_channels,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(identifier);
        }
    }
    pub fn close_curve_editor(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_close_curve_editor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_close_curve_editor,
                __buffer,
            )
        };
    }
    pub fn apply_filter(
        &mut self,
        filter: UPtr<crate::bindings::curve_editor::UCurveEditorFilterBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_apply_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::curve_editor::UCurveEditorFilterBase>,
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
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_curve_editor_object_apply_filter,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USequencerToolsFunctionLibrary {
    __padding_end: [u8; 48],
}
impl USequencerToolsFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerToolsFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerToolsFunctionLibrary")
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
    pub fn render_movie(
        in_capture_settings: UPtr<
            crate::bindings::movie_scene_capture::UMovieSceneCapture,
        >,
        on_finished_callback: FRenderMovie_OnFinishedCallback,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_render_movie,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_capture_settings,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::movie_scene_capture::UMovieSceneCapture>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_finished_callback,
                __buffer.add(8).cast::<FRenderMovie_OnFinishedCallback>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_render_movie,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn link_anim_sequence(
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        export_options: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_link_anim_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_options,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_link_anim_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn is_rendering_movie() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_rendering_movie,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_rendering_movie,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_event_endpoint_valid(in_endpoint: &FSequencerQuickBindingResult) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_event_endpoint_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_endpoint,
                __buffer.add(0).cast::<FSequencerQuickBindingResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_is_event_endpoint_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn import_level_sequence_fbx(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
        in_import_fbx_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXSettings,
        >,
        in_import_filename: FString,
        actor_context: UPtr<crate::bindings::level_sequence::ALevelSequenceActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_level_sequence_fbx,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bindings,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_import_fbx_settings,
                __buffer
                    .add(32)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXSettings,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_import_filename,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_context,
                __buffer
                    .add(56)
                    .cast::<
                        UPtr<crate::bindings::level_sequence::ALevelSequenceActor>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_level_sequence_fbx,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn import_fbx_to_control_rig(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor_with_control_rig_track: FString,
        selected_control_rig_names: &TArray<FString>,
        import_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
        >,
        import_filename: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_fbx_to_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_with_control_rig_track,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                selected_control_rig_names,
                __buffer.add(32).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_fbx_control_rig_settings,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_filename,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_import_fbx_to_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn get_object_bindings(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_object: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        in_range: &crate::bindings::sequencer_scripting::FSequencerScriptingRange,
    ) -> TArray<FSequencerBoundObjects> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_object_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_object,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_range,
                __buffer
                    .add(32)
                    .cast::<
                        crate::bindings::sequencer_scripting::FSequencerScriptingRange,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_object_bindings,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<FSequencerBoundObjects>>().read() }
    }
    pub fn get_level_sequence_link_from_anim_sequence(
        in_anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) -> UPtr<crate::bindings::level_sequence::UAnimSequenceLevelSequenceLink> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_level_sequence_link_from_anim_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_anim_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_level_sequence_link_from_anim_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<crate::bindings::level_sequence::UAnimSequenceLevelSequenceLink>,
                >()
                .read()
        }
    }
    pub fn get_bound_objects(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_bindings: &TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
        in_range: &crate::bindings::sequencer_scripting::FSequencerScriptingRange,
    ) -> TArray<FSequencerBoundObjects> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_bound_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_bindings,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_range,
                __buffer
                    .add(32)
                    .cast::<
                        crate::bindings::sequencer_scripting::FSequencerScriptingRange,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_bound_objects,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<FSequencerBoundObjects>>().read() }
    }
    pub fn get_anim_sequence_link_from_level_sequence(
        in_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) -> UPtr<crate::bindings::level_sequence::ULevelSequenceAnimSequenceLink> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_anim_sequence_link_from_level_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_get_anim_sequence_link_from_level_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<crate::bindings::level_sequence::ULevelSequenceAnimSequenceLink>,
                >()
                .read()
        }
    }
    pub fn export_level_sequence_fbx(in_params: &FSequencerExportFBXParams) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_level_sequence_fbx,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FSequencerExportFBXParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_level_sequence_fbx,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn export_fbx_from_control_rig(
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor_with_control_rig_track: FString,
        export_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_fbx_from_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_with_control_rig_track,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_fbx_control_rig_settings,
                __buffer
                    .add(24)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_fbx_from_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn export_anim_sequence_wait_for_delegate(
        world: UPtr<crate::bindings::engine::UWorld>,
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        export_options: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_create_link: bool,
        delegate: FExportAnimSequenceWaitForDelegate_Delegate,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence_wait_for_delegate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_options,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_link,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delegate,
                __buffer.add(64).cast::<FExportAnimSequenceWaitForDelegate_Delegate>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence_wait_for_delegate,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn export_anim_sequence(
        world: UPtr<crate::bindings::engine::UWorld>,
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        export_options: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_create_link: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_options,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_link,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_export_anim_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(57).cast::<bool>().read() }
    }
    pub fn create_quick_binding(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_function_name: FString,
        b_call_in_editor: bool,
    ) -> FSequencerQuickBindingResult {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_create_quick_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_call_in_editor,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_create_quick_binding,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FSequencerQuickBindingResult>().read() }
    }
    pub fn create_event(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_section: UPtr<
            crate::bindings::movie_scene_tracks::UMovieSceneEventSectionBase,
        >,
        in_endpoint: &FSequencerQuickBindingResult,
        in_payload: &TArray<FString>,
    ) -> crate::bindings::movie_scene_tracks::FMovieSceneEvent {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_create_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneEventSectionBase,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_endpoint,
                __buffer.add(16).cast::<FSequencerQuickBindingResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_payload,
                __buffer.add(40).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_create_event,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::movie_scene_tracks::FMovieSceneEvent>()
                .read()
        }
    }
    pub fn clear_linked_anim_sequences(
        in_level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_clear_linked_anim_sequences,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_clear_linked_anim_sequences,
                __buffer,
            )
        };
    }
    pub fn cancel_movie_render() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_cancel_movie_render,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::__FUNCTION_PTRS
                    .u_sequencer_tools_function_library_cancel_movie_render,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FExportAnimSequenceWaitForDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRenderMovie_OnFinishedCallback {
    _opague: [u8; 32],
}
