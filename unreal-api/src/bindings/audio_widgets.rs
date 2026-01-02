#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FMeterChannelInfo {
    __padding_end: [u8; 12],
}
impl FMeterChannelInfo {}
#[repr(C, align(8))]
pub struct FAudioMaterialWidgetStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub desired_size: crate::bindings::core_u_object::FVector2f,
}
impl FAudioMaterialWidgetStyle {}
#[repr(C, align(8))]
pub struct FAudioMaterialMeterStyle {
    #[doc(hidden)]
    __padding_24: [u8; 24],
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
    __padding_end: [u8; 1248],
}
impl FAudioMeterStyle {}
#[repr(C, align(16))]
pub struct FAudioOscilloscopePanelStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub time_ruler_style: FFixedSampleSequenceRulerStyle,
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub wave_viewer_style: FSampledSequenceViewerStyle,
    pub trigger_threshold_line_style: FTriggerThresholdLineStyle,
    __padding_end: [u8; 8],
}
impl FAudioOscilloscopePanelStyle {}
#[repr(C, align(8))]
pub struct FTriggerThresholdLineStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub line_color: crate::bindings::core_u_object::FLinearColor,
}
impl FTriggerThresholdLineStyle {}
#[repr(C, align(16))]
pub struct FSampledSequenceViewerStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
    __padding_end: [u8; 8],
}
impl FSampledSequenceViewerStyle {}
#[repr(C, align(8))]
pub struct FSampledSequenceValueGridOverlayStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
    __padding_8: [u8; 8],
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
    __padding_end: [u8; 8],
}
impl FFixedSampleSequenceRulerStyle {}
#[repr(C, align(16))]
pub struct FAudioVectorscopePanelStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub value_grid_style: FSampledSequenceValueGridOverlayStyle,
    pub vector_viewer_style: FSampledSequenceVectorViewerStyle,
}
impl FAudioVectorscopePanelStyle {}
#[repr(C, align(16))]
pub struct FSampledSequenceVectorViewerStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub background_brush: crate::bindings::slate_core::FSlateBrush,
    pub line_color: crate::bindings::core_u_object::FLinearColor,
    pub line_thickness: f32,
    __padding_end: [u8; 12],
}
impl FSampledSequenceVectorViewerStyle {}
#[repr(C, align(4))]
pub struct FAudioMaterialEnvelopeSettings {
    #[doc(hidden)]
    __padding_4: [u8; 4],
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
    __padding_24: [u8; 24],
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
    __padding_24: [u8; 24],
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
    __padding_16: [u8; 16],
    pub background_image: crate::bindings::slate_core::FSlateBrush,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    __padding_end: [u8; 12],
}
impl FAudioTextBoxStyle {}
#[repr(C, align(16))]
pub struct FAudioMaterialKnobStyle {
    #[doc(hidden)]
    __padding_24: [u8; 24],
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
    __padding_24: [u8; 24],
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub outline_color: crate::bindings::core_u_object::FLinearColor,
}
impl FAudioMaterialEnvelopeStyle {}
#[repr(C, align(8))]
pub struct FAudioMeterDefaultColorStyle {
    __padding_end: [u8; 104],
}
impl FAudioMeterDefaultColorStyle {}
#[repr(C, align(8))]
pub struct FAudioSpectrumPlotStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
    __padding_16: [u8; 16],
    pub slider_style: crate::bindings::slate_core::FSliderStyle,
    pub text_box_style: FAudioTextBoxStyle,
    pub widget_background_image: crate::bindings::slate_core::FSlateBrush,
    pub slider_background_color: crate::bindings::slate_core::FSlateColor,
    pub slider_background_size: crate::bindings::core_u_object::FVector2D,
    pub label_padding: f32,
    pub slider_bar_color: crate::bindings::slate_core::FSlateColor,
    pub slider_thumb_color: crate::bindings::slate_core::FSlateColor,
    pub widget_background_color: crate::bindings::slate_core::FSlateColor,
    __padding_end: [u8; 8],
}
impl FAudioSliderStyle {}
#[repr(C, align(16))]
pub struct FAudioRadialSliderStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub text_box_style: FAudioTextBoxStyle,
    pub center_background_color: crate::bindings::slate_core::FSlateColor,
    pub slider_bar_color: crate::bindings::slate_core::FSlateColor,
    pub slider_progress_color: crate::bindings::slate_core::FSlateColor,
    pub label_padding: f32,
    pub default_slider_radius: f32,
    __padding_end: [u8; 12],
}
impl FAudioRadialSliderStyle {}
#[repr(C, align(8))]
pub struct FPlayheadOverlayStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
impl UAudioMaterialKnobWidgetStyle {}
#[repr(C, align(8))]
pub struct UAudioMaterialMeterWidgetStyle {
    __padding_end: [u8; 312],
}
impl UAudioMaterialMeterWidgetStyle {}
#[repr(C, align(8))]
pub struct UAudioMaterialButtonWidgetStyle {
    __padding_end: [u8; 192],
}
impl UAudioMaterialButtonWidgetStyle {}
#[repr(C, align(16))]
pub struct UAudioMaterialSliderWidgetStyle {
    __padding_end: [u8; 432],
}
impl UAudioMaterialSliderWidgetStyle {}
#[repr(C, align(8))]
pub struct UAudioMaterialButton {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialButtonStyle,
    #[doc(hidden)]
    __padding_856: [u8; 24],
    pub b_is_pressed: bool,
    __padding_end: [u8; 23],
}
impl UAudioMaterialButton {}
#[repr(C, align(8))]
pub struct UAudioMaterialEnvelope {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialEnvelopeStyle,
    pub envelope_settings: FAudioMaterialEnvelopeSettings,
    __padding_end: [u8; 20],
}
impl UAudioMaterialEnvelope {}
#[repr(C, align(16))]
pub struct UAudioMaterialKnob {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: FAudioMaterialKnobStyle,
    #[doc(hidden)]
    __padding_1208: [u8; 24],
    pub value: f32,
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
    __padding_end: [u8; 20],
}
impl UAudioMaterialKnob {}
#[repr(C, align(8))]
pub struct UAudioMaterialMeter {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub widget_style: FAudioMaterialMeterStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    #[doc(hidden)]
    __padding_1024: [u8; 64],
    pub meter_channel_info: TArray<FMeterChannelInfo>,
}
impl UAudioMaterialMeter {}
#[repr(C, align(16))]
pub struct UAudioMaterialSlider {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: FAudioMaterialSliderStyle,
    #[doc(hidden)]
    __padding_1096: [u8; 24],
    pub value: f32,
    #[doc(hidden)]
    __padding_1104: [u8; 4],
    pub tune_speed: f32,
    pub fine_tune_speed: f32,
    pub b_locked: bool,
    pub b_mouse_uses_step: bool,
    pub step_size: f32,
    __padding_end: [u8; 16],
}
impl UAudioMaterialSlider {}
#[repr(C, align(16))]
pub struct UAudioMeter {
    #[doc(hidden)]
    __padding_752: [u8; 752],
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
impl UAudioMeter {}
#[repr(C, align(16))]
pub struct UAudioOscilloscope {
    #[doc(hidden)]
    __padding_704: [u8; 704],
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
impl UAudioOscilloscope {}
#[repr(C, align(16))]
pub struct UAudioRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioRadialSlider {}
#[repr(C, align(16))]
pub struct UAudioVolumeRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioVolumeRadialSlider {}
#[repr(C, align(16))]
pub struct UAudioFrequencyRadialSlider {
    __padding_end: [u8; 1280],
}
impl UAudioFrequencyRadialSlider {}
#[repr(C, align(16))]
pub struct UAudioSliderBase {
    #[doc(hidden)]
    __padding_1000: [u8; 1000],
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 2023],
}
impl UAudioSliderBase {}
#[repr(C, align(16))]
pub struct UAudioSlider {
    #[doc(hidden)]
    __padding_3016: [u8; 3016],
    pub lin_to_output_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
    pub output_to_lin_curve: TWeakObjectPtr<crate::bindings::engine::UCurveFloat>,
    __padding_end: [u8; 8],
}
impl UAudioSlider {}
#[repr(C, align(16))]
pub struct UAudioVolumeSlider {
    __padding_end: [u8; 3040],
}
impl UAudioVolumeSlider {}
#[repr(C, align(16))]
pub struct UAudioFrequencySlider {
    __padding_end: [u8; 3040],
}
impl UAudioFrequencySlider {}
#[repr(C, align(16))]
pub struct UAudioVectorscope {
    #[doc(hidden)]
    __padding_704: [u8; 704],
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
impl UAudioVectorscope {}
#[repr(transparent)]
pub struct FAudioMaterialButton_OnButtonPressedChangedEvent {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioMaterialKnob_OnKnobValueChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioMaterialMeter_MeterChannelInfoDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioMaterialSlider_OnValueChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioMeter_MeterChannelInfoDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioRadialSlider_ValueDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioRadialSlider_OnValueChanged {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_TextLabelBackgroundColorDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_ValueDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_SliderBackgroundColorDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_SliderBarColorDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_SliderThumbColorDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_WidgetBackgroundColorDelegate {
    _opague: u8,
}
#[repr(transparent)]
pub struct FAudioSliderBase_OnValueChanged {
    _opague: u8,
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
