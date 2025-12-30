#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FEventReply {}
#[repr(C, align(8))]
pub struct FWidgetTransform {
    pub translation: FVector2D,
    pub scale: FVector2D,
    pub shear: FVector2D,
    pub angle: f32,
}
#[repr(C, align(1))]
pub struct FShapedTextOptions {
    pub flags_0: u8,
    pub text_shaping_method: ETextShapingMethod,
    pub text_flow_direction: ETextFlowDirection,
}
#[repr(C, align(8))]
pub struct FPaintContext {}
#[repr(C, align(4))]
pub struct FRadialBoxSettings {
    pub starting_angle: f32,
    pub b_distribute_items_evenly: bool,
    pub b_clockwise_order: bool,
    pub angle_between_items: f32,
    pub sector_central_angle: f32,
}
#[repr(C, align(4))]
pub struct FSlateChildSize {
    pub value: f32,
    pub size_rule: ESlateSizeRule,
}
#[repr(C, align(1))]
pub struct FWidgetEventField {}
#[repr(C, align(8))]
pub struct FWidgetNavigationData {
    pub rule: EUINavigationRule,
    pub widget_to_focus: FName,
    pub widget: TWeakObjectPtr<UWidget>,
    pub custom_delegate: FWidgetNavigationData_CustomDelegate,
}
#[repr(C, align(4))]
pub struct FMovieScene2DTransformMask {
    pub mask: u32,
}
#[repr(C, align(1))]
pub struct FSequenceTickManagerWidgetData {}
#[repr(C, align(8))]
pub struct FWidgetAnimationBinding {
    pub widget_name: FName,
    pub slot_widget_name: FName,
    pub animation_guid: FGuid,
    pub b_is_root_widget: bool,
    pub dynamic_binding: FMovieSceneDynamicBinding,
}
#[repr(C, align(4))]
pub struct FBlueprintWidgetAnimationDelegateBinding {
    pub action: EWidgetAnimationEvent,
    pub animation_to_bind: FName,
    pub function_name_to_bind: FName,
    pub user_tag: FName,
}
#[repr(C, align(8))]
pub struct FWidgetAnimationHandle {}
#[repr(C, align(8))]
pub struct FDynamicPropertyPath {}
#[repr(C, align(8))]
pub struct FWidgetStateBitfield {}
#[repr(C, align(8))]
pub struct FGameViewportWidgetSlot {
    pub anchors: FAnchors,
    pub offsets: FMargin,
    pub alignment: FVector2D,
    pub z_order: i32,
    pub b_auto_remove_on_world_removed: bool,
}
#[repr(C, align(8))]
pub struct FUserWidgetPool {
    pub active_widgets: TArray<UPtr<UUserWidget>>,
    pub inactive_widgets: TArray<UPtr<UUserWidget>>,
}
#[repr(C, align(8))]
pub struct FAnchorData {
    pub offsets: FMargin,
    pub anchors: FAnchors,
    pub alignment: FVector2D,
}
#[repr(C, align(8))]
pub struct FSlatePostBufferUpdateInfo {
    pub buffer_to_update: ESlatePostRT,
    pub post_param_updater: UPtr<USlatePostBufferProcessorUpdater>,
}
#[repr(C, align(16))]
pub struct FRichTextStyleRow {
    pub text_style: FTextBlockStyle,
}
#[repr(C, align(16))]
pub struct FRichImageRow {
    pub brush: FSlateBrush,
}
#[repr(C, align(8))]
pub struct FWidgetComponentInstanceData {}
#[repr(C, align(8))]
pub struct FUIComponentTarget {
    pub target_name: FName,
    pub component: UPtr<UUIComponent>,
}
#[repr(C, align(4))]
pub struct FSlateMeshVertex {
    pub position: FVector2f,
    pub color: FColor,
    pub uv0: FVector2f,
    pub uv1: FVector2f,
    pub uv2: FVector2f,
    pub uv3: FVector2f,
    pub uv4: FVector2f,
    pub uv5: FVector2f,
}
#[repr(C, align(8))]
pub struct FQueuedWidgetAnimationTransition {
    pub widget_animation: UPtr<UWidgetAnimation>,
}
#[repr(C, align(8))]
pub struct FAnimationEventBinding {
    pub animation: UPtr<UWidgetAnimation>,
    pub delegate: FAnimationEventBinding_Delegate,
    pub animation_event: EWidgetAnimationEvent,
    pub user_tag: FName,
}
#[repr(C, align(8))]
pub struct FNamedSlotBinding {
    pub name: FName,
    pub guid: FGuid,
    pub content: UPtr<UWidget>,
}
#[repr(C, align(8))]
pub struct FDelegateRuntimeBinding {
    pub object_name: FString,
    pub property_name: FName,
    pub function_name: FName,
    pub source_path: FDynamicPropertyPath,
    pub kind: EBindingKind,
}
#[repr(C, align(4))]
pub struct FWidgetChild {
    pub widget_name: FName,
    pub widget_ptr: TWeakObjectPtr<UWidget>,
}
pub struct UVisual {}
pub struct UWidget {
    pub slot: UPtr<UPanelSlot>,
    pub b_is_enabled_delegate: FWidget_bIsEnabledDelegate,
    pub tool_tip_text_delegate: FWidget_ToolTipTextDelegate,
    pub tool_tip_text: FText,
    pub tool_tip_widget: UPtr<UWidget>,
    pub tool_tip_widget_delegate: FWidget_ToolTipWidgetDelegate,
    pub visibility_delegate: FWidget_VisibilityDelegate,
    pub render_transform: FWidgetTransform,
    pub render_transform_pivot: FVector2D,
    pub flow_direction_preference: EFlowDirectionPreference,
    pub flags_289: u8,
    pub accessible_behavior: ESlateAccessibleBehavior,
    pub accessible_summary_behavior: ESlateAccessibleBehavior,
    pub accessible_text: FText,
    pub accessible_text_delegate: FWidget_AccessibleTextDelegate,
    pub accessible_summary_text: FText,
    pub accessible_summary_text_delegate: FWidget_AccessibleSummaryTextDelegate,
    pub flags_392: u8,
    pub cursor: EMouseCursor,
    pub clipping: EWidgetClipping,
    pub visibility: ESlateVisibility,
    pub pixel_snapping: EWidgetPixelSnapping,
    pub render_opacity: f32,
    pub accessible_widget_data: UPtr<USlateAccessibleWidgetData>,
    pub navigation: UPtr<UWidgetNavigation>,
    pub native_bindings: TArray<UPtr<UPropertyBinding>>,
    pub designer_flags: u8,
    pub display_label: FString,
    pub category_name: FString,
}
pub struct UUserWidget {
    pub color_and_opacity: FLinearColor,
    pub color_and_opacity_delegate: FUserWidget_ColorAndOpacityDelegate,
    pub foreground_color: FSlateColor,
    pub foreground_color_delegate: FUserWidget_ForegroundColorDelegate,
    pub on_visibility_changed: FUserWidget_OnVisibilityChanged,
    pub padding: FMargin,
    pub priority: i32,
    pub flags_980: u8,
    pub queued_widget_animation_transitions: TArray<FQueuedWidgetAnimationTransition>,
    pub active_sequence_players: TArray<UPtr<UUMGSequencePlayer>>,
    pub animation_tick_manager: UPtr<UUMGSequenceTickManager>,
    pub stopped_sequence_players: TArray<UPtr<UUMGSequencePlayer>>,
    pub named_slot_bindings: TArray<FNamedSlotBinding>,
    pub extensions: TArray<UPtr<UUserWidgetExtension>>,
    pub widget_tree: UPtr<UWidgetTree>,
    pub design_time_size: FVector2D,
    pub design_size_mode: EDesignPreviewSizeMode,
    pub palette_category: FText,
    pub preview_background: UPtr<UTexture2D>,
    pub flags_1152: u8,
    pub tick_frequency: EWidgetTickFrequency,
    pub desired_focus_widget: FWidgetChild,
    pub input_component: UPtr<UInputComponent>,
    pub animation_callbacks: TArray<FAnimationEventBinding>,
}
pub struct UWidgetComponent {
    pub space: EWidgetSpace,
    pub timing_policy: EWidgetTimingPolicy,
    pub widget_class: TSubclassOf<UUserWidget>,
    pub draw_size: FIntPoint,
    pub b_manually_redraw: bool,
    pub b_redraw_requested: bool,
    pub redraw_time: f32,
    pub current_draw_size: FIntPoint,
    pub b_use_invalidation_in_world_space: bool,
    pub b_draw_at_desired_size: bool,
    pub pivot: FVector2D,
    pub b_receive_hardware_input: bool,
    pub b_window_focusable: bool,
    pub window_visibility: EWindowVisibility,
    pub b_apply_gamma_correction: bool,
    pub owner_player: UPtr<ULocalPlayer>,
    pub background_color: FLinearColor,
    pub tint_color_and_opacity: FLinearColor,
    pub opacity_from_texture: f32,
    pub blend_mode: EWidgetBlendMode,
    pub b_is_two_sided: bool,
    pub tick_when_offscreen: bool,
    pub body_setup: UPtr<UBodySetup>,
    pub translucent_material: UPtr<UMaterialInterface>,
    pub translucent_material_one_sided: UPtr<UMaterialInterface>,
    pub opaque_material: UPtr<UMaterialInterface>,
    pub opaque_material_one_sided: UPtr<UMaterialInterface>,
    pub masked_material: UPtr<UMaterialInterface>,
    pub masked_material_one_sided: UPtr<UMaterialInterface>,
    pub render_target: UPtr<UTextureRenderTarget2D>,
    pub material_instance: UPtr<UMaterialInstanceDynamic>,
    pub b_added_to_screen: bool,
    pub b_edit_time_usable: bool,
    pub shared_layer_name: FName,
    pub layer_z_order: i32,
    pub geometry_mode: EWidgetGeometryMode,
    pub cylinder_arc_angle: f64,
    pub tick_mode: ETickMode,
    pub widget: UPtr<UUserWidget>,
}
pub struct UUserWidgetBlueprint {}
pub struct UPanelWidget {
    pub slots: TArray<UPtr<UPanelSlot>>,
}
pub struct UContentWidget {}
pub struct UButton {
    pub widget_style: FButtonStyle,
    pub color_and_opacity: FLinearColor,
    pub background_color: FLinearColor,
    pub click_method: EButtonClickMethod,
    pub touch_method: EButtonTouchMethod,
    pub press_method: EButtonPressMethod,
    pub is_focusable: bool,
    pub on_clicked: FButton_OnClicked,
    pub on_pressed: FButton_OnPressed,
    pub on_released: FButton_OnReleased,
    pub on_hovered: FButton_OnHovered,
    pub on_unhovered: FButton_OnUnhovered,
    pub b_allow_drag_drop: bool,
}
pub struct UCheckBox {
    pub checked_state: ECheckBoxState,
    pub checked_state_delegate: FCheckBox_CheckedStateDelegate,
    pub widget_style: FCheckBoxStyle,
    pub horizontal_alignment: EHorizontalAlignment,
    pub click_method: EButtonClickMethod,
    pub touch_method: EButtonTouchMethod,
    pub press_method: EButtonPressMethod,
    pub is_focusable: bool,
    pub on_check_state_changed: FCheckBox_OnCheckStateChanged,
}
pub struct UCircularThrobber {
    pub number_of_pieces: i32,
    pub period: f32,
    pub radius: f32,
    pub image: FSlateBrush,
    pub b_enable_radius: bool,
}
pub struct UComboBoxKey {
    pub options: TArray<FName>,
    pub selected_option: FName,
    pub widget_style: FComboBoxStyle,
    pub item_style: FTableRowStyle,
    pub scroll_bar_style: FScrollBarStyle,
    pub foreground_color: FSlateColor,
    pub content_padding: FMargin,
    pub max_list_height: f32,
    pub b_has_down_arrow: bool,
    pub b_enable_gamepad_navigation_mode: bool,
    pub b_is_focusable: bool,
    pub on_generate_content_widget: FComboBoxKey_OnGenerateContentWidget,
    pub on_generate_item_widget: FComboBoxKey_OnGenerateItemWidget,
    pub on_selection_changed: FComboBoxKey_OnSelectionChanged,
    pub on_opening: FComboBoxKey_OnOpening,
}
pub struct UComboBoxString {
    pub default_options: TArray<FString>,
    pub selected_option: FString,
    pub widget_style: FComboBoxStyle,
    pub item_style: FTableRowStyle,
    pub scroll_bar_style: FScrollBarStyle,
    pub content_padding: FMargin,
    pub max_list_height: f32,
    pub has_down_arrow: bool,
    pub enable_gamepad_navigation_mode: bool,
    pub font: FSlateFontInfo,
    pub foreground_color: FSlateColor,
    pub b_is_focusable: bool,
    pub on_generate_widget_event: FComboBoxString_OnGenerateWidgetEvent,
    pub on_selection_changed: FComboBoxString_OnSelectionChanged,
    pub on_opening: FComboBoxString_OnOpening,
}
pub struct UEditableText {
    pub text: FText,
    pub text_delegate: FEditableText_TextDelegate,
    pub hint_text: FText,
    pub hint_text_delegate: FEditableText_HintTextDelegate,
    pub widget_style: FEditableTextStyle,
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub allow_context_menu: bool,
    pub keyboard_type: EVirtualKeyboardType,
    pub virtual_keyboard_options: FVirtualKeyboardOptions,
    pub virtual_keyboard_trigger: EVirtualKeyboardTrigger,
    pub virtual_keyboard_dismiss_action: EVirtualKeyboardDismissAction,
    pub justification: ETextJustify,
    pub overflow_policy: ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    pub on_text_changed: FEditableText_OnTextChanged,
    pub on_text_committed: FEditableText_OnTextCommitted,
    pub enable_integrated_keyboard: bool,
}
pub struct UEditableTextBox {
    pub text: FText,
    pub text_delegate: FEditableTextBox_TextDelegate,
    pub widget_style: FEditableTextBoxStyle,
    pub hint_text: FText,
    pub hint_text_delegate: FEditableTextBox_HintTextDelegate,
    pub is_read_only: bool,
    pub is_password: bool,
    pub minimum_desired_width: f32,
    pub is_caret_moved_when_gain_focus: bool,
    pub select_all_text_when_focused: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub allow_context_menu: bool,
    pub keyboard_type: EVirtualKeyboardType,
    pub virtual_keyboard_options: FVirtualKeyboardOptions,
    pub virtual_keyboard_trigger: EVirtualKeyboardTrigger,
    pub virtual_keyboard_dismiss_action: EVirtualKeyboardDismissAction,
    pub justification: ETextJustify,
    pub overflow_policy: ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    pub on_text_changed: FEditableTextBox_OnTextChanged,
    pub on_text_committed: FEditableTextBox_OnTextCommitted,
    pub b_is_font_deprecation_done: bool,
}
pub struct UExpandableArea {
    pub style: FExpandableAreaStyle,
    pub border_brush: FSlateBrush,
    pub border_color: FSlateColor,
    pub b_is_expanded: bool,
    pub max_height: f32,
    pub header_padding: FMargin,
    pub area_padding: FMargin,
    pub on_expansion_changed: FExpandableArea_OnExpansionChanged,
    pub header_content: UPtr<UWidget>,
    pub body_content: UPtr<UWidget>,
}
pub struct UInputKeySelector {
    pub widget_style: FButtonStyle,
    pub text_style: FTextBlockStyle,
    pub selected_key: FInputChord,
    pub margin: FMargin,
    pub key_selection_text: FText,
    pub no_key_specified_text: FText,
    pub b_allow_modifier_keys: bool,
    pub b_allow_gamepad_keys: bool,
    pub escape_keys: TArray<FKey>,
    pub on_key_selected: FInputKeySelector_OnKeySelected,
    pub on_is_selecting_key_changed: FInputKeySelector_OnIsSelectingKeyChanged,
}
pub struct UListViewBase {
    pub bp_on_entry_generated: FListViewBase_BP_OnEntryGenerated,
    pub bp_on_entries_generated: FListViewBase_BP_OnEntriesGenerated,
    pub entry_widget_class: TSubclassOf<UUserWidget>,
    pub wheel_scroll_multiplier: f32,
    pub b_enable_scroll_animation: bool,
    pub scrolling_animation_interpolation_speed: f32,
    pub b_in_enable_touch_animated_scrolling: bool,
    pub allow_overscroll: bool,
    pub b_enable_right_click_scrolling: bool,
    pub b_enable_touch_scrolling: bool,
    pub b_is_pointer_scrolling_enabled: bool,
    pub b_is_gamepad_scrolling_enabled: bool,
    pub b_enable_fixed_line_offset: bool,
    pub fixed_line_scroll_offset: f32,
    pub b_allow_dragging: bool,
    pub b_allow_drag_drop: bool,
    pub drag_drop_visual_pivot: EDragPivot,
    pub drag_drop_visual_offset: FVector2D,
    pub drag_drop_visual_entry_class: TSubclassOf<UUserWidget>,
    pub drag_drop_operation_class: TSubclassOf<UDragDropOperation>,
    pub drag_visual_widget: UPtr<UWidget>,
    pub b_is_dragging: bool,
    pub b_select_item_on_navigation: bool,
    pub b_allow_keep_preselected_items: bool,
    pub bp_on_entry_released: FListViewBase_BP_OnEntryReleased,
    pub num_designer_preview_entries: i32,
    pub entry_widget_pool: FUserWidgetPool,
}
pub struct UListView {
    pub widget_style: FTableViewStyle,
    pub scroll_bar_style: FScrollBarStyle,
    pub b_enable_shadow_brush: bool,
    pub shadow_brush_style: FScrollBoxStyle,
    pub orientation: EOrientation,
    pub selection_mode: ESelectionMode,
    pub consume_mouse_wheel: EConsumeMouseWheel,
    pub b_clear_selection_on_click: bool,
    pub b_is_focusable: bool,
    pub b_clear_scroll_velocity_on_selection: bool,
    pub b_return_focus_to_selection: bool,
    pub scroll_into_view_alignment: EScrollIntoViewAlignment,
    pub list_items: TArray<UPtr<UObject>>,
    pub entry_spacing: f32,
    pub horizontal_entry_spacing: f32,
    pub vertical_entry_spacing: f32,
    pub scroll_bar_padding: FMargin,
    pub bp_on_entry_initialized: FListView_BP_OnEntryInitialized,
    pub bp_on_item_clicked: FListView_BP_OnItemClicked,
    pub bp_on_item_double_clicked: FListView_BP_OnItemDoubleClicked,
    pub bp_on_item_drag_detected: FListView_BP_OnItemDragDetected,
    pub bp_on_item_drag_enter: FListView_BP_OnItemDragEnter,
    pub bp_on_item_drag_leave: FListView_BP_OnItemDragLeave,
    pub bp_on_item_accept_drop: FListView_BP_OnItemAcceptDrop,
    pub bp_on_item_drag_cancelled: FListView_BP_OnItemDragCancelled,
    pub bp_on_list_view_dragging_state_changed: FListView_BP_OnListViewDraggingStateChanged,
    pub bp_on_item_is_hovered_changed: FListView_BP_OnItemIsHoveredChanged,
    pub bp_on_item_selection_changed: FListView_BP_OnItemSelectionChanged,
    pub bp_on_item_scrolled_into_view: FListView_BP_OnItemScrolledIntoView,
    pub bp_on_list_view_scrolled: FListView_BP_OnListViewScrolled,
    pub bp_on_list_view_finished_scrolling: FListView_BP_OnListViewFinishedScrolling,
    pub bp_on_is_item_selectable_or_navigable: FListView_BP_OnIsItemSelectableOrNavigable,
}
pub struct UTextLayoutWidget {
    pub shaped_text_options: FShapedTextOptions,
    pub justification: ETextJustify,
    pub wrapping_policy: ETextWrappingPolicy,
    pub flags_701: u8,
    pub apply_line_height_to_bottom_line: bool,
    pub wrap_text_at: f32,
    pub margin: FMargin,
    pub line_height_percentage: f32,
}
pub struct UMultiLineEditableText {
    pub text: FText,
    pub hint_text: FText,
    pub hint_text_delegate: FMultiLineEditableText_HintTextDelegate,
    pub widget_style: FTextBlockStyle,
    pub b_is_read_only: bool,
    pub select_all_text_when_focused: bool,
    pub clear_text_selection_on_focus_loss: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub allow_context_menu: bool,
    pub virtual_keyboard_options: FVirtualKeyboardOptions,
    pub virtual_keyboard_dismiss_action: EVirtualKeyboardDismissAction,
    pub on_text_changed: FMultiLineEditableText_OnTextChanged,
    pub on_text_committed: FMultiLineEditableText_OnTextCommitted,
}
pub struct UMultiLineEditableTextBox {
    pub text: FText,
    pub hint_text: FText,
    pub hint_text_delegate: FMultiLineEditableTextBox_HintTextDelegate,
    pub widget_style: FEditableTextBoxStyle,
    pub text_style_deprecated: FTextBlockStyle,
    pub b_is_read_only: bool,
    pub allow_context_menu: bool,
    pub virtual_keyboard_options: FVirtualKeyboardOptions,
    pub virtual_keyboard_dismiss_action: EVirtualKeyboardDismissAction,
    pub on_text_changed: FMultiLineEditableTextBox_OnTextChanged,
    pub on_text_committed: FMultiLineEditableTextBox_OnTextCommitted,
    pub b_is_font_deprecation_done: bool,
}
pub struct UProgressBar {
    pub widget_style: FProgressBarStyle,
    pub percent: f32,
    pub bar_fill_type: EProgressBarFillType,
    pub bar_fill_style: EProgressBarFillStyle,
    pub b_is_marquee: bool,
    pub border_padding: FVector2D,
    pub percent_delegate: FProgressBar_PercentDelegate,
    pub fill_color_and_opacity: FLinearColor,
    pub fill_color_and_opacity_delegate: FProgressBar_FillColorAndOpacityDelegate,
}
pub struct UScrollBar {
    pub widget_style: FScrollBarStyle,
    pub b_always_show_scrollbar: bool,
    pub b_always_show_scrollbar_track: bool,
    pub orientation: EOrientation,
    pub thickness: FVector2D,
    pub padding: FMargin,
}
pub struct UScrollBox {
    pub scroll_animation_interpolation_speed: f32,
    pub b_enable_touch_scrolling: bool,
    pub b_consume_pointer_input: bool,
    pub analog_mouse_wheel_key: FKey,
    pub b_is_focusable: bool,
    pub widget_style: FScrollBoxStyle,
    pub widget_bar_style: FScrollBarStyle,
    pub orientation: EOrientation,
    pub scroll_bar_visibility: ESlateVisibility,
    pub consume_mouse_wheel: EConsumeMouseWheel,
    pub scrollbar_thickness: FVector2D,
    pub scrollbar_padding: FMargin,
    pub always_show_scrollbar: bool,
    pub always_show_scrollbar_track: bool,
    pub allow_overscroll: bool,
    pub back_pad_scrolling: bool,
    pub front_pad_scrolling: bool,
    pub b_animate_wheel_scrolling: bool,
    pub navigation_destination: EDescendantScrollDestination,
    pub navigation_scroll_padding: f32,
    pub scroll_when_focus_changes: EScrollWhenFocusChanges,
    pub b_allow_right_click_drag_scrolling: bool,
    pub wheel_scroll_multiplier: f32,
    pub on_user_scrolled: FScrollBox_OnUserScrolled,
    pub on_scroll_bar_visibility_changed: FScrollBox_OnScrollBarVisibilityChanged,
    pub on_focus_received: FScrollBox_OnFocusReceived,
    pub on_focus_lost: FScrollBox_OnFocusLost,
}
pub struct USlider {
    pub value: f32,
    pub value_delegate: FSlider_ValueDelegate,
    pub min_value: f32,
    pub max_value: f32,
    pub widget_style: FSliderStyle,
    pub orientation: EOrientation,
    pub slider_bar_color: FLinearColor,
    pub slider_handle_color: FLinearColor,
    pub indent_handle: bool,
    pub locked: bool,
    pub mouse_uses_step: bool,
    pub requires_controller_lock: bool,
    pub step_size: f32,
    pub is_focusable: bool,
    pub b_prevent_throttling: bool,
    pub on_mouse_capture_begin: FSlider_OnMouseCaptureBegin,
    pub on_mouse_capture_end: FSlider_OnMouseCaptureEnd,
    pub on_controller_capture_begin: FSlider_OnControllerCaptureBegin,
    pub on_controller_capture_end: FSlider_OnControllerCaptureEnd,
    pub on_value_changed: FSlider_OnValueChanged,
}
pub struct USpinBox {
    pub value: f32,
    pub value_delegate: FSpinBox_ValueDelegate,
    pub widget_style: FSpinBoxStyle,
    pub min_fractional_digits: i32,
    pub max_fractional_digits: i32,
    pub b_always_uses_delta_snap: bool,
    pub b_enable_slider: bool,
    pub delta: f32,
    pub slider_exponent: f32,
    pub font: FSlateFontInfo,
    pub justification: ETextJustify,
    pub min_desired_width: f32,
    pub keyboard_type: EVirtualKeyboardType,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub foreground_color: FSlateColor,
    pub on_value_changed: FSpinBox_OnValueChanged,
    pub on_value_committed: FSpinBox_OnValueCommitted,
    pub on_begin_slider_movement: FSpinBox_OnBeginSliderMovement,
    pub on_end_slider_movement: FSpinBox_OnEndSliderMovement,
    pub flags_2536: u8,
    pub min_value: f32,
    pub max_value: f32,
    pub min_slider_value: f32,
    pub max_slider_value: f32,
}
pub struct UThrobber {
    pub number_of_pieces: i32,
    pub b_animate_horizontally: bool,
    pub b_animate_vertically: bool,
    pub b_animate_opacity: bool,
    pub image: FSlateBrush,
}
pub struct UTreeView {
    pub bp_on_get_item_children: FTreeView_BP_OnGetItemChildren,
    pub bp_on_item_expansion_changed: FTreeView_BP_OnItemExpansionChanged,
}
pub struct UListViewDesignerPreviewItem {}
pub struct USlateAccessibleWidgetData {
    pub b_can_children_be_accessible: bool,
    pub accessible_behavior: ESlateAccessibleBehavior,
    pub accessible_summary_behavior: ESlateAccessibleBehavior,
    pub accessible_text: FText,
    pub accessible_text_delegate: FSlateAccessibleWidgetData_AccessibleTextDelegate,
    pub accessible_summary_text: FText,
    pub accessible_summary_text_delegate: FSlateAccessibleWidgetData_AccessibleSummaryTextDelegate,
}
pub struct UUserWidgetExtension {}
pub struct UWidgetBlueprintGeneratedClassExtension {}
pub struct UWidgetFieldNotificationExtension {}
pub struct UWidgetNavigation {
    pub up: FWidgetNavigationData,
    pub down: FWidgetNavigationData,
    pub left: FWidgetNavigationData,
    pub right: FWidgetNavigationData,
    pub next: FWidgetNavigationData,
    pub previous: FWidgetNavigationData,
    pub routing_policy: EWidgetNavigationRoutingPolicy,
    pub navigation_method: FInstancedStruct,
}
pub struct UMovieScene2DTransformPropertySystem {}
pub struct UMovieScene2DTransformSection {
    pub transform_mask: FMovieScene2DTransformMask,
    pub translation: FMovieSceneFloatChannel,
    pub rotation: FMovieSceneFloatChannel,
    pub scale: FMovieSceneFloatChannel,
    pub shear: FMovieSceneFloatChannel,
}
pub struct UMovieScene2DTransformTrack {}
pub struct UMovieSceneMarginPropertySystem {}
pub struct UMovieSceneMarginSection {
    pub top_curve: FMovieSceneFloatChannel,
    pub left_curve: FMovieSceneFloatChannel,
    pub right_curve: FMovieSceneFloatChannel,
    pub bottom_curve: FMovieSceneFloatChannel,
}
pub struct UMovieSceneMarginTrack {}
pub struct UMovieSceneWidgetMaterialSystem {}
pub struct UMovieSceneWidgetMaterialTrack {
    pub brush_property_name_path: TArray<FName>,
    pub track_name: FName,
}
pub struct UUMGSequencePlayer {}
pub struct UUMGSequenceTickManager {
    pub weak_user_widget_data: TMap<
        TWeakObjectPtr<UUserWidget>,
        FSequenceTickManagerWidgetData,
    >,
    pub pending_user_widgets: TArray<UPtr<UUserWidget>>,
    pub linker: UPtr<UMovieSceneEntitySystemLinker>,
}
pub struct UWidgetAnimation {
    pub movie_scene: UPtr<UMovieScene>,
    pub animation_bindings: TArray<FWidgetAnimationBinding>,
    pub b_legacy_finish_on_stop: bool,
    pub display_label: FString,
}
pub struct UWidgetAnimationDelegateBinding {
    pub widget_animation_delegate_bindings: TArray<
        FBlueprintWidgetAnimationDelegateBinding,
    >,
}
pub struct UWidgetAnimationHandleFunctionLibrary {}
pub struct UWidgetAnimationPlayCallbackProxy {
    pub finished: FWidgetAnimationPlayCallbackProxy_Finished,
}
pub struct UPropertyBinding {
    pub source_object: TWeakObjectPtr<UObject>,
    pub source_path: FDynamicPropertyPath,
    pub destination_property: FName,
}
pub struct UBoolBinding {}
pub struct UBrushBinding {}
pub struct UCheckedStateBinding {}
pub struct UColorBinding {}
pub struct UFloatBinding {}
pub struct UInt32Binding {}
pub struct UMouseCursorBinding {}
pub struct UWidgetBinaryStateRegistration {}
pub struct UWidgetHoveredStateRegistration {}
pub struct UWidgetPressedStateRegistration {}
pub struct UWidgetDisabledStateRegistration {}
pub struct UWidgetSelectedStateRegistration {}
pub struct UWidgetEnumStateRegistration {}
pub struct UWidgetStateSettings {}
pub struct UTextBinding {}
pub struct UVisibilityBinding {}
pub struct UWidgetBinding {}
pub struct UAsyncTaskDownloadImage {
    pub on_success: FAsyncTaskDownloadImage_OnSuccess,
    pub on_fail: FAsyncTaskDownloadImage_OnFail,
}
pub struct UGameViewportSubsystem {}
pub struct UUserListEntry {}
pub struct IserListEntry {}
pub struct UUserListEntryLibrary {}
pub struct UUserObjectListEntry {}
pub struct IserObjectListEntry {}
pub struct UUserObjectListEntryLibrary {}
pub struct UBackgroundBlur {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub b_apply_alpha_to_blur: bool,
    pub blur_strength: f32,
    pub b_override_auto_radius_calculation: bool,
    pub blur_radius: i32,
    pub corner_radius: FVector4,
    pub low_quality_fallback_brush: FSlateBrush,
}
pub struct UPanelSlot {
    pub parent: UPtr<UPanelWidget>,
    pub content: UPtr<UWidget>,
}
pub struct UBackgroundBlurSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UBorder {
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub flags_722: u8,
    pub content_color_and_opacity: FLinearColor,
    pub content_color_and_opacity_delegate: FBorder_ContentColorAndOpacityDelegate,
    pub padding: FMargin,
    pub background: FSlateBrush,
    pub background_delegate: FBorder_BackgroundDelegate,
    pub brush_color: FLinearColor,
    pub brush_color_delegate: FBorder_BrushColorDelegate,
    pub desired_size_scale: FVector2D,
    pub b_flip_for_right_to_left_flow_direction: bool,
    pub on_mouse_button_down_event: FBorder_OnMouseButtonDownEvent,
    pub on_mouse_button_up_event: FBorder_OnMouseButtonUpEvent,
    pub on_mouse_move_event: FBorder_OnMouseMoveEvent,
    pub on_mouse_double_click_event: FBorder_OnMouseDoubleClickEvent,
}
pub struct UBorderSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UButtonSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UCanvasPanel {}
pub struct UCanvasPanelSlot {
    pub layout_data: FAnchorData,
    pub b_auto_size: bool,
    pub z_order: i32,
}
pub struct UWidgetCheckedStateRegistration {}
pub struct UComboBox {
    pub scroll_bar_style: FScrollBarStyle,
    pub items: TArray<UPtr<UObject>>,
    pub on_generate_widget_event: FComboBox_OnGenerateWidgetEvent,
    pub b_is_focusable: bool,
}
pub struct UDynamicEntryBoxBase {
    pub entry_spacing: FVector2D,
    pub spacing_pattern: TArray<FVector2D>,
    pub entry_box_type: EDynamicBoxType,
    pub entry_size_rule: FSlateChildSize,
    pub entry_horizontal_alignment: EHorizontalAlignment,
    pub entry_vertical_alignment: EVerticalAlignment,
    pub max_element_size: i32,
    pub radial_box_settings: FRadialBoxSettings,
    pub entry_widget_pool: FUserWidgetPool,
}
pub struct UDynamicEntryBox {
    pub num_designer_preview_entries: i32,
    pub entry_widget_class: TSubclassOf<UUserWidget>,
}
pub struct UGridPanel {
    pub column_fill: TArray<f32>,
    pub row_fill: TArray<f32>,
}
pub struct UGridSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub row: i32,
    pub row_span: i32,
    pub column: i32,
    pub column_span: i32,
    pub layer: i32,
    pub nudge: FVector2D,
}
pub struct UHorizontalBox {}
pub struct UHorizontalBoxSlot {
    pub size: FSlateChildSize,
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UImage {
    pub brush: FSlateBrush,
    pub brush_delegate: FImage_BrushDelegate,
    pub color_and_opacity: FLinearColor,
    pub color_and_opacity_delegate: FImage_ColorAndOpacityDelegate,
    pub b_flip_for_right_to_left_flow_direction: bool,
    pub on_mouse_button_down_event: FImage_OnMouseButtonDownEvent,
}
pub struct UInvalidationBox {
    pub b_can_cache: bool,
}
pub struct UMenuAnchor {
    pub menu_class: TSubclassOf<UUserWidget>,
    pub on_get_menu_content_event: FMenuAnchor_OnGetMenuContentEvent,
    pub on_get_user_menu_content_event: FMenuAnchor_OnGetUserMenuContentEvent,
    pub placement: EMenuPlacement,
    pub b_fit_in_window: bool,
    pub should_defer_painting_after_window_content: bool,
    pub use_application_menu_stack: bool,
    pub show_menu_background: bool,
    pub on_menu_open_changed: FMenuAnchor_OnMenuOpenChanged,
}
pub struct UUIComponent {
    pub owner: TWeakObjectPtr<UWidget>,
}
pub struct UMouseHoverComponent {
    pub b_is_hovered: bool,
}
pub struct UNamedSlot {
    pub b_expose_on_instance_only: bool,
    pub slot_guid: FGuid,
}
pub struct UNamedSlotInterface {}
pub struct INamedSlotInterface {}
pub struct UNativeWidgetHost {}
pub struct UOverlay {}
pub struct UOverlaySlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct USlatePostBufferProcessorUpdater {
    pub b_skip_buffer_update: bool,
}
pub struct UPostBufferBlurUpdater {
    pub gaussian_blur_strength: f32,
}
pub struct UPostBufferUpdate {
    pub b_update_only_paint_area: bool,
    pub b_perform_default_post_buffer_update: bool,
    pub buffers_to_update: TArray<ESlatePostRT>,
    pub update_buffer_infos: TArray<FSlatePostBufferUpdateInfo>,
}
pub struct URetainerBox {
    pub b_retain_render: bool,
    pub render_on_invalidation: bool,
    pub render_on_phase: bool,
    pub phase: i32,
    pub phase_count: i32,
    pub effect_material: UPtr<UMaterialInterface>,
    pub texture_parameter: FName,
    pub b_show_effects_in_designer: bool,
}
pub struct URichTextBlock {
    pub text: FText,
    pub text_style_set: UPtr<UDataTable>,
    pub decorator_classes: TArray<TSubclassOf<URichTextBlockDecorator>>,
    pub default_text_style_override: FTextBlockStyle,
    pub min_desired_width: f32,
    pub b_override_default_style: bool,
    pub text_transform_policy: ETextTransformPolicy,
    pub text_overflow_policy: ETextOverflowPolicy,
    pub default_text_style: FTextBlockStyle,
    pub instance_decorators: TArray<UPtr<URichTextBlockDecorator>>,
}
pub struct URichTextBlockDecorator {}
pub struct URichTextBlockImageDecorator {
    pub image_set: UPtr<UDataTable>,
}
pub struct USafeZone {
    pub pad_left: bool,
    pub pad_right: bool,
    pub pad_top: bool,
    pub pad_bottom: bool,
}
pub struct USafeZoneSlot {
    pub b_is_title_safe: bool,
    pub safe_area_scale: FMargin,
    pub h_align: EHorizontalAlignment,
    pub v_align: EVerticalAlignment,
    pub padding: FMargin,
}
pub struct UScaleBox {
    pub stretch: EStretch,
    pub stretch_direction: EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
}
pub struct UScaleBoxComponent {
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub stretch: EStretch,
    pub stretch_direction: EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
}
pub struct UScaleBoxSlot {
    pub padding_deprecated: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UScrollBoxSlot {
    pub size: FSlateChildSize,
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct USizeBox {
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    pub flags_768: u8,
}
pub struct USizeBoxComponent {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub width_override: f32,
    pub height_override: f32,
    pub min_desired_width: f32,
    pub min_desired_height: f32,
    pub max_desired_width: f32,
    pub max_desired_height: f32,
    pub min_aspect_ratio: f32,
    pub max_aspect_ratio: f32,
    pub flags_148: u8,
}
pub struct USizeBoxSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct USpacer {
    pub size: FVector2D,
}
pub struct UStackBox {
    pub orientation: EOrientation,
}
pub struct UStackBoxSlot {
    pub padding: FMargin,
    pub size: FSlateChildSize,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UTextBlock {
    pub text: FText,
    pub text_delegate: FTextBlock_TextDelegate,
    pub color_and_opacity: FSlateColor,
    pub color_and_opacity_delegate: FTextBlock_ColorAndOpacityDelegate,
    pub min_desired_width: f32,
    pub font: FSlateFontInfo,
    pub strike_brush: FSlateBrush,
    pub shadow_offset: FVector2D,
    pub shadow_color_and_opacity: FLinearColor,
    pub shadow_color_and_opacity_delegate: FTextBlock_ShadowColorAndOpacityDelegate,
    pub b_wrap_with_invalidation_panel: bool,
    pub text_transform_policy: ETextTransformPolicy,
    pub text_overflow_policy: ETextOverflowPolicy,
    pub b_simple_text_mode: bool,
}
pub struct UTileView {
    pub entry_height: f32,
    pub entry_width: f32,
    pub tile_alignment: EListItemAlignment,
    pub b_wrap_horizontal_navigation: bool,
    pub scrollbar_disabled_visibility: ESlateVisibility,
    pub b_entry_size_includes_entry_spacing: bool,
}
pub struct UUniformGridPanel {
    pub slot_padding: FMargin,
    pub min_desired_slot_width: f32,
    pub min_desired_slot_height: f32,
}
pub struct UUniformGridSlot {
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub row: i32,
    pub column: i32,
}
pub struct UVerticalBox {}
pub struct UVerticalBoxSlot {
    pub size: FSlateChildSize,
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UViewport {
    pub background_color: FLinearColor,
}
pub struct UWidgetInteractionComponent {
    pub on_hovered_widget_changed: FWidgetInteractionComponent_OnHoveredWidgetChanged,
    pub virtual_user_index: i32,
    pub pointer_index: i32,
    pub trace_channel: ECollisionChannel,
    pub interaction_distance: f32,
    pub interaction_source: EWidgetInteractionSource,
    pub b_enable_hit_testing: bool,
    pub b_show_debug: bool,
    pub debug_sphere_line_thickness: f32,
    pub debug_line_thickness: f32,
    pub debug_color: FLinearColor,
    pub custom_hit_result: FHitResult,
    pub local_hit_location: FVector2D,
    pub last_local_hit_location: FVector2D,
    pub hovered_widget_component: UPtr<UWidgetComponent>,
    pub weak_hovered_widget_component: TWeakObjectPtr<UWidgetComponent>,
    pub last_hit_result: FHitResult,
    pub b_is_hovered_widget_interactable: bool,
    pub b_is_hovered_widget_focusable: bool,
    pub b_is_hovered_widget_hit_test_visible: bool,
    pub arrow_component: UPtr<UArrowComponent>,
}
pub struct UWidgetSwitcher {
    pub active_widget_index: i32,
}
pub struct UWidgetSwitcherSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UWindowTitleBarArea {
    pub b_window_buttons_enabled: bool,
    pub b_double_click_toggles_fullscreen: bool,
}
pub struct UWindowTitleBarAreaSlot {
    pub padding: FMargin,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
}
pub struct UWrapBox {
    pub inner_slot_padding: FVector2D,
    pub wrap_size: f32,
    pub b_explicit_wrap_size: bool,
    pub horizontal_alignment: EHorizontalAlignment,
    pub orientation: EOrientation,
}
pub struct UWrapBoxSlot {
    pub padding: FMargin,
    pub fill_span_when_less_than: f32,
    pub horizontal_alignment: EHorizontalAlignment,
    pub vertical_alignment: EVerticalAlignment,
    pub b_fill_empty_space: bool,
    pub b_force_new_line: bool,
}
pub struct UDragDropOperation {
    pub tag: FString,
    pub payload: UPtr<UObject>,
    pub default_drag_visual: UPtr<UWidget>,
    pub pivot: EDragPivot,
    pub offset: FVector2D,
    pub on_drop: FDragDropOperation_OnDrop,
    pub on_drag_cancelled: FDragDropOperation_OnDragCancelled,
    pub on_dragged: FDragDropOperation_OnDragged,
}
pub struct UUIComponentContainer {
    pub components: TArray<FUIComponentTarget>,
}
pub struct UNavigationUIComponent {
    pub on_navigation_entered: FName,
    pub on_navigation_exited: FName,
    pub on_navigation_entered_delegate: FNavigationUIComponent_OnNavigationEnteredDelegate,
    pub on_navigation_exited_delegate: FNavigationUIComponent_OnNavigationExitedDelegate,
}
pub struct UUIComponentUserWidgetExtension {
    pub component_container: UPtr<UUIComponentContainer>,
}
pub struct UUIComponentWidgetBlueprintGeneratedClassExtension {
    pub component_container: UPtr<UUIComponentContainer>,
}
pub struct USlateBlueprintLibrary {}
pub struct USlateVectorArtData {
    pub mesh_asset: UPtr<UStaticMesh>,
    pub source_material: UPtr<UMaterialInterface>,
    pub vertex_data: TArray<FSlateMeshVertex>,
    pub index_data: TArray<u32>,
    pub material: UPtr<UMaterialInterface>,
    pub extent_min: FVector2D,
    pub extent_max: FVector2D,
}
pub struct UUserWidgetFunctionLibrary {}
pub struct UWidgetBlueprintGeneratedClass {
    pub widget_tree: UPtr<UWidgetTree>,
    pub extensions: TArray<UPtr<UWidgetBlueprintGeneratedClassExtension>>,
    pub flags_1664: u8,
    pub bindings: TArray<FDelegateRuntimeBinding>,
    pub animations: TArray<UPtr<UWidgetAnimation>>,
    pub named_slots: TArray<FName>,
    pub named_slots_with_id: TMap<FName, FGuid>,
    pub named_slots_with_content_in_same_tree: TSet<FName>,
    pub name_clashing_in_hierarchy: TSet<FName>,
    pub available_named_slots: TArray<FName>,
    pub instance_named_slots: TArray<FName>,
}
pub struct UWidgetBlueprintLibrary {}
pub struct UWidgetLayoutLibrary {}
pub struct UWidgetTree {
    pub root_widget: UPtr<UWidget>,
    pub named_slot_bindings: TMap<FName, UPtr<UWidget>>,
    pub all_widgets: TArray<UPtr<UWidget>>,
}
