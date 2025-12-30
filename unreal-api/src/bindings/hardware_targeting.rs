#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UHardwareTargetingSettings {
    pub targeted_hardware_class: EHardwareClass,
    pub applied_targeted_hardware_class: EHardwareClass,
    pub default_graphics_performance: EGraphicsPreset,
    pub applied_default_graphics_performance: EGraphicsPreset,
}
