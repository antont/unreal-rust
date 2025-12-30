#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UInterchangeResultMeshWarning {
    pub mesh_name: FString,
}
pub struct UInterchangeResultTextureDisplay {
    pub texture_name: FString,
}
pub struct UInterchangeResultTextureWarning {
    pub texture_name: FString,
}
pub struct UInterchangeResultMeshError {
    pub mesh_name: FString,
}
pub struct UInterchangeResultMeshWarning_Generic {
    pub text: FText,
}
pub struct UInterchangeResultMeshError_Generic {
    pub text: FText,
}
pub struct UInterchangeResultMeshWarning_TooManyUVs {
    pub excess_u_vs: i32,
}
pub struct UInterchangeResultTextureDisplay_TextureFileDoNotExist {
    pub material_name: FString,
}
