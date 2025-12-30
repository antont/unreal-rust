#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FHeaderViewSyntaxColors {
    pub comment: FLinearColor,
    pub error: FLinearColor,
    pub macro_: FLinearColor,
    pub typename: FLinearColor,
    pub identifier: FLinearColor,
    pub keyword: FLinearColor,
}
pub struct UBlueprintHeaderViewSettings {
    pub syntax_colors: FHeaderViewSyntaxColors,
    pub selection_color: FLinearColor,
    pub font_size: i32,
    pub sort_method: EHeaderViewSortMethod,
}
