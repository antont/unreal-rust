#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
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
    pub tint_deprecated: crate::bindings::core_u_object::FLinearColor,
    pub resource_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub outline_settings: FSlateBrushOutlineSettings,
    pub uv_region: crate::bindings::core_u_object::FBox2f,
    pub flags_164: u8,
    pub resource_name: FName,
}
#[repr(C, align(16))]
pub struct FSlateBrushOutlineSettings {
    pub corner_radii: crate::bindings::core_u_object::FVector4,
    pub color: FSlateColor,
    pub width: f32,
    pub rounding_type: ESlateBrushRoundingType,
    pub b_use_brush_transparency: bool,
}
#[repr(C, align(4))]
pub struct FSlateColor {
    pub specified_color: crate::bindings::core_u_object::FLinearColor,
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
    pub resource_object: UPtr<crate::bindings::core_u_object::UObject>,
}
#[repr(C, align(16))]
pub struct FComboButtonStyle {
    pub button_style: FButtonStyle,
    pub down_arrow_image: FSlateBrush,
    pub shadow_offset: FDeprecateSlateVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
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
    pub font_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub font_material: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub outline_material: UPtr<crate::bindings::core_u_object::UObject>,
    pub outline_color: crate::bindings::core_u_object::FLinearColor,
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
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
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
    pub font_face_asset: UPtr<crate::bindings::core_u_object::UObject>,
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
    pub character_ranges: TArray<crate::bindings::core_u_object::FInt32Range>,
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
    pub image_size_override: TOptional<crate::bindings::core_u_object::FVector2D>,
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
    pub style_colors: crate::bindings::core_u_object::FLinearColor,
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
    pub current_theme_id: crate::bindings::core_u_object::FGuid,
    pub active_colors: FStyleColorList,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateColorStylingMode(pub u8);
impl ESlateColorStylingMode {
    pub const USE_COLOR_SPECIFIED: ESlateColorStylingMode = ESlateColorStylingMode(0);
    pub const USE_COLOR_COLOR_TABLE: ESlateColorStylingMode = ESlateColorStylingMode(1);
    pub const USE_COLOR_FOREGROUND: ESlateColorStylingMode = ESlateColorStylingMode(2);
    pub const USE_COLOR_FOREGROUND_SUBDUED: ESlateColorStylingMode = ESlateColorStylingMode(
        3,
    );
    pub const USE_COLOR_USE_STYLE: ESlateColorStylingMode = ESlateColorStylingMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateBrushDrawType(pub u8);
impl ESlateBrushDrawType {
    pub const NO_DRAW_TYPE: ESlateBrushDrawType = ESlateBrushDrawType(0);
    pub const BOX: ESlateBrushDrawType = ESlateBrushDrawType(1);
    pub const BORDER: ESlateBrushDrawType = ESlateBrushDrawType(2);
    pub const IMAGE: ESlateBrushDrawType = ESlateBrushDrawType(3);
    pub const ROUNDED_BOX: ESlateBrushDrawType = ESlateBrushDrawType(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateBrushTileType(pub u8);
impl ESlateBrushTileType {
    pub const NO_TILE: ESlateBrushTileType = ESlateBrushTileType(0);
    pub const HORIZONTAL: ESlateBrushTileType = ESlateBrushTileType(1);
    pub const VERTICAL: ESlateBrushTileType = ESlateBrushTileType(2);
    pub const BOTH: ESlateBrushTileType = ESlateBrushTileType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateBrushMirrorType(pub u8);
impl ESlateBrushMirrorType {
    pub const NO_MIRROR: ESlateBrushMirrorType = ESlateBrushMirrorType(0);
    pub const HORIZONTAL: ESlateBrushMirrorType = ESlateBrushMirrorType(1);
    pub const VERTICAL: ESlateBrushMirrorType = ESlateBrushMirrorType(2);
    pub const BOTH: ESlateBrushMirrorType = ESlateBrushMirrorType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateBrushImageType(pub u8);
impl ESlateBrushImageType {
    pub const NO_IMAGE: ESlateBrushImageType = ESlateBrushImageType(0);
    pub const FULL_COLOR: ESlateBrushImageType = ESlateBrushImageType(1);
    pub const LINEAR: ESlateBrushImageType = ESlateBrushImageType(2);
    pub const VECTOR: ESlateBrushImageType = ESlateBrushImageType(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateBrushRoundingType(pub u8);
impl ESlateBrushRoundingType {
    pub const FIXED_RADIUS: ESlateBrushRoundingType = ESlateBrushRoundingType(0);
    pub const HALF_HEIGHT_RADIUS: ESlateBrushRoundingType = ESlateBrushRoundingType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EVerticalAlignment(pub u8);
impl EVerticalAlignment {
    pub const V_ALIGN_FILL: EVerticalAlignment = EVerticalAlignment(0);
    pub const V_ALIGN_TOP: EVerticalAlignment = EVerticalAlignment(1);
    pub const V_ALIGN_CENTER: EVerticalAlignment = EVerticalAlignment(2);
    pub const V_ALIGN_BOTTOM: EVerticalAlignment = EVerticalAlignment(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontHinting(pub u8);
impl EFontHinting {
    pub const DEFAULT: EFontHinting = EFontHinting(0);
    pub const AUTO: EFontHinting = EFontHinting(1);
    pub const AUTO_LIGHT: EFontHinting = EFontHinting(2);
    pub const MONOCHROME: EFontHinting = EFontHinting(3);
    pub const NONE: EFontHinting = EFontHinting(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextShapingMethod(pub u8);
impl ETextShapingMethod {
    pub const AUTO: ETextShapingMethod = ETextShapingMethod(0);
    pub const KERNING_ONLY: ETextShapingMethod = ETextShapingMethod(1);
    pub const FULL_SHAPING: ETextShapingMethod = ETextShapingMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextTransformPolicy(pub u8);
impl ETextTransformPolicy {
    pub const NONE: ETextTransformPolicy = ETextTransformPolicy(0);
    pub const TO_LOWER: ETextTransformPolicy = ETextTransformPolicy(1);
    pub const TO_UPPER: ETextTransformPolicy = ETextTransformPolicy(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextOverflowPolicy(pub u8);
impl ETextOverflowPolicy {
    pub const CLIP: ETextOverflowPolicy = ETextOverflowPolicy(0);
    pub const ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(1);
    pub const MULTILINE_ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(2);
    pub const MIDDLE_ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontLoadingPolicy(pub u8);
impl EFontLoadingPolicy {
    pub const LAZY_LOAD: EFontLoadingPolicy = EFontLoadingPolicy(0);
    pub const STREAM: EFontLoadingPolicy = EFontLoadingPolicy(1);
    pub const INLINE: EFontLoadingPolicy = EFontLoadingPolicy(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateCheckBoxType(pub u8);
impl ESlateCheckBoxType {
    pub const CHECK_BOX: ESlateCheckBoxType = ESlateCheckBoxType(0);
    pub const TOGGLE_BUTTON: ESlateCheckBoxType = ESlateCheckBoxType(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHorizontalAlignment(pub u8);
impl EHorizontalAlignment {
    pub const H_ALIGN_FILL: EHorizontalAlignment = EHorizontalAlignment(0);
    pub const H_ALIGN_LEFT: EHorizontalAlignment = EHorizontalAlignment(1);
    pub const H_ALIGN_CENTER: EHorizontalAlignment = EHorizontalAlignment(2);
    pub const H_ALIGN_RIGHT: EHorizontalAlignment = EHorizontalAlignment(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ECheckBoxState(pub u8);
impl ECheckBoxState {
    pub const UNCHECKED: ECheckBoxState = ECheckBoxState(0);
    pub const CHECKED: ECheckBoxState = ECheckBoxState(1);
    pub const UNDETERMINED: ECheckBoxState = ECheckBoxState(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ENavigationGenesis(pub u8);
impl ENavigationGenesis {
    pub const KEYBOARD: ENavigationGenesis = ENavigationGenesis(0);
    pub const CONTROLLER: ENavigationGenesis = ENavigationGenesis(1);
    pub const USER: ENavigationGenesis = ENavigationGenesis(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUINavigation(pub u8);
impl EUINavigation {
    pub const LEFT: EUINavigation = EUINavigation(0);
    pub const RIGHT: EUINavigation = EUINavigation(1);
    pub const UP: EUINavigation = EUINavigation(2);
    pub const DOWN: EUINavigation = EUINavigation(3);
    pub const NEXT: EUINavigation = EUINavigation(4);
    pub const PREVIOUS: EUINavigation = EUINavigation(5);
    pub const NUM: EUINavigation = EUINavigation(6);
    pub const INVALID: EUINavigation = EUINavigation(7);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUINavigationRule(pub u8);
impl EUINavigationRule {
    pub const ESCAPE: EUINavigationRule = EUINavigationRule(0);
    pub const EXPLICIT: EUINavigationRule = EUINavigationRule(1);
    pub const WRAP: EUINavigationRule = EUINavigationRule(2);
    pub const STOP: EUINavigationRule = EUINavigationRule(3);
    pub const CUSTOM: EUINavigationRule = EUINavigationRule(4);
    pub const CUSTOM_BOUNDARY: EUINavigationRule = EUINavigationRule(5);
    pub const INVALID: EUINavigationRule = EUINavigationRule(6);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlatePostRT(pub u8);
impl ESlatePostRT {
    pub const NONE: ESlatePostRT = ESlatePostRT(0);
    pub const E_SLATE_POST_RT_0: ESlatePostRT = ESlatePostRT(1);
    pub const E_SLATE_POST_RT_1: ESlatePostRT = ESlatePostRT(2);
    pub const E_SLATE_POST_RT_2: ESlatePostRT = ESlatePostRT(4);
    pub const E_SLATE_POST_RT_3: ESlatePostRT = ESlatePostRT(8);
    pub const E_SLATE_POST_RT_4: ESlatePostRT = ESlatePostRT(16);
    pub const ALL: ESlatePostRT = ESlatePostRT(31);
    pub const NUM: ESlatePostRT = ESlatePostRT(5);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontRasterizationMode(pub u8);
impl EFontRasterizationMode {
    pub const BITMAP: EFontRasterizationMode = EFontRasterizationMode(0);
    pub const MSDF: EFontRasterizationMode = EFontRasterizationMode(1);
    pub const SDF: EFontRasterizationMode = EFontRasterizationMode(2);
    pub const SDF_APPROXIMATION: EFontRasterizationMode = EFontRasterizationMode(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EOrientation(pub u8);
impl EOrientation {
    pub const ORIENT_HORIZONTAL: EOrientation = EOrientation(0);
    pub const ORIENT_VERTICAL: EOrientation = EOrientation(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetClipping(pub u8);
impl EWidgetClipping {
    pub const INHERIT: EWidgetClipping = EWidgetClipping(0);
    pub const CLIP_TO_BOUNDS: EWidgetClipping = EWidgetClipping(1);
    pub const CLIP_TO_BOUNDS_WITHOUT_INTERSECTING: EWidgetClipping = EWidgetClipping(2);
    pub const CLIP_TO_BOUNDS_ALWAYS: EWidgetClipping = EWidgetClipping(3);
    pub const ON_DEMAND: EWidgetClipping = EWidgetClipping(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EMenuPlacement(pub u8);
impl EMenuPlacement {
    pub const MENU_PLACEMENT_BELOW_ANCHOR: EMenuPlacement = EMenuPlacement(0);
    pub const MENU_PLACEMENT_CENTERED_BELOW_ANCHOR: EMenuPlacement = EMenuPlacement(1);
    pub const MENU_PLACEMENT_BELOW_RIGHT_ANCHOR: EMenuPlacement = EMenuPlacement(2);
    pub const MENU_PLACEMENT_COMBO_BOX: EMenuPlacement = EMenuPlacement(3);
    pub const MENU_PLACEMENT_COMBO_BOX_RIGHT: EMenuPlacement = EMenuPlacement(4);
    pub const MENU_PLACEMENT_MENU_RIGHT: EMenuPlacement = EMenuPlacement(5);
    pub const MENU_PLACEMENT_ABOVE_ANCHOR: EMenuPlacement = EMenuPlacement(6);
    pub const MENU_PLACEMENT_CENTERED_ABOVE_ANCHOR: EMenuPlacement = EMenuPlacement(7);
    pub const MENU_PLACEMENT_ABOVE_RIGHT_ANCHOR: EMenuPlacement = EMenuPlacement(8);
    pub const MENU_PLACEMENT_MENU_LEFT: EMenuPlacement = EMenuPlacement(9);
    pub const MENU_PLACEMENT_CENTER: EMenuPlacement = EMenuPlacement(10);
    pub const MENU_PLACEMENT_RIGHT_LEFT_CENTER: EMenuPlacement = EMenuPlacement(11);
    pub const MENU_PLACEMENT_MATCH_BOTTOM_LEFT: EMenuPlacement = EMenuPlacement(12);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EColorVisionDeficiency(pub u8);
impl EColorVisionDeficiency {
    pub const NORMAL_VISION: EColorVisionDeficiency = EColorVisionDeficiency(0);
    pub const DEUTERANOPE: EColorVisionDeficiency = EColorVisionDeficiency(1);
    pub const PROTANOPE: EColorVisionDeficiency = EColorVisionDeficiency(2);
    pub const TRITANOPE: EColorVisionDeficiency = EColorVisionDeficiency(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EButtonClickMethod(pub u8);
impl EButtonClickMethod {
    pub const DOWN_AND_UP: EButtonClickMethod = EButtonClickMethod(0);
    pub const MOUSE_DOWN: EButtonClickMethod = EButtonClickMethod(1);
    pub const MOUSE_UP: EButtonClickMethod = EButtonClickMethod(2);
    pub const PRECISE_CLICK: EButtonClickMethod = EButtonClickMethod(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EButtonPressMethod(pub u8);
impl EButtonPressMethod {
    pub const DOWN_AND_UP: EButtonPressMethod = EButtonPressMethod(0);
    pub const BUTTON_PRESS: EButtonPressMethod = EButtonPressMethod(1);
    pub const BUTTON_RELEASE: EButtonPressMethod = EButtonPressMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EButtonTouchMethod(pub u8);
impl EButtonTouchMethod {
    pub const DOWN_AND_UP: EButtonTouchMethod = EButtonTouchMethod(0);
    pub const DOWN: EButtonTouchMethod = EButtonTouchMethod(1);
    pub const PRECISE_TAP: EButtonTouchMethod = EButtonTouchMethod(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EConsumeMouseWheel(pub u8);
impl EConsumeMouseWheel {
    pub const WHEN_SCROLLING_POSSIBLE: EConsumeMouseWheel = EConsumeMouseWheel(0);
    pub const ALWAYS: EConsumeMouseWheel = EConsumeMouseWheel(1);
    pub const NEVER: EConsumeMouseWheel = EConsumeMouseWheel(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUINavigationAction(pub u8);
impl EUINavigationAction {
    pub const ACCEPT: EUINavigationAction = EUINavigationAction(0);
    pub const BACK: EUINavigationAction = EUINavigationAction(1);
    pub const NUM: EUINavigationAction = EUINavigationAction(2);
    pub const INVALID: EUINavigationAction = EUINavigationAction(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFlowDirectionPreference(pub u8);
impl EFlowDirectionPreference {
    pub const INHERIT: EFlowDirectionPreference = EFlowDirectionPreference(0);
    pub const CULTURE: EFlowDirectionPreference = EFlowDirectionPreference(1);
    pub const LEFT_TO_RIGHT: EFlowDirectionPreference = EFlowDirectionPreference(2);
    pub const RIGHT_TO_LEFT: EFlowDirectionPreference = EFlowDirectionPreference(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetPixelSnapping(pub u8);
impl EWidgetPixelSnapping {
    pub const INHERIT: EWidgetPixelSnapping = EWidgetPixelSnapping(0);
    pub const DISABLED: EWidgetPixelSnapping = EWidgetPixelSnapping(1);
    pub const SNAP_TO_PIXEL: EWidgetPixelSnapping = EWidgetPixelSnapping(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESelectInfo(pub u8);
impl ESelectInfo {
    pub const ON_KEY_PRESS: ESelectInfo = ESelectInfo(0);
    pub const ON_NAVIGATION: ESelectInfo = ESelectInfo(1);
    pub const ON_MOUSE_CLICK: ESelectInfo = ESelectInfo(2);
    pub const DIRECT: ESelectInfo = ESelectInfo(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETextCommit(pub u8);
impl ETextCommit {
    pub const DEFAULT: ETextCommit = ETextCommit(0);
    pub const ON_ENTER: ETextCommit = ETextCommit(1);
    pub const ON_USER_MOVED_FOCUS: ETextCommit = ETextCommit(2);
    pub const ON_CLEARED: ETextCommit = ETextCommit(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetNavigationRoutingPolicy(pub u8);
impl EWidgetNavigationRoutingPolicy {
    pub const ACCEPT_FOCUS: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        0,
    );
    pub const ROUTE_TO_TOP_MOST_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        1,
    );
    pub const ROUTE_TO_BOTTOM_MOST_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        2,
    );
    pub const ROUTE_TO_LEFT_MOST_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        3,
    );
    pub const ROUTE_TO_RIGHT_MOST_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        4,
    );
    pub const ROUTE_TO_TOP_LEFT_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        5,
    );
    pub const ROUTE_TO_TOP_RIGHT_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        6,
    );
    pub const ROUTE_TO_BOTTOM_LEFT_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        7,
    );
    pub const ROUTE_TO_BOTTOM_RIGHT_CHILD: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        8,
    );
    pub const MAX: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(9);
    pub const DEFAULT: EWidgetNavigationRoutingPolicy = EWidgetNavigationRoutingPolicy(
        0,
    );
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFontLayoutMethod(pub u8);
impl EFontLayoutMethod {
    pub const METRICS: EFontLayoutMethod = EFontLayoutMethod(0);
    pub const BOUNDING_BOX: EFontLayoutMethod = EFontLayoutMethod(1);
}
