#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FGeometry {}
#[repr(C, align(16))]
pub struct FSlateBrush {
    pub tint_color: FSlateColor,
    pub draw_as: ESlateBrushDrawType,
    pub tiling: ESlateBrushTileType,
    pub mirroring: ESlateBrushMirrorType,
    pub image_type: ESlateBrushImageType,
    pub image_size: FDeprecateSlateVector2D,
    pub margin: FMargin,
    pub tint_deprecated: FLinearColor,
    pub resource_object: UPtr<UObject>,
    pub outline_settings: FSlateBrushOutlineSettings,
    pub uv_region: FBox2f,
    pub flags_164: u8,
    pub resource_name: FName,
}
#[repr(C, align(16))]
pub struct FSlateBrushOutlineSettings {
    pub corner_radii: FVector4,
    pub color: FSlateColor,
    pub width: f32,
    pub rounding_type: ESlateBrushRoundingType,
    pub b_use_brush_transparency: bool,
}
#[repr(C, align(4))]
pub struct FSlateColor {
    pub specified_color: FLinearColor,
    pub color_use_rule: ESlateColorStylingMode,
}
#[repr(C, align(4))]
pub struct FMargin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[repr(C, align(4))]
pub struct FDeprecateSlateVector2D {}
#[repr(C, align(8))]
pub struct FInputEvent {}
#[repr(C, align(8))]
pub struct FPointerEvent {}
#[repr(C, align(8))]
pub struct FSlateWidgetStyle {}
#[repr(C, align(16))]
pub struct FScrollBarStyle {
    pub horizontal_background_image: FSlateBrush,
    pub vertical_background_image: FSlateBrush,
    pub vertical_top_slot_image: FSlateBrush,
    pub horizontal_top_slot_image: FSlateBrush,
    pub vertical_bottom_slot_image: FSlateBrush,
    pub horizontal_bottom_slot_image: FSlateBrush,
    pub normal_thumb_image: FSlateBrush,
    pub hovered_thumb_image: FSlateBrush,
    pub dragged_thumb_image: FSlateBrush,
    pub thickness: f32,
}
#[repr(C, align(16))]
pub struct FTableRowStyle {
    pub selector_focused_brush: FSlateBrush,
    pub active_hovered_brush: FSlateBrush,
    pub active_brush: FSlateBrush,
    pub inactive_hovered_brush: FSlateBrush,
    pub inactive_brush: FSlateBrush,
    pub b_use_parent_row_brush: bool,
    pub parent_row_background_brush: FSlateBrush,
    pub parent_row_background_hovered_brush: FSlateBrush,
    pub even_row_background_hovered_brush: FSlateBrush,
    pub even_row_background_brush: FSlateBrush,
    pub odd_row_background_hovered_brush: FSlateBrush,
    pub odd_row_background_brush: FSlateBrush,
    pub text_color: FSlateColor,
    pub selected_text_color: FSlateColor,
    pub drop_indicator_above: FSlateBrush,
    pub drop_indicator_onto: FSlateBrush,
    pub drop_indicator_below: FSlateBrush,
    pub active_highlighted_brush: FSlateBrush,
    pub inactive_highlighted_brush: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FComboBoxStyle {
    pub combo_button_style: FComboButtonStyle,
    pub pressed_slate_sound: FSlateSound,
    pub selection_change_slate_sound: FSlateSound,
    pub content_padding: FMargin,
    pub menu_row_padding: FMargin,
    pub pressed_sound_deprecated: FName,
    pub selection_change_sound_deprecated: FName,
}
#[repr(C, align(8))]
pub struct FSlateSound {
    pub resource_object: UPtr<UObject>,
}
#[repr(C, align(16))]
pub struct FComboButtonStyle {
    pub button_style: FButtonStyle,
    pub down_arrow_image: FSlateBrush,
    pub shadow_offset: FDeprecateSlateVector2D,
    pub shadow_color_and_opacity: FLinearColor,
    pub menu_border_brush: FSlateBrush,
    pub menu_border_padding: FMargin,
    pub content_padding: FMargin,
    pub down_arrow_padding: FMargin,
    pub down_arrow_align: EVerticalAlignment,
}
#[repr(C, align(16))]
pub struct FButtonStyle {
    pub normal: FSlateBrush,
    pub hovered: FSlateBrush,
    pub pressed: FSlateBrush,
    pub disabled: FSlateBrush,
    pub normal_foreground: FSlateColor,
    pub hovered_foreground: FSlateColor,
    pub pressed_foreground: FSlateColor,
    pub disabled_foreground: FSlateColor,
    pub normal_padding: FMargin,
    pub pressed_padding: FMargin,
    pub pressed_slate_sound: FSlateSound,
    pub clicked_slate_sound: FSlateSound,
    pub hovered_slate_sound: FSlateSound,
    pub pressed_sound_deprecated: FName,
    pub hovered_sound_deprecated: FName,
}
#[repr(C, align(8))]
pub struct FSlateFontInfo {
    pub font_object: UPtr<UObject>,
    pub font_material: UPtr<UObject>,
    pub outline_settings: FFontOutlineSettings,
    pub typeface_font_name: FName,
    pub size: f32,
    pub letter_spacing: i32,
    pub skew_amount: f32,
    pub b_force_monospaced: bool,
    pub b_material_is_stencil: bool,
    pub monospaced_width: f32,
    pub font_name_deprecated: FName,
    pub hinting_deprecated: EFontHinting,
}
#[repr(C, align(8))]
pub struct FFontOutlineSettings {
    pub outline_size: i32,
    pub b_mitered_corners: bool,
    pub b_separate_fill_alpha: bool,
    pub b_apply_outline_to_drop_shadows: bool,
    pub outline_material: UPtr<UObject>,
    pub outline_color: FLinearColor,
}
#[repr(C, align(16))]
pub struct FEditableTextStyle {
    pub font: FSlateFontInfo,
    pub color_and_opacity: FSlateColor,
    pub background_image_selected: FSlateBrush,
    pub background_image_composing: FSlateBrush,
    pub caret_image: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FEditableTextBoxStyle {
    pub background_image_normal: FSlateBrush,
    pub background_image_hovered: FSlateBrush,
    pub background_image_focused: FSlateBrush,
    pub background_image_read_only: FSlateBrush,
    pub padding: FMargin,
    pub font_deprecated: FSlateFontInfo,
    pub text_style: FTextBlockStyle,
    pub foreground_color: FSlateColor,
    pub background_color: FSlateColor,
    pub read_only_foreground_color: FSlateColor,
    pub focused_foreground_color: FSlateColor,
    pub h_scroll_bar_padding: FMargin,
    pub v_scroll_bar_padding: FMargin,
    pub scroll_bar_style: FScrollBarStyle,
}
#[repr(C, align(16))]
pub struct FTextBlockStyle {
    pub font: FSlateFontInfo,
    pub color_and_opacity: FSlateColor,
    pub shadow_offset: FDeprecateSlateVector2D,
    pub shadow_color_and_opacity: FLinearColor,
    pub selected_background_color: FSlateColor,
    pub highlight_color: FSlateColor,
    pub highlight_shape: FSlateBrush,
    pub strike_brush: FSlateBrush,
    pub underline_brush: FSlateBrush,
    pub transform_policy: ETextTransformPolicy,
    pub overflow_policy: ETextOverflowPolicy,
}
#[repr(C, align(16))]
pub struct FSpinBoxStyle {
    pub background_brush: FSlateBrush,
    pub active_background_brush: FSlateBrush,
    pub hovered_background_brush: FSlateBrush,
    pub active_fill_brush: FSlateBrush,
    pub hovered_fill_brush: FSlateBrush,
    pub inactive_fill_brush: FSlateBrush,
    pub arrows_image: FSlateBrush,
    pub foreground_color: FSlateColor,
    pub text_padding: FMargin,
    pub inset_padding: FMargin,
}
#[repr(C, align(8))]
pub struct FCharacterEvent {}
#[repr(C, align(8))]
pub struct FKeyEvent {}
#[repr(C, align(8))]
pub struct FNavigationEvent {}
#[repr(C, align(8))]
pub struct FAnalogInputEvent {}
#[repr(C, align(4))]
pub struct FFontSdfSettings {
    pub base_ppem: i32,
}
#[repr(C, align(8))]
pub struct FFontData {
    pub font_filename: FString,
    pub hinting: EFontHinting,
    pub loading_policy: EFontLoadingPolicy,
    pub sub_face_index: i32,
    pub font_face_asset: UPtr<UObject>,
    pub bulk_data_ptr_deprecated: UPtr<UFontBulkData>,
    pub font_data_deprecated: TArray<u8>,
}
#[repr(C, align(8))]
pub struct FTypefaceEntry {
    pub name: FName,
    pub font: FFontData,
}
#[repr(C, align(8))]
pub struct FTypeface {
    pub fonts: TArray<FTypefaceEntry>,
}
#[repr(C, align(8))]
pub struct FCompositeFallbackFont {
    pub typeface: FTypeface,
    pub scaling_factor: f32,
}
#[repr(C, align(8))]
pub struct FCompositeSubFont {
    pub character_ranges: TArray<FInt32Range>,
    pub cultures: FString,
    pub editor_name: FName,
}
#[repr(C, align(8))]
pub struct FCompositeFont {
    pub default_typeface: FTypeface,
    pub fallback_typeface: FCompositeFallbackFont,
    pub sub_typefaces: TArray<FCompositeSubFont>,
    pub b_enable_ascent_descent_override: bool,
}
#[repr(C, align(4))]
pub struct FFocusEvent {}
#[repr(C, align(4))]
pub struct FCaptureLostEvent {}
#[repr(C, align(8))]
pub struct FMotionEvent {}
#[repr(C, align(8))]
pub struct FNavigationMethod {}
#[repr(C, align(8))]
pub struct FNavigationMethodOrthogonal {}
#[repr(C, align(8))]
pub struct FNavigationMethodProximity {
    pub alignment_factor: f32,
    pub search_angle_degrees: f32,
    pub overlap_threshold: f32,
}
#[repr(C, align(16))]
pub struct FSegmentedControlStyle {
    pub control_style: FCheckBoxStyle,
    pub first_control_style: FCheckBoxStyle,
    pub last_control_style: FCheckBoxStyle,
    pub background_brush: FSlateBrush,
    pub uniform_padding: FMargin,
}
#[repr(C, align(16))]
pub struct FCheckBoxStyle {
    pub check_box_type: ESlateCheckBoxType,
    pub unchecked_image: FSlateBrush,
    pub unchecked_hovered_image: FSlateBrush,
    pub unchecked_pressed_image: FSlateBrush,
    pub checked_image: FSlateBrush,
    pub checked_hovered_image: FSlateBrush,
    pub checked_pressed_image: FSlateBrush,
    pub undetermined_image: FSlateBrush,
    pub undetermined_hovered_image: FSlateBrush,
    pub undetermined_pressed_image: FSlateBrush,
    pub padding: FMargin,
    pub background_image: FSlateBrush,
    pub background_hovered_image: FSlateBrush,
    pub background_pressed_image: FSlateBrush,
    pub foreground_color: FSlateColor,
    pub hovered_foreground: FSlateColor,
    pub pressed_foreground: FSlateColor,
    pub checked_foreground: FSlateColor,
    pub checked_hovered_foreground: FSlateColor,
    pub checked_pressed_foreground: FSlateColor,
    pub undetermined_foreground: FSlateColor,
    pub border_background_color: FSlateColor,
    pub checked_slate_sound: FSlateSound,
    pub unchecked_slate_sound: FSlateSound,
    pub hovered_slate_sound: FSlateSound,
    pub checked_sound_deprecated: FName,
    pub unchecked_sound_deprecated: FName,
    pub hovered_sound_deprecated: FName,
}
#[repr(C, align(16))]
pub struct FHyperlinkStyle {
    pub underline_style: FButtonStyle,
    pub text_style: FTextBlockStyle,
    pub padding: FMargin,
}
#[repr(C, align(16))]
pub struct FInlineEditableTextBlockStyle {
    pub editable_text_box_style: FEditableTextBoxStyle,
    pub text_style: FTextBlockStyle,
}
#[repr(C, align(16))]
pub struct FProgressBarStyle {
    pub background_image: FSlateBrush,
    pub fill_image: FSlateBrush,
    pub marquee_image: FSlateBrush,
    pub enable_fill_animation: bool,
}
#[repr(C, align(16))]
pub struct FExpandableAreaStyle {
    pub collapsed_image: FSlateBrush,
    pub expanded_image: FSlateBrush,
    pub rollout_animation_seconds: f32,
}
#[repr(C, align(16))]
pub struct FSearchBoxStyle {
    pub text_box_style: FEditableTextBoxStyle,
    pub active_font_info: FSlateFontInfo,
    pub up_arrow_image: FSlateBrush,
    pub down_arrow_image: FSlateBrush,
    pub glass_image: FSlateBrush,
    pub clear_image: FSlateBrush,
    pub image_padding: FMargin,
    pub image_size_override: TOptional<FVector2D>,
    pub b_left_align_buttons_deprecated: bool,
    pub b_left_align_search_result_buttons: bool,
    pub b_left_align_glass_image_and_clear_button: bool,
}
#[repr(C, align(16))]
pub struct FSliderStyle {
    pub normal_bar_image: FSlateBrush,
    pub hovered_bar_image: FSlateBrush,
    pub disabled_bar_image: FSlateBrush,
    pub normal_thumb_image: FSlateBrush,
    pub hovered_thumb_image: FSlateBrush,
    pub disabled_thumb_image: FSlateBrush,
    pub bar_thickness: f32,
}
#[repr(C, align(16))]
pub struct FVolumeControlStyle {
    pub slider_style: FSliderStyle,
    pub high_volume_image: FSlateBrush,
    pub mid_volume_image: FSlateBrush,
    pub low_volume_image: FSlateBrush,
    pub no_volume_image: FSlateBrush,
    pub muted_image: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FInlineTextImageStyle {
    pub image: FSlateBrush,
    pub baseline: i16,
}
#[repr(C, align(16))]
pub struct FSplitterStyle {
    pub handle_normal_brush: FSlateBrush,
    pub handle_highlight_brush: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FTableViewStyle {
    pub background_brush: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FTableColumnHeaderStyle {
    pub sort_primary_ascending_image: FSlateBrush,
    pub sort_primary_descending_image: FSlateBrush,
    pub sort_secondary_ascending_image: FSlateBrush,
    pub sort_secondary_descending_image: FSlateBrush,
    pub normal_brush: FSlateBrush,
    pub hovered_brush: FSlateBrush,
    pub menu_dropdown_image: FSlateBrush,
    pub menu_dropdown_normal_border_brush: FSlateBrush,
    pub menu_dropdown_hovered_border_brush: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FHeaderRowStyle {
    pub column_style: FTableColumnHeaderStyle,
    pub last_column_style: FTableColumnHeaderStyle,
    pub column_splitter_style: FSplitterStyle,
    pub splitter_handle_size: f32,
    pub background_brush: FSlateBrush,
    pub foreground_color: FSlateColor,
    pub horizontal_separator_brush: FSlateBrush,
    pub horizontal_separator_thickness: f32,
}
#[repr(C, align(16))]
pub struct FDockTabStyle {
    pub close_button_style: FButtonStyle,
    pub normal_brush: FSlateBrush,
    pub color_overlay_tab_brush: FSlateBrush,
    pub color_overlay_icon_brush: FSlateBrush,
    pub foreground_brush: FSlateBrush,
    pub hovered_brush: FSlateBrush,
    pub content_area_brush: FSlateBrush,
    pub tab_well_brush: FSlateBrush,
    pub tab_text_style: FTextBlockStyle,
    pub tab_padding: FMargin,
    pub icon_size: FDeprecateSlateVector2D,
    pub overlap_width: f32,
    pub flash_color: FSlateColor,
    pub normal_foreground_color: FSlateColor,
    pub hovered_foreground_color: FSlateColor,
    pub active_foreground_color: FSlateColor,
    pub foreground_foreground_color: FSlateColor,
    pub icon_border_padding: f32,
}
#[repr(C, align(16))]
pub struct FScrollBoxStyle {
    pub bar_thickness: f32,
    pub top_shadow_brush: FSlateBrush,
    pub bottom_shadow_brush: FSlateBrush,
    pub left_shadow_brush: FSlateBrush,
    pub right_shadow_brush: FSlateBrush,
    pub horizontal_scrolled_content_padding: FMargin,
    pub vertical_scrolled_content_padding: FMargin,
}
#[repr(C, align(16))]
pub struct FScrollBorderStyle {
    pub top_shadow_brush: FSlateBrush,
    pub bottom_shadow_brush: FSlateBrush,
}
#[repr(C, align(16))]
pub struct FWindowStyle {
    pub minimize_button_style: FButtonStyle,
    pub maximize_button_style: FButtonStyle,
    pub restore_button_style: FButtonStyle,
    pub enter_fullscreen_button_style: FButtonStyle,
    pub exit_fullscreen_button_style: FButtonStyle,
    pub close_button_style: FButtonStyle,
    pub title_text_style: FTextBlockStyle,
    pub active_title_brush: FSlateBrush,
    pub inactive_title_brush: FSlateBrush,
    pub flash_title_brush: FSlateBrush,
    pub background_color: FSlateColor,
    pub outline_brush: FSlateBrush,
    pub outline_color: FSlateColor,
    pub border_brush: FSlateBrush,
    pub border_color: FSlateColor,
    pub background_brush: FSlateBrush,
    pub child_background_brush: FSlateBrush,
    pub window_corner_radius: i32,
    pub border_padding: FMargin,
}
#[repr(C, align(8))]
pub struct FStyleColorList {
    pub style_colors: FLinearColor,
}
#[repr(C, align(8))]
pub struct FStyleTheme {}
#[repr(C, align(16))]
pub struct FWrapButtonStyle {
    pub padding: FMargin,
    pub wrap_button_index: i32,
    pub expand_brush: FSlateBrush,
    pub b_has_down_arrow: bool,
    pub combo_button_style: TOptional<FComboButtonStyle>,
    pub b_include_separator: bool,
    pub separator_brush: TOptional<FSlateBrush>,
    pub separator_thickness: TOptional<f32>,
    pub separator_padding: TOptional<FMargin>,
}
#[repr(C, align(16))]
pub struct FToolBarStyle {
    pub background_brush: FSlateBrush,
    pub expand_brush_deprecated: FSlateBrush,
    pub separator_brush: FSlateBrush,
    pub label_style: FTextBlockStyle,
    pub editable_text_style: FEditableTextBoxStyle,
    pub toggle_button: FCheckBoxStyle,
    pub combo_button_style: FComboButtonStyle,
    pub settings_button_style: FButtonStyle,
    pub settings_combo_button: FComboButtonStyle,
    pub settings_toggle_button: FCheckBoxStyle,
    pub button_style: FButtonStyle,
    pub label_padding: FMargin,
    pub uniform_block_width: f32,
    pub uniform_block_height: f32,
    pub num_columns: i32,
    pub icon_padding: FMargin,
    pub separator_padding: FMargin,
    pub separator_thickness: f32,
    pub combo_button_padding: FMargin,
    pub button_padding: FMargin,
    pub check_box_padding: FMargin,
    pub block_padding: FMargin,
    pub indented_block_padding: FMargin,
    pub block_hovered: FSlateBrush,
    pub background_padding: FMargin,
    pub wrap_button_style: FWrapButtonStyle,
    pub wrap_button_padding_deprecated: FMargin,
    pub wrap_button_index_deprecated: i32,
    pub b_allow_wrap_button: bool,
    pub b_allow_wrapping_default: bool,
    pub icon_size: FDeprecateSlateVector2D,
    pub b_show_labels: bool,
    pub button_content_max_width: f32,
    pub button_text_min_width: f32,
    pub button_text_max_width: f32,
    pub button_content_fill_width: f32,
    pub combo_content_min_width: f32,
    pub combo_content_max_width: f32,
    pub combo_content_horizontal_alignment: EHorizontalAlignment,
    pub icon_padding_with_visible_label: FMargin,
    pub icon_padding_with_collapsed_label: FMargin,
    pub vertical_alignment_override: TOptional<EVerticalAlignment>,
    pub raised_children_right_padding: f32,
}
pub struct USlateWidgetStyleAsset {
    pub custom_style: UPtr<USlateWidgetStyleContainerBase>,
}
pub struct UFontBulkData {}
pub struct UFontFaceInterface {}
pub struct IFontFaceInterface {}
pub struct UFontProviderInterface {}
pub struct IFontProviderInterface {}
pub struct USlateTypes {}
pub struct USlateWidgetStyleContainerBase {}
pub struct USlateWidgetStyleContainerInterface {}
pub struct ISlateWidgetStyleContainerInterface {}
pub struct USlateThemeManager {
    pub current_theme_id: FGuid,
    pub active_colors: FStyleColorList,
}
