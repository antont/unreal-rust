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
pub static mut U_MOVIE_SCENE_CAPTURE_PROTOCOL_BASE_IS_CAPTURING: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_PROTOCOL_BASE_GET_STATE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_SET_IMAGE_CAPTURE_PROTOCOL_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_SET_AUDIO_CAPTURE_PROTOCOL_TYPE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_GET_IMAGE_CAPTURE_PROTOCOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_GET_AUDIO_CAPTURE_PROTOCOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_IS_CAPTURE_IN_PROGRESS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_GET_CAPTURE_FRAME_NUMBER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_GET_CAPTURE_ELAPSED_TIME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_FIND_IMAGE_CAPTURE_PROTOCOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_FIND_AUDIO_CAPTURE_PROTOCOL: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_STOP_CAPTURING_FINAL_PIXELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_START_CAPTURING_FINAL_PIXELS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_RESOLVE_BUFFER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_WARM_UP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_START_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_SETUP: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PRE_TICK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PIXELS_RECEIVED: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PAUSE_CAPTURE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_FINALIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_CAPTURE_FRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_CAN_FINALIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_BEGIN_FINALIZE: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_GET_CURRENT_FRAME_METRICS: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_CAPTURE_PROTOCOL_GENERATE_FILENAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_WRITE_IMAGE_TO_DISK: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_GENERATE_FILENAME_FOR_CURRENT_FRAME: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
#[doc(hidden)]
pub static mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_GENERATE_FILENAME_FOR_BUFFER: *mut crate::ffi::UFunctionOpague = std::ptr::null_mut();
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneCaptureProtocolBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCapturing"),
            &raw mut U_MOVIE_SCENE_CAPTURE_PROTOCOL_BASE_IS_CAPTURING,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetState"),
            &raw mut U_MOVIE_SCENE_CAPTURE_PROTOCOL_BASE_GET_STATE,
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
            &raw mut U_MOVIE_SCENE_CAPTURE_SET_IMAGE_CAPTURE_PROTOCOL_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAudioCaptureProtocolType"),
            &raw mut U_MOVIE_SCENE_CAPTURE_SET_AUDIO_CAPTURE_PROTOCOL_TYPE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetImageCaptureProtocol"),
            &raw mut U_MOVIE_SCENE_CAPTURE_GET_IMAGE_CAPTURE_PROTOCOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAudioCaptureProtocol"),
            &raw mut U_MOVIE_SCENE_CAPTURE_GET_AUDIO_CAPTURE_PROTOCOL,
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
            &raw mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_IS_CAPTURE_IN_PROGRESS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCaptureFrameNumber"),
            &raw mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_GET_CAPTURE_FRAME_NUMBER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCaptureElapsedTime"),
            &raw mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_GET_CAPTURE_ELAPSED_TIME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindImageCaptureProtocol"),
            &raw mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_FIND_IMAGE_CAPTURE_PROTOCOL,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAudioCaptureProtocol"),
            &raw mut U_MOVIE_SCENE_CAPTURE_ENVIRONMENT_FIND_AUDIO_CAPTURE_PROTOCOL,
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
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_STOP_CAPTURING_FINAL_PIXELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartCapturingFinalPixels"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_START_CAPTURING_FINAL_PIXELS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveBuffer"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_RESOLVE_BUFFER,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnWarmUp"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_WARM_UP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTick"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnStartCapture"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_START_CAPTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSetup"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_SETUP,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPreTick"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PRE_TICK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPixelsReceived"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PIXELS_RECEIVED,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnPauseCapture"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_PAUSE_CAPTURE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnFinalize"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_FINALIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCaptureFrame"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_CAPTURE_FRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCanFinalize"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_CAN_FINALIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBeginFinalize"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_ON_BEGIN_FINALIZE,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurrentFrameMetrics"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_GET_CURRENT_FRAME_METRICS,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilename"),
            &raw mut U_USER_DEFINED_CAPTURE_PROTOCOL_GENERATE_FILENAME,
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
            &raw mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_WRITE_IMAGE_TO_DISK,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilenameForCurrentFrame"),
            &raw mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_GENERATE_FILENAME_FOR_CURRENT_FRAME,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GenerateFilenameForBuffer"),
            &raw mut U_USER_DEFINED_IMAGE_CAPTURE_PROTOCOL_GENERATE_FILENAME_FOR_BUFFER,
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
    __padding_end: [u8; 4],
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
    __padding_end: [u8; 4],
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
