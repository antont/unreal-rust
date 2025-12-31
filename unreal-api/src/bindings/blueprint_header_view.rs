#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FHeaderViewSyntaxColors {
    pub comment: crate::bindings::core_u_object::FLinearColor,
    pub error: crate::bindings::core_u_object::FLinearColor,
    pub macro_: crate::bindings::core_u_object::FLinearColor,
    pub typename: crate::bindings::core_u_object::FLinearColor,
    pub identifier: crate::bindings::core_u_object::FLinearColor,
    pub keyword: crate::bindings::core_u_object::FLinearColor,
}
pub struct UBlueprintHeaderViewSettings {
    pub syntax_colors: FHeaderViewSyntaxColors,
    pub selection_color: crate::bindings::core_u_object::FLinearColor,
    pub font_size: i32,
    pub sort_method: EHeaderViewSortMethod,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct EHeaderViewSortMethod(pub u8);
impl EHeaderViewSortMethod {
    pub const NONE: EHeaderViewSortMethod = EHeaderViewSortMethod(0);
    pub const SORT_BY_ACCESS_SPECIFIER: EHeaderViewSortMethod = EHeaderViewSortMethod(1);
    pub const SORT_FOR_OPTIMAL_PADDING: EHeaderViewSortMethod = EHeaderViewSortMethod(2);
}
