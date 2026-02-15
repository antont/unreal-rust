use std::{ffi::c_void, os::raw::c_char};

#[repr(u8)]
#[derive(Debug)]
pub enum ResultCode {
    Success = 0,
    Panic = 1,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UObjectType {
    UClass,
    USceneComponent,
    UPrimtiveComponent,
}

#[repr(C)]
pub struct Utf8Str {
    pub ptr: *const c_char,
    pub len: usize,
}
impl Utf8Str {
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8(std::slice::from_raw_parts(self.ptr as _, self.len)).unwrap() }
    }
}

impl<'a> From<&'a str> for Utf8Str {
    fn from(s: &'a str) -> Self {
        Self {
            ptr: s.as_ptr() as *const c_char,
            len: s.len(),
        }
    }
}

// TODO: Is there a more typesafe way of defining an opaque type that
// is c ffi safe in Rust without nightly?
pub type UClassOpague = c_void;
pub type UFunctionOpague = c_void;
pub type UObjectOpague = c_void;
pub type FScriptArrayOpaque = c_void;

pub type LogFn = unsafe extern "C" fn(message: Utf8Str);

pub type IsAFn = unsafe extern "C" fn(object: *mut UObjectOpague, ty: UObjectType) -> u32;

unsafe extern "C" {
    pub fn Log(message: Utf8Str);
}

#[repr(C)]
#[derive(Clone)]
pub struct UnrealBindings {
    pub log: LogFn,
    pub core_fns: CoreFns,
    pub fscript_array_fns: FScriptArrayFns,
}

unsafe impl Sync for UnrealBindings {}
unsafe impl Send for UnrealBindings {}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Uuid {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

pub type InitializeUnrealApiFn = extern "C" fn();

pub type RegisterUnrealBindings =
    extern "C" fn(bindings: UnrealBindings, rust_bindings: *mut RustBindings) -> u32;

pub type EntryUnrealBindingsFn =
    unsafe extern "C" fn(bindings: UnrealBindings, rust_bindings: *mut RustBindings) -> u32;

pub type BeginPlayFn = unsafe extern "C" fn() -> ResultCode;
pub type TickFn = unsafe extern "C" fn(dt: f32) -> ResultCode;
pub type TryLoadFn = unsafe extern "C" fn(*mut RustBindings) -> u32;

#[repr(C)]
pub struct PluginBindings {
    pub tick: TickFn,
    pub begin_play: BeginPlayFn,
    pub try_load: TryLoadFn,
}

#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {
    pub tick: TickFn,
    pub begin_play: BeginPlayFn,
    pub allocate: AllocateFn,
}

impl RustBindings {
    pub fn uninit() -> Self {
        unsafe extern "C" fn tick_stub(_: f32) -> ResultCode {
            ResultCode::Panic
        }
        unsafe extern "C" fn begin_play_stub() -> ResultCode {
            ResultCode::Panic
        }

        unsafe extern "C" fn allocate_stub(_: usize, _: usize, _: *mut RustAlloc) -> u32 {
            0
        }

        Self {
            tick: tick_stub,
            begin_play: begin_play_stub,
            allocate: allocate_stub,
        }
    }
}

pub type InitializeModulesFn = unsafe extern "C" fn();

#[repr(u32)]
pub enum ReflectionType {
    Float,
    Vector3,
    Bool,
    Quaternion,
    UClass,
    USound,
    Composite,
}
#[repr(C)]
pub struct StrRustAlloc {
    pub alloc: RustAlloc,
}
impl StrRustAlloc {
    pub fn empty() -> Self {
        Self {
            alloc: RustAlloc::empty(),
        }
    }
    pub fn into_string(self) -> String {
        unsafe {
            let name = {
                let slice = std::slice::from_raw_parts(self.alloc.ptr, self.alloc.size);
                let name = std::str::from_utf8(slice).unwrap();
                name.to_string()
            };
            self.alloc.free();
            name
        }
    }
}
#[repr(C)]
pub struct RustAlloc {
    pub ptr: *mut u8,
    pub size: usize,
    pub align: usize,
}

impl RustAlloc {
    pub fn empty() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
            align: 0,
        }
    }
    /// # Safety
    /// Must have a valid allocation from within unreal c++
    /// Only free if the ptr is not already in use
    /// Ptr must be valid, and allocated from the same allocator
    pub unsafe fn free(self) {
        unsafe {
            if self.size == 0 || self.ptr.is_null() {
                return;
            }
            std::alloc::dealloc(
                self.ptr,
                std::alloc::Layout::from_size_align(self.size, self.align).unwrap(),
            );
        }
    }
}

pub type AllocateFn = unsafe extern "C" fn(size: usize, align: usize, ptr: *mut RustAlloc) -> u32;

#[repr(C)]
#[derive(Clone)]
pub struct AllocateFns {
    pub allocate: AllocateFn,
}

pub type GetCDOFromClassCoreFn =
    unsafe extern "C" fn(cdo_opague: *const UClassOpague, *mut *mut UObjectOpague) -> u32;
pub type GetAllUClassesCoreFn = unsafe extern "C" fn(out: *mut RustAlloc) -> u32;
pub type GetClassNameCoreFn =
    unsafe extern "C" fn(opague_class: *const UClassOpague, out: *mut StrRustAlloc) -> u32;
pub type FindFunctionByNameCoreFn = unsafe extern "C" fn(
    class_opague: *const UClassOpague,
    str: Utf8Str,
    function_opague: *mut *mut UFunctionOpague,
) -> u32;
pub type InitializeValuesInParamBufferCoreFn =
    unsafe extern "C" fn(function_opague: *const UFunctionOpague, buffer: *mut c_void) -> u32;
