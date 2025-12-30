#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UCompositeCorePluginSettings {
    pub flags_104: u8,
    pub disabled_primitive_classes: TArray<FSoftClassPath>,
    pub allowed_component_classes: TArray<FSoftClassPath>,
    pub scene_view_extension_priority: i32,
}
pub struct UCompositeCoreSubsystem {}
pub struct UHoldoutCompositeComponent {
    pub b_is_enabled: bool,
}
