#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class ResultCode : uint8_t {
  Success = 0,
  Panic = 1,
};

// clang-format off
// NOLINTBEGIN
/// Rust Option<fn pointer> is ABI-compatible with a nullable pointer.
/// This minimal definition lets C++ code compile against the FFI types.
template<typename T = void>
struct Option {
  T value;
  Option() : value(nullptr) {}
  Option(T v) : value(v) {} // NOLINT
  bool IsSome() const { return value != nullptr; }
  bool IsNone() const { return value == nullptr; }
  T Unwrap() const { return value; }
};
// NOLINTEND
// clang-format on

struct Utf8Str {
  const char *ptr;
  uintptr_t len;
};

using UClassOpague = void;

using UObjectOpague = void;

struct RustAlloc {
  uint8_t *ptr;
  uintptr_t size;
  uintptr_t align;
};

struct StrRustAlloc {
  RustAlloc alloc;
};

using UFunctionOpague = void;

struct FRustString {
  uint16_t *data;
  int32_t num;
  int32_t max;
};

struct FRustScriptArray {
  uint64_t opaque_words[2];
};

using LogFn = void(*)(Utf8Str message);

using GetCDOFromClassCoreFn = uint32_t(*)(const UClassOpague *cdo_opague, UObjectOpague**);

using GetAllUClassesCoreFn = uint32_t(*)(RustAlloc *out);

using GetClassNameCoreFn = uint32_t(*)(const UClassOpague *opague_class, StrRustAlloc *out);

using FindFunctionByNameCoreFn = uint32_t(*)(const UClassOpague *class_opague,
                                             Utf8Str str,
                                             UFunctionOpague **function_opague);

using InitializeValuesInParamBufferCoreFn = uint32_t(*)(const UFunctionOpague *function_opague,
                                                        void *buffer);

using DestroyValuesInParamBufferCoreFn = uint32_t(*)(const UFunctionOpague *function_opague,
                                                     void *buffer);

using ProcessEventsCoreFn = uint32_t(*)(UObjectOpague *object_opague,
                                        UFunctionOpague *function_opague,
                                        void *buffer);

using BeginTraceCoreFn = void(*)(const char *name);

using EndTraceCoreFn = void(*)();

struct CoreFns {
  GetCDOFromClassCoreFn get_cdo_from_class;
  GetAllUClassesCoreFn get_all_uclasses;
  GetClassNameCoreFn get_class_name;
  FindFunctionByNameCoreFn find_function_by_name;
  InitializeValuesInParamBufferCoreFn initialize_values_in_param_buffer;
  DestroyValuesInParamBufferCoreFn destroy_values_in_param_buffer;
  ProcessEventsCoreFn process_event;
  BeginTraceCoreFn begin_trace;
  EndTraceCoreFn end_trace;
};

using NewFStringFromUtf8Fn = void(*)(Utf8Str str, FRustString *fstring);

using CopyFromFStringFn = void(*)(const FRustString *source, FRustString *fstring);

using FStringDtorFn = void(*)(FRustString *fstring);

struct FStringFns {
  NewFStringFromUtf8Fn new_fstring_from_utf8;
  CopyFromFStringFn copy_from_fstring;
  FStringDtorFn dtor;
};

using FScriptArrayNumFn = uint32_t(*)(const FRustScriptArray *array, int32_t *out_num);

using FScriptArrayCtorFn = uint32_t(*)(FRustScriptArray *array);

using FScriptArrayDtorFn = uint32_t(*)(FRustScriptArray *array);

using FScriptArrayMaxFn = uint32_t(*)(const FRustScriptArray *array, int32_t *out_max);

using FScriptArrayGetDataFn = uint32_t(*)(FRustScriptArray *array, void **out_data);

using FScriptArrayIsValidIndexFn = uint32_t(*)(const FRustScriptArray *array, int32_t index);

using FScriptArrayReserveFn = uint32_t(*)(FRustScriptArray *array,
                                          int32_t capacity,
                                          int32_t elem_size,
                                          uint32_t elem_align);

using FScriptArrayAddFn = uint32_t(*)(FRustScriptArray *array,
                                      int32_t count,
                                      int32_t elem_size,
                                      uint32_t elem_align,
                                      int32_t *out_index);

using FScriptArrayInsertFn = uint32_t(*)(FRustScriptArray *array,
                                         int32_t index,
                                         int32_t count,
                                         int32_t elem_size,
                                         uint32_t elem_align);

using FScriptArrayRemoveFn = uint32_t(*)(FRustScriptArray *array,
                                         int32_t index,
                                         int32_t count,
                                         int32_t elem_size,
                                         uint32_t elem_align);

using FScriptArrayEmptyFn = uint32_t(*)(FRustScriptArray *array,
                                        int32_t slack,
                                        int32_t elem_size,
                                        uint32_t elem_align);