pub type DestroyValuesInParamBufferCoreFn =
    unsafe extern "C" fn(function_opague: *const UFunctionOpague, buffer: *mut c_void) -> u32;
pub type ProcessEventsCoreFn = unsafe extern "C" fn(
    object_opague: *mut UObjectOpague,
    function_opague: *mut UFunctionOpague,
    buffer: *mut c_void,
) -> u32;
pub type BeginTraceCoreFn = unsafe extern "C" fn(name: *const c_char);
pub type EndTraceCoreFn = unsafe extern "C" fn();

pub type FScriptArrayNumFn =
    unsafe extern "C" fn(array: *const FScriptArrayOpaque, out_num: *mut i32) -> u32;
pub type FScriptArrayMaxFn =
    unsafe extern "C" fn(array: *const FScriptArrayOpaque, out_max: *mut i32) -> u32;
pub type FScriptArrayGetDataFn =
    unsafe extern "C" fn(array: *mut FScriptArrayOpaque, out_data: *mut *mut c_void) -> u32;
pub type FScriptArrayIsValidIndexFn =
    unsafe extern "C" fn(array: *const FScriptArrayOpaque, index: i32) -> u32;

pub type FScriptArrayReserveFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    capacity: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayAddFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    count: i32,
    elem_size: i32,
    elem_align: u32,
    out_index: *mut i32,
) -> u32;
pub type FScriptArrayInsertFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    index: i32,
    count: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayRemoveFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    index: i32,
    count: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayEmptyFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    slack: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayResetFn = unsafe extern "C" fn(
    array: *mut FScriptArrayOpaque,
    new_size: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayShrinkFn =
    unsafe extern "C" fn(array: *mut FScriptArrayOpaque, elem_size: i32, elem_align: u32) -> u32;
//
unsafe extern "C" {
    pub fn GetCDOFromClass(
        class_opague: *const UClassOpague,
        out_object: *mut *mut UObjectOpague,
    ) -> u32;
    pub fn GetAllUClasses(out: *mut RustAlloc) -> u32;
    pub fn GetClassName(opague_class: *const UClassOpague, out: *mut StrRustAlloc) -> u32;
    pub fn FindFunctionByName(
        class_opague: *const UClassOpague,
        name: Utf8Str,
        function_opague: *mut *mut UFunctionOpague,
    ) -> u32;
    pub fn InitializeValuesInParamBuffer(
        function_opague: *const UFunctionOpague,
        buffer: *mut c_void,
    ) -> u32;
    pub fn DestroyValuesInParamBuffer(
        function_opague: *const UFunctionOpague,
        buffer: *mut c_void,
    ) -> u32;
    pub fn ProcessEventFromRust(
        cdo_opague: *mut UObjectOpague,
        function_opague: *mut UFunctionOpague,
        buffer: *mut c_void,
    ) -> u32;
    pub fn BeginTrace(name: *const c_char);
    pub fn EndTrace();

    pub fn FScriptArrayNum(array: *const FScriptArrayOpaque, out_num: *mut i32) -> u32;
    pub fn FScriptArrayMax(array: *const FScriptArrayOpaque, out_max: *mut i32) -> u32;
    pub fn FScriptArrayGetData(array: *mut FScriptArrayOpaque, out_data: *mut *mut c_void) -> u32;
    pub fn FScriptArrayIsValidIndex(array: *const FScriptArrayOpaque, index: i32) -> u32;
    pub fn FScriptArrayReserve(
        array: *mut FScriptArrayOpaque,
        capacity: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayAdd(
        array: *mut FScriptArrayOpaque,
        count: i32,
        elem_size: i32,
        elem_align: u32,
        out_index: *mut i32,
    ) -> u32;
    pub fn FScriptArrayInsert(
        array: *mut FScriptArrayOpaque,
        index: i32,
        count: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayRemove(
        array: *mut FScriptArrayOpaque,
        index: i32,
        count: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayEmpty(
        array: *mut FScriptArrayOpaque,
        slack: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayReset(
        array: *mut FScriptArrayOpaque,
        new_size: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayShrink(
        array: *mut FScriptArrayOpaque,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
}

#[repr(C)]
#[derive(Clone)]
pub struct CoreFns {
    pub get_cdo_from_class: GetCDOFromClassCoreFn,
    pub get_all_uclasses: GetAllUClassesCoreFn,
    pub get_class_name: GetClassNameCoreFn,
    pub find_function_by_name: FindFunctionByNameCoreFn,
    pub initialize_values_in_param_buffer: InitializeValuesInParamBufferCoreFn,
    pub destroy_values_in_param_buffer: DestroyValuesInParamBufferCoreFn,
    pub process_event: ProcessEventsCoreFn,
    pub begin_trace: BeginTraceCoreFn,
    pub end_trace: EndTraceCoreFn,
}

#[repr(C)]
#[derive(Clone)]
pub struct FScriptArrayFns {
    pub num: FScriptArrayNumFn,
    pub max: FScriptArrayMaxFn,
    pub get_data: FScriptArrayGetDataFn,
    pub is_valid_index: FScriptArrayIsValidIndexFn,
    pub reserve: FScriptArrayReserveFn,
    pub add: FScriptArrayAddFn,
    pub insert: FScriptArrayInsertFn,
    pub remove: FScriptArrayRemoveFn,
    pub empty: FScriptArrayEmptyFn,
    pub reset: FScriptArrayResetFn,
    pub shrink: FScriptArrayShrinkFn,
}
