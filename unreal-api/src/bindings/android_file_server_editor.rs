#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAndroidFileServerRuntimeSettings {
    pub b_enable_plugin: bool,
    pub b_allow_network_connection: bool,
    pub security_token: FString,
    pub b_include_in_shipping: bool,
    pub b_allow_external_start_in_shipping: bool,
    pub b_compile_afs_project: bool,
    pub b_use_compression: bool,
    pub b_log_files: bool,
    pub b_report_stats: bool,
    pub connection_type: EAFSConnectionType,
    pub b_use_manual_ip_address: bool,
    pub manual_ip_address: FString,
}
