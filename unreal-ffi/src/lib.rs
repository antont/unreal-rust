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
pub struct FRustString {
    pub data: *mut u16,
    pub num: i32,
    pub max: i32,
}

impl FRustString {
    pub fn uninit() -> Self {
        FRustString {
            data: std::ptr::null_mut(),
            num: 0,
            max: 0,
        }
    }
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FRustScriptArray {
    pub opaque_words: [u64; 2],
}

impl FRustScriptArray {
    pub const fn uninit() -> Self {
        Self {
            opaque_words: [0; 2],
        }
    }
}

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
    pub fstring_fns: FStringFns,
    pub fscript_array_fns: FScriptArrayFns,
    /// Spawn entities in Mass Entity. Null until a Mass subsystem sets it.
    pub spawn_entities: Option<SpawnEntitiesFn>,
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

pub type EntryUnrealBindingsFn = unsafe extern "C" fn(bindings: UnrealBindings) -> u32;

pub type BeginPlayFn = unsafe extern "C" fn() -> ResultCode;
pub type TickFn = unsafe extern "C" fn(dt: f32) -> ResultCode;
pub type MassBobProcessFn = unsafe extern "C" fn(data: *mut std::ffi::c_void, count: i32, dt: f32);
// --- Dynamic Mass System Registration ---

/// Data for one fragment slice in a chunk, passed from C++ ForEachEntityChunk to Rust.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MassFragmentSlice {
    /// Raw pointer to contiguous fragment data in the chunk.
    pub data: *mut std::ffi::c_void,
    /// Number of entities in this chunk.
    pub count: i32,
    /// Size of each fragment element in bytes.
    pub stride: u32,
}

/// One chunk's slice of data for a global query — points directly into Mass Entity chunk memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MassGlobalChunkSlice {
    /// Direct pointer into chunk fragment memory (zero-copy).
    pub data: *mut std::ffi::c_void,
    /// Number of entities in this chunk.
    pub count: i32,
    /// Size of each fragment element in bytes.
    pub stride: u32,
}

/// All chunk slices for one global fragment type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MassGlobalFragmentChunks {
    /// Pointer to array of MassGlobalChunkSlice, one per chunk.
    pub chunks: *const MassGlobalChunkSlice,
    /// Number of chunks.
    pub num_chunks: u32,
    /// Total entity count across all chunks.
    pub total_count: i32,
    /// Stride (same for all chunks — same fragment type).
    pub stride: u32,
}

/// Chunk data passed from C++ Execute to a Rust mass system function.
#[repr(C)]
pub struct MassChunkData {
    /// Number of entities in this chunk.
    pub num_entities: i32,
    /// Delta time for this simulation step.
    pub dt: f32,
    /// Number of primary fragment slices (matches system's primary requirement count).
    pub num_fragments: u32,
    /// Pointer to array of MassFragmentSlice, one per primary requirement in declaration order.
    pub fragments: *const MassFragmentSlice,
    /// Number of entities across all chunks for global queries.
    pub global_num_entities: i32,
    /// Number of global fragment slices.
    pub num_global_fragments: u32,
    /// Deprecated: was contiguous-copy global fragments. Set to null for zero-copy path.
    pub global_fragments: *const MassFragmentSlice,
    /// Deprecated: was entity handle pairs. Set to null for index-based access.
    pub global_entity_handles: *const i32,
    /// Zero-copy chunked global fragments: array of MassGlobalFragmentChunks, one per global fragment type.
    pub global_chunked_fragments: *const MassGlobalFragmentChunks,
}

/// Describes one fragment/tag requirement for a Rust mass system.
#[repr(C)]
pub struct MassFragmentRequirement {
    /// C++ USTRUCT type name (e.g. "FGatherersMassAntFragment").
    pub cpp_type_name: Utf8Str,
    /// Size of the fragment in bytes.
    pub size: u32,
    /// Alignment of the fragment.
    pub alignment: u32,
    /// Access mode: 0 = ReadOnly, 1 = ReadWrite.
    pub access_mode: u8,
    /// Whether this is a tag (1) or fragment (0).
    pub is_tag: u8,
    /// Query scope: 0 = primary (per-chunk), 1 = global (all matching entities).
    pub query_scope: u8,
    pub _padding: u8,
}

/// Execute function signature for a dynamically registered mass system.
pub type MassSystemExecuteFn = unsafe extern "C" fn(chunk: *const MassChunkData);

// --- Bevy-scheduled per-frame dispatch ---

