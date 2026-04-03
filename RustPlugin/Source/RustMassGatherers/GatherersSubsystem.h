#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityElementTypes.h"
#include "MassEntityHandle.h"
#include "MassExternalSubsystemTraits.h"
#include "MassProcessingTypes.h"
#include "Subsystems/WorldSubsystem.h"
#include "GatherersFragments.h"
#include "GatherersSubsystem.generated.h"

class UMassEntitySubsystem;

UCLASS()
class RUSTMASSGATHERERS_API UGatherersRustSubsystem : public UTickableWorldSubsystem
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
	void AdvanceAccumulatedSimulationSeconds(float DeltaTime);
	void SyncManagedVisuals();

	TArray<FGatherersMassFoodEncounter> QueryLooseFoodEncountersAlongSweep(
		const FVector& SweepStart, const FVector& SweepEnd, float Radius) const;

	int32 GetManagedAntCount() const;
	int32 GetManagedFoodCount() const;
	const TArray<FMassEntityHandle>& GetManagedFoodEntities() const { return ManagedFoodEntities; }
	bool HasManagedSimulation() const;
	float GetAccumulatedSimulationSeconds() const;
	const FBox& GetSimulationBounds() const;

public:
	TArray<FMassEntityHandle> ManagedAntEntities;
	TArray<FMassEntityHandle> ManagedFoodEntities;
	FBox SimulationBounds = FBox(EForceInit::ForceInit);
	float AccumulatedSimulationSeconds = 0.0f;
	float SimulationRateMultiplier = 1.0f;
	float SimulationTimeAccumulatorSeconds = 0.0f;
	int32 MaxSimulationStepsPerTick = 4096;

private:
	bool EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem);
	void RunSimulationProcessorStep(float SimulatedDeltaTime);
	void RunVisualSyncProcessor(float SimulatedDeltaTime);
	bool EnsureVisualComponents();
	void RebuildVisualInstances(UMassEntitySubsystem& MassEntitySubsystem);
	void SyncVisualInstances(UMassEntitySubsystem& MassEntitySubsystem);

private:
	UPROPERTY(Transient)
	FMassRuntimePipeline SimulationProcessorPipeline;

	UPROPERTY(Transient)
	FMassRuntimePipeline VisualProcessorPipeline;

	bool bProcessorPipelinesInitialized = false;

	UPROPERTY(Transient)
	TObjectPtr<AActor> VisualizerActor = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UInstancedStaticMeshComponent> AntVisualComponent = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UInstancedStaticMeshComponent> FoodRepresentationComponent = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UStaticMesh> VisualSphereMesh = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMaterialInstanceDynamic> AntVisualMaterial = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMaterialInstanceDynamic> FoodVisualMaterial = nullptr;
};

template<>
struct TMassExternalSubsystemTraits<UGatherersRustSubsystem>
{
	enum
	{
		GameThreadOnly = true,
		ThreadSafeWrite = false,
	};
};
