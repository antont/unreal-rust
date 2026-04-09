#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "MassExternalSubsystemTraits.h"
#include "MassProcessingTypes.h"
#include "Subsystems/WorldSubsystem.h"
#include "GatherersFragments.h"
#include "GatherersBevyMassSubsystem.generated.h"

class UMassEntitySubsystem;
class URustMassGenericVisualizer;

UCLASS()
class RUSTMASSGATHERERS_API UGatherersBevyMassSubsystem : public UTickableWorldSubsystem
{
	GENERATED_BODY()

public:
	virtual void Initialize(FSubsystemCollectionBase& Collection) override;
	virtual void Deinitialize() override;
	virtual void Tick(float DeltaTime) override;
	virtual TStatId GetStatId() const override;

	void InitializeSimulation(int32 AntCount, int32 FoodCount, const FBox& Bounds, int32 RandomSeedBase);
	void ResetSimulation();
	void RunSimulationProcessorsForTesting(float DeltaTime);

	int32 GetManagedAntCount() const;
	int32 GetManagedFoodCount() const;
	bool HasManagedSimulation() const;

	/** Get the food ISMC (needed by spatial query for overlap detection). */
	UInstancedStaticMeshComponent* GetFoodISMC() const;

public:
	TArray<FMassEntityHandle> ManagedAntEntities;
	TArray<FMassEntityHandle> ManagedFoodEntities;
	FBox SimulationBounds = FBox(EForceInit::ForceInit);

private:
	bool EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem);
	void RunSimulationProcessorStep(float SimulatedDeltaTime);
	TArray<TArray<FMassEntityHandle>*> BuildGroupEntities();

private:
	UPROPERTY(Transient)
	FMassRuntimePipeline SimulationProcessorPipeline;

	bool bProcessorPipelinesInitialized = false;
	float SimulationTimeAccumulatorSeconds = 0.0f;

	UPROPERTY(Transient)
	TObjectPtr<URustMassGenericVisualizer> Visualizer = nullptr;
};

template<>
struct TMassExternalSubsystemTraits<UGatherersBevyMassSubsystem>
{
	enum
	{
		GameThreadOnly = true,
		ThreadSafeWrite = false,
	};
};
