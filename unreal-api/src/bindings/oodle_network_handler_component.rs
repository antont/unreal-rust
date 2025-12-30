#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
pub struct UOodleNetworkTrainerCommandlet {
    pub b_compression_test: bool,
    pub b_write_v5_dictionaries: bool,
    pub hash_table_size: i32,
    pub dictionary_size: i32,
    pub dictionary_trials: i32,
    pub trial_randomness: i32,
    pub trial_generations: i32,
    pub b_no_trials: bool,
}
