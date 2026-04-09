#include "RustMassScheduleCoordinator.h"

URustMassScheduleCoordinator::URustMassScheduleCoordinator()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = static_cast<uint8>(EProcessorExecutionFlags::All);
}

void URustMassScheduleCoordinator::InitializeDispatch(
	MassFrameDispatchFn InDispatchFn,
	const TArray<URustMassDynamicProcessor*>& InProcessors)
{
	DispatchFn = InDispatchFn;
	ManagedProcessors = InProcessors;

	// Put all managed processors in cache-only mode.
	// Each processor already has its original descriptor index from CreateAllRustProcessors.
	for (int32 i = 0; i < ManagedProcessors.Num(); ++i)
	{
		ManagedProcessors[i]->SetCacheOnly(true);
		// Do NOT overwrite SystemIndex — it was set to the original descriptor index
		// during CreateAllRustProcessors, before the processors were sorted by execution order.
	}

	UE_LOG(LogTemp, Log, TEXT("RustMassScheduleCoordinator: Managing %d processors via Bevy dispatch"),
		ManagedProcessors.Num());
}

void URustMassScheduleCoordinator::SetSpatialQueryCallback(MassSpatialQueryFn InFn, float InPickupRadius)
{
	SpatialQueryFn = InFn;
	PickupRadius = InPickupRadius;
}

void URustMassScheduleCoordinator::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	// No queries of our own — the dynamic processors own the queries.
}

void URustMassScheduleCoordinator::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	if (DispatchFn == nullptr || ManagedProcessors.Num() == 0)
	{
		return;
	}

	// Check that all processors have valid caches
	for (const URustMassDynamicProcessor* Proc : ManagedProcessors)
	{
		if (!Proc->IsChunkCacheValid())
		{
			return;
		}
	}

	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	// Build batch data from each processor's cached chunks
	TArray<MassSystemChunkBatch> Batches;
	Batches.SetNum(ManagedProcessors.Num());

	for (int32 i = 0; i < ManagedProcessors.Num(); ++i)
	{
		const URustMassDynamicProcessor* Proc = ManagedProcessors[i];
		const TArray<MassChunkData>& Chunks = Proc->GetCachedPrimaryChunks();

		Batches[i].system_index = Proc->GetSystemIndex();
		Batches[i].num_primary_chunks = static_cast<uint32>(Chunks.Num());
		Batches[i].primary_chunks = Chunks.GetData();
	}

	MassFrameDispatchData Data;
	Data.dt = DeltaSeconds;
	Data.num_systems = static_cast<uint32>(Batches.Num());
	Data.systems = Batches.GetData();
	Data.spatial_query_fn = SpatialQueryFn;
	Data.pickup_radius = PickupRadius;
	Data._pad = 0;

	DispatchFn(&Data);
}
