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
    pub u_movie_scene_director_blueprint_condition_endpoint_util_sample_director_blueprint_condition: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_dynamic_binding_endpoint_util_sample_resolve_binding: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_user_import_fbx_control_rig_settings_load_control_mappings_from_preset: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_user_export_fbx_control_rig_settings_load_control_mappings_from_preset: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_movie_scene_director_blueprint_condition_endpoint_util_sample_director_blueprint_condition: std::ptr::null_mut(),
            u_movie_scene_dynamic_binding_endpoint_util_sample_resolve_binding: std::ptr::null_mut(),
            u_movie_scene_user_import_fbx_control_rig_settings_load_control_mappings_from_preset: std::ptr::null_mut(),
            u_movie_scene_user_export_fbx_control_rig_settings_load_control_mappings_from_preset: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneDirectorBlueprintConditionEndpointUtil::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SampleDirectorBlueprintCondition"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_director_blueprint_condition_endpoint_util_sample_director_blueprint_condition,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneDynamicBindingEndpointUtil::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SampleResolveBinding"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_dynamic_binding_endpoint_util_sample_resolve_binding,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneUserImportFBXControlRigSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadControlMappingsFromPreset"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_user_import_fbx_control_rig_settings_load_control_mappings_from_preset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneUserExportFBXControlRigSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadControlMappingsFromPreset"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_user_export_fbx_control_rig_settings_load_control_mappings_from_preset,
        );
    }
}
#[repr(C, align(4))]
pub struct FBakingAnimationKeySettings {
    pub start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub baking_key_settings: EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
    pub b_time_warp: bool,
}
impl FBakingAnimationKeySettings {}
#[repr(C, align(1))]
pub struct FControlToTransformMappings {
    __padding_end: [u8; 3],
}
impl FControlToTransformMappings {}
#[repr(C, align(8))]
pub struct FControlFindReplaceString {
    __padding_end: [u8; 32],
}
impl FControlFindReplaceString {}
#[repr(C, align(8))]
pub struct UMovieSceneTextKeyStruct {
    __padding_end: [u8; 416],
}
impl UMovieSceneTextKeyStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTextKeyStruct")
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
pub struct USequencerExportTask {
    #[doc(hidden)]
    __padding_128: [u8; 128],
    pub sequencer_context: UPtr<crate::bindings::core_u_object::UObject>,
}
impl USequencerExportTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerExportTask")
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
pub struct UAutomatedLevelSequenceCapture {
    #[doc(hidden)]
    __padding_600: [u8; 600],
    pub level_sequence_asset: crate::bindings::core_u_object::FSoftObjectPath,
    pub shot_name: FString,
    pub b_use_custom_start_frame: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub b_use_custom_end_frame: bool,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub warm_up_frame_count: i32,
    pub delay_before_warm_up: f32,
    pub delay_before_shot_warm_up: f32,
    pub delay_every_frame: f32,
    pub burn_in_options: UPtr<
        crate::bindings::level_sequence::ULevelSequenceBurnInOptions,
    >,
    pub b_write_edit_decision_list: bool,
    pub b_write_final_cut_pro_xml: bool,
    __padding_end: [u8; 326],
}
impl UAutomatedLevelSequenceCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomatedLevelSequenceCapture")
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
pub struct UBoolChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UBoolChannelKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoolChannelKeyProxy")
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
pub struct UByteChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UByteChannelKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UByteChannelKeyProxy")
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
pub struct UDoubleChannelKeyProxy {
    __padding_end: [u8; 152],
}
impl UDoubleChannelKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleChannelKeyProxy")
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
pub struct UFloatChannelKeyProxy {
    __padding_end: [u8; 144],
}
impl UFloatChannelKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatChannelKeyProxy")
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
pub struct UIntegerChannelKeyProxy {
    __padding_end: [u8; 120],
}
impl UIntegerChannelKeyProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIntegerChannelKeyProxy")
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
pub struct UMovieSceneTrackRowMetadataHelper {
    __padding_end: [u8; 64],
}
impl UMovieSceneTrackRowMetadataHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackRowMetadataHelper")
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
pub struct UMovieSceneDirectorBlueprintConditionExtension {
    __padding_end: [u8; 64],
}
impl UMovieSceneDirectorBlueprintConditionExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDirectorBlueprintConditionExtension")
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
pub struct UMovieSceneDirectorBlueprintConditionEndpointUtil {
    __padding_end: [u8; 48],
}
impl UMovieSceneDirectorBlueprintConditionEndpointUtil {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDirectorBlueprintConditionEndpointUtil")
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
pub struct UK2Node_GetSequenceBinding {
    __padding_end: [u8; 672],
}
impl UK2Node_GetSequenceBinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_GetSequenceBinding")
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
pub struct UMovieSceneDynamicBindingBlueprintExtension {
    __padding_end: [u8; 72],
}
impl UMovieSceneDynamicBindingBlueprintExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDynamicBindingBlueprintExtension")
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
pub struct UMovieSceneDynamicBindingEndpointUtil {
    __padding_end: [u8; 48],
}
impl UMovieSceneDynamicBindingEndpointUtil {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDynamicBindingEndpointUtil")
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
pub struct UMovieSceneEventBlueprintExtension {
    __padding_end: [u8; 64],
}
impl UMovieSceneEventBlueprintExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEventBlueprintExtension")
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
pub struct UMovieSceneToolsProjectSettings {
    __padding_end: [u8; 176],
}
impl UMovieSceneToolsProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneToolsProjectSettings")
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
pub struct UMovieSceneUserThumbnailSettings {
    __padding_end: [u8; 152],
}
impl UMovieSceneUserThumbnailSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneUserThumbnailSettings")
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
pub struct UMovieSceneUserImportFBXSettings {
    __padding_end: [u8; 64],
}
impl UMovieSceneUserImportFBXSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneUserImportFBXSettings")
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
pub struct UMovieSceneUserImportFBXControlRigSettings {
    __padding_end: [u8; 168],
}
impl UMovieSceneUserImportFBXControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneUserImportFBXControlRigSettings")
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
    pub fn load_control_mappings_from_preset(&mut self, b_meta_human_preset: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_tools::__FUNCTION_PTRS
                    .u_movie_scene_user_import_fbx_control_rig_settings_load_control_mappings_from_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_meta_human_preset,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_tools::__FUNCTION_PTRS
                    .u_movie_scene_user_import_fbx_control_rig_settings_load_control_mappings_from_preset,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneUserExportFBXControlRigSettings {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub fbx_export_compatibility: crate::bindings::unreal_ed::EFbxExportCompatibility,
    #[doc(hidden)]
    __padding_68: [u8; 3],
    pub flags_68: u8,
    #[doc(hidden)]
    __padding_96: [u8; 27],
    pub flags_96: u8,
}
impl UMovieSceneUserExportFBXControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneUserExportFBXControlRigSettings")
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
    pub fn load_control_mappings_from_preset(&mut self, b_meta_human_preset: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_tools::__FUNCTION_PTRS
                    .u_movie_scene_user_export_fbx_control_rig_settings_load_control_mappings_from_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_meta_human_preset,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_tools::__FUNCTION_PTRS
                    .u_movie_scene_user_export_fbx_control_rig_settings_load_control_mappings_from_preset,
                __buffer,
            )
        };
    }
}
#[repr(transparent)]
pub struct EBakingKeySettings(pub u8);
impl EBakingKeySettings {
    pub const KEYS_ONLY: EBakingKeySettings = EBakingKeySettings(0);
    pub const ALL_FRAMES: EBakingKeySettings = EBakingKeySettings(1);
}
#[repr(transparent)]
pub struct ECreationTime(pub u8);
impl ECreationTime {
    pub const CURRENT_FRAME: ECreationTime = ECreationTime(0);
    pub const FROM_START: ECreationTime = ECreationTime(1);
    pub const INFINITE: ECreationTime = ECreationTime(2);
}
#[repr(transparent)]
pub struct EMovieSceneToolsPropertyTrackType(pub i32);
impl EMovieSceneToolsPropertyTrackType {
    pub const FLOAT_TRACK: EMovieSceneToolsPropertyTrackType = EMovieSceneToolsPropertyTrackType(
        0,
    );
    pub const DOUBLE_TRACK: EMovieSceneToolsPropertyTrackType = EMovieSceneToolsPropertyTrackType(
        1,
    );
}
#[repr(transparent)]
pub struct FControlRigChannelEnum(pub u8);
impl FControlRigChannelEnum {
    pub const BOOL: FControlRigChannelEnum = FControlRigChannelEnum(0);
    pub const ENUM: FControlRigChannelEnum = FControlRigChannelEnum(1);
    pub const INTEGER: FControlRigChannelEnum = FControlRigChannelEnum(2);
    pub const FLOAT: FControlRigChannelEnum = FControlRigChannelEnum(3);
    pub const VECTOR2_DX: FControlRigChannelEnum = FControlRigChannelEnum(4);
    pub const VECTOR2_DY: FControlRigChannelEnum = FControlRigChannelEnum(5);
    pub const POSITION_X: FControlRigChannelEnum = FControlRigChannelEnum(6);
    pub const POSITION_Y: FControlRigChannelEnum = FControlRigChannelEnum(7);
    pub const POSITION_Z: FControlRigChannelEnum = FControlRigChannelEnum(8);
    pub const ROTATOR_X: FControlRigChannelEnum = FControlRigChannelEnum(9);
    pub const ROTATOR_Y: FControlRigChannelEnum = FControlRigChannelEnum(10);
    pub const ROTATOR_Z: FControlRigChannelEnum = FControlRigChannelEnum(11);
    pub const SCALE_X: FControlRigChannelEnum = FControlRigChannelEnum(12);
    pub const SCALE_Y: FControlRigChannelEnum = FControlRigChannelEnum(13);
    pub const SCALE_Z: FControlRigChannelEnum = FControlRigChannelEnum(14);
}
#[repr(transparent)]
pub struct FTransformChannelEnum(pub u8);
impl FTransformChannelEnum {
    pub const TRANSLATE_X: FTransformChannelEnum = FTransformChannelEnum(0);
    pub const TRANSLATE_Y: FTransformChannelEnum = FTransformChannelEnum(1);
    pub const TRANSLATE_Z: FTransformChannelEnum = FTransformChannelEnum(2);
    pub const ROTATE_X: FTransformChannelEnum = FTransformChannelEnum(3);
    pub const ROTATE_Y: FTransformChannelEnum = FTransformChannelEnum(4);
    pub const ROTATE_Z: FTransformChannelEnum = FTransformChannelEnum(5);
    pub const SCALE_X: FTransformChannelEnum = FTransformChannelEnum(6);
    pub const SCALE_Y: FTransformChannelEnum = FTransformChannelEnum(7);
    pub const SCALE_Z: FTransformChannelEnum = FTransformChannelEnum(8);
}
#[repr(transparent)]
pub struct EThumbnailQuality(pub u8);
impl EThumbnailQuality {
    pub const DRAFT: EThumbnailQuality = EThumbnailQuality(0);
    pub const NORMAL: EThumbnailQuality = EThumbnailQuality(1);
    pub const BEST: EThumbnailQuality = EThumbnailQuality(2);
}
