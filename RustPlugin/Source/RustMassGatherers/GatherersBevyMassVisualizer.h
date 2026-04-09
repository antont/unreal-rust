#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "GatherersBevyMassVisualizer.generated.h"

struct FMassEntityManager;
class UMassEntitySubsystem;

/**
 * Handles all visualization (ISMCs, materials, meshes) for the GatherersBevyMass simulation.
 * Reads entity data directly from Mass Entity — no coupling to the sim subsystem internals.
 */
UCLASS()
class RUSTMASSGATHERERS_API UGatherersBevyMassVisualizer : public UObject
{
	GENERATED_BODY()

public:
	/**
	 * Initialize visual components in the given world.
	 * Must be called before RebuildInstances/SyncInstances.
	 */
	bool Initialize(UWorld* World);

	/** Tear down all visual state (destroy actor, clear components). */
	void Teardown();

	/** Rebuild all ISM instances from scratch. */
	void RebuildInstances(
		FMassEntityManager& EntityManager,
		const TArray<FMassEntityHandle>& AntEntities,
		const TArray<FMassEntityHandle>& FoodEntities);

	/** Update ISM transforms to match current entity positions. */
	void SyncInstances(
		FMassEntityManager& EntityManager,
		const TArray<FMassEntityHandle>& AntEntities,
		const TArray<FMassEntityHandle>& FoodEntities);

	/** Get the food ISM component (needed by spatial query for overlap detection). */
	UInstancedStaticMeshComponent* GetFoodISMC() const { return FoodRepresentationComponent; }

private:
	bool EnsureVisualComponents(UWorld* World);

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
