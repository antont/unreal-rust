// Coordinator processor that gathers cached chunk data from all dynamic processors
// and dispatches them to Rust's Bevy-scheduled mass_frame_dispatch in a single call.

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

	/// Set the spatial query callback for collision detection.
	void SetSpatialQueryCallback(MassSpatialQueryFn InFn, float InPickupRadius);


protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	MassFrameDispatchFn DispatchFn = nullptr;
	MassSpatialQueryFn SpatialQueryFn = nullptr;
	float PickupRadius = 15.0f;
	TArray<URustMassDynamicProcessor*> ManagedProcessors;
};
