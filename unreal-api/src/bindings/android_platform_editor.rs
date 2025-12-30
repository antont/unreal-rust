#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UAndroidSDKSettings {
    pub sdk_path: FDirectoryPath,
    pub ndk_path: FDirectoryPath,
    pub java_path: FDirectoryPath,
    pub sdkapi_level: FString,
    pub ndkapi_level: FString,
}
