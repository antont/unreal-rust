#include "Bindings.h"
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

UE_DISABLE_OPTIMIZATION
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
UE_ENABLE_OPTIMIZATION

void BeginTrace(const char* name)
{
	FCpuProfilerTrace::OutputBeginDynamicEvent(name);
}

void EndTrace()
{
	FCpuProfilerTrace::OutputEndEvent();
}
