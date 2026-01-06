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
}
#[repr(C, align(8))]
pub struct FExportAnimSequenceWaitForDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRenderMovie_OnFinishedCallback {
    _opague: [u8; 32],
}
