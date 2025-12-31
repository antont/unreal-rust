#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FCaptureResolution {
    pub res_x: i32,
    pub res_y: i32,
}
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
    pub post_processing_material: crate::bindings::core_u_object::FSoftObjectPath,
    pub b_disable_screen_percentage: bool,
    pub post_processing_material_ptr: UPtr<crate::bindings::engine::UMaterialInterface>,
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
    pub image_capture_protocol_type: crate::bindings::core_u_object::FSoftClassPath,
    pub audio_capture_protocol_type: crate::bindings::core_u_object::FSoftClassPath,
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
    pub prerequisite_actor_id: crate::bindings::core_u_object::FGuid,
}
pub struct UMovieSceneCaptureEnvironment {}
pub struct UUserDefinedCaptureProtocol {
    pub world: UPtr<crate::bindings::engine::UWorld>,
}
pub struct UUserDefinedImageCaptureProtocol {
    pub format: crate::bindings::image_write_queue::EDesiredImageFormat,
    pub b_enable_compression: bool,
    pub compression_quality: i32,
}
pub struct UVideoCaptureProtocol {
    pub b_use_compression: bool,
    pub compression_quality: f32,
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
