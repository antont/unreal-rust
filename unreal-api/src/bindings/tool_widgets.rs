#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCustomTextFilterData {
    pub filter_label: FText,
    pub filter_string: FText,
    pub filter_color: FLinearColor,
}
#[repr(C, align(4))]
pub struct FToolWidget_DragBoxPosition {
    pub relative_offset: FVector2f,
    pub h_align: EHorizontalAlignment,
    pub v_align: EVerticalAlignment,
}
#[repr(C, align(8))]
pub struct FExternalWidgetSelectionColumn {}
#[repr(C, align(8))]
pub struct FWidgetContextMenuColumn {}
#[repr(C, align(8))]
pub struct FWidgetRowScrolledIntoView {}
#[repr(C, align(8))]
pub struct FWidgetDoubleClickedColumn {}
#[repr(C, align(8))]
pub struct FWidgetEnterEditModeColumn {}
#[repr(C, align(4))]
pub struct FHeaderWidgetSizeColumn {
    pub width: f32,
}
#[repr(C, align(8))]
pub struct FSidebarDrawerState {
    pub drawer_id: FName,
    pub selected_sections: TSet<FName>,
    pub b_is_pinned: bool,
    pub b_is_docked: bool,
}
#[repr(C, align(8))]
pub struct FSidebarState {
    pub b_hidden: bool,
    pub drawer_size: f32,
    pub content_size: f32,
    pub drawer_states: TArray<FSidebarDrawerState>,
}
#[repr(C, align(16))]
pub struct FActionButtonStyle {
    pub button_style: FButtonStyle,
    pub button_content_padding: TOptional<FMargin>,
    pub combo_button_style: FComboButtonStyle,
    pub b_has_down_arrow: bool,
    pub combo_button_content_padding: TOptional<FMargin>,
    pub horizontal_content_alignment: EHorizontalAlignment,
    pub text_block_style: FTextBlockStyle,
    pub icon_brush: TOptional<FSlateBrush>,
    pub icon_color_and_opacity: TOptional<FSlateColor>,
    pub icon_normal_padding: TOptional<FMargin>,
    pub icon_pressed_padding: TOptional<FMargin>,
    pub action_button_type: FName,
    pub icon_button_style: TOptional<FButtonStyle>,
}
pub struct UFilterBarContext {}
pub struct USidebarButtonMenuContext {}
pub struct UToolSlateWidgetTypesFunctionLibrary {}
