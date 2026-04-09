#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "GatherersSimActivator.generated.h"

UCLASS()
class RUSTMASSGATHERERS_API AGatherersSimActivator : public AActor
{
	GENERATED_BODY()

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gatherers")
	int32 AntCount = 100;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gatherers")
	int32 FoodCount = 50;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gatherers")
	int32 RandomSeed = 42;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Gatherers")
	FBox SimulationBounds = FBox(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));

protected:
	virtual void BeginPlay() override;
};
