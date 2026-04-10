#include "RustMassBevySubsystem.h"

#include "Engine/World.h"
#include "GameFramework/Actor.h"
#include "MassExecutor.h"
#include "MassEntityManager.h"
#include "MassProcessingContext.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.h"
#include "RustPlugin.h"
#include "RustUtils.h"
#include "RustMassGenericVisualizer.h"

// ---------------------------------------------------------------------------
// Spatial query trampoline: bounces C function pointer to game-module delegate
// ---------------------------------------------------------------------------

namespace RustMassSpatialQuery
{
	static URustMassBevySubsystem::FSpatialQueryCallback* ActiveCallback = nullptr;

	uint32_t Trampoline(
		const double* PreviousPos,
		const double* CurrentPos,
		float PickupRadius,
		MassSpatialQueryResult* Out)
	{
		if (ActiveCallback && *ActiveCallback)
		{
			return (*ActiveCallback)(PreviousPos, CurrentPos, PickupRadius, Out);
		}
		if (Out)
		{
			Out->has_encounter = false;
			Out->food_index = -1;
		}
		return 0;
	}
} // namespace RustMassSpatialQuery

// ---------------------------------------------------------------------------
// Subsystem lifecycle
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
	bProcessorPipelinesInitialized = false;
}

void URustMassBevySubsystem::Deinitialize()
{
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	Super::Deinitialize();
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

const TArray<FMassEntityHandle>* URustMassBevySubsystem::GetGroupEntities(const FString& GroupName) const
{
	return EntityGroups.Find(GroupName);
}

UInstancedStaticMeshComponent* URustMassBevySubsystem::GetGroupISMC(const FString& GroupName) const
{
	if (!Visualizer) return nullptr;
	for (int32 i = 0; i < Visualizer->GetGroupCount(); ++i)
	{
		if (Visualizer->GetGroupName(i) == GroupName)
		{
			return Visualizer->GetGroupISMC(i);
		}
	}
	return nullptr;
}

int32 URustMassBevySubsystem::GetGroupEntityCount(const FString& GroupName) const
{
	const TArray<FMassEntityHandle>* Arr = EntityGroups.Find(GroupName);
	return Arr ? Arr->Num() : 0;
}

bool URustMassBevySubsystem::HasManagedSimulation() const
{
	for (const auto& Pair : EntityGroups)
	{
		if (Pair.Value.Num() > 0) return true;
	}
	return false;
}

void URustMassBevySubsystem::SetSpatialQueryCallback(FSpatialQueryCallback InCallback, float InRadius)
{
	SpatialQueryCallback = MoveTemp(InCallback);
	SpatialQueryRadius = InRadius;
}

void URustMassBevySubsystem::SetupSpatialQueriesFromRust()
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (!Module.Plugin.Rust.get_spatial_query_config_count.IsSome() ||
		!Module.Plugin.Rust.get_spatial_query_config_desc.IsSome())
	{
		return;
	}

	uint32 ConfigCount = Module.Plugin.Rust.get_spatial_query_config_count.Unwrap()();
	if (ConfigCount == 0)
	{
		return;
	}

	// Use the first config (single spatial query per sim for now)
	MassSpatialQueryConfigDesc Config = {};
	if (Module.Plugin.Rust.get_spatial_query_config_desc.Unwrap()(0, &Config) == 0)
	{
		return;
	}

	// Extract config values into captured variables
	FString QueryGroup = FString(Config.query_group.len, UTF8_TO_TCHAR(Config.query_group.ptr));
	FString FilterFragmentType = FString(Config.filter_fragment_type.len, UTF8_TO_TCHAR(Config.filter_fragment_type.ptr));
	float Radius = Config.radius;
	uint32 FilterBoolOffset = Config.filter_bool_offset;
	bool FilterBoolMustBe = Config.filter_bool_must_be;

	// Resolve the filter fragment UScriptStruct for runtime access.
	// Strip F/U prefix — UE reflection registers structs without the prefix.
	FString SearchName = FilterFragmentType;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	const UScriptStruct* FilterStruct = FindFirstObject<UScriptStruct>(
		*SearchName, EFindFirstObjectOptions::NativeFirst);

	if (!FilterStruct)
	{
		UE_LOG(LogTemp, Warning, TEXT("RustMassBevySubsystem: Could not find UScriptStruct '%s'. "
			"Spatial query filtering disabled."), *FilterFragmentType);
	}

	// Store for the lambda to capture
	SpatialQueryFilterScriptStruct = FilterStruct;

	// Capture subsystem pointer and config for the lambda
	URustMassBevySubsystem* Self = this;

	SetSpatialQueryCallback(
		[Self, QueryGroup, FilterBoolOffset, FilterBoolMustBe, FilterStruct](
			const double* PreviousPos, const double* CurrentPos,
			float PickupRadius, MassSpatialQueryResult* Out) -> uint32
		{
			if (!Out) return 0;

			Out->has_encounter = false;
			Out->food_index = -1;
			Out->encounter_position[0] = 0.0;
			Out->encounter_position[1] = 0.0;
			Out->encounter_position[2] = 0.0;

			const TArray<FMassEntityHandle>* TargetEntities = Self->GetGroupEntities(QueryGroup);
			UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);

			UWorld* W = Self->GetWorld();
			UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
			if (!TargetEntities || !TargetISMC || !MES)
			{
				return 0;
			}

			FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
			const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
			const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
			const float PickupRadiusSq = PickupRadius * PickupRadius;

			// ISMC spatial query: sphere or box depending on movement
			TSet<int32> CandidateIndices;
			if (SweepStart.Equals(SweepEnd))
			{
				for (const int32 Idx : TargetISMC->GetInstancesOverlappingSphere(
					SweepEnd, PickupRadius, true))
				{
					if (TargetEntities->IsValidIndex(Idx))
					{
						CandidateIndices.Add(Idx);
					}
				}
			}
			else
			{
				FBox SweptBounds(ForceInit);
				SweptBounds += SweepStart;
				SweptBounds += SweepEnd;
				SweptBounds = SweptBounds.ExpandBy(PickupRadius);
				for (const int32 Idx : TargetISMC->GetInstancesOverlappingBox(SweptBounds, true))
				{
					if (TargetEntities->IsValidIndex(Idx))
					{
						CandidateIndices.Add(Idx);
					}
				}
			}

			// Find nearest matching entity
			float BestDistSq = TNumericLimits<float>::Max();
			for (const int32 Idx : CandidateIndices)
			{
				const FMassEntityHandle Entity = (*TargetEntities)[Idx];
				if (!EntityManager.IsEntityValid(Entity)) continue;

				// Read filter fragment data as raw bytes and check bool at registered offset
				if (FilterStruct)
				{
					FMassEntityView EntityView(EntityManager, Entity);
					FStructView FragView = EntityView.GetFragmentDataStruct(FilterStruct);
					if (FragView.IsValid())
					{
						const uint8* FragmentBytes = FragView.GetMemory();
						bool FilterValue = *reinterpret_cast<const bool*>(FragmentBytes + FilterBoolOffset);
						if (FilterValue != FilterBoolMustBe) continue;
					}
				}

				// Get position from ISMC transform (synced by visualizer each frame)
				FTransform InstanceTransform;
				TargetISMC->GetInstanceTransform(Idx, InstanceTransform, true);
				const FVector EntityPos = InstanceTransform.GetLocation();

				const FVector Closest = FMath::ClosestPointOnSegment(
					EntityPos, SweepStart, SweepEnd);
				const float DistSq = FVector::DistSquared(Closest, EntityPos);
				if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->food_index = Idx;
					Out->encounter_position[0] = Closest.X;
					Out->encounter_position[1] = Closest.Y;
					Out->encounter_position[2] = Closest.Z;
				}
			}

			return 1;
		},
		Radius);

	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-setup spatial query for group '%s' "
		"(radius=%.1f, filter=%s, bool_offset=%d, must_be=%s)"),
		*QueryGroup, Radius, *FilterFragmentType, FilterBoolOffset,
		FilterBoolMustBe ? TEXT("true") : TEXT("false"));
}

