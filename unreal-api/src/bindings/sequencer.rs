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
    pub u_sequencer_module_outliner_scripting_object_remove_binding_tags: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_outliner_scripting_object_get_sections: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_outliner_scripting_object_get_previous_key: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_outliner_scripting_object_get_next_key: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_outliner_scripting_object_get_binding_tags: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_outliner_scripting_object_add_binding_tags: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_module_scripting_layer_get_outliner: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_settings_should_show_thumbnail_capture_settings: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_sequencer_module_outliner_scripting_object_remove_binding_tags: std::ptr::null_mut(),
            u_sequencer_module_outliner_scripting_object_get_sections: std::ptr::null_mut(),
            u_sequencer_module_outliner_scripting_object_get_previous_key: std::ptr::null_mut(),
            u_sequencer_module_outliner_scripting_object_get_next_key: std::ptr::null_mut(),
            u_sequencer_module_outliner_scripting_object_get_binding_tags: std::ptr::null_mut(),
            u_sequencer_module_outliner_scripting_object_add_binding_tags: std::ptr::null_mut(),
            u_sequencer_module_scripting_layer_get_outliner: std::ptr::null_mut(),
            u_sequencer_settings_should_show_thumbnail_capture_settings: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerModuleOutlinerScriptingObject::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBindingTags"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_remove_binding_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSections"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_get_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviousKey"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_get_previous_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNextKey"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_get_next_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBindingTags"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_get_binding_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddBindingTags"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_module_outliner_scripting_object_add_binding_tags,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerModuleScriptingLayer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutliner"),
            &raw mut __FUNCTION_PTRS.u_sequencer_module_scripting_layer_get_outliner,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ShouldShowThumbnailCaptureSettings"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_settings_should_show_thumbnail_capture_settings,
        );
    }
}
#[repr(C, align(4))]
pub struct FColumnVisibilitySetting {
    pub(crate) __padding_end: [u8; 16],
}
impl FColumnVisibilitySetting {}
#[repr(C, align(8))]
pub struct FMovieScenePasteFoldersParams {
    pub sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
}
impl FMovieScenePasteFoldersParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteSectionsParams {
    pub tracks: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>,
    pub track_row_indices: TArray<i32>,
    pub time: crate::bindings::core_u_object::FFrameTime,
}
impl FMovieScenePasteSectionsParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteTracksParams {
    pub sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    pub folders: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
}
impl FMovieScenePasteTracksParams {}
#[repr(C, align(8))]
pub struct FMovieScenePasteBindingsParams {
    pub bindings: TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>,
    pub parent_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    pub folders: TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>,
    pub b_duplicate_existing_actors: bool,
    pub pasted_actors: TMap<FName, UPtr<crate::bindings::engine::AActor>>,
}
impl FMovieScenePasteBindingsParams {}
#[repr(C, align(8))]
pub struct UMotionTrailToolOptions {
    __padding_end: [u8; 600],
}
impl UMotionTrailToolOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionTrailToolOptions")
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
#[repr(C, align(16))]
pub struct UMovieSceneCopyableBinding {
    __padding_end: [u8; 944],
}
impl UMovieSceneCopyableBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCopyableBinding")
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
pub struct UMovieSceneCopyableTrack {
    __padding_end: [u8; 80],
}
impl UMovieSceneCopyableTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCopyableTrack")
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
pub struct USequencerFilterBarContext {
    __padding_end: [u8; 88],
}
impl USequencerFilterBarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerFilterBarContext")
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
pub struct USequencerFilterMenuContext {
    __padding_end: [u8; 88],
}
impl USequencerFilterMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerFilterMenuContext")
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
pub struct USequencerMenuContext {
    __padding_end: [u8; 88],
}
impl USequencerMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerMenuContext")
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
pub struct USequencerToolMenuContext {
    __padding_end: [u8; 64],
}
impl USequencerToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerToolMenuContext")
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
pub struct USequencerClockSourceMenuContext {
    __padding_end: [u8; 80],
}
impl USequencerClockSourceMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerClockSourceMenuContext")
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
pub struct USequencerTimeSliderControllerMenuContext {
    __padding_end: [u8; 264],
}
impl USequencerTimeSliderControllerMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerTimeSliderControllerMenuContext")
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
pub struct USequencerTrackFilterExtension {
    __padding_end: [u8; 48],
}
impl USequencerTrackFilterExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerTrackFilterExtension")
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
pub struct USequencerTrackFilterTextExpressionExtension {
    __padding_end: [u8; 48],
}
impl USequencerTrackFilterTextExpressionExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerTrackFilterTextExpressionExtension")
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
pub struct USequencerModuleOutlinerScriptingObject {
    __padding_end: [u8; 88],
}
impl USequencerModuleOutlinerScriptingObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerModuleOutlinerScriptingObject")
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
    pub fn remove_binding_tags(
        &mut self,
        in_nodes: &TArray<
            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
        >,
        tag_names: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_remove_binding_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_remove_binding_tags,
                __buffer,
            )
        };
    }
    pub fn get_sections(
        &self,
        in_nodes: &TArray<
            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
        >,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
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
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_sections,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>>()
                .read()
        }
    }
    pub fn get_previous_key(
        &self,
        in_nodes: &TArray<
            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
        >,
        frame_number: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameNumber {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_previous_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_number,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_previous_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FFrameNumber>()
                .read()
        }
    }
    pub fn get_next_key(
        &self,
        in_nodes: &TArray<
            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
        >,
        frame_number: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameNumber {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_next_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_number,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_next_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FFrameNumber>()
                .read()
        }
    }
    pub fn get_binding_tags(
        &mut self,
        in_node: crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_binding_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
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
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_get_binding_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<FName>>().read() }
    }
    pub fn add_binding_tags(
        &mut self,
        in_nodes: &TArray<
            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
        >,
        tag_names: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_add_binding_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_nodes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<
                            crate::bindings::sequencer_core::FSequencerViewModelScriptingStruct,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_outliner_scripting_object_add_binding_tags,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct USequencerModuleScriptingLayer {
    __padding_end: [u8; 56],
}
impl USequencerModuleScriptingLayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerModuleScriptingLayer")
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
    pub fn get_outliner(&mut self) -> UPtr<USequencerModuleOutlinerScriptingObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_scripting_layer_get_outliner,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer::__FUNCTION_PTRS
                    .u_sequencer_module_scripting_layer_get_outliner,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<USequencerModuleOutlinerScriptingObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneKeyStructType {
    __padding_end: [u8; 416],
}
impl UMovieSceneKeyStructType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneKeyStructType")
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
pub struct USequencerSettingsContainer {
    __padding_end: [u8; 48],
}
impl USequencerSettingsContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerSettingsContainer")
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
pub struct USequencerSettings {
    __padding_end: [u8; 960],
}
impl USequencerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerSettings")
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
pub struct USequencerTimeChangeUndoRedoProxy {
    __padding_end: [u8; 88],
}
impl USequencerTimeChangeUndoRedoProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerTimeChangeUndoRedoProxy")
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
#[repr(transparent)]
pub struct ESequencerThumbnailCaptureTimeLocation(pub u8);
impl ESequencerThumbnailCaptureTimeLocation {
    pub const FIRST_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        0,
    );
    pub const MIDDLE_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        1,
    );
    pub const LAST_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        2,
    );
    pub const CURRENT_FRAME: ESequencerThumbnailCaptureTimeLocation = ESequencerThumbnailCaptureTimeLocation(
        3,
    );
}
#[repr(transparent)]
pub struct ESequencerLoopMode(pub u8);
impl ESequencerLoopMode {
    pub const SLM_NO_LOOP: ESequencerLoopMode = ESequencerLoopMode(0);
    pub const SLM_LOOP: ESequencerLoopMode = ESequencerLoopMode(1);
    pub const SLM_LOOP_SELECTION_RANGE: ESequencerLoopMode = ESequencerLoopMode(2);
}
#[repr(transparent)]
pub struct EMotionTrailTrailStyle(pub u8);
impl EMotionTrailTrailStyle {
    pub const DEFAULT: EMotionTrailTrailStyle = EMotionTrailTrailStyle(0);
    pub const DASHED: EMotionTrailTrailStyle = EMotionTrailTrailStyle(1);
    pub const TIME: EMotionTrailTrailStyle = EMotionTrailTrailStyle(2);
    pub const HEAT_MAP: EMotionTrailTrailStyle = EMotionTrailTrailStyle(3);
}
#[repr(transparent)]
pub struct EAutoChangeMode(pub u8);
impl EAutoChangeMode {
    pub const AUTO_KEY: EAutoChangeMode = EAutoChangeMode(0);
    pub const AUTO_TRACK: EAutoChangeMode = EAutoChangeMode(1);
    pub const ALL: EAutoChangeMode = EAutoChangeMode(2);
    pub const NONE: EAutoChangeMode = EAutoChangeMode(3);
}
#[repr(transparent)]
pub struct EAllowEditsMode(pub u8);
impl EAllowEditsMode {
    pub const ALL_EDITS: EAllowEditsMode = EAllowEditsMode(0);
    pub const ALLOW_SEQUENCER_EDITS_ONLY: EAllowEditsMode = EAllowEditsMode(1);
    pub const ALLOW_LEVEL_EDITS_ONLY: EAllowEditsMode = EAllowEditsMode(2);
}
#[repr(transparent)]
pub struct EKeyGroupMode(pub u8);
impl EKeyGroupMode {
    pub const KEY_CHANGED: EKeyGroupMode = EKeyGroupMode(0);
    pub const KEY_GROUP: EKeyGroupMode = EKeyGroupMode(1);
    pub const KEY_ALL: EKeyGroupMode = EKeyGroupMode(2);
}
#[repr(transparent)]
pub struct ESequencerSpawnPosition(pub u8);
impl ESequencerSpawnPosition {
    pub const SSP_ORIGIN: ESequencerSpawnPosition = ESequencerSpawnPosition(0);
    pub const SSP_PLACE_IN_FRONT_OF_CAMERA: ESequencerSpawnPosition = ESequencerSpawnPosition(
        1,
    );
}
#[repr(transparent)]
pub struct ESequencerZoomPosition(pub u8);
impl ESequencerZoomPosition {
    pub const SZP_CURRENT_TIME: ESequencerZoomPosition = ESequencerZoomPosition(0);
    pub const SZP_MOUSE_POSITION: ESequencerZoomPosition = ESequencerZoomPosition(1);
}
#[repr(transparent)]
pub struct ESequencerTimeWarpDisplay(pub u8);
impl ESequencerTimeWarpDisplay {
    pub const UNWARPED_TIME: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(1);
    pub const WARPED_TIME: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(2);
    pub const BOTH: ESequencerTimeWarpDisplay = ESequencerTimeWarpDisplay(3);
}
