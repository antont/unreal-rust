#include "GatherersBevyMassSubsystem.h"

#include "Engine/StaticMesh.h"
#include "GameFramework/Actor.h"
#include "MassExecutor.h"
#include "MassEntityManager.h"
#include "MassProcessingContext.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "Materials/MaterialInterface.h"
#include "StructUtils/InstancedStruct.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.h"
#include "RustPlugin.h"
#include "GatherersMassRuntime.h"

constexpr ECollisionChannel BevyMassFoodQueryChannel = ECC_GameTraceChannel1;

// ---------------------------------------------------------------------------
// Spatial query callback for Rust collision pre-pass
// ---------------------------------------------------------------------------

namespace BevyMassSpatialQuery
{
	// Thread-local state set before each dispatch call.
	// Safe because bRequiresGameThreadExecution = true on all processors.
	static UWorld* QueryWorld = nullptr;
	static UInstancedStaticMeshComponent* FoodISM = nullptr;
	static const TArray<FMassEntityHandle>* FoodEntities = nullptr;
	static FMassEntityManager* CachedEntityManager = nullptr;

	uint32_t SpatialQueryCallback(
		const double* PreviousPos,
		const double* CurrentPos,
		float PickupRadius,
		MassSpatialQueryResult* Out)
	{
		if (!Out || !QueryWorld || !FoodISM || !FoodEntities || !CachedEntityManager)
		{
			static int32 NullCount = 0;
			if (NullCount++ < 5)
			{
				UE_LOG(LogTemp, Warning, TEXT("SpatialQuery: null state - Out=%d World=%d ISM=%d Food=%d EM=%d"),
					!!Out, !!QueryWorld, !!FoodISM, !!FoodEntities, !!CachedEntityManager);
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

		if (SweepStart.Equals(SweepEnd))
		{
			// Stationary — overlap query
			const TArray<int32> OverlappingIndices =
				FoodISM->GetInstancesOverlappingSphere(SweepStart, PickupRadius, true);

			for (const int32 InstanceIndex : OverlappingIndices)
			{
				if (!FoodEntities->IsValidIndex(InstanceIndex))
				{
					continue;
				}
				const FMassEntityHandle FoodEntity = (*FoodEntities)[InstanceIndex];
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
				const float DistSq = FVector::DistSquared(SweepStart, Food.Position);
				if (DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->food_index = InstanceIndex;
					Out->encounter_position[0] = SweepStart.X;
					Out->encounter_position[1] = SweepStart.Y;
					Out->encounter_position[2] = SweepStart.Z;
				}
			}
		}
		else
		{
			// Moving — sweep query
			TArray<FHitResult> SweepHits;
			FCollisionQueryParams QueryParams(SCENE_QUERY_STAT(BevyMassFoodSweep), false);
			QueryParams.bTraceComplex = false;

			const bool bHit = QueryWorld->SweepMultiByChannel(
				SweepHits,
				SweepStart,
				SweepEnd,
				FQuat::Identity,
				BevyMassFoodQueryChannel,
				FCollisionShape::MakeSphere(PickupRadius),
				QueryParams);

			if (bHit)
			{
				for (const FHitResult& Hit : SweepHits)
				{
					if (Hit.Component.Get() != FoodISM || !FoodEntities->IsValidIndex(Hit.Item))
					{
						continue;
					}
					const FMassEntityHandle FoodEntity = (*FoodEntities)[Hit.Item];
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
					const float DistSq = FVector::DistSquared(SweepStart, Food.Position);
					if (DistSq < BestDistSq)
					{
						BestDistSq = DistSq;
						Out->has_encounter = true;
						Out->food_index = Hit.Item;
						Out->encounter_position[0] = Hit.ImpactPoint.X;
						Out->encounter_position[1] = Hit.ImpactPoint.Y;
						Out->encounter_position[2] = Hit.ImpactPoint.Z;
					}
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

namespace
{
const FLinearColor BevyMassAntColor(0.2f, 0.8f, 0.8f, 1.0f);
const FLinearColor BevyMassFoodColor(192.0f / 255.0f, 2.0f / 255.0f, 2.0f / 255.0f, 1.0f);
const FVector BevyMassAntVisualScale(0.2f, 0.2f, 0.2f);
const FVector BevyMassFoodVisualScale(0.1f, 0.1f, 0.1f);
constexpr TCHAR BevyMassSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");
constexpr TCHAR BevyMassBasicMaterialPath[] = TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial");
constexpr TCHAR BevyMassVisualizerActorName[] = TEXT("GatherersBevyMassVisualizer");

FTransform BevyMassBuildVisualTransform(const FVector& Position, const FVector& VisualScale)
{
	return FTransform(FQuat::Identity, Position, VisualScale);
}

FVector BevyMassComputeFoodVisualPosition(
	FMassEntityManager& EntityManager,
	const TArray<FMassEntityHandle>& AntEntities,
	int32 FoodIndex,
	const FGatherersMassFoodFragment& FoodFragment)
{
	if (FoodFragment.bIsLoose)
	{
		return FoodFragment.Position;
	}

	for (const FMassEntityHandle AntEntity : AntEntities)
	{
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			continue;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersCarrying& CarryingFrag = AntView.GetFragmentData<FGatherersCarrying>();
		if (CarryingFrag.FoodIndex == FoodIndex)
		{
			const FGatherersPosition& PositionFrag = AntView.GetFragmentData<FGatherersPosition>();
			return PositionFrag.Position + ComputeCarriedFoodRelativeLocation(GatherersMassCarriedFoodHeight);
		}
	}

	return FoodFragment.Position;
}
}

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

	// Separate Rust systems by name for pipeline ordering
	URustMassDynamicProcessor* MovementProc = nullptr;
	URustMassDynamicProcessor* FoodDecisionProc = nullptr;
	TArray<URustMassDynamicProcessor*> OtherRustProcs;

	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		Proc->AddExtraTagRequirement(FGatherersBevyMassAntTag::StaticStruct());

		// Identify systems by name for ordered pipeline
		const FString& ProcName = Proc->GetSystemName();
		if (ProcName.Contains(TEXT("ant_movement")))
		{
			MovementProc = Proc;
		}
		else if (ProcName.Contains(TEXT("ant_food_decision")))
		{
			FoodDecisionProc = Proc;
		}
		else
		{
			OtherRustProcs.Add(Proc);
		}
	}

	// Pipeline order: movement → food decision → others (cooldown, boundary) → coordinator
	if (MovementProc)
	{
		SimProcessors.Add(MovementProc);
	}

	if (FoodDecisionProc)
	{
		SimProcessors.Add(FoodDecisionProc);
	}

	for (URustMassDynamicProcessor* Proc : OtherRustProcs)
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
	BevyMassSpatialQuery::QueryWorld = World;
	BevyMassSpatialQuery::FoodISM = FoodRepresentationComponent;
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
	UWorld* World = GetWorld();
	if (World != nullptr)
	{
		UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
		if (MassEntitySubsystem != nullptr)
		{
			SyncVisualInstances(*MassEntitySubsystem);
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

	RebuildVisualInstances(*MassEntitySubsystem);
	UE_LOG(LogTemp, Log, TEXT("GatherersBevyMass: Spawned %d ants and %d food (dynamic Rust systems)"),
		ManagedAntEntities.Num(), ManagedFoodEntities.Num());
}

void UGatherersBevyMassSubsystem::ResetSimulation()
{
	UWorld* World = GetWorld();
	UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;

	if (MassEntitySubsystem != nullptr)
	{
		FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
		if (AntVisualComponent != nullptr)
		{
			AntVisualComponent->ClearInstances();
		}
		if (FoodRepresentationComponent != nullptr)
		{
			FoodRepresentationComponent->ClearInstances();
		}
		if (VisualizerActor != nullptr)
		{
			VisualizerActor->Destroy();
			VisualizerActor = nullptr;
			AntVisualComponent = nullptr;
			FoodRepresentationComponent = nullptr;
		}

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

bool UGatherersBevyMassSubsystem::EnsureVisualComponents()
{
	if (!VisualSphereMesh)
	{
		VisualSphereMesh = LoadObject<UStaticMesh>(nullptr, BevyMassSphereMeshPath);
	}

	UMaterialInterface* BaseMaterial = LoadObject<UMaterialInterface>(nullptr, BevyMassBasicMaterialPath);
	if (VisualSphereMesh == nullptr || BaseMaterial == nullptr)
	{
		return false;
	}

	if (!AntVisualMaterial)
	{
		AntVisualMaterial = UMaterialInstanceDynamic::Create(BaseMaterial, this);
		if (AntVisualMaterial)
		{
			AntVisualMaterial->SetVectorParameterValue(TEXT("Color"), BevyMassAntColor);
		}
	}

	if (!FoodVisualMaterial)
	{
		FoodVisualMaterial = UMaterialInstanceDynamic::Create(BaseMaterial, this);
		if (FoodVisualMaterial)
		{
			FoodVisualMaterial->SetVectorParameterValue(TEXT("Color"), BevyMassFoodColor);
		}
	}

	if (AntVisualMaterial == nullptr || FoodVisualMaterial == nullptr)
	{
		return false;
	}

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return false;
	}

	if (!VisualizerActor)
	{
		FActorSpawnParameters SpawnParameters;
		SpawnParameters.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
		VisualizerActor = World->SpawnActor<AActor>(AActor::StaticClass(), FTransform::Identity, SpawnParameters);
	}

	if (VisualizerActor == nullptr)
	{
		return false;
	}

	USceneComponent* RootComponent = VisualizerActor->GetRootComponent();
	if (RootComponent == nullptr)
	{
		RootComponent = NewObject<USceneComponent>(VisualizerActor, TEXT("Root"));
		VisualizerActor->AddInstanceComponent(RootComponent);
		RootComponent->RegisterComponent();
		VisualizerActor->SetRootComponent(RootComponent);
	}

	if (!AntVisualComponent)
	{
		AntVisualComponent = NewObject<UInstancedStaticMeshComponent>(VisualizerActor, TEXT("BevyMassAnts"));
		VisualizerActor->AddInstanceComponent(AntVisualComponent);

		if (!AntVisualComponent->IsRegistered())
		{
			AntVisualComponent->SetupAttachment(RootComponent);
			AntVisualComponent->RegisterComponent();
		}

		AntVisualComponent->SetStaticMesh(VisualSphereMesh);
		AntVisualComponent->SetMaterial(0, AntVisualMaterial);
		AntVisualComponent->SetMobility(EComponentMobility::Movable);
		AntVisualComponent->SetCollisionEnabled(ECollisionEnabled::NoCollision);
		AntVisualComponent->SetCastShadow(false);
		AntVisualComponent->SetCanEverAffectNavigation(false);
	}

	if (!FoodRepresentationComponent)
	{
		FoodRepresentationComponent = NewObject<UInstancedStaticMeshComponent>(VisualizerActor, TEXT("BevyMassFood"));
		VisualizerActor->AddInstanceComponent(FoodRepresentationComponent);

		if (!FoodRepresentationComponent->IsRegistered())
		{
			FoodRepresentationComponent->SetupAttachment(RootComponent);
			FoodRepresentationComponent->RegisterComponent();
		}

		FoodRepresentationComponent->SetStaticMesh(VisualSphereMesh);
		FoodRepresentationComponent->SetMaterial(0, FoodVisualMaterial);
		FoodRepresentationComponent->SetMobility(EComponentMobility::Movable);
		FoodRepresentationComponent->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
		FoodRepresentationComponent->SetCollisionResponseToAllChannels(ECR_Ignore);
		FoodRepresentationComponent->SetCollisionResponseToChannel(BevyMassFoodQueryChannel, ECR_Block);
		FoodRepresentationComponent->SetCastShadow(false);
		FoodRepresentationComponent->SetCanEverAffectNavigation(false);
	}

	return true;
}

void UGatherersBevyMassSubsystem::RebuildVisualInstances(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (!EnsureVisualComponents())
	{
		return;
	}

	AntVisualComponent->ClearInstances();
	FoodRepresentationComponent->ClearInstances();

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	for (const FMassEntityHandle AntEntity : ManagedAntEntities)
	{
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			continue;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersPosition& PositionFrag = AntView.GetFragmentData<FGatherersPosition>();
		AntVisualComponent->AddInstance(BevyMassBuildVisualTransform(PositionFrag.Position, BevyMassAntVisualScale), true);
	}

	for (int32 FoodIndex = 0; FoodIndex < ManagedFoodEntities.Num(); ++FoodIndex)
	{
		const FMassEntityHandle FoodEntity = ManagedFoodEntities[FoodIndex];
		if (!EntityManager.IsEntityValid(FoodEntity))
		{
			continue;
		}

		FMassEntityView FoodView(EntityManager, FoodEntity);
		const FGatherersMassFoodFragment& FoodFragment = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
		FoodRepresentationComponent->AddInstance(
			BevyMassBuildVisualTransform(
				BevyMassComputeFoodVisualPosition(EntityManager, ManagedAntEntities, FoodIndex, FoodFragment),
				BevyMassFoodVisualScale),
			true);
	}
}

void UGatherersBevyMassSubsystem::SyncVisualInstances(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (!EnsureVisualComponents())
	{
		return;
	}

	if (AntVisualComponent->GetInstanceCount() != ManagedAntEntities.Num()
		|| FoodRepresentationComponent->GetInstanceCount() != ManagedFoodEntities.Num())
	{
		RebuildVisualInstances(MassEntitySubsystem);
		return;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	for (int32 AntIndex = 0; AntIndex < ManagedAntEntities.Num(); ++AntIndex)
	{
		const FMassEntityHandle AntEntity = ManagedAntEntities[AntIndex];
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			RebuildVisualInstances(MassEntitySubsystem);
			return;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersPosition& PositionFrag = AntView.GetFragmentData<FGatherersPosition>();
		AntVisualComponent->UpdateInstanceTransform(
			AntIndex,
			BevyMassBuildVisualTransform(PositionFrag.Position, BevyMassAntVisualScale),
			true,
			AntIndex == ManagedAntEntities.Num() - 1 && ManagedFoodEntities.Num() == 0,
			true);
	}

	for (int32 FoodIndex = 0; FoodIndex < ManagedFoodEntities.Num(); ++FoodIndex)
	{
		const FMassEntityHandle FoodEntity = ManagedFoodEntities[FoodIndex];
		if (!EntityManager.IsEntityValid(FoodEntity))
		{
			RebuildVisualInstances(MassEntitySubsystem);
			return;
		}

		FMassEntityView FoodView(EntityManager, FoodEntity);
		const FGatherersMassFoodFragment& FoodFragment = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
		FoodRepresentationComponent->UpdateInstanceTransform(
			FoodIndex,
			BevyMassBuildVisualTransform(
				BevyMassComputeFoodVisualPosition(EntityManager, ManagedAntEntities, FoodIndex, FoodFragment),
				BevyMassFoodVisualScale),
			true,
			FoodIndex == ManagedFoodEntities.Num() - 1,
			true);
	}
}
