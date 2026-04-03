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
#include "RustPlugin.h"

namespace
{
const FLinearColor BevyMassAntColor(0.2f, 0.8f, 0.8f, 1.0f);
const FVector BevyMassAntVisualScale(0.2f, 0.2f, 0.2f);
constexpr TCHAR BevyMassSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");
constexpr TCHAR BevyMassBasicMaterialPath[] = TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial");
constexpr TCHAR BevyMassVisualizerActorName[] = TEXT("GatherersBevyMassVisualizer");
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
	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		Proc->AddExtraTagRequirement(FGatherersBevyMassAntTag::StaticStruct());
		SimProcessors.Add(Proc);
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

void UGatherersBevyMassSubsystem::InitializeSimulation(int32 AntCount, const FBox& Bounds, int32 RandomSeedBase)
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
	const FVector BoundsCenter = Bounds.GetCenter();
	const float AntSpawnStep = 50.0f;

	for (int32 AntIndex = 0; AntIndex < AntCount; ++AntIndex)
	{
		FGatherersMassAntFragment AntFragment;
		AntFragment.Position = FVector(
			BoundsCenter.X + (AntIndex - AntCount / 2) * AntSpawnStep,
			BoundsCenter.Y + 100.0,
			50.0);
		AntFragment.PreviousPosition = AntFragment.Position;

		const float Angle = Random.FRandRange(0.0f, 2.0f * PI);
		AntFragment.Direction = FVector(FMath::Cos(Angle), FMath::Sin(Angle), 0.0f);
		AntFragment.RandomSeed = RandomSeedBase + AntIndex;
		AntFragment.MovementSpeed = 100.0f;
		AntFragment.TurnJitterRadians = PI / 2.0f;

		TArray<FInstancedStruct, TInlineAllocator<1>> AntFragments;
		AntFragments.Add(FInstancedStruct::Make(AntFragment));
		const FMassEntityHandle AntEntity = EntityManager.CreateEntity(AntFragments);
		EntityManager.AddTagToEntity(AntEntity, FGatherersBevyMassAntTag::StaticStruct());
		ManagedAntEntities.Add(AntEntity);
	}

	RebuildVisualInstances(*MassEntitySubsystem);
	UE_LOG(LogTemp, Log, TEXT("GatherersBevyMass: Spawned %d ants (dynamic Rust systems)"),
		ManagedAntEntities.Num());
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
		if (VisualizerActor != nullptr)
		{
			VisualizerActor->Destroy();
			VisualizerActor = nullptr;
			AntVisualComponent = nullptr;
		}

		for (const FMassEntityHandle Entity : ManagedAntEntities)
		{
			if (EntityManager.IsEntityValid(Entity))
			{
				EntityManager.DestroyEntity(Entity);
			}
		}
	}

	ManagedAntEntities.Reset();
	SimulationBounds = FBox(EForceInit::ForceInit);
	SimulationTimeAccumulatorSeconds = 0.0f;
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
}

int32 UGatherersBevyMassSubsystem::GetManagedAntCount() const
{
	return ManagedAntEntities.Num();
}

bool UGatherersBevyMassSubsystem::HasManagedSimulation() const
{
	return ManagedAntEntities.Num() > 0;
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

	if (AntVisualMaterial == nullptr)
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
		SpawnParameters.Name = BevyMassVisualizerActorName;
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

	return true;
}

void UGatherersBevyMassSubsystem::RebuildVisualInstances(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (!EnsureVisualComponents())
	{
		return;
	}

	AntVisualComponent->ClearInstances();

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	for (const FMassEntityHandle AntEntity : ManagedAntEntities)
	{
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			continue;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersMassAntFragment& AntFragment = AntView.GetFragmentData<FGatherersMassAntFragment>();
		AntVisualComponent->AddInstance(
			FTransform(FQuat::Identity, AntFragment.Position, BevyMassAntVisualScale), true);
	}
}

void UGatherersBevyMassSubsystem::SyncVisualInstances(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (!EnsureVisualComponents())
	{
		return;
	}

	if (AntVisualComponent->GetInstanceCount() != ManagedAntEntities.Num())
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
		const FGatherersMassAntFragment& AntFragment = AntView.GetFragmentData<FGatherersMassAntFragment>();
		AntVisualComponent->UpdateInstanceTransform(
			AntIndex,
			FTransform(FQuat::Identity, AntFragment.Position, BevyMassAntVisualScale),
			true,
			AntIndex == ManagedAntEntities.Num() - 1,
			true);
	}
}
