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
    pub u_constant_q_analyzer_get_num_center_frequencies: *mut crate::ffi::UFunctionOpague,
    pub u_constant_q_analyzer_get_center_frequencies: *mut crate::ffi::UFunctionOpague,
    pub u_constant_qnrt_get_normalized_channel_constant_q_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_constant_qnrt_get_channel_constant_q_at_time: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_loudness_data_for_channel_at_time: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_loudness_data_for_channel: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_loudness_data_at_time: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_loudness_data: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_integrated_loudness_for_channel: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_integrated_loudness: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_gated_loudness_for_channel: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_gated_loudness: *mut crate::ffi::UFunctionOpague,
    pub ulkfsnrt_get_channel_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_loudness_nrt_get_normalized_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_loudness_nrt_get_normalized_channel_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_loudness_nrt_get_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_loudness_nrt_get_channel_loudness_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_onset_nrt_get_normalized_channel_onsets_between_times: *mut crate::ffi::UFunctionOpague,
    pub u_onset_nrt_get_channel_onsets_between_times: *mut crate::ffi::UFunctionOpague,
    pub u_synesthesia_spectrum_analyzer_get_num_center_frequencies: *mut crate::ffi::UFunctionOpague,
    pub u_synesthesia_spectrum_analyzer_get_center_frequencies: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_constant_q_analyzer_get_num_center_frequencies: std::ptr::null_mut(),
            u_constant_q_analyzer_get_center_frequencies: std::ptr::null_mut(),
            u_constant_qnrt_get_normalized_channel_constant_q_at_time: std::ptr::null_mut(),
            u_constant_qnrt_get_channel_constant_q_at_time: std::ptr::null_mut(),
            ulkfsnrt_get_loudness_data_for_channel_at_time: std::ptr::null_mut(),
            ulkfsnrt_get_loudness_data_for_channel: std::ptr::null_mut(),
            ulkfsnrt_get_loudness_data_at_time: std::ptr::null_mut(),
            ulkfsnrt_get_loudness_data: std::ptr::null_mut(),
            ulkfsnrt_get_loudness_at_time: std::ptr::null_mut(),
            ulkfsnrt_get_integrated_loudness_for_channel: std::ptr::null_mut(),
            ulkfsnrt_get_integrated_loudness: std::ptr::null_mut(),
            ulkfsnrt_get_gated_loudness_for_channel: std::ptr::null_mut(),
            ulkfsnrt_get_gated_loudness: std::ptr::null_mut(),
            ulkfsnrt_get_channel_loudness_at_time: std::ptr::null_mut(),
            u_loudness_nrt_get_normalized_loudness_at_time: std::ptr::null_mut(),
            u_loudness_nrt_get_normalized_channel_loudness_at_time: std::ptr::null_mut(),
            u_loudness_nrt_get_loudness_at_time: std::ptr::null_mut(),
            u_loudness_nrt_get_channel_loudness_at_time: std::ptr::null_mut(),
            u_onset_nrt_get_normalized_channel_onsets_between_times: std::ptr::null_mut(),
            u_onset_nrt_get_channel_onsets_between_times: std::ptr::null_mut(),
            u_synesthesia_spectrum_analyzer_get_num_center_frequencies: std::ptr::null_mut(),
            u_synesthesia_spectrum_analyzer_get_center_frequencies: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UConstantQAnalyzer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumCenterFrequencies"),
            &raw mut __FUNCTION_PTRS.u_constant_q_analyzer_get_num_center_frequencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCenterFrequencies"),
            &raw mut __FUNCTION_PTRS.u_constant_q_analyzer_get_center_frequencies,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UConstantQNRT::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedChannelConstantQAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_constant_qnrt_get_normalized_channel_constant_q_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelConstantQAtTime"),
            &raw mut __FUNCTION_PTRS.u_constant_qnrt_get_channel_constant_q_at_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULKFSNRT::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessDataForChannelAtTime"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_loudness_data_for_channel_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessDataForChannel"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_loudness_data_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessDataAtTime"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_loudness_data_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessData"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_loudness_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_loudness_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntegratedLoudnessForChannel"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_integrated_loudness_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntegratedLoudness"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_integrated_loudness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGatedLoudnessForChannel"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_gated_loudness_for_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGatedLoudness"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_gated_loudness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS.ulkfsnrt_get_channel_loudness_at_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULoudnessNRT::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS.u_loudness_nrt_get_normalized_loudness_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedChannelLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_loudness_nrt_get_normalized_channel_loudness_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS.u_loudness_nrt_get_loudness_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelLoudnessAtTime"),
            &raw mut __FUNCTION_PTRS.u_loudness_nrt_get_channel_loudness_at_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UOnsetNRT::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNormalizedChannelOnsetsBetweenTimes"),
            &raw mut __FUNCTION_PTRS
                .u_onset_nrt_get_normalized_channel_onsets_between_times,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelOnsetsBetweenTimes"),
            &raw mut __FUNCTION_PTRS.u_onset_nrt_get_channel_onsets_between_times,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USynesthesiaSpectrumAnalyzer::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumCenterFrequencies"),
            &raw mut __FUNCTION_PTRS
                .u_synesthesia_spectrum_analyzer_get_num_center_frequencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCenterFrequencies"),
            &raw mut __FUNCTION_PTRS
                .u_synesthesia_spectrum_analyzer_get_center_frequencies,
        );
    }
}
#[repr(C, align(8))]
pub struct FConstantQResults {
    pub time_seconds: f32,
    pub spectrum_values: TArray<f32>,
}
impl FConstantQResults {}
#[repr(C, align(4))]
pub struct FLoudnessResults {
    pub loudness: f32,
    pub normalized_loudness: f32,
    pub perceptual_energy: f32,
    pub time_seconds: f32,
}
impl FLoudnessResults {}
#[repr(C, align(4))]
pub struct FMeterResults {
    pub time_seconds: f32,
    pub meter_value: f32,
    pub peak_value: f32,
    pub num_samples_clipping: i32,
    pub clipping_value: f32,
}
impl FMeterResults {}
#[repr(C, align(8))]
pub struct FSynesthesiaSpectrumResults {
    pub time_seconds: f32,
    pub spectrum_values: TArray<f32>,
}
impl FSynesthesiaSpectrumResults {}
#[repr(C, align(8))]
pub struct UAudioSynesthesiaSettings {
    __padding_end: [u8; 48],
}
impl UAudioSynesthesiaSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioSynesthesiaSettings")
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
pub struct UAudioSynesthesiaNRTSettings {
    __padding_end: [u8; 80],
}
impl UAudioSynesthesiaNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioSynesthesiaNRTSettings")
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
pub struct UAudioSynesthesiaNRT {
    __padding_end: [u8; 232],
}
impl UAudioSynesthesiaNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioSynesthesiaNRT")
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
pub struct UConstantQSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub starting_frequency_hz: f32,
    pub num_bands: i32,
    pub num_bands_per_octave: f32,
    pub analysis_period_in_seconds: f32,
    pub b_downmix_to_mono: bool,
    pub fft_size: EConstantQFFTSizeEnum,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub band_width_stretch: f32,
    pub cqt_normalization: EConstantQNormalizationEnum,
    pub noise_floor_db: f32,
}
impl UConstantQSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstantQSettings")
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
pub struct UConstantQAnalyzer {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: UPtr<UConstantQSettings>,
    __padding_end: [u8; 112],
}
impl UConstantQAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstantQAnalyzer")
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
    pub fn get_num_center_frequencies(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_q_analyzer_get_num_center_frequencies,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_q_analyzer_get_num_center_frequencies,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_center_frequencies(&mut self, out_center_frequencies: &mut TArray<f32>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_q_analyzer_get_center_frequencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_center_frequencies,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_q_analyzer_get_center_frequencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<f32>>().swap(out_center_frequencies);
        }
    }
}
#[repr(C, align(8))]
pub struct UConstantQNRTSettings {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub starting_frequency: f32,
    pub num_bands: i32,
    pub num_bands_per_octave: f32,
    pub analysis_period: f32,
    pub b_downmix_to_mono: bool,
    pub fft_size: EConstantQFFTSizeEnum,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub band_width_stretch: f32,
    pub cqt_normalization: EConstantQNormalizationEnum,
    pub noise_floor_db: f32,
}
impl UConstantQNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstantQNRTSettings")
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
pub struct UConstantQNRT {
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 232],
    pub settings: UPtr<UConstantQNRTSettings>,
}
impl UConstantQNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstantQNRT")
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
    pub fn get_normalized_channel_constant_q_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
        out_constant_q: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_qnrt_get_normalized_channel_constant_q_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_constant_q,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_qnrt_get_normalized_channel_constant_q_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<f32>>().swap(out_constant_q);
        }
    }
    pub fn get_channel_constant_q_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
        out_constant_q: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_qnrt_get_channel_constant_q_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_constant_q,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_constant_qnrt_get_channel_constant_q_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<f32>>().swap(out_constant_q);
        }
    }
}
#[repr(C, align(8))]
pub struct ULKFSSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
    pub integrated_loudness_analysis_period: f32,
    pub integrated_loudness_duration: f32,
}
impl ULKFSSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULKFSSettings")
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
pub struct ULKFSAnalyzer {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: UPtr<ULKFSSettings>,
    __padding_end: [u8; 224],
}
impl ULKFSAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULKFSAnalyzer")
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
pub struct ULKFSNRTSettings {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub analysis_period: f32,
    pub analysis_window_duration: f32,
    pub short_term_loudness_duration: f32,
}
impl ULKFSNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULKFSNRTSettings")
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
pub struct ULKFSNRT {
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 232],
    pub settings: UPtr<ULKFSNRTSettings>,
}
impl ULKFSNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("ULKFSNRT").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_loudness_data_for_channel_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
    ) -> crate::bindings::audio_synesthesia_core::FLKFSNRTResults {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_for_channel_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_for_channel_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::audio_synesthesia_core::FLKFSNRTResults>()
                .read()
        }
    }
    pub fn get_loudness_data_for_channel(
        &self,
        in_channel: i32,
    ) -> TArray<crate::bindings::audio_synesthesia_core::FLKFSNRTResults> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_for_channel,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<crate::bindings::audio_synesthesia_core::FLKFSNRTResults>,
                >()
                .read()
        }
    }
    pub fn get_loudness_data_at_time(
        &self,
        in_seconds: f32,
    ) -> crate::bindings::audio_synesthesia_core::FLKFSNRTResults {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(4)
                .cast::<crate::bindings::audio_synesthesia_core::FLKFSNRTResults>()
                .read()
        }
    }
    pub fn get_loudness_data(
        &self,
    ) -> TArray<crate::bindings::audio_synesthesia_core::FLKFSNRTResults> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<crate::bindings::audio_synesthesia_core::FLKFSNRTResults>,
                >()
                .read()
        }
    }
    pub fn get_loudness_at_time(&self, in_seconds: f32, out_loudness: &mut f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<f32>().swap(out_loudness);
        }
    }
    pub fn get_integrated_loudness_for_channel(&self, in_channel: i32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_integrated_loudness_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_integrated_loudness_for_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_integrated_loudness(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_integrated_loudness,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_integrated_loudness,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_gated_loudness_for_channel(&self, in_channel: i32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_gated_loudness_for_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_gated_loudness_for_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_gated_loudness(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_gated_loudness,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_gated_loudness,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_channel_loudness_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
        out_loudness: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .ulkfsnrt_get_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<f32>().swap(out_loudness);
        }
    }
}
#[repr(C, align(8))]
pub struct ULoudnessSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessCurveTypeEnum,
    pub noise_floor_db: f32,
    pub expected_max_loudness: f32,
}
impl ULoudnessSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoudnessSettings")
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
pub struct ULoudnessAnalyzer {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: UPtr<ULoudnessSettings>,
    __padding_end: [u8; 96],
}
impl ULoudnessAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoudnessAnalyzer")
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
pub struct ULoudnessNRTSettings {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub analysis_period: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
    pub curve_type: ELoudnessNRTCurveTypeEnum,
    pub noise_floor_db: f32,
}
impl ULoudnessNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoudnessNRTSettings")
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
pub struct ULoudnessNRT {
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 232],
    pub settings: UPtr<ULoudnessNRTSettings>,
}
impl ULoudnessNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoudnessNRT")
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
    pub fn get_normalized_loudness_at_time(
        &self,
        in_seconds: f32,
        out_loudness: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_normalized_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_normalized_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<f32>().swap(out_loudness);
        }
    }
    pub fn get_normalized_channel_loudness_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
        out_loudness: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_normalized_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_normalized_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<f32>().swap(out_loudness);
        }
    }
    pub fn get_loudness_at_time(&self, in_seconds: f32, out_loudness: &mut f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<f32>().swap(out_loudness);
        }
    }
    pub fn get_channel_loudness_at_time(
        &self,
        in_seconds: f32,
        in_channel: i32,
        out_loudness: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_loudness,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_loudness_nrt_get_channel_loudness_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<f32>().swap(out_loudness);
        }
    }
}
#[repr(C, align(8))]
pub struct UMeterSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub peak_mode: EMeterPeakType,
    pub meter_attack_time: i32,
    pub meter_release_time: i32,
    pub peak_hold_time: i32,
    pub clipping_threshold: f32,
}
impl UMeterSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeterSettings")
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
pub struct UMeterAnalyzer {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: UPtr<UMeterSettings>,
    __padding_end: [u8; 224],
}
impl UMeterAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeterAnalyzer")
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
pub struct UOnsetNRTSettings {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub b_downmix_to_mono: bool,
    pub granularity_in_seconds: f32,
    pub sensitivity: f32,
    pub minimum_frequency: f32,
    pub maximum_frequency: f32,
}
impl UOnsetNRTSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnsetNRTSettings")
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
pub struct UOnsetNRT {
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 232],
    pub settings: UPtr<UOnsetNRTSettings>,
}
impl UOnsetNRT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOnsetNRT")
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
    pub fn get_normalized_channel_onsets_between_times(
        &self,
        in_start_seconds: f32,
        in_end_seconds: f32,
        in_channel: i32,
        out_onset_timestamps: &mut TArray<f32>,
        out_onset_strengths: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_onset_nrt_get_normalized_channel_onsets_between_times,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_end_seconds,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_onset_timestamps,
                __buffer.add(16).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_onset_strengths,
                __buffer.add(32).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_onset_nrt_get_normalized_channel_onsets_between_times,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<f32>>().swap(out_onset_timestamps);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<f32>>().swap(out_onset_strengths);
        }
    }
    pub fn get_channel_onsets_between_times(
        &self,
        in_start_seconds: f32,
        in_end_seconds: f32,
        in_channel: i32,
        out_onset_timestamps: &mut TArray<f32>,
        out_onset_strengths: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_onset_nrt_get_channel_onsets_between_times,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_end_seconds,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_channel, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_onset_timestamps,
                __buffer.add(16).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_onset_strengths,
                __buffer.add(32).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_onset_nrt_get_channel_onsets_between_times,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<f32>>().swap(out_onset_timestamps);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<f32>>().swap(out_onset_strengths);
        }
    }
}
#[repr(C, align(8))]
pub struct USynesthesiaSpectrumAnalysisSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub analysis_period: f32,
    pub fft_size: crate::bindings::engine::EFFTSize,
    pub spectrum_type: crate::bindings::engine::EAudioSpectrumType,
    pub window_type: crate::bindings::engine::EFFTWindowType,
    pub b_downmix_to_mono: bool,
}
impl USynesthesiaSpectrumAnalysisSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynesthesiaSpectrumAnalysisSettings")
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
pub struct USynesthesiaSpectrumAnalyzer {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub settings: UPtr<USynesthesiaSpectrumAnalysisSettings>,
    __padding_end: [u8; 112],
}
impl USynesthesiaSpectrumAnalyzer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USynesthesiaSpectrumAnalyzer")
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
    pub fn get_num_center_frequencies(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_synesthesia_spectrum_analyzer_get_num_center_frequencies,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_synesthesia_spectrum_analyzer_get_num_center_frequencies,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_center_frequencies(
        &mut self,
        in_sample_rate: f32,
        out_center_frequencies: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_synesthesia_spectrum_analyzer_get_center_frequencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sample_rate,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_center_frequencies,
                __buffer.add(8).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_synesthesia::__FUNCTION_PTRS
                    .u_synesthesia_spectrum_analyzer_get_center_frequencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<f32>>().swap(out_center_frequencies);
        }
    }
}
#[repr(C, align(8))]
pub struct FConstantQAnalyzer_OnConstantQResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FConstantQAnalyzer_OnLatestConstantQResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLKFSAnalyzer_OnOverallLKFSResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLKFSAnalyzer_OnPerChannelLKFSResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLKFSAnalyzer_OnLatestOverallLKFSResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLKFSAnalyzer_OnLatestPerChannelLKFSResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLoudnessAnalyzer_OnOverallLoudnessResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLoudnessAnalyzer_OnPerChannelLoudnessResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLoudnessAnalyzer_OnLatestOverallLoudnessResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FLoudnessAnalyzer_OnLatestPerChannelLoudnessResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMeterAnalyzer_OnOverallMeterResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMeterAnalyzer_OnPerChannelMeterResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMeterAnalyzer_OnLatestOverallMeterResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMeterAnalyzer_OnLatestPerChannelMeterResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynesthesiaSpectrumAnalyzer_OnSpectrumResults {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSynesthesiaSpectrumAnalyzer_OnLatestSpectrumResults {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EConstantQFFTSizeEnum(pub u8);
impl EConstantQFFTSizeEnum {
    pub const MIN: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(0);
    pub const XX_SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(1);
    pub const X_SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(2);
    pub const SMALL: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(3);
    pub const MEDIUM: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(4);
    pub const LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(5);
    pub const X_LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(6);
    pub const XX_LARGE: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(7);
    pub const MAX: EConstantQFFTSizeEnum = EConstantQFFTSizeEnum(8);
}
#[repr(transparent)]
pub struct EConstantQNormalizationEnum(pub u8);
impl EConstantQNormalizationEnum {
    pub const EQUAL_EUCLIDEAN_NORM: EConstantQNormalizationEnum = EConstantQNormalizationEnum(
        0,
    );
    pub const EQUAL_ENERGY: EConstantQNormalizationEnum = EConstantQNormalizationEnum(1);
    pub const EQUAL_AMPLITUDE: EConstantQNormalizationEnum = EConstantQNormalizationEnum(
        2,
    );
}
#[repr(transparent)]
pub struct ELoudnessCurveTypeEnum(pub u8);
impl ELoudnessCurveTypeEnum {
    pub const A: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(0);
    pub const B: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(1);
    pub const C: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(2);
    pub const D: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(3);
    pub const K: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(4);
    pub const NONE: ELoudnessCurveTypeEnum = ELoudnessCurveTypeEnum(5);
}
#[repr(transparent)]
pub struct ELoudnessNRTCurveTypeEnum(pub u8);
impl ELoudnessNRTCurveTypeEnum {
    pub const A: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(0);
    pub const B: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(1);
    pub const C: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(2);
    pub const D: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(3);
    pub const K: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(4);
    pub const NONE: ELoudnessNRTCurveTypeEnum = ELoudnessNRTCurveTypeEnum(5);
}
#[repr(transparent)]
pub struct EMeterPeakType(pub u8);
impl EMeterPeakType {
    pub const MEAN_SQUARED: EMeterPeakType = EMeterPeakType(0);
    pub const ROOT_MEAN_SQUARED: EMeterPeakType = EMeterPeakType(1);
    pub const PEAK: EMeterPeakType = EMeterPeakType(2);
    pub const COUNT: EMeterPeakType = EMeterPeakType(3);
}
