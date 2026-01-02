#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct UCSVtoSVGArugments {
    __padding_end: [u8; 240],
}
impl UCSVtoSVGArugments {}
#[repr(transparent)]
pub struct ESVGTheme(pub i32);
impl ESVGTheme {
    pub const DARK: ESVGTheme = ESVGTheme(0);
    pub const LIGHT: ESVGTheme = ESVGTheme(1);
}
