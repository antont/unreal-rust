#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(16))]
pub struct UNamingTokensEditableText {
    #[doc(hidden)]
    __padding_1736: [u8; 1736],
    pub filter_args: crate::bindings::naming_tokens::FNamingTokenFilterArgs,
    #[doc(hidden)]
    __padding_1761: [u8; 1],
    pub b_is_multiline: bool,
    pub b_display_token_icon: bool,
    pub b_display_error_message: bool,
    pub b_display_border_image: bool,
    pub argument_style: crate::bindings::slate_core::FTextBlockStyle,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub b_can_display_resolved_text: bool,
    #[doc(hidden)]
    __padding_2704: [u8; 56],
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
impl UNamingTokensEditableText {}
#[repr(transparent)]
pub struct FNamingTokensEditableText_OnPreEvaluateNamingTokens {
    _opague: u8,
}
