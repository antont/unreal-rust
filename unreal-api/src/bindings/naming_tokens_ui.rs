#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UNamingTokensEditableText {
    pub filter_args: crate::bindings::naming_tokens::FNamingTokenFilterArgs,
    pub b_enable_suggestion_dropdown: bool,
    pub b_is_multiline: bool,
    pub b_display_token_icon: bool,
    pub b_display_error_message: bool,
    pub b_display_border_image: bool,
    pub argument_style: crate::bindings::slate_core::FTextBlockStyle,
    pub background_color: crate::bindings::slate_core::FSlateColor,
    pub b_can_display_resolved_text: bool,
    pub b_display_resolved_text_in_designer: bool,
    pub on_pre_evaluate_naming_tokens: FNamingTokensEditableText_OnPreEvaluateNamingTokens,
    pub contexts: TArray<UPtr<crate::bindings::core_u_object::UObject>>,
}
pub struct FNamingTokensEditableText_OnPreEvaluateNamingTokens;
