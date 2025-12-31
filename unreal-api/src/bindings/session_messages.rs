#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FSessionServicePing {
    pub user_name: FString,
}
#[repr(C, align(8))]
pub struct FSessionServicePong {
    pub authorized: bool,
    pub build_date: FString,
    pub device_name: FString,
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub instance_name: FString,
    pub platform_name: FString,
    pub session_id: crate::bindings::core_u_object::FGuid,
    pub session_name: FString,
    pub session_owner: FString,
    pub standalone: bool,
}
#[repr(C, align(8))]
pub struct FSessionServiceLog {
    pub category: FName,
    pub data: FString,
    pub instance_id: crate::bindings::core_u_object::FGuid,
    pub time_seconds: f64,
    pub verbosity: u8,
}
#[repr(C, align(1))]
pub struct FSessionServiceLogSubscribe {}
#[repr(C, align(1))]
pub struct FSessionServiceLogUnsubscribe {}
