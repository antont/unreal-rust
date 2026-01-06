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
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SOLO_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_PINNED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_MUTE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_LOCKED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_DEACTIVATED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SOLO_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SELECTION: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_ROOT_NODE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_PINNED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_MUTE_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_LOCKED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_DEACTIVATED_NODES: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_CHILDREN: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_PATH: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_LABEL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerOutlinerScriptingObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSoloNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SOLO_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelection"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPinnedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_PINNED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMuteNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_MUTE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLockedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_LOCKED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDeactivatedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_DEACTIVATED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSoloNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SOLO_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelection"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SELECTION,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootNode"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_ROOT_NODE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPinnedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_PINNED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMuteNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_MUTE_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLockedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_LOCKED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDeactivatedNodes"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_DEACTIVATED_NODES,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildren"),
            &raw mut U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_CHILDREN,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerViewModelStructExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPath"),
            &raw mut U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_PATH,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLabel"),
            &raw mut U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_LABEL,
        );
    }
}
#[repr(C, align(8))]
pub struct FSequencerViewModelScriptingStruct {
    pub ty: FName,
    __padding_end: [u8; 20],
}
impl FSequencerViewModelScriptingStruct {}
#[repr(C, align(8))]
pub struct USequencerOutlinerScriptingObject {
    __padding_end: [u8; 88],
}
impl USequencerOutlinerScriptingObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerOutlinerScriptingObject")
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
pub struct USequencerScriptingLayer {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub outliner: UPtr<USequencerOutlinerScriptingObject>,
}
impl USequencerScriptingLayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerScriptingLayer")
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
pub struct USequencerViewModelStructExtensions {
    __padding_end: [u8; 48],
}
impl USequencerViewModelStructExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerViewModelStructExtensions")
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
pub struct FSequencerOutlinerScriptingObject_OnSelectionChanged {
    _opague: [u8; 24],
}
