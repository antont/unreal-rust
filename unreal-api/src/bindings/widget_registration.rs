#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct FToolkitWidgetStyle {
    #[doc(hidden)]
    __padding_16: [u8; 16],
    pub title_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub tool_details_background_brush: crate::bindings::slate_core::FSlateBrush,
    pub title_foreground_color: crate::bindings::slate_core::FSlateColor,
    pub title_padding: crate::bindings::slate_core::FMargin,
    pub active_tool_title_border_padding: crate::bindings::slate_core::FMargin,
    pub tool_context_text_block_padding: crate::bindings::slate_core::FMargin,
    pub title_font: crate::bindings::slate_core::FSlateFontInfo,
    __padding_end: [u8; 8],
}
impl FToolkitWidgetStyle {}
#[repr(C, align(8))]
pub struct UBuilderPersistenceManager {
    __padding_end: [u8; 208],
}
impl UBuilderPersistenceManager {}