using FScriptArrayResetFn = uint32_t(*)(FRustScriptArray *array,
                                        int32_t new_size,
                                        int32_t elem_size,
                                        uint32_t elem_align);

using FScriptArrayShrinkFn = uint32_t(*)(FRustScriptArray *array,
                                         int32_t elem_size,
                                         uint32_t elem_align);

struct FScriptArrayFns {
  FScriptArrayNumFn num;
  FScriptArrayCtorFn ctor;
  FScriptArrayDtorFn dtor;
  FScriptArrayMaxFn max;
  FScriptArrayGetDataFn get_data;
  FScriptArrayIsValidIndexFn is_valid_index;
  FScriptArrayReserveFn reserve;
  FScriptArrayAddFn add;
  FScriptArrayInsertFn insert;
  FScriptArrayRemoveFn remove;
  FScriptArrayEmptyFn empty;
  FScriptArrayResetFn reset;
  FScriptArrayShrinkFn shrink;
};

/// Describes one fragment type's initial data for a batch spawn request.
struct SpawnFragmentData {
  /// C++ USTRUCT type name (e.g. "FGatherersPosition").
  Utf8Str cpp_type_name;
  /// Size of one fragment instance in bytes.
  uint32_t size;
  /// Alignment of the fragment type.
  uint32_t alignment;
  /// Pointer to `count` contiguous fragment values (size * count bytes).
  /// Null = use C++ default constructor for all entities.
  const void *initial_data;
};

/// Describes one tag type to apply to spawned entities.
struct SpawnTagData {
  /// C++ USTRUCT type name (e.g. "FGatherersBevyMassAntTag").
  Utf8Str cpp_type_name;
};

/// A batch spawn request: create `count` entities with the given fragments and tags.
struct SpawnEntityRequest {
  /// Number of fragment types.
  uint32_t num_fragments;
  /// Pointer to array of SpawnFragmentData (one per fragment type).
  const SpawnFragmentData *fragments;
  /// Number of tags.
  uint32_t num_tags;
  /// Pointer to array of SpawnTagData.
  const SpawnTagData *tags;
  /// Number of entities to spawn.
  uint32_t count;
  uint32_t _padding;
};

/// FFI-safe entity handle matching FMassEntityHandle layout.
struct MassEntityHandle {
  /// Entity index in the entity manager.
  int32_t index;
  /// Serial number for handle validation.
  int32_t serial_number;
};

/// Spawns `request.count` entities. Writes handles to `out_handles` (must have room for `count`).
/// Returns the number of entities actually created (should equal `count` on success).
using SpawnEntitiesFn = uint32_t(*)(const SpawnEntityRequest *request, MassEntityHandle *out_handles);

struct UnrealBindings {
  LogFn log;
  CoreFns core_fns;
  FStringFns fstring_fns;
  FScriptArrayFns fscript_array_fns;
  /// Spawn entities in Mass Entity. Null until a Mass subsystem sets it.
  Option<SpawnEntitiesFn> spawn_entities;
};

using TickFn = ResultCode(*)(float dt);

using BeginPlayFn = ResultCode(*)();

using AllocateFn = uint32_t(*)(uintptr_t size, uintptr_t align, RustAlloc *ptr);

using MassBobProcessFn = void(*)(void *data, int32_t count, float dt);

/// Returns the number of dynamically registered mass systems.
using GetMassSystemCountFn = uint32_t(*)();

/// Describes one fragment/tag requirement for a Rust mass system.
struct MassFragmentRequirement {
  /// C++ USTRUCT type name (e.g. "FGatherersMassAntFragment").
  Utf8Str cpp_type_name;
  /// Size of the fragment in bytes.
  uint32_t size;
  /// Alignment of the fragment.
  uint32_t alignment;
  /// Access mode: 0 = ReadOnly, 1 = ReadWrite.
  uint8_t access_mode;
  /// Whether this is a tag (1) or fragment (0).
  uint8_t is_tag;
  /// Query scope: 0 = primary (per-chunk), 1 = global (all matching entities).
  uint8_t query_scope;
  uint8_t _padding;
};

/// Data for one fragment slice in a chunk, passed from C++ ForEachEntityChunk to Rust.
struct MassFragmentSlice {
  /// Raw pointer to contiguous fragment data in the chunk.
  void *data;
  /// Number of entities in this chunk.
  int32_t count;
  /// Size of each fragment element in bytes.
  uint32_t stride;
};

/// One chunk's slice of data for a global query — points directly into Mass Entity chunk memory.
struct MassGlobalChunkSlice {
  /// Direct pointer into chunk fragment memory (zero-copy).
  void *data;
  /// Number of entities in this chunk.
  int32_t count;
  /// Size of each fragment element in bytes.
  uint32_t stride;
};

