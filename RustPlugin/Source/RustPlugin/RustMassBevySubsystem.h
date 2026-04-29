#pragma once

#include "Components/InstancedStaticMeshComponent.h"
#include "CoreMinimal.h"
#include "MassEntityHandle.h"
#include "MassExternalSubsystemTraits.h"
#include "MassProcessingTypes.h"
#include "MassNavigationSubsystem.h"
#include "Subsystems/WorldSubsystem.h"
#include "Bindings.h"
#include "RustMassBevySubsystem.generated.h"

class UMassEntitySubsystem;
class UMassProcessor;
class URustMassVisualizationSetup;
class UMassSimpleMovementProcessor;
class URustMassPostMovementProcessor;
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

	using FSpatialEnumerateCallback = TFunction<uint32(
		const double* Center, float Radius, MassSpatialNeighbor* Out, uint32 Max)>;

	struct FSpatialEnumerateEntry
	{
		FString Name;
		FSpatialEnumerateCallback Callback;
		float Radius = 0.0f;
		int32 TrampolineIndex = -1;
	};

	/** Register a named spatial enumerate callback. */
	void RegisterSpatialEnumerate(const FString& Name, FSpatialEnumerateCallback InCallback, float InRadius);

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

	/** Whether native vis processors are registered with MassSimulationSubsystem. */
	bool HasVisProcessorsRegistered() const { return VisProcessor != nullptr; }

	/** Read raw fragment data for an entity by group name, index, and C++ fragment type name.
	 *  Returns true if successful, false if entity/fragment not found or size mismatch. */
	bool ReadFragmentData(const FString& GroupName, int32 EntityIndex,
		const FString& FragmentTypeName, void* OutData, int32 DataSize) const;

	/** Write raw fragment data for an entity by group name, index, and C++ fragment type name.
	 *  Returns true if successful, false if entity/fragment not found or size mismatch. */
	bool WriteFragmentData(const FString& GroupName, int32 EntityIndex,
		const FString& FragmentTypeName, const void* InData, int32 DataSize);

	/** Read the reverse entity→instance-index map for a GridHash-owned group.
	 *  Returns nullptr if the group is not GridHash-owned or doesn't exist.
	 *  Exposed for automation tests that drive ExecuteGridHashSpatialQuery directly. */
	const TMap<FMassEntityHandle, int32>* GetGroupEntityToIndex(const FString& GroupName) const;

	/** Snapshot of the GridHash diagnostic counters. Values accumulate across every
	 *  invocation of ExecuteGridHashSpatialQuery and are reset at the end of each
	 *  RunSimulationProcessorStep. Exposed so automation tests can measure where
	 *  candidates get dropped in the pickup pipeline. */
	struct FGridHashCounters
	{
		uint64 Calls = 0;
		uint64 Candidates = 0;       // items returned by QuerySmall
		uint64 CandidatesValid = 0;  // after EntityManager.IsEntityValid
		uint64 CandidatesMapped = 0; // after EntityToIndex lookup
		uint64 CandidatesPassed = 0; // after filter fragment read
		uint64 EncountersWithin = 0; // passed the radius distance check (per candidate)
		uint64 EncountersReturned = 0; // calls that returned has_encounter=true
	};
	static FGridHashCounters GetGridHashCounters();
	static void ResetGridHashCounters();

	/** Re-run PopulateGridHashForGroup for a named GridHash-owned group.
	 *  Exposed for automation tests that need to drive the populate path
	 *  more than once (e.g. the two-queries-one-group regression spec).
	 *  Returns true if the group exists and is GridHash-owned. */
	bool RepopulateGridHashForGroupForTesting(const FString& GroupName);

	/** Drive the GridHash ownership setup for a group exactly like
	 *  SetupSpatialQueriesFromRust does for QueryType==2. Returns true if the
	 *  group was newly marked as GridHash-owned, false if refused (e.g. a
	 *  different group is already the GridHash owner — enforced single-owner
	 *  constraint for the single-group food-event FFI). */
	bool TryMarkGridHashOwnerForTesting(const FString& GroupName);

	/** Apply a single food-drop event through the same code path as the real
	 *  FFI drain (ApplyFoodEvents' drop loop body). Exposed for automation
	 *  tests that need to verify the single-group scope of the drop path
	 *  without standing up a full Bevy→C++ FFI round-trip. */
	void ApplyFoodDropEventForTesting(int32 FoodIdx, const FVector& NewPos);

