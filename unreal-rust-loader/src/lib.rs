use std::path::Path;

use libloading::{Library, Symbol};
use unreal_ffi::{self as ffi, TickFn};

use unreal_ffi::{PluginBindings, RegisterUnrealBindings, RustBindings, UnrealBindings};

pub struct Plugin {
    library: Library,
    register_unreal_bindings: RegisterUnrealBindings,
    bindings: Option<RustBindings>,
}
impl Plugin {
    pub fn new(path: &Path) -> Self {
        let library = unsafe { Library::new(path).unwrap() };

        let register_unreal_bindings: RegisterUnrealBindings = unsafe {
            *library
                .get::<RegisterUnrealBindings>("register_unreal_bindings\0")
                .unwrap()
        };

        Plugin {
            library,
            register_unreal_bindings,
            bindings: None,
        }
    }
}

static mut LOADER: *mut Loader = std::ptr::null_mut();

#[derive(Default)]
pub struct Loader {
    loaded_plugin: Option<Plugin>,
}

#[unsafe(no_mangle)]
extern "C" fn register_unreal_bindings(
    bindings: UnrealBindings,
    rust_bindings: *mut RustBindings,
) -> u32 {
    unsafe {
        if LOADER.is_null() {
            LOADER = Box::leak(Box::new(Loader::default())) as *mut _;
        }
        let loader = &mut (*LOADER);
        loader.loaded_plugin = Some(Plugin::new(Path::new(
            "D:\\projects\\unreal-rust\\target\\development\\unreal_rust_example.dll",
        )));

        (loader
            .loaded_plugin
            .as_ref()
            .unwrap()
            .register_unreal_bindings)(bindings, rust_bindings);
        unsafe {
            loader.loaded_plugin.as_mut().unwrap().bindings = Some((*rust_bindings).clone());
        }
    }
    1
}

#[unsafe(no_mangle)]
unsafe extern "C" fn initialize(plugin_bindings: *mut PluginBindings) -> u32 {
    unsafe {
        let bindings = PluginBindings { tick, begin_play };
        *plugin_bindings = bindings;
    }
    1
}

unsafe extern "C" fn tick(dt: f32) -> ffi::ResultCode {
    unsafe {
        if !LOADER.is_null()
            && let Some(plugin) = &mut (*LOADER).loaded_plugin
            && let Some(bindings) = plugin.bindings.as_ref()
        {
            (bindings.tick)(dt);
        }
    }
    ffi::ResultCode::Success
}

unsafe extern "C" fn begin_play() -> ffi::ResultCode {
    ffi::ResultCode::Success
}
