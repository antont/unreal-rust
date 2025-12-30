#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UImgMediaSourceFactory {}
pub struct UImgMediaSourceFactoryNew {}
pub struct UImgMediaProcessEXROptions {
    pub input_path: FFilePath,
    pub output_path: FDirectoryPath,
    pub b_enable_mip_mapping: bool,
    pub b_enable_tiling: bool,
    pub tile_size_x: i32,
    pub tile_size_y: i32,
    pub num_threads: i32,
    pub b_use_player: bool,
    pub b_remove_alpha_channel: bool,
    pub b_enable_mip_level_tint: bool,
    pub mip_level_tints: TArray<FLinearColor>,
}
