#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct ULinuxTargetSettings {
    pub spatialization_plugin: FString,
    pub source_data_override_plugin: FString,
    pub reverb_plugin: FString,
    pub occlusion_plugin: FString,
    pub sound_cue_cook_quality_index: i32,
    pub targeted_rh_is: TArray<FString>,
    pub b_generate_nanite_fallback_meshes: bool,
}
