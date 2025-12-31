#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FTargetDeviceServiceTerminateLaunchedProcess {
    pub variant: FName,
    pub app_id: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceClaimDenied {
    pub device_name: FString,
    pub host_name: FString,
    pub host_user: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceClaimed {
    pub device_name: FString,
    pub host_name: FString,
    pub host_user: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceUnclaimed {
    pub device_name: FString,
    pub host_name: FString,
    pub host_user: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceServicePing {
    pub host_user: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceVariant {
    pub device_id: FString,
    pub variant_name: FName,
    pub target_platform_name: FString,
    pub target_platform_id: FName,
    pub vanilla_platform_id: FName,
    pub platform_display_name: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceServicePong {
    pub connected: bool,
    pub authorized: bool,
    pub host_name: FString,
    pub host_user: FString,
    pub make: FString,
    pub model: FString,
    pub name: FString,
    pub device_user: FString,
    pub device_user_password: FString,
    pub shared: bool,
    pub supports_multi_launch: bool,
    pub supports_power_off: bool,
    pub supports_power_on: bool,
    pub supports_reboot: bool,
    pub supports_variants: bool,
    pub ty: FString,
    pub os_version: FString,
    pub architecture: FString,
    pub connection_type: FString,
    pub default_variant: FName,
    pub variants: TArray<FTargetDeviceVariant>,
    pub aggregated: bool,
    pub all_devices_name: FString,
    pub all_devices_default_variant: FName,
}
#[repr(C, align(8))]
pub struct FTargetDeviceServicePowerOff {
    pub force: bool,
    pub operator: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceServicePowerOn {
    pub operator: FString,
}
#[repr(C, align(8))]
pub struct FTargetDeviceServiceReboot {
    pub operator: FString,
}
