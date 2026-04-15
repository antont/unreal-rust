#pragma once

#include "CoreMinimal.h"
#include "MassEntityManager.h"
#include "MassProcessor.h"
#include "MassEntityQuery.h"
#include "RustMassPostMovementProcessor.generated.h"

/**
 * Minimal post-movement processor that runs AFTER UE's UMassApplyMovementProcessor.
 * Two responsibilities:
 * 1. Store PreviousTranslation (for spatial sweep queries next frame)
 * 2. Clamp position to simulation bounds
 */
UCLASS()
class URustMassPostMovementProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustMassPostMovementProcessor();

	void SetSimulationBounds(const FBox& InBounds) { SimBounds = InBounds; }

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
	FBox SimBounds = FBox(EForceInit::ForceInit);
};
