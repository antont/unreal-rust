#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(4))]
pub struct FInt32Setting {
    pub value: i32,
}
#[repr(C, align(4))]
pub struct FFloatSetting {
    pub value: f32,
}
#[repr(C, align(8))]
pub struct FDoubleSetting {
    pub value: f64,
}
#[repr(C, align(4))]
pub struct FVector3fSetting {
    pub value: FVector3f,
}
#[repr(C, align(8))]
pub struct FVector3dSetting {
    pub value: FVector3d,
}
#[repr(C, align(4))]
pub struct FNameSetting {
    pub value: FName,
}
