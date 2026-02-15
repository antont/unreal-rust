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

using FScriptArrayOpaque = void;

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

using FScriptArrayNumFn = uint32_t(*)(const FScriptArrayOpaque *array, int32_t *out_num);

using FScriptArrayMaxFn = uint32_t(*)(const FScriptArrayOpaque *array, int32_t *out_max);

using FScriptArrayGetDataFn = uint32_t(*)(FScriptArrayOpaque *array, void **out_data);

using FScriptArrayIsValidIndexFn = uint32_t(*)(const FScriptArrayOpaque *array, int32_t index);

using FScriptArrayReserveFn = uint32_t(*)(FScriptArrayOpaque *array,
                                          int32_t capacity,
                                          int32_t elem_size,
                                          uint32_t elem_align);

using FScriptArrayAddFn = uint32_t(*)(FScriptArrayOpaque *array,
                                      int32_t count,
                                      int32_t elem_size,
                                      uint32_t elem_align,
                                      int32_t *out_index);

using FScriptArrayInsertFn = uint32_t(*)(FScriptArrayOpaque *array,
                                         int32_t index,
                                         int32_t count,
                                         int32_t elem_size,
                                         uint32_t elem_align);

using FScriptArrayRemoveFn = uint32_t(*)(FScriptArrayOpaque *array,
                                         int32_t index,
                                         int32_t count,
                                         int32_t elem_size,
                                         uint32_t elem_align);

using FScriptArrayEmptyFn = uint32_t(*)(FScriptArrayOpaque *array,
                                        int32_t slack,
                                        int32_t elem_size,
                                        uint32_t elem_align);

using FScriptArrayResetFn = uint32_t(*)(FScriptArrayOpaque *array,
                                        int32_t new_size,
                                        int32_t elem_size,
                                        uint32_t elem_align);

using FScriptArrayShrinkFn = uint32_t(*)(FScriptArrayOpaque *array,
                                         int32_t elem_size,
                                         uint32_t elem_align);

struct FScriptArrayFns {
  FScriptArrayNumFn num;
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
  FScriptArrayFns fscript_array_fns;
};

using TickFn = ResultCode(*)(float dt);

using BeginPlayFn = ResultCode(*)();

using AllocateFn = uint32_t(*)(uintptr_t size, uintptr_t align, RustAlloc *ptr);

struct RustBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  AllocateFn allocate;
};

using EntryUnrealBindingsFn = uint32_t(*)(UnrealBindings bindings, RustBindings *rust_bindings);

using TryLoadFn = uint32_t(*)(RustBindings*);

struct PluginBindings {
  TickFn tick;
  BeginPlayFn begin_play;
  TryLoadFn try_load;
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

extern uint32_t FScriptArrayNum(const FScriptArrayOpaque *array, int32_t *out_num);

extern uint32_t FScriptArrayMax(const FScriptArrayOpaque *array, int32_t *out_max);

extern uint32_t FScriptArrayGetData(FScriptArrayOpaque *array, void **out_data);

extern uint32_t FScriptArrayIsValidIndex(const FScriptArrayOpaque *array, int32_t index);

extern uint32_t FScriptArrayReserve(FScriptArrayOpaque *array,
                                    int32_t capacity,
                                    int32_t elem_size,
                                    uint32_t elem_align);

extern uint32_t FScriptArrayAdd(FScriptArrayOpaque *array,
                                int32_t count,
                                int32_t elem_size,
                                uint32_t elem_align,
                                int32_t *out_index);

extern uint32_t FScriptArrayInsert(FScriptArrayOpaque *array,
                                   int32_t index,
                                   int32_t count,
                                   int32_t elem_size,
                                   uint32_t elem_align);

extern uint32_t FScriptArrayRemove(FScriptArrayOpaque *array,
                                   int32_t index,
                                   int32_t count,
                                   int32_t elem_size,
                                   uint32_t elem_align);

extern uint32_t FScriptArrayEmpty(FScriptArrayOpaque *array,
                                  int32_t slack,
                                  int32_t elem_size,
                                  uint32_t elem_align);

extern uint32_t FScriptArrayReset(FScriptArrayOpaque *array,
                                  int32_t new_size,
                                  int32_t elem_size,
                                  uint32_t elem_align);

extern uint32_t FScriptArrayShrink(FScriptArrayOpaque *array,
                                   int32_t elem_size,
                                   uint32_t elem_align);

}  // extern "C"
