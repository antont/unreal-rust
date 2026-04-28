use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use libloading::Library;

use unreal_ffi::{RegisterUnrealBindings, RustBindings, UnrealBindings};

#[cfg(target_os = "windows")]
const PLUGIN_EXTENSION: &str = "dll";
#[cfg(target_os = "linux")]
const PLUGIN_EXTENSION: &str = "so";
#[cfg(target_os = "macos")]
const PLUGIN_EXTENSION: &str = "dylib";

/// Default host crate name (the gatherers example).
/// Override per-project with `UNREAL_RUST_HOST_NAME` (e.g. `vivarium_rust_host`).
const DEFAULT_HOST_CRATE: &str = "unreal_rust_host";

/// Resolve the dylib filename for the configured host crate.
///
/// Cargo emits host-crate dashes as underscores in the dylib filename
/// (`vivarium-rust-host` → `libvivarium_rust_host.dylib`), so the env var
/// is expected in underscore form — matching the crate name Cargo uses.
fn plugin_lib_name() -> String {
    let crate_name = std::env::var("UNREAL_RUST_HOST_NAME")
        .unwrap_or_else(|_| DEFAULT_HOST_CRATE.to_string());
    #[cfg(target_os = "windows")]
    { format!("{}.{}", crate_name, PLUGIN_EXTENSION) }
    #[cfg(not(target_os = "windows"))]
    { format!("lib{}.{}", crate_name, PLUGIN_EXTENSION) }
}

/// Find the loader dylib's own filesystem path at runtime using platform APIs.
/// This lets us locate other files relative to the UE project Binaries directory.
fn find_own_dylib_path() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        unsafe extern "C" {
            fn dladdr(addr: *const u8, info: *mut libc::Dl_info) -> libc::c_int;
        }
        let mut info: libc::Dl_info = unsafe { std::mem::zeroed() };
        let result =
            unsafe { dladdr(find_own_dylib_path as *const u8, &mut info) };
        if result != 0 && !info.dli_fname.is_null() {
            let path = unsafe { std::ffi::CStr::from_ptr(info.dli_fname) };
            path.to_str().ok().map(PathBuf::from)
        } else {
            None
        }
    }
    #[cfg(windows)]
    {
        None // Windows uses hardcoded paths in the original code
    }
}

/// Cargo target profile directory baked in at build time by `build.rs`.
/// This is the directory the loader dylib itself was written into —
/// cargo places every artifact from this workspace there, including the
/// sibling host dylib we want to load. Honours user-configured target dirs
/// (e.g. `~/.cargo/config.toml` pointing at a shared cache) automatically.
const BUILD_TARGET_DIR: &str = env!("UNREAL_RUST_LOADER_BUILD_TARGET_DIR");

/// Resolve the path to the host plugin library by trying candidates in order.
///
/// Search order:
/// 1. `UNREAL_RUST_TARGET_DIR` env var (explicit override, primarily for CI)
/// 2. The cargo target dir this loader was built into (baked in by build.rs)
/// 3. Workspace-relative fallback derived from loader dylib location
/// 4. `target/release/` relative to cwd (last-ditch)
///
/// Returns the first candidate whose file exists, or the highest-priority
/// candidate otherwise (so callers can report a meaningful "not found" path).
fn resolve_plugin_path() -> PathBuf {
    let lib_name = plugin_lib_name();

    // Explicit env var is an unconditional override — used in CI and when a
    // caller wants to point the loader at a specific build deliberately.
    if let Ok(dir) = std::env::var("UNREAL_RUST_TARGET_DIR") {
        return PathBuf::from(dir).join(&lib_name);
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    candidates.push(PathBuf::from(BUILD_TARGET_DIR).join(&lib_name));

    if let Some(own_path) = find_own_dylib_path()
        && let Some(binaries_dir) = own_path.parent()
    {
        let workspace_root = binaries_dir.join("../../..");
        for profile in ["release", "debug", "development"] {
            candidates.push(workspace_root.join("target").join(profile).join(&lib_name));
        }
    }

    candidates.push(PathBuf::from("target/release").join(&lib_name));

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.clone();
        }
    }
    candidates.into_iter().next().unwrap_or_else(|| PathBuf::from(&lib_name))
}

