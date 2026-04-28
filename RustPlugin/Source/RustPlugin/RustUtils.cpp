#include "RustUtils.h"
#include "Modules/ModuleManager.h"
#include "RustPlugin.h"
#include "MassEntityManager.h"
#include "StructUtils/InstancedStruct.h"

// ---------------------------------------------------------------------------
// Entity spawning callback for Rust
// ---------------------------------------------------------------------------

namespace RustMassSpawn
{
	// Set by the subsystem before calling into Rust for spawning.
	static FMassEntityManager* CachedEntityManager = nullptr;

	uint32_t SpawnEntitiesImpl(
		const SpawnEntityRequest* Request,
		MassEntityHandle* OutHandles)
	{
		if (!Request || !CachedEntityManager)
		{
			return 0;
		}

		const uint32_t Count = Request->count;
		if (Count == 0)
		{
			return 0;
		}

		// Resolve fragment UScriptStructs
		TArray<const UScriptStruct*> FragmentStructs;
		FragmentStructs.SetNum(Request->num_fragments);
		for (uint32_t i = 0; i < Request->num_fragments; ++i)
		{
			const SpawnFragmentData& Frag = Request->fragments[i];
			FString TypeName = ToFString(Frag.cpp_type_name);
			// Strip F/U prefix — UE reflection registers structs without the prefix
			FString SearchName = TypeName;
			if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
			{
				SearchName.RightChopInline(1);
			}
			const UScriptStruct* Struct = FindFirstObject<UScriptStruct>(*SearchName, EFindFirstObjectOptions::NativeFirst);
			if (!Struct)
			{
				UE_LOG(LogTemp, Error, TEXT("RustMassSpawn: Could not find UScriptStruct '%s'"), *TypeName);
				return 0;
			}
			FragmentStructs[i] = Struct;
		}

		// Resolve tag UScriptStructs
		TArray<const UScriptStruct*> TagStructs;
		TagStructs.SetNum(Request->num_tags);
		for (uint32_t i = 0; i < Request->num_tags; ++i)
		{
			FString TypeName = ToFString(Request->tags[i].cpp_type_name);
			FString TagSearchName = TypeName;
			if (TagSearchName.Len() > 1 && (TagSearchName[0] == TEXT('F') || TagSearchName[0] == TEXT('U')))
			{
				TagSearchName.RightChopInline(1);
			}
			const UScriptStruct* Struct = FindFirstObject<UScriptStruct>(*TagSearchName, EFindFirstObjectOptions::NativeFirst);
			if (!Struct)
			{
				UE_LOG(LogTemp, Error, TEXT("RustMassSpawn: Could not find tag UScriptStruct '%s'"), *TypeName);
				return 0;
			}
			TagStructs[i] = Struct;
		}

		// Spawn entities one by one (CreateEntity returns handle)
		uint32_t Spawned = 0;
		for (uint32_t EntityIdx = 0; EntityIdx < Count; ++EntityIdx)
		{
			TArray<FInstancedStruct> Fragments;
			Fragments.Reserve(Request->num_fragments);

			for (uint32_t FragIdx = 0; FragIdx < Request->num_fragments; ++FragIdx)
			{
				const SpawnFragmentData& Frag = Request->fragments[FragIdx];
				FInstancedStruct Instance;
				Instance.InitializeAs(FragmentStructs[FragIdx]);

				// If Rust provided initial data, memcpy it over the default
				if (Frag.initial_data)
				{
					const uint8_t* SrcBase = static_cast<const uint8_t*>(Frag.initial_data);
					FMemory::Memcpy(
						Instance.GetMutableMemory(),
						SrcBase + EntityIdx * Frag.size,
						Frag.size);
				}

				Fragments.Add(MoveTemp(Instance));
			}

			const FMassEntityHandle Handle = CachedEntityManager->CreateEntity(Fragments);

			// Re-write fragment data after spawn to undo any observer mutation.
			// UE's UMassRandomVelocityInitializer auto-registers as an Add observer
			// on FMassVelocityFragment and reinterprets Value.X/Y/Z as (MinSpeed,
			// MaxSpeed, bSetZComponent). Our spawn-time velocities are real
			// direction*speed vectors, so letting the observer run corrupts them.
			// SetEntityFragmentValues writes directly to chunk memory without
			// firing observers, restoring our authored values.
			if (Fragments.Num() > 0)
			{
				CachedEntityManager->SetEntityFragmentValues(Handle, Fragments);
			}

			// Add tags
			for (const UScriptStruct* TagStruct : TagStructs)
			{
				CachedEntityManager->AddTagToEntity(Handle, TagStruct);
			}

			if (OutHandles)
			{
				OutHandles[Spawned].index = Handle.Index;
				OutHandles[Spawned].serial_number = Handle.SerialNumber;
			}
			++Spawned;
		}

		return Spawned;
	}
}

void RustMassSpawnSetEntityManager(FMassEntityManager* Manager)
{
	RustMassSpawn::CachedEntityManager = Manager;
}

UnrealBindings CreateBindings()
{
	CoreFns core_fns = {};
	core_fns.get_all_uclasses = &GetAllUClasses;
	core_fns.get_class_name = &GetClassName;
	core_fns.get_cdo_from_class = &GetCDOFromClass;
	core_fns.find_function_by_name = &FindFunctionByName;
	core_fns.initialize_values_in_param_buffer = &InitializeValuesInParamBuffer;
	core_fns.destroy_values_in_param_buffer = &DestroyValuesInParamBuffer;
	core_fns.process_event = &ProcessEventFromRust;
	core_fns.end_trace = &EndTrace;
	core_fns.begin_trace = &BeginTrace;

	FStringFns fstring_fns = {};
	fstring_fns.new_fstring_from_utf8 = &NewFStringFromUtf8;
	fstring_fns.copy_from_fstring = &CopyFromFString;
	fstring_fns.dtor = &FStringDtor;

	FScriptArrayFns fscript_array_fns = {};
	fscript_array_fns.num = &FScriptArrayNum;
	fscript_array_fns.ctor = &FScriptArrayCtor;
	fscript_array_fns.dtor = &FScriptArrayDtor;
	fscript_array_fns.max = &FScriptArrayMax;
	fscript_array_fns.get_data = &FScriptArrayGetData;
	fscript_array_fns.is_valid_index = &FScriptArrayIsValidIndex;
	fscript_array_fns.reserve = &FScriptArrayReserve;
	fscript_array_fns.add = &FScriptArrayAdd;
	fscript_array_fns.insert = &FScriptArrayInsert;
	fscript_array_fns.remove = &FScriptArrayRemove;
	fscript_array_fns.empty = &FScriptArrayEmpty;
	fscript_array_fns.reset = &FScriptArrayReset;
	fscript_array_fns.shrink = &FScriptArrayShrink;

	UnrealBindings b = {};
	b.core_fns = core_fns;
	b.fstring_fns = fstring_fns;
	b.fscript_array_fns = fscript_array_fns;
	b.log = &Log;
	b.spawn_entities = &RustMassSpawn::SpawnEntitiesImpl;
	return b;
}

FRustPluginModule& GetRustModule()
{
	return FModuleManager::LoadModuleChecked<FRustPluginModule>(TEXT("RustPlugin"));
}

FString ToFString(Utf8Str Str)
{
	if (Str.len == 0)
	{
		return FString();
	}

	return FString(Str.len, StringCast<TCHAR>(reinterpret_cast<const UTF8CHAR*>(Str.ptr), Str.len).Get());
}
