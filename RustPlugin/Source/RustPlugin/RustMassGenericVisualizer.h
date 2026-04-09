#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "Bindings.h"
#include "RustMassGenericVisualizer.generated.h"

struct FMassEntityManager;

/**
 * Generic position-to-ISMC visualizer driven by Rust visual group registrations.
 * Each group gets one ISMC; transforms are synced from a position fragment at a known offset.
 */
UCLASS()
class RUSTPLUGIN_API URustMassGenericVisualizer : public UObject
{
	GENERATED_BODY()

public:
	/**
	 * Query Rust for visual group descriptors and create ISMCs.
	 * @return true if at least one group was initialized.
	 */
	bool Initialize(UWorld* World, const RustBindings& Bindings);

	/** Destroy the actor and clear all ISMCs. */
	void Teardown();

	/** Rebuild all ISMC instances from entity arrays. */
	void RebuildInstances(FMassEntityManager& EM, const TArray<TArray<FMassEntityHandle>*>& GroupEntities);

	/** Update ISMC transforms from current entity positions. */
	void SyncInstances(FMassEntityManager& EM, const TArray<TArray<FMassEntityHandle>*>& GroupEntities);

	/** Get the number of initialized groups. */
	int32 GetGroupCount() const { return Groups.Num(); }

private:
	struct VisualGroup
	{
		FString Name;
		const UScriptStruct* PositionFragmentStruct = nullptr;
		uint32 PositionOffset = 0;
		FVector Scale = FVector::OneVector;

		UPROPERTY()
		TObjectPtr<UInstancedStaticMeshComponent> ISMC = nullptr;
	};

	TArray<VisualGroup> Groups;

	UPROPERTY(Transient)
	TObjectPtr<AActor> VisualizerActor = nullptr;
};
