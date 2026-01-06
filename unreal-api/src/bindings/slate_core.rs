#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub fn initialize() {}
#[repr(C, align(4))]
pub struct FGeometry {
    __padding_end: [u8; 56],
}
impl FGeometry {}
#[repr(C, align(16))]
pub struct FSlateBrush {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub tint_color: FSlateColor,
    pub draw_as: ESlateBrushDrawType,
    pub tiling: ESlateBrushTileType,
    pub mirroring: ESlateBrushMirrorType,
    pub image_size: FDeprecateSlateVector2D,
    pub margin: FMargin,
    #[doc(hidden)]
    __padding_72: [u8; 16],
    pub resource_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub outline_settings: FSlateBrushOutlineSettings,
    __padding_end: [u8; 64],
}
impl FSlateBrush {}
#[repr(C, align(16))]
pub struct FSlateBrushOutlineSettings {
    pub corner_radii: crate::bindings::core_u_object::FVector4,
    pub color: FSlateColor,
    pub width: f32,
    pub rounding_type: ESlateBrushRoundingType,
    pub b_use_brush_transparency: bool,
    __padding_end: [u8; 6],
}
impl FSlateBrushOutlineSettings {}
#[repr(C, align(4))]
pub struct FSlateColor {
    pub specified_color: crate::bindings::core_u_object::FLinearColor,
    pub color_use_rule: ESlateColorStylingMode,
    __padding_end: [u8; 3],
}
impl FSlateColor {}
#[repr(C, align(4))]
pub struct FMargin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl FMargin {}
#[repr(C, align(4))]
pub struct FDeprecateSlateVector2D {
    __padding_end: [u8; 8],
}
impl FDeprecateSlateVector2D {}
#[repr(C, align(8))]
pub struct FInputEvent {
    __padding_end: [u8; 40],
}
impl FInputEvent {}
#[repr(C, align(8))]
pub struct FPointerEvent {
    __padding_end: [u8; 136],
}
impl FPointerEvent {}
#[repr(C, align(16))]
pub struct FScrollBarStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
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
    __padding_end: [u8; 12],
}
impl FScrollBarStyle {}
#[repr(C, align(16))]
pub struct FTableRowStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
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
impl FTableRowStyle {}
#[repr(C, align(16))]
pub struct FComboBoxStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub combo_button_style: FComboButtonStyle,
    pub pressed_slate_sound: FSlateSound,
    pub selection_change_slate_sound: FSlateSound,
    pub content_padding: FMargin,
    pub menu_row_padding: FMargin,
    __padding_end: [u8; 32],
}
impl FComboBoxStyle {}
#[repr(C, align(8))]
pub struct FSlateSound {
    pub resource_object: UPtr<crate::bindings::core_u_object::UObject>,
    __padding_end: [u8; 24],
}
impl FSlateSound {}
#[repr(C, align(16))]
pub struct FComboButtonStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub button_style: FButtonStyle,
    pub down_arrow_image: FSlateBrush,
    pub shadow_offset: FDeprecateSlateVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub menu_border_brush: FSlateBrush,
    pub menu_border_padding: FMargin,
    pub content_padding: FMargin,
    pub down_arrow_padding: FMargin,
    pub down_arrow_align: EVerticalAlignment,
    __padding_end: [u8; 15],
}
impl FComboButtonStyle {}
#[repr(C, align(16))]
pub struct FButtonStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
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
    __padding_end: [u8; 32],
}
impl FButtonStyle {}
#[repr(C, align(8))]
pub struct FSlateFontInfo {
    pub font_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub font_material: UPtr<crate::bindings::core_u_object::UObject>,
    pub outline_settings: FFontOutlineSettings,
    #[doc(hidden)]
    __padding_64: [u8; 16],
    pub typeface_font_name: FName,
    pub size: f32,
    pub letter_spacing: i32,
    pub skew_amount: f32,
    #[doc(hidden)]
    __padding_89: [u8; 1],
    pub b_force_monospaced: bool,
    pub b_material_is_stencil: bool,
    pub monospaced_width: f32,
    __padding_end: [u8; 16],
}
impl FSlateFontInfo {}
#[repr(C, align(8))]
pub struct FFontOutlineSettings {
    pub outline_size: i32,
    pub b_mitered_corners: bool,
    pub b_separate_fill_alpha: bool,
    pub b_apply_outline_to_drop_shadows: bool,
    pub outline_material: UPtr<crate::bindings::core_u_object::UObject>,
    pub outline_color: crate::bindings::core_u_object::FLinearColor,
}
impl FFontOutlineSettings {}
#[repr(C, align(16))]
pub struct FEditableTextStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub font: FSlateFontInfo,
    pub color_and_opacity: FSlateColor,
    pub background_image_selected: FSlateBrush,
    pub background_image_composing: FSlateBrush,
    pub caret_image: FSlateBrush,
}
impl FEditableTextStyle {}
#[repr(C, align(16))]
pub struct FEditableTextBoxStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub background_image_normal: FSlateBrush,
    pub background_image_hovered: FSlateBrush,
    pub background_image_focused: FSlateBrush,
    pub background_image_read_only: FSlateBrush,
    pub padding: FMargin,
    #[doc(hidden)]
    __padding_976: [u8; 112],
    pub text_style: FTextBlockStyle,
    pub foreground_color: FSlateColor,
    pub background_color: FSlateColor,
    pub read_only_foreground_color: FSlateColor,
    pub focused_foreground_color: FSlateColor,
    pub h_scroll_bar_padding: FMargin,
    pub v_scroll_bar_padding: FMargin,
    pub scroll_bar_style: FScrollBarStyle,
}
impl FEditableTextBoxStyle {}
#[repr(C, align(16))]
pub struct FTextBlockStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub font: FSlateFontInfo,
    pub color_and_opacity: FSlateColor,
    pub shadow_offset: FDeprecateSlateVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_184: [u8; 20],
    pub highlight_color: FSlateColor,
    pub highlight_shape: FSlateBrush,
    pub strike_brush: FSlateBrush,
    pub underline_brush: FSlateBrush,
    pub transform_policy: ETextTransformPolicy,
    pub overflow_policy: ETextOverflowPolicy,
    __padding_end: [u8; 14],
}
impl FTextBlockStyle {}
#[repr(C, align(16))]
pub struct FSpinBoxStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub background_brush: FSlateBrush,
    pub active_background_brush: FSlateBrush,
    pub hovered_background_brush: FSlateBrush,
    pub active_fill_brush: FSlateBrush,
    pub hovered_fill_brush: FSlateBrush,
    pub inactive_fill_brush: FSlateBrush,
    pub arrows_image: FSlateBrush,
    #[doc(hidden)]
    __padding_1492: [u8; 20],
    pub text_padding: FMargin,
    pub inset_padding: FMargin,
    __padding_end: [u8; 12],
}
impl FSpinBoxStyle {}
#[repr(C, align(8))]
pub struct FCharacterEvent {
    __padding_end: [u8; 48],
}
impl FCharacterEvent {}
#[repr(C, align(8))]
pub struct FKeyEvent {
    __padding_end: [u8; 80],
}
impl FKeyEvent {}
#[repr(C, align(8))]
pub struct FNavigationEvent {
    __padding_end: [u8; 48],
}
impl FNavigationEvent {}
#[repr(C, align(8))]
pub struct FAnalogInputEvent {
    __padding_end: [u8; 88],
}
impl FAnalogInputEvent {}
#[repr(C, align(4))]
pub struct FFontSdfSettings {
    __padding_end: [u8; 4],
}
impl FFontSdfSettings {}
#[repr(C, align(4))]
pub struct FFocusEvent {
    __padding_end: [u8; 8],
}
impl FFocusEvent {}
#[repr(C, align(4))]
pub struct FCaptureLostEvent {
    __padding_end: [u8; 8],
}
impl FCaptureLostEvent {}
#[repr(C, align(8))]
pub struct FMotionEvent {
    __padding_end: [u8; 136],
}
impl FMotionEvent {}
#[repr(C, align(16))]
pub struct FSegmentedControlStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub control_style: FCheckBoxStyle,
    pub first_control_style: FCheckBoxStyle,
    pub last_control_style: FCheckBoxStyle,
    pub background_brush: FSlateBrush,
    pub uniform_padding: FMargin,
}
impl FSegmentedControlStyle {}
#[repr(C, align(16))]
pub struct FCheckBoxStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
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
    __padding_end: [u8; 48],
}
impl FCheckBoxStyle {}
#[repr(C, align(16))]
pub struct FHyperlinkStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub underline_style: FButtonStyle,
    pub text_style: FTextBlockStyle,
    pub padding: FMargin,
}
impl FHyperlinkStyle {}
#[repr(C, align(16))]
pub struct FInlineEditableTextBlockStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub editable_text_box_style: FEditableTextBoxStyle,
    pub text_style: FTextBlockStyle,
}
impl FInlineEditableTextBlockStyle {}
#[repr(C, align(16))]
pub struct FProgressBarStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub background_image: FSlateBrush,
    pub fill_image: FSlateBrush,
    pub marquee_image: FSlateBrush,
    pub enable_fill_animation: bool,
    __padding_end: [u8; 15],
}
impl FProgressBarStyle {}
#[repr(C, align(16))]
pub struct FSliderStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub normal_bar_image: FSlateBrush,
    pub hovered_bar_image: FSlateBrush,
    pub disabled_bar_image: FSlateBrush,
    pub normal_thumb_image: FSlateBrush,
    pub hovered_thumb_image: FSlateBrush,
    pub disabled_thumb_image: FSlateBrush,
    pub bar_thickness: f32,
    __padding_end: [u8; 12],
}
impl FSliderStyle {}
#[repr(C, align(16))]
pub struct FSplitterStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub handle_normal_brush: FSlateBrush,
    pub handle_highlight_brush: FSlateBrush,
}
impl FSplitterStyle {}
#[repr(C, align(16))]
pub struct FTableViewStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub background_brush: FSlateBrush,
}
impl FTableViewStyle {}
#[repr(C, align(16))]
pub struct FScrollBoxStyle {
    #[doc(hidden)]
    __padding_8: [u8; 8],
    pub bar_thickness: f32,
    pub top_shadow_brush: FSlateBrush,
    pub bottom_shadow_brush: FSlateBrush,
    pub left_shadow_brush: FSlateBrush,
    pub right_shadow_brush: FSlateBrush,
    pub horizontal_scrolled_content_padding: FMargin,
    pub vertical_scrolled_content_padding: FMargin,
}
impl FScrollBoxStyle {}
#[repr(C, align(16))]
pub struct FScrollBorderStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub top_shadow_brush: FSlateBrush,
    pub bottom_shadow_brush: FSlateBrush,
}
impl FScrollBorderStyle {}
#[repr(C, align(16))]
pub struct FWindowStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
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
    __padding_end: [u8; 12],
}
impl FWindowStyle {}
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
    __padding_end: [u8; 4],
}
impl FWrapButtonStyle {}
#[repr(C, align(16))]
pub struct FToolBarStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub background_brush: FSlateBrush,
    #[doc(hidden)]
    __padding_432: [u8; 208],
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
    #[doc(hidden)]
    __padding_18948: [u8; 20],
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
    __padding_end: [u8; 8],
}
impl FToolBarStyle {}
#[repr(C, align(8))]
pub struct USlateWidgetStyleAsset {
    __padding_end: [u8; 56],
}
impl USlateWidgetStyleAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateWidgetStyleAsset")
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
pub struct UFontBulkData {
    __padding_end: [u8; 168],
}
impl UFontBulkData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontBulkData")
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
pub struct IFontFaceInterface {}
#[repr(C, align(8))]
pub struct UFontFaceInterface {
    __padding_end: [u8; 48],
}
impl UFontFaceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontFaceInterface")
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
pub struct IFontProviderInterface {}
#[repr(C, align(8))]
pub struct UFontProviderInterface {
    __padding_end: [u8; 48],
}
impl UFontProviderInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontProviderInterface")
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
pub struct USlateTypes {
    __padding_end: [u8; 48],
}
impl USlateTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateTypes")
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
pub struct USlateWidgetStyleContainerBase {
    __padding_end: [u8; 56],
}
impl USlateWidgetStyleContainerBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateWidgetStyleContainerBase")
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
pub struct ISlateWidgetStyleContainerInterface {}
#[repr(C, align(8))]
pub struct USlateWidgetStyleContainerInterface {
    __padding_end: [u8; 48],
}
impl USlateWidgetStyleContainerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateWidgetStyleContainerInterface")
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
pub struct USlateThemeManager {
    __padding_end: [u8; 3104],
}
impl USlateThemeManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateThemeManager")
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
#[repr(transparent)]
pub struct ESlateBrushDrawType(pub u8);
impl ESlateBrushDrawType {
    pub const NO_DRAW_TYPE: ESlateBrushDrawType = ESlateBrushDrawType(0);
    pub const BOX: ESlateBrushDrawType = ESlateBrushDrawType(1);
    pub const BORDER: ESlateBrushDrawType = ESlateBrushDrawType(2);
    pub const IMAGE: ESlateBrushDrawType = ESlateBrushDrawType(3);
    pub const ROUNDED_BOX: ESlateBrushDrawType = ESlateBrushDrawType(4);
}
#[repr(transparent)]
pub struct ESlateBrushTileType(pub u8);
impl ESlateBrushTileType {
    pub const NO_TILE: ESlateBrushTileType = ESlateBrushTileType(0);
    pub const HORIZONTAL: ESlateBrushTileType = ESlateBrushTileType(1);
    pub const VERTICAL: ESlateBrushTileType = ESlateBrushTileType(2);
    pub const BOTH: ESlateBrushTileType = ESlateBrushTileType(3);
}
#[repr(transparent)]
pub struct ESlateBrushMirrorType(pub u8);
impl ESlateBrushMirrorType {
    pub const NO_MIRROR: ESlateBrushMirrorType = ESlateBrushMirrorType(0);
    pub const HORIZONTAL: ESlateBrushMirrorType = ESlateBrushMirrorType(1);
    pub const VERTICAL: ESlateBrushMirrorType = ESlateBrushMirrorType(2);
    pub const BOTH: ESlateBrushMirrorType = ESlateBrushMirrorType(3);
}
#[repr(transparent)]
pub struct ESlateBrushImageType(pub u8);
impl ESlateBrushImageType {
    pub const NO_IMAGE: ESlateBrushImageType = ESlateBrushImageType(0);
    pub const FULL_COLOR: ESlateBrushImageType = ESlateBrushImageType(1);
    pub const LINEAR: ESlateBrushImageType = ESlateBrushImageType(2);
    pub const VECTOR: ESlateBrushImageType = ESlateBrushImageType(3);
}
#[repr(transparent)]
pub struct ESlateBrushRoundingType(pub u8);
impl ESlateBrushRoundingType {
    pub const FIXED_RADIUS: ESlateBrushRoundingType = ESlateBrushRoundingType(0);
    pub const HALF_HEIGHT_RADIUS: ESlateBrushRoundingType = ESlateBrushRoundingType(1);
}
#[repr(transparent)]
pub struct EVerticalAlignment(pub u8);
impl EVerticalAlignment {
    pub const V_ALIGN_FILL: EVerticalAlignment = EVerticalAlignment(0);
    pub const V_ALIGN_TOP: EVerticalAlignment = EVerticalAlignment(1);
    pub const V_ALIGN_CENTER: EVerticalAlignment = EVerticalAlignment(2);
    pub const V_ALIGN_BOTTOM: EVerticalAlignment = EVerticalAlignment(3);
}
#[repr(transparent)]
pub struct EFontHinting(pub u8);
impl EFontHinting {
    pub const DEFAULT: EFontHinting = EFontHinting(0);
    pub const AUTO: EFontHinting = EFontHinting(1);
    pub const AUTO_LIGHT: EFontHinting = EFontHinting(2);
    pub const MONOCHROME: EFontHinting = EFontHinting(3);
    pub const NONE: EFontHinting = EFontHinting(4);
}
#[repr(transparent)]
pub struct ETextShapingMethod(pub u8);
impl ETextShapingMethod {
    pub const AUTO: ETextShapingMethod = ETextShapingMethod(0);
    pub const KERNING_ONLY: ETextShapingMethod = ETextShapingMethod(1);
    pub const FULL_SHAPING: ETextShapingMethod = ETextShapingMethod(2);
}
#[repr(transparent)]
pub struct ETextTransformPolicy(pub u8);
impl ETextTransformPolicy {
    pub const NONE: ETextTransformPolicy = ETextTransformPolicy(0);
    pub const TO_LOWER: ETextTransformPolicy = ETextTransformPolicy(1);
    pub const TO_UPPER: ETextTransformPolicy = ETextTransformPolicy(2);
}
#[repr(transparent)]
pub struct ETextOverflowPolicy(pub u8);
impl ETextOverflowPolicy {
    pub const CLIP: ETextOverflowPolicy = ETextOverflowPolicy(0);
    pub const ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(1);
    pub const MULTILINE_ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(2);
    pub const MIDDLE_ELLIPSIS: ETextOverflowPolicy = ETextOverflowPolicy(3);
}
#[repr(transparent)]
pub struct EFontLoadingPolicy(pub u8);
impl EFontLoadingPolicy {
    pub const LAZY_LOAD: EFontLoadingPolicy = EFontLoadingPolicy(0);
    pub const STREAM: EFontLoadingPolicy = EFontLoadingPolicy(1);
    pub const INLINE: EFontLoadingPolicy = EFontLoadingPolicy(2);
}
#[repr(transparent)]
pub struct ESlateCheckBoxType(pub u8);
impl ESlateCheckBoxType {
    pub const CHECK_BOX: ESlateCheckBoxType = ESlateCheckBoxType(0);
    pub const TOGGLE_BUTTON: ESlateCheckBoxType = ESlateCheckBoxType(1);
}
#[repr(transparent)]
pub struct EHorizontalAlignment(pub u8);
impl EHorizontalAlignment {
    pub const H_ALIGN_FILL: EHorizontalAlignment = EHorizontalAlignment(0);
    pub const H_ALIGN_LEFT: EHorizontalAlignment = EHorizontalAlignment(1);
    pub const H_ALIGN_CENTER: EHorizontalAlignment = EHorizontalAlignment(2);
    pub const H_ALIGN_RIGHT: EHorizontalAlignment = EHorizontalAlignment(3);
}
#[repr(transparent)]
pub struct ECheckBoxState(pub u8);
impl ECheckBoxState {
    pub const UNCHECKED: ECheckBoxState = ECheckBoxState(0);
    pub const CHECKED: ECheckBoxState = ECheckBoxState(1);
    pub const UNDETERMINED: ECheckBoxState = ECheckBoxState(2);
}
#[repr(transparent)]
pub struct ENavigationGenesis(pub u8);
impl ENavigationGenesis {
    pub const KEYBOARD: ENavigationGenesis = ENavigationGenesis(0);
    pub const CONTROLLER: ENavigationGenesis = ENavigationGenesis(1);
    pub const USER: ENavigationGenesis = ENavigationGenesis(2);
}
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
#[repr(transparent)]
pub struct EFontRasterizationMode(pub u8);
impl EFontRasterizationMode {
    pub const BITMAP: EFontRasterizationMode = EFontRasterizationMode(0);
    pub const MSDF: EFontRasterizationMode = EFontRasterizationMode(1);
    pub const SDF: EFontRasterizationMode = EFontRasterizationMode(2);
    pub const SDF_APPROXIMATION: EFontRasterizationMode = EFontRasterizationMode(3);
}
#[repr(transparent)]
pub struct EOrientation(pub u8);
impl EOrientation {
    pub const ORIENT_HORIZONTAL: EOrientation = EOrientation(0);
    pub const ORIENT_VERTICAL: EOrientation = EOrientation(1);
}
#[repr(transparent)]
pub struct EWidgetClipping(pub u8);
impl EWidgetClipping {
    pub const INHERIT: EWidgetClipping = EWidgetClipping(0);
    pub const CLIP_TO_BOUNDS: EWidgetClipping = EWidgetClipping(1);
    pub const CLIP_TO_BOUNDS_WITHOUT_INTERSECTING: EWidgetClipping = EWidgetClipping(2);
    pub const CLIP_TO_BOUNDS_ALWAYS: EWidgetClipping = EWidgetClipping(3);
    pub const ON_DEMAND: EWidgetClipping = EWidgetClipping(4);
}
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
#[repr(transparent)]
pub struct EColorVisionDeficiency(pub u8);
impl EColorVisionDeficiency {
    pub const NORMAL_VISION: EColorVisionDeficiency = EColorVisionDeficiency(0);
    pub const DEUTERANOPE: EColorVisionDeficiency = EColorVisionDeficiency(1);
    pub const PROTANOPE: EColorVisionDeficiency = EColorVisionDeficiency(2);
    pub const TRITANOPE: EColorVisionDeficiency = EColorVisionDeficiency(3);
}
#[repr(transparent)]
pub struct EButtonClickMethod(pub u8);
impl EButtonClickMethod {
    pub const DOWN_AND_UP: EButtonClickMethod = EButtonClickMethod(0);
    pub const MOUSE_DOWN: EButtonClickMethod = EButtonClickMethod(1);
    pub const MOUSE_UP: EButtonClickMethod = EButtonClickMethod(2);
    pub const PRECISE_CLICK: EButtonClickMethod = EButtonClickMethod(3);
}
#[repr(transparent)]
pub struct EButtonPressMethod(pub u8);
impl EButtonPressMethod {
    pub const DOWN_AND_UP: EButtonPressMethod = EButtonPressMethod(0);
    pub const BUTTON_PRESS: EButtonPressMethod = EButtonPressMethod(1);
    pub const BUTTON_RELEASE: EButtonPressMethod = EButtonPressMethod(2);
}
#[repr(transparent)]
pub struct EButtonTouchMethod(pub u8);
impl EButtonTouchMethod {
    pub const DOWN_AND_UP: EButtonTouchMethod = EButtonTouchMethod(0);
    pub const DOWN: EButtonTouchMethod = EButtonTouchMethod(1);
    pub const PRECISE_TAP: EButtonTouchMethod = EButtonTouchMethod(2);
}
#[repr(transparent)]
pub struct EConsumeMouseWheel(pub u8);
impl EConsumeMouseWheel {
    pub const WHEN_SCROLLING_POSSIBLE: EConsumeMouseWheel = EConsumeMouseWheel(0);
    pub const ALWAYS: EConsumeMouseWheel = EConsumeMouseWheel(1);
    pub const NEVER: EConsumeMouseWheel = EConsumeMouseWheel(2);
}
#[repr(transparent)]
pub struct EUINavigationAction(pub u8);
impl EUINavigationAction {
    pub const ACCEPT: EUINavigationAction = EUINavigationAction(0);
    pub const BACK: EUINavigationAction = EUINavigationAction(1);
    pub const NUM: EUINavigationAction = EUINavigationAction(2);
    pub const INVALID: EUINavigationAction = EUINavigationAction(3);
}
#[repr(transparent)]
pub struct EFlowDirectionPreference(pub u8);
impl EFlowDirectionPreference {
    pub const INHERIT: EFlowDirectionPreference = EFlowDirectionPreference(0);
    pub const CULTURE: EFlowDirectionPreference = EFlowDirectionPreference(1);
    pub const LEFT_TO_RIGHT: EFlowDirectionPreference = EFlowDirectionPreference(2);
    pub const RIGHT_TO_LEFT: EFlowDirectionPreference = EFlowDirectionPreference(3);
}
#[repr(transparent)]
pub struct EWidgetPixelSnapping(pub u8);
impl EWidgetPixelSnapping {
    pub const INHERIT: EWidgetPixelSnapping = EWidgetPixelSnapping(0);
    pub const DISABLED: EWidgetPixelSnapping = EWidgetPixelSnapping(1);
    pub const SNAP_TO_PIXEL: EWidgetPixelSnapping = EWidgetPixelSnapping(2);
}
#[repr(transparent)]
pub struct ESelectInfo(pub u8);
impl ESelectInfo {
    pub const ON_KEY_PRESS: ESelectInfo = ESelectInfo(0);
    pub const ON_NAVIGATION: ESelectInfo = ESelectInfo(1);
    pub const ON_MOUSE_CLICK: ESelectInfo = ESelectInfo(2);
    pub const DIRECT: ESelectInfo = ESelectInfo(3);
}
#[repr(transparent)]
pub struct ETextCommit(pub u8);
impl ETextCommit {
    pub const DEFAULT: ETextCommit = ETextCommit(0);
    pub const ON_ENTER: ETextCommit = ETextCommit(1);
    pub const ON_USER_MOVED_FOCUS: ETextCommit = ETextCommit(2);
    pub const ON_CLEARED: ETextCommit = ETextCommit(3);
}
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
#[repr(transparent)]
pub struct EFontLayoutMethod(pub u8);
impl EFontLayoutMethod {
    pub const METRICS: EFontLayoutMethod = EFontLayoutMethod(0);
    pub const BOUNDING_BOX: EFontLayoutMethod = EFontLayoutMethod(1);
}
