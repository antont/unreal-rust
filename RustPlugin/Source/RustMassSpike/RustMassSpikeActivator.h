#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "MassEntityHandle.h"
#include "RustMassSpikeActivator.generated.h"

/**
 * Place this actor in a level to spawn Bob entities.
 * Unlike the old URustMassSpikeSubsystem (which ran in every world),
 * this only activates in levels where the actor is placed.
 */
UCLASS()
class RUSTMASSSPIKE_API ARustMassSpikeActivator : public AActor
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Spike")
	int32 EntityCount = 10;

	virtual void BeginPlay() override;

	int32 GetSpawnedEntityCount() const { return SpawnedEntities.Num(); }

private:
	TArray<FMassEntityHandle> SpawnedEntities;
};
