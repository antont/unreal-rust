#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FActionButtonStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
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
impl FActionButtonStyle {}
#[repr(C, align(8))]
pub struct UFilterBarContext {
    __padding_end: [u8; 96],
}
impl UFilterBarContext {}
#[repr(C, align(8))]
pub struct USidebarButtonMenuContext {
    __padding_end: [u8; 80],
}
impl USidebarButtonMenuContext {}
#[repr(C, align(8))]
pub struct UToolSlateWidgetTypesFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UToolSlateWidgetTypesFunctionLibrary {}
#[repr(transparent)]
pub struct EFilterBarLayout(pub u8);
impl EFilterBarLayout {
    pub const HORIZONTAL: EFilterBarLayout = EFilterBarLayout(0);
    pub const VERTICAL: EFilterBarLayout = EFilterBarLayout(1);
}
