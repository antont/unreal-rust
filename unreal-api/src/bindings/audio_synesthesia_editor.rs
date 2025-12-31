#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UAudioSynesthesiaNRTFactory {
    pub audio_synesthesia_nrt_class: TSubclassOf<
        crate::bindings::audio_synesthesia::UAudioSynesthesiaNRT,
    >,
}
pub struct UAudioSynesthesiaNRTSettingsFactory {
    pub audio_synesthesia_nrt_settings_class: TSubclassOf<
        crate::bindings::audio_synesthesia::UAudioSynesthesiaNRTSettings,
    >,
}
pub struct UAudioSynesthesiaSettingsFactory {
    pub audio_synesthesia_settings_class: TSubclassOf<
        crate::bindings::audio_synesthesia::UAudioSynesthesiaSettings,
    >,
}
