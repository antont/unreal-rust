#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
pub struct UMoviePlayerSettings {
    pub b_wait_for_movies_to_complete: bool,
    pub b_movies_are_skippable: bool,
    pub startup_movies: TArray<FString>,
}
