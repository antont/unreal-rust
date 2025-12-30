#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FWaveTableData {
    pub bit_depth: EWaveTableBitDepth,
    pub data: TArray<u8>,
    pub final_value: f32,
}
#[repr(C, align(8))]
pub struct FWaveTableBankEntry {
    pub transform: FWaveTableTransform,
}
#[repr(C, align(8))]
pub struct FWaveTableTransform {
    pub curve: EWaveTableCurve,
    pub scalar: f32,
    pub curve_custom: FRichCurve,
    pub curve_shared: UPtr<UCurveFloat>,
    pub table_data: FWaveTableData,
    pub duration: f32,
    pub wave_table_settings: FWaveTableSettings,
    pub wave_table: TArray<f32>,
    pub final_value: f32,
}
#[repr(C, align(8))]
pub struct FWaveTableSettings {
    pub file_path: FFilePath,
    pub channel_index: i32,
    pub source_data: FWaveTableData,
    pub source_sample_rate: i32,
    pub phase: f32,
    pub top: f32,
    pub tail: f32,
    pub fade_in: f32,
    pub fade_out: f32,
    pub b_normalize: bool,
    pub b_remove_offset: bool,
    pub source_pcm_data: TArray<f32>,
}
pub struct UWaveTableBank {
    pub sample_mode: EWaveTableSamplingMode,
    pub resolution: EWaveTableResolution,
    pub sample_rate: i32,
    pub b_bipolar: bool,
    pub wave_table_size_mb: f32,
    pub entries: TArray<FWaveTableBankEntry>,
}
