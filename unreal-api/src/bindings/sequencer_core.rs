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
    pub fn set_solo_nodes(
        &mut self,
        in_nodes: &TArray<FSequencerViewModelScriptingStruct>,
        b_in_soloed: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SOLO_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_soloed,
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
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SOLO_NODES,
                __buffer,
            )
        };
    }
    pub fn set_selection(
        &mut self,
        in_selection: &TArray<FSequencerViewModelScriptingStruct>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SELECTION,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_selection,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_SELECTION,
                __buffer,
            )
        };
    }
    pub fn set_pinned_nodes(
        &mut self,
        in_nodes: &TArray<FSequencerViewModelScriptingStruct>,
        b_in_pinned: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_PINNED_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_pinned,
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
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_PINNED_NODES,
                __buffer,
            )
        };
    }
    pub fn set_mute_nodes(
        &mut self,
        in_nodes: &TArray<FSequencerViewModelScriptingStruct>,
        b_in_muted: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_MUTE_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_muted,
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
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_MUTE_NODES,
                __buffer,
            )
        };
    }
    pub fn set_locked_nodes(
        &mut self,
        in_nodes: &TArray<FSequencerViewModelScriptingStruct>,
        b_in_locked: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_LOCKED_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_locked,
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
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_LOCKED_NODES,
                __buffer,
            )
        };
    }
    pub fn set_deactivated_nodes(
        &mut self,
        in_nodes: &TArray<FSequencerViewModelScriptingStruct>,
        b_in_deactivated: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_DEACTIVATED_NODES,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_deactivated,
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
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_SET_DEACTIVATED_NODES,
                __buffer,
            )
        };
    }
    pub fn get_solo_nodes(&mut self) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SOLO_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SOLO_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_selection(&self) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SELECTION,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_SELECTION,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_root_node(&self) -> FSequencerViewModelScriptingStruct {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_ROOT_NODE,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_ROOT_NODE,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSequencerViewModelScriptingStruct>().read() }
    }
    pub fn get_pinned_nodes(&mut self) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_PINNED_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_PINNED_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_mute_nodes(&mut self) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_MUTE_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_MUTE_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_locked_nodes(&mut self) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_LOCKED_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_LOCKED_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_deactivated_nodes(
        &mut self,
    ) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_DEACTIVATED_NODES,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_DEACTIVATED_NODES,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
        }
    }
    pub fn get_children(
        &self,
        node: FSequencerViewModelScriptingStruct,
        type_name: FName,
    ) -> TArray<FSequencerViewModelScriptingStruct> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_CHILDREN,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node,
                __buffer.add(0).cast::<FSequencerViewModelScriptingStruct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &type_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_OUTLINER_SCRIPTING_OBJECT_GET_CHILDREN,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<TArray<FSequencerViewModelScriptingStruct>>().read()
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
    pub fn get_path(view_model: &FSequencerViewModelScriptingStruct) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_PATH,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                view_model,
                __buffer.add(0).cast::<FSequencerViewModelScriptingStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_core::USequencerViewModelStructExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_PATH,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_label(view_model: &FSequencerViewModelScriptingStruct) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_LABEL,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                view_model,
                __buffer.add(0).cast::<FSequencerViewModelScriptingStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_core::USequencerViewModelStructExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::U_SEQUENCER_VIEW_MODEL_STRUCT_EXTENSIONS_GET_LABEL,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct FSequencerOutlinerScriptingObject_OnSelectionChanged {
    _opague: [u8; 24],
}
