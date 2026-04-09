#include "GatherersBevyMassSubsystem.h"

#include "GameFramework/Actor.h"
#include "MassExecutor.h"
#include "MassEntityManager.h"
#include "MassProcessingContext.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "StructUtils/InstancedStruct.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.h"
#include "RustPlugin.h"
#include "RustUtils.h"
#include "GatherersMassRuntime.h"
#include "GatherersBevyMassVisualizer.h"



// ---------------------------------------------------------------------------
// Spatial query callback for Rust collision pre-pass
// ---------------------------------------------------------------------------

namespace BevyMassSpatialQuery
{
	// Thread-local state set before each dispatch call.
	// Safe because bRequiresGameThreadExecution = true on all processors.
	static const TArray<FMassEntityHandle>* FoodEntities = nullptr;
	static FMassEntityManager* CachedEntityManager = nullptr;

	uint32_t SpatialQueryCallback(
		const double* PreviousPos,
		const double* CurrentPos,
		float PickupRadius,
		MassSpatialQueryResult* Out)
	{
		if (!Out || !FoodEntities || !CachedEntityManager)
		{
			static int32 NullCount = 0;
			if (NullCount++ < 5)
			{
				UE_LOG(LogTemp, Warning, TEXT("SpatialQuery: null state - Out=%d Food=%d EM=%d"),
					!!Out, !!FoodEntities, !!CachedEntityManager);
			}
			return 0;
		}

		Out->has_encounter = false;
		Out->food_index = -1;
		Out->encounter_position[0] = 0.0;
		Out->encounter_position[1] = 0.0;
		Out->encounter_position[2] = 0.0;

		const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
		const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
		FMassEntityManager& EntityManager = *CachedEntityManager;

		float BestDistSq = TNumericLimits<float>::Max();
		const float PickupRadiusSq = PickupRadius * PickupRadius;

		if (SweepStart.Equals(SweepEnd))
		{
			static bool bWarnedOnce = false;
			if (!bWarnedOnce)
			{
				UE_LOG(LogTemp, Warning, TEXT("SpatialQuery: stationary ant detected — ants should always be moving"));
				bWarnedOnce = true;
			}
			return 1;
		}

		{
			// Direct distance check against food entity positions
			for (int32 i = 0; i < FoodEntities->Num(); ++i)
			{
				const FMassEntityHandle FoodEntity = (*FoodEntities)[i];
				if (!EntityManager.IsEntityValid(FoodEntity))
				{
					continue;
				}
				FMassEntityView FoodView(EntityManager, FoodEntity);
				const FGatherersMassFoodFragment& Food =
					FoodView.GetFragmentData<FGatherersMassFoodFragment>();
				if (!Food.bIsLoose)
				{
					continue;
				}
				const float DistSq = FVector::DistSquared(SweepEnd, Food.Position);
				if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->food_index = i;
					Out->encounter_position[0] = SweepEnd.X;
					Out->encounter_position[1] = SweepEnd.Y;
					Out->encounter_position[2] = SweepEnd.Z;
				}
			}
		}

		static int32 ResultLogCount = 0;
		if (ResultLogCount++ < 5 && Out->has_encounter)
		{
			UE_LOG(LogTemp, Log, TEXT("SpatialQuery: HIT food_index=%d pos=(%.1f,%.1f,%.1f)"),
				Out->food_index,
				Out->encounter_position[0], Out->encounter_position[1], Out->encounter_position[2]);
		}

		return 1;
	}
} // namespace BevyMassSpatialQuery

void UGatherersBevyMassSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
	bProcessorPipelinesInitialized = false;
}

void UGatherersBevyMassSubsystem::Deinitialize()
{
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	Super::Deinitialize();
}

UInstancedStaticMeshComponent* UGatherersBevyMassSubsystem::GetFoodISMC() const
{
	return Visualizer ? Visualizer->GetFoodISMC() : nullptr;
}

bool UGatherersBevyMassSubsystem::EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem)
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

	// Sort processors by execution order declared in Rust via #[mass_system(order = N)]
	DynamicProcessors.Sort([](const URustMassDynamicProcessor& A, const URustMassDynamicProcessor& B)
	{
		return A.GetExecutionOrder() < B.GetExecutionOrder();
	});

	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		SimProcessors.Add(Proc);
	}

	// Bevy schedule coordinator — dispatches all cached chunk data to Rust's
	// Bevy scheduler in a single call. Collision detection is handled by the
	// Rust collision pre-pass system via the spatial query callback.
	if (Module.Plugin.Rust.mass_frame_dispatch != nullptr && DynamicProcessors.Num() > 0)
	{
		URustMassScheduleCoordinator* Coordinator = NewObject<URustMassScheduleCoordinator>(this);
		Coordinator->InitializeDispatch(Module.Plugin.Rust.mass_frame_dispatch, DynamicProcessors);
		Coordinator->SetSpatialQueryCallback(BevyMassSpatialQuery::SpatialQueryCallback, GatherersMassPickupRadius);
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

void UGatherersBevyMassSubsystem::RunSimulationProcessorStep(float SimulatedDeltaTime)
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

	// Set spatial query context for the Rust collision pre-pass callback
	BevyMassSpatialQuery::FoodEntities = &ManagedFoodEntities;
	BevyMassSpatialQuery::CachedEntityManager = &MassEntitySubsystem->GetMutableEntityManager();

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	FMassProcessingContext SimulationContext(EntityManager, SimulatedDeltaTime);
	UE::Mass::Executor::Run(SimulationProcessorPipeline, SimulationContext);
}

