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
	GlobalEntityQuery.RegisterWithProcessor(*this);
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

	// Invalidate cached chunk pointers — chunks may have moved since last play session
	bChunkCacheValid = false;
	CachedPrimarySlices.Empty();
	CachedPrimaryChunks.Empty();
	CachedChunkSlices.Empty();
	CachedChunkedFrags.Empty();
	CachedGlobalEntityCount = 0;

	// Requirements only need to be added once; they persist across pipeline re-initializations
	if (bQueriesConfigured)
	{
		return;
	}
	bQueriesConfigured = true;

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
	}
}

void URustMassDynamicProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	if (!bInitialized || CachedExecuteFn == nullptr)
	{
		return;
	}

	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	// --- First frame: cache all chunk pointers (entities are stable, so these never change) ---
	if (!bChunkCacheValid)
	{
		// Build filtered primary fragment metadata (non-tag only)
		PrimaryFragmentStructs.Empty();
		PrimaryFragmentAccess.Empty();
		for (int32 i = 0; i < FragmentStructs.Num(); ++i)
		{
			if (!FragmentIsTags[i])
			{
				PrimaryFragmentStructs.Add(FragmentStructs[i]);
				PrimaryFragmentAccess.Add(FragmentAccessModes[i]);
			}
		}

		// Cache primary chunk pointers
		CachedPrimarySlices.Empty();
		CachedPrimaryChunks.Empty();

		EntityQuery.ForEachEntityChunk(Context,
			[this](FMassExecutionContext& ChunkContext)
		{
			const int32 NumEntities = ChunkContext.GetNumEntities();
			const int32 ChunkIdx = CachedPrimarySlices.Num();

			TArray<MassFragmentSlice>& FragSlices = CachedPrimarySlices.AddDefaulted_GetRef();
			FragSlices.SetNum(PrimaryFragmentStructs.Num());

			for (int32 i = 0; i < PrimaryFragmentStructs.Num(); ++i)
			{
				const UScriptStruct* FragStruct = PrimaryFragmentStructs[i];
				if (PrimaryFragmentAccess[i] == 1)
				{
					TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
					FragSlices[i].data = View.GetData();
					FragSlices[i].count = View.Num();
				}
				else
				{
					TConstArrayView<FMassFragment> View = ChunkContext.GetFragmentView(FragStruct);
					FragSlices[i].data = const_cast<FMassFragment*>(View.GetData());
					FragSlices[i].count = View.Num();
				}
				FragSlices[i].stride = FragStruct->GetStructureSize();
			}

			MassChunkData& Chunk = CachedPrimaryChunks.AddDefaulted_GetRef();
			Chunk.num_entities = NumEntities;
			Chunk.dt = 0.0f; // updated per frame
			Chunk.num_fragments = FragSlices.Num();
			Chunk.fragments = FragSlices.GetData();
			Chunk.global_num_entities = 0;
			Chunk.num_global_fragments = 0;
			Chunk.global_fragments = nullptr;
			Chunk.global_entity_handles = nullptr;
			Chunk.global_chunked_fragments = nullptr;
		});

		// Cache global chunk pointers (if any)
		if (bHasGlobalQueries)
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
					void* Data;
					if (GlobalFragmentAccessModes[i] == 1)
					{
						Data = ChunkContext.GetMutableFragmentView(FragStruct).GetData();
					}
					else
					{
						Data = const_cast<FMassFragment*>(ChunkContext.GetFragmentView(FragStruct).GetData());
					}

					MassGlobalChunkSlice Slice;
					Slice.data = Data;
					Slice.count = NumEntities;
					Slice.stride = FragStruct->GetStructureSize();
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

			// Wire global data into each cached primary chunk
			for (MassChunkData& Chunk : CachedPrimaryChunks)
			{
				Chunk.global_num_entities = CachedGlobalEntityCount;
				Chunk.num_global_fragments = CachedChunkedFrags.Num();
				Chunk.global_chunked_fragments = CachedChunkedFrags.GetData();
			}
		}

		bChunkCacheValid = true;
	}

	// --- Every frame: update dt ---
	for (MassChunkData& Chunk : CachedPrimaryChunks)
	{
		Chunk.dt = DeltaSeconds;
	}

	// In cache-only mode, the coordinator will dispatch all systems together.
	// In direct mode, call Rust per cached chunk as before.
	if (!bCacheOnly)
	{
		for (MassChunkData& Chunk : CachedPrimaryChunks)
		{
			CachedExecuteFn(&Chunk);
		}
	}
}
