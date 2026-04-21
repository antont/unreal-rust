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
/// FFI-safe Option type matching Rust's niche-optimized Option<fn pointer> layout.
/// Rust represents Option<fn pointer> as just the pointer (null = None), so this
/// must NOT include a separate bool field. sizeof(Option<T>) must equal sizeof(T).
/// WARNING: Do not add a `bool is_some` field — it breaks the FFI layout.
template<typename T = void>
struct Option {
  T value;
  Option() : value{} {}
  Option(T v) : value(v) {}
  Option& operator=(T v) { value = v; return *this; }
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

/// Log level values matching the Rust `log` crate:
/// 1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace.
using LogFn = void(*)(Utf8Str message, uint8_t level);

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
  /// C++ USTRUCT type name (e.g. "FGatherersAntTag").
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

/// Result of a spatial query (nearest entity encounter detection).
struct MassSpatialQueryResult {
  /// Index of the nearest matching entity, or -1 if none found.
  int32_t entity_index;
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
/// - `previous_pos`: entity's previous position (3 x f64)
/// - `current_pos`: entity's current position (3 x f64)
/// - `pickup_radius`: radius for collision detection
/// - `out`: result written here
///
/// Returns 1 on success, 0 on failure.
using MassSpatialQueryFn = uint32_t(*)(const double *previous_pos,
                                       const double *current_pos,
                                       float pickup_radius,
                                       MassSpatialQueryResult *out);

/// One named spatial query slot in the per-frame dispatch.
struct MassSpatialQuerySlot {
  /// Query name (e.g. "food_pickup"). Borrowed from C++ — valid for this frame.
  Utf8Str name;
  /// Callback function implementing this query.
  MassSpatialQueryFn query_fn;
  /// Radius for this query.
  float radius;
  uint32_t _pad;
};

/// Per-frame dispatch data: dt + all system chunk batches + named spatial queries.
struct MassFrameDispatchData {
  /// Delta time for this frame.
  float dt;
  /// Number of system batches.
  uint32_t num_systems;
  /// Pointer to array of MassSystemChunkBatch.
  const MassSystemChunkBatch *systems;
  /// Number of spatial query slots available this frame.
  uint32_t num_spatial_queries;
  uint32_t _pad;
  /// Pointer to array of MassSpatialQuerySlot.
  const MassSpatialQuerySlot *spatial_queries;
};

/// Function signature for per-frame Bevy-scheduled dispatch.
/// Returns a bitmask of post-dispatch flags (reserved for future use).
using MassFrameDispatchFn = uint32_t(*)(const MassFrameDispatchData *data);

/// Returns the number of registered visualizer groups.
using GetVisualizerGroupCountFn = uint32_t(*)();

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

/// Fills a MassVisualizerGroupDesc for the group at `index`.
/// Returns 1 on success, 0 on failure.
using GetVisualizerGroupDescFn = uint32_t(*)(uint32_t index, MassVisualizerGroupDesc *out);

/// Describes one entity group to spawn.
struct MassEntityGroupDesc {
  /// Group name (e.g. "ants", "food").
  Utf8Str name;
  /// Number of entities to spawn in this group.
  int32_t count;
  int32_t _pad;
};

/// Parameters for initializing a simulation.
struct MassInitSimulationParams {
  const MassEntityGroupDesc *groups;
  uint32_t num_groups;
  uint32_t _pad0;
  double bounds_min[3];
  double bounds_max[3];
  int32_t random_seed;
  int32_t _pad1;
};

/// One group of spawned entity handles in the init result.
struct MassEntityGroupResult {
  /// Group name (matches the request name).
  Utf8Str name;
  /// Pointer to array of entity handles.
  const MassEntityHandle *handles;
  /// Number of handles.
  uint32_t count;
  uint32_t _pad;
};

/// Result of simulation init: named groups of entity handles.
struct MassInitSimulationResult {
  const MassEntityGroupResult *groups;
  uint32_t num_groups;
  uint32_t _pad;
};

/// Init function type.
using MassInitSimulationFn = uint32_t(*)(const MassInitSimulationParams *params,
                                         MassInitSimulationResult *result);

/// Returns the number of registered spatial query configurations.
using GetSpatialQueryConfigCountFn = uint32_t(*)();

/// Tells C++ how to implement spatial queries for a given entity group.
/// Rust game crates declare these via inventory; C++ reads them at startup
/// to create query callbacks (ISMC overlap or physics sweep).
struct MassSpatialQueryConfigDesc {
  /// Unique query name (e.g. "food_pickup").
  Utf8Str query_name;
  /// ISMC group name to search (e.g. "food").
  Utf8Str query_group;
  /// Overlap sphere radius in Unreal units.
  float radius;
  uint32_t _pad0;
  /// C++ fragment type name that has the bool to filter on (e.g. "FGatherersFood").
  Utf8Str filter_fragment_type;
  /// Byte offset of the bool field within the filter fragment.
  uint32_t filter_bool_offset;
  /// Required value of the bool field (true = only match when bool is true).
  bool filter_bool_must_be;
  /// Query type: 0 = ISMC overlap, 1 = physics sweep.
  uint8_t query_type;
  /// Collision channel index for physics sweep (0 = ECC_GameTraceChannel1).
  uint8_t collision_channel_index;
  uint8_t _pad1;
};

/// Fills a MassSpatialQueryConfigDesc for the config at `index`.
/// Returns 1 on success, 0 on failure.
using GetSpatialQueryConfigDescFn = uint32_t(*)(uint32_t index, MassSpatialQueryConfigDesc *out);

/// Default simulation configuration registered from Rust.
/// C++ reads this at startup; actor UPROPERTY values can override.
struct MassSimDefaultsDesc {
  const MassEntityGroupDesc *groups;
  uint32_t num_groups;
  uint32_t _pad;
  double bounds_min[3];
  double bounds_max[3];
  int32_t random_seed;
  int32_t _pad2;
};

/// Fills the sim defaults from Rust. Returns 1 if defaults are available, 0 otherwise.
using GetSimDefaultsFn = uint32_t(*)(MassSimDefaultsDesc *out);

/// Returns the number of registered Rust-authored tests.
using GetMassTestCountFn = uint32_t(*)();

/// Describes one registered Rust-authored test.
struct MassTestDesc {
  Utf8Str name;
};

/// Fills a MassTestDesc for the test at `index`. Returns 1 on success, 0 on failure.
using GetMassTestDescFn = uint32_t(*)(uint32_t index, MassTestDesc *out);

/// Result of running a Rust-authored test.
struct MassTestResult {
  /// 1 = passed, 0 = failed.
  uint32_t passed;
  /// Length of error message in bytes (0 if passed).
  uint32_t error_len;
  /// Pointer to UTF-8 error message (null if passed).
  /// Owned by Rust — C++ must copy before next FFI call.
  const uint8_t *error_ptr;
};

/// Callback function pointers that C++ provides to Rust test functions.
/// Each callback wraps a URustMassBevySubsystem operation.
struct MassTestCallbacks {
  /// Opaque pointer to C++ test harness (URustMassBevySubsystem* + world context).
  void *opaque;
  uint32_t (*init_sim)(void *ctx, const MassInitSimulationParams *params);
  void (*step_sim)(void *ctx, float dt, uint32_t count);
  void (*reset_sim)(void *ctx);
  void (*tick)(void *ctx, float dt);
  void (*on_rust_reloaded)(void *ctx);
  int32_t (*entity_count)(void *ctx, Utf8Str group);
  uint32_t (*has_managed_sim)(void *ctx);
  uint32_t (*has_spatial_query)(void *ctx, Utf8Str name);
  uint32_t (*read_fragment)(void *ctx,
                            Utf8Str group,
                            uint32_t index,
                            Utf8Str fragment_type,
                            uint8_t *out,
                            uint32_t size);
  uint32_t (*write_fragment)(void *ctx,
                             Utf8Str group,
                             uint32_t index,
                             Utf8Str fragment_type,
                             const uint8_t *data,
                             uint32_t size);
};

/// Runs the named Rust test with the provided callbacks. Returns test result.
using RunMassTestFn = MassTestResult(*)(Utf8Str name, const MassTestCallbacks *callbacks);

/// A food drop event: one food entity was dropped at a new position.
/// C++ reads these after dispatch to call UpdateInstanceTransform on the specific instance.
///
/// The accompanying `GetFoodDropEventsFn` function pointer is supplied by the game crate
/// (not the framework) via `unreal_api::mass::MassExternBinding`, and the underlying cache
/// is drained post-dispatch by a `MassDispatchHook`. See `gatherers-bevy-mass/src/systems.rs`
/// for the reference implementation.
struct FoodDropEvent {
  int32_t food_index;
  int32_t _pad;
  double position[3];
};

/// Copies food drop events into `out` buffer. Returns the number of events written.
/// C++ calls this immediately after mass_frame_dispatch returns.
/// Registered via `unreal_api::mass::MassExternBinding` from the game crate.
using GetFoodDropEventsFn = uint32_t(*)(FoodDropEvent *out, uint32_t max);

/// A food pickup event: one food entity was picked up by an ant.
/// C++ reads these after dispatch to remove the food from the navigation hash grid
/// (so GridHash queries don't return picked-up food).
using GetFoodPickupEventsFn = uint32_t(*)(int32_t *out, uint32_t max);

/// Diagnostic counters for the `ant_food_decision` system. Populated each frame
/// inside the Rust sim; tests read these via the loader's `RustBindings` to
/// measure where the pickup pipeline drops candidates.
struct DecisionCounters {
  uint64_t calls;
  uint64_t hits_seen;
  uint64_t ants_seen;
  uint64_t matched;
  uint64_t pickups;
  uint64_t drops;
  uint64_t no_actions;
};

/// Copies current decision counters into `*out`. Counters are monotonic within
/// a dispatch run and reset by `ResetDecisionCountersFn`.
using GetDecisionCountersFn = void(*)(DecisionCounters *out);

using ResetDecisionCountersFn = void(*)();

struct RustBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  AllocateFn allocate;
  MassBobProcessFn mass_bob_process;
  GetMassSystemCountFn get_mass_system_count;
  GetMassSystemDescriptorFn get_mass_system_descriptor;
  MassFrameDispatchFn mass_frame_dispatch;
  Option<GetVisualizerGroupCountFn> get_visualizer_group_count;
  Option<GetVisualizerGroupDescFn> get_visualizer_group_desc;
  Option<MassInitSimulationFn> mass_init_simulation;
  Option<GetSpatialQueryConfigCountFn> get_spatial_query_config_count;
  Option<GetSpatialQueryConfigDescFn> get_spatial_query_config_desc;
  Option<GetSimDefaultsFn> get_sim_defaults;
  Option<GetMassTestCountFn> get_mass_test_count;
  Option<GetMassTestDescFn> get_mass_test_desc;
  Option<RunMassTestFn> run_mass_test;
  Option<GetFoodDropEventsFn> get_food_drop_events;
  Option<GetFoodPickupEventsFn> get_food_pickup_events;
  Option<GetDecisionCountersFn> get_decision_counters;
  Option<ResetDecisionCountersFn> reset_decision_counters;
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

extern void Log(Utf8Str message, uint8_t level);

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

// =========================================================================
// FFI Layout Verification (auto-generated by unreal-ffi/build.rs)
// These static_asserts catch Rust/C++ struct layout mismatches at compile time.
// Values must match the layout tests in unreal-ffi/src/lib.rs.
// =========================================================================

// --- Option<fn ptr> niche optimization: must be pointer-sized (8 bytes) ---
static_assert(sizeof(Option<SpawnEntitiesFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<MassSpatialQueryFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<GetVisualizerGroupCountFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<GetVisualizerGroupDescFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<MassInitSimulationFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<GetSpatialQueryConfigCountFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<GetSpatialQueryConfigDescFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<GetSimDefaultsFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");

// --- Fundamental types ---
static_assert(sizeof(Utf8Str) == 16, "Utf8Str: ptr(8) + usize(8)");
static_assert(alignof(Utf8Str) == 8, "Utf8Str alignment");
static_assert(sizeof(RustAlloc) == 24, "RustAlloc: ptr(8) + usize(8) + usize(8)");
static_assert(alignof(RustAlloc) == 8, "RustAlloc alignment");
static_assert(sizeof(StrRustAlloc) == 24, "StrRustAlloc wraps RustAlloc");

// --- Function table structs ---
static_assert(sizeof(CoreFns) == 72, "CoreFns: 9 fn ptrs");
static_assert(sizeof(FStringFns) == 24, "FStringFns: 3 fn ptrs");
static_assert(sizeof(FScriptArrayFns) == 104, "FScriptArrayFns: 13 fn ptrs");

// --- Binding structs ---
static_assert(sizeof(UnrealBindings) == 216,
    "UnrealBindings: LogFn(8) + CoreFns(72) + FStringFns(24) + FScriptArrayFns(104) + Option<SpawnEntitiesFn>(8)");
static_assert(sizeof(RustBindings) == 160, "RustBindings: 7 fn ptrs + 13 Option<fn ptr> = 20 pointers");
static_assert(sizeof(PluginBindings) == 32, "PluginBindings: 4 fn ptrs");

// --- Mass Entity types ---
static_assert(sizeof(MassEntityHandle) == 8, "MassEntityHandle: i32 + i32");
static_assert(alignof(MassEntityHandle) == 4, "MassEntityHandle alignment");

// --- Spawn types ---
static_assert(sizeof(SpawnFragmentData) == 32, "SpawnFragmentData");
static_assert(alignof(SpawnFragmentData) == 8, "SpawnFragmentData alignment");
static_assert(sizeof(SpawnTagData) == 16, "SpawnTagData: Utf8Str(16)");
static_assert(alignof(SpawnTagData) == 8, "SpawnTagData alignment");
static_assert(sizeof(SpawnEntityRequest) == 40, "SpawnEntityRequest");
static_assert(alignof(SpawnEntityRequest) == 8, "SpawnEntityRequest alignment");
static_assert(offsetof(SpawnEntityRequest, fragments) == 8, "SpawnEntityRequest.fragments offset");

// --- Mass system descriptors ---
static_assert(sizeof(MassFragmentRequirement) == 32, "MassFragmentRequirement");
static_assert(alignof(MassFragmentRequirement) == 8, "MassFragmentRequirement alignment");
static_assert(offsetof(MassFragmentRequirement, access_mode) == 24, "MassFragmentRequirement.access_mode offset");

static_assert(sizeof(MassSystemDescriptor) == 40, "MassSystemDescriptor");
static_assert(alignof(MassSystemDescriptor) == 8, "MassSystemDescriptor alignment");
static_assert(offsetof(MassSystemDescriptor, requirements) == 24, "MassSystemDescriptor.requirements offset");
static_assert(offsetof(MassSystemDescriptor, execute_fn) == 32, "MassSystemDescriptor.execute_fn offset");

// --- Chunk / slice types ---
static_assert(sizeof(MassFragmentSlice) == 16, "MassFragmentSlice");
static_assert(alignof(MassFragmentSlice) == 8, "MassFragmentSlice alignment");

static_assert(sizeof(MassGlobalChunkSlice) == 16, "MassGlobalChunkSlice");
static_assert(alignof(MassGlobalChunkSlice) == 8, "MassGlobalChunkSlice alignment");

static_assert(sizeof(MassGlobalFragmentChunks) == 24, "MassGlobalFragmentChunks");
static_assert(alignof(MassGlobalFragmentChunks) == 8, "MassGlobalFragmentChunks alignment");

static_assert(sizeof(MassChunkData) == 56, "MassChunkData");
static_assert(alignof(MassChunkData) == 8, "MassChunkData alignment");
static_assert(offsetof(MassChunkData, fragments) == 16, "MassChunkData.fragments offset");
static_assert(offsetof(MassChunkData, global_chunked_fragments) == 48, "MassChunkData.global_chunked_fragments offset");

static_assert(sizeof(MassSystemChunkBatch) == 16, "MassSystemChunkBatch");
static_assert(alignof(MassSystemChunkBatch) == 8, "MassSystemChunkBatch alignment");

// --- Spatial query ---
static_assert(sizeof(MassSpatialQueryResult) == 40, "MassSpatialQueryResult");
static_assert(alignof(MassSpatialQueryResult) == 8, "MassSpatialQueryResult alignment");
static_assert(offsetof(MassSpatialQueryResult, encounter_position) == 8, "MassSpatialQueryResult.encounter_position offset");
static_assert(offsetof(MassSpatialQueryResult, has_encounter) == 32, "MassSpatialQueryResult.has_encounter offset");

// --- Spatial query slot ---
static_assert(sizeof(MassSpatialQuerySlot) == 32, "MassSpatialQuerySlot");
static_assert(alignof(MassSpatialQuerySlot) == 8, "MassSpatialQuerySlot alignment");
static_assert(offsetof(MassSpatialQuerySlot, name) == 0, "MassSpatialQuerySlot.name offset");
static_assert(offsetof(MassSpatialQuerySlot, query_fn) == 16, "MassSpatialQuerySlot.query_fn offset");
static_assert(offsetof(MassSpatialQuerySlot, radius) == 24, "MassSpatialQuerySlot.radius offset");

// --- Frame dispatch ---
static_assert(sizeof(MassFrameDispatchData) == 32, "MassFrameDispatchData");
static_assert(alignof(MassFrameDispatchData) == 8, "MassFrameDispatchData alignment");
static_assert(offsetof(MassFrameDispatchData, num_spatial_queries) == 16, "MassFrameDispatchData.num_spatial_queries offset");
static_assert(offsetof(MassFrameDispatchData, spatial_queries) == 24, "MassFrameDispatchData.spatial_queries offset");

// --- Visualizer ---
static_assert(sizeof(MassVisualizerGroupDesc) == 40, "MassVisualizerGroupDesc");
static_assert(alignof(MassVisualizerGroupDesc) == 8, "MassVisualizerGroupDesc alignment");

// --- Simulation init ---
static_assert(sizeof(MassEntityGroupDesc) == 24, "MassEntityGroupDesc");
static_assert(alignof(MassEntityGroupDesc) == 8, "MassEntityGroupDesc alignment");

static_assert(sizeof(MassInitSimulationParams) == 72, "MassInitSimulationParams");
static_assert(alignof(MassInitSimulationParams) == 8, "MassInitSimulationParams alignment");

static_assert(sizeof(MassEntityGroupResult) == 32, "MassEntityGroupResult");
static_assert(alignof(MassEntityGroupResult) == 8, "MassEntityGroupResult alignment");

static_assert(sizeof(MassInitSimulationResult) == 16, "MassInitSimulationResult");
static_assert(alignof(MassInitSimulationResult) == 8, "MassInitSimulationResult alignment");

// --- Spatial query config ---
static_assert(sizeof(MassSpatialQueryConfigDesc) == 64, "MassSpatialQueryConfigDesc");
static_assert(alignof(MassSpatialQueryConfigDesc) == 8, "MassSpatialQueryConfigDesc alignment");
static_assert(offsetof(MassSpatialQueryConfigDesc, query_name) == 0, "MassSpatialQueryConfigDesc.query_name offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, query_group) == 16, "MassSpatialQueryConfigDesc.query_group offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, radius) == 32, "MassSpatialQueryConfigDesc.radius offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, filter_fragment_type) == 40, "MassSpatialQueryConfigDesc.filter_fragment_type offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, filter_bool_offset) == 56, "MassSpatialQueryConfigDesc.filter_bool_offset offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, filter_bool_must_be) == 60, "MassSpatialQueryConfigDesc.filter_bool_must_be offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, query_type) == 61, "MassSpatialQueryConfigDesc.query_type offset");
static_assert(offsetof(MassSpatialQueryConfigDesc, collision_channel_index) == 62, "MassSpatialQueryConfigDesc.collision_channel_index offset");

// --- Food drop events ---
static_assert(sizeof(FoodDropEvent) == 32, "FoodDropEvent: i32 + i32 + [f64;3]");
static_assert(alignof(FoodDropEvent) == 8, "FoodDropEvent alignment");
static_assert(offsetof(FoodDropEvent, food_index) == 0, "FoodDropEvent.food_index offset");
static_assert(offsetof(FoodDropEvent, position) == 8, "FoodDropEvent.position offset");

// --- Decision counters ---
static_assert(sizeof(DecisionCounters) == 56, "DecisionCounters: 7 u64");
static_assert(alignof(DecisionCounters) == 8, "DecisionCounters alignment");
static_assert(sizeof(Option<GetDecisionCountersFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");
static_assert(sizeof(Option<ResetDecisionCountersFn>) == sizeof(void*),
    "Option<fn ptr> must be pointer-sized (Rust niche optimization)");

// --- Sim defaults ---
static_assert(sizeof(MassSimDefaultsDesc) == 72, "MassSimDefaultsDesc");
static_assert(alignof(MassSimDefaultsDesc) == 8, "MassSimDefaultsDesc alignment");

