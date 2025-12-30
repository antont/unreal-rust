#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct URewindDebuggerVLogSettings {
    pub display_verbosity: u8,
    pub display_categories: TSet<FName>,
}
pub struct AVLogRenderingActor {}
