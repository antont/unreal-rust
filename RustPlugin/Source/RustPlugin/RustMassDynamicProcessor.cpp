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

void URustMassDynamicProcessor::InitFromDescriptor(const MassSystemDescriptor& Descriptor)
{
	SystemName = FString(UTF8_TO_TCHAR(Descriptor.name.ptr));
	CachedExecuteFn = Descriptor.execute_fn;

	for (uint32 i = 0; i < Descriptor.num_requirements; ++i)
	{
		const MassFragmentRequirement& Req = Descriptor.requirements[i];
		FString CppTypeName = FString(UTF8_TO_TCHAR(Req.cpp_type_name.ptr));

		// Find the UScriptStruct by name at runtime
		UScriptStruct* FoundStruct = FindFirstObject<UScriptStruct>(*CppTypeName, EFindFirstObjectOptions::NativeFirst);
		if (FoundStruct == nullptr)
		{
			UE_LOG(LogTemp, Error,
				TEXT("RustMassDynamicProcessor [%s]: Could not find UScriptStruct '%s'"),
				*SystemName, *CppTypeName);
			return;
		}

		FragmentStructs.Add(FoundStruct);
		FragmentAccessModes.Add(Req.access_mode);
		FragmentIsTags.Add(Req.is_tag);
	}

	bInitialized = true;
	UE_LOG(LogTemp, Log,
		TEXT("RustMassDynamicProcessor [%s]: Initialized with %d requirements"),
		*SystemName, FragmentStructs.Num());
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

void URustMassDynamicProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	if (!bInitialized)
	{
		return;
	}

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
}

void URustMassDynamicProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	if (!bInitialized || CachedExecuteFn == nullptr)
	{
		return;
	}

	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	// Capture non-tag fragment structs and their indices for chunk iteration
	TArray<const UScriptStruct*> ChunkFragmentStructs;
	TArray<int32> ChunkFragmentIndices;
	TArray<uint8> ChunkFragmentAccess;
	for (int32 i = 0; i < FragmentStructs.Num(); ++i)
	{
		if (!FragmentIsTags[i])
		{
			ChunkFragmentStructs.Add(FragmentStructs[i]);
			ChunkFragmentIndices.Add(i);
			ChunkFragmentAccess.Add(FragmentAccessModes[i]);
		}
	}

	EntityQuery.ForEachEntityChunk(Context,
		[this, DeltaSeconds, &ChunkFragmentStructs, &ChunkFragmentAccess](FMassExecutionContext& ChunkContext)
	{
		const int32 NumEntities = ChunkContext.GetNumEntities();

		// Build fragment slices from chunk data
		TArray<MassFragmentSlice> FragSlices;
		FragSlices.SetNum(ChunkFragmentStructs.Num());

		for (int32 i = 0; i < ChunkFragmentStructs.Num(); ++i)
		{
			const UScriptStruct* FragStruct = ChunkFragmentStructs[i];

			if (ChunkFragmentAccess[i] == 1) // ReadWrite
			{
				TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
				FragSlices[i].data = View.GetData();
				FragSlices[i].count = View.Num();
				FragSlices[i].stride = FragStruct->GetStructureSize();
			}
			else // ReadOnly
			{
				// Use const view - cast away const for the generic slice pointer
				// (Rust side respects access mode based on MassQuery<&T> vs MassQuery<&mut T>)
				TArrayView<FMassFragment> View = ChunkContext.GetMutableFragmentView(FragStruct);
				FragSlices[i].data = View.GetData();
				FragSlices[i].count = View.Num();
				FragSlices[i].stride = FragStruct->GetStructureSize();
			}
		}

		MassChunkData ChunkData;
		ChunkData.num_entities = NumEntities;
		ChunkData.dt = DeltaSeconds;
		ChunkData.num_fragments = FragSlices.Num();
		ChunkData.fragments = FragSlices.GetData();

		CachedExecuteFn(&ChunkData);
	});
}
