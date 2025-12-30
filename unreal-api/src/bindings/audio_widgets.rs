#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FMeterChannelInfo {}
#[repr(C, align(8))]
pub struct FAudioMaterialWidgetStyle {
    pub material: UPtr<UMaterialInterface>,
    pub desired_size: FVector2f,
}
#[repr(C, align(8))]
pub struct FAudioMaterialMeterStyle {
    pub meter_fill_min_color: FLinearColor,
    pub meter_fill_mid_color: FLinearColor,
    pub meter_fill_max_color: FLinearColor,
    pub meter_fill_background_color: FLinearColor,
    pub meter_padding: FVector2D,
    pub value_range_db: FVector2D,
    pub b_show_scale: bool,
    pub b_scale_side: bool,
    pub scale_hash_offset: f32,
    pub scale_hash_width: f32,
    pub scale_hash_height: f32,
    pub decibels_per_hash: i32,
    pub font: FSlateFontInfo,
}
#[repr(C, align(16))]
pub struct FAudioMeterStyle {}
#[repr(C, align(16))]
pub struct FAudioOscilloscopePanelStyle {
    pub time_ruler_style: FFixedSampleSequenceRulerStyle,
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub wave_viewer_style: FSampledSequenceViewerStyle,
    pub trigger_threshold_line_style: FTriggerThresholdLineStyle,
}
#[repr(C, align(8))]
pub struct FTriggerThresholdLineStyle {
    pub line_color: FLinearColor,
}
#[repr(C, align(16))]
pub struct FSampledSequenceViewerStyle {
    pub sequence_color: FSlateColor,
    pub sequence_line_thickness: f32,
    pub major_grid_line_color: FSlateColor,
    pub minor_grid_line_color: FSlateColor,
    pub zero_crossing_line_color: FSlateColor,
    pub zero_crossing_line_thickness: f32,
    pub sample_markers_size: f32,
    pub sequence_background_color: FSlateColor,
    pub background_brush: FSlateBrush,
    pub desired_width: f32,
    pub desired_height: f32,
}
#[repr(C, align(8))]
pub struct FSampledSequenceValueGridOverlayStyle {
    pub grid_color: FSlateColor,
    pub grid_thickness: f32,
    pub label_text_color: FSlateColor,
    pub label_text_font: FSlateFontInfo,
    pub desired_width: f32,
    pub desired_height: f32,
}
#[repr(C, align(16))]
pub struct FFixedSampleSequenceRulerStyle {
    pub handle_width: f32,
    pub handle_color: FSlateColor,
    pub handle_brush: FSlateBrush,
    pub ticks_color: FSlateColor,
    pub ticks_text_color: FSlateColor,
    pub ticks_text_font: FSlateFontInfo,
    pub ticks_text_offset: f32,
    pub background_color: FSlateColor,
    pub background_brush: FSlateBrush,
    pub desired_width: f32,
    pub desired_height: f32,
}
#[repr(C, align(16))]
pub struct FAudioVectorscopePanelStyle {
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub vector_viewer_style: FSampledSequenceVectorViewerStyle,
}
#[repr(C, align(16))]
pub struct FSampledSequenceVectorViewerStyle {
    pub background_color: FSlateColor,
    pub background_brush: FSlateBrush,
    pub line_color: FLinearColor,
    pub line_thickness: f32,
}
#[repr(C, align(4))]
pub struct FAudioMaterialEnvelopeSettings {
    pub envelope_type: EAudioMaterialEnvelopeType,
    pub attack_curve: f32,
    pub attack_value: f32,
    pub attack_time: f32,
    pub decay_curve: f32,
    pub decay_time: f32,
    pub sustain_value: f32,
    pub release_curve: f32,
    pub release_time: f32,
}
#[repr(C, align(1))]
pub struct FLoudnessMetricDisplayOptions {
    pub b_show_value: bool,
    pub b_show_meter: bool,
}
#[repr(C, align(1))]
pub struct FLoudnessMeterRackUnitSettings {
    pub b_display_analysis_timer: bool,
    pub long_term_loudness: FLoudnessMetricDisplayOptions,
    pub short_term_loudness: FLoudnessMetricDisplayOptions,
    pub momentary_loudness: FLoudnessMetricDisplayOptions,
}
#[repr(C, align(8))]
pub struct FAudioMaterialButtonStyle {
    pub button_main_color: FLinearColor,
    pub button_main_color_tint_1: FLinearColor,
    pub button_main_color_tint_2: FLinearColor,
    pub button_accent_color: FLinearColor,
    pub button_shadow_color: FLinearColor,
    pub button_unpressed_outline_color: FLinearColor,
    pub button_pressed_outline_color: FLinearColor,
}
#[repr(C, align(16))]
pub struct FAudioMaterialSliderStyle {
    pub slider_background_color: FLinearColor,
    pub slider_background_accent_color: FLinearColor,
    pub slider_value_main_color: FLinearColor,
    pub slider_handle_main_color: FLinearColor,
    pub slider_handle_outline_color: FLinearColor,
    pub text_box_style: FAudioTextBoxStyle,
}
#[repr(C, align(16))]
pub struct FAudioTextBoxStyle {
    pub background_image: FSlateBrush,
    pub background_color: FSlateColor,
}
#[repr(C, align(16))]
pub struct FAudioMaterialKnobStyle {
    pub knob_main_color: FLinearColor,
    pub knob_accent_color: FLinearColor,
    pub knob_shadow_color: FLinearColor,
    pub knob_smooth_bevel_color: FLinearColor,
    pub knob_indicator_dot_color: FLinearColor,
    pub knob_edge_fill_color: FLinearColor,
    pub knob_bar_color: FLinearColor,
    pub knob_bar_shadow_color: FLinearColor,
    pub knob_bar_fill_min_color: FLinearColor,
    pub knob_bar_fill_mid_color: FLinearColor,
    pub knob_bar_fill_max_color: FLinearColor,
    pub knob_bar_fill_tint_color: FLinearColor,
    pub text_box_style: FAudioTextBoxStyle,
}
#[repr(C, align(8))]
pub struct FAudioMaterialEnvelopeStyle {
    pub curve_color: FLinearColor,
    pub background_color: FLinearColor,
    pub outline_color: FLinearColor,
}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorStyle {}
#[repr(C, align(1))]
pub struct FSpectrogramRackUnitSettings {
    pub analyzer_type: EAudioSpectrumAnalyzerType,
    pub fft_analyzer_fft_size: EFFTSize,
    pub cqt_analyzer_fft_size: EConstantQFFTSizeEnum,
    pub pixel_plot_mode: EAudioSpectrogramFrequencyAxisPixelBucketMode,
    pub frequency_scale: EAudioSpectrogramFrequencyAxisScale,
    pub color_map: EAudioColorGradient,
    pub orientation: EOrientation,
}
#[repr(C, align(1))]
pub struct FSpectrumAnalyzerRackUnitSettings {
    pub ballistics: EAudioSpectrumAnalyzerBallistics,
    pub analyzer_type: EAudioSpectrumAnalyzerType,
    pub fft_analyzer_fft_size: EFFTSize,
    pub cqt_analyzer_fft_size: EConstantQFFTSizeEnum,
    pub tilt_spectrum: EAudioSpectrumPlotTilt,
    pub pixel_plot_mode: EAudioSpectrumPlotFrequencyAxisPixelBucketMode,
    pub frequency_scale: EAudioSpectrumPlotFrequencyAxisScale,
    pub b_display_frequency_axis_labels: bool,
    pub b_display_sound_level_axis_labels: bool,
}
#[repr(C, align(8))]
pub struct FAudioSpectrumPlotStyle {
    pub background_color: FSlateColor,
    pub grid_color: FSlateColor,
    pub axis_label_color: FSlateColor,
    pub axis_label_font: FSlateFontInfo,
    pub spectrum_color: FSlateColor,
    pub crosshair_color: FSlateColor,
    pub crosshair_label_font: FSlateFontInfo,
}
#[repr(C, align(16))]
pub struct FAudioSliderStyle {
    pub slider_style: FSliderStyle,
    pub text_box_style: FAudioTextBoxStyle,
    pub widget_background_image: FSlateBrush,
    pub slider_background_color: FSlateColor,
    pub slider_background_size: FVector2D,
    pub label_padding: f32,
    pub slider_bar_color: FSlateColor,
    pub slider_thumb_color: FSlateColor,
    pub widget_background_color: FSlateColor,
}
#[repr(C, align(16))]
pub struct FAudioRadialSliderStyle {
    pub text_box_style: FAudioTextBoxStyle,
    pub center_background_color: FSlateColor,
    pub slider_bar_color: FSlateColor,
    pub slider_progress_color: FSlateColor,
    pub label_padding: f32,
    pub default_slider_radius: f32,
}
#[repr(C, align(8))]
pub struct FPlayheadOverlayStyle {
    pub playhead_color: FSlateColor,
    pub playhead_width: f32,
    pub desired_width: f32,
    pub desired_height: f32,
}
pub struct UAudioMaterialKnobWidgetStyle {
    pub knob_style: FAudioMaterialKnobStyle,
}
pub struct UAudioMaterialMeterWidgetStyle {
    pub meter_style: FAudioMaterialMeterStyle,
}
pub struct UAudioMaterialButtonWidgetStyle {
    pub button_style: FAudioMaterialButtonStyle,
}
pub struct UAudioMaterialSliderWidgetStyle {
    pub slider_style: FAudioMaterialSliderStyle,
}
pub struct UAudioMaterialButton {
    pub widget_style: FAudioMaterialButtonStyle,
    pub on_button_pressed_changed_event: FAudioMaterialButton_OnButtonPressedChangedEvent,
    pub b_is_pressed: bool,
}
pub struct UAudioMaterialEnvelope {
    pub widget_style: FAudioMaterialEnvelopeStyle,
    pub envelope_settings: FAudioMaterialEnvelopeSettings,
}
pub struct UAudioMaterialKnob {
    pub widget_style: FAudioMaterialKnobStyle,
    pub on_knob_value_changed: FAudioMaterialKnob_OnKnobValueChanged,
    pub value: f32,
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
}
pub struct UAudioMaterialMeter {
    pub widget_style: FAudioMaterialMeterStyle,
    pub orientation: EOrientation,
    pub meter_channel_info_delegate: FAudioMaterialMeter_MeterChannelInfoDelegate,
    pub meter_channel_info: TArray<FMeterChannelInfo>,
}
pub struct UAudioMaterialSlider {
    pub widget_style: FAudioMaterialSliderStyle,
    pub on_value_changed: FAudioMaterialSlider_OnValueChanged,
    pub value: f32,
    pub orientation: EOrientation,
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
}
pub struct UAudioMeter {
    pub meter_channel_info: TArray<FMeterChannelInfo>,
    pub meter_channel_info_delegate: FAudioMeter_MeterChannelInfoDelegate,
    pub widget_style: FAudioMeterStyle,
    pub orientation: EOrientation,
    pub background_color: FLinearColor,
    pub meter_background_color: FLinearColor,
    pub meter_value_color: FLinearColor,
    pub meter_peak_color: FLinearColor,
    pub meter_clipping_color: FLinearColor,
    pub meter_scale_color: FLinearColor,
    pub meter_scale_label_color: FLinearColor,
}
pub struct UAudioOscilloscope {
    pub oscilloscope_style: FAudioOscilloscopePanelStyle,
    pub audio_bus: UPtr<UAudioBus>,
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
}
pub struct UAudioRadialSlider {
    pub value: f32,
    pub value_delegate: FAudioRadialSlider_ValueDelegate,
    pub widget_layout: EAudioRadialSliderLayout,
    pub center_background_color: FLinearColor,
    pub slider_progress_color: FLinearColor,
    pub slider_bar_color: FLinearColor,
    pub hand_start_end_ratio: FVector2D,
    pub units_text: FText,
    pub text_label_background_color: FLinearColor,
    pub show_label_only_on_hover: bool,
    pub show_units_text: bool,
    pub is_units_text_read_only: bool,
    pub is_value_text_read_only: bool,
    pub slider_thickness: f32,
    pub output_range: FVector2D,
    pub on_value_changed: FAudioRadialSlider_OnValueChanged,
}
pub struct UAudioVolumeRadialSlider {}
pub struct UAudioFrequencyRadialSlider {}
pub struct UAudioSliderBase {
    pub value: f32,
    pub units_text: FText,
    pub text_label_background_color: FLinearColor,
    pub text_label_background_color_delegate: FAudioSliderBase_TextLabelBackgroundColorDelegate,
    pub show_label_only_on_hover: bool,
    pub show_units_text: bool,
    pub is_units_text_read_only: bool,
    pub is_value_text_read_only: bool,
    pub value_delegate: FAudioSliderBase_ValueDelegate,
    pub slider_background_color: FLinearColor,
    pub slider_background_color_delegate: FAudioSliderBase_SliderBackgroundColorDelegate,
    pub slider_bar_color: FLinearColor,
    pub slider_bar_color_delegate: FAudioSliderBase_SliderBarColorDelegate,
    pub slider_thumb_color: FLinearColor,
    pub slider_thumb_color_delegate: FAudioSliderBase_SliderThumbColorDelegate,
    pub widget_background_color: FLinearColor,
    pub widget_background_color_delegate: FAudioSliderBase_WidgetBackgroundColorDelegate,
    pub orientation: EOrientation,
    pub on_value_changed: FAudioSliderBase_OnValueChanged,
}
pub struct UAudioSlider {
    pub lin_to_output_curve: TWeakObjectPtr<UCurveFloat>,
    pub output_to_lin_curve: TWeakObjectPtr<UCurveFloat>,
}
pub struct UAudioVolumeSlider {}
pub struct UAudioFrequencySlider {
    pub output_range: FVector2D,
}
pub struct UAudioVectorscope {
    pub vectorscope_style: FAudioVectorscopePanelStyle,
    pub audio_bus: UPtr<UAudioBus>,
    pub b_show_grid: bool,
    pub grid_divisions: i32,
    pub max_display_persistence_ms: f32,
    pub display_persistence_ms: f32,
    pub scale: f32,
    pub panel_layout_type: EAudioPanelLayoutType,
}
