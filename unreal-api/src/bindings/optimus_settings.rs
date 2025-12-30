#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UOptimusSettings {
    pub default_mode: EOptimusDefaultDeformerMode,
    pub default_deformer: TSoftObjectPtr<UMeshDeformer>,
    pub default_recompute_tangent_deformer: TSoftObjectPtr<UMeshDeformer>,
}
