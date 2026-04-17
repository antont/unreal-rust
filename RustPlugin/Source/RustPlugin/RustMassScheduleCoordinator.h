// Coordinator processor that gathers cached chunk data from all dynamic processors
// and dispatches them to Rust's Bevy-scheduled mass_frame_dispatch in a single call.
//
// Execution order invariant: all URustMassDynamicProcessor instances must run their
// Execute() and populate chunk caches BEFORE this coordinator reads them. The subsystem
// enforces this by adding the coordinator last to the processing pipeline and setting
// ExecutionOrder higher than all dynamic processors.

#pragma once

#include "CoreMinimal.h"
#include "MassProcessor.h"
#include "Bindings.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.generated.h"

UCLASS()
class RUSTPLUGIN_API URustMassScheduleCoordinator : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustMassScheduleCoordinator();

	/// Set the dispatch function and the processors to coordinate.
	void InitializeDispatch(
		MassFrameDispatchFn InDispatchFn,
		const TArray<URustMassDynamicProcessor*>& InProcessors);

	/// Set the spatial query slots for collision detection.
	void SetSpatialQuerySlots(TArray<MassSpatialQuerySlot> InSlots, TArray<TArray<char>> InNameBuffers);


protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

public:
	/** Post-dispatch flags from the most recent Execute call. */
	uint32 GetLastDispatchFlags() const { return LastDispatchFlags; }

private:
	MassFrameDispatchFn DispatchFn = nullptr;
	uint32 LastDispatchFlags = 0;
	TArray<MassSpatialQuerySlot> SpatialQuerySlots;
	TArray<TArray<char>> SpatialQueryNameBuffers; // Owns name string data for SpatialQuerySlots
	TArray<URustMassDynamicProcessor*> ManagedProcessors;
};