void UGatherersBevyMassSubsystem::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!HasManagedSimulation())
	{
		return;
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
				Visualizer->SyncInstances(EntityManager, ManagedAntEntities, ManagedFoodEntities);
			}
		}
	}
}

TStatId UGatherersBevyMassSubsystem::GetStatId() const
{
	RETURN_QUICK_DECLARE_CYCLE_STAT(UGatherersBevyMassSubsystem, STATGROUP_Tickables);
}

void UGatherersBevyMassSubsystem::InitializeSimulation(int32 AntCount, int32 FoodCount, const FBox& Bounds, int32 RandomSeedBase)
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

	FRandomStream Random(RandomSeedBase);

	// Spawn food entities
	for (int32 FoodIndex = 0; FoodIndex < FoodCount; ++FoodIndex)
	{
		FGatherersMassFoodFragment FoodFragment;
		FoodFragment.Position = FVector(
			Random.FRandRange(Bounds.Min.X, Bounds.Max.X),
			Random.FRandRange(Bounds.Min.Y, Bounds.Max.Y),
			50.0);

		TArray<FInstancedStruct, TInlineAllocator<1>> FoodFragments;
		FoodFragments.Add(FInstancedStruct::Make(FoodFragment));
		const FMassEntityHandle FoodEntity = EntityManager.CreateEntity(FoodFragments);
		EntityManager.AddTagToEntity(FoodEntity, FGatherersMassFoodTag::StaticStruct());
		ManagedFoodEntities.Add(FoodEntity);
	}

	// Spawn ant entities
	const FVector BoundsCenter = Bounds.GetCenter();
	const float AntSpawnStep = 50.0f;

	for (int32 AntIndex = 0; AntIndex < AntCount; ++AntIndex)
	{
		const FVector SpawnPos(
			BoundsCenter.X + (AntIndex - AntCount / 2) * AntSpawnStep,
			BoundsCenter.Y + 100.0,
			50.0);
		const float Angle = Random.FRandRange(0.0f, 2.0f * PI);

		FGatherersPosition PositionFrag;
		PositionFrag.Position = SpawnPos;
		PositionFrag.PreviousPosition = SpawnPos;

		FGatherersMovement MovementFrag;
		MovementFrag.Direction = FVector(FMath::Cos(Angle), FMath::Sin(Angle), 0.0f);
		MovementFrag.MovementSpeed = 100.0f;

		FGatherersCooldown CooldownFrag;
		FGatherersCarrying CarryingFrag;

		FGatherersBehavior BehaviorFrag;
		BehaviorFrag.RandomSeed = RandomSeedBase + AntIndex;

		FGatherersAntEncounterFragment EncounterFrag;

		TArray<FInstancedStruct, TInlineAllocator<6>> AntFragments;
		AntFragments.Add(FInstancedStruct::Make(PositionFrag));
		AntFragments.Add(FInstancedStruct::Make(MovementFrag));
		AntFragments.Add(FInstancedStruct::Make(CooldownFrag));
		AntFragments.Add(FInstancedStruct::Make(CarryingFrag));
		AntFragments.Add(FInstancedStruct::Make(BehaviorFrag));
		AntFragments.Add(FInstancedStruct::Make(EncounterFrag));
		const FMassEntityHandle AntEntity = EntityManager.CreateEntity(AntFragments);
		EntityManager.AddTagToEntity(AntEntity, FGatherersBevyMassAntTag::StaticStruct());
		ManagedAntEntities.Add(AntEntity);
	}

	// Initialize visualizer
	if (!Visualizer)
	{
		Visualizer = NewObject<UGatherersBevyMassVisualizer>(this);
	}
	Visualizer->Initialize(World);
	Visualizer->RebuildInstances(EntityManager, ManagedAntEntities, ManagedFoodEntities);

	UE_LOG(LogTemp, Log, TEXT("GatherersBevyMass: Spawned %d ants and %d food (dynamic Rust systems)"),
		ManagedAntEntities.Num(), ManagedFoodEntities.Num());
}

void UGatherersBevyMassSubsystem::ResetSimulation()
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

		for (const FMassEntityHandle Entity : ManagedAntEntities)
		{
			if (EntityManager.IsEntityValid(Entity))
			{
				EntityManager.DestroyEntity(Entity);
			}
		}
		for (const FMassEntityHandle Entity : ManagedFoodEntities)
		{
			if (EntityManager.IsEntityValid(Entity))
			{
				EntityManager.DestroyEntity(Entity);
			}
		}
	}

	ManagedAntEntities.Reset();
	ManagedFoodEntities.Reset();
	SimulationBounds = FBox(EForceInit::ForceInit);
	SimulationTimeAccumulatorSeconds = 0.0f;
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
}

int32 UGatherersBevyMassSubsystem::GetManagedAntCount() const
{
	return ManagedAntEntities.Num();
}

int32 UGatherersBevyMassSubsystem::GetManagedFoodCount() const
{
	return ManagedFoodEntities.Num();
}

bool UGatherersBevyMassSubsystem::HasManagedSimulation() const
{
	return ManagedAntEntities.Num() > 0 || ManagedFoodEntities.Num() > 0;
}

void UGatherersBevyMassSubsystem::RunSimulationProcessorsForTesting(float DeltaTime)
{
	RunSimulationProcessorStep(FMath::Max(0.0f, DeltaTime));
}