// ---------------------------------------------------------------------------
// Processor pipelines
// ---------------------------------------------------------------------------

bool URustMassBevySubsystem::EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (bProcessorPipelinesInitialized)
	{
		return true;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	TSharedRef<FMassEntityManager> EntityManagerRef = EntityManager.AsShared();

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	TArray<URustMassDynamicProcessor*> DynamicProcessors =
		URustMassDynamicProcessor::CreateAllRustProcessors(Module.Plugin.Rust, this);

	TArray<UMassProcessor*> SimProcessors;

	DynamicProcessors.Sort([](const URustMassDynamicProcessor& A, const URustMassDynamicProcessor& B)
	{
		return A.GetExecutionOrder() < B.GetExecutionOrder();
	});

	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		SimProcessors.Add(Proc);
	}

	if (Module.Plugin.Rust.mass_frame_dispatch != nullptr && DynamicProcessors.Num() > 0)
	{
		URustMassScheduleCoordinator* Coordinator = NewObject<URustMassScheduleCoordinator>(this);
		Coordinator->InitializeDispatch(Module.Plugin.Rust.mass_frame_dispatch, DynamicProcessors);
		if (SpatialQueryCallback)
		{
			Coordinator->SetSpatialQueryCallback(RustMassSpatialQuery::Trampoline, SpatialQueryRadius);
		}
		SimProcessors.Add(Coordinator);
	}

	if (SimProcessors.Num() > 0)
	{
		SimulationProcessorPipeline.SetProcessors(SimProcessors);
		SimulationProcessorPipeline.Initialize(*this, EntityManagerRef);
	}

	bProcessorPipelinesInitialized = true;
	return true;
}

