#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UTcpMessagingSettings {
    pub enable_transport: bool,
    pub listen_endpoint: FString,
    pub connect_to_endpoints: TArray<FString>,
    pub connection_retry_delay: i32,
    pub connection_retry_period: i32,
    pub b_stop_service_when_app_deactivates: bool,
}
