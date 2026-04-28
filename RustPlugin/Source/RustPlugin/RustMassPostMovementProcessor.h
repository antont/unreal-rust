#pragma once

#include "CoreMinimal.h"
#include "MassEntityManager.h"
#include "MassProcessor.h"
#include "MassEntityQuery.h"
#include "RustMassPostMovementProcessor.generated.h"

/**
 * Pre-movement processor: saves PreviousTranslation for spatial sweep queries.
 * Runs BEFORE UE's UMassSimpleMovementProcessor so PreviousTranslation captures
 * the position before this frame's movement.
 */
UCLASS()
class URustMassPostMovementProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustMassPostMovementProcessor();

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
};
