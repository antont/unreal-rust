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
    pub u_sequencer_outliner_scripting_object_set_solo_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_set_selection: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_set_pinned_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_set_mute_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_set_locked_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_set_deactivated_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_solo_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_selection: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_root_node: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_pinned_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_mute_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_locked_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_deactivated_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_outliner_scripting_object_get_children: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_view_model_struct_extensions_get_path: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_view_model_struct_extensions_get_label: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_sequencer_outliner_scripting_object_set_solo_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_set_selection: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_set_pinned_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_set_mute_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_set_locked_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_set_deactivated_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_solo_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_selection: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_root_node: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_pinned_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_mute_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_locked_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_deactivated_nodes: std::ptr::null_mut(),
            u_sequencer_outliner_scripting_object_get_children: std::ptr::null_mut(),
            u_sequencer_view_model_struct_extensions_get_path: std::ptr::null_mut(),
            u_sequencer_view_model_struct_extensions_get_label: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencerOutlinerScriptingObject::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSoloNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_solo_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinnedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_pinned_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMuteNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_mute_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLockedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_locked_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDeactivatedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_deactivated_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSoloNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_solo_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRootNode"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_root_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinnedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_pinned_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMuteNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_mute_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLockedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_locked_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDeactivatedNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_deactivated_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChildren"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_children,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USequencerViewModelStructExtensions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPath"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLabel"),
                &raw mut __FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_label,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FSequencerViewModelScriptingStruct {
    pub ty: FName,
    pub(crate) __padding_end: [u8; 20],
}
impl FSequencerViewModelScriptingStruct {}
#[repr(C, align(8))]
pub struct USequencerOutlinerScriptingObject {
    __padding_end: [u8; 88],
}
impl USequencerOutlinerScriptingObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerOutlinerScriptingObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerOutlinerScriptingObject")
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_solo_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_solo_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_selection,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_selection,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_pinned_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_pinned_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_mute_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_mute_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_locked_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_locked_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_deactivated_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_set_deactivated_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_solo_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_solo_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_selection,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_selection,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_root_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_root_node,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_pinned_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_pinned_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_mute_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_mute_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_locked_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_locked_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_deactivated_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_deactivated_nodes,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_children,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_outliner_scripting_object_get_children,
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
    pub(crate) __padding_48: [u8; 48],
    pub outliner: UPtr<USequencerOutlinerScriptingObject>,
}
impl USequencerScriptingLayer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerScriptingLayer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerScriptingLayer")
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
pub struct USequencerViewModelStructExtensions {
    __padding_end: [u8; 48],
}
impl USequencerViewModelStructExtensions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerViewModelStructExtensions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerViewModelStructExtensions")
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
    pub fn get_path(view_model: &FSequencerViewModelScriptingStruct) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_path,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_path,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_label,
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
                crate::bindings::sequencer_core::__FUNCTION_PTRS
                    .u_sequencer_view_model_struct_extensions_get_label,
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
