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
class URustMassScheduleCoordinator;

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

	/** Called by the plugin loader after a Rust dylib hot-reload.
	 *  Resets simulation state so processors are rebuilt with fresh function pointers. */
	void OnRustReloaded();

	/** Get entity array for a group by name. Returns nullptr if not found. */
	const TArray<FMassEntityHandle>* GetGroupEntities(const FString& GroupName) const;

	/** Get the ISMC for a visualizer group by name. Returns nullptr if not found. */
	UInstancedStaticMeshComponent* GetGroupISMC(const FString& GroupName) const;

	/** Get entity count for a group. */
	int32 GetGroupEntityCount(const FString& GroupName) const;

	using FSpatialQueryCallback = TFunction<uint32(const double*, const double*, float, MassSpatialQueryResult*)>;

	struct FSpatialQueryEntry
	{
		FString Name;
		FSpatialQueryCallback Callback;
		float Radius = 0.0f;
		int32 TrampolineIndex = -1; // Assigned during EnsureProcessorPipelines
	};

	/** Register a named spatial query callback. */
	void RegisterSpatialQuery(const FString& QueryName, FSpatialQueryCallback InCallback, float InRadius);

	/** Check if a named spatial query is registered. */
	bool HasSpatialQuery(const FString& QueryName) const { return SpatialQueries.Contains(QueryName); }

	/** Get number of registered spatial queries. */
	int32 GetSpatialQueryCount() const { return SpatialQueries.Num(); }

	/**
	 * Auto-setup spatial queries from Rust-registered config.
	 * Reads MassSpatialQueryConfigDesc entries and creates generic ISMC overlap callbacks.
	 * Called automatically after InitializeSimulation().
	 */
	void SetupSpatialQueriesFromRust();

	/** For testing: run one simulation step directly. */
	void RunSimulationProcessorsForTesting(float DeltaTime);

	/** Read raw fragment data for an entity by group name, index, and C++ fragment type name.
	 *  Returns true if successful, false if entity/fragment not found or size mismatch. */
	bool ReadFragmentData(const FString& GroupName, int32 EntityIndex,
		const FString& FragmentTypeName, void* OutData, int32 DataSize) const;

	/** Write raw fragment data for an entity by group name, index, and C++ fragment type name.
	 *  Returns true if successful, false if entity/fragment not found or size mismatch. */
	bool WriteFragmentData(const FString& GroupName, int32 EntityIndex,
		const FString& FragmentTypeName, const void* InData, int32 DataSize);

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

	UPROPERTY(Transient)
	TObjectPtr<URustMassScheduleCoordinator> ScheduleCoordinator = nullptr;

	/** Named spatial queries: key = query name, value = callback + radius. */
	TMap<FString, FSpatialQueryEntry> SpatialQueries;

	/** Saved init params for hot-reload re-initialization. */
	TArray<TPair<FString, int32>> SavedGroupCounts;
	FBox SavedBounds = FBox(EForceInit::ForceInit);
	int32 SavedRandomSeed = 0;
	bool bHasSavedInitParams = false;

	/** Whether we've already attempted auto-init from Rust defaults (prevent repeated attempts). */
	bool bAutoInitAttempted = false;

	/** Fallback: auto-initialize from Rust-registered sim defaults if no activator actor
	 *  called InitializeSimulation(). Runs once on first Tick. */
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
