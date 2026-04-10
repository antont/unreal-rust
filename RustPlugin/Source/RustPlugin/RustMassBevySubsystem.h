#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "MassExternalSubsystemTraits.h"
#include "MassProcessingTypes.h"
#include "Subsystems/WorldSubsystem.h"
#include "Bindings.h"
#include "RustMassBevySubsystem.generated.h"

class UMassEntitySubsystem;
class URustMassGenericVisualizer;

/**
 * Generic Bevy-Mass subsystem: manages named entity groups, processor pipelines,
 * tick sub-stepping, visualization, and spatial query delegation.
 *
 * Game modules provide only:
 *   - Group counts + bounds via InitializeSimulation()
 *   - A spatial query callback via SetSpatialQueryCallback()
 *   - Any post-init collision/ISMC configuration
 */
UCLASS()
class RUSTPLUGIN_API URustMassBevySubsystem : public UTickableWorldSubsystem
{
	GENERATED_BODY()

public:
	virtual void Initialize(FSubsystemCollectionBase& Collection) override;
	virtual void Deinitialize() override;
	virtual void Tick(float DeltaTime) override;
	virtual TStatId GetStatId() const override;

	/** Initialize simulation with named entity groups. */
	void InitializeSimulation(
		const TArray<TPair<FString, int32>>& GroupCounts,
		const FBox& Bounds,
		int32 RandomSeed);

	void ResetSimulation();
	bool HasManagedSimulation() const;

	/** Get entity array for a group by name. Returns nullptr if not found. */
	const TArray<FMassEntityHandle>* GetGroupEntities(const FString& GroupName) const;

	/** Get the ISMC for a visualizer group by name. Returns nullptr if not found. */
	UInstancedStaticMeshComponent* GetGroupISMC(const FString& GroupName) const;

	/** Get entity count for a group. */
	int32 GetGroupEntityCount(const FString& GroupName) const;

	using FSpatialQueryCallback = TFunction<uint32(const double*, const double*, float, MassSpatialQueryResult*)>;

	/** Register a spatial query callback from the game module. */
	void SetSpatialQueryCallback(FSpatialQueryCallback InCallback, float InRadius);

	/**
	 * Auto-setup spatial queries from Rust-registered config.
	 * Reads MassSpatialQueryConfigDesc entries and creates generic ISMC overlap callbacks.
	 * Called automatically after InitializeSimulation().
	 */
	void SetupSpatialQueriesFromRust();

	/** For testing: run one simulation step directly. */
	void RunSimulationProcessorsForTesting(float DeltaTime);

public:
	/** Named entity groups: key = group name, value = entity handles. */
	TMap<FString, TArray<FMassEntityHandle>> EntityGroups;
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

	FSpatialQueryCallback SpatialQueryCallback;
	float SpatialQueryRadius = 0.0f;

	/** Resolved UScriptStruct for the spatial query filter fragment (set by SetupSpatialQueriesFromRust). */
	const UScriptStruct* SpatialQueryFilterScriptStruct = nullptr;

	/** Whether we've already attempted auto-init from Rust defaults (prevent repeated attempts). */
	bool bAutoInitAttempted = false;

	/** Try to auto-initialize from Rust-registered sim defaults if no activator actor called InitializeSimulation(). */
	void TryAutoInitFromRustDefaults();
};

template<>
struct TMassExternalSubsystemTraits<URustMassBevySubsystem>
{
	enum
	{
		GameThreadOnly = true,
		ThreadSafeWrite = false,
	};
};
