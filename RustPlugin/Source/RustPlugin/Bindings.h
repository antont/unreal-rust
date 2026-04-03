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

struct UnrealBindings {
  LogFn log;
  CoreFns core_fns;
  FStringFns fstring_fns;
  FScriptArrayFns fscript_array_fns;
};

using TickFn = ResultCode(*)(float dt);

using BeginPlayFn = ResultCode(*)();

using AllocateFn = uint32_t(*)(uintptr_t size, uintptr_t align, RustAlloc *ptr);

using MassBobProcessFn = void(*)(void *data, int32_t count, float dt);

using MassAntMovementFn = void(*)(void *ants,
                                  int32_t count,
                                  float dt,
                                  const double *bounds_min,
                                  const double *bounds_max);

using MassAntFoodDecisionFn = int32_t(*)(void *ant, const void *encounter, int32_t has_encounter);

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

/// Chunk data passed from C++ Execute to a Rust mass system function.
struct MassChunkData {
  /// Number of entities in this chunk.
  int32_t num_entities;
  /// Delta time for this simulation step.
  float dt;
  /// Number of fragment slices (matches system's requirement count).
  uint32_t num_fragments;
  /// Pointer to array of MassFragmentSlice, one per requirement in declaration order.
  const MassFragmentSlice *fragments;
};

/// Execute function signature for a dynamically registered mass system.
using MassSystemExecuteFn = void(*)(const MassChunkData *chunk);

/// Describes one registered Rust mass system. C++ queries these at init time.
struct MassSystemDescriptor {
  /// Unique system name (e.g. "ant_movement").
  Utf8Str name;
  /// Number of fragment/tag requirements.
  uint32_t num_requirements;
  /// Pointer to array of MassFragmentRequirement.
  const MassFragmentRequirement *requirements;
  /// The Rust function to call during Execute.
  MassSystemExecuteFn execute_fn;
};

/// Fills a MassSystemDescriptor for the system at `index`. Returns 1 on success, 0 on failure.
using GetMassSystemDescriptorFn = uint32_t(*)(uint32_t index, MassSystemDescriptor *out);

struct RustBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  AllocateFn allocate;
  MassBobProcessFn mass_bob_process;
  MassAntMovementFn mass_ant_movement;
  MassAntFoodDecisionFn mass_ant_food_decision;
  GetMassSystemCountFn get_mass_system_count;
  GetMassSystemDescriptorFn get_mass_system_descriptor;
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
