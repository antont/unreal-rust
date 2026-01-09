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
    pub u_audio_material_button_set_is_pressed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_button_get_is_pressed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_mouse_uses_step: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_locked: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_set_fine_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_mouse_uses_step: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_is_locked: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_knob_get_fine_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_meter_set_meter_channel_info: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_meter_get_meter_channel_info_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_meter_get_meter_channel_info: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_mouse_uses_step: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_locked: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_set_fine_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_step_size: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_mouse_uses_step: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_is_locked: *mut crate::ffi::UFunctionOpague,
    pub u_audio_material_slider_get_fine_tune_speed: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_value_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_scale_label_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_scale_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_peak_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_clipping_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_channel_info: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_meter_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_set_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_get_meter_channel_info_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_audio_meter_get_meter_channel_info: *mut crate::ffi::UFunctionOpague,
    pub u_audio_oscilloscope_stop_processing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_oscilloscope_start_processing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_oscilloscope_get_oscilloscope_audio_samples_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_audio_oscilloscope_can_triggering_be_set: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_widget_layout: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_value_text_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_units_text_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_units_text: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_text_label_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_slider_thickness: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_slider_progress_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_slider_bar_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_show_units_text: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_show_label_only_on_hover: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_output_range: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_hand_start_end_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_set_center_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_get_slider_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_radial_slider_get_output_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_widget_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_value_text_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_units_text_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_units_text: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_text_label_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_slider_thumb_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_slider_bar_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_slider_background_color: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_show_units_text: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_set_show_label_only_on_hover: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_get_slider_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_get_output_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_slider_base_get_lin_value: *mut crate::ffi::UFunctionOpague,
    pub u_audio_vectorscope_stop_processing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_vectorscope_start_processing: *mut crate::ffi::UFunctionOpague,
    pub u_audio_vectorscope_get_vectorscope_audio_samples_delegate_signature: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_audio_material_button_set_is_pressed: std::ptr::null_mut(),
            u_audio_material_button_get_is_pressed: std::ptr::null_mut(),
            u_audio_material_knob_set_value: std::ptr::null_mut(),
            u_audio_material_knob_set_tune_speed: std::ptr::null_mut(),
            u_audio_material_knob_set_step_size: std::ptr::null_mut(),
            u_audio_material_knob_set_mouse_uses_step: std::ptr::null_mut(),
            u_audio_material_knob_set_locked: std::ptr::null_mut(),
            u_audio_material_knob_set_fine_tune_speed: std::ptr::null_mut(),
            u_audio_material_knob_get_value: std::ptr::null_mut(),
            u_audio_material_knob_get_tune_speed: std::ptr::null_mut(),
            u_audio_material_knob_get_step_size: std::ptr::null_mut(),
            u_audio_material_knob_get_mouse_uses_step: std::ptr::null_mut(),
            u_audio_material_knob_get_is_locked: std::ptr::null_mut(),
            u_audio_material_knob_get_fine_tune_speed: std::ptr::null_mut(),
            u_audio_material_meter_set_meter_channel_info: std::ptr::null_mut(),
            u_audio_material_meter_get_meter_channel_info_delegate_signature: std::ptr::null_mut(),
            u_audio_material_meter_get_meter_channel_info: std::ptr::null_mut(),
            u_audio_material_slider_set_value: std::ptr::null_mut(),
            u_audio_material_slider_set_tune_speed: std::ptr::null_mut(),
            u_audio_material_slider_set_step_size: std::ptr::null_mut(),
            u_audio_material_slider_set_mouse_uses_step: std::ptr::null_mut(),
            u_audio_material_slider_set_locked: std::ptr::null_mut(),
            u_audio_material_slider_set_fine_tune_speed: std::ptr::null_mut(),
            u_audio_material_slider_get_value: std::ptr::null_mut(),
            u_audio_material_slider_get_tune_speed: std::ptr::null_mut(),
            u_audio_material_slider_get_step_size: std::ptr::null_mut(),
            u_audio_material_slider_get_mouse_uses_step: std::ptr::null_mut(),
            u_audio_material_slider_get_is_locked: std::ptr::null_mut(),
            u_audio_material_slider_get_fine_tune_speed: std::ptr::null_mut(),
            u_audio_meter_set_meter_value_color: std::ptr::null_mut(),
            u_audio_meter_set_meter_scale_label_color: std::ptr::null_mut(),
            u_audio_meter_set_meter_scale_color: std::ptr::null_mut(),
            u_audio_meter_set_meter_peak_color: std::ptr::null_mut(),
            u_audio_meter_set_meter_clipping_color: std::ptr::null_mut(),
            u_audio_meter_set_meter_channel_info: std::ptr::null_mut(),
            u_audio_meter_set_meter_background_color: std::ptr::null_mut(),
            u_audio_meter_set_background_color: std::ptr::null_mut(),
            u_audio_meter_get_meter_channel_info_delegate_signature: std::ptr::null_mut(),
            u_audio_meter_get_meter_channel_info: std::ptr::null_mut(),
            u_audio_oscilloscope_stop_processing: std::ptr::null_mut(),
            u_audio_oscilloscope_start_processing: std::ptr::null_mut(),
            u_audio_oscilloscope_get_oscilloscope_audio_samples_delegate_signature: std::ptr::null_mut(),
            u_audio_oscilloscope_can_triggering_be_set: std::ptr::null_mut(),
            u_audio_radial_slider_set_widget_layout: std::ptr::null_mut(),
            u_audio_radial_slider_set_value_text_read_only: std::ptr::null_mut(),
            u_audio_radial_slider_set_units_text_read_only: std::ptr::null_mut(),
            u_audio_radial_slider_set_units_text: std::ptr::null_mut(),
            u_audio_radial_slider_set_text_label_background_color: std::ptr::null_mut(),
            u_audio_radial_slider_set_slider_thickness: std::ptr::null_mut(),
            u_audio_radial_slider_set_slider_progress_color: std::ptr::null_mut(),
            u_audio_radial_slider_set_slider_bar_color: std::ptr::null_mut(),
            u_audio_radial_slider_set_show_units_text: std::ptr::null_mut(),
            u_audio_radial_slider_set_show_label_only_on_hover: std::ptr::null_mut(),
            u_audio_radial_slider_set_output_range: std::ptr::null_mut(),
            u_audio_radial_slider_set_hand_start_end_ratio: std::ptr::null_mut(),
            u_audio_radial_slider_set_center_background_color: std::ptr::null_mut(),
            u_audio_radial_slider_get_slider_value: std::ptr::null_mut(),
            u_audio_radial_slider_get_output_value: std::ptr::null_mut(),
            u_audio_slider_base_set_widget_background_color: std::ptr::null_mut(),
            u_audio_slider_base_set_value_text_read_only: std::ptr::null_mut(),
            u_audio_slider_base_set_units_text_read_only: std::ptr::null_mut(),
            u_audio_slider_base_set_units_text: std::ptr::null_mut(),
            u_audio_slider_base_set_text_label_background_color: std::ptr::null_mut(),
            u_audio_slider_base_set_slider_thumb_color: std::ptr::null_mut(),
            u_audio_slider_base_set_slider_bar_color: std::ptr::null_mut(),
            u_audio_slider_base_set_slider_background_color: std::ptr::null_mut(),
            u_audio_slider_base_set_show_units_text: std::ptr::null_mut(),
            u_audio_slider_base_set_show_label_only_on_hover: std::ptr::null_mut(),
            u_audio_slider_base_get_slider_value: std::ptr::null_mut(),
            u_audio_slider_base_get_output_value: std::ptr::null_mut(),
            u_audio_slider_base_get_lin_value: std::ptr::null_mut(),
            u_audio_vectorscope_stop_processing: std::ptr::null_mut(),
            u_audio_vectorscope_start_processing: std::ptr::null_mut(),
            u_audio_vectorscope_get_vectorscope_audio_samples_delegate_signature: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMaterialButton::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsPressed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_button_set_is_pressed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsPressed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_button_get_is_pressed,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMaterialKnob::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMouseUsesStep"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_mouse_uses_step,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFineTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_set_fine_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStepSize"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMouseUsesStep"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_mouse_uses_step,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsLocked"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_is_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFineTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_knob_get_fine_tune_speed,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMaterialMeter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterChannelInfo"),
            &raw mut __FUNCTION_PTRS.u_audio_material_meter_set_meter_channel_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeterChannelInfo__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_audio_material_meter_get_meter_channel_info_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeterChannelInfo"),
            &raw mut __FUNCTION_PTRS.u_audio_material_meter_get_meter_channel_info,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMaterialSlider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStepSize"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMouseUsesStep"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_mouse_uses_step,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLocked"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFineTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_set_fine_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_tune_speed,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStepSize"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_step_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMouseUsesStep"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_mouse_uses_step,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIsLocked"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_is_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFineTuneSpeed"),
            &raw mut __FUNCTION_PTRS.u_audio_material_slider_get_fine_tune_speed,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioMeter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterValueColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_value_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterScaleLabelColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_scale_label_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterScaleColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_scale_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterPeakColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_peak_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterClippingColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_clipping_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterChannelInfo"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_channel_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMeterBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_meter_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_set_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeterChannelInfo__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_audio_meter_get_meter_channel_info_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeterChannelInfo"),
            &raw mut __FUNCTION_PTRS.u_audio_meter_get_meter_channel_info,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioOscilloscope::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopProcessing"),
            &raw mut __FUNCTION_PTRS.u_audio_oscilloscope_stop_processing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartProcessing"),
            &raw mut __FUNCTION_PTRS.u_audio_oscilloscope_start_processing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOscilloscopeAudioSamples__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_audio_oscilloscope_get_oscilloscope_audio_samples_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanTriggeringBeSet"),
            &raw mut __FUNCTION_PTRS.u_audio_oscilloscope_can_triggering_be_set,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioRadialSlider::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetLayout"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_widget_layout,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueTextReadOnly"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_value_text_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUnitsTextReadOnly"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_units_text_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUnitsText"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_units_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextLabelBackgroundColor"),
            &raw mut __FUNCTION_PTRS
                .u_audio_radial_slider_set_text_label_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderThickness"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_slider_thickness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderProgressColor"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_slider_progress_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderBarColor"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_slider_bar_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowUnitsText"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_show_units_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowLabelOnlyOnHover"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_show_label_only_on_hover,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOutputRange"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_output_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetHandStartEndRatio"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_hand_start_end_ratio,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCenterBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_set_center_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSliderValue"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_get_slider_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutputValue"),
            &raw mut __FUNCTION_PTRS.u_audio_radial_slider_get_output_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioSliderBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWidgetBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_widget_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValueTextReadOnly"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_value_text_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUnitsTextReadOnly"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_units_text_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUnitsText"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_units_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextLabelBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_text_label_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderThumbColor"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_slider_thumb_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderBarColor"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_slider_bar_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSliderBackgroundColor"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_slider_background_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowUnitsText"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_show_units_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetShowLabelOnlyOnHover"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_set_show_label_only_on_hover,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSliderValue"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_get_slider_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOutputValue"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_get_output_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLinValue"),
            &raw mut __FUNCTION_PTRS.u_audio_slider_base_get_lin_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAudioVectorscope::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StopProcessing"),
            &raw mut __FUNCTION_PTRS.u_audio_vectorscope_stop_processing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("StartProcessing"),
            &raw mut __FUNCTION_PTRS.u_audio_vectorscope_start_processing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorscopeAudioSamples__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_audio_vectorscope_get_vectorscope_audio_samples_delegate_signature,
        );
    }
}
#[repr(C, align(4))]
pub struct FMeterChannelInfo {
    pub(crate) __padding_end: [u8; 12],
}
impl FMeterChannelInfo {}
#[repr(C, align(8))]
pub struct FAudioMaterialWidgetStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub desired_size: crate::bindings::core_u_object::FVector2f,
}
impl FAudioMaterialWidgetStyle {}
#[repr(C, align(8))]
pub struct FAudioMaterialMeterStyle {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub meter_fill_min_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_fill_mid_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_fill_max_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_fill_background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_padding: crate::bindings::core_u_object::FVector2D,
    pub value_range_db: crate::bindings::core_u_object::FVector2D,
    pub b_show_scale: bool,
    pub b_scale_side: bool,
    pub scale_hash_offset: f32,
    pub scale_hash_width: f32,
    pub scale_hash_height: f32,
    pub decibels_per_hash: i32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
}
impl FAudioMaterialMeterStyle {}
#[repr(C, align(16))]
pub struct FAudioMeterStyle {
    pub(crate) __padding_end: [u8; 1248],
}
impl FAudioMeterStyle {}
#[repr(C, align(16))]
pub struct FAudioOscilloscopePanelStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub time_ruler_style: FFixedSampleSequenceRulerStyle,
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub wave_viewer_style: FSampledSequenceViewerStyle,
    pub trigger_threshold_line_style: FTriggerThresholdLineStyle,
}
impl FAudioOscilloscopePanelStyle {}
#[repr(C, align(8))]
pub struct FTriggerThresholdLineStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub line_color: crate::bindings::core_u_object::FLinearColor,
}
impl FTriggerThresholdLineStyle {}
#[repr(C, align(16))]
pub struct FSampledSequenceViewerStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub sequence_color: crate::bindings::slate_core::FSlateColor,
    pub sequence_line_thickness: f32,
    pub major_grid_line_color: crate::bindings::slate_core::FSlateColor,
    pub minor_grid_line_color: crate::bindings::slate_core::FSlateColor,
    pub zero_crossing_line_color: crate::bindings::slate_core::FSlateColor,
    pub zero_crossing_line_thickness: f32,
    pub sample_markers_size: f32,
    pub sequence_background_color: crate::bindings::slate_core::FSlateColor,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub desired_width: f32,
    pub desired_height: f32,
}
impl FSampledSequenceViewerStyle {}
#[repr(C, align(8))]
pub struct FSampledSequenceValueGridOverlayStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub grid_color: crate::bindings::slate_core::FSlateColor,
    pub grid_thickness: f32,
    pub label_text_color: crate::bindings::slate_core::FSlateColor,
    pub label_text_font: crate::bindings::slate_core::FSlateFontInfo,
    pub desired_width: f32,
    pub desired_height: f32,
}
impl FSampledSequenceValueGridOverlayStyle {}
#[repr(C, align(16))]
pub struct FFixedSampleSequenceRulerStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub handle_width: f32,
    pub handle_color: crate::bindings::slate_core::FSlateColor,
    pub handle_brush: crate::bindings::slate_core::FSlateBrush,
    pub ticks_color: crate::bindings::slate_core::FSlateColor,
    pub ticks_text_color: crate::bindings::slate_core::FSlateColor,
    pub ticks_text_font: crate::bindings::slate_core::FSlateFontInfo,
    pub ticks_text_offset: f32,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub desired_width: f32,
    pub desired_height: f32,
}
impl FFixedSampleSequenceRulerStyle {}
#[repr(C, align(16))]
pub struct FAudioVectorscopePanelStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub vector_viewer_style: FSampledSequenceVectorViewerStyle,
}
impl FAudioVectorscopePanelStyle {}
#[repr(C, align(16))]
pub struct FSampledSequenceVectorViewerStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub line_color: crate::bindings::core_u_object::FLinearColor,
    pub line_thickness: f32,
}
impl FSampledSequenceVectorViewerStyle {}
#[repr(C, align(4))]
pub struct FAudioMaterialEnvelopeSettings {
    #[doc(hidden)]
    pub(crate) __padding_4: [u8; 4],
    pub attack_curve: f32,
    pub attack_value: f32,
    pub attack_time: f32,
    pub decay_curve: f32,
    pub decay_time: f32,
    pub sustain_value: f32,
    pub release_curve: f32,
    pub release_time: f32,
}
impl FAudioMaterialEnvelopeSettings {}
#[repr(C, align(8))]
pub struct FAudioMaterialButtonStyle {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub button_main_color: crate::bindings::core_u_object::FLinearColor,
    pub button_main_color_tint_1: crate::bindings::core_u_object::FLinearColor,
    pub button_main_color_tint_2: crate::bindings::core_u_object::FLinearColor,
    pub button_accent_color: crate::bindings::core_u_object::FLinearColor,
    pub button_shadow_color: crate::bindings::core_u_object::FLinearColor,
    pub button_unpressed_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub button_pressed_outline_color: crate::bindings::core_u_object::FLinearColor,
}
impl FAudioMaterialButtonStyle {}
#[repr(C, align(16))]
pub struct FAudioMaterialSliderStyle {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub slider_background_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_background_accent_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_value_main_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_main_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub text_box_style: FAudioTextBoxStyle,
}
impl FAudioMaterialSliderStyle {}
#[repr(C, align(16))]
pub struct FAudioTextBoxStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub background_color: crate::bindings::slate_core::FSlateColor,
}
impl FAudioTextBoxStyle {}
#[repr(C, align(16))]
pub struct FAudioMaterialKnobStyle {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub knob_main_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_accent_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_shadow_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_smooth_bevel_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_indicator_dot_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_edge_fill_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_shadow_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_fill_min_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_fill_mid_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_fill_max_color: crate::bindings::core_u_object::FLinearColor,
    pub knob_bar_fill_tint_color: crate::bindings::core_u_object::FLinearColor,
    pub text_box_style: FAudioTextBoxStyle,
}
impl FAudioMaterialKnobStyle {}
#[repr(C, align(8))]
pub struct FAudioMaterialEnvelopeStyle {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub outline_color: crate::bindings::core_u_object::FLinearColor,
}
impl FAudioMaterialEnvelopeStyle {}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorStyle {
    pub(crate) __padding_end: [u8; 104],
}
impl FAudioMeterDefaultColorStyle {}
#[repr(C, align(8))]
pub struct FAudioSpectrumPlotStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub grid_color: crate::bindings::slate_core::FSlateColor,
    pub axis_label_color: crate::bindings::slate_core::FSlateColor,
    pub axis_label_font: crate::bindings::slate_core::FSlateFontInfo,
    pub spectrum_color: crate::bindings::slate_core::FSlateColor,
    pub crosshair_color: crate::bindings::slate_core::FSlateColor,
    pub crosshair_label_font: crate::bindings::slate_core::FSlateFontInfo,
}
impl FAudioSpectrumPlotStyle {}
#[repr(C, align(16))]
pub struct FAudioSliderStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub slider_style: crate::bindings::slate_core::FSliderStyle,
    pub text_box_style: FAudioTextBoxStyle,
    pub widget_background_image: crate::bindings::slate_core::FSlateBrush,
    pub slider_background_color: crate::bindings::slate_core::FSlateColor,
    pub slider_background_size: crate::bindings::core_u_object::FVector2D,
    pub label_padding: f32,
    pub slider_bar_color: crate::bindings::slate_core::FSlateColor,
    pub slider_thumb_color: crate::bindings::slate_core::FSlateColor,
    pub widget_background_color: crate::bindings::slate_core::FSlateColor,
}
impl FAudioSliderStyle {}
#[repr(C, align(16))]
pub struct FAudioRadialSliderStyle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub text_box_style: FAudioTextBoxStyle,
    pub center_background_color: crate::bindings::slate_core::FSlateColor,
    pub slider_bar_color: crate::bindings::slate_core::FSlateColor,
    pub slider_progress_color: crate::bindings::slate_core::FSlateColor,
    pub label_padding: f32,
    pub default_slider_radius: f32,
}
impl FAudioRadialSliderStyle {}
#[repr(C, align(8))]
pub struct FPlayheadOverlayStyle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub playhead_color: crate::bindings::slate_core::FSlateColor,
    pub playhead_width: f32,
    pub desired_width: f32,
    pub desired_height: f32,
}
impl FPlayheadOverlayStyle {}
#[repr(C, align(16))]
pub struct UAudioMaterialKnobWidgetStyle {
    __padding_end: [u8; 544],
}
impl UAudioMaterialKnobWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialKnobWidgetStyle")
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
pub struct UAudioMaterialMeterWidgetStyle {
    __padding_end: [u8; 312],
}
impl UAudioMaterialMeterWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialMeterWidgetStyle")
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
pub struct UAudioMaterialButtonWidgetStyle {
    __padding_end: [u8; 192],
}
impl UAudioMaterialButtonWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialButtonWidgetStyle")
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
pub struct UAudioMaterialSliderWidgetStyle {
    __padding_end: [u8; 432],
}
impl UAudioMaterialSliderWidgetStyle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialSliderWidgetStyle")
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
pub struct UAudioMaterialButton {
    #[doc(hidden)]
    pub(crate) __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialButtonStyle,
    #[doc(hidden)]
    pub(crate) __padding_856: [u8; 24],
    pub b_is_pressed: bool,
    __padding_end: [u8; 23],
}
impl UAudioMaterialButton {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialButton")
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
    pub fn set_is_pressed(&mut self, in_pressed: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_button_set_is_pressed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pressed,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_button_set_is_pressed,
                __buffer,
            )
        };
    }
    pub fn get_is_pressed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_button_get_is_pressed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_button_get_is_pressed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAudioMaterialEnvelope {
    #[doc(hidden)]
    pub(crate) __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialEnvelopeStyle,
    pub envelope_settings: FAudioMaterialEnvelopeSettings,
    __padding_end: [u8; 20],
}
impl UAudioMaterialEnvelope {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialEnvelope")
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
pub struct UAudioMaterialKnob {
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 704],
    pub widget_style: FAudioMaterialKnobStyle,
    #[doc(hidden)]
    pub(crate) __padding_1208: [u8; 24],
    pub value: f32,
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
    __padding_end: [u8; 20],
}
impl UAudioMaterialKnob {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialKnob")
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
    pub fn set_value(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_value,
                __buffer,
            )
        };
    }
    pub fn set_tune_speed(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_tune_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_tune_speed,
                __buffer,
            )
        };
    }
    pub fn set_step_size(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_step_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_step_size,
                __buffer,
            )
        };
    }
    pub fn set_mouse_uses_step(&mut self, in_uses_step: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_mouse_uses_step,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_uses_step,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_mouse_uses_step,
                __buffer,
            )
        };
    }
    pub fn set_locked(&mut self, in_locked: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_locked, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_locked,
                __buffer,
            )
        };
    }
    pub fn set_fine_tune_speed(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_fine_tune_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_set_fine_tune_speed,
                __buffer,
            )
        };
    }
    pub fn get_value(&mut self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_tune_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_tune_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_tune_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_step_size(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_step_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_step_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_mouse_uses_step(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_mouse_uses_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_mouse_uses_step,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_is_locked(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_is_locked,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_is_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_fine_tune_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_fine_tune_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_knob_get_fine_tune_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAudioMaterialMeter {
    #[doc(hidden)]
    pub(crate) __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialMeterStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    #[doc(hidden)]
    pub(crate) __padding_1024: [u8; 64],
    pub meter_channel_info: TArray<FMeterChannelInfo>,
}
impl UAudioMaterialMeter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialMeter")
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
    pub fn set_meter_channel_info(
        &mut self,
        in_meter_channel_info: &TArray<FMeterChannelInfo>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_meter_set_meter_channel_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_meter_channel_info,
                __buffer.add(0).cast::<TArray<FMeterChannelInfo>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_meter_set_meter_channel_info,
                __buffer,
            )
        };
    }
    pub fn get_meter_channel_info(&self) -> TArray<FMeterChannelInfo> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_meter_get_meter_channel_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_meter_get_meter_channel_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FMeterChannelInfo>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioMaterialSlider {
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 704],
    pub widget_style: FAudioMaterialSliderStyle,
    #[doc(hidden)]
    pub(crate) __padding_1096: [u8; 24],
    pub value: f32,
    #[doc(hidden)]
    pub(crate) __padding_1104: [u8; 4],
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
    __padding_end: [u8; 16],
}
impl UAudioMaterialSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMaterialSlider")
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
    pub fn set_value(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_value,
                __buffer,
            )
        };
    }
    pub fn set_tune_speed(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_tune_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_tune_speed,
                __buffer,
            )
        };
    }
    pub fn set_step_size(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_step_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_step_size,
                __buffer,
            )
        };
    }
    pub fn set_mouse_uses_step(&mut self, b_in_uses_step: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_mouse_uses_step,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_uses_step,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_mouse_uses_step,
                __buffer,
            )
        };
    }
    pub fn set_locked(&mut self, b_in_locked: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_locked,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_locked,
                __buffer,
            )
        };
    }
    pub fn set_fine_tune_speed(&mut self, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_fine_tune_speed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_set_fine_tune_speed,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_tune_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_tune_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_tune_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_step_size(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_step_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_step_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_mouse_uses_step(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_mouse_uses_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_mouse_uses_step,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_is_locked(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_is_locked,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_is_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_fine_tune_speed(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_fine_tune_speed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_material_slider_get_fine_tune_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioMeter {
    #[doc(hidden)]
    pub(crate) __padding_752: [u8; 752],
    pub widget_style: FAudioMeterStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_value_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_peak_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_clipping_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_label_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 44],
}
impl UAudioMeter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMeter")
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
    pub fn set_meter_value_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_value_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_value_color,
                __buffer,
            )
        };
    }
    pub fn set_meter_scale_label_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_scale_label_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_scale_label_color,
                __buffer,
            )
        };
    }
    pub fn set_meter_scale_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_scale_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_scale_color,
                __buffer,
            )
        };
    }
    pub fn set_meter_peak_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_peak_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_peak_color,
                __buffer,
            )
        };
    }
    pub fn set_meter_clipping_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_clipping_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_clipping_color,
                __buffer,
            )
        };
    }
    pub fn set_meter_channel_info(
        &mut self,
        in_meter_channel_info: &TArray<FMeterChannelInfo>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_channel_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_meter_channel_info,
                __buffer.add(0).cast::<TArray<FMeterChannelInfo>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_channel_info,
                __buffer,
            )
        };
    }
    pub fn set_meter_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_meter_background_color,
                __buffer,
            )
        };
    }
    pub fn set_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_set_background_color,
                __buffer,
            )
        };
    }
    pub fn get_meter_channel_info(&self) -> TArray<FMeterChannelInfo> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_get_meter_channel_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_meter_get_meter_channel_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FMeterChannelInfo>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioOscilloscope {
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 704],
    pub oscilloscope_style: FAudioOscilloscopePanelStyle,
    pub audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub max_time_window_ms: f32,
    pub time_window_ms: f32,
    pub analysis_period_ms: f32,
    pub b_show_time_grid: bool,
    pub time_grid_labels_unit: EXAxisLabelsUnit,
    pub b_show_amplitude_grid: bool,
    pub b_show_amplitude_labels: bool,
    pub amplitude_grid_labels_unit: EYAxisLabelsUnit,
    pub trigger_mode: EAudioOscilloscopeTriggerMode,
    pub trigger_threshold: f32,
    pub panel_layout_type: EAudioPanelLayoutType,
    pub channel_to_analyze: i32,
    __padding_end: [u8; 88],
}
impl UAudioOscilloscope {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioOscilloscope")
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
    pub fn stop_processing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_oscilloscope_stop_processing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_oscilloscope_stop_processing,
                __buffer,
            )
        };
    }
    pub fn start_processing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_oscilloscope_start_processing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_oscilloscope_start_processing,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UAudioRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioRadialSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioRadialSlider")
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
    pub fn set_widget_layout(&mut self, in_layout: EAudioRadialSliderLayout) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_widget_layout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_layout,
                __buffer.add(0).cast::<EAudioRadialSliderLayout>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_widget_layout,
                __buffer,
            )
        };
    }
    pub fn set_value_text_read_only(&mut self, b_is_read_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_value_text_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_value_text_read_only,
                __buffer,
            )
        };
    }
    pub fn set_units_text_read_only(&mut self, b_is_read_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_units_text_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_units_text_read_only,
                __buffer,
            )
        };
    }
    pub fn set_units_text(&mut self, units: FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_units_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&units, __buffer.add(0).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_units_text,
                __buffer,
            )
        };
    }
    pub fn set_text_label_background_color(
        &mut self,
        in_color: crate::bindings::slate_core::FSlateColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_text_label_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_color,
                __buffer.add(0).cast::<crate::bindings::slate_core::FSlateColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_text_label_background_color,
                __buffer,
            )
        };
    }
    pub fn set_slider_thickness(&mut self, in_thickness: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_thickness,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_thickness,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_thickness,
                __buffer,
            )
        };
    }
    pub fn set_slider_progress_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_progress_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_progress_color,
                __buffer,
            )
        };
    }
    pub fn set_slider_bar_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_bar_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_slider_bar_color,
                __buffer,
            )
        };
    }
    pub fn set_show_units_text(&mut self, b_show_units_text: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_show_units_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_units_text,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_show_units_text,
                __buffer,
            )
        };
    }
    pub fn set_show_label_only_on_hover(&mut self, b_show_label_only_on_hover: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_show_label_only_on_hover,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_label_only_on_hover,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_show_label_only_on_hover,
                __buffer,
            )
        };
    }
    pub fn set_output_range(
        &mut self,
        in_output_range: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_output_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_output_range,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_output_range,
                __buffer,
            )
        };
    }
    pub fn set_hand_start_end_ratio(
        &mut self,
        in_hand_start_end_ratio: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_hand_start_end_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_hand_start_end_ratio,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_hand_start_end_ratio,
                __buffer,
            )
        };
    }
    pub fn set_center_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_center_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_set_center_background_color,
                __buffer,
            )
        };
    }
    pub fn get_slider_value(&mut self, output_value: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_get_slider_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_get_slider_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_output_value(&mut self, in_slider_value: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_get_output_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slider_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_radial_slider_get_output_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioVolumeRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioVolumeRadialSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioVolumeRadialSlider")
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
pub struct UAudioFrequencyRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioFrequencyRadialSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioFrequencyRadialSlider")
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
pub struct UAudioSliderBase {
    #[doc(hidden)]
    pub(crate) __padding_1000: [u8; 1000],
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 2023],
}
impl UAudioSliderBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioSliderBase")
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
    pub fn set_widget_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_widget_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_widget_background_color,
                __buffer,
            )
        };
    }
    pub fn set_value_text_read_only(&mut self, b_is_read_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_value_text_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_value_text_read_only,
                __buffer,
            )
        };
    }
    pub fn set_units_text_read_only(&mut self, b_is_read_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_units_text_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_units_text_read_only,
                __buffer,
            )
        };
    }
    pub fn set_units_text(&mut self, units: FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_units_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&units, __buffer.add(0).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_units_text,
                __buffer,
            )
        };
    }
    pub fn set_text_label_background_color(
        &mut self,
        in_color: crate::bindings::slate_core::FSlateColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_text_label_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_color,
                __buffer.add(0).cast::<crate::bindings::slate_core::FSlateColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_text_label_background_color,
                __buffer,
            )
        };
    }
    pub fn set_slider_thumb_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_thumb_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_thumb_color,
                __buffer,
            )
        };
    }
    pub fn set_slider_bar_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_bar_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_bar_color,
                __buffer,
            )
        };
    }
    pub fn set_slider_background_color(
        &mut self,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_background_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_slider_background_color,
                __buffer,
            )
        };
    }
    pub fn set_show_units_text(&mut self, b_show_units_text: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_show_units_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_units_text,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_show_units_text,
                __buffer,
            )
        };
    }
    pub fn set_show_label_only_on_hover(&mut self, b_show_label_only_on_hover: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_show_label_only_on_hover,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_show_label_only_on_hover,
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
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_set_show_label_only_on_hover,
                __buffer,
            )
        };
    }
    pub fn get_slider_value(&mut self, output_value: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_slider_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_slider_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_output_value(&mut self, in_slider_value: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_output_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_slider_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_output_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_lin_value(&mut self, output_value: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_lin_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_slider_base_get_lin_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(16))]
pub struct UAudioSlider {
    #[doc(hidden)]
    pub(crate) __padding_3016: [u8; 3016],
    pub lin_to_output_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
    pub output_to_lin_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
}
impl UAudioSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioSlider")
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
pub struct UAudioVolumeSlider {
    __padding_end: [u8; 3040],
}
impl UAudioVolumeSlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioVolumeSlider")
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
pub struct UAudioFrequencySlider {
    __padding_end: [u8; 3040],
}
impl UAudioFrequencySlider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioFrequencySlider")
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
pub struct UAudioVectorscope {
    #[doc(hidden)]
    pub(crate) __padding_704: [u8; 704],
    pub vectorscope_style: FAudioVectorscopePanelStyle,
    pub audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub b_show_grid: bool,
    pub grid_divisions: i32,
    pub max_display_persistence_ms: f32,
    pub display_persistence_ms: f32,
    pub scale: f32,
    pub panel_layout_type: EAudioPanelLayoutType,
    __padding_end: [u8; 83],
}
impl UAudioVectorscope {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioVectorscope")
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
    pub fn stop_processing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_vectorscope_stop_processing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_vectorscope_stop_processing,
                __buffer,
            )
        };
    }
    pub fn start_processing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_vectorscope_start_processing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::audio_widgets::__FUNCTION_PTRS
                    .u_audio_vectorscope_start_processing,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct FAudioMaterialButton_OnButtonPressedChangedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioMaterialKnob_OnKnobValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioMaterialMeter_MeterChannelInfoDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioMaterialSlider_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioMeter_MeterChannelInfoDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioRadialSlider_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioRadialSlider_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_TextLabelBackgroundColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_SliderBackgroundColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_SliderBarColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_SliderThumbColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_WidgetBackgroundColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAudioSliderBase_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EAudioMaterialEnvelopeType(pub u8);
