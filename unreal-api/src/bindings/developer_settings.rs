#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(8))]
pub struct FPerPlatformSettings {
    pub settings: TArray<UPtr<UPlatformSettings>>,
}
#[repr(C, align(8))]
pub struct FPlatformSettingsInstances {
    pub platform_instance: UPtr<UPlatformSettings>,
    pub other_platforms: TMap<FName, UPtr<UPlatformSettings>>,
}
pub struct UDeveloperSettings {}
pub struct UDeveloperSettingsBackedByCVars {}
pub struct UPlatformSettings {}
pub struct UPlatformSettingsManager {
    pub settings_map: TMap<TSubclassOf<UPlatformSettings>, FPlatformSettingsInstances>,
}
