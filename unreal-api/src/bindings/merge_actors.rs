#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UMeshApproximationSettingsObject {
    pub settings: crate::bindings::engine::FMeshApproximationSettings,
}
pub struct UMeshInstancingSettingsObject {
    pub settings: crate::bindings::engine::FMeshInstancingSettings,
}
pub struct UMeshMergingSettingsObject {
    pub settings: crate::bindings::engine::FMeshMergingSettings,
}
pub struct UMeshProxySettingsObject {
    pub settings: crate::bindings::engine::FMeshProxySettings,
}