impl EAudioMaterialEnvelopeType {
    pub const AD: EAudioMaterialEnvelopeType = EAudioMaterialEnvelopeType(0);
    pub const ADSR: EAudioMaterialEnvelopeType = EAudioMaterialEnvelopeType(1);
}
#[repr(transparent)]
pub struct EAudioSpectrumAnalyzerType(pub u8);
impl EAudioSpectrumAnalyzerType {
    pub const FFT: EAudioSpectrumAnalyzerType = EAudioSpectrumAnalyzerType(0);
    pub const CQT: EAudioSpectrumAnalyzerType = EAudioSpectrumAnalyzerType(1);
}
#[repr(transparent)]
pub struct EAudioSpectrogramFrequencyAxisPixelBucketMode(pub u8);
impl EAudioSpectrogramFrequencyAxisPixelBucketMode {
    pub const SAMPLE: EAudioSpectrogramFrequencyAxisPixelBucketMode = EAudioSpectrogramFrequencyAxisPixelBucketMode(
        0,
    );
    pub const PEAK: EAudioSpectrogramFrequencyAxisPixelBucketMode = EAudioSpectrogramFrequencyAxisPixelBucketMode(
        1,
    );
    pub const AVERAGE: EAudioSpectrogramFrequencyAxisPixelBucketMode = EAudioSpectrogramFrequencyAxisPixelBucketMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAudioSpectrogramFrequencyAxisScale(pub u8);
impl EAudioSpectrogramFrequencyAxisScale {
    pub const LINEAR: EAudioSpectrogramFrequencyAxisScale = EAudioSpectrogramFrequencyAxisScale(
        0,
    );
    pub const LOGARITHMIC: EAudioSpectrogramFrequencyAxisScale = EAudioSpectrogramFrequencyAxisScale(
        1,
    );
}
#[repr(transparent)]
pub struct EAudioColorGradient(pub u8);
impl EAudioColorGradient {
    pub const BLACK_TO_WHITE: EAudioColorGradient = EAudioColorGradient(0);
    pub const WHITE_TO_BLACK: EAudioColorGradient = EAudioColorGradient(1);
}
#[repr(transparent)]
pub struct EAudioSpectrumAnalyzerBallistics(pub u8);
impl EAudioSpectrumAnalyzerBallistics {
    pub const ANALOG: EAudioSpectrumAnalyzerBallistics = EAudioSpectrumAnalyzerBallistics(
        0,
    );
    pub const DIGITAL: EAudioSpectrumAnalyzerBallistics = EAudioSpectrumAnalyzerBallistics(
        1,
    );
}
#[repr(transparent)]
pub struct EAudioSpectrumPlotTilt(pub u8);
impl EAudioSpectrumPlotTilt {
    pub const NO_TILT: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(0);
    pub const PLUS1_5D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(1);
    pub const PLUS3D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(2);
    pub const PLUS4_5D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(3);
    pub const PLUS6D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(4);
}
#[repr(transparent)]
pub struct EAudioSpectrumPlotFrequencyAxisPixelBucketMode(pub u8);
impl EAudioSpectrumPlotFrequencyAxisPixelBucketMode {
    pub const SAMPLE: EAudioSpectrumPlotFrequencyAxisPixelBucketMode = EAudioSpectrumPlotFrequencyAxisPixelBucketMode(
        0,
    );
    pub const PEAK: EAudioSpectrumPlotFrequencyAxisPixelBucketMode = EAudioSpectrumPlotFrequencyAxisPixelBucketMode(
        1,
    );
    pub const AVERAGE: EAudioSpectrumPlotFrequencyAxisPixelBucketMode = EAudioSpectrumPlotFrequencyAxisPixelBucketMode(
        2,
    );
}
#[repr(transparent)]
pub struct EAudioSpectrumPlotFrequencyAxisScale(pub u8);
impl EAudioSpectrumPlotFrequencyAxisScale {
    pub const LINEAR: EAudioSpectrumPlotFrequencyAxisScale = EAudioSpectrumPlotFrequencyAxisScale(
        0,
    );
    pub const LOGARITHMIC: EAudioSpectrumPlotFrequencyAxisScale = EAudioSpectrumPlotFrequencyAxisScale(
        1,
    );
}
#[repr(transparent)]
pub struct EAudioRadialSliderLayout(pub u8);
impl EAudioRadialSliderLayout {
    pub const LAYOUT_LABEL_TOP: EAudioRadialSliderLayout = EAudioRadialSliderLayout(0);
    pub const LAYOUT_LABEL_CENTER: EAudioRadialSliderLayout = EAudioRadialSliderLayout(
        1,
    );
    pub const LAYOUT_LABEL_BOTTOM: EAudioRadialSliderLayout = EAudioRadialSliderLayout(
        2,
    );
}
#[repr(transparent)]
pub struct EXAxisLabelsUnit(pub u8);
impl EXAxisLabelsUnit {
    pub const SAMPLES: EXAxisLabelsUnit = EXAxisLabelsUnit(0);
    pub const SECONDS: EXAxisLabelsUnit = EXAxisLabelsUnit(1);
}
#[repr(transparent)]
pub struct EYAxisLabelsUnit(pub u8);
impl EYAxisLabelsUnit {
    pub const LINEAR: EYAxisLabelsUnit = EYAxisLabelsUnit(0);
    pub const DB: EYAxisLabelsUnit = EYAxisLabelsUnit(1);
}
#[repr(transparent)]
pub struct EAudioOscilloscopeTriggerMode(pub u8);
impl EAudioOscilloscopeTriggerMode {
    pub const NONE: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(0);
    pub const RISING: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(1);
    pub const FALLING: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(2);
}
#[repr(transparent)]
pub struct EAudioPanelLayoutType(pub u8);
impl EAudioPanelLayoutType {
    pub const BASIC: EAudioPanelLayoutType = EAudioPanelLayoutType(0);
    pub const ADVANCED: EAudioPanelLayoutType = EAudioPanelLayoutType(1);
}
#[repr(transparent)]
pub struct EAudioUnitsValueType(pub u8);
impl EAudioUnitsValueType {
    pub const LINEAR: EAudioUnitsValueType = EAudioUnitsValueType(0);
    pub const FREQUENCY: EAudioUnitsValueType = EAudioUnitsValueType(1);
    pub const VOLUME: EAudioUnitsValueType = EAudioUnitsValueType(2);
}