pub struct Plugin {
    #[allow(dead_code)] // Kept to prevent library from being unloaded
    library: Library,
    #[allow(dead_code)] // Stored for potential re-registration
    register_unreal_bindings: RegisterUnrealBindings,
    timestamp: SystemTime,
}
impl Plugin {
    pub fn new(
        unreal_bindings: &UnrealBindings,
        rust_bindings: &mut RustBindings,
        path: &Path,
    ) -> Result<Self, String> {
        let library = unsafe {
            Library::new(path).map_err(|e| format!("Failed to load {:?}: {}", path, e))?
        };

        let metadata = std::fs::metadata(path)
            .map_err(|e| format!("Failed to read metadata for {:?}: {}", path, e))?;

        let register_unreal_bindings: RegisterUnrealBindings = unsafe {
            *library
                .get::<RegisterUnrealBindings>("register_unreal_bindings\0")
                .map_err(|e| format!("Failed to find register_unreal_bindings in {:?}: {}", path, e))?
        };

        (register_unreal_bindings)(unreal_bindings.clone(), rust_bindings);

        Ok(Plugin {
            library,
            register_unreal_bindings,
            timestamp: metadata.modified().unwrap(),
        })
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

    pub fn load(&mut self, rust_bindings: &mut RustBindings) -> Result<(), String> {
        // Unload the currently loaded plugin
        self.loaded_plugin = None;

        if !self.path.exists() {
            let host_name = std::env::var("UNREAL_RUST_HOST_NAME")
                .unwrap_or_else(|_| DEFAULT_HOST_CRATE.to_string());
            let crate_name = host_name.replace('_', "-");
            let msg = format!(
                "Plugin not found at {:?}. Build with: cargo build --release -p {}. \
                 If using a shared cargo target dir, set UNREAL_RUST_TARGET_DIR or rebuild the loader. \
                 To select a different host crate, set UNREAL_RUST_HOST_NAME.",
                self.path, crate_name
            );
            eprintln!("[unreal-rust] {}", msg);
            return Err(msg);
        }

        let root_dir = self
            .path
            .parent()
            .map(|p| p.join("rust"))
            .unwrap_or_else(|| PathBuf::from("Binaries/rust"));

        for entry in root_dir.read_dir().into_iter().flatten().flatten() {
            if entry.path().is_dir()
                && let Some(file_name) = entry.file_name().to_str()
                && file_name.starts_with("rust-plugin")
            {
                // Try to delete every hot reload folder. Some we can't delete because the debugger
                // might keep them open
                let _ = fs::remove_dir_all(entry.path());
            }
        }

        let _ = fs::create_dir_all(&root_dir);

        let hot_reload_dir = root_dir.join(format!("rust-plugin-{}", self.hotreload_id));
        if hot_reload_dir.exists() {
            let _ = fs::remove_dir_all(&hot_reload_dir);
        }
        let hotreload_lib_name = format!("rust_plugin.{}", PLUGIN_EXTENSION);
        let hotreload_lib_path = hot_reload_dir.join(&hotreload_lib_name);
        let _ = fs::create_dir_all(&hot_reload_dir);

        if let Err(e) = fs::copy(&self.path, &hotreload_lib_path) {
            let msg = format!(
                "Failed to copy {:?} to {:?}: {}",
                self.path, hotreload_lib_path, e
            );
            eprintln!("[unreal-rust] {}", msg);
            return Err(msg);
        }

        // On macOS, copied dylibs lose their code signature and the OS will
        // kill the process with SIGKILL (Code Signature Invalid). Re-sign
        // with an ad-hoc signature so the hot-reloaded copy can be loaded.
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("codesign")
                .args(["--force", "--sign", "-"])
                .arg(&hotreload_lib_path)
                .output();
        }

        // Copy debug symbols on platforms that use sidecar files
        #[cfg(target_os = "windows")]
        if let Some(pdb_dir) = self.path.parent() {
            let pdb_stem = self
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(DEFAULT_HOST_CRATE);
            let pdb_file = format!("{}.pdb", pdb_stem);
            let pdb_path = pdb_dir.join(&pdb_file);
            if pdb_path.exists() {
                let _ = fs::copy(&pdb_path, hot_reload_dir.join(&pdb_file));
            }
        }

        match Plugin::new(&self.unreal_bindings, rust_bindings, &hotreload_lib_path) {
            Ok(plugin) => {
                eprintln!("[unreal-rust] Loaded plugin from {:?}", self.path);
                self.loaded_plugin = Some(plugin);
                self.hotreload_id += 1;
                Ok(())
            }
            Err(e) => {
                eprintln!("[unreal-rust] {}", e);
                Err(e)
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn register_unreal_bindings(bindings: UnrealBindings) -> u32 {
    unsafe {
        if LOADER.is_null() {
            let plugin_path = resolve_plugin_path();
            eprintln!(
                "[unreal-rust] Resolved plugin path: {:?} (exists: {}, build-time target dir: {})",
                plugin_path,
                plugin_path.exists(),
                BUILD_TARGET_DIR,
            );
            LOADER = Box::leak(Box::new(Loader::new(bindings, plugin_path))) as *mut _;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    // These tests mutate process-global env vars (UNREAL_RUST_HOST_NAME,
    // UNREAL_RUST_TARGET_DIR). Cargo runs tests in parallel by default, so
    // without serialization they race against each other. The mutex is the
    // standard workaround — every test in this module takes it before
    // touching env vars.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn plugin_lib_name_default_has_correct_extension() {
        let _g = env_guard();
        unsafe { std::env::remove_var("UNREAL_RUST_HOST_NAME") };
        let name = plugin_lib_name();
        assert!(
            name.ends_with(PLUGIN_EXTENSION),
            "plugin_lib_name() '{}' should end with PLUGIN_EXTENSION '{}'",
            name,
            PLUGIN_EXTENSION,
        );
        assert!(
            name.contains(DEFAULT_HOST_CRATE),
            "default lib name '{}' should contain DEFAULT_HOST_CRATE '{}'",
            name,
            DEFAULT_HOST_CRATE,
        );
    }

    #[test]
    fn plugin_lib_name_respects_host_env_var() {
        let _g = env_guard();
        unsafe { std::env::set_var("UNREAL_RUST_HOST_NAME", "vivarium_rust_host") };
        let name = plugin_lib_name();
        unsafe { std::env::remove_var("UNREAL_RUST_HOST_NAME") };
        assert!(
            name.contains("vivarium_rust_host"),
            "overridden lib name '{}' should contain 'vivarium_rust_host'",
            name,
        );
        assert!(name.ends_with(PLUGIN_EXTENSION));
    }

    #[test]
    fn resolve_plugin_path_with_env_var() {
        let _g = env_guard();
        let dir = "/tmp/test-unreal-rust-target";
        unsafe { std::env::set_var("UNREAL_RUST_TARGET_DIR", dir) };
        unsafe { std::env::remove_var("UNREAL_RUST_HOST_NAME") };
        let path = resolve_plugin_path();
        let expected_name = plugin_lib_name();
        unsafe { std::env::remove_var("UNREAL_RUST_TARGET_DIR") };
        assert_eq!(path, PathBuf::from(dir).join(expected_name));
    }

    #[test]
    fn resolve_plugin_path_fallback() {
        let _g = env_guard();
        unsafe { std::env::remove_var("UNREAL_RUST_TARGET_DIR") };
        unsafe { std::env::remove_var("UNREAL_RUST_HOST_NAME") };
        let path = resolve_plugin_path();
        let expected = plugin_lib_name();
        assert!(
            path.file_name().unwrap() == std::ffi::OsStr::new(&expected),
            "resolved path {:?} should end with {}",
            path,
            expected,
        );
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn is_out_of_date() -> u32 {
    unsafe {
        if !LOADER.is_null() {
            (*LOADER).is_out_of_date() as u32
        } else {
            0
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn try_load(rust_bindings: *mut RustBindings) -> u32 {
    unsafe {
        if LOADER.is_null() {
            return 0;
        }
        match (*LOADER).load(&mut *rust_bindings) {
            Ok(()) => 1,
            Err(_) => 0,
        }
    }
}
