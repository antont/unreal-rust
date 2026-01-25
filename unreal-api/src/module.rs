use std::sync::OnceLock;

use crate::ffi::UnrealBindings;

pub static BINDINGS: OnceLock<UnrealBindings> = OnceLock::new();

pub fn bindings() -> &'static UnrealBindings {
    BINDINGS.wait()
}
