use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::SystemTime;

use libloading::{Library, Symbol};
use unreal_ffi::{self as ffi, TickFn};

use unreal_ffi::{PluginBindings, RegisterUnrealBindings, RustBindings, UnrealBindings};

pub struct Plugin {
    library: Library,
    register_unreal_bindings: RegisterUnrealBindings,
    rust_bindings: RustBindings,
    timestamp: SystemTime,
}
impl Plugin {
    pub fn new(unreal_bindings: &UnrealBindings, path: &Path) -> Self {
        let library = unsafe { Library::new(path).unwrap() };

        let metadata = std::fs::metadata(path).unwrap();

        let register_unreal_bindings: RegisterUnrealBindings = unsafe {
            *library
                .get::<RegisterUnrealBindings>("register_unreal_bindings\0")
                .unwrap()
        };

        let mut rust_bindings = RustBindings::uninit();

        (register_unreal_bindings)(unreal_bindings.clone(), &mut rust_bindings);

        Plugin {
            library,
            register_unreal_bindings,
            rust_bindings,
            timestamp: metadata.modified().unwrap(),
        }
    }
}

static mut LOADER: *mut Loader = std::ptr::null_mut();

pub struct Loader {
    path: PathBuf,
    loaded_plugin: Option<Plugin>,
    unreal_bindings: UnrealBindings,
    hotreload_id: u32,
}
impl Loader {
    pub fn new(unreal_bindings: UnrealBindings, path: PathBuf) -> Self {
        Loader {
            path,
            loaded_plugin: None,
            unreal_bindings,
            hotreload_id: 0,
        }
    }

    pub fn is_out_of_date(&self) -> bool {
        let Ok(metadata) = std::fs::metadata(&self.path) else {
            return false;
        };
        let Ok(timestamp) = metadata.modified() else {
            return false;
        };

        let Some(plugin) = self.loaded_plugin.as_ref() else {
            return false;
        };

        timestamp > plugin.timestamp
    }

    /// Safety
    pub fn load(&mut self, rust_bindings: &mut RustBindings) {
        // TODO: Maybe add some unloading logic here
        self.loaded_plugin = None;

        let root_dir =
            PathBuf::from("D:\\projects\\unreal-rust\\example\\RustExample\\Binaries\\rust");

        let _ = fs::create_dir_all(&root_dir);

        let hot_reload_dir = root_dir.join(format!("rust-plugin-{}", self.hotreload_id));
        if hot_reload_dir.exists() {
            let _ = fs::remove_dir_all(&hot_reload_dir);
        }
        let hotreload_dll_path = hot_reload_dir.join("rust_plugin.dll");
        let _ = fs::create_dir_all(&hot_reload_dir);
        let _ = fs::copy(&self.path, &hotreload_dll_path);

        let plugin = Plugin::new(&self.unreal_bindings, &hotreload_dll_path);
        *rust_bindings = plugin.rust_bindings.clone();

        self.loaded_plugin = Some(plugin);

        self.hotreload_id += 1;
    }

    pub fn try_load(&mut self, rust_bindings: &mut RustBindings) -> bool {
        if self.is_out_of_date() {
            self.load(rust_bindings);
            return true;
        }

        false
    }
}

#[unsafe(no_mangle)]
extern "C" fn register_unreal_bindings(
    bindings: UnrealBindings,
    rust_bindings: *mut RustBindings,
) -> u32 {
    unsafe {
        if LOADER.is_null() {
            LOADER = Box::leak(Box::new(Loader::new(
                bindings,
                PathBuf::from(
                    "D:\\projects\\unreal-rust\\target\\development\\unreal_rust_example.dll",
                ),
            ))) as *mut _;
        }
        let loader = &mut (*LOADER);
        loader.load(&mut *rust_bindings);
    }
    1
}

// #[unsafe(no_mangle)]
// unsafe extern "C" fn initialize(plugin_bindings: *mut PluginBindings) -> u32 {
//     unsafe {
//         let bindings = PluginBindings { tick, begin_play };
//         *plugin_bindings = bindings;
//     }
//     1
// }

#[unsafe(no_mangle)]
unsafe extern "C" fn try_load(rust_bindings: *mut RustBindings) -> u32 {
    unsafe {
        if !LOADER.is_null() {
            (*LOADER).try_load(&mut *rust_bindings) as u32
        } else {
            0
        }
    }
}
