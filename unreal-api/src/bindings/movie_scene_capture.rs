#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FCaptureResolution {
    pub res_x: i32,
    pub res_y: i32,
}
#[repr(C, align(8))]
pub struct FMovieSceneCaptureSettings {
    pub output_directory: FDirectoryPath,
    pub game_mode_override: TSubclassOf<AGameModeBase>,
    pub output_format: FString,
    pub b_overwrite_existing: bool,
    pub b_use_relative_frame_numbers: bool,
    pub handle_frames: i32,
    pub movie_extension: FString,
    pub zero_pad_frame_numbers: u8,
    pub frame_rate: FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: FFrameRate,
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
#[repr(C, align(8))]
pub struct FCompositionGraphCapturePasses {
    pub value: TArray<FString>,
}
#[repr(C, align(4))]
pub struct FFrameMetrics {
    pub total_elapsed_time: f32,
    pub frame_delta: f32,
    pub frame_number: i32,
    pub num_dropped_frames: i32,
}
#[repr(C, align(8))]
pub struct FCapturedPixelsID {
    pub identifiers: TMap<FName, FName>,
}
#[repr(C, align(8))]
pub struct FCapturedPixels {}
pub struct UMovieSceneCaptureProtocolBase {
    pub state: EMovieSceneCaptureProtocolState,
}
pub struct UMovieSceneAudioCaptureProtocolBase {}
pub struct UNullAudioCaptureProtocol {}
pub struct UMasterAudioSubmixCaptureProtocol {
    pub file_name: FString,
}
pub struct UMovieSceneCaptureInterface {}
pub struct IMovieSceneCaptureInterface {}
pub struct UMovieSceneImageCaptureProtocolBase {}
pub struct UCompositionGraphCaptureProtocol {
    pub include_render_passes: FCompositionGraphCapturePasses,
    pub b_capture_frames_in_hdr: bool,
    pub hdr_compression_quality: i32,
    pub capture_gamut: EHDRCaptureGamut,
    pub post_processing_material: FSoftObjectPath,
    pub b_disable_screen_percentage: bool,
    pub post_processing_material_ptr: UPtr<UMaterialInterface>,
}
pub struct UFrameGrabberProtocol {}
pub struct UImageSequenceProtocol {}
pub struct UCompressedImageSequenceProtocol {
    pub compression_quality: i32,
}
pub struct UImageSequenceProtocol_BMP {}
pub struct UImageSequenceProtocol_PNG {}
pub struct UImageSequenceProtocol_JPG {}
pub struct UImageSequenceProtocol_EXR {
    pub b_compressed: bool,
    pub capture_gamut: EHDRCaptureGamut,
}
pub struct UMovieSceneCapture {
    pub image_capture_protocol_type: FSoftClassPath,
    pub audio_capture_protocol_type: FSoftClassPath,
    pub image_capture_protocol: UPtr<UMovieSceneImageCaptureProtocolBase>,
    pub audio_capture_protocol: UPtr<UMovieSceneAudioCaptureProtocolBase>,
    pub settings: FMovieSceneCaptureSettings,
    pub b_use_separate_process: bool,
    pub b_close_editor_when_capture_starts: bool,
    pub additional_command_line_arguments: FString,
    pub inherited_command_line_arguments: FString,
}
pub struct ULevelCapture {
    pub b_auto_start_capture: bool,
    pub prerequisite_actor_id: FGuid,
}
pub struct UMovieSceneCaptureEnvironment {}
pub struct UUserDefinedCaptureProtocol {
    pub world: UPtr<UWorld>,
}
pub struct UUserDefinedImageCaptureProtocol {
    pub format: EDesiredImageFormat,
    pub b_enable_compression: bool,
    pub compression_quality: i32,
}
pub struct UVideoCaptureProtocol {
    pub b_use_compression: bool,
    pub compression_quality: f32,
}
