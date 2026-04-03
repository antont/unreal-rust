#include "RustMassDynamicProcessor.h"

#include "MassEntityManager.h"
#include "MassExecutionContext.h"

URustMassDynamicProcessor::URustMassDynamicProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	bAllowMultipleInstances = true;
	ExecutionFlags = static_cast<uint8>(EProcessorExecutionFlags::All);
	EntityQuery.RegisterWithProcessor(*this);
}

static FString Utf8StrToFString(const Utf8Str& Str)
{
	FUTF8ToTCHAR Converter(Str.ptr, static_cast<int32>(Str.len));
	return FString(Converter.Length(), Converter.Get());
}

void URustMassDynamicProcessor::InitFromDescriptor(const MassSystemDescriptor& Descriptor)
{
	SystemName = Utf8StrToFString(Descriptor.name);
	CachedExecuteFn = Descriptor.execute_fn;

	for (uint32 i = 0; i < Descriptor.num_requirements; ++i)
	{
		const MassFragmentRequirement& Req = Descriptor.requirements[i];
		FString CppTypeName = Utf8StrToFString(Req.cpp_type_name);

		// Strip the F/U prefix — UE reflection registers structs without the prefix
		FString SearchName = CppTypeName;
		if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
		{
			SearchName.RightChopInline(1);
		}

		// Find the UScriptStruct by name at runtime
		UScriptStruct* FoundStruct = FindFirstObject<UScriptStruct>(*SearchName, EFindFirstObjectOptions::NativeFirst);
		if (FoundStruct == nullptr)
		{
			UE_LOG(LogTemp, Error,
				TEXT("RustMassDynamicProcessor [%s]: Could not find UScriptStruct '%s'"),
				*SystemName, *CppTypeName);
			return;
		}

		if (Req.query_scope == 1)
		{
			// Global query requirement
			GlobalFragmentStructs.Add(FoundStruct);
			GlobalFragmentAccessModes.Add(Req.access_mode);
			bHasGlobalQueries = true;
		}
		else
		{
			// Primary query requirement
			FragmentStructs.Add(FoundStruct);
			FragmentAccessModes.Add(Req.access_mode);
			FragmentIsTags.Add(Req.is_tag);
		}
	}

	bInitialized = true;
	UE_LOG(LogTemp, Log,
		TEXT("RustMassDynamicProcessor [%s]: Initialized with %d primary + %d global requirements"),
		*SystemName, FragmentStructs.Num(), GlobalFragmentStructs.Num());
}

TArray<URustMassDynamicProcessor*> URustMassDynamicProcessor::CreateAllRustProcessors(
	const RustBindings& Bindings,
	UObject* Outer)
{
	TArray<URustMassDynamicProcessor*> Processors;

	if (Bindings.get_mass_system_count == nullptr || Bindings.get_mass_system_descriptor == nullptr)
	{
		return Processors;
	}

	const uint32 Count = Bindings.get_mass_system_count();
	UE_LOG(LogTemp, Log, TEXT("RustMassDynamicProcessor: Discovered %u Rust mass systems"), Count);

	for (uint32 i = 0; i < Count; ++i)
	{
		MassSystemDescriptor Descriptor = {};
		if (Bindings.get_mass_system_descriptor(i, &Descriptor) == 0)
		{
			UE_LOG(LogTemp, Warning,
				TEXT("RustMassDynamicProcessor: Failed to get descriptor for system %u"), i);
			continue;
		}

		URustMassDynamicProcessor* Processor = NewObject<URustMassDynamicProcessor>(Outer);
		Processor->InitFromDescriptor(Descriptor);

		if (Processor->bInitialized)
		{
			Processors.Add(Processor);
		}
	}

	return Processors;
}

void URustMassDynamicProcessor::AddExtraTagRequirement(const UScriptStruct* TagStruct)
{
	if (TagStruct != nullptr)
	{
		ExtraTagRequirements.AddUnique(TagStruct);
	}
}

void URustMassDynamicProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	if (!bInitialized)
	{
		return;
	}

	// Configure primary query
	for (int32 i = 0; i < FragmentStructs.Num(); ++i)
	{
		const UScriptStruct* FragStruct = FragmentStructs[i];

		if (FragmentIsTags[i])
		{
			EntityQuery.AddTagRequirement(FragStruct, EMassFragmentPresence::All);
		}
		else
		{
			const EMassFragmentAccess Access = (FragmentAccessModes[i] == 1)
				? EMassFragmentAccess::ReadWrite
				: EMassFragmentAccess::ReadOnly;
			EntityQuery.AddRequirement(FragStruct, Access);
		}
	}

	// Append extra tag requirements added by the subsystem
	for (const UScriptStruct* ExtraTag : ExtraTagRequirements)
	{
		EntityQuery.AddTagRequirement(ExtraTag, EMassFragmentPresence::All);
	}

	// Configure global query (if any)
	if (bHasGlobalQueries)
	{
		for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
		{
			const UScriptStruct* FragStruct = GlobalFragmentStructs[i];
			const EMassFragmentAccess Access = (GlobalFragmentAccessModes[i] == 1)
				? EMassFragmentAccess::ReadWrite
				: EMassFragmentAccess::ReadOnly;
			GlobalEntityQuery.AddRequirement(FragStruct, Access);
		}
		GlobalEntityQuery.RegisterWithProcessor(*this);
	}
}

void URustMassDynamicProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	if (!bInitialized || CachedExecuteFn == nullptr)
	{
		return;
	}

	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	// --- Gather primary (per-chunk) fragment info ---
	TArray<const UScriptStruct*> ChunkFragmentStructs;
	TArray<uint8> ChunkFragmentAccess;
	for (int32 i = 0; i < FragmentStructs.Num(); ++i)
	{
		if (!FragmentIsTags[i])
		{
			ChunkFragmentStructs.Add(FragmentStructs[i]);
			ChunkFragmentAccess.Add(FragmentAccessModes[i]);
		}
	}

	// --- Build global chunk descriptors (zero-copy, cached after first frame) ---
	if (bHasGlobalQueries && !bGlobalCacheValid)
	{
		CachedChunkSlices.SetNum(GlobalFragmentStructs.Num());
		CachedChunkedFrags.SetNum(GlobalFragmentStructs.Num());
		CachedGlobalEntityCount = 0;

		GlobalEntityQuery.ForEachEntityChunk(Context,
			[this](FMassExecutionContext& ChunkContext)
		{
			const int32 NumEntities = ChunkContext.GetNumEntities();
			CachedGlobalEntityCount += NumEntities;

			for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
			{
				const UScriptStruct* FragStruct = GlobalFragmentStructs[i];
				const uint32 Stride = FragStruct->GetStructureSize();

				void* ChunkData;
				if (GlobalFragmentAccessModes[i] == 1)
				{
					TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
					ChunkData = View.GetData();
				}
				else
				{
					TConstArrayView<FMassFragment> View = ChunkContext.GetFragmentView(FragStruct);
					ChunkData = const_cast<FMassFragment*>(View.GetData());
				}

				MassGlobalChunkSlice Slice;
				Slice.data = ChunkData;
				Slice.count = NumEntities;
				Slice.stride = Stride;
				CachedChunkSlices[i].Add(Slice);
			}
		});

		for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
		{
			CachedChunkedFrags[i].chunks = CachedChunkSlices[i].GetData();
			CachedChunkedFrags[i].num_chunks = CachedChunkSlices[i].Num();
			CachedChunkedFrags[i].total_count = CachedGlobalEntityCount;
			CachedChunkedFrags[i].stride = GlobalFragmentStructs[i]->GetStructureSize();
		}

		bGlobalCacheValid = true;
	}

	// --- Helper lambda to build primary fragment slices and call Rust ---
	auto IteratePrimaryChunks = [this, DeltaSeconds, &ChunkFragmentStructs, &ChunkFragmentAccess]
		(FMassExecutionContext& ChunkContext)
	{
		const int32 NumEntities = ChunkContext.GetNumEntities();

		TArray<MassFragmentSlice> FragSlices;
		FragSlices.SetNum(ChunkFragmentStructs.Num());
		for (int32 i = 0; i < ChunkFragmentStructs.Num(); ++i)
		{
			const UScriptStruct* FragStruct = ChunkFragmentStructs[i];
			if (ChunkFragmentAccess[i] == 1)
			{
				TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
				FragSlices[i].data = View.GetData();
				FragSlices[i].count = View.Num();
				FragSlices[i].stride = FragStruct->GetStructureSize();
			}
			else
			{
				TConstArrayView<FMassFragment> View = ChunkContext.GetFragmentView(FragStruct);
				FragSlices[i].data = const_cast<FMassFragment*>(View.GetData());
				FragSlices[i].count = View.Num();
				FragSlices[i].stride = FragStruct->GetStructureSize();
			}
		}

		MassChunkData ChunkData;
		ChunkData.num_entities = NumEntities;
		ChunkData.dt = DeltaSeconds;
		ChunkData.num_fragments = FragSlices.Num();
		ChunkData.fragments = FragSlices.GetData();

		if (bHasGlobalQueries)
		{
			ChunkData.global_num_entities = CachedGlobalEntityCount;
			ChunkData.num_global_fragments = CachedChunkedFrags.Num();
			ChunkData.global_fragments = nullptr;
			ChunkData.global_entity_handles = nullptr;
			ChunkData.global_chunked_fragments = CachedChunkedFrags.GetData();
		}
		else
		{
			ChunkData.global_num_entities = 0;
			ChunkData.num_global_fragments = 0;
			ChunkData.global_fragments = nullptr;
			ChunkData.global_entity_handles = nullptr;
			ChunkData.global_chunked_fragments = nullptr;
		}

		CachedExecuteFn(&ChunkData);
	};

	EntityQuery.ForEachEntityChunk(Context, IteratePrimaryChunks);
}