/// One system's cached primary chunk data, passed from C++ coordinator to Rust.
#[repr(C)]
pub struct MassSystemChunkBatch {
    /// Index of the system in registration order.
    pub system_index: u32,
    /// Number of primary chunks for this system.
    pub num_primary_chunks: u32,
    /// Pointer to array of MassChunkData, one per primary chunk.
    pub primary_chunks: *const MassChunkData,
}

/// Per-frame dispatch data: dt + all system chunk batches + spatial query callback.
#[repr(C)]
pub struct MassFrameDispatchData {
    /// Delta time for this frame.
    pub dt: f32,
    /// Number of system batches.
    pub num_systems: u32,
    /// Pointer to array of MassSystemChunkBatch.
    pub systems: *const MassSystemChunkBatch,
    /// Optional spatial query callback for collision detection. Null if not available.
    pub spatial_query_fn: Option<MassSpatialQueryFn>,
    /// Pickup radius for spatial queries (Unreal units).
    pub pickup_radius: f32,
    pub _pad: u32,
}

/// Function signature for per-frame Bevy-scheduled dispatch.
pub type MassFrameDispatchFn = unsafe extern "C" fn(data: *const MassFrameDispatchData);

// --- Spatial query callback for Rust collision processor ---

/// Result of a spatial query for one ant (food encounter detection).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MassSpatialQueryResult {
    /// Index of the nearest food entity, or -1 if none found.
    pub food_index: i32,
    pub _pad: i32,
    /// Position where the encounter occurred.
    pub encounter_position: [f64; 3],
    /// Whether a food encounter was found.
    pub has_encounter: bool,
    pub _result_pad: [u8; 7],
}

/// Callback function for spatial queries. C++ implements this using UE physics.
///
/// Parameters:
/// - `previous_pos`: ant's previous position (3 x f64)
/// - `current_pos`: ant's current position (3 x f64)
/// - `pickup_radius`: radius for collision detection
/// - `out`: result written here
///
/// Returns 1 on success, 0 on failure.
pub type MassSpatialQueryFn = unsafe extern "C" fn(
    previous_pos: *const f64,
    current_pos: *const f64,
    pickup_radius: f32,
    out: *mut MassSpatialQueryResult,
) -> u32;

// --- Entity spawning ---

/// Describes one fragment type's initial data for a batch spawn request.
#[repr(C)]
pub struct SpawnFragmentData {
    /// C++ USTRUCT type name (e.g. "FGatherersPosition").
    pub cpp_type_name: Utf8Str,
    /// Size of one fragment instance in bytes.
    pub size: u32,
    /// Alignment of the fragment type.
    pub alignment: u32,
    /// Pointer to `count` contiguous fragment values (size * count bytes).
    /// Null = use C++ default constructor for all entities.
    pub initial_data: *const c_void,
}

/// Describes one tag type to apply to spawned entities.
#[repr(C)]
pub struct SpawnTagData {
    /// C++ USTRUCT type name (e.g. "FGatherersBevyMassAntTag").
    pub cpp_type_name: Utf8Str,
}

/// A batch spawn request: create `count` entities with the given fragments and tags.
#[repr(C)]
pub struct SpawnEntityRequest {
    /// Number of fragment types.
    pub num_fragments: u32,
    /// Pointer to array of SpawnFragmentData (one per fragment type).
    pub fragments: *const SpawnFragmentData,
    /// Number of tags.
    pub num_tags: u32,
    /// Pointer to array of SpawnTagData.
    pub tags: *const SpawnTagData,
    /// Number of entities to spawn.
    pub count: u32,
    pub _padding: u32,
}

/// FFI-safe entity handle matching FMassEntityHandle layout.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct MassEntityHandle {
    /// Entity index in the entity manager.
    pub index: i32,
    /// Serial number for handle validation.
    pub serial_number: i32,
}

/// Spawns `request.count` entities. Writes handles to `out_handles` (must have room for `count`).
/// Returns the number of entities actually created (should equal `count` on success).
pub type SpawnEntitiesFn = unsafe extern "C" fn(
    request: *const SpawnEntityRequest,
    out_handles: *mut MassEntityHandle,
) -> u32;

// Compile-time check: Option<fn ptr> must be pointer-sized for C FFI compatibility.
// Rust guarantees niche optimization for Option<extern "C" fn>, but assert to catch
// any future type changes that would break the C++ nullable function pointer layout.
const _: () = assert!(
    std::mem::size_of::<Option<MassSpatialQueryFn>>() == std::mem::size_of::<*const ()>(),
    "Option<MassSpatialQueryFn> must be pointer-sized for FFI"
);