/// All chunk slices for one global fragment type.
struct MassGlobalFragmentChunks {
  /// Pointer to array of MassGlobalChunkSlice, one per chunk.
  const MassGlobalChunkSlice *chunks;
  /// Number of chunks.
  uint32_t num_chunks;
  /// Total entity count across all chunks.
  int32_t total_count;
  /// Stride (same for all chunks — same fragment type).
  uint32_t stride;
};

/// Chunk data passed from C++ Execute to a Rust mass system function.
struct MassChunkData {
  /// Number of entities in this chunk.
  int32_t num_entities;
  /// Delta time for this simulation step.
  float dt;
  /// Number of primary fragment slices (matches system's primary requirement count).
  uint32_t num_fragments;
  /// Pointer to array of MassFragmentSlice, one per primary requirement in declaration order.
  const MassFragmentSlice *fragments;
  /// Number of entities across all chunks for global queries.
  int32_t global_num_entities;
  /// Number of global fragment slices.
  uint32_t num_global_fragments;
  /// Deprecated: was contiguous-copy global fragments. Set to null for zero-copy path.
  const MassFragmentSlice *global_fragments;
  /// Deprecated: was entity handle pairs. Set to null for index-based access.
  const int32_t *global_entity_handles;
  /// Zero-copy chunked global fragments: array of MassGlobalFragmentChunks, one per global fragment type.
  const MassGlobalFragmentChunks *global_chunked_fragments;
};

/// Execute function signature for a dynamically registered mass system.
using MassSystemExecuteFn = void(*)(const MassChunkData *chunk);

/// Describes one registered Rust mass system. C++ queries these at init time.
struct MassSystemDescriptor {
  /// Unique system name (e.g. "ant_movement").
  Utf8Str name;
  /// Number of fragment/tag requirements.
  uint32_t num_requirements;
  /// Execution order within a pipeline. Lower values run first.
  uint32_t order;
  /// Pointer to array of MassFragmentRequirement.
  const MassFragmentRequirement *requirements;
  /// The Rust function to call during Execute.
  MassSystemExecuteFn execute_fn;
};

/// Fills a MassSystemDescriptor for the system at `index`. Returns 1 on success, 0 on failure.
using GetMassSystemDescriptorFn = uint32_t(*)(uint32_t index, MassSystemDescriptor *out);

/// One system's cached primary chunk data, passed from C++ coordinator to Rust.
struct MassSystemChunkBatch {
  /// Index of the system in registration order.
  uint32_t system_index;
  /// Number of primary chunks for this system.
  uint32_t num_primary_chunks;
  /// Pointer to array of MassChunkData, one per primary chunk.
  const MassChunkData *primary_chunks;
};

/// Result of a spatial query for one ant (food encounter detection).
struct MassSpatialQueryResult {
  /// Index of the nearest food entity, or -1 if none found.
  int32_t food_index;
  int32_t _pad;
  /// Position where the encounter occurred.
  double encounter_position[3];
  /// Whether a food encounter was found.
  bool has_encounter;
  uint8_t _result_pad[7];
};

/// Callback function for spatial queries. C++ implements this using UE physics.
///
/// Parameters:
/// - `previous_pos`: ant's previous position (3 x f64)
/// - `current_pos`: ant's current position (3 x f64)
/// - `pickup_radius`: radius for collision detection
/// - `out`: result written here
///
/// Returns 1 on success, 0 on failure.
using MassSpatialQueryFn = uint32_t(*)(const double *previous_pos,
                                       const double *current_pos,
                                       float pickup_radius,
                                       MassSpatialQueryResult *out);

/// Per-frame dispatch data: dt + all system chunk batches + spatial query callback.
struct MassFrameDispatchData {
  /// Delta time for this frame.
  float dt;
  /// Number of system batches.
  uint32_t num_systems;
  /// Pointer to array of MassSystemChunkBatch.
  const MassSystemChunkBatch *systems;
  /// Optional spatial query callback for collision detection. Null if not available.
  Option<MassSpatialQueryFn> spatial_query_fn;
  /// Pickup radius for spatial queries (Unreal units).
  float pickup_radius;
  uint32_t _pad;
};

/// Function signature for per-frame Bevy-scheduled dispatch.
using MassFrameDispatchFn = void(*)(const MassFrameDispatchData *data);

/// Parameters for initializing a simulation from Rust.
struct MassInitSimulationParams {
  int32_t ant_count;
  int32_t food_count;
  double bounds_min[3];
  double bounds_max[3];
  int32_t random_seed;
  int32_t _pad;
};

/// Result of simulation init: handles to spawned entities.
struct MassInitSimulationResult {
  const MassEntityHandle *ant_handles;
  uint32_t num_ants;
  const MassEntityHandle *food_handles;
  uint32_t num_food;
};

