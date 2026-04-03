#pragma once

#include "CoreMinimal.h"
#include "MassEntityQuery.h"
#include "MassProcessor.h"
#include "GatherersProcessors.generated.h"

UCLASS()
class UGatherersTimeAccumulationProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	UGatherersTimeAccumulationProcessor();

protected:
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;
};

UCLASS()
class UGatherersAntMovementProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	UGatherersAntMovementProcessor();

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
};

UCLASS()
class UGatherersFoodInteractionProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	UGatherersFoodInteractionProcessor();

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
};

UCLASS()
class UGatherersVisualSyncProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	UGatherersVisualSyncProcessor();

protected:
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;
};