/// Describes one registered Rust mass system. C++ queries these at init time.
#[repr(C)]
pub struct MassSystemDescriptor {
    /// Unique system name (e.g. "ant_movement").
    pub name: Utf8Str,
    /// Number of fragment/tag requirements.
    pub num_requirements: u32,
    /// Execution order within a pipeline. Lower values run first.
    pub order: u32,
    /// Pointer to array of MassFragmentRequirement.
    pub requirements: *const MassFragmentRequirement,
    /// The Rust function to call during Execute.
    pub execute_fn: MassSystemExecuteFn,
}

/// Returns the number of dynamically registered mass systems.
pub type GetMassSystemCountFn = unsafe extern "C" fn() -> u32;

/// Fills a MassSystemDescriptor for the system at `index`. Returns 1 on success, 0 on failure.
pub type GetMassSystemDescriptorFn =
    unsafe extern "C" fn(index: u32, out: *mut MassSystemDescriptor) -> u32;

pub type TryLoadFn = unsafe extern "C" fn(*mut RustBindings) -> u32;
pub type IsOutOfDateFn = unsafe extern "C" fn() -> u32;

#[repr(C)]
pub struct PluginBindings {
    pub tick: TickFn,
    pub begin_play: BeginPlayFn,
    pub try_load: TryLoadFn,
    pub is_out_of_date: IsOutOfDateFn,
}

#[repr(C)]
#[derive(Clone)]
pub struct RustBindings {
    pub tick: TickFn,
    pub begin_play: BeginPlayFn,
    pub allocate: AllocateFn,
    pub mass_bob_process: MassBobProcessFn,
    pub get_mass_system_count: GetMassSystemCountFn,
    pub get_mass_system_descriptor: GetMassSystemDescriptorFn,
    pub mass_frame_dispatch: MassFrameDispatchFn,
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

        unsafe extern "C" fn mass_bob_process_stub(
            _: *mut std::ffi::c_void,
            _: i32,
            _: f32,
        ) {
        }

        unsafe extern "C" fn get_mass_system_count_stub() -> u32 {
            0
        }

        unsafe extern "C" fn get_mass_system_descriptor_stub(
            _: u32,
            _: *mut MassSystemDescriptor,
        ) -> u32 {
            0
        }

        unsafe extern "C" fn mass_frame_dispatch_stub(
            _: *const MassFrameDispatchData,
        ) {
        }

        Self {
            tick: tick_stub,
            begin_play: begin_play_stub,
            allocate: allocate_stub,
            mass_bob_process: mass_bob_process_stub,
            get_mass_system_count: get_mass_system_count_stub,
            get_mass_system_descriptor: get_mass_system_descriptor_stub,
            mass_frame_dispatch: mass_frame_dispatch_stub,
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
    unsafe extern "C" fn(array: *const FRustScriptArray, out_num: *mut i32) -> u32;
pub type FScriptArrayCtorFn = unsafe extern "C" fn(array: *mut FRustScriptArray) -> u32;
pub type FScriptArrayDtorFn = unsafe extern "C" fn(array: *mut FRustScriptArray) -> u32;
pub type FScriptArrayMaxFn =
    unsafe extern "C" fn(array: *const FRustScriptArray, out_max: *mut i32) -> u32;
pub type FScriptArrayGetDataFn =
    unsafe extern "C" fn(array: *mut FRustScriptArray, out_data: *mut *mut c_void) -> u32;
pub type FScriptArrayIsValidIndexFn =
    unsafe extern "C" fn(array: *const FRustScriptArray, index: i32) -> u32;

pub type FScriptArrayReserveFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    capacity: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayAddFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    count: i32,
    elem_size: i32,
    elem_align: u32,
    out_index: *mut i32,
) -> u32;
pub type FScriptArrayInsertFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    index: i32,
    count: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayRemoveFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    index: i32,
    count: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayEmptyFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    slack: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayResetFn = unsafe extern "C" fn(
    array: *mut FRustScriptArray,
    new_size: i32,
    elem_size: i32,
    elem_align: u32,
) -> u32;
pub type FScriptArrayShrinkFn =
    unsafe extern "C" fn(array: *mut FRustScriptArray, elem_size: i32, elem_align: u32) -> u32;

pub type NewFStringFromUtf8Fn = unsafe extern "C" fn(str: Utf8Str, fstring: *mut FRustString);
pub type CopyFromFStringFn =
    unsafe extern "C" fn(source: *const FRustString, fstring: *mut FRustString);
pub type FStringDtorFn = unsafe extern "C" fn(fstring: *mut FRustString);
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

    pub fn NewFStringFromUtf8(str: Utf8Str, fstring: *mut FRustString);
    pub fn CopyFromFString(source: *const FRustString, fstring: *mut FRustString);
    pub fn FStringDtor(fstring: *mut FRustString);

