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

public:
	TArray<FMassEntityHandle> ManagedAntEntities;
	TArray<FMassEntityHandle> ManagedFoodEntities;
	FBox SimulationBounds = FBox(EForceInit::ForceInit);

	/// Food ISM — public so the collision processor can access it.
	UPROPERTY(Transient)
	TObjectPtr<UInstancedStaticMeshComponent> FoodRepresentationComponent = nullptr;

private:
	bool EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem);
	void RunSimulationProcessorStep(float SimulatedDeltaTime);

	bool EnsureVisualComponents();
	void RebuildVisualInstances(UMassEntitySubsystem& MassEntitySubsystem);
	void SyncVisualInstances(UMassEntitySubsystem& MassEntitySubsystem);

private:
	UPROPERTY(Transient)
	FMassRuntimePipeline SimulationProcessorPipeline;

	bool bProcessorPipelinesInitialized = false;
	float SimulationTimeAccumulatorSeconds = 0.0f;

	UPROPERTY(Transient)
	TObjectPtr<AActor> VisualizerActor = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UInstancedStaticMeshComponent> AntVisualComponent = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UStaticMesh> VisualSphereMesh = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMaterialInstanceDynamic> AntVisualMaterial = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMaterialInstanceDynamic> FoodVisualMaterial = nullptr;
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
