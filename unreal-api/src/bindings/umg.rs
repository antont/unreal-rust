#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FEventReply {}
#[repr(C, align(8))]
pub struct FWidgetTransform {
    pub translation: crate::bindings::core_u_object::FVector2D,
    pub scale: crate::bindings::core_u_object::FVector2D,
    pub shear: crate::bindings::core_u_object::FVector2D,
    pub angle: f32,
}
#[repr(C, align(1))]
pub struct FShapedTextOptions {
    pub flags_0: u8,
    pub text_shaping_method: crate::bindings::slate_core::ETextShapingMethod,
    pub text_flow_direction: crate::bindings::slate::ETextFlowDirection,
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
    pub rule: crate::bindings::slate_core::EUINavigationRule,
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
    pub animation_guid: crate::bindings::core_u_object::FGuid,
    pub b_is_root_widget: bool,
    pub dynamic_binding: crate::bindings::movie_scene::FMovieSceneDynamicBinding,
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
    pub anchors: crate::bindings::slate::FAnchors,
    pub offsets: crate::bindings::slate_core::FMargin,
    pub alignment: crate::bindings::core_u_object::FVector2D,
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
    pub offsets: crate::bindings::slate_core::FMargin,
    pub anchors: crate::bindings::slate::FAnchors,
    pub alignment: crate::bindings::core_u_object::FVector2D,
}
#[repr(C, align(8))]
pub struct FSlatePostBufferUpdateInfo {
    pub buffer_to_update: crate::bindings::slate_core::ESlatePostRT,
    pub post_param_updater: UPtr<USlatePostBufferProcessorUpdater>,
}
#[repr(C, align(16))]
pub struct FRichTextStyleRow {
    pub text_style: crate::bindings::slate_core::FTextBlockStyle,
}
#[repr(C, align(16))]
pub struct FRichImageRow {
    pub brush: crate::bindings::slate_core::FSlateBrush,
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
    pub position: crate::bindings::core_u_object::FVector2f,
    pub color: crate::bindings::core_u_object::FColor,
    pub uv0: crate::bindings::core_u_object::FVector2f,
    pub uv1: crate::bindings::core_u_object::FVector2f,
    pub uv2: crate::bindings::core_u_object::FVector2f,
    pub uv3: crate::bindings::core_u_object::FVector2f,
    pub uv4: crate::bindings::core_u_object::FVector2f,
    pub uv5: crate::bindings::core_u_object::FVector2f,
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
    pub guid: crate::bindings::core_u_object::FGuid,
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
    pub render_transform_pivot: crate::bindings::core_u_object::FVector2D,
    pub flow_direction_preference: crate::bindings::slate_core::EFlowDirectionPreference,
    pub flags_289: u8,
    pub accessible_behavior: ESlateAccessibleBehavior,
    pub accessible_summary_behavior: ESlateAccessibleBehavior,
    pub accessible_text: FText,
    pub accessible_text_delegate: FWidget_AccessibleTextDelegate,
    pub accessible_summary_text: FText,
    pub accessible_summary_text_delegate: FWidget_AccessibleSummaryTextDelegate,
    pub flags_392: u8,
    pub cursor: crate::bindings::core_u_object::EMouseCursor,
    pub clipping: crate::bindings::slate_core::EWidgetClipping,
    pub visibility: ESlateVisibility,
    pub pixel_snapping: crate::bindings::slate_core::EWidgetPixelSnapping,
    pub render_opacity: f32,
    pub accessible_widget_data: UPtr<USlateAccessibleWidgetData>,
    pub navigation: UPtr<UWidgetNavigation>,
    pub native_bindings: TArray<UPtr<UPropertyBinding>>,
    pub designer_flags: u8,
    pub display_label: FString,
    pub category_name: FString,
}
pub struct UUserWidget {
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub color_and_opacity_delegate: FUserWidget_ColorAndOpacityDelegate,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub foreground_color_delegate: FUserWidget_ForegroundColorDelegate,
    pub on_visibility_changed: FUserWidget_OnVisibilityChanged,
    pub padding: crate::bindings::slate_core::FMargin,
    pub priority: i32,
    pub flags_980: u8,
    pub queued_widget_animation_transitions: TArray<FQueuedWidgetAnimationTransition>,
    pub active_sequence_players: TArray<UPtr<UUMGSequencePlayer>>,
    pub animation_tick_manager: UPtr<UUMGSequenceTickManager>,
    pub stopped_sequence_players: TArray<UPtr<UUMGSequencePlayer>>,
    pub named_slot_bindings: TArray<FNamedSlotBinding>,
    pub extensions: TArray<UPtr<UUserWidgetExtension>>,
    pub widget_tree: UPtr<UWidgetTree>,
    pub design_time_size: crate::bindings::core_u_object::FVector2D,
    pub design_size_mode: EDesignPreviewSizeMode,
    pub palette_category: FText,
    pub preview_background: UPtr<crate::bindings::engine::UTexture2D>,
    pub flags_1152: u8,
    pub tick_frequency: EWidgetTickFrequency,
    pub desired_focus_widget: FWidgetChild,
    pub input_component: UPtr<crate::bindings::engine::UInputComponent>,
    pub animation_callbacks: TArray<FAnimationEventBinding>,
}
pub struct UWidgetComponent {
    pub space: EWidgetSpace,
    pub timing_policy: EWidgetTimingPolicy,
    pub widget_class: TSubclassOf<UUserWidget>,
    pub draw_size: crate::bindings::core_u_object::FIntPoint,
    pub b_manually_redraw: bool,
    pub b_redraw_requested: bool,
    pub redraw_time: f32,
    pub current_draw_size: crate::bindings::core_u_object::FIntPoint,
    pub b_use_invalidation_in_world_space: bool,
    pub b_draw_at_desired_size: bool,
    pub pivot: crate::bindings::core_u_object::FVector2D,
    pub b_receive_hardware_input: bool,
    pub b_window_focusable: bool,
    pub window_visibility: EWindowVisibility,
    pub b_apply_gamma_correction: bool,
    pub owner_player: UPtr<crate::bindings::engine::ULocalPlayer>,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub tint_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub opacity_from_texture: f32,
    pub blend_mode: EWidgetBlendMode,
    pub b_is_two_sided: bool,
    pub tick_when_offscreen: bool,
    pub body_setup: UPtr<crate::bindings::engine::UBodySetup>,
    pub translucent_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub translucent_material_one_sided: UPtr<
        crate::bindings::engine::UMaterialInterface,
    >,
    pub opaque_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub opaque_material_one_sided: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub masked_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub masked_material_one_sided: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub render_target: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
    pub material_instance: UPtr<crate::bindings::engine::UMaterialInstanceDynamic>,
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
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub background_color: crate::bindings::core_u_object::FLinearColor,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    pub on_clicked: FButton_OnClicked,
    pub on_pressed: FButton_OnPressed,
    pub on_released: FButton_OnReleased,
    pub on_hovered: FButton_OnHovered,
    pub on_unhovered: FButton_OnUnhovered,
    pub b_allow_drag_drop: bool,
}
pub struct UCheckBox {
    pub checked_state: crate::bindings::slate_core::ECheckBoxState,
    pub checked_state_delegate: FCheckBox_CheckedStateDelegate,
    pub widget_style: crate::bindings::slate_core::FCheckBoxStyle,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub click_method: crate::bindings::slate_core::EButtonClickMethod,
    pub touch_method: crate::bindings::slate_core::EButtonTouchMethod,
    pub press_method: crate::bindings::slate_core::EButtonPressMethod,
    pub is_focusable: bool,
    pub on_check_state_changed: FCheckBox_OnCheckStateChanged,
}
pub struct UCircularThrobber {
    pub number_of_pieces: i32,
    pub period: f32,
    pub radius: f32,
    pub image: crate::bindings::slate_core::FSlateBrush,
    pub b_enable_radius: bool,
}
pub struct UComboBoxKey {
    pub options: TArray<FName>,
    pub selected_option: FName,
    pub widget_style: crate::bindings::slate_core::FComboBoxStyle,
    pub item_style: crate::bindings::slate_core::FTableRowStyle,
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
    pub content_padding: crate::bindings::slate_core::FMargin,
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
    pub on_generate_widget_event: FComboBoxString_OnGenerateWidgetEvent,
    pub on_selection_changed: FComboBoxString_OnSelectionChanged,
    pub on_opening: FComboBoxString_OnOpening,
}
pub struct UEditableText {
    pub text: FText,
    pub text_delegate: FEditableText_TextDelegate,
    pub hint_text: FText,
    pub hint_text_delegate: FEditableText_HintTextDelegate,
    pub widget_style: crate::bindings::slate_core::FEditableTextStyle,
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
    pub virtual_keyboard_options: crate::bindings::slate::FVirtualKeyboardOptions,
    pub virtual_keyboard_trigger: crate::bindings::slate::EVirtualKeyboardTrigger,
    pub virtual_keyboard_dismiss_action: crate::bindings::slate::EVirtualKeyboardDismissAction,
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    pub on_text_changed: FEditableText_OnTextChanged,
    pub on_text_committed: FEditableText_OnTextCommitted,
    pub enable_integrated_keyboard: bool,
}
pub struct UEditableTextBox {
    pub text: FText,
    pub text_delegate: FEditableTextBox_TextDelegate,
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
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
    pub virtual_keyboard_options: crate::bindings::slate::FVirtualKeyboardOptions,
    pub virtual_keyboard_trigger: crate::bindings::slate::EVirtualKeyboardTrigger,
    pub virtual_keyboard_dismiss_action: crate::bindings::slate::EVirtualKeyboardDismissAction,
    pub justification: crate::bindings::slate::ETextJustify,
    pub overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub shaped_text_options: FShapedTextOptions,
    pub on_text_changed: FEditableTextBox_OnTextChanged,
    pub on_text_committed: FEditableTextBox_OnTextCommitted,
    pub b_is_font_deprecation_done: bool,
}
pub struct UExpandableArea {
    pub style: crate::bindings::slate_core::FExpandableAreaStyle,
    pub border_brush: crate::bindings::slate_core::FSlateBrush,
    pub border_color: crate::bindings::slate_core::FSlateColor,
    pub b_is_expanded: bool,
    pub max_height: f32,
    pub header_padding: crate::bindings::slate_core::FMargin,
    pub area_padding: crate::bindings::slate_core::FMargin,
    pub on_expansion_changed: FExpandableArea_OnExpansionChanged,
    pub header_content: UPtr<UWidget>,
    pub body_content: UPtr<UWidget>,
}
pub struct UInputKeySelector {
    pub widget_style: crate::bindings::slate_core::FButtonStyle,
    pub text_style: crate::bindings::slate_core::FTextBlockStyle,
    pub selected_key: crate::bindings::slate::FInputChord,
    pub margin: crate::bindings::slate_core::FMargin,
    pub key_selection_text: FText,
    pub no_key_specified_text: FText,
    pub b_allow_modifier_keys: bool,
    pub b_allow_gamepad_keys: bool,
    pub escape_keys: TArray<crate::bindings::input_core::FKey>,
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
    pub drag_drop_visual_offset: crate::bindings::core_u_object::FVector2D,
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
    pub list_items: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub entry_spacing: f32,
    pub horizontal_entry_spacing: f32,
    pub vertical_entry_spacing: f32,
    pub scroll_bar_padding: crate::bindings::slate_core::FMargin,
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
    pub justification: crate::bindings::slate::ETextJustify,
    pub wrapping_policy: crate::bindings::slate::ETextWrappingPolicy,
    pub flags_701: u8,
    pub apply_line_height_to_bottom_line: bool,
    pub wrap_text_at: f32,
    pub margin: crate::bindings::slate_core::FMargin,
    pub line_height_percentage: f32,
}
pub struct UMultiLineEditableText {
    pub text: FText,
    pub hint_text: FText,
    pub hint_text_delegate: FMultiLineEditableText_HintTextDelegate,
    pub widget_style: crate::bindings::slate_core::FTextBlockStyle,
    pub b_is_read_only: bool,
    pub select_all_text_when_focused: bool,
    pub clear_text_selection_on_focus_loss: bool,
    pub revert_text_on_escape: bool,
    pub clear_keyboard_focus_on_commit: bool,
    pub allow_context_menu: bool,
    pub virtual_keyboard_options: crate::bindings::slate::FVirtualKeyboardOptions,
    pub virtual_keyboard_dismiss_action: crate::bindings::slate::EVirtualKeyboardDismissAction,
    pub on_text_changed: FMultiLineEditableText_OnTextChanged,
    pub on_text_committed: FMultiLineEditableText_OnTextCommitted,
}
pub struct UMultiLineEditableTextBox {
    pub text: FText,
    pub hint_text: FText,
    pub hint_text_delegate: FMultiLineEditableTextBox_HintTextDelegate,
    pub widget_style: crate::bindings::slate_core::FEditableTextBoxStyle,
    pub text_style_deprecated: crate::bindings::slate_core::FTextBlockStyle,
    pub b_is_read_only: bool,
    pub allow_context_menu: bool,
    pub virtual_keyboard_options: crate::bindings::slate::FVirtualKeyboardOptions,
    pub virtual_keyboard_dismiss_action: crate::bindings::slate::EVirtualKeyboardDismissAction,
    pub on_text_changed: FMultiLineEditableTextBox_OnTextChanged,
    pub on_text_committed: FMultiLineEditableTextBox_OnTextCommitted,
    pub b_is_font_deprecation_done: bool,
}
pub struct UProgressBar {
    pub widget_style: crate::bindings::slate_core::FProgressBarStyle,
    pub percent: f32,
    pub bar_fill_type: crate::bindings::slate::EProgressBarFillType,
    pub bar_fill_style: crate::bindings::slate::EProgressBarFillStyle,
    pub b_is_marquee: bool,
    pub border_padding: crate::bindings::core_u_object::FVector2D,
    pub percent_delegate: FProgressBar_PercentDelegate,
    pub fill_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub fill_color_and_opacity_delegate: FProgressBar_FillColorAndOpacityDelegate,
}
pub struct UScrollBar {
    pub widget_style: crate::bindings::slate_core::FScrollBarStyle,
    pub b_always_show_scrollbar: bool,
    pub b_always_show_scrollbar_track: bool,
    pub orientation: crate::bindings::slate_core::EOrientation,
    pub thickness: crate::bindings::core_u_object::FVector2D,
    pub padding: crate::bindings::slate_core::FMargin,
}
pub struct UScrollBox {
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
    pub on_mouse_capture_begin: FSlider_OnMouseCaptureBegin,
    pub on_mouse_capture_end: FSlider_OnMouseCaptureEnd,
    pub on_controller_capture_begin: FSlider_OnControllerCaptureBegin,
    pub on_controller_capture_end: FSlider_OnControllerCaptureEnd,
    pub on_value_changed: FSlider_OnValueChanged,
}
pub struct USpinBox {
    pub value: f32,
    pub value_delegate: FSpinBox_ValueDelegate,
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
    pub keyboard_type: EVirtualKeyboardType,
    pub clear_keyboard_focus_on_commit: bool,
    pub select_all_text_on_commit: bool,
    pub foreground_color: crate::bindings::slate_core::FSlateColor,
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
    pub image: crate::bindings::slate_core::FSlateBrush,
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
    pub routing_policy: crate::bindings::slate_core::EWidgetNavigationRoutingPolicy,
    pub navigation_method: crate::bindings::core_u_object::FInstancedStruct,
}
pub struct UMovieScene2DTransformPropertySystem {}
pub struct UMovieScene2DTransformSection {
    pub transform_mask: FMovieScene2DTransformMask,
    pub translation: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub rotation: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub scale: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub shear: crate::bindings::movie_scene::FMovieSceneFloatChannel,
}
pub struct UMovieScene2DTransformTrack {}
pub struct UMovieSceneMarginPropertySystem {}
pub struct UMovieSceneMarginSection {
    pub top_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub left_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub right_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
    pub bottom_curve: crate::bindings::movie_scene::FMovieSceneFloatChannel,
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
    pub linker: UPtr<crate::bindings::movie_scene::UMovieSceneEntitySystemLinker>,
}
pub struct UWidgetAnimation {
    pub movie_scene: UPtr<crate::bindings::movie_scene::UMovieScene>,
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
    pub source_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
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
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_apply_alpha_to_blur: bool,
    pub blur_strength: f32,
    pub b_override_auto_radius_calculation: bool,
    pub blur_radius: i32,
    pub corner_radius: crate::bindings::core_u_object::FVector4,
    pub low_quality_fallback_brush: crate::bindings::slate_core::FSlateBrush,
}
pub struct UPanelSlot {
    pub parent: UPtr<UPanelWidget>,
    pub content: UPtr<UWidget>,
}
pub struct UBackgroundBlurSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UBorder {
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub flags_722: u8,
    pub content_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub content_color_and_opacity_delegate: FBorder_ContentColorAndOpacityDelegate,
    pub padding: crate::bindings::slate_core::FMargin,
    pub background: crate::bindings::slate_core::FSlateBrush,
    pub background_delegate: FBorder_BackgroundDelegate,
    pub brush_color: crate::bindings::core_u_object::FLinearColor,
    pub brush_color_delegate: FBorder_BrushColorDelegate,
    pub desired_size_scale: crate::bindings::core_u_object::FVector2D,
    pub b_flip_for_right_to_left_flow_direction: bool,
    pub on_mouse_button_down_event: FBorder_OnMouseButtonDownEvent,
    pub on_mouse_button_up_event: FBorder_OnMouseButtonUpEvent,
    pub on_mouse_move_event: FBorder_OnMouseMoveEvent,
    pub on_mouse_double_click_event: FBorder_OnMouseDoubleClickEvent,
}
pub struct UBorderSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UButtonSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UCanvasPanel {}
pub struct UCanvasPanelSlot {
    pub layout_data: FAnchorData,
    pub b_auto_size: bool,
    pub z_order: i32,
}
pub struct UWidgetCheckedStateRegistration {}
pub struct UComboBox {
    pub scroll_bar_style: crate::bindings::slate_core::FScrollBarStyle,
    pub items: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    pub on_generate_widget_event: FComboBox_OnGenerateWidgetEvent,
    pub b_is_focusable: bool,
}
pub struct UDynamicEntryBoxBase {
    pub entry_spacing: crate::bindings::core_u_object::FVector2D,
    pub spacing_pattern: TArray<crate::bindings::core_u_object::FVector2D>,
    pub entry_box_type: EDynamicBoxType,
    pub entry_size_rule: FSlateChildSize,
    pub entry_horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub entry_vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
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
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub row_span: i32,
    pub column: i32,
    pub column_span: i32,
    pub layer: i32,
    pub nudge: crate::bindings::core_u_object::FVector2D,
}
pub struct UHorizontalBox {}
pub struct UHorizontalBoxSlot {
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UImage {
    pub brush: crate::bindings::slate_core::FSlateBrush,
    pub brush_delegate: FImage_BrushDelegate,
    pub color_and_opacity: crate::bindings::core_u_object::FLinearColor,
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
    pub placement: crate::bindings::slate_core::EMenuPlacement,
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
    pub slot_guid: crate::bindings::core_u_object::FGuid,
}
pub struct UNamedSlotInterface {}
pub struct INamedSlotInterface {}
pub struct UNativeWidgetHost {}
pub struct UOverlay {}
pub struct UOverlaySlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
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
    pub buffers_to_update: TArray<crate::bindings::slate_core::ESlatePostRT>,
    pub update_buffer_infos: TArray<FSlatePostBufferUpdateInfo>,
}
pub struct URetainerBox {
    pub b_retain_render: bool,
    pub render_on_invalidation: bool,
    pub render_on_phase: bool,
    pub phase: i32,
    pub phase_count: i32,
    pub effect_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub texture_parameter: FName,
    pub b_show_effects_in_designer: bool,
}
pub struct URichTextBlock {
    pub text: FText,
    pub text_style_set: UPtr<crate::bindings::engine::UDataTable>,
    pub decorator_classes: TArray<TSubclassOf<URichTextBlockDecorator>>,
    pub default_text_style_override: crate::bindings::slate_core::FTextBlockStyle,
    pub min_desired_width: f32,
    pub b_override_default_style: bool,
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub default_text_style: crate::bindings::slate_core::FTextBlockStyle,
    pub instance_decorators: TArray<UPtr<URichTextBlockDecorator>>,
}
pub struct URichTextBlockDecorator {}
pub struct URichTextBlockImageDecorator {
    pub image_set: UPtr<crate::bindings::engine::UDataTable>,
}
pub struct USafeZone {
    pub pad_left: bool,
    pub pad_right: bool,
    pub pad_top: bool,
    pub pad_bottom: bool,
}
pub struct USafeZoneSlot {
    pub b_is_title_safe: bool,
    pub safe_area_scale: crate::bindings::slate_core::FMargin,
    pub h_align: crate::bindings::slate_core::EHorizontalAlignment,
    pub v_align: crate::bindings::slate_core::EVerticalAlignment,
    pub padding: crate::bindings::slate_core::FMargin,
}
pub struct UScaleBox {
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
}
pub struct UScaleBoxComponent {
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub stretch: crate::bindings::slate::EStretch,
    pub stretch_direction: crate::bindings::slate::EStretchDirection,
    pub user_specified_scale: f32,
    pub ignore_inherited_scale: bool,
}
pub struct UScaleBoxSlot {
    pub padding_deprecated: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UScrollBoxSlot {
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
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
    pub flags_148: u8,
}
pub struct USizeBoxSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct USpacer {
    pub size: crate::bindings::core_u_object::FVector2D,
}
pub struct UStackBox {
    pub orientation: crate::bindings::slate_core::EOrientation,
}
pub struct UStackBoxSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub size: FSlateChildSize,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UTextBlock {
    pub text: FText,
    pub text_delegate: FTextBlock_TextDelegate,
    pub color_and_opacity: crate::bindings::slate_core::FSlateColor,
    pub color_and_opacity_delegate: FTextBlock_ColorAndOpacityDelegate,
    pub min_desired_width: f32,
    pub font: crate::bindings::slate_core::FSlateFontInfo,
    pub strike_brush: crate::bindings::slate_core::FSlateBrush,
    pub shadow_offset: crate::bindings::core_u_object::FVector2D,
    pub shadow_color_and_opacity: crate::bindings::core_u_object::FLinearColor,
    pub shadow_color_and_opacity_delegate: FTextBlock_ShadowColorAndOpacityDelegate,
    pub b_wrap_with_invalidation_panel: bool,
    pub text_transform_policy: crate::bindings::slate_core::ETextTransformPolicy,
    pub text_overflow_policy: crate::bindings::slate_core::ETextOverflowPolicy,
    pub b_simple_text_mode: bool,
}
pub struct UTileView {
    pub entry_height: f32,
    pub entry_width: f32,
    pub tile_alignment: crate::bindings::slate::EListItemAlignment,
    pub b_wrap_horizontal_navigation: bool,
    pub scrollbar_disabled_visibility: ESlateVisibility,
    pub b_entry_size_includes_entry_spacing: bool,
}
pub struct UUniformGridPanel {
    pub slot_padding: crate::bindings::slate_core::FMargin,
    pub min_desired_slot_width: f32,
    pub min_desired_slot_height: f32,
}
pub struct UUniformGridSlot {
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub row: i32,
    pub column: i32,
}
pub struct UVerticalBox {}
pub struct UVerticalBoxSlot {
    pub size: FSlateChildSize,
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UViewport {
    pub background_color: crate::bindings::core_u_object::FLinearColor,
}
pub struct UWidgetInteractionComponent {
    pub on_hovered_widget_changed: FWidgetInteractionComponent_OnHoveredWidgetChanged,
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
    pub custom_hit_result: crate::bindings::engine::FHitResult,
    pub local_hit_location: crate::bindings::core_u_object::FVector2D,
    pub last_local_hit_location: crate::bindings::core_u_object::FVector2D,
    pub hovered_widget_component: UPtr<UWidgetComponent>,
    pub weak_hovered_widget_component: TWeakObjectPtr<UWidgetComponent>,
    pub last_hit_result: crate::bindings::engine::FHitResult,
    pub b_is_hovered_widget_interactable: bool,
    pub b_is_hovered_widget_focusable: bool,
    pub b_is_hovered_widget_hit_test_visible: bool,
    pub arrow_component: UPtr<crate::bindings::engine::UArrowComponent>,
}
pub struct UWidgetSwitcher {
    pub active_widget_index: i32,
}
pub struct UWidgetSwitcherSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UWindowTitleBarArea {
    pub b_window_buttons_enabled: bool,
    pub b_double_click_toggles_fullscreen: bool,
}
pub struct UWindowTitleBarAreaSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
}
pub struct UWrapBox {
    pub inner_slot_padding: crate::bindings::core_u_object::FVector2D,
    pub wrap_size: f32,
    pub b_explicit_wrap_size: bool,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub orientation: crate::bindings::slate_core::EOrientation,
}
pub struct UWrapBoxSlot {
    pub padding: crate::bindings::slate_core::FMargin,
    pub fill_span_when_less_than: f32,
    pub horizontal_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub vertical_alignment: crate::bindings::slate_core::EVerticalAlignment,
    pub b_fill_empty_space: bool,
    pub b_force_new_line: bool,
}
pub struct UDragDropOperation {
    pub tag: FString,
    pub payload: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_drag_visual: UPtr<UWidget>,
    pub pivot: EDragPivot,
    pub offset: crate::bindings::core_u_object::FVector2D,
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
    pub mesh_asset: UPtr<crate::bindings::engine::UStaticMesh>,
    pub source_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub vertex_data: TArray<FSlateMeshVertex>,
    pub index_data: TArray<u32>,
    pub material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub extent_min: crate::bindings::core_u_object::FVector2D,
    pub extent_max: crate::bindings::core_u_object::FVector2D,
}
pub struct UUserWidgetFunctionLibrary {}
pub struct UWidgetBlueprintGeneratedClass {
    pub widget_tree: UPtr<UWidgetTree>,
    pub extensions: TArray<UPtr<UWidgetBlueprintGeneratedClassExtension>>,
    pub flags_1664: u8,
    pub bindings: TArray<FDelegateRuntimeBinding>,
    pub animations: TArray<UPtr<UWidgetAnimation>>,
    pub named_slots: TArray<FName>,
    pub named_slots_with_id: TMap<FName, crate::bindings::core_u_object::FGuid>,
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
pub struct FWidgetNavigationData_CustomDelegate;
pub struct FAnimationEventBinding_Delegate;
pub struct FK2_AddFieldValueChangedDelegate_Delegate;
pub struct FK2_RemoveFieldValueChangedDelegate_Delegate;
pub struct FSetNavigationRuleCustom_InCustomDelegate;
pub struct FSetNavigationRuleCustomBoundary_InCustomDelegate;
pub struct FSetWindowTitleBarOnCloseClickedDelegate_Delegate;
pub struct FBindToAnimationEvent_Delegate;
pub struct FBindToAnimationFinished_Delegate;
pub struct FBindToAnimationStarted_Delegate;
pub struct FListenForInputAction_Callback;
pub struct FUnbindFromAnimationFinished_Delegate;
pub struct FUnbindFromAnimationStarted_Delegate;
pub struct FWidget_bIsEnabledDelegate;
pub struct FWidget_ToolTipTextDelegate;
pub struct FWidget_ToolTipWidgetDelegate;
pub struct FWidget_VisibilityDelegate;
pub struct FWidget_AccessibleTextDelegate;
pub struct FWidget_AccessibleSummaryTextDelegate;
pub struct FUserWidget_ColorAndOpacityDelegate;
pub struct FUserWidget_ForegroundColorDelegate;
pub struct FUserWidget_OnVisibilityChanged;
pub struct FUserWidget_Delegate;
pub struct FButton_OnClicked;
pub struct FButton_OnPressed;
pub struct FButton_OnReleased;
pub struct FButton_OnHovered;
pub struct FButton_OnUnhovered;
pub struct FCheckBox_CheckedStateDelegate;
pub struct FCheckBox_OnCheckStateChanged;
pub struct FComboBoxKey_OnGenerateContentWidget;
pub struct FComboBoxKey_OnGenerateItemWidget;
pub struct FComboBoxKey_OnSelectionChanged;
pub struct FComboBoxKey_OnOpening;
pub struct FComboBoxString_OnGenerateWidgetEvent;
pub struct FComboBoxString_OnSelectionChanged;
pub struct FComboBoxString_OnOpening;
pub struct FEditableText_TextDelegate;
pub struct FEditableText_HintTextDelegate;
pub struct FEditableText_OnTextChanged;
pub struct FEditableText_OnTextCommitted;
pub struct FEditableTextBox_TextDelegate;
pub struct FEditableTextBox_HintTextDelegate;
pub struct FEditableTextBox_OnTextChanged;
pub struct FEditableTextBox_OnTextCommitted;
pub struct FExpandableArea_OnExpansionChanged;
pub struct FInputKeySelector_OnKeySelected;
pub struct FInputKeySelector_OnIsSelectingKeyChanged;
pub struct FListViewBase_BP_OnEntryGenerated;
pub struct FListViewBase_BP_OnEntriesGenerated;
pub struct FListViewBase_BP_OnEntryReleased;
pub struct FListView_BP_OnEntryInitialized;
pub struct FListView_BP_OnItemClicked;
pub struct FListView_BP_OnItemDoubleClicked;
pub struct FListView_BP_OnItemDragDetected;
pub struct FListView_BP_OnItemDragEnter;
pub struct FListView_BP_OnItemDragLeave;
pub struct FListView_BP_OnItemAcceptDrop;
pub struct FListView_BP_OnItemDragCancelled;
pub struct FListView_BP_OnListViewDraggingStateChanged;
pub struct FListView_BP_OnItemIsHoveredChanged;
pub struct FListView_BP_OnItemSelectionChanged;
pub struct FListView_BP_OnItemScrolledIntoView;
pub struct FListView_BP_OnListViewScrolled;
pub struct FListView_BP_OnListViewFinishedScrolling;
pub struct FListView_BP_OnIsItemSelectableOrNavigable;
pub struct FMultiLineEditableText_HintTextDelegate;
pub struct FMultiLineEditableText_OnTextChanged;
pub struct FMultiLineEditableText_OnTextCommitted;
pub struct FMultiLineEditableTextBox_HintTextDelegate;
pub struct FMultiLineEditableTextBox_OnTextChanged;
pub struct FMultiLineEditableTextBox_OnTextCommitted;
pub struct FProgressBar_PercentDelegate;
pub struct FProgressBar_FillColorAndOpacityDelegate;
pub struct FScrollBox_OnUserScrolled;
pub struct FScrollBox_OnScrollBarVisibilityChanged;
pub struct FScrollBox_OnFocusReceived;
pub struct FScrollBox_OnFocusLost;
pub struct FSlider_ValueDelegate;
pub struct FSlider_OnMouseCaptureBegin;
pub struct FSlider_OnMouseCaptureEnd;
pub struct FSlider_OnControllerCaptureBegin;
pub struct FSlider_OnControllerCaptureEnd;
pub struct FSlider_OnValueChanged;
pub struct FSpinBox_ValueDelegate;
pub struct FSpinBox_OnValueChanged;
pub struct FSpinBox_OnValueCommitted;
pub struct FSpinBox_OnBeginSliderMovement;
pub struct FSpinBox_OnEndSliderMovement;
pub struct FTreeView_BP_OnGetItemChildren;
pub struct FTreeView_BP_OnItemExpansionChanged;
pub struct FSlateAccessibleWidgetData_AccessibleTextDelegate;
pub struct FSlateAccessibleWidgetData_AccessibleSummaryTextDelegate;
pub struct FWidgetNavigation_CustomDelegate;
pub struct FWidgetAnimationPlayCallbackProxy_Finished;
pub struct FAsyncTaskDownloadImage_OnSuccess;
pub struct FAsyncTaskDownloadImage_OnFail;
pub struct FBorder_ContentColorAndOpacityDelegate;
pub struct FBorder_BackgroundDelegate;
pub struct FBorder_BrushColorDelegate;
pub struct FBorder_OnMouseButtonDownEvent;
pub struct FBorder_OnMouseButtonUpEvent;
pub struct FBorder_OnMouseMoveEvent;
pub struct FBorder_OnMouseDoubleClickEvent;
pub struct FComboBox_OnGenerateWidgetEvent;
pub struct FImage_BrushDelegate;
pub struct FImage_ColorAndOpacityDelegate;
pub struct FImage_OnMouseButtonDownEvent;
pub struct FMenuAnchor_OnGetMenuContentEvent;
pub struct FMenuAnchor_OnGetUserMenuContentEvent;
pub struct FMenuAnchor_OnMenuOpenChanged;
pub struct FTextBlock_TextDelegate;
pub struct FTextBlock_ColorAndOpacityDelegate;
pub struct FTextBlock_ShadowColorAndOpacityDelegate;
pub struct FWidgetInteractionComponent_OnHoveredWidgetChanged;
pub struct FDragDropOperation_OnDrop;
pub struct FDragDropOperation_OnDragCancelled;
pub struct FDragDropOperation_OnDragged;
pub struct FNavigationUIComponent_OnNavigationEnteredDelegate;
pub struct FNavigationUIComponent_OnNavigationExitedDelegate;
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EBindingKind(pub u8);
impl EBindingKind {
    pub const FUNCTION: EBindingKind = EBindingKind(0);
    pub const PROPERTY: EBindingKind = EBindingKind(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateSizeRule(pub u8);
impl ESlateSizeRule {
    pub const AUTOMATIC: ESlateSizeRule = ESlateSizeRule(0);
    pub const FILL: ESlateSizeRule = ESlateSizeRule(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetAnimationEvent(pub u8);
impl EWidgetAnimationEvent {
    pub const STARTED: EWidgetAnimationEvent = EWidgetAnimationEvent(0);
    pub const FINISHED: EWidgetAnimationEvent = EWidgetAnimationEvent(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateVisibility(pub u8);
impl ESlateVisibility {
    pub const VISIBLE: ESlateVisibility = ESlateVisibility(0);
    pub const COLLAPSED: ESlateVisibility = ESlateVisibility(1);
    pub const HIDDEN: ESlateVisibility = ESlateVisibility(2);
    pub const HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(3);
    pub const SELF_HIT_TEST_INVISIBLE: ESlateVisibility = ESlateVisibility(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUMGSequencePlayMode(pub u8);
impl EUMGSequencePlayMode {
    pub const FORWARD: EUMGSequencePlayMode = EUMGSequencePlayMode(0);
    pub const REVERSE: EUMGSequencePlayMode = EUMGSequencePlayMode(1);
    pub const PING_PONG: EUMGSequencePlayMode = EUMGSequencePlayMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetGeometryMode(pub u8);
impl EWidgetGeometryMode {
    pub const PLANE: EWidgetGeometryMode = EWidgetGeometryMode(0);
    pub const CYLINDER: EWidgetGeometryMode = EWidgetGeometryMode(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetSpace(pub u8);
impl EWidgetSpace {
    pub const WORLD: EWidgetSpace = EWidgetSpace(0);
    pub const SCREEN: EWidgetSpace = EWidgetSpace(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWindowVisibility(pub u8);
impl EWindowVisibility {
    pub const VISIBLE: EWindowVisibility = EWindowVisibility(0);
    pub const SELF_HIT_TEST_INVISIBLE: EWindowVisibility = EWindowVisibility(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ETickMode(pub u8);
impl ETickMode {
    pub const DISABLED: ETickMode = ETickMode(0);
    pub const ENABLED: ETickMode = ETickMode(1);
    pub const AUTOMATIC: ETickMode = ETickMode(2);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EUMGItemDropZone(pub u8);
impl EUMGItemDropZone {
    pub const ABOVE_ITEM: EUMGItemDropZone = EUMGItemDropZone(0);
    pub const ONTO_ITEM: EUMGItemDropZone = EUMGItemDropZone(1);
    pub const BELOW_ITEM: EUMGItemDropZone = EUMGItemDropZone(2);
    pub const NONE: EUMGItemDropZone = EUMGItemDropZone(3);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESlateAccessibleBehavior(pub u8);
impl ESlateAccessibleBehavior {
    pub const NOT_ACCESSIBLE: ESlateAccessibleBehavior = ESlateAccessibleBehavior(0);
    pub const AUTO: ESlateAccessibleBehavior = ESlateAccessibleBehavior(1);
    pub const SUMMARY: ESlateAccessibleBehavior = ESlateAccessibleBehavior(2);
    pub const CUSTOM: ESlateAccessibleBehavior = ESlateAccessibleBehavior(3);
    pub const TOOL_TIP: ESlateAccessibleBehavior = ESlateAccessibleBehavior(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EDesignPreviewSizeMode(pub u8);
impl EDesignPreviewSizeMode {
    pub const FILL_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(0);
    pub const CUSTOM: EDesignPreviewSizeMode = EDesignPreviewSizeMode(1);
    pub const CUSTOM_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(2);
    pub const DESIRED: EDesignPreviewSizeMode = EDesignPreviewSizeMode(3);
    pub const DESIRED_ON_SCREEN: EDesignPreviewSizeMode = EDesignPreviewSizeMode(4);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetTickFrequency(pub u8);
impl EWidgetTickFrequency {
    pub const NEVER: EWidgetTickFrequency = EWidgetTickFrequency(0);
    pub const AUTO: EWidgetTickFrequency = EWidgetTickFrequency(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetTimingPolicy(pub u8);
impl EWidgetTimingPolicy {
    pub const REAL_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(0);
    pub const GAME_TIME: EWidgetTimingPolicy = EWidgetTimingPolicy(1);
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetBlendMode(pub u8);
impl EWidgetBlendMode {
    pub const OPAQUE: EWidgetBlendMode = EWidgetBlendMode(0);
    pub const MASKED: EWidgetBlendMode = EWidgetBlendMode(1);
    pub const TRANSPARENT: EWidgetBlendMode = EWidgetBlendMode(2);
}
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EWidgetInteractionSource(pub u8);
impl EWidgetInteractionSource {
    pub const WORLD: EWidgetInteractionSource = EWidgetInteractionSource(0);
    pub const MOUSE: EWidgetInteractionSource = EWidgetInteractionSource(1);
    pub const CENTER_SCREEN: EWidgetInteractionSource = EWidgetInteractionSource(2);
    pub const CUSTOM: EWidgetInteractionSource = EWidgetInteractionSource(3);
}
