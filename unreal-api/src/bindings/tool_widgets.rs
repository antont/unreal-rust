#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FCustomTextFilterData {
    pub filter_label: FText,
    pub filter_string: FText,
    pub filter_color: crate::bindings::core_u_object::FLinearColor,
}
#[repr(C, align(4))]
pub struct FToolWidget_DragBoxPosition {
    pub relative_offset: crate::bindings::core_u_object::FVector2f,
    pub h_align: crate::bindings::slate_core::EHorizontalAlignment,
    pub v_align: crate::bindings::slate_core::EVerticalAlignment,
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
    pub button_style: crate::bindings::slate_core::FButtonStyle,
    pub button_content_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub combo_button_style: crate::bindings::slate_core::FComboButtonStyle,
    pub b_has_down_arrow: bool,
    pub combo_button_content_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub horizontal_content_alignment: crate::bindings::slate_core::EHorizontalAlignment,
    pub text_block_style: crate::bindings::slate_core::FTextBlockStyle,
    pub icon_brush: TOptional<crate::bindings::slate_core::FSlateBrush>,
    pub icon_color_and_opacity: TOptional<crate::bindings::slate_core::FSlateColor>,
    pub icon_normal_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub icon_pressed_padding: TOptional<crate::bindings::slate_core::FMargin>,
    pub action_button_type: FName,
    pub icon_button_style: TOptional<crate::bindings::slate_core::FButtonStyle>,
}
pub struct UFilterBarContext {}
pub struct USidebarButtonMenuContext {}
pub struct UToolSlateWidgetTypesFunctionLibrary {}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EFilterBarLayout(pub u8);
impl EFilterBarLayout {
    pub const HORIZONTAL: EFilterBarLayout = EFilterBarLayout(0);
    pub const VERTICAL: EFilterBarLayout = EFilterBarLayout(1);
}
