#![allow(clippy::missing_safety_doc)]
extern crate self as unreal_api;

pub mod bindings;
pub mod core_data;
pub use unreal_ffi as ffi;
pub mod log;
pub mod module;
pub use unreal_api_derive::{Component, Event, UClass, inherit};
pub mod property;
pub mod registration;

// // TODO: Here for the unreal_api_derive macro. Lets restructure this
// pub use bevy_ecs as ecs;
pub mod ecs {
    pub use bevy_app::prelude::*;
    pub use bevy_ecs::*;
}
pub use glam as math;
pub use unreal_reflect::*;

pub use serde;
pub use uuid;

pub use inventory;
pub use serde::{Deserialize, Serialize};
pub use serde_json;
