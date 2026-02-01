#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::new_ret_no_self)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(transparent)]
pub struct ESoundwaveSampleRateSettings(pub u8);
impl ESoundwaveSampleRateSettings {
    pub const MAX: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(0);
    pub const HIGH: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(1);
    pub const MEDIUM: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(2);
    pub const LOW: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(3);
    pub const MIN: ESoundwaveSampleRateSettings = ESoundwaveSampleRateSettings(4);
}
