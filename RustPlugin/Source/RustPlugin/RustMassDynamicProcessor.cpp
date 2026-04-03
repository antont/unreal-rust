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

	// --- Gather global query data (if any) ---
	TArray<MassFragmentSlice> GlobalFragSlices;
	TArray<int32> GlobalEntityHandles;
	int32 GlobalEntityCount = 0;

	// Source tracking for write-back of mutable global fragments
	struct GlobalChunkSource
	{
		uint8* SrcData;
		int32 Count;
		uint32 Stride;
	};
	TArray<TArray<GlobalChunkSource>> GlobalFragmentSources; // [fragment_idx][chunk_idx]

	if (bHasGlobalQueries)
	{
		GlobalFragSlices.SetNum(GlobalFragmentStructs.Num());
		GlobalFragmentSources.SetNum(GlobalFragmentStructs.Num());

		// First pass: count total entities
		GlobalEntityQuery.ForEachEntityChunk(Context,
			[&GlobalEntityCount](FMassExecutionContext& ChunkContext)
		{
			GlobalEntityCount += ChunkContext.GetNumEntities();
		});

		if (GlobalEntityCount > 0)
		{
			// Allocate contiguous buffers for each global fragment
			TArray<TArray<uint8>> GlobalBuffers;
			GlobalBuffers.SetNum(GlobalFragmentStructs.Num());
			for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
			{
				const uint32 Stride = GlobalFragmentStructs[i]->GetStructureSize();
				GlobalBuffers[i].SetNumUninitialized(GlobalEntityCount * Stride);
			}

			// Allocate entity handle buffer (2 ints per entity)
			GlobalEntityHandles.SetNumUninitialized(GlobalEntityCount * 2);

			// Second pass: copy fragment data + entity handles
			int32 Offset = 0;
			GlobalEntityQuery.ForEachEntityChunk(Context,
				[&](FMassExecutionContext& ChunkContext)
			{
				const int32 NumEntities = ChunkContext.GetNumEntities();

				// Copy entity handles
				for (int32 e = 0; e < NumEntities; ++e)
				{
					FMassEntityHandle Handle = ChunkContext.GetEntity(e);
					GlobalEntityHandles[(Offset + e) * 2 + 0] = Handle.Index;
					GlobalEntityHandles[(Offset + e) * 2 + 1] = Handle.SerialNumber;
				}

				// Copy each global fragment
				for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
				{
					const UScriptStruct* FragStruct = GlobalFragmentStructs[i];
					const uint32 Stride = FragStruct->GetStructureSize();

					TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
					uint8* Src = reinterpret_cast<uint8*>(View.GetData());
					uint8* Dst = GlobalBuffers[i].GetData() + Offset * Stride;
					FMemory::Memcpy(Dst, Src, NumEntities * Stride);

					// Track source for write-back
					GlobalFragmentSources[i].Add({Src, NumEntities, Stride});
				}

				Offset += NumEntities;
			});

			// Build MassFragmentSlice for each global fragment
			for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
			{
				GlobalFragSlices[i].data = GlobalBuffers[i].GetData();
				GlobalFragSlices[i].count = GlobalEntityCount;
				GlobalFragSlices[i].stride = GlobalFragmentStructs[i]->GetStructureSize();
			}

			// --- Iterate primary chunks, passing both chunk + global data to Rust ---
			EntityQuery.ForEachEntityChunk(Context,
				[this, DeltaSeconds, &ChunkFragmentStructs, &ChunkFragmentAccess,
				 &GlobalFragSlices, &GlobalEntityHandles, GlobalEntityCount]
				(FMassExecutionContext& ChunkContext)
			{
				const int32 NumEntities = ChunkContext.GetNumEntities();

				// Build primary fragment slices
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
				ChunkData.global_num_entities = GlobalEntityCount;
				ChunkData.num_global_fragments = GlobalFragSlices.Num();
				ChunkData.global_fragments = GlobalFragSlices.GetData();
				ChunkData.global_entity_handles = GlobalEntityHandles.GetData();

				CachedExecuteFn(&ChunkData);
			});

			// --- Write back mutable global fragments ---
			for (int32 i = 0; i < GlobalFragmentStructs.Num(); ++i)
			{
				if (GlobalFragmentAccessModes[i] != 1) continue; // Skip read-only

				const uint32 Stride = GlobalFragmentStructs[i]->GetStructureSize();
				const uint8* BufferData = reinterpret_cast<const uint8*>(GlobalFragSlices[i].data);
				int32 WriteOffset = 0;

				for (const GlobalChunkSource& Src : GlobalFragmentSources[i])
				{
					FMemory::Memcpy(Src.SrcData, BufferData + WriteOffset * Stride, Src.Count * Stride);
					WriteOffset += Src.Count;
				}
			}
		}
		else
		{
			// No global entities — still iterate primary chunks with empty global data
			EntityQuery.ForEachEntityChunk(Context,
				[this, DeltaSeconds, &ChunkFragmentStructs, &ChunkFragmentAccess]
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
				ChunkData.global_num_entities = 0;
				ChunkData.num_global_fragments = 0;
				ChunkData.global_fragments = nullptr;
				ChunkData.global_entity_handles = nullptr;

				CachedExecuteFn(&ChunkData);
			});
		}
	}
	else
	{
		// No global queries — simple per-chunk iteration (original path)
		EntityQuery.ForEachEntityChunk(Context,
			[this, DeltaSeconds, &ChunkFragmentStructs, &ChunkFragmentAccess](FMassExecutionContext& ChunkContext)
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
			ChunkData.global_num_entities = 0;
			ChunkData.num_global_fragments = 0;
			ChunkData.global_fragments = nullptr;
			ChunkData.global_entity_handles = nullptr;

			CachedExecuteFn(&ChunkData);
		});
	}
}
