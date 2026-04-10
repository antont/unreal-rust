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

/// One named spatial query slot in the per-frame dispatch.
#[repr(C)]
pub struct MassSpatialQuerySlot {
    /// Query name (e.g. "food_pickup"). Borrowed from C++ — valid for this frame.
    pub name: Utf8Str,
    /// Callback function implementing this query.
    pub query_fn: MassSpatialQueryFn,
    /// Radius for this query.
    pub radius: f32,
    pub _pad: u32,
}

/// Per-frame dispatch data: dt + all system chunk batches + named spatial queries.
#[repr(C)]
pub struct MassFrameDispatchData {
    /// Delta time for this frame.
    pub dt: f32,
    /// Number of system batches.
    pub num_systems: u32,
    /// Pointer to array of MassSystemChunkBatch.
    pub systems: *const MassSystemChunkBatch,
    /// Number of spatial query slots available this frame.
    pub num_spatial_queries: u32,
    pub _pad: u32,
    /// Pointer to array of MassSpatialQuerySlot.
    pub spatial_queries: *const MassSpatialQuerySlot,
}

/// Function signature for per-frame Bevy-scheduled dispatch.
pub type MassFrameDispatchFn = unsafe extern "C" fn(data: *const MassFrameDispatchData);

// --- Spatial query callback for Rust collision processor ---

/// Result of a spatial query (nearest entity encounter detection).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MassSpatialQueryResult {
    /// Index of the nearest matching entity, or -1 if none found.
    pub entity_index: i32,
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
/// - `previous_pos`: entity's previous position (3 x f64)
/// - `current_pos`: entity's current position (3 x f64)
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

// --- Simulation init: named entity groups ---

/// Describes one entity group to spawn.
#[repr(C)]
pub struct MassEntityGroupDesc {
    /// Group name (e.g. "ants", "food").
    pub name: Utf8Str,
    /// Number of entities to spawn in this group.
    pub count: i32,
    pub _pad: i32,
}

/// Parameters for initializing a simulation.
#[repr(C)]
pub struct MassInitSimulationParams {
    pub groups: *const MassEntityGroupDesc,
    pub num_groups: u32,
    pub _pad0: u32,
    pub bounds_min: [f64; 3],
    pub bounds_max: [f64; 3],
    pub random_seed: i32,
    pub _pad1: i32,
}

/// One group of spawned entity handles in the init result.
#[repr(C)]
pub struct MassEntityGroupResult {
    /// Group name (matches the request name).
    pub name: Utf8Str,
    /// Pointer to array of entity handles.
    pub handles: *const MassEntityHandle,
    /// Number of handles.
    pub count: u32,
    pub _pad: u32,
}

/// Result of simulation init: named groups of entity handles.
#[repr(C)]
pub struct MassInitSimulationResult {
    pub groups: *const MassEntityGroupResult,
    pub num_groups: u32,
    pub _pad: u32,
}

/// Init function type.
pub type MassInitSimulationFn = unsafe extern "C" fn(
    params: *const MassInitSimulationParams,
    result: *mut MassInitSimulationResult,
) -> u32;

// --- Spatial query configuration (declared by Rust, read by C++) ---

/// Tells C++ how to implement spatial queries for a given entity group.
/// Rust game crates declare these via inventory; C++ reads them at startup
/// to create query callbacks (ISMC overlap or physics sweep).
#[repr(C)]
pub struct MassSpatialQueryConfigDesc {
    /// Unique query name (e.g. "food_pickup").
    pub query_name: Utf8Str,
    /// ISMC group name to search (e.g. "food").
    pub query_group: Utf8Str,
    /// Overlap sphere radius in Unreal units.
    pub radius: f32,
    pub _pad0: u32,
    /// C++ fragment type name that has the bool to filter on (e.g. "FGatherersFood").
    pub filter_fragment_type: Utf8Str,
    /// Byte offset of the bool field within the filter fragment.
    pub filter_bool_offset: u32,
    /// Required value of the bool field (true = only match when bool is true).
    pub filter_bool_must_be: bool,
    /// Query type: 0 = ISMC overlap, 1 = physics sweep.
    pub query_type: u8,
    /// Collision channel index for physics sweep (0 = ECC_GameTraceChannel1).
    pub collision_channel_index: u8,
    pub _pad1: u8,
}

// --- Rust-authored test infrastructure ---

/// Callback function pointers that C++ provides to Rust test functions.
/// Each callback wraps a URustMassBevySubsystem operation.
#[repr(C)]
pub struct MassTestCallbacks {
    /// Opaque pointer to C++ test harness (URustMassBevySubsystem* + world context).
    pub opaque: *mut c_void,

