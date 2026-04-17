use std::panic::catch_unwind;

use log::{LevelFilter, Metadata, Record, SetLoggerError, set_boxed_logger, set_max_level};
pub use unreal_api::ffi;

mod mass_system_registry;
use unreal_api::{
    bindings::globals::{self, ClassPtrDB},
    ffi::{ResultCode, RustBindings},
    module::bindings,
};

struct UnrealLogger;

impl log::Log for UnrealLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                log::Level::Error => 1u8,
                log::Level::Warn => 2,
                log::Level::Info => 3,
                log::Level::Debug => 4,
                log::Level::Trace => 5,
            };
            if let Some(text) = record.args().as_str() {
                unsafe {
                    (bindings().log)(ffi::Utf8Str::from(text), level);
                }
            } else {
                unsafe {
                    (bindings().log)(ffi::Utf8Str::from(record.args().to_string().as_str()), level);
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init_log() -> Result<(), SetLoggerError> {
    set_boxed_logger(Box::new(UnrealLogger)).map(|()| set_max_level(LevelFilter::Info))
}

pub struct Module {
    pub user_module: Box<dyn UserModule>,
}

pub trait UserModule {
    fn initialize(&mut self);
    fn tick(&mut self, dt: f32);
    fn begin_play(&mut self);
}

fn set_custom_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        debug_break();
        let bt: std::backtrace::Backtrace = std::backtrace::Backtrace::force_capture();
        log::error!("{}", bt);
        let info = panic_info
            .payload()
            .downcast_ref::<&'static str>()
            .copied()
            .or(panic_info
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str));

        if let Some(s) = info {
            let location = panic_info.location().map_or("".to_string(), |loc| {
                format!("{}, at line {}", loc.file(), loc.line())
            });
            log::error!("Panic: {} => {}", location, s);
        } else {
            log::error!("panic occurred");
        }
    }));
}

fn create_rust_bindings() -> RustBindings {
    unsafe extern "C" fn mass_bob_process_noop(
        _: *mut std::ffi::c_void,
        _: i32,
        _: f32,
    ) {
    }

    RustBindings {
        tick,
        begin_play,
        allocate,
        mass_bob_process: mass_bob_process_noop,
        get_mass_system_count: mass_system_registry::get_mass_system_count,
        get_mass_system_descriptor: mass_system_registry::get_mass_system_descriptor,
        mass_frame_dispatch: mass_system_registry::mass_frame_dispatch,
        get_visualizer_group_count: Some(mass_system_registry::get_visualizer_group_count),
        get_visualizer_group_desc: Some(mass_system_registry::get_visualizer_group_desc),
        mass_init_simulation: Some(mass_system_registry::mass_init_simulation),
        get_spatial_query_config_count: Some(mass_system_registry::get_spatial_query_config_count),
        get_spatial_query_config_desc: Some(mass_system_registry::get_spatial_query_config_desc),
        get_sim_defaults: Some(mass_system_registry::get_sim_defaults),
        get_mass_test_count: Some(mass_system_registry::get_mass_test_count),
        get_mass_test_desc: Some(mass_system_registry::get_mass_test_desc),
        run_mass_test: Some(mass_system_registry::run_mass_test),
    }
}
fn debug_break() {
    #[cfg(windows)]
    unsafe {
        if windows_sys::Win32::System::Diagnostics::Debug::IsDebuggerPresent() != 0 {
            windows_sys::Win32::System::Diagnostics::Debug::DebugBreak();
        }
    }
}

unsafe extern "C" fn tick(dt: f32) -> ffi::ResultCode {
    let r = catch_unwind(|| unsafe {
        if !MODULE.is_null() {
            (*MODULE).user_module.tick(dt);
        }
    });

    match r {
        Ok(_) => ResultCode::Success,
        Err(_) => ResultCode::Panic,
    }
}

unsafe extern "C" fn begin_play() -> ffi::ResultCode {
    let r = catch_unwind(|| unsafe {
        if !MODULE.is_null() {
            (*MODULE).user_module.begin_play();
        }
    });

    match r {
        Ok(_) => ResultCode::Success,
        Err(_) => ResultCode::Panic,
    }
}

unsafe extern "C" fn allocate(size: usize, align: usize, ptr: *mut ffi::RustAlloc) -> u32 {
    unsafe {
        use std::alloc::{Layout, alloc};
        let layout = Layout::from_size_align(size, align);
        match layout {
            Ok(layout) => {
                let alloc_ptr = alloc(layout);
                *ptr = ffi::RustAlloc {
                    ptr: alloc_ptr,
                    size,
                    align,
                };

                1
            }
            Err(_) => 0,
        }
    }
}

pub static mut MODULE: *mut Module = std::ptr::null_mut();

/// # Safety
pub unsafe fn initialize_module(
    unreal_bindings: ffi::UnrealBindings,
    rust_bindings: *mut RustBindings,
    user_module: Box<dyn UserModule>,
) {
    set_custom_panic_hook();
    let _ = init_log();
    {
        let _ = unreal_api::module::BINDINGS.set(unreal_bindings);
    }
    unsafe {
        *rust_bindings = create_rust_bindings();
    }

    unsafe {
        MODULE = Box::leak(Box::new(Module { user_module })) as *mut _;
    }

    {
        let class_db = ClassPtrDB::from(bindings());
        let _ = globals::CLASS_PTRS.set(class_db);
    }

    unreal_api::bindings::globals::initialize_modules();

    // Clear stale descriptor cache from previous load (hot-reload)
    mass_system_registry::reset_descriptor_cache();

    // Build the Bevy schedule from all registered mass systems
    mass_system_registry::init_global_schedule();

    unsafe {
        (*MODULE).user_module.initialize();
    }
}

#[macro_export]
macro_rules! implement_unreal_module {
    ($module: expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn register_unreal_bindings(
            bindings: $crate::ffi::UnrealBindings,
            rust_bindings: *mut $crate::ffi::RustBindings,
        ) -> u32 {
            $crate::initialize_module(bindings, rust_bindings, Box::new($module));
            1
        }
    };
    ($module: expr, mass_bob_process: $mass_bob:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn register_unreal_bindings(
            bindings: $crate::ffi::UnrealBindings,
            rust_bindings: *mut $crate::ffi::RustBindings,
        ) -> u32 {
            $crate::initialize_module(bindings, rust_bindings, Box::new($module));
            unsafe { (*rust_bindings).mass_bob_process = $mass_bob; }
            1
        }
    };
}
