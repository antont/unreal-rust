#pragma once

#include "CoreMinimal.h"
#include "MassEntityQuery.h"
#include "MassProcessor.h"
#include "GatherersBevyMassCollisionProcessor.generated.h"

class UGatherersBevyMassSubsystem;

UCLASS()
class UGatherersBevyMassCollisionProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	UGatherersBevyMassCollisionProcessor();

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
};
