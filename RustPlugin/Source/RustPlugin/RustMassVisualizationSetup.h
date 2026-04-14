#pragma once

#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "RustMassVisualizationSetup.generated.h"

struct FMassEntityManager;
class UStaticMesh;
class UMaterialInterface;

/**
 * Configures UE's native MassRepresentation visualization on programmatically-spawned entities.
 * Adds all required fragments, shared fragments, chunk fragments, and tags so that
 * UMassVisualizationProcessor and UMassVisualizationLODProcessor can render them.
 */
UCLASS()
class RUSTPLUGIN_API URustMassVisualizationSetup : public UObject
{
	GENERATED_BODY()

public:
	/**
	 * Add MassRepresentation visualization to a group of entities.
	 * @param World          Current world (needed for RepresentationSubsystem)
	 * @param EntityManager  MassEntity manager
	 * @param Entities       Handles of entities to visualize
	 * @param Mesh           Static mesh for ISM rendering
	 * @param Material       Material (nullptr = mesh default)
	 * @param Scale          Per-instance scale
	 * @param LODDistances   Distance thresholds [High, Medium, Low, Off]
	 * @return true if visualization was successfully configured
	 */
	bool SetupGroupVisualization(
		UWorld* World,
		FMassEntityManager& EntityManager,
		const TArray<FMassEntityHandle>& Entities,
		UStaticMesh* Mesh,
		UMaterialInterface* Material,
		FVector Scale,
		const float LODDistances[4]);

	/** Tear down: nothing to do — MassRepresentation subsystem owns ISM lifecycle. */
	void Teardown() {}
};