// ---------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::RunSimulationProcessorStep(float SimulatedDeltaTime)
{
	if (!HasManagedSimulation())
	{
		return;
	}

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr || !EnsureProcessorPipelines(*MassEntitySubsystem))
	{
		return;
	}

	// Set spatial query trampoline context
	RustMassSpatialQuery::ActiveCallback = SpatialQueryCallback ? &SpatialQueryCallback : nullptr;

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	FMassProcessingContext SimulationContext(EntityManager, SimulatedDeltaTime);
	UE::Mass::Executor::Run(SimulationProcessorPipeline, SimulationContext);

	// Clear after dispatch
	RustMassSpatialQuery::ActiveCallback = nullptr;
}

// ---------------------------------------------------------------------------
// Group → Visualizer mapping
// ---------------------------------------------------------------------------

TArray<TArray<FMassEntityHandle>*> URustMassBevySubsystem::BuildGroupEntities()
{
	TArray<TArray<FMassEntityHandle>*> GroupEntitiesArr;
	if (!Visualizer) return GroupEntitiesArr;

	for (int32 i = 0; i < Visualizer->GetGroupCount(); ++i)
	{
		FString Name = Visualizer->GetGroupName(i);
		TArray<FMassEntityHandle>* Found = EntityGroups.Find(Name);
		GroupEntitiesArr.Add(Found); // nullptr if not found — visualizer handles gracefully
	}
	return GroupEntitiesArr;
}

// ---------------------------------------------------------------------------
// Tick
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::TryAutoInitFromRustDefaults()
{
	if (bAutoInitAttempted) return;
	bAutoInitAttempted = true;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (!Module.Plugin.Rust.get_sim_defaults.IsSome()) return;

	MassSimDefaultsDesc Defaults = {};
	if (Module.Plugin.Rust.get_sim_defaults.Unwrap()(&Defaults) == 0) return;

	TArray<TPair<FString, int32>> GroupCounts;
	for (uint32 i = 0; i < Defaults.num_groups; ++i)
	{
		FString Name = FString(Defaults.groups[i].name.len,
			UTF8_TO_TCHAR(Defaults.groups[i].name.ptr));
		GroupCounts.Add(TPair<FString, int32>(Name, Defaults.groups[i].count));
	}

	FBox Bounds(
		FVector(Defaults.bounds_min[0], Defaults.bounds_min[1], Defaults.bounds_min[2]),
		FVector(Defaults.bounds_max[0], Defaults.bounds_max[1], Defaults.bounds_max[2]));

	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-initializing from Rust defaults "
		"(%d groups, seed=%d)"), GroupCounts.Num(), Defaults.random_seed);

	InitializeSimulation(GroupCounts, Bounds, Defaults.random_seed);
}

void URustMassBevySubsystem::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!HasManagedSimulation())
	{
		TryAutoInitFromRustDefaults();
		if (!HasManagedSimulation()) return;
	}

	const float SimulatedSecondsThisFrame = FMath::Max(0.0f, DeltaTime);
	SimulationTimeAccumulatorSeconds += SimulatedSecondsThisFrame;

	const FVector BoundsSize = SimulationBounds.GetSize();
	const float BoundsMaxStepDistance = SimulationBounds.IsValid
		? 0.5f * FMath::Min(BoundsSize.X, BoundsSize.Y)
		: 500.0f;
	const float DefaultMovementSpeed = 100.0f;
	const float MaxStepSeconds = BoundsMaxStepDistance / FMath::Max(KINDA_SMALL_NUMBER, DefaultMovementSpeed);

	int32 StepsExecutedThisFrame = 0;
	constexpr int32 MaxStepsPerTick = 4096;
	while (SimulationTimeAccumulatorSeconds >= MaxStepSeconds
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(MaxStepSeconds);
		SimulationTimeAccumulatorSeconds -= MaxStepSeconds;
		++StepsExecutedThisFrame;
	}

	if (SimulationTimeAccumulatorSeconds > KINDA_SMALL_NUMBER
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(SimulationTimeAccumulatorSeconds);
		SimulationTimeAccumulatorSeconds = 0.0f;
	}

	// Sync visualization
	if (Visualizer)
	{
		UWorld* World = GetWorld();
		if (World != nullptr)
		{
			UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
			if (MassEntitySubsystem != nullptr)
			{
				FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
				TArray<TArray<FMassEntityHandle>*> GroupEntitiesArr = BuildGroupEntities();
				Visualizer->SyncInstances(EntityManager, GroupEntitiesArr);
			}
		}
	}
}

