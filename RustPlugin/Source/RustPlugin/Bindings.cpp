#include "Bindings.h"
#include "Containers/ScriptArray.h"
#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "RustUtils.h"
#include "Containers/UnrealString.h"
#include "EngineUtils.h"
#include "RustPlugin.h"
#include "RustGameModeBase.h"
#include "Components/PrimitiveComponent.h"
#include "Sound/SoundBase.h"
#include "VisualLogger/VisualLogger.h"

DEFINE_LOG_CATEGORY(RustVisualLog);

namespace
{
static_assert(sizeof(FRustScriptArray) == sizeof(FScriptArray), "FFI script array size must match Unreal FScriptArray");
static_assert(alignof(FRustScriptArray) == alignof(FScriptArray), "FFI script array alignment must match Unreal FScriptArray");
	
static_assert(sizeof(FRustString) == sizeof(FString), "FFI FRustString alignment must match Unreal FString");
static_assert(alignof(FRustString) == alignof(FString), "FFI FRustString alignment must match Unreal FString");

FScriptArray* AsNative(FRustScriptArray* array)
{
	return reinterpret_cast<FScriptArray*>(array);
}

const FScriptArray* AsNative(const FRustScriptArray* array)
{
	return reinterpret_cast<const FScriptArray*>(array);
}
}

void Log(Utf8Str message)
{
	// TODO: Can we get rid of that allocation?
	const FString LogString = ToFString(message);
	UE_LOG(LogTemp, Warning, TEXT("%s"), *LogString);
}

uint32_t GetAllUClasses(RustAlloc* out)
{
	TArray<UClass*> Classes;
	for (TObjectIterator<UClass> It; It; ++It)
	{
		Classes.Add(*It);
	}

	GetRustModule().Plugin.Rust.allocate(Classes.Num() * Classes.GetTypeSize(), 16, out);
	FMemory::Memcpy(out->ptr, Classes.GetData(), out->size);

	return 1;
}

uint32_t GetClassName(const UClassOpague* InClass, StrRustAlloc* out)
{
	UClass* Class = (UClass*)(InClass);

	FString Name = Class->GetPrefixCPP() + Class->GetName();
	auto Utf8 = FTCHARToUTF8(*Name);
	GetRustModule().Plugin.Rust.allocate(Utf8.Length(), 1, &out->alloc);
	FMemory::Memcpy(out->alloc.ptr, Utf8.Get(), out->alloc.size);

	return 1;
}

UE_DISABLE_OPTIMIZATION

uint32_t ProcessEventFromRust(UObjectOpague* object_opague, UFunctionOpague* function_opague, void* buffer)
{
	UObject* Object = (UObject*)(object_opague);
	UFunction* Function = (UFunction*)(function_opague);

	if (Object == nullptr || Function == nullptr)
	{
		return 0;
	}
	Object->ProcessEvent(Function, buffer);

	return 1;
}

UE_ENABLE_OPTIMIZATION

uint32_t GetCDOFromClass(const UClassOpague* cdo_opague, UObjectOpague** out_object)
{
	const UClass* Class = static_cast<const UClass*>(cdo_opague);

	if (Class == nullptr || out_object == nullptr)
	{
		return 0;
	}

	auto CDO = Class->GetDefaultObject<UObject>();

	(*out_object) = static_cast<UObjectOpague*>(CDO);

	return 1;
}

uint32_t InitializeValuesInParamBuffer(const UFunctionOpague* function_opague, void* buffer)
{
	const UFunction* Function = static_cast<const UFunction*>(function_opague);

	if (Function == nullptr || buffer == nullptr)
	{
		return 0;
	}

	for (TFieldIterator<FProperty> It(Function); It; ++It)
	{
		It->InitializeValue_InContainer(buffer);
	}

	return 1;
}

uint32_t DestroyValuesInParamBuffer(const UFunctionOpague* function_opague, void* buffer)
{
	const UFunction* Function = static_cast<const UFunction*>(function_opague);

	if (Function == nullptr || buffer == nullptr)
	{
		return 0;
	}

	for (TFieldIterator<FProperty> It(Function); It; ++It)
	{
		It->DestroyValue_InContainer(buffer);
	}

	return 1;
}

uint32_t FindFunctionByName(const UClassOpague* cdo_opague, Utf8Str name, UFunctionOpague** function_opague)
{
	const UClass* Class = static_cast<const UClass*>(cdo_opague);

	if (Class == nullptr)
	{
		return 0;
	}

	FString FunctionNameString = ToFString(name);
	FName FunctionName = FName(*FunctionNameString);

	UFunction* Function = Class->FindFunctionByName(FunctionName);

	if (Function == nullptr)
	{
		return 0;
	}

	(*function_opague) = static_cast<UFunctionOpague*>(Function);

	return 1;
}

