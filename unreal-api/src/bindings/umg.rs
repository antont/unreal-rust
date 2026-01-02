#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEventReply {
    __padding_end: [u8; 192],
}
impl FEventReply {}
#[repr(C, align(8))]
pub struct FWidgetTransform {
    pub translation: crate::bindings::core_u_object::FVector2D,
    pub scale: crate::bindings::core_u_object::FVector2D,
    pub shear: crate::bindings::core_u_object::FVector2D,
    pub angle: f32,
    __padding_end: [u8; 4],
}
impl FWidgetTransform {}
#[repr(C, align(1))]
pub struct FShapedTextOptions {
    #[doc(hidden)]
    __padding_1: [u8; 1],
    pub text_shaping_method: crate::bindings::slate_core::ETextShapingMethod,
    pub text_flow_direction: crate::bindings::slate::ETextFlowDirection,
}
impl FShapedTextOptions {}
#[repr(C, align(8))]
pub struct FPaintContext {
    __padding_end: [u8; 48],
}
impl FPaintContext {}
#[repr(C, align(4))]
pub struct FRadialBoxSettings {
    __padding_end: [u8; 16],
}
impl FRadialBoxSettings {}
#[repr(C, align(4))]
pub struct FSlateChildSize {
    pub value: f32,
    pub size_rule: ESlateSizeRule,
    __padding_end: [u8; 3],
}
impl FSlateChildSize {}
#[repr(C, align(1))]
pub struct FWidgetEventField {
    __padding_end: [u8; 1],
}
impl FWidgetEventField {}
#[repr(C, align(8))]
pub struct FWidgetNavigationData {
    pub rule: crate::bindings::slate_core::EUINavigationRule,
    pub widget_to_focus: FName,
    __padding_end: [u8; 40],
}
impl FWidgetNavigationData {}
#[repr(C, align(8))]
pub struct FWidgetAnimationHandle {
    __padding_end: [u8; 16],
}
impl FWidgetAnimationHandle {}
#[repr(C, align(8))]
pub struct FGameViewportWidgetSlot {
    pub anchors: crate::bindings::slate::FAnchors,
    pub offsets: crate::bindings::slate_core::FMargin,
    pub alignment: crate::bindings::core_u_object::FVector2D,
    pub z_order: i32,
    pub b_auto_remove_on_world_removed: bool,
    __padding_end: [u8; 3],
}
impl FGameViewportWidgetSlot {}
#[repr(C, align(8))]
pub struct FAnchorData {
    pub offsets: crate::bindings::slate_core::FMargin,
    pub anchors: crate::bindings::slate::FAnchors,
    pub alignment: crate::bindings::core_u_object::FVector2D,
}
impl FAnchorData {}
#[repr(C, align(16))]
pub struct FRichTextStyleRow {
    __padding_end: [u8; 864],
}
impl FRichTextStyleRow {}
#[repr(C, align(16))]
pub struct FRichImageRow {
    __padding_end: [u8; 224],
}
impl FRichImageRow {}
#[repr(C, align(8))]
pub struct UVisual {
    __padding_end: [u8; 48],
}
impl UVisual {}
#[repr(C, align(8))]
pub struct UWidget {
    #[doc(hidden)]
    __padding_56: [u8; 56],
    pub slot: UPtr<UPanelSlot>,
    #[doc(hidden)]
    __padding_128: [u8; 64],
    pub tool_tip_text: FText,
    pub tool_tip_widget: UPtr<UWidget>,
    #[doc(hidden)]
    __padding_216: [u8; 64],
    pub render_transform: FWidgetTransform,
    pub render_transform_pivot: crate::bindings::core_u_object::FVector2D,
    pub flow_direction_preference: crate::bindings::slate_core::EFlowDirectionPreference,
    pub flags_289: u8,
    #[doc(hidden)]
    __padding_392: [u8; 102],
    pub flags_392: u8,
    pub cursor: crate::bindings::core_u_object::EMouseCursor,
    pub clipping: crate::bindings::slate_core::EWidgetClipping,
    pub visibility: ESlateVisibility,
    pub pixel_snapping: crate::bindings::slate_core::EWidgetPixelSnapping,
    pub render_opacity: f32,
    #[doc(hidden)]
    __padding_416: [u8; 8],
    pub navigation: UPtr<UWidgetNavigation>,
    __padding_end: [u8; 272],
}
impl UWidget {
    pub fn verify_layout() {
        log::warn!(
            "{} = {} vs {}", "slot", std::mem::offset_of!(UWidget, slot), 56usize
        );
        log::warn!(
            "{} = {} vs {}", "tool_tip_text", std::mem::offset_of!(UWidget,
            tool_tip_text), 128usize
        );
        log::warn!(
            "{} = {} vs {}", "tool_tip_widget", std::mem::offset_of!(UWidget,
            tool_tip_widget), 144usize
        );
        log::warn!(
            "{} = {} vs {}", "render_transform", std::mem::offset_of!(UWidget,
            render_transform), 216usize
        );
        log::warn!(
            "{} = {} vs {}", "render_transform_pivot", std::mem::offset_of!(UWidget,
            render_transform_pivot), 272usize
        );
        log::warn!(
            "{} = {} vs {}", "flow_direction_preference", std::mem::offset_of!(UWidget,
            flow_direction_preference), 288usize
        );
        log::warn!(
            "{} = {} vs {}", "cursor", std::mem::offset_of!(UWidget, cursor), 393usize
        );
        log::warn!(
            "{} = {} vs {}", "clipping", std::mem::offset_of!(UWidget, clipping),
            394usize
        );
        log::warn!(
            "{} = {} vs {}", "visibility", std::mem::offset_of!(UWidget, visibility),
            395usize
        );
        log::warn!(
            "{} = {} vs {}", "pixel_snapping", std::mem::offset_of!(UWidget,
            pixel_snapping), 396usize
        );
        log::warn!(
            "{} = {} vs {}", "render_opacity", std::mem::offset_of!(UWidget,
            render_opacity), 400usize
        );
        log::warn!(
            "{} = {} vs {}", "navigation", std::mem::offset_of!(UWidget, navigation),
            416usize
        );
    }
}
#[repr(C, align(8))]
pub struct UUserWidget {
    #[doc(hidden)]
    __padding_768: [u8; 768],
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_816: [u8; 32],
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_960: [u8; 124],
    pub padding: crate::bindings::slate_core::FMargin,
    pub priority: i32,
    pub flags_980: u8,
    #[doc(hidden)]
    __padding_1176: [u8; 195],
    pub tick_frequency: EWidgetTickFrequency,
    __padding_end: [u8; 111],
}
impl UUserWidget {}
#[repr(C, align(16))]
pub struct UWidgetComponent {
    __padding_end: [u8; 2016],
}
impl UWidgetComponent {}
#[repr(C, align(8))]
pub struct UUserWidgetBlueprint {
    __padding_end: [u8; 1432],
}
impl UUserWidgetBlueprint {}
#[repr(C, align(8))]
pub struct UPanelWidget {
    __padding_end: [u8; 720],
}
impl UPanelWidget {}
#[repr(C, align(8))]
pub struct UContentWidget {
    __padding_end: [u8; 720],
}
impl UContentWidget {}
#[repr(C, align(16))]
pub struct UButton {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    #[doc(hidden)]
    __padding_2152: [u8; 308],
    pub b_allow_drag_drop: bool,
    __padding_end: [u8; 7],
}
impl UButton {}
#[repr(C, align(16))]
pub struct UCheckBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub checked_state: crate::bindings::slate_core::ECheckBoxState,
    #[doc(hidden)]
    __padding_768: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FCheckBoxStyle,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    __padding_end: [u8; 59],
}
impl UCheckBox {}
#[repr(C, align(16))]
pub struct UCircularThrobber {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub number_of_pieces: i32,
    pub period: f32,
    pub radius: f32,
    pub image: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 32],
}
impl UCircularThrobber {}
#[repr(C, align(16))]
pub struct UComboBoxKey {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub widget_style: crate::bindings::slate_core::FComboBoxStyle,
    pub item_style: crate::bindings::slate_core::FTableRowStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub content_padding: crate::bindings::slate_core::FMargin,
    pub max_list_height: f32,
    pub b_has_down_arrow: bool,
    pub b_enable_gamepad_navigation_mode: bool,
    pub b_is_focusable: bool,
    __padding_end: [u8; 149],
}
impl UComboBoxKey {}
#[repr(C, align(16))]
pub struct UComboBoxString {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub widget_style: crate::bindings::slate_core::FComboBoxStyle,
    pub item_style: crate::bindings::slate_core::FTableRowStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub content_padding: crate::bindings::slate_core::FMargin,
    pub max_list_height: f32,
    pub has_down_arrow: bool,
    pub enable_gamepad_navigation_mode: bool,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub b_is_focusable: bool,
    __padding_end: [u8; 163],
}
impl UComboBoxString {}
#[repr(C, align(16))]
pub struct UEditableText {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub text: FText,
    #[doc(hidden)]
    __padding_744: [u8; 32],
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextStyle,
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    #[doc(hidden)]
    __padding_1586: [u8; 5],
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    #[doc(hidden)]
    __padding_1688: [u8; 97],
    pub enable_integrated_keyboard: bool,
    __padding_end: [u8; 7],
}
impl UEditableText {}
#[repr(C, align(16))]
pub struct UEditableTextBox {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub text: FText,
    #[doc(hidden)]
    __padding_752: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_4640: [u8; 32],
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    #[doc(hidden)]
    __padding_4658: [u8; 5],
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    __padding_end: [u8; 105],
}
impl UEditableTextBox {}
#[repr(C, align(16))]
pub struct UExpandableArea {
    #[doc(hidden)]
    __padding_1152: [u8; 1152],
    pub border_brush: crate::bindings::slate_core::FSlateBrush,
    pub border_color: crate::bindings::slate_core::FSlateColor,
    pub b_is_expanded: bool,
    pub max_height: f32,
    pub header_padding: crate::bindings::slate_core::FMargin,
    pub area_padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 68],
}
impl UExpandableArea {}
#[repr(C, align(16))]
pub struct UInputKeySelector {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub text_style: crate::bindings::slate_core::FTextBlockStyle,
    pub selected_key: crate::bindings::slate::FInputChord,
    pub margin: crate::bindings::slate_core::FMargin,
    pub key_selection_text: FText,
    pub no_key_specified_text: FText,
    pub b_allow_modifier_keys: bool,
    pub b_allow_gamepad_keys: bool,
    pub escape_keys: TArray<crate::bindings::input_core::FKey>,
    __padding_end: [u8; 64],
}
impl UInputKeySelector {}
#[repr(C, align(8))]
pub struct UListViewBase {
    #[doc(hidden)]
    __padding_760: [u8; 760],
    pub entry_widget_class: TSubclassOf<UUserWidget>,
    pub wheel_scroll_multiplier: f32,
    pub b_enable_scroll_animation: bool,
    pub scrolling_animation_interpolation_speed: f32,
    pub b_in_enable_touch_animated_scrolling: bool,
    #[doc(hidden)]
    __padding_792: [u8; 11],
    pub b_allow_dragging: bool,
    pub b_allow_drag_drop: bool,
    pub drag_drop_visual_pivot: EDragPivot,
    pub drag_drop_visual_offset: crate::bindings::core_u_object::FVector2D,
    pub drag_drop_visual_entry_class: TSubclassOf<UUserWidget>,
    pub drag_drop_operation_class: TSubclassOf<UDragDropOperation>,
    pub drag_visual_widget: UPtr<UWidget>,
    pub b_is_dragging: bool,
    pub b_select_item_on_navigation: bool,
    pub b_allow_keep_preselected_items: bool,
    __padding_end: [u8; 269],
}
impl UListViewBase {}
#[repr(C, align(16))]
pub struct UListView {
    #[doc(hidden)]
    __padding_1616: [u8; 1616],
    pub widget_style: crate::bindings::slate_core::FTableViewStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub b_enable_shadow_brush: bool,
    pub shadow_brush_style: crate::bindings::slate_core::FScrollBoxStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub selection_mode: crate::bindings::slate::ESelectionMode,
    pub consume_mouse_wheel: crate::bindings::slate_core::EConsumeMouseWheel,
    pub b_clear_selection_on_click: bool,
    pub b_is_focusable: bool,
    pub b_clear_scroll_velocity_on_selection: bool,
    pub b_return_focus_to_selection: bool,
    pub scroll_into_view_alignment: crate::bindings::slate::EScrollIntoViewAlignment,
    #[doc(hidden)]
    __padding_4680: [u8; 32],
    pub entry_spacing: f32,
    pub horizontal_entry_spacing: f32,
    pub vertical_entry_spacing: f32,
    pub scroll_bar_padding: crate::bindings::slate_core::FMargin,
    #[doc(hidden)]
    __padding_5048: [u8; 336],
    pub bp_on_is_item_selectable_or_navigable: FListView_BP_OnIsItemSelectableOrNavigable,
    __padding_end: [u8; 8],
}
impl UListView {}
#[repr(C, align(8))]
pub struct UTextLayoutWidget {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub shaped_text_options: FShapedTextOptions,
    pub justification: crate::bindings::slate::ETextJustify,
    pub wrapping_policy: crate::bindings::slate::ETextWrappingPolicy,
    pub flags_701: u8,
    pub apply_line_height_to_bottom_line: bool,
    pub wrap_text_at: f32,
    pub margin: crate::bindings::slate_core::FMargin,
    pub line_height_percentage: f32,
}
impl UTextLayoutWidget {}
#[repr(C, align(16))]
pub struct UMultiLineEditableText {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FTextBlockStyle,
    pub b_is_read_only: bool,
    pub select_all_text_when_focused: bool,
    #[doc(hidden)]
    __padding_1651: [u8; 1],
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    __padding_end: [u8; 91],
}
impl UMultiLineEditableText {}
#[repr(C, align(16))]
pub struct UMultiLineEditableTextBox {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub hint_text: FText,
    #[doc(hidden)]
    __padding_800: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
    #[doc(hidden)]
    __padding_5488: [u8; 848],
    pub b_is_read_only: bool,
    __padding_end: [u8; 95],
}
impl UMultiLineEditableTextBox {}
#[repr(C, align(16))]
pub struct UProgressBar {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FProgressBarStyle,
    pub percent: f32,
    pub bar_fill_type: crate::bindings::slate::EProgressBarFillType,
    pub bar_fill_style: crate::bindings::slate::EProgressBarFillStyle,
    pub b_is_marquee: bool,
    pub border_padding: crate::bindings::core_u_object::FVector2D,
    #[doc(hidden)]
    __padding_1416: [u8; 32],
    pub fill_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 72],
}
impl UProgressBar {}
#[repr(C, align(16))]
pub struct UScrollBar {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub widget_style: crate::bindings::slate_core::FScrollBarStyle,
    pub b_always_show_scrollbar: bool,
    pub b_always_show_scrollbar_track: bool,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub thickness: crate::bindings::core_u_object::FVector2D,
    pub padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 24],
}
impl UScrollBar {}
#[repr(C, align(16))]
pub struct UScrollBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub scroll_animation_interpolation_speed: f32,
    pub b_enable_touch_scrolling: bool,
    pub b_consume_pointer_input: bool,
    pub analog_mouse_wheel_key: crate::bindings::input_core::FKey,
    pub b_is_focusable: bool,
    pub widget_style: crate::bindings::slate_core::FScrollBoxStyle,
    pub widget_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub scroll_bar_visibility: ESlateVisibility,
    pub consume_mouse_wheel: crate::bindings::slate_core::EConsumeMouseWheel,
    pub scrollbar_thickness: crate::bindings::core_u_object::FVector2D,
    pub scrollbar_padding: crate::bindings::slate_core::FMargin,
    pub always_show_scrollbar: bool,
    pub always_show_scrollbar_track: bool,
    pub allow_overscroll: bool,
    pub back_pad_scrolling: bool,
    pub front_pad_scrolling: bool,
    pub b_animate_wheel_scrolling: bool,
    pub navigation_destination: crate::bindings::slate::EDescendantScrollDestination,
    pub navigation_scroll_padding: f32,
    pub scroll_when_focus_changes: crate::bindings::slate::EScrollWhenFocusChanges,
    pub b_allow_right_click_drag_scrolling: bool,
    pub wheel_scroll_multiplier: f32,
    __padding_end: [u8; 148],
}
impl UScrollBox {}
#[repr(C, align(16))]
pub struct USlider {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_736: [u8; 36],
    pub min_value: f32,
    pub max_value: f32,
    pub widget_style: crate::bindings::slate_core::FSliderStyle,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub slider_bar_color: crate::bindings::core_u_object::FLinearColor,
    pub slider_handle_color: crate::bindings::core_u_object::FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub mouse_uses_step: bool,
    pub requires_controller_lock: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub b_prevent_throttling: bool,
    __padding_end: [u8; 146],
}
impl USlider {}
#[repr(C, align(16))]
pub struct USpinBox {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub value: f32,
    #[doc(hidden)]
    __padding_736: [u8; 32],
    pub widget_style: crate::bindings::slate_core::FSpinBoxStyle,
    pub min_fractional_digits: i32,
    pub max_fractional_digits: i32,
    pub b_always_uses_delta_snap: bool,
    pub b_enable_slider: bool,
    pub delta: f32,
    pub slider_exponent: f32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub justification: crate::bindings::slate::ETextJustify,
    pub min_desired_width: f32,
    #[doc(hidden)]
    __padding_2417: [u8; 1],
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_2540: [u8; 100],
    pub min_value: f32,
    pub max_value: f32,
    pub min_slider_value: f32,
    pub max_slider_value: f32,
    __padding_end: [u8; 36],
}
impl USpinBox {}
#[repr(C, align(16))]
pub struct UThrobber {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub number_of_pieces: i32,
    pub b_animate_horizontally: bool,
    pub b_animate_vertically: bool,
    pub b_animate_opacity: bool,
    pub image: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 16],
}
impl UThrobber {}
#[repr(C, align(16))]
pub struct UTreeView {
    __padding_end: [u8; 5216],
}
impl UTreeView {}
#[repr(C, align(8))]
pub struct UListViewDesignerPreviewItem {
    __padding_end: [u8; 48],
}
impl UListViewDesignerPreviewItem {}
#[repr(C, align(8))]
pub struct USlateAccessibleWidgetData {
    __padding_end: [u8; 152],
}
impl USlateAccessibleWidgetData {}
#[repr(C, align(8))]
pub struct UUserWidgetExtension {
    __padding_end: [u8; 48],
}
impl UUserWidgetExtension {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintGeneratedClassExtension {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintGeneratedClassExtension {}
#[repr(C, align(8))]
pub struct UWidgetFieldNotificationExtension {
    __padding_end: [u8; 72],
}
impl UWidgetFieldNotificationExtension {}
#[repr(C, align(8))]
pub struct UWidgetNavigation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub up: FWidgetNavigationData,
    pub down: FWidgetNavigationData,
    pub left: FWidgetNavigationData,
    pub right: FWidgetNavigationData,
    pub next: FWidgetNavigationData,
    pub previous: FWidgetNavigationData,
    pub routing_policy: crate::bindings::slate_core::EWidgetNavigationRoutingPolicy,
    pub navigation_method: crate::bindings::core_u_object::FInstancedStruct,
}
impl UWidgetNavigation {}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieScene2DTransformPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformSection {
    __padding_end: [u8; 2504],
}
impl UMovieScene2DTransformSection {}
#[repr(C, align(8))]
pub struct UMovieScene2DTransformTrack {
    __padding_end: [u8; 480],
}
impl UMovieScene2DTransformTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneMarginPropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneMarginPropertySystem {}
#[repr(C, align(8))]
pub struct UMovieSceneMarginSection {
    __padding_end: [u8; 1584],
}
impl UMovieSceneMarginSection {}
#[repr(C, align(8))]
pub struct UMovieSceneMarginTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneMarginTrack {}
#[repr(C, align(8))]
pub struct UMovieSceneWidgetMaterialSystem {
    __padding_end: [u8; 536],
}
impl UMovieSceneWidgetMaterialSystem {}
#[repr(C, align(8))]
pub struct UMovieSceneWidgetMaterialTrack {
    __padding_end: [u8; 456],
}
impl UMovieSceneWidgetMaterialTrack {}
#[repr(C, align(8))]
pub struct UUMGSequencePlayer {
    __padding_end: [u8; 624],
}
impl UUMGSequencePlayer {}
#[repr(C, align(8))]
pub struct UUMGSequenceTickManager {
    __padding_end: [u8; 216],
}
impl UUMGSequenceTickManager {}
#[repr(C, align(8))]
pub struct UWidgetAnimation {
    __padding_end: [u8; 176],
}
impl UWidgetAnimation {}
#[repr(C, align(8))]
pub struct UWidgetAnimationDelegateBinding {
    __padding_end: [u8; 64],
}
impl UWidgetAnimationDelegateBinding {}
#[repr(C, align(8))]
pub struct UWidgetAnimationHandleFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetAnimationHandleFunctionLibrary {}
#[repr(C, align(8))]
pub struct UWidgetAnimationPlayCallbackProxy {
    __padding_end: [u8; 96],
}
impl UWidgetAnimationPlayCallbackProxy {}
#[repr(C, align(8))]
pub struct UPropertyBinding {
    __padding_end: [u8; 128],
}
impl UPropertyBinding {}
#[repr(C, align(8))]
pub struct UBoolBinding {
    __padding_end: [u8; 128],
}
impl UBoolBinding {}
#[repr(C, align(8))]
pub struct UBrushBinding {
    __padding_end: [u8; 136],
}
impl UBrushBinding {}
#[repr(C, align(8))]
pub struct UCheckedStateBinding {
    __padding_end: [u8; 136],
}
impl UCheckedStateBinding {}
#[repr(C, align(8))]
pub struct UColorBinding {
    __padding_end: [u8; 136],
}
impl UColorBinding {}
#[repr(C, align(8))]
pub struct UFloatBinding {
    __padding_end: [u8; 128],
}
impl UFloatBinding {}
#[repr(C, align(8))]
pub struct UInt32Binding {
    __padding_end: [u8; 128],
}
impl UInt32Binding {}
#[repr(C, align(8))]
pub struct UMouseCursorBinding {
    __padding_end: [u8; 128],
}
impl UMouseCursorBinding {}
#[repr(C, align(8))]
pub struct UWidgetBinaryStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetBinaryStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetHoveredStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetHoveredStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetPressedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetPressedStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetDisabledStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetDisabledStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetSelectedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetSelectedStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetEnumStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetEnumStateRegistration {}
#[repr(C, align(8))]
pub struct UWidgetStateSettings {
    __padding_end: [u8; 2136],
}
impl UWidgetStateSettings {}
#[repr(C, align(8))]
pub struct UTextBinding {
    __padding_end: [u8; 136],
}
impl UTextBinding {}
#[repr(C, align(8))]
pub struct UVisibilityBinding {
    __padding_end: [u8; 128],
}
impl UVisibilityBinding {}
#[repr(C, align(8))]
pub struct UWidgetBinding {
    __padding_end: [u8; 128],
}
impl UWidgetBinding {}
#[repr(C, align(8))]
pub struct UAsyncTaskDownloadImage {
    __padding_end: [u8; 104],
}
impl UAsyncTaskDownloadImage {}
#[repr(C, align(8))]
pub struct UGameViewportSubsystem {
    __padding_end: [u8; 200],
}
impl UGameViewportSubsystem {}
pub struct UUserListEntry {}
pub struct IserListEntry {}
#[repr(C, align(8))]
pub struct UUserListEntryLibrary {
    __padding_end: [u8; 48],
}
impl UUserListEntryLibrary {}
pub struct UUserObjectListEntry {}
pub struct IserObjectListEntry {}
#[repr(C, align(8))]
pub struct UUserObjectListEntryLibrary {
    __padding_end: [u8; 48],
}
impl UUserObjectListEntryLibrary {}
#[repr(C, align(16))]
pub struct UBackgroundBlur {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_apply_alpha_to_blur: bool,
    pub blur_strength: f32,
    #[doc(hidden)]
    __padding_748: [u8; 4],
    pub blur_radius: i32,
    pub corner_radius: crate::bindings::core_u_object::FVector4,
    pub low_quality_fallback_brush: crate::bindings::slate_core::FSlateBrush,
    __padding_end: [u8; 16],
}
impl UBackgroundBlur {}
#[repr(C, align(8))]
pub struct UPanelSlot {
    __padding_end: [u8; 64],
}
impl UPanelSlot {}
#[repr(C, align(8))]
pub struct UBackgroundBlurSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UBackgroundBlurSlot {}
#[repr(C, align(16))]
pub struct UBorder {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub flags_722: u8,
    pub content_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_776: [u8; 36],
    pub padding: crate::bindings::slate_core::FMargin,
    pub background: crate::bindings::slate_core::FSlateBrush,
    #[doc(hidden)]
    __padding_1040: [u8; 32],
    pub brush_color: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1088: [u8; 32],
    pub desired_size_scale: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 176],
}
impl UBorder {}
#[repr(C, align(8))]
pub struct UBorderSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UBorderSlot {}
#[repr(C, align(8))]
pub struct UButtonSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UButtonSlot {}
#[repr(C, align(8))]
pub struct UCanvasPanel {
    __padding_end: [u8; 736],
}
impl UCanvasPanel {}
#[repr(C, align(8))]
pub struct UCanvasPanelSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub layout_data: FAnchorData,
    pub b_auto_size: bool,
    pub z_order: i32,
    __padding_end: [u8; 152],
}
impl UCanvasPanelSlot {}
#[repr(C, align(8))]
pub struct UWidgetCheckedStateRegistration {
    __padding_end: [u8; 48],
}
impl UWidgetCheckedStateRegistration {}
#[repr(C, align(16))]
pub struct UComboBox {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub items: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    #[doc(hidden)]
    __padding_2656: [u8; 32],
    pub b_is_focusable: bool,
    __padding_end: [u8; 31],
}
impl UComboBox {}
#[repr(C, align(8))]
pub struct UDynamicEntryBoxBase {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub entry_spacing: crate::bindings::core_u_object::FVector2D,
    pub spacing_pattern: TArray<crate::bindings::core_u_object::FVector2D>,
    pub entry_box_type: EDynamicBoxType,
    pub entry_size_rule: FSlateChildSize,
    pub entry_horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub entry_vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub max_element_size: i32,
    pub radial_box_settings: FRadialBoxSettings,
    __padding_end: [u8; 156],
}
impl UDynamicEntryBoxBase {}
#[repr(C, align(16))]
pub struct UDynamicEntryBox {
    #[doc(hidden)]
    __padding_976: [u8; 976],
    pub entry_widget_class: TSubclassOf<UUserWidget>,
    __padding_end: [u8; 8],
}
impl UDynamicEntryBox {}
#[repr(C, align(8))]
pub struct UGridPanel {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub column_fill: TArray<f32>,
    pub row_fill: TArray<f32>,
    __padding_end: [u8; 16],
}
impl UGridPanel {}
#[repr(C, align(8))]
pub struct UGridSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub row_span: i32,
    pub column: i32,
    pub column_span: i32,
    pub layer: i32,
    pub nudge: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 8],
}
impl UGridSlot {}
#[repr(C, align(8))]
pub struct UHorizontalBox {
    __padding_end: [u8; 736],
}
impl UHorizontalBox {}
#[repr(C, align(8))]
pub struct UHorizontalBoxSlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UHorizontalBoxSlot {}
#[repr(C, align(16))]
pub struct UImage {
    #[doc(hidden)]
    __padding_704: [u8; 704],
    pub brush: crate::bindings::slate_core::FSlateBrush,
    #[doc(hidden)]
    __padding_944: [u8; 32],
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_992: [u8; 32],
    pub b_flip_for_right_to_left_flow_direction: bool,
    __padding_end: [u8; 143],
}
impl UImage {}
#[repr(C, align(8))]
pub struct UInvalidationBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_can_cache: bool,
    __padding_end: [u8; 23],
}
impl UInvalidationBox {}
#[repr(C, align(8))]
pub struct UMenuAnchor {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub menu_class: TSubclassOf<UUserWidget>,
    #[doc(hidden)]
    __padding_792: [u8; 64],
    pub placement: crate::bindings::slate_core::EMenuPlacement,
    pub b_fit_in_window: bool,
    pub should_defer_painting_after_window_content: bool,
    pub use_application_menu_stack: bool,
    pub show_menu_background: bool,
    __padding_end: [u8; 43],
}
impl UMenuAnchor {}
#[repr(C, align(8))]
pub struct UUIComponent {
    __padding_end: [u8; 96],
}
impl UUIComponent {}
#[repr(C, align(8))]
pub struct UMouseHoverComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub b_is_hovered: bool,
    __padding_end: [u8; 7],
}
impl UMouseHoverComponent {}
#[repr(C, align(8))]
pub struct UNamedSlot {
    __padding_end: [u8; 760],
}
impl UNamedSlot {}
pub struct UNamedSlotInterface {}
pub struct INamedSlotInterface {}
#[repr(C, align(8))]
pub struct UNativeWidgetHost {
    __padding_end: [u8; 712],
}
impl UNativeWidgetHost {}
#[repr(C, align(8))]
pub struct UOverlay {
    __padding_end: [u8; 736],
}
impl UOverlay {}
#[repr(C, align(8))]
pub struct UOverlaySlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UOverlaySlot {}
#[repr(C, align(8))]
pub struct USlatePostBufferProcessorUpdater {
    __padding_end: [u8; 56],
}
impl USlatePostBufferProcessorUpdater {}
#[repr(C, align(8))]
pub struct UPostBufferBlurUpdater {
    __padding_end: [u8; 64],
}
impl UPostBufferBlurUpdater {}
#[repr(C, align(8))]
pub struct UPostBufferUpdate {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub b_update_only_paint_area: bool,
    pub b_perform_default_post_buffer_update: bool,
    __padding_end: [u8; 54],
}
impl UPostBufferUpdate {}
#[repr(C, align(8))]
pub struct URetainerBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_retain_render: bool,
    pub render_on_invalidation: bool,
    pub render_on_phase: bool,
    pub phase: i32,
    pub phase_count: i32,
    pub effect_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub texture_parameter: FName,
    pub b_show_effects_in_designer: bool,
    __padding_end: [u8; 19],
}
impl URetainerBox {}
#[repr(C, align(16))]
pub struct URichTextBlock {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    pub text_style_set: UPtr<crate::bindings::engine::UDataTable>,
    #[doc(hidden)]
    __padding_768: [u8; 16],
    pub default_text_style_override: crate::bindings::slate_core::FTextBlockStyle,
    pub min_desired_width: f32,
    #[doc(hidden)]
    __padding_1621: [u8; 1],
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    __padding_end: [u8; 905],
}
impl URichTextBlock {}
#[repr(C, align(8))]
pub struct URichTextBlockDecorator {
    __padding_end: [u8; 48],
}
impl URichTextBlockDecorator {}
#[repr(C, align(8))]
pub struct URichTextBlockImageDecorator {
    __padding_end: [u8; 56],
}
impl URichTextBlockImageDecorator {}
#[repr(C, align(8))]
pub struct USafeZone {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub pad_left: bool,
    pub pad_right: bool,
    pub pad_top: bool,
    pub pad_bottom: bool,
    __padding_end: [u8; 52],
}
impl USafeZone {}
#[repr(C, align(8))]
pub struct USafeZoneSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub b_is_title_safe: bool,
    pub safe_area_scale: crate::bindings::slate_core::FMargin,
    pub h_align: crate::bindings::slate_core::EHorizontalAlignment,
    pub v_align: crate::bindings::slate_core::EVerticalAlignment,
    pub padding: crate::bindings::slate_core::FMargin,
    __padding_end: [u8; 16],
}
impl USafeZoneSlot {}
#[repr(C, align(8))]
pub struct UScaleBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
    __padding_end: [u8; 47],
}
impl UScaleBox {}
#[repr(C, align(8))]
pub struct UScaleBoxComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
    __padding_end: [u8; 23],
}
impl UScaleBoxComponent {}
#[repr(C, align(8))]
pub struct UScaleBoxSlot {
    #[doc(hidden)]
    __padding_80: [u8; 80],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UScaleBoxSlot {}
#[repr(C, align(8))]
pub struct UScrollBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UScrollBoxSlot {}
#[repr(C, align(8))]
pub struct USizeBox {
    #[doc(hidden)]
    __padding_736: [u8; 736],
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    __padding_end: [u8; 8],
}
impl USizeBox {}
#[repr(C, align(8))]
pub struct USizeBoxComponent {
    #[doc(hidden)]
    __padding_96: [u8; 96],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    __padding_end: [u8; 20],
}
impl USizeBoxComponent {}
#[repr(C, align(8))]
pub struct USizeBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    #[doc(hidden)]
    __padding_96: [u8; 16],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl USizeBoxSlot {}
#[repr(C, align(8))]
pub struct USpacer {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub size: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 16],
}
impl USpacer {}
#[repr(C, align(8))]
pub struct UStackBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 23],
}
impl UStackBox {}
#[repr(C, align(8))]
pub struct UStackBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub size: FSlateChildSize,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UStackBoxSlot {}
#[repr(C, align(16))]
pub struct UTextBlock {
    #[doc(hidden)]
    __padding_728: [u8; 728],
    pub text: FText,
    #[doc(hidden)]
    __padding_776: [u8; 32],
    pub color_and_opacity: crate::bindings::slate_core::FSlateColor,
    #[doc(hidden)]
    __padding_832: [u8; 36],
    pub min_desired_width: f32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub strike_brush: crate::bindings::slate_core::FSlateBrush,
    pub shadow_offset: crate::bindings::core_u_object::FVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    __padding_1232: [u8; 32],
    pub b_wrap_with_invalidation_panel: bool,
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub b_simple_text_mode: bool,
    __padding_end: [u8; 76],
}
impl UTextBlock {}
#[repr(C, align(16))]
pub struct UTileView {
    __padding_end: [u8; 5120],
}
impl UTileView {}
#[repr(C, align(8))]
pub struct UUniformGridPanel {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub slot_padding: crate::bindings::slate_core::FMargin,
    pub min_desired_slot_width: f32,
    pub min_desired_slot_height: f32,
    __padding_end: [u8; 16],
}
impl UUniformGridPanel {}
#[repr(C, align(8))]
pub struct UUniformGridSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub column: i32,
    __padding_end: [u8; 12],
}
impl UUniformGridSlot {}
#[repr(C, align(8))]
pub struct UVerticalBox {
    __padding_end: [u8; 736],
}
impl UVerticalBox {}
#[repr(C, align(8))]
pub struct UVerticalBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 14],
}
impl UVerticalBoxSlot {}
#[repr(C, align(8))]
pub struct UViewport {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 80],
}
impl UViewport {}
#[repr(C, align(16))]
pub struct UWidgetInteractionComponent {
    #[doc(hidden)]
    __padding_696: [u8; 696],
    pub virtual_user_index: i32,
    pub pointer_index: i32,
    pub trace_channel: crate::bindings::engine::ECollisionChannel,
    pub interaction_distance: f32,
    pub interaction_source: EWidgetInteractionSource,
    pub b_enable_hit_testing: bool,
    pub b_show_debug: bool,
    pub debug_sphere_line_thickness: f32,
    pub debug_line_thickness: f32,
    pub debug_color: crate::bindings::core_u_object::FLinearColor,
    __padding_end: [u8; 716],
}
impl UWidgetInteractionComponent {}
#[repr(C, align(8))]
pub struct UWidgetSwitcher {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub active_widget_index: i32,
    __padding_end: [u8; 20],
}
impl UWidgetSwitcher {}
#[repr(C, align(8))]
pub struct UWidgetSwitcherSlot {
    #[doc(hidden)]
    __padding_72: [u8; 72],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 6],
}
impl UWidgetSwitcherSlot {}
#[repr(C, align(8))]
pub struct UWindowTitleBarArea {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub b_window_buttons_enabled: bool,
    pub b_double_click_toggles_fullscreen: bool,
    __padding_end: [u8; 30],
}
impl UWindowTitleBarArea {}
#[repr(C, align(8))]
pub struct UWindowTitleBarAreaSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    __padding_end: [u8; 22],
}
impl UWindowTitleBarAreaSlot {}
#[repr(C, align(8))]
pub struct UWrapBox {
    #[doc(hidden)]
    __padding_720: [u8; 720],
    pub inner_slot_padding: crate::bindings::core_u_object::FVector2D,
    pub wrap_size: f32,
    pub b_explicit_wrap_size: bool,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub orientation: crate::bindings::slate_core::EOrientation,
    __padding_end: [u8; 17],
}
impl UWrapBox {}
#[repr(C, align(8))]
pub struct UWrapBoxSlot {
    #[doc(hidden)]
    __padding_64: [u8; 64],
    pub padding: crate::bindings::slate_core::FMargin,
    pub fill_span_when_less_than: f32,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_fill_empty_space: bool,
    pub b_force_new_line: bool,
    __padding_end: [u8; 8],
}
impl UWrapBoxSlot {}
#[repr(C, align(8))]
pub struct UDragDropOperation {
    #[doc(hidden)]
    __padding_48: [u8; 48],
    pub tag: FString,
    pub payload: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_drag_visual: UPtr<UWidget>,
    pub pivot: EDragPivot,
    pub offset: crate::bindings::core_u_object::FVector2D,
    __padding_end: [u8; 72],
}
impl UDragDropOperation {}
#[repr(C, align(8))]
pub struct UUIComponentContainer {
    __padding_end: [u8; 64],
}
impl UUIComponentContainer {}
#[repr(C, align(8))]
pub struct UNavigationUIComponent {
    __padding_end: [u8; 184],
}
impl UNavigationUIComponent {}
#[repr(C, align(8))]
pub struct UUIComponentUserWidgetExtension {
    __padding_end: [u8; 64],
}
impl UUIComponentUserWidgetExtension {}
#[repr(C, align(8))]
pub struct UUIComponentWidgetBlueprintGeneratedClassExtension {
    __padding_end: [u8; 56],
}
impl UUIComponentWidgetBlueprintGeneratedClassExtension {}
#[repr(C, align(8))]
pub struct USlateBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl USlateBlueprintLibrary {}
#[repr(C, align(8))]
pub struct USlateVectorArtData {
    __padding_end: [u8; 136],
}
impl USlateVectorArtData {}
#[repr(C, align(8))]
pub struct UUserWidgetFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UUserWidgetFunctionLibrary {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintGeneratedClass {
    __padding_end: [u8; 1992],
}
impl UWidgetBlueprintGeneratedClass {}
#[repr(C, align(8))]
pub struct UWidgetBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetBlueprintLibrary {}
#[repr(C, align(8))]
pub struct UWidgetLayoutLibrary {
    __padding_end: [u8; 48],
}
impl UWidgetLayoutLibrary {}
#[repr(C, align(8))]
pub struct UWidgetTree {
    __padding_end: [u8; 160],
}
impl UWidgetTree {}
#[repr(C, align(8))]
pub struct FWidgetNavigationData_CustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAnimationEventBinding_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_AddFieldValueChangedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FK2_RemoveFieldValueChangedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetNavigationRuleCustom_InCustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetNavigationRuleCustomBoundary_InCustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetWindowTitleBarOnCloseClickedDelegate_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationEvent_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationFinished_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindToAnimationStarted_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FListenForInputAction_Callback {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUnbindFromAnimationFinished_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUnbindFromAnimationStarted_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_bIsEnabledDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_ToolTipTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_ToolTipWidgetDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_VisibilityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_AccessibleTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidget_AccessibleSummaryTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_ForegroundColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUserWidget_OnVisibilityChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FUserWidget_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FButton_OnClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnPressed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnReleased {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnHovered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FButton_OnUnhovered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCheckBox_CheckedStateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FCheckBox_OnCheckStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnGenerateContentWidget {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnGenerateItemWidget {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxKey_OnOpening {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnGenerateWidgetEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FComboBoxString_OnOpening {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableText_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableText_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableText_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableText_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditableTextBox_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FExpandableArea_OnExpansionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInputKeySelector_OnKeySelected {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FInputKeySelector_OnIsSelectingKeyChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntryGenerated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntriesGenerated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListViewBase_BP_OnEntryReleased {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnEntryInitialized {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDoubleClicked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragDetected {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragEnter {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragLeave {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemAcceptDrop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemDragCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewDraggingStateChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemIsHoveredChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemSelectionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnItemScrolledIntoView {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewScrolled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnListViewFinishedScrolling {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FListView_BP_OnIsItemSelectableOrNavigable {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableText_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_HintTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_OnTextChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMultiLineEditableTextBox_OnTextCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FProgressBar_PercentDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FProgressBar_FillColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnUserScrolled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnScrollBarVisibilityChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnFocusReceived {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FScrollBox_OnFocusLost {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSlider_OnMouseCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnMouseCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnControllerCaptureBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnControllerCaptureEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlider_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_ValueDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnValueChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnValueCommitted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnBeginSliderMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSpinBox_OnEndSliderMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTreeView_BP_OnGetItemChildren {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTreeView_BP_OnItemExpansionChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FSlateAccessibleWidgetData_AccessibleTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSlateAccessibleWidgetData_AccessibleSummaryTextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetNavigation_CustomDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetAnimationPlayCallbackProxy_Finished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncTaskDownloadImage_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncTaskDownloadImage_OnFail {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FBorder_ContentColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_BackgroundDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_BrushColorDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseButtonDownEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseButtonUpEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseMoveEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBorder_OnMouseDoubleClickEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FComboBox_OnGenerateWidgetEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_BrushDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FImage_OnMouseButtonDownEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnGetMenuContentEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnGetUserMenuContentEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMenuAnchor_OnMenuOpenChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FTextBlock_TextDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTextBlock_ColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FTextBlock_ShadowColorAndOpacityDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWidgetInteractionComponent_OnHoveredWidgetChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDrop {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDragCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FDragDropOperation_OnDragged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationUIComponent_OnNavigationEnteredDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavigationUIComponent_OnNavigationExitedDelegate {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBindingKind(pub u8);
impl EBindingKind {
    pub const FUNCTION: EBindingKind = EBindingKind(0);
    pub const PROPERTY: EBindingKind = EBindingKind(1);
}
#[repr(transparent)]
pub struct ESlateSizeRule(pub u8);
impl ESlateSizeRule {
    pub const AUTOMATIC: ESlateSizeRule = ESlateSizeRule(0);
    pub const FILL: ESlateSizeRule = ESlateSizeRule(1);
}
#[repr(transparent)]
pub struct EWidgetAnimationEvent(pub u8);
impl EWidgetAnimationEvent {
    pub const STARTED: EWidgetAnimationEvent = EWidgetAnimationEvent(0);
    pub const FINISHED: EWidgetAnimationEvent = EWidgetAnimationEvent(1);
}
#[repr(transparent)]
pub struct ESlateVisibility(pub u8);
impl ESlateVisibility {
    pub const VISIBLE: ESlateVisibility = ESlateVisibility(0);
    pub const COLLAPSED: ESlateVisibility = ESlateVisibility(1);
    pub const HIDDEN: ESlateVisibility = ESlateVisibility(2);
    pub const HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(3);
    pub const SELF_HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(4);
}
#[repr(transparent)]
pub struct EUMGSequencePlayMode(pub u8);
impl EUMGSequencePlayMode {
    pub const FORWARD: EUMGSequencePlayMode = EUMGSequencePlayMode(0);
    pub const REVERSE: EUMGSequencePlayMode = EUMGSequencePlayMode(1);
    pub const PING_PONG: EUMGSequencePlayMode = EUMGSequencePlayMode(2);
}
#[repr(transparent)]
pub struct EWidgetGeometryMode(pub u8);
impl EWidgetGeometryMode {
    pub const PLANE: EWidgetGeometryMode = EWidgetGeometryMode(0);
    pub const CYLINDER: EWidgetGeometryMode = EWidgetGeometryMode(1);
}
#[repr(transparent)]
pub struct EWidgetSpace(pub u8);
impl EWidgetSpace {
    pub const WORLD: EWidgetSpace = EWidgetSpace(0);
    pub const SCREEN: EWidgetSpace = EWidgetSpace(1);
}
#[repr(transparent)]
pub struct EWindowVisibility(pub u8);
impl EWindowVisibility {
    pub const VISIBLE: EWindowVisibility = EWindowVisibility(0);
    pub const SELF_HIT_TEST_INVISIBLE: EWindowVisibility = EWindowVisibility(1);
}
#[repr(transparent)]
pub struct ETickMode(pub u8);
impl ETickMode {
    pub const DISABLED: ETickMode = ETickMode(0);
    pub const ENABLED: ETickMode = ETickMode(1);
    pub const AUTOMATIC: ETickMode = ETickMode(2);
}
#[repr(transparent)]
pub struct EUMGItemDropZone(pub u8);
impl EUMGItemDropZone {
    pub const ABOVE_ITEM: EUMGItemDropZone = EUMGItemDropZone(0);
    pub const ONTO_ITEM: EUMGItemDropZone = EUMGItemDropZone(1);
    pub const BELOW_ITEM: EUMGItemDropZone = EUMGItemDropZone(2);
    pub const NONE: EUMGItemDropZone = EUMGItemDropZone(3);
}
#[repr(transparent)]
pub struct ESlateAccessibleBehavior(pub u8);
impl ESlateAccessibleBehavior {
    pub const NOT_ACCESSIBLE: ESlateAccessibleBehavior = ESlateAccessibleBehavior(0);
    pub const AUTO: ESlateAccessibleBehavior = ESlateAccessibleBehavior(1);
    pub const SUMMARY: ESlateAccessibleBehavior = ESlateAccessibleBehavior(2);
    pub const CUSTOM: ESlateAccessibleBehavior = ESlateAccessibleBehavior(3);
    pub const TOOL_TIP: ESlateAccessibleBehavior = ESlateAccessibleBehavior(4);
}
#[repr(transparent)]
pub struct EDesignPreviewSizeMode(pub u8);
impl EDesignPreviewSizeMode {
    pub const FILL_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(0);
    pub const CUSTOM: EDesignPreviewSizeMode = EDesignPreviewSizeMode(1);
    pub const CUSTOM_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(2);
    pub const DESIRED: EDesignPreviewSizeMode = EDesignPreviewSizeMode(3);
    pub const DESIRED_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(4);
}
#[repr(transparent)]
pub struct EWidgetTickFrequency(pub u8);
impl EWidgetTickFrequency {
    pub const NEVER: EWidgetTickFrequency = EWidgetTickFrequency(0);
    pub const AUTO: EWidgetTickFrequency = EWidgetTickFrequency(1);
}
#[repr(transparent)]
pub struct EWidgetTimingPolicy(pub u8);
impl EWidgetTimingPolicy {
    pub const REAL_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(0);
    pub const GAME_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(1);
}
#[repr(transparent)]
pub struct EWidgetBlendMode(pub u8);
impl EWidgetBlendMode {
    pub const OPAQUE: EWidgetBlendMode = EWidgetBlendMode(0);
    pub const MASKED: EWidgetBlendMode = EWidgetBlendMode(1);
    pub const TRANSPARENT: EWidgetBlendMode = EWidgetBlendMode(2);
}
#[repr(transparent)]
pub struct EVirtualKeyboardType(pub u8);
impl EVirtualKeyboardType {
    pub const DEFAULT: EVirtualKeyboardType = EVirtualKeyboardType(0);
    pub const NUMBER: EVirtualKeyboardType = EVirtualKeyboardType(1);
    pub const WEB: EVirtualKeyboardType = EVirtualKeyboardType(2);
    pub const EMAIL: EVirtualKeyboardType = EVirtualKeyboardType(3);
    pub const PASSWORD: EVirtualKeyboardType = EVirtualKeyboardType(4);
    pub const ALPHA_NUMERIC: EVirtualKeyboardType = EVirtualKeyboardType(5);
}
#[repr(transparent)]
pub struct EDragPivot(pub u8);
impl EDragPivot {
    pub const MOUSE_DOWN: EDragPivot = EDragPivot(0);
    pub const TOP_LEFT: EDragPivot = EDragPivot(1);
    pub const TOP_CENTER: EDragPivot = EDragPivot(2);
    pub const TOP_RIGHT: EDragPivot = EDragPivot(3);
    pub const CENTER_LEFT: EDragPivot = EDragPivot(4);
    pub const CENTER_CENTER: EDragPivot = EDragPivot(5);
    pub const CENTER_RIGHT: EDragPivot = EDragPivot(6);
    pub const BOTTOM_LEFT: EDragPivot = EDragPivot(7);
    pub const BOTTOM_CENTER: EDragPivot = EDragPivot(8);
    pub const BOTTOM_RIGHT: EDragPivot = EDragPivot(9);
}
#[repr(transparent)]
pub struct EDynamicBoxType(pub u8);
impl EDynamicBoxType {
    pub const HORIZONTAL: EDynamicBoxType = EDynamicBoxType(0);
    pub const VERTICAL: EDynamicBoxType = EDynamicBoxType(1);
    pub const WRAP: EDynamicBoxType = EDynamicBoxType(2);
    pub const VERTICAL_WRAP: EDynamicBoxType = EDynamicBoxType(3);
    pub const RADIAL: EDynamicBoxType = EDynamicBoxType(4);
    pub const OVERLAY: EDynamicBoxType = EDynamicBoxType(5);
}
#[repr(transparent)]
pub struct EWidgetInteractionSource(pub u8);
impl EWidgetInteractionSource {
    pub const WORLD: EWidgetInteractionSource = EWidgetInteractionSource(0);
    pub const MOUSE: EWidgetInteractionSource = EWidgetInteractionSource(1);
    pub const CENTER_SCREEN: EWidgetInteractionSource = EWidgetInteractionSource(2);
    pub const CUSTOM: EWidgetInteractionSource = EWidgetInteractionSource(3);
}
