#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAndroidSDKSettings {
    pub sdk_path: crate::bindings::core_u_object::FDirectoryPath,
    pub ndk_path: crate::bindings::core_u_object::FDirectoryPath,
    pub java_path: crate::bindings::core_u_object::FDirectoryPath,
    pub sdkapi_level: FString,
    pub ndkapi_level: FString,
}