    pub fn FScriptArrayNum(array: *const FRustScriptArray, out_num: *mut i32) -> u32;
    pub fn FScriptArrayCtor(array: *mut FRustScriptArray) -> u32;
    pub fn FScriptArrayDtor(array: *mut FRustScriptArray) -> u32;
    pub fn FScriptArrayMax(array: *const FRustScriptArray, out_max: *mut i32) -> u32;
    pub fn FScriptArrayGetData(array: *mut FRustScriptArray, out_data: *mut *mut c_void) -> u32;
    pub fn FScriptArrayIsValidIndex(array: *const FRustScriptArray, index: i32) -> u32;
    pub fn FScriptArrayReserve(
        array: *mut FRustScriptArray,
        capacity: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayAdd(
        array: *mut FRustScriptArray,
        count: i32,
        elem_size: i32,
        elem_align: u32,
        out_index: *mut i32,
    ) -> u32;
    pub fn FScriptArrayInsert(
        array: *mut FRustScriptArray,
        index: i32,
        count: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayRemove(
        array: *mut FRustScriptArray,
        index: i32,
        count: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayEmpty(
        array: *mut FRustScriptArray,
        slack: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayReset(
        array: *mut FRustScriptArray,
        new_size: i32,
        elem_size: i32,
        elem_align: u32,
    ) -> u32;
    pub fn FScriptArrayShrink(array: *mut FRustScriptArray, elem_size: i32, elem_align: u32)
    -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8str_from_str_roundtrip() {
        let s = "hello";
        let utf8 = Utf8Str::from(s);
        assert_eq!(utf8.len, 5);
        assert!(!utf8.ptr.is_null());
        assert_eq!(utf8.as_str(), "hello");
    }

    #[test]
    fn utf8str_empty_string() {
        let utf8 = Utf8Str::from("");
        assert_eq!(utf8.len, 0);
        assert_eq!(utf8.as_str(), "");
    }

    #[test]
    fn utf8str_unicode() {
        let s = "héllo wörld 🦀";
        let utf8 = Utf8Str::from(s);
        assert_eq!(utf8.len, s.len());
        assert_eq!(utf8.as_str(), s);
    }

    #[test]
    fn frust_string_uninit_is_null() {
        let s = FRustString::uninit();
        assert!(s.data.is_null());
        assert_eq!(s.num, 0);
        assert_eq!(s.max, 0);
    }

    #[test]
    fn frust_script_array_uninit_is_zeroed() {
        let arr = FRustScriptArray::uninit();
        assert_eq!(arr.opaque_words, [0, 0]);
    }

    #[test]
    fn rust_alloc_empty_is_null() {
        let alloc = RustAlloc::empty();
        assert!(alloc.ptr.is_null());
        assert_eq!(alloc.size, 0);
        assert_eq!(alloc.align, 0);
    }

    #[test]
    fn rust_alloc_free_empty_is_safe() {
        let alloc = RustAlloc::empty();
        // Should not panic or crash
        unsafe { alloc.free() };
    }

    #[test]
    fn uuid_default_is_zeroed() {
        let uuid = Uuid::default();
        assert_eq!(uuid.a, 0);
        assert_eq!(uuid.b, 0);
        assert_eq!(uuid.c, 0);
        assert_eq!(uuid.d, 0);
    }

    #[test]
    fn rust_bindings_uninit_stubs_return_panic() {
        let bindings = RustBindings::uninit();
        let result = unsafe { (bindings.tick)(0.016) };
        assert!(matches!(result, ResultCode::Panic));
        let result = unsafe { (bindings.begin_play)() };
        assert!(matches!(result, ResultCode::Panic));
    }

    #[test]
    fn rust_bindings_has_mass_bob_process_field() {
        // RustBindings should have 7 function pointers
        let size = std::mem::size_of::<RustBindings>();
        assert_eq!(size, 7 * std::mem::size_of::<usize>());
    }

    #[test]
    fn str_rust_alloc_empty() {
        let alloc = StrRustAlloc::empty();
        assert!(alloc.alloc.ptr.is_null());
        assert_eq!(alloc.alloc.size, 0);
    }

    #[test]
    fn mass_fragment_slice_layout() {
        assert_eq!(std::mem::size_of::<MassFragmentSlice>(), 16);
        assert_eq!(std::mem::align_of::<MassFragmentSlice>(), 8);
    }

    #[test]
    fn mass_chunk_data_layout() {
        // i32 + f32 + u32 + pad + ptr + i32 + u32 + ptr + ptr + ptr = 56 on 64-bit
        let size = std::mem::size_of::<MassChunkData>();
        assert_eq!(size, 56);
        assert_eq!(std::mem::align_of::<MassChunkData>(), 8);
    }

    #[test]
    fn mass_global_chunk_slice_layout() {
        assert_eq!(std::mem::size_of::<MassGlobalChunkSlice>(), 16);
        assert_eq!(std::mem::align_of::<MassGlobalChunkSlice>(), 8);
    }

    #[test]
    fn mass_global_fragment_chunks_layout() {
        // ptr(8) + u32(4) + i32(4) + u32(4) + pad(4) = 24
        assert_eq!(std::mem::size_of::<MassGlobalFragmentChunks>(), 24);
        assert_eq!(std::mem::align_of::<MassGlobalFragmentChunks>(), 8);
    }

    #[test]
    fn mass_fragment_requirement_layout() {
        // Utf8Str (ptr + usize = 16) + u32 + u32 + u8 + u8 + u8 + u8 + padding
        let size = std::mem::size_of::<MassFragmentRequirement>();
        assert_eq!(size, 32);
    }

    #[test]
    fn mass_system_descriptor_layout() {
        // Utf8Str(16) + u32(4) + u32(4) + ptr(8) + fn_ptr(8) = 40
        let size = std::mem::size_of::<MassSystemDescriptor>();
        assert_eq!(size, 40);
    }

    #[test]
    fn mass_system_chunk_batch_layout() {
        // u32(4) + u32(4) + ptr(8) = 16
        assert_eq!(std::mem::size_of::<MassSystemChunkBatch>(), 16);
        assert_eq!(std::mem::align_of::<MassSystemChunkBatch>(), 8);
    }

    #[test]
    fn mass_frame_dispatch_data_layout() {
        // f32(4) + u32(4) + ptr(8) + Option<fn>(8) + f32(4) + u32(4) = 32
        assert_eq!(std::mem::size_of::<MassFrameDispatchData>(), 32);
        assert_eq!(std::mem::align_of::<MassFrameDispatchData>(), 8);
    }

    #[test]
    fn mass_spatial_query_result_layout() {
        // i32(4) + i32(4) + [f64;3](24) + bool(1) + [u8;7](7) = 40
        assert_eq!(std::mem::size_of::<MassSpatialQueryResult>(), 40);
        assert_eq!(std::mem::align_of::<MassSpatialQueryResult>(), 8);
    }

    #[test]
    fn uninit_stubs_return_zero_systems() {
        let bindings = RustBindings::uninit();
        let count = unsafe { (bindings.get_mass_system_count)() };
        assert_eq!(count, 0);
    }

    #[test]
    fn mass_entity_handle_layout() {
        // i32(4) + i32(4) = 8
        assert_eq!(std::mem::size_of::<MassEntityHandle>(), 8);
        assert_eq!(std::mem::align_of::<MassEntityHandle>(), 4);
    }

    #[test]
    fn mass_entity_handle_default_is_invalid() {
        let h = MassEntityHandle::default();
        assert_eq!(h.index, 0);
        assert_eq!(h.serial_number, 0);
    }

    #[test]
    fn spawn_fragment_data_layout() {
        // Utf8Str(16) + u32(4) + u32(4) + ptr(8) = 32
        assert_eq!(std::mem::size_of::<SpawnFragmentData>(), 32);
        assert_eq!(std::mem::align_of::<SpawnFragmentData>(), 8);
    }

    #[test]
    fn spawn_tag_data_layout() {
        // Utf8Str(16)
        assert_eq!(std::mem::size_of::<SpawnTagData>(), 16);
        assert_eq!(std::mem::align_of::<SpawnTagData>(), 8);
    }

    #[test]
    fn spawn_entity_request_layout() {
        // u32(4) + pad(4) + ptr(8) + u32(4) + pad(4) + ptr(8) + u32(4) + u32(4) = 40
        assert_eq!(std::mem::size_of::<SpawnEntityRequest>(), 40);
        assert_eq!(std::mem::align_of::<SpawnEntityRequest>(), 8);
    }

    #[test]
    fn spawn_entities_fn_is_option_safe() {
        // Option<SpawnEntitiesFn> must be pointer-sized for nullable fn ptr
        assert_eq!(
            std::mem::size_of::<Option<SpawnEntitiesFn>>(),
            std::mem::size_of::<*const ()>(),
        );
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct FStringFns {
    pub new_fstring_from_utf8: NewFStringFromUtf8Fn,
    pub copy_from_fstring: CopyFromFStringFn,
    pub dtor: FStringDtorFn,
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
    pub ctor: FScriptArrayCtorFn,
    pub dtor: FScriptArrayDtorFn,
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