public:
	/** Named entity groups: key = group name, value = entity handles. */
	TMap<FString, TArray<FMassEntityHandle>> EntityGroups;
	FBox SimulationBounds = FBox(EForceInit::ForceInit);

private:
	bool EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem);
	void RunSimulationProcessorStep(float SimulatedDeltaTime);

	/** Create collision-only ISMCs for spatial query groups (from Rust visualizer group descs). */
	void InitializeCollisionISMCs(UWorld* World, const RustBindings& Bindings, FMassEntityManager& EntityManager);

	/** Sync collision ISMC instance positions from entity fragment data. */
	void SyncCollisionISMCs(FMassEntityManager& EntityManager);

	/** Tear down collision ISMCs and their owning actor. */
	void TeardownCollisionISMCs();

	/** Per-group collision ISMC metadata. */
	struct FCollisionGroupEntry
	{
		FString Name;
		const UScriptStruct* PositionStruct = nullptr;
		uint32 PositionOffset = 0;
		FVector Scale = FVector::OneVector;
		UInstancedStaticMeshComponent* ISMC = nullptr;
		/** When true, this group's entities are tracked in the navigation hash grid
		 *  (populated at init, updated on food-drop events & pickup events).
		 *  Multiple groups may be nav-grid participants simultaneously — the per-group
		 *  EntityToIndex map keeps query results scoped correctly. */
		bool bOwnedByGridHash = false;
		/** When true, this group owns the food-event instance-index space — pickups
		 *  and drops from the Rust FFI (`get_food_pickup_events` / `get_food_drop_events`)
		 *  carry a bare `food_index` with no group identifier, so only one group can
		 *  be the authoritative owner. Set on the first group registered via
		 *  TryMarkGridHashOwner; later groups participate in the grid but don't
		 *  receive food events. */
		bool bIsFoodEventIndexOwner = false;
		/** Cached grid cell locations per instance, for incremental Move/Remove. */
		TArray<FNavigationObstacleHashGrid2D::FCellLocation> GridCellLocations;
		/** Per-instance: whether this entity currently has a grid cell location.
		 *  Instances removed on pickup are cleared; re-added on drop. */
		TBitArray<> InGrid;
		/** Reverse map from entity handle to group instance index (for GridHash queries). */
		TMap<FMassEntityHandle, int32> EntityToIndex;
	};

	/** Populate the navigation hash grid from the current instance transforms for a GridHash-owned group. */
	void PopulateGridHashForGroup(FCollisionGroupEntry& Group);

	/** Mark a group as a nav-grid participant and populate its grid entries.
	 *  Multiple groups may participate simultaneously — the first group registered
	 *  also claims `bIsFoodEventIndexOwner` to retain the food-event FFI's single
	 *  index-space guarantee. Returns false only if the group doesn't exist.
	 *  Idempotent. LogPrefix is reserved for future diagnostics. */
	bool TryMarkGridHashOwner(const FString& GroupName, const TCHAR* LogPrefix);

	/** Remove the group's entries from the navigation hash grid (called on reset). */
	void ClearGridHashForGroup(FCollisionGroupEntry& Group);

	/** Drain Rust's food pickup + drop event queues and apply ISM-transform + grid-location updates. */
	void ApplyFoodEvents();

	/** Drain Rust's shadow-entity despawn queue: destroy the corresponding Mass entities
	 *  and clean up nav hash grid entries. Leaves spawn-index slots in place as tombstones
	 *  so downstream indexing (food events, nav cell arrays, ISMC transforms) stays stable.
	 *  Called at the end of each RunSimulationProcessorStep so the next substep in the
	 *  same tick sees a consistent Bevy/Mass view — deferring until after all substeps
	 *  would leave just-despawned entities Mass-valid and spatial-query visible. */
	void ApplyDespawnEvents();

	/** Apply a single food-drop event across all collision groups. Shared body between
	 *  the production FFI drain and the test-only `ApplyFoodDropEventForTesting` hook.
	 *  Validates the target entity handle against `EntityManager` before mutating ISMC
	 *  transforms or the nav grid — a just-despawned shadow entity could otherwise be
	 *  "revived" as a grid hit. */
	void ApplyOneFoodDropEvent(int32 FoodIdx, const FVector& NewPos,
		FNavigationObstacleHashGrid2D* Grid, FMassEntityManager& EntityManager);