    // Simulation lifecycle
    pub init_sim: unsafe extern "C" fn(ctx: *mut c_void, params: *const MassInitSimulationParams) -> u32,
    pub step_sim: unsafe extern "C" fn(ctx: *mut c_void, dt: f32, count: u32),
    pub reset_sim: unsafe extern "C" fn(ctx: *mut c_void),
    pub tick: unsafe extern "C" fn(ctx: *mut c_void, dt: f32),
    pub on_rust_reloaded: unsafe extern "C" fn(ctx: *mut c_void),

    // Queries
    pub entity_count: unsafe extern "C" fn(ctx: *mut c_void, group: Utf8Str) -> i32,
    pub has_managed_sim: unsafe extern "C" fn(ctx: *mut c_void) -> u32,
    pub has_spatial_query: unsafe extern "C" fn(ctx: *mut c_void, name: Utf8Str) -> u32,

    // Generic fragment access by C++ type name
    pub read_fragment: unsafe extern "C" fn(
        ctx: *mut c_void, group: Utf8Str, index: u32,
        fragment_type: Utf8Str, out: *mut u8, size: u32,
    ) -> u32,
    pub write_fragment: unsafe extern "C" fn(
        ctx: *mut c_void, group: Utf8Str, index: u32,
        fragment_type: Utf8Str, data: *const u8, size: u32,
    ) -> u32,
}

/// Describes one registered Rust-authored test.
#[repr(C)]
pub struct MassTestDesc {
    pub name: Utf8Str,
}

/// Result of running a Rust-authored test.
#[repr(C)]
pub struct MassTestResult {
    /// 1 = passed, 0 = failed.
    pub passed: u32,
    /// Length of error message in bytes (0 if passed).
    pub error_len: u32,
    /// Pointer to UTF-8 error message (null if passed).
    /// Owned by Rust — C++ must copy before next FFI call.
    pub error_ptr: *const u8,
}

/// Returns the number of registered Rust-authored tests.
pub type GetMassTestCountFn = unsafe extern "C" fn() -> u32;

/// Fills a MassTestDesc for the test at `index`. Returns 1 on success, 0 on failure.
pub type GetMassTestDescFn = unsafe extern "C" fn(index: u32, out: *mut MassTestDesc) -> u32;

/// Runs the named Rust test with the provided callbacks. Returns test result.
pub type RunMassTestFn = unsafe extern "C" fn(
    name: Utf8Str,
    callbacks: *const MassTestCallbacks,
) -> MassTestResult;

/// Returns the number of registered spatial query configurations.
pub type GetSpatialQueryConfigCountFn = unsafe extern "C" fn() -> u32;

/// Fills a MassSpatialQueryConfigDesc for the config at `index`.
/// Returns 1 on success, 0 on failure.
pub type GetSpatialQueryConfigDescFn =
    unsafe extern "C" fn(index: u32, out: *mut MassSpatialQueryConfigDesc) -> u32;

// --- Simulation defaults (declared by Rust, overridable by C++ actor) ---

/// Default simulation configuration registered from Rust.
/// C++ reads this at startup; actor UPROPERTY values can override.
#[repr(C)]
pub struct MassSimDefaultsDesc {
    pub groups: *const MassEntityGroupDesc,
    pub num_groups: u32,
    pub _pad: u32,
    pub bounds_min: [f64; 3],
    pub bounds_max: [f64; 3],
    pub random_seed: i32,
    pub _pad2: i32,
}

/// Fills the sim defaults from Rust. Returns 1 if defaults are available, 0 otherwise.
pub type GetSimDefaultsFn = unsafe extern "C" fn(out: *mut MassSimDefaultsDesc) -> u32;

// --- Visualizer group descriptors ---

/// Describes one visual group (entity type with position data for ISMC rendering).
#[repr(C)]
pub struct MassVisualizerGroupDesc {
    /// Group name (e.g. "ants", "food").
    pub name: Utf8Str,
    /// C++ USTRUCT name of the fragment that contains position data.
    pub position_fragment_type: Utf8Str,
    /// Byte offset of the position [f64; 3] within the fragment.
    pub position_offset: u32,
    /// Uniform scale for the ISMC instances.
    pub scale: f32,
}

/// Returns the number of registered visualizer groups.
pub type GetVisualizerGroupCountFn = unsafe extern "C" fn() -> u32;

