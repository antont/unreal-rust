#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FPortalRpcLocateServer {
    pub product_id: FGuid,
    pub product_version: FString,
    pub host_mac_address: FString,
    pub host_user_id: FString,
}
#[repr(C, align(8))]
pub struct FPortalRpcServer {
    pub server_address: FString,
}
