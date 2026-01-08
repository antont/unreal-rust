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
    pub u_movie_scene_capture_protocol_base_is_capturing: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_protocol_base_get_state: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_set_image_capture_protocol_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_set_audio_capture_protocol_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_get_image_capture_protocol: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_get_audio_capture_protocol: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_environment_is_capture_in_progress: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_environment_get_capture_frame_number: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_environment_get_capture_elapsed_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_environment_find_image_capture_protocol: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_capture_environment_find_audio_capture_protocol: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_stop_capturing_final_pixels: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_start_capturing_final_pixels: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_resolve_buffer: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_warm_up: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_tick: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_start_capture: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_setup: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_pre_tick: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_pixels_received: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_pause_capture: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_capture_frame: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_can_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_on_begin_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_get_current_frame_metrics: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_capture_protocol_generate_filename: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_image_capture_protocol_write_image_to_disk: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_image_capture_protocol_generate_filename_for_current_frame: *mut crate::ffi::UFunctionOpague,
    pub u_user_defined_image_capture_protocol_generate_filename_for_buffer: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_movie_scene_capture_protocol_base_is_capturing: std::ptr::null_mut(),
            u_movie_scene_capture_protocol_base_get_state: std::ptr::null_mut(),
            u_movie_scene_capture_set_image_capture_protocol_type: std::ptr::null_mut(),
            u_movie_scene_capture_set_audio_capture_protocol_type: std::ptr::null_mut(),
            u_movie_scene_capture_get_image_capture_protocol: std::ptr::null_mut(),
            u_movie_scene_capture_get_audio_capture_protocol: std::ptr::null_mut(),
            u_movie_scene_capture_environment_is_capture_in_progress: std::ptr::null_mut(),
            u_movie_scene_capture_environment_get_capture_frame_number: std::ptr::null_mut(),
            u_movie_scene_capture_environment_get_capture_elapsed_time: std::ptr::null_mut(),
            u_movie_scene_capture_environment_find_image_capture_protocol: std::ptr::null_mut(),
            u_movie_scene_capture_environment_find_audio_capture_protocol: std::ptr::null_mut(),
            u_user_defined_capture_protocol_stop_capturing_final_pixels: std::ptr::null_mut(),
            u_user_defined_capture_protocol_start_capturing_final_pixels: std::ptr::null_mut(),
            u_user_defined_capture_protocol_resolve_buffer: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_warm_up: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_tick: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_start_capture: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_setup: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_pre_tick: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_pixels_received: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_pause_capture: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_finalize: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_capture_frame: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_can_finalize: std::ptr::null_mut(),
            u_user_defined_capture_protocol_on_begin_finalize: std::ptr::null_mut(),
            u_user_defined_capture_protocol_get_current_frame_metrics: std::ptr::null_mut(),
            u_user_defined_capture_protocol_generate_filename: std::ptr::null_mut(),
            u_user_defined_image_capture_protocol_write_image_to_disk: std::ptr::null_mut(),
            u_user_defined_image_capture_protocol_generate_filename_for_current_frame: std::ptr::null_mut(),
            u_user_defined_image_capture_protocol_generate_filename_for_buffer: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCaptureProtocolBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCapturing"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_capture_protocol_base_is_capturing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_capture_protocol_base_get_state,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCapture::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetImageCaptureProtocolType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_set_image_capture_protocol_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioCaptureProtocolType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_set_audio_capture_protocol_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetImageCaptureProtocol"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_capture_get_image_capture_protocol,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioCaptureProtocol"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_capture_get_audio_capture_protocol,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCaptureEnvironment::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCaptureInProgress"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_environment_is_capture_in_progress,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCaptureFrameNumber"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_environment_get_capture_frame_number,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCaptureElapsedTime"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_environment_get_capture_elapsed_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindImageCaptureProtocol"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_environment_find_image_capture_protocol,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAudioCaptureProtocol"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_capture_environment_find_audio_capture_protocol,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserDefinedCaptureProtocol::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopCapturingFinalPixels"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_capture_protocol_stop_capturing_final_pixels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCapturingFinalPixels"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_capture_protocol_start_capturing_final_pixels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveBuffer"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_resolve_buffer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnWarmUp"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_warm_up,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTick"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStartCapture"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_start_capture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSetup"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_setup,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreTick"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_pre_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPixelsReceived"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_pixels_received,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPauseCapture"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_pause_capture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFinalize"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_finalize,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCaptureFrame"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_capture_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCanFinalize"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_can_finalize,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBeginFinalize"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_on_begin_finalize,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentFrameMetrics"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_capture_protocol_get_current_frame_metrics,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilename"),
            &raw mut __FUNCTION_PTRS.u_user_defined_capture_protocol_generate_filename,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUserDefinedImageCaptureProtocol::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteImageToDisk"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_image_capture_protocol_write_image_to_disk,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilenameForCurrentFrame"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_image_capture_protocol_generate_filename_for_current_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilenameForBuffer"),
            &raw mut __FUNCTION_PTRS
                .u_user_defined_image_capture_protocol_generate_filename_for_buffer,
        );
    }
}
#[repr(C, align(4))]
pub struct FCaptureResolution {
    pub res_x: i32,
    pub res_y: i32,
}
impl FCaptureResolution {}
#[repr(C, align(8))]
pub struct FMovieSceneCaptureSettings {
    pub output_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub game_mode_override: TSubclassOf<crate::bindings::engine::AGameModeBase>,
    pub output_format: FString,
    pub b_overwrite_existing: bool,
    pub b_use_relative_frame_numbers: bool,
    pub handle_frames: i32,
    pub movie_extension: FString,
    pub zero_pad_frame_numbers: u8,
    pub frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
    pub resolution: FCaptureResolution,
    pub b_enable_texture_streaming: bool,
    pub b_cinematic_engine_scalability: bool,
    pub b_cinematic_mode: bool,
    pub b_allow_movement: bool,
    pub b_allow_turning: bool,
    pub b_show_player: bool,
    pub b_show_hud: bool,
    pub b_use_path_tracer: bool,
    pub path_tracer_sample_per_pixel: i32,
}
impl FMovieSceneCaptureSettings {}
#[repr(C, align(8))]
pub struct FCompositionGraphCapturePasses {
    pub value: TArray<FString>,
}
impl FCompositionGraphCapturePasses {}
#[repr(C, align(4))]
pub struct FFrameMetrics {
    pub total_elapsed_time: f32,
    pub frame_delta: f32,
    pub frame_number: i32,
    pub num_dropped_frames: i32,
}
impl FFrameMetrics {}
#[repr(C, align(8))]
pub struct FCapturedPixelsID {
    pub identifiers: TMap<FName, FName>,
}
impl FCapturedPixelsID {}
#[repr(C, align(8))]
pub struct FCapturedPixels {
    __padding_end: [u8; 16],
}
impl FCapturedPixels {}
#[repr(C, align(8))]
pub struct UMovieSceneCaptureProtocolBase {
    __padding_end: [u8; 96],
}
impl UMovieSceneCaptureProtocolBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCaptureProtocolBase")
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
    pub fn is_capturing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_protocol_base_is_capturing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_protocol_base_is_capturing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_state(&self) -> EMovieSceneCaptureProtocolState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_protocol_base_get_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_protocol_base_get_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EMovieSceneCaptureProtocolState>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneAudioCaptureProtocolBase {
    __padding_end: [u8; 96],
}
impl UMovieSceneAudioCaptureProtocolBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneAudioCaptureProtocolBase")
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
pub struct UNullAudioCaptureProtocol {
    __padding_end: [u8; 96],
}
impl UNullAudioCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNullAudioCaptureProtocol")
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
pub struct UMasterAudioSubmixCaptureProtocol {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub file_name: FString,
    __padding_end: [u8; 40],
}
impl UMasterAudioSubmixCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMasterAudioSubmixCaptureProtocol")
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
pub struct IMovieSceneCaptureInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneCaptureInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneCaptureInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCaptureInterface")
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
pub struct UMovieSceneImageCaptureProtocolBase {
    __padding_end: [u8; 96],
}
impl UMovieSceneImageCaptureProtocolBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneImageCaptureProtocolBase")
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
pub struct UCompositionGraphCaptureProtocol {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub include_render_passes: FCompositionGraphCapturePasses,
    pub b_capture_frames_in_hdr: bool,
    pub hdr_compression_quality: i32,
    pub capture_gamut: EHDRCaptureGamut,
    pub post_processing_material: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_disable_screen_percentage: bool,
    __padding_end: [u8; 47],
}
impl UCompositionGraphCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositionGraphCaptureProtocol")
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
pub struct UFrameGrabberProtocol {
    __padding_end: [u8; 112],
}
impl UFrameGrabberProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFrameGrabberProtocol")
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
pub struct UImageSequenceProtocol {
    __padding_end: [u8; 224],
}
impl UImageSequenceProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageSequenceProtocol")
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
pub struct UCompressedImageSequenceProtocol {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub compression_quality: i32,
}
impl UCompressedImageSequenceProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompressedImageSequenceProtocol")
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
pub struct UImageSequenceProtocol_BMP {
    __padding_end: [u8; 224],
}
impl UImageSequenceProtocol_BMP {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageSequenceProtocol_BMP")
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
pub struct UImageSequenceProtocol_PNG {
    __padding_end: [u8; 232],
}
impl UImageSequenceProtocol_PNG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageSequenceProtocol_PNG")
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
pub struct UImageSequenceProtocol_JPG {
    __padding_end: [u8; 232],
}
impl UImageSequenceProtocol_JPG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageSequenceProtocol_JPG")
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
pub struct UImageSequenceProtocol_EXR {
    __padding_end: [u8; 240],
}
impl UImageSequenceProtocol_EXR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImageSequenceProtocol_EXR")
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
pub struct UMovieSceneCapture {
    #[doc(hidden)]
    __padding_160: [u8; 160],
    pub settings: FMovieSceneCaptureSettings,
    pub b_use_separate_process: bool,
    pub b_close_editor_when_capture_starts: bool,
    pub additional_command_line_arguments: FString,
    pub inherited_command_line_arguments: FString,
    __padding_end: [u8; 288],
}
impl UMovieSceneCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCapture")
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
    pub fn set_image_capture_protocol_type(
        &mut self,
        protocol_type: TSubclassOf<UMovieSceneCaptureProtocolBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_set_image_capture_protocol_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &protocol_type,
                __buffer.add(0).cast::<TSubclassOf<UMovieSceneCaptureProtocolBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_set_image_capture_protocol_type,
                __buffer,
            )
        };
    }
    pub fn set_audio_capture_protocol_type(
        &mut self,
        protocol_type: TSubclassOf<UMovieSceneCaptureProtocolBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_set_audio_capture_protocol_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &protocol_type,
                __buffer.add(0).cast::<TSubclassOf<UMovieSceneCaptureProtocolBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_set_audio_capture_protocol_type,
                __buffer,
            )
        };
    }
    pub fn get_image_capture_protocol(
        &mut self,
    ) -> UPtr<UMovieSceneCaptureProtocolBase> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_get_image_capture_protocol,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_get_image_capture_protocol,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMovieSceneCaptureProtocolBase>>().read() }
    }
    pub fn get_audio_capture_protocol(
        &mut self,
    ) -> UPtr<UMovieSceneCaptureProtocolBase> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_get_audio_capture_protocol,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_get_audio_capture_protocol,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMovieSceneCaptureProtocolBase>>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULevelCapture {
    __padding_end: [u8; 632],
}
impl ULevelCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelCapture")
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
pub struct UMovieSceneCaptureEnvironment {
    __padding_end: [u8; 48],
}
impl UMovieSceneCaptureEnvironment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCaptureEnvironment")
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
    pub fn is_capture_in_progress() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_is_capture_in_progress,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene_capture::UMovieSceneCaptureEnvironment::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_is_capture_in_progress,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_capture_frame_number() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_get_capture_frame_number,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene_capture::UMovieSceneCaptureEnvironment::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_get_capture_frame_number,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_capture_elapsed_time() -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_get_capture_elapsed_time,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene_capture::UMovieSceneCaptureEnvironment::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_get_capture_elapsed_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn find_image_capture_protocol() -> UPtr<UMovieSceneImageCaptureProtocolBase> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_find_image_capture_protocol,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene_capture::UMovieSceneCaptureEnvironment::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_find_image_capture_protocol,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<UMovieSceneImageCaptureProtocolBase>>().read()
        }
    }
    pub fn find_audio_capture_protocol() -> UPtr<UMovieSceneAudioCaptureProtocolBase> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_find_audio_capture_protocol,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::movie_scene_capture::UMovieSceneCaptureEnvironment::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_movie_scene_capture_environment_find_audio_capture_protocol,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<UMovieSceneAudioCaptureProtocolBase>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UUserDefinedCaptureProtocol {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub world: UPtr<crate::bindings::engine::UWorld>,
    __padding_end: [u8; 120],
}
impl UUserDefinedCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserDefinedCaptureProtocol")
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
    pub fn stop_capturing_final_pixels(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_stop_capturing_final_pixels,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_stop_capturing_final_pixels,
                __buffer,
            )
        };
    }
    pub fn start_capturing_final_pixels(&mut self, stream_id: &FCapturedPixelsID) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_start_capturing_final_pixels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                stream_id,
                __buffer.add(0).cast::<FCapturedPixelsID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_start_capturing_final_pixels,
                __buffer,
            )
        };
    }
    pub fn resolve_buffer(
        &mut self,
        buffer: UPtr<crate::bindings::engine::UTexture>,
        buffer_id: &FCapturedPixelsID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_resolve_buffer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &buffer,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                buffer_id,
                __buffer.add(8).cast::<FCapturedPixelsID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_resolve_buffer,
                __buffer,
            )
        };
    }
    pub fn on_warm_up(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_warm_up,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_warm_up,
                __buffer,
            )
        };
    }
    pub fn on_tick(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_tick,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_tick,
                __buffer,
            )
        };
    }
    pub fn on_start_capture(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_start_capture,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_start_capture,
                __buffer,
            )
        };
    }
    pub fn on_setup(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_setup,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_setup,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn on_pre_tick(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pre_tick,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pre_tick,
                __buffer,
            )
        };
    }
    pub fn on_pixels_received(
        &mut self,
        pixels: &FCapturedPixels,
        id: &FCapturedPixelsID,
        frame_metrics: FFrameMetrics,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pixels_received,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pixels,
                __buffer.add(0).cast::<FCapturedPixels>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                id,
                __buffer.add(16).cast::<FCapturedPixelsID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_metrics,
                __buffer.add(96).cast::<FFrameMetrics>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pixels_received,
                __buffer,
            )
        };
    }
    pub fn on_pause_capture(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pause_capture,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_pause_capture,
                __buffer,
            )
        };
    }
    pub fn on_finalize(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_finalize,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_finalize,
                __buffer,
            )
        };
    }
    pub fn on_capture_frame(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_capture_frame,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_capture_frame,
                __buffer,
            )
        };
    }
    pub fn on_can_finalize(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_can_finalize,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_can_finalize,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn on_begin_finalize(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_begin_finalize,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_on_begin_finalize,
                __buffer,
            )
        };
    }
    pub fn get_current_frame_metrics(&self) -> FFrameMetrics {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_get_current_frame_metrics,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_get_current_frame_metrics,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FFrameMetrics>().read() }
    }
    pub fn generate_filename(&self, in_frame_metrics: &FFrameMetrics) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_generate_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_frame_metrics,
                __buffer.add(0).cast::<FFrameMetrics>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_capture_protocol_generate_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UUserDefinedImageCaptureProtocol {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub format: crate::bindings::image_write_queue::EDesiredImageFormat,
    pub b_enable_compression: bool,
    pub compression_quality: i32,
}
impl UUserDefinedImageCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserDefinedImageCaptureProtocol")
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
    pub fn write_image_to_disk(
        &mut self,
        pixel_data: &FCapturedPixels,
        stream_id: &FCapturedPixelsID,
        frame_metrics: &FFrameMetrics,
        b_copy_image_data: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_write_image_to_disk,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pixel_data,
                __buffer.add(0).cast::<FCapturedPixels>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                stream_id,
                __buffer.add(16).cast::<FCapturedPixelsID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frame_metrics,
                __buffer.add(96).cast::<FFrameMetrics>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_copy_image_data,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_write_image_to_disk,
                __buffer,
            )
        };
    }
    pub fn generate_filename_for_current_frame(&mut self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_generate_filename_for_current_frame,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_generate_filename_for_current_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn generate_filename_for_buffer(
        &mut self,
        buffer: UPtr<crate::bindings::engine::UTexture>,
        stream_id: &FCapturedPixelsID,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_generate_filename_for_buffer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &buffer,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                stream_id,
                __buffer.add(8).cast::<FCapturedPixelsID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::movie_scene_capture::__FUNCTION_PTRS
                    .u_user_defined_image_capture_protocol_generate_filename_for_buffer,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVideoCaptureProtocol {
    __padding_end: [u8; 136],
}
impl UVideoCaptureProtocol {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVideoCaptureProtocol")
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
pub struct EMovieSceneCaptureProtocolState(pub u8);
impl EMovieSceneCaptureProtocolState {
    pub const IDLE: EMovieSceneCaptureProtocolState = EMovieSceneCaptureProtocolState(0);
    pub const INITIALIZED: EMovieSceneCaptureProtocolState = EMovieSceneCaptureProtocolState(
        1,
    );
    pub const CAPTURING: EMovieSceneCaptureProtocolState = EMovieSceneCaptureProtocolState(
        2,
    );
    pub const FINALIZING: EMovieSceneCaptureProtocolState = EMovieSceneCaptureProtocolState(
        3,
    );
}
#[repr(transparent)]
pub struct EHDRCaptureGamut(pub u8);
impl EHDRCaptureGamut {
    pub const HCGM_REC709: EHDRCaptureGamut = EHDRCaptureGamut(0);
    pub const HCGM_P3DCI: EHDRCaptureGamut = EHDRCaptureGamut(1);
    pub const HCGM_REC2020: EHDRCaptureGamut = EHDRCaptureGamut(2);
    pub const HCGM_ACES: EHDRCaptureGamut = EHDRCaptureGamut(3);
    pub const HCGM_ACE_SCG: EHDRCaptureGamut = EHDRCaptureGamut(4);
    pub const HCGM_LINEAR: EHDRCaptureGamut = EHDRCaptureGamut(5);
}
