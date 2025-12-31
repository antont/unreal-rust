#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FAudioInputDeviceInfo {
    pub device_name: FString,
    pub device_id: FString,
    pub input_channels: i32,
    pub preferred_sample_rate: i32,
    pub flags_40: u8,
}
#[repr(C, align(4))]
pub struct FAudioCaptureDeviceInfo {
    pub device_name: FName,
    pub num_input_channels: i32,
    pub sample_rate: i32,
}
pub struct UAudioCapture {}
pub struct UAudioCaptureFunctionLibrary {}
pub struct UAudioCaptureBlueprintLibrary {}
pub struct UAudioCaptureComponent {
    pub jitter_latency_frames: i32,
}
pub struct FGetAvailableAudioInputDevices_OnObtainDevicesEvent;
