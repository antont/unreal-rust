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
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SHOW_CURVE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_RANDOM_COLOR_FOR_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SELECT_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_OPEN_CURVE_EDITOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_SHOWN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_EDITOR_OPEN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_HAS_CUSTOM_COLOR_FOR_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_SELECTED_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CUSTOM_COLOR_FOR_CHANNEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CHANNELS_WITH_SELECTED_KEYS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_EMPTY_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_DELETE_COLOR_FOR_CHANNELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_CLOSE_CURVE_EDITOR: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_CURVE_EDITOR_OBJECT_APPLY_FILTER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_RENDER_MOVIE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_LINK_ANIM_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_RENDERING_MOVIE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_EVENT_ENDPOINT_VALID: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_LEVEL_SEQUENCE_FBX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_OBJECT_BINDINGS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_LEVEL_SEQUENCE_LINK_FROM_ANIM_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_BOUND_OBJECTS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_ANIM_SEQUENCE_LINK_FROM_LEVEL_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_LEVEL_SEQUENCE_FBX: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE_WAIT_FOR_DELEGATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_QUICK_BINDING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_EVENT: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CLEAR_LINKED_ANIM_SEQUENCES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CANCEL_MOVIE_RENDER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerCurveEditorObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShowCurve"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SHOW_CURVE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRandomColorForChannels"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_RANDOM_COLOR_FOR_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomColorForChannels"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomColorForChannel"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectKeys"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_SELECT_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenCurveEditor"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_OPEN_CURVE_EDITOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurveShown"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_SHOWN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCurveEditorOpen"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_EDITOR_OPEN,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasCustomColorForChannel"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_HAS_CUSTOM_COLOR_FOR_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedKeys"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_SELECTED_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomColorForChannel"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CUSTOM_COLOR_FOR_CHANNEL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelsWithSelectedKeys"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CHANNELS_WITH_SELECTED_KEYS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmptySelection"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_EMPTY_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteColorForChannels"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_DELETE_COLOR_FOR_CHANNELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseCurveEditor"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_CLOSE_CURVE_EDITOR,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyFilter"),
            &raw mut U_SEQUENCER_CURVE_EDITOR_OBJECT_APPLY_FILTER,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerToolsFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenderMovie"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_RENDER_MOVIE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LinkAnimSequence"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_LINK_ANIM_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRenderingMovie"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_RENDERING_MOVIE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEventEndpointValid"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_EVENT_ENDPOINT_VALID,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportLevelSequenceFBX"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_LEVEL_SEQUENCE_FBX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportFBXToControlRig"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectBindings"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_OBJECT_BINDINGS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelSequenceLinkFromAnimSequence"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_LEVEL_SEQUENCE_LINK_FROM_ANIM_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjects"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_BOUND_OBJECTS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimSequenceLinkFromLevelSequence"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_ANIM_SEQUENCE_LINK_FROM_LEVEL_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportLevelSequenceFBX"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_LEVEL_SEQUENCE_FBX,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportFBXFromControlRig"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAnimSequenceWaitForDelegate"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE_WAIT_FOR_DELEGATE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportAnimSequence"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateQuickBinding"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_QUICK_BINDING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateEvent"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_EVENT,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearLinkedAnimSequences"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CLEAR_LINKED_ANIM_SEQUENCES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CancelMovieRender"),
            &raw mut U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CANCEL_MOVIE_RENDER,
        );
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
    __padding_8: [u8; 8],
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerCurveEditorObject")
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
    pub fn show_curve(&mut self, channel: &FSequencerChannelProxy, b_show_curve: bool) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SHOW_CURVE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SHOW_CURVE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_RANDOM_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_RANDOM_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SET_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SELECT_KEYS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_SELECT_KEYS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_OPEN_CURVE_EDITOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_OPEN_CURVE_EDITOR,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_SHOWN,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_SHOWN,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_EDITOR_OPEN,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_IS_CURVE_EDITOR_OPEN,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_HAS_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_HAS_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_SELECTED_KEYS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_SELECTED_KEYS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CUSTOM_COLOR_FOR_CHANNEL,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CHANNELS_WITH_SELECTED_KEYS,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_GET_CHANNELS_WITH_SELECTED_KEYS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_EMPTY_SELECTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_EMPTY_SELECTION,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_DELETE_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_DELETE_COLOR_FOR_CHANNELS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_CLOSE_CURVE_EDITOR,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_CLOSE_CURVE_EDITOR,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_APPLY_FILTER,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_CURVE_EDITOR_OBJECT_APPLY_FILTER,
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
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerToolsFunctionLibrary")
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_RENDER_MOVIE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_RENDER_MOVIE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_LINK_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_LINK_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_RENDERING_MOVIE,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_RENDERING_MOVIE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_EVENT_ENDPOINT_VALID,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IS_EVENT_ENDPOINT_VALID,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_LEVEL_SEQUENCE_FBX,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_LEVEL_SEQUENCE_FBX,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_IMPORT_FBX_TO_CONTROL_RIG,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_OBJECT_BINDINGS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_OBJECT_BINDINGS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_LEVEL_SEQUENCE_LINK_FROM_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_LEVEL_SEQUENCE_LINK_FROM_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_BOUND_OBJECTS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_BOUND_OBJECTS,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_ANIM_SEQUENCE_LINK_FROM_LEVEL_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_GET_ANIM_SEQUENCE_LINK_FROM_LEVEL_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_LEVEL_SEQUENCE_FBX,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_LEVEL_SEQUENCE_FBX,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_FBX_FROM_CONTROL_RIG,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE_WAIT_FOR_DELEGATE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE_WAIT_FOR_DELEGATE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_EXPORT_ANIM_SEQUENCE,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_QUICK_BINDING,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_QUICK_BINDING,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_EVENT,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CREATE_EVENT,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CLEAR_LINKED_ANIM_SEQUENCES,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CLEAR_LINKED_ANIM_SEQUENCES,
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
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CANCEL_MOVIE_RENDER,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::sequencer_scripting_editor::USequencerToolsFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting_editor::U_SEQUENCER_TOOLS_FUNCTION_LIBRARY_CANCEL_MOVIE_RENDER,
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
