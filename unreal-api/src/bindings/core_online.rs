#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::opague_definitions::*;
pub use crate::core_data::*;
#[repr(C, align(4))]
pub struct FJoinabilitySettings {
    pub session_name: FName,
    pub b_public_searchable: bool,
    pub b_allow_invites: bool,
    pub b_join_via_presence: bool,
    pub b_join_via_presence_friends_only: bool,
    pub max_players: i32,
    pub max_party_size: i32,
}
#[repr(C, align(1))]
pub struct FUniqueNetIdWrapper {}