TStatId URustMassBevySubsystem::GetStatId() const
{
	RETURN_QUICK_DECLARE_CYCLE_STAT(URustMassBevySubsystem, STATGROUP_Tickables);
}

// ---------------------------------------------------------------------------
// Init / Reset
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::InitializeSimulation(
	const TArray<TPair<FString, int32>>& GroupCounts,
	const FBox& Bounds,
	int32 RandomSeed)
{
	ResetSimulation();

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr)
	{
		return;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	RustMassSpawnSetEntityManager(&EntityManager);
	SimulationBounds = Bounds;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (Module.Plugin.Rust.mass_init_simulation.IsSome())
	{
		// Build params with named groups
		TArray<MassEntityGroupDesc> Descs;
		TArray<TArray<uint8>> NameBuffers; // keep name bytes alive
		for (const auto& Pair : GroupCounts)
		{
			TArray<uint8> NameBytes;
			FTCHARToUTF8 Converter(*Pair.Key);
			NameBytes.Append(reinterpret_cast<const uint8*>(Converter.Get()), Converter.Length());
			NameBytes.Add(0); // null-terminate

			MassEntityGroupDesc Desc = {};
			Desc.name.ptr = reinterpret_cast<const char*>(NameBytes.GetData());
			Desc.name.len = Converter.Length();
			Desc.count = Pair.Value;
			Descs.Add(Desc);
			NameBuffers.Add(MoveTemp(NameBytes));
		}
		// Fix up pointers after NameBuffers is stable (no more Add)
		for (int32 i = 0; i < Descs.Num(); ++i)
		{
			Descs[i].name.ptr = reinterpret_cast<const char*>(NameBuffers[i].GetData());
		}

		MassInitSimulationParams Params = {};
		Params.groups = Descs.GetData();
		Params.num_groups = Descs.Num();
		Params.bounds_min[0] = Bounds.Min.X; Params.bounds_min[1] = Bounds.Min.Y; Params.bounds_min[2] = Bounds.Min.Z;
		Params.bounds_max[0] = Bounds.Max.X; Params.bounds_max[1] = Bounds.Max.Y; Params.bounds_max[2] = Bounds.Max.Z;
		Params.random_seed = RandomSeed;

		MassInitSimulationResult Result = {};
		Module.Plugin.Rust.mass_init_simulation.Unwrap()(&Params, &Result);

		for (uint32 g = 0; g < Result.num_groups; ++g)
		{
			const MassEntityGroupResult& Group = Result.groups[g];
			FString GroupName = FString(Group.name.len, UTF8_TO_TCHAR(Group.name.ptr));
			TArray<FMassEntityHandle>& Entities = EntityGroups.FindOrAdd(GroupName);
			for (uint32 i = 0; i < Group.count; ++i)
			{
				Entities.Add(FMassEntityHandle(Group.handles[i].index, Group.handles[i].serial_number));
			}
		}
	}

	// Initialize generic visualizer
	if (!Visualizer)
	{
		Visualizer = NewObject<URustMassGenericVisualizer>(this);
	}
	Visualizer->Initialize(World, Module.Plugin.Rust);
	{
		TArray<TArray<FMassEntityHandle>*> GroupEntitiesArr = BuildGroupEntities();
		Visualizer->RebuildInstances(EntityManager, GroupEntitiesArr);
	}

	int32 TotalEntities = 0;
	for (const auto& Pair : EntityGroups)
	{
		TotalEntities += Pair.Value.Num();
	}
	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Initialized %d entity groups (%d total entities)"),
		EntityGroups.Num(), TotalEntities);

	// Auto-setup spatial queries from Rust-registered config (if no callback already set)
	if (!SpatialQueryCallback)
	{
		SetupSpatialQueriesFromRust();
	}
}

void URustMassBevySubsystem::ResetSimulation()
{
	UWorld* World = GetWorld();
	UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;

	if (Visualizer)
	{
		Visualizer->Teardown();
	}

	if (MassEntitySubsystem != nullptr)
	{
		FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
		for (auto& Pair : EntityGroups)
		{
			for (const FMassEntityHandle Entity : Pair.Value)
			{
				if (EntityManager.IsEntityValid(Entity))
				{
					EntityManager.DestroyEntity(Entity);
				}
			}
		}
	}

	EntityGroups.Empty();
	SimulationBounds = FBox(EForceInit::ForceInit);
	SimulationTimeAccumulatorSeconds = 0.0f;
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	SpatialQueryCallback = nullptr;
	SpatialQueryRadius = 0.0f;
	bAutoInitAttempted = false;
}

void URustMassBevySubsystem::RunSimulationProcessorsForTesting(float DeltaTime)
{
	RunSimulationProcessorStep(FMath::Max(0.0f, DeltaTime));
}
