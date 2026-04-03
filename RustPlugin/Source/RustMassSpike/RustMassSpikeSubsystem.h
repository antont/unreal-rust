#pragma once

#include "Subsystems/WorldSubsystem.h"
#include "MassEntitySubsystem.h"
#include "RustBobFragment.h"
#include "RustMassSpikeSubsystem.generated.h"

UCLASS()
class URustMassSpikeSubsystem : public UWorldSubsystem
{
	GENERATED_BODY()

public:
	virtual void PostInitialize() override;
	virtual void Deinitialize() override;

	int32 GetSpawnedEntityCount() const { return SpawnedEntities.Num(); }

private:
	TArray<FMassEntityHandle> SpawnedEntities;
};