uint32_t FScriptArrayNum(const FRustScriptArray* array, int32_t* out_num)
{
	if (array == nullptr || out_num == nullptr)
	{
		return 0;
	}

	const FScriptArray* ScriptArray = AsNative(array);
	*out_num = ScriptArray->Num();
	return 1;
}

uint32_t FScriptArrayCtor(FRustScriptArray* array)
{
	if (array == nullptr)
	{
		return 0;
	}

	new (array) FScriptArray();
	return 1;
}

uint32_t FScriptArrayDtor(FRustScriptArray* array)
{
	if (array == nullptr)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	ScriptArray->~FScriptArray();
	return 1;
}

uint32_t FScriptArrayMax(const FRustScriptArray* array, int32_t* out_max)
{
	if (array == nullptr || out_max == nullptr)
	{
		return 0;
	}

	const FScriptArray* ScriptArray = AsNative(array);
	*out_max = ScriptArray->Max();
	return 1;
}

uint32_t FScriptArrayGetData(FRustScriptArray* array, void** out_data)
{
	if (array == nullptr || out_data == nullptr)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	*out_data = ScriptArray->GetData();
	return 1;
}

uint32_t FScriptArrayIsValidIndex(const FRustScriptArray* array, int32_t index)
{
	if (array == nullptr)
	{
		return 0;
	}

	const FScriptArray* ScriptArray = AsNative(array);
	return ScriptArray->IsValidIndex(index) ? 1 : 0;
}

uint32_t FScriptArrayReserve(FRustScriptArray* array,
	int32_t capacity,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || capacity < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	const int32_t OldNum = ScriptArray->Num();
	if (capacity > ScriptArray->Max())
	{
		const int32_t GrowCount = capacity - OldNum;
		if (GrowCount > 0)
		{
			ScriptArray->Add(GrowCount, elem_size, elem_align);
			ScriptArray->Remove(OldNum, GrowCount, elem_size, elem_align, EAllowShrinking::No);
		}
	}
	return 1;
}

uint32_t FScriptArrayAdd(FRustScriptArray* array,
	int32_t count,
	int32_t elem_size,
	uint32_t elem_align,
	int32_t* out_index)
{
	if (array == nullptr || count < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	const int32_t NewIndex = ScriptArray->Add(count, elem_size, elem_align);

	if (out_index != nullptr)
	{
		*out_index = NewIndex;
	}

	return 1;
}

uint32_t FScriptArrayInsert(FRustScriptArray* array,
	int32_t index,
	int32_t count,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || index < 0 || count < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	if (index > ScriptArray->Num())
	{
		return 0;
	}

	ScriptArray->Insert(index, count, elem_size, elem_align);
	return 1;
}

uint32_t FScriptArrayRemove(FRustScriptArray* array,
	int32_t index,
	int32_t count,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || index < 0 || count < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	if (index + count > ScriptArray->Num())
	{
		return 0;
	}

	ScriptArray->Remove(index, count, elem_size, elem_align);
	return 1;
}

uint32_t FScriptArrayEmpty(FRustScriptArray* array,
	int32_t slack,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || slack < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	ScriptArray->Empty(slack, elem_size, elem_align);
	return 1;
}

uint32_t FScriptArrayReset(FRustScriptArray* array,
	int32_t new_size,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || new_size < 0 || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	ScriptArray->Reset(new_size, elem_size, elem_align);
	return 1;
}

uint32_t FScriptArrayShrink(FRustScriptArray* array,
	int32_t elem_size,
	uint32_t elem_align)
{
	if (array == nullptr || elem_size <= 0 || elem_align == 0)
	{
		return 0;
	}

	FScriptArray* ScriptArray = AsNative(array);
	ScriptArray->Shrink(elem_size, elem_align);
	return 1;
}

void BeginTrace(const char* name)
{
	FCpuProfilerTrace::OutputBeginDynamicEvent(name);
}

void EndTrace()
{
	FCpuProfilerTrace::OutputEndEvent();
}

void NewFStringFromUtf8(Utf8Str str, FRustString *fstring)
{
	FString* Out = reinterpret_cast<FString*>(fstring);
	// TODO: Avoid allocation
    new (Out) FString(ToFString(str));
}

void CopyFromFString(const FRustString* source, FRustString* fstring)
{
	if (source == nullptr || fstring == nullptr)
	{
		return;
	}

	const FString* In = reinterpret_cast<const FString*>(source);
	FString* Out = reinterpret_cast<FString*>(fstring);
	new (Out) FString(*In);
}

void FStringDtor(FRustString* fstring)
{
	if (fstring == nullptr)
	{
		return;
	}

	FString* InOut = reinterpret_cast<FString*>(fstring);
	InOut->~FString();
}
