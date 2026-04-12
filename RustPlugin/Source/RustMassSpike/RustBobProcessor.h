#pragma once

#include "MassProcessor.h"
#include "RustBobFragment.h"
#include "RustBobProcessor.generated.h"

UCLASS()
class URustBobProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustBobProcessor();

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;
};
