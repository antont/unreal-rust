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
impl UMovieSceneCaptureProtocolBase {}
#[repr(C, align(8))]
pub struct UMovieSceneAudioCaptureProtocolBase {
    __padding_end: [u8; 96],
}
impl UMovieSceneAudioCaptureProtocolBase {}
#[repr(C, align(8))]
pub struct UNullAudioCaptureProtocol {
    __padding_end: [u8; 96],
}
impl UNullAudioCaptureProtocol {}
#[repr(C, align(8))]
pub struct UMasterAudioSubmixCaptureProtocol {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub file_name: FString,
    __padding_end: [u8; 40],
}
impl UMasterAudioSubmixCaptureProtocol {}
pub struct IMovieSceneCaptureInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneCaptureInterface {
    __padding_end: [u8; 48],
}
impl UMovieSceneCaptureInterface {}
#[repr(C, align(8))]
pub struct UMovieSceneImageCaptureProtocolBase {
    __padding_end: [u8; 96],
}
impl UMovieSceneImageCaptureProtocolBase {}
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
impl UCompositionGraphCaptureProtocol {}
#[repr(C, align(8))]
pub struct UFrameGrabberProtocol {
    __padding_end: [u8; 112],
}
impl UFrameGrabberProtocol {}
#[repr(C, align(8))]
pub struct UImageSequenceProtocol {
    __padding_end: [u8; 224],
}
impl UImageSequenceProtocol {}
#[repr(C, align(8))]
pub struct UCompressedImageSequenceProtocol {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub compression_quality: i32,
    __padding_end: [u8; 4],
}
impl UCompressedImageSequenceProtocol {}
#[repr(C, align(8))]
pub struct UImageSequenceProtocol_BMP {
    __padding_end: [u8; 224],
}
impl UImageSequenceProtocol_BMP {}
#[repr(C, align(8))]
pub struct UImageSequenceProtocol_PNG {
    __padding_end: [u8; 232],
}
impl UImageSequenceProtocol_PNG {}
#[repr(C, align(8))]
pub struct UImageSequenceProtocol_JPG {
    __padding_end: [u8; 232],
}
impl UImageSequenceProtocol_JPG {}
#[repr(C, align(8))]
pub struct UImageSequenceProtocol_EXR {
    __padding_end: [u8; 240],
}
impl UImageSequenceProtocol_EXR {}
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
impl UMovieSceneCapture {}
#[repr(C, align(8))]
pub struct ULevelCapture {
    __padding_end: [u8; 632],
}
impl ULevelCapture {}
#[repr(C, align(8))]
pub struct UMovieSceneCaptureEnvironment {
    __padding_end: [u8; 48],
}
impl UMovieSceneCaptureEnvironment {}
#[repr(C, align(8))]
pub struct UUserDefinedCaptureProtocol {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub world: UPtr<crate::bindings::engine::UWorld>,
    __padding_end: [u8; 120],
}
impl UUserDefinedCaptureProtocol {}
#[repr(C, align(8))]
pub struct UUserDefinedImageCaptureProtocol {
    #[doc(hidden)]
    __padding_224: [u8; 224],
    pub format: crate::bindings::image_write_queue::EDesiredImageFormat,
    pub b_enable_compression: bool,
    pub compression_quality: i32,
}
impl UUserDefinedImageCaptureProtocol {}
#[repr(C, align(8))]
pub struct UVideoCaptureProtocol {
    __padding_end: [u8; 136],
}
impl UVideoCaptureProtocol {}
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