/// Fills a MassVisualizerGroupDesc for the group at `index`.
/// Returns 1 on success, 0 on failure.
pub type GetVisualizerGroupDescFn =
    unsafe extern "C" fn(index: u32, out: *mut MassVisualizerGroupDesc) -> u32;

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
    pub get_visualizer_group_count: Option<GetVisualizerGroupCountFn>,
    pub get_visualizer_group_desc: Option<GetVisualizerGroupDescFn>,
    pub mass_init_simulation: Option<MassInitSimulationFn>,
    pub get_spatial_query_config_count: Option<GetSpatialQueryConfigCountFn>,
    pub get_spatial_query_config_desc: Option<GetSpatialQueryConfigDescFn>,
    pub get_sim_defaults: Option<GetSimDefaultsFn>,
    pub get_mass_test_count: Option<GetMassTestCountFn>,
    pub get_mass_test_desc: Option<GetMassTestDescFn>,
    pub run_mass_test: Option<RunMassTestFn>,
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
            get_visualizer_group_count: None,
            get_visualizer_group_desc: None,
            mass_init_simulation: None,
            get_spatial_query_config_count: None,
            get_spatial_query_config_desc: None,
            get_sim_defaults: None,
            get_mass_test_count: None,
            get_mass_test_desc: None,
            run_mass_test: None,
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
        // RustBindings: 7 non-optional fn ptrs + 6 Option<fn ptr> = 13 pointers
        let size = std::mem::size_of::<RustBindings>();
        assert_eq!(size, 13 * std::mem::size_of::<usize>(),
            "actual size = {}, expected = {}", size, 13 * std::mem::size_of::<usize>());
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
    fn mass_spatial_query_slot_layout() {
        // Utf8Str(16) + fn_ptr(8) + f32(4) + u32(4) = 32
        assert_eq!(std::mem::size_of::<MassSpatialQuerySlot>(), 32);
        assert_eq!(std::mem::align_of::<MassSpatialQuerySlot>(), 8);
        assert_eq!(std::mem::offset_of!(MassSpatialQuerySlot, name), 0);
        assert_eq!(std::mem::offset_of!(MassSpatialQuerySlot, query_fn), 16);
        assert_eq!(std::mem::offset_of!(MassSpatialQuerySlot, radius), 24);
    }

    #[test]
    fn mass_frame_dispatch_data_layout() {
        // f32(4) + u32(4) + ptr(8) + u32(4) + u32(4) + ptr(8) = 32
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
    fn mass_entity_group_desc_layout() {
        // Utf8Str(16) + i32(4) + i32(4) = 24
        assert_eq!(std::mem::size_of::<MassEntityGroupDesc>(), 24);
        assert_eq!(std::mem::align_of::<MassEntityGroupDesc>(), 8);
    }

    #[test]
    fn mass_init_simulation_params_layout() {
        // ptr(8) + u32(4) + u32(4) + [f64;3](24) + [f64;3](24) + i32(4) + i32(4) = 72
        assert_eq!(std::mem::size_of::<MassInitSimulationParams>(), 72);
        assert_eq!(std::mem::align_of::<MassInitSimulationParams>(), 8);
    }

    #[test]
    fn mass_entity_group_result_layout() {
        // Utf8Str(16) + ptr(8) + u32(4) + u32(4) = 32
        assert_eq!(std::mem::size_of::<MassEntityGroupResult>(), 32);
        assert_eq!(std::mem::align_of::<MassEntityGroupResult>(), 8);
    }

    #[test]
    fn mass_init_simulation_result_layout() {
        // ptr(8) + u32(4) + u32(4) = 16
        assert_eq!(std::mem::size_of::<MassInitSimulationResult>(), 16);
        assert_eq!(std::mem::align_of::<MassInitSimulationResult>(), 8);
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
    fn mass_spatial_query_config_desc_layout() {
        // Utf8Str(16) + Utf8Str(16) + f32(4) + u32(4) + Utf8Str(16) + u32(4) + bool(1) + u8(1) + u8(1) + u8(1) = 64
        assert_eq!(std::mem::size_of::<MassSpatialQueryConfigDesc>(), 64);
        assert_eq!(std::mem::align_of::<MassSpatialQueryConfigDesc>(), 8);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, query_name), 0);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, query_group), 16);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, radius), 32);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, filter_fragment_type), 40);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, filter_bool_offset), 56);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, filter_bool_must_be), 60);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, query_type), 61);
        assert_eq!(std::mem::offset_of!(MassSpatialQueryConfigDesc, collision_channel_index), 62);
    }

    #[test]
    fn mass_sim_defaults_desc_layout() {
        // ptr(8) + u32(4) + u32(4) + [f64;3](24) + [f64;3](24) + i32(4) + i32(4) = 72
        assert_eq!(std::mem::size_of::<MassSimDefaultsDesc>(), 72);
        assert_eq!(std::mem::align_of::<MassSimDefaultsDesc>(), 8);
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