private:
	UPROPERTY(Transient)
	FMassRuntimePipeline SimulationProcessorPipeline;

	/** Vis pipeline: LOD + Representation processors, run after simulation each tick. */
	UPROPERTY(Transient)
	FMassRuntimePipeline VisualizationPipeline;

	bool bProcessorPipelinesInitialized = false;
	float SimulationTimeAccumulatorSeconds = 0.0f;

	UPROPERTY(Transient)
	TObjectPtr<URustMassVisualizationSetup> VisualizationSetup = nullptr;

	/** Native vis processors registered with MassSimulationSubsystem. */
	UPROPERTY(Transient)
	TObjectPtr<UMassProcessor> VisProcessor = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMassProcessor> VisLODProcessor = nullptr;

	/** Collision-only ISMCs for spatial queries (not rendering). */
	TArray<FCollisionGroupEntry> CollisionGroups;

	UPROPERTY(Transient)
	TObjectPtr<AActor> CollisionActor = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<URustMassScheduleCoordinator> ScheduleCoordinator = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<UMassSimpleMovementProcessor> NativeMovementProcessor = nullptr;

	UPROPERTY(Transient)
	TObjectPtr<URustMassPostMovementProcessor> PostMovementProcessor = nullptr;

	/** Named spatial queries: key = query name, value = callback + radius. */
	TMap<FString, FSpatialQueryEntry> SpatialQueries;

	/** Named spatial enumerates: key = enumerate name, value = callback + radius. */
	TMap<FString, FSpatialEnumerateEntry> SpatialEnumerates;

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

/**
 * Execute the GridHash spatial-query callback body against a prebuilt grid/group.
 *
 * Extracted from URustMassBevySubsystem::RegisterSpatialQueryFromConfig (QueryType=2)
 * so automation tests can exercise it without standing up a full simulation.
 * Grid membership IS the is_loose filter — the caller is responsible for
 * adding/removing items from Grid on drop/pickup.
 *
 * Returns 1 (query ran); encounter data in *Out.
 */
RUSTPLUGIN_API uint32 ExecuteGridHashSpatialQuery(
	FMassEntityManager& EntityManager,
	const FNavigationObstacleHashGrid2D& Grid,
	const TMap<FMassEntityHandle, int32>& EntityToIndex,
	const UInstancedStaticMeshComponent& ISMC,
	const double* PreviousPos,
	const double* CurrentPos,
	float PickupRadius,
	MassSpatialQueryResult* Out);

/**
 * Execute the GridHash enumerate callback body against a prebuilt grid/group.
 *
 * Returns all entities within radius of Center (up to Max results in Out).
 * Return value is the uncapped count — callers test `returned > Max` to detect
 * truncation and retry with a larger buffer.
 */
RUSTPLUGIN_API uint32 ExecuteGridHashEnumerate(
	FMassEntityManager& EntityManager,
	const FNavigationObstacleHashGrid2D& Grid,
	const TMap<FMassEntityHandle, int32>& EntityToIndex,
	const UInstancedStaticMeshComponent& ISMC,
	const double* Center,
	float Radius,
	MassSpatialNeighbor* Out,
	uint32 Max);

template<>
struct TMassExternalSubsystemTraits<URustMassBevySubsystem>
{
	enum
	{
		GameThreadOnly = true,
		ThreadSafeWrite = false,
	};
};
