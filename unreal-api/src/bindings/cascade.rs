#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FModuleMenuMapper {
    pub obj_name: FString,
    pub invalid_obj_names: TArray<FString>,
}
pub struct UCascadeConfiguration {
    pub module_menu_type_data_to_base_module_rejections: TArray<FModuleMenuMapper>,
    pub module_menu_type_data_to_specific_module_rejections: TArray<FModuleMenuMapper>,
    pub module_menu_module_rejections: TArray<FString>,
}
pub struct UCascadeParticleSystemComponent {}