/// Initialize simulation: spawn entities from Rust.
/// Returns 1 on success, 0 on failure.
using MassInitSimulationFn = uint32_t(*)(const MassInitSimulationParams *params,
                                         MassInitSimulationResult *result);

/// Describes one visual group (entity type with position data for ISMC rendering).
struct MassVisualizerGroupDesc {
  /// Group name (e.g. "ants", "food").
  Utf8Str name;
  /// C++ USTRUCT name of the fragment that contains position data.
  Utf8Str position_fragment_type;
  /// Byte offset of the position [f64; 3] within the fragment.
  uint32_t position_offset;
  /// Uniform scale for the ISMC instances.
  float scale;
};

/// Returns the number of registered visualizer groups.
using GetVisualizerGroupCountFn = uint32_t(*)();

/// Fills a MassVisualizerGroupDesc for the group at `index`.
/// Returns 1 on success, 0 on failure.
using GetVisualizerGroupDescFn = uint32_t(*)(uint32_t index, MassVisualizerGroupDesc *out);

struct RustBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  AllocateFn allocate;
  MassBobProcessFn mass_bob_process;
  GetMassSystemCountFn get_mass_system_count;
  GetMassSystemDescriptorFn get_mass_system_descriptor;
  MassFrameDispatchFn mass_frame_dispatch;
  Option<MassInitSimulationFn> mass_init_simulation;
  Option<GetVisualizerGroupCountFn> get_visualizer_group_count;
  Option<GetVisualizerGroupDescFn> get_visualizer_group_desc;
};

using EntryUnrealBindingsFn = uint32_t(*)(UnrealBindings bindings);

using TryLoadFn = uint32_t(*)(RustBindings*);

using IsOutOfDateFn = uint32_t(*)();

struct PluginBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  TryLoadFn try_load;
  IsOutOfDateFn is_out_of_date;
};

extern "C" {

extern void Log(Utf8Str message);

extern uint32_t GetCDOFromClass(const UClassOpague *class_opague, UObjectOpague **out_object);

extern uint32_t GetAllUClasses(RustAlloc *out);

extern uint32_t GetClassName(const UClassOpague *opague_class, StrRustAlloc *out);

extern uint32_t FindFunctionByName(const UClassOpague *class_opague,
                                   Utf8Str name,
                                   UFunctionOpague **function_opague);

extern uint32_t InitializeValuesInParamBuffer(const UFunctionOpague *function_opague, void *buffer);

extern uint32_t DestroyValuesInParamBuffer(const UFunctionOpague *function_opague, void *buffer);

extern uint32_t ProcessEventFromRust(UObjectOpague *cdo_opague,
                                     UFunctionOpague *function_opague,
                                     void *buffer);

extern void BeginTrace(const char *name);

extern void EndTrace();

extern void NewFStringFromUtf8(Utf8Str str, FRustString *fstring);

extern void CopyFromFString(const FRustString *source, FRustString *fstring);

extern void FStringDtor(FRustString *fstring);

extern uint32_t FScriptArrayNum(const FRustScriptArray *array, int32_t *out_num);

extern uint32_t FScriptArrayCtor(FRustScriptArray *array);

extern uint32_t FScriptArrayDtor(FRustScriptArray *array);

extern uint32_t FScriptArrayMax(const FRustScriptArray *array, int32_t *out_max);

extern uint32_t FScriptArrayGetData(FRustScriptArray *array, void **out_data);

extern uint32_t FScriptArrayIsValidIndex(const FRustScriptArray *array, int32_t index);

extern uint32_t FScriptArrayReserve(FRustScriptArray *array,
                                    int32_t capacity,
                                    int32_t elem_size,
                                    uint32_t elem_align);

extern uint32_t FScriptArrayAdd(FRustScriptArray *array,
                                int32_t count,
                                int32_t elem_size,
                                uint32_t elem_align,
                                int32_t *out_index);

extern uint32_t FScriptArrayInsert(FRustScriptArray *array,
                                   int32_t index,
                                   int32_t count,
                                   int32_t elem_size,
                                   uint32_t elem_align);

extern uint32_t FScriptArrayRemove(FRustScriptArray *array,
                                   int32_t index,
                                   int32_t count,
                                   int32_t elem_size,
                                   uint32_t elem_align);

extern uint32_t FScriptArrayEmpty(FRustScriptArray *array,
                                  int32_t slack,
                                  int32_t elem_size,
                                  uint32_t elem_align);

extern uint32_t FScriptArrayReset(FRustScriptArray *array,
                                  int32_t new_size,
                                  int32_t elem_size,
                                  uint32_t elem_align);

extern uint32_t FScriptArrayShrink(FRustScriptArray *array, int32_t elem_size, uint32_t elem_align);

}  // extern "C"
