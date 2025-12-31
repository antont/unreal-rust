#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FMeterChannelInfo {}
#[repr(C, align(8))]
pub struct FAudioMaterialWidgetStyle {
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub desired_size: crate::bindings::core_u_object::FVector2f,
}
#[repr(C, align(8))]
pub struct FAudioMaterialMeterStyle {
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
    pub line_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FSampledSequenceViewerStyle {
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
#[repr(C, align(8))]
pub struct FSampledSequenceValueGridOverlayStyle {
    pub grid_color: crate::bindings::slate_core::FSlateColor,
    pub grid_thickness: f32,
    pub label_text_color: crate::bindings::slate_core::FSlateColor,
    pub label_text_font: crate::bindings::slate_core::FSlateFontInfo,
    pub desired_width: f32,
    pub desired_height: f32,
}
#[repr(C, align(16))]
pub struct FFixedSampleSequenceRulerStyle {
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
#[repr(C, align(16))]
pub struct FAudioVectorscopePanelStyle {
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub vector_viewer_style: FSampledSequenceVectorViewerStyle,
}
#[repr(C, align(16))]
pub struct FSampledSequenceVectorViewerStyle {
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub line_color: crate::bindings::core_u_object::FLinearColor,
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
    pub button_main_color: crate::bindings::core_u_object::FLinearColor,
    pub button_main_color_tint_1: crate::bindings::core_u_object::FLinearColor,
    pub button_main_color_tint_2: crate::bindings::core_u_object::FLinearColor,
    pub button_accent_color: crate::bindings::core_u_object::FLinearColor,
    pub button_shadow_color: crate::bindings::core_u_object::FLinearColor,
    pub button_unpressed_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub button_pressed_outline_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(16))]
pub struct FAudioMaterialSliderStyle {
    pub slider_background_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_background_accent_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_value_main_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_main_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_outline_color: crate::bindings::core_u_object::FLinearColor,
    pub text_box_style: FAudioTextBoxStyle,
}
#[repr(C, align(16))]
pub struct FAudioTextBoxStyle {
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub background_color: crate::bindings::slate_core::FSlateColor,
}
#[repr(C, align(16))]
pub struct FAudioMaterialKnobStyle {
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
#[repr(C, align(8))]
pub struct FAudioMaterialEnvelopeStyle {
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub outline_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorStyle {}
#[repr(C, align(1))]
pub struct FSpectrogramRackUnitSettings {
    pub analyzer_type: EAudioSpectrumAnalyzerType,
    pub fft_analyzer_fft_size: crate::bindings::engine::EFFTSize,
    pub cqt_analyzer_fft_size: crate::bindings::audio_synesthesia::EConstantQFFTSizeEnum,
    pub pixel_plot_mode: EAudioSpectrogramFrequencyAxisPixelBucketMode,
    pub frequency_scale: EAudioSpectrogramFrequencyAxisScale,
    pub color_map: EAudioColorGradient,
    pub orientation: crate::bindings::slate_core::EOrientation,
}
#[repr(C, align(1))]
pub struct FSpectrumAnalyzerRackUnitSettings {
    pub ballistics: EAudioSpectrumAnalyzerBallistics,
    pub analyzer_type: EAudioSpectrumAnalyzerType,
    pub fft_analyzer_fft_size: crate::bindings::engine::EFFTSize,
    pub cqt_analyzer_fft_size: crate::bindings::audio_synesthesia::EConstantQFFTSizeEnum,
    pub tilt_spectrum: EAudioSpectrumPlotTilt,
    pub pixel_plot_mode: EAudioSpectrumPlotFrequencyAxisPixelBucketMode,
    pub frequency_scale: EAudioSpectrumPlotFrequencyAxisScale,
    pub b_display_frequency_axis_labels: bool,
    pub b_display_sound_level_axis_labels: bool,
}
#[repr(C, align(8))]
pub struct FAudioSpectrumPlotStyle {
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub grid_color: crate::bindings::slate_core::FSlateColor,
    pub axis_label_color: crate::bindings::slate_core::FSlateColor,
    pub axis_label_font: crate::bindings::slate_core::FSlateFontInfo,
    pub spectrum_color: crate::bindings::slate_core::FSlateColor,
    pub crosshair_color: crate::bindings::slate_core::FSlateColor,
    pub crosshair_label_font: crate::bindings::slate_core::FSlateFontInfo,
}
#[repr(C, align(16))]
pub struct FAudioSliderStyle {
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
#[repr(C, align(16))]
pub struct FAudioRadialSliderStyle {
    pub text_box_style: FAudioTextBoxStyle,
    pub center_background_color: crate::bindings::slate_core::FSlateColor,
    pub slider_bar_color: crate::bindings::slate_core::FSlateColor,
    pub slider_progress_color: crate::bindings::slate_core::FSlateColor,
    pub label_padding: f32,
    pub default_slider_radius: f32,
}
#[repr(C, align(8))]
pub struct FPlayheadOverlayStyle {
    pub playhead_color: crate::bindings::slate_core::FSlateColor,
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
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub meter_channel_info_delegate: FAudioMaterialMeter_MeterChannelInfoDelegate,
    pub meter_channel_info: TArray<FMeterChannelInfo>,
}
pub struct UAudioMaterialSlider {
    pub widget_style: FAudioMaterialSliderStyle,
    pub on_value_changed: FAudioMaterialSlider_OnValueChanged,
    pub value: f32,
    pub orientation: crate::bindings::slate_core::EOrientation,
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
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_background_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_value_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_peak_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_clipping_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_color: crate::bindings::core_u_object::FLinearColor,
    pub meter_scale_label_color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UAudioOscilloscope {
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
}
pub struct UAudioRadialSlider {
    pub value: f32,
    pub value_delegate: FAudioRadialSlider_ValueDelegate,
    pub widget_layout: EAudioRadialSliderLayout,
    pub center_background_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_progress_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub hand_start_end_ratio: crate::bindings::core_u_object::FVector2D,
    pub units_text: FText,
    pub text_label_background_color: crate::bindings::core_u_object::FLinearColor,
    pub show_label_only_on_hover: bool,
    pub show_units_text: bool,
    pub is_units_text_read_only: bool,
    pub is_value_text_read_only: bool,
    pub slider_thickness: f32,
    pub output_range: crate::bindings::core_u_object::FVector2D,
    pub on_value_changed: FAudioRadialSlider_OnValueChanged,
}
pub struct UAudioVolumeRadialSlider {}
pub struct UAudioFrequencyRadialSlider {}
pub struct UAudioSliderBase {
    pub value: f32,
    pub units_text: FText,
    pub text_label_background_color: crate::bindings::core_u_object::FLinearColor,
    pub text_label_background_color_delegate: FAudioSliderBase_TextLabelBackgroundColorDelegate,
    pub show_label_only_on_hover: bool,
    pub show_units_text: bool,
    pub is_units_text_read_only: bool,
    pub is_value_text_read_only: bool,
    pub value_delegate: FAudioSliderBase_ValueDelegate,
    pub slider_background_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_background_color_delegate: FAudioSliderBase_SliderBackgroundColorDelegate,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_bar_color_delegate: FAudioSliderBase_SliderBarColorDelegate,
    pub slider_thumb_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_thumb_color_delegate: FAudioSliderBase_SliderThumbColorDelegate,
    pub widget_background_color: crate::bindings::core_u_object::FLinearColor,
    pub widget_background_color_delegate: FAudioSliderBase_WidgetBackgroundColorDelegate,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub on_value_changed: FAudioSliderBase_OnValueChanged,
}
pub struct UAudioSlider {
    pub lin_to_output_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
    pub output_to_lin_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
}
pub struct UAudioVolumeSlider {}
pub struct UAudioFrequencySlider {
    pub output_range: crate::bindings::core_u_object::FVector2D,
}
pub struct UAudioVectorscope {
    pub vectorscope_style: FAudioVectorscopePanelStyle,
    pub audio_bus: UPtr<crate::bindings::engine::UAudioBus>,
    pub b_show_grid: bool,
    pub grid_divisions: i32,
    pub max_display_persistence_ms: f32,
    pub display_persistence_ms: f32,
    pub scale: f32,
    pub panel_layout_type: EAudioPanelLayoutType,
}
pub struct FAudioMaterialButton_OnButtonPressedChangedEvent;
pub struct FAudioMaterialKnob_OnKnobValueChanged;
pub struct FAudioMaterialMeter_MeterChannelInfoDelegate;
pub struct FAudioMaterialSlider_OnValueChanged;
pub struct FAudioMeter_MeterChannelInfoDelegate;
pub struct FAudioRadialSlider_ValueDelegate;
pub struct FAudioRadialSlider_OnValueChanged;
pub struct FAudioSliderBase_TextLabelBackgroundColorDelegate;
pub struct FAudioSliderBase_ValueDelegate;
pub struct FAudioSliderBase_SliderBackgroundColorDelegate;
pub struct FAudioSliderBase_SliderBarColorDelegate;
pub struct FAudioSliderBase_SliderThumbColorDelegate;
pub struct FAudioSliderBase_WidgetBackgroundColorDelegate;
pub struct FAudioSliderBase_OnValueChanged;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioMaterialEnvelopeType(pub u8);
impl EAudioMaterialEnvelopeType {
    pub const AD: EAudioMaterialEnvelopeType = EAudioMaterialEnvelopeType(0);
    pub const ADSR: EAudioMaterialEnvelopeType = EAudioMaterialEnvelopeType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioSpectrumAnalyzerType(pub u8);
impl EAudioSpectrumAnalyzerType {
    pub const FFT: EAudioSpectrumAnalyzerType = EAudioSpectrumAnalyzerType(0);
    pub const CQT: EAudioSpectrumAnalyzerType = EAudioSpectrumAnalyzerType(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioColorGradient(pub u8);
impl EAudioColorGradient {
    pub const BLACK_TO_WHITE: EAudioColorGradient = EAudioColorGradient(0);
    pub const WHITE_TO_BLACK: EAudioColorGradient = EAudioColorGradient(1);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioSpectrumPlotTilt(pub u8);
impl EAudioSpectrumPlotTilt {
    pub const NO_TILT: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(0);
    pub const PLUS1_5D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(1);
    pub const PLUS3D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(2);
    pub const PLUS4_5D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(3);
    pub const PLUS6D_B_PER_OCTAVE: EAudioSpectrumPlotTilt = EAudioSpectrumPlotTilt(4);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EXAxisLabelsUnit(pub u8);
impl EXAxisLabelsUnit {
    pub const SAMPLES: EXAxisLabelsUnit = EXAxisLabelsUnit(0);
    pub const SECONDS: EXAxisLabelsUnit = EXAxisLabelsUnit(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EYAxisLabelsUnit(pub u8);
impl EYAxisLabelsUnit {
    pub const LINEAR: EYAxisLabelsUnit = EYAxisLabelsUnit(0);
    pub const DB: EYAxisLabelsUnit = EYAxisLabelsUnit(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioOscilloscopeTriggerMode(pub u8);
impl EAudioOscilloscopeTriggerMode {
    pub const NONE: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(0);
    pub const RISING: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(1);
    pub const FALLING: EAudioOscilloscopeTriggerMode = EAudioOscilloscopeTriggerMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioPanelLayoutType(pub u8);
impl EAudioPanelLayoutType {
    pub const BASIC: EAudioPanelLayoutType = EAudioPanelLayoutType(0);
    pub const ADVANCED: EAudioPanelLayoutType = EAudioPanelLayoutType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EAudioUnitsValueType(pub u8);
impl EAudioUnitsValueType {
    pub const LINEAR: EAudioUnitsValueType = EAudioUnitsValueType(0);
    pub const FREQUENCY: EAudioUnitsValueType = EAudioUnitsValueType(1);
    pub const VOLUME: EAudioUnitsValueType = EAudioUnitsValueType(2);
}
