#pragma once

#include "CoreMinimal.h"
#include "MassEntityManager.h"
#include "MassProcessor.h"
#include "MassEntityQuery.h"
#include "RustMassMovementApplyProcessor.generated.h"

/**
 * Native C++ processor that applies velocity to transform.
 * Runs AFTER the Rust coordinator so that Rust systems can set velocity,
 * and this processor handles all position updates natively within the
 * UE Mass Entity pipeline.
 *
 * Responsibilities:
 * - Store PreviousTranslation
 * - Apply velocity: position += velocity * dt
 * - Clamp position to simulation bounds
 * - Reflect velocity at boundaries
 */
UCLASS()
class URustMassMovementApplyProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustMassMovementApplyProcessor();

	void SetSimulationBounds(const FBox& InBounds) { SimBounds = InBounds; }

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
	FBox SimBounds = FBox(EForceInit::ForceInit);
};
