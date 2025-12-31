#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UCSVtoSVGArugments {
    pub csv: crate::bindings::core_u_object::FFilePath,
    pub output_directory: crate::bindings::core_u_object::FDirectoryPath,
    pub output_filename: FString,
    pub skip_rows: i32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub smooth: bool,
    pub smooth_kernel_size: i32,
    pub smooth_kernel_percent: f32,
    pub width: i32,
    pub height: i32,
    pub title: FString,
    pub no_metadata: bool,
    pub graph_only: bool,
    pub budget: f32,
    pub thickness: f32,
    pub theme: ESVGTheme,
    pub threshold: f32,
    pub stacked: bool,
    pub stacktotalstack: FString,
    pub interactive: bool,
    pub showaverages: bool,
    pub colour_offset: i32,
    pub average_threshold: f32,
}
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct ESVGTheme(pub i32);
impl ESVGTheme {
    pub const DARK: ESVGTheme = ESVGTheme(0);
    pub const LIGHT: ESVGTheme = ESVGTheme(1);
}
