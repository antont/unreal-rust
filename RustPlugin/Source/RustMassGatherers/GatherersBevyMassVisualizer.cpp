#include "GatherersBevyMassVisualizer.h"

#include "Engine/StaticMesh.h"
#include "GameFramework/Actor.h"
#include "MassEntityManager.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "Materials/MaterialInterface.h"
#include "GatherersFragments.h"
#include "GatherersMassRuntime.h"
#include "GatherersAntSimulation.h"
#include "GeneratedFragments.h"

constexpr ECollisionChannel BevyMassVisFoodQueryChannel = ECC_GameTraceChannel1;

namespace
{
const FLinearColor VisAntColor(0.2f, 0.8f, 0.8f, 1.0f);
const FLinearColor VisFoodColor(192.0f / 255.0f, 2.0f / 255.0f, 2.0f / 255.0f, 1.0f);
const FVector VisAntScale(0.2f, 0.2f, 0.2f);
const FVector VisFoodScale(0.1f, 0.1f, 0.1f);
constexpr TCHAR VisSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");
constexpr TCHAR VisBasicMaterialPath[] = TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial");
constexpr TCHAR VisActorName[] = TEXT("GatherersBevyMassVisualizer");

FTransform BuildVisualTransform(const FVector& Position, const FVector& Scale)
{
	return FTransform(FQuat::Identity, Position, Scale);
}

FVector ComputeFoodVisualPosition(
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
} // anonymous namespace

bool UGatherersBevyMassVisualizer::Initialize(UWorld* World)
{
	return EnsureVisualComponents(World);
}

void UGatherersBevyMassVisualizer::Teardown()
{
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
}

bool UGatherersBevyMassVisualizer::EnsureVisualComponents(UWorld* World)
{
	if (!VisualSphereMesh)
	{
		VisualSphereMesh = LoadObject<UStaticMesh>(nullptr, VisSphereMeshPath);
	}

	UMaterialInterface* BaseMaterial = LoadObject<UMaterialInterface>(nullptr, VisBasicMaterialPath);
	if (VisualSphereMesh == nullptr || BaseMaterial == nullptr)
	{
		return false;
	}

	if (!AntVisualMaterial)
	{
		AntVisualMaterial = UMaterialInstanceDynamic::Create(BaseMaterial, this);
		if (AntVisualMaterial)
		{
			AntVisualMaterial->SetVectorParameterValue(TEXT("Color"), VisAntColor);
		}
	}

	if (!FoodVisualMaterial)
	{
		FoodVisualMaterial = UMaterialInstanceDynamic::Create(BaseMaterial, this);
		if (FoodVisualMaterial)
		{
			FoodVisualMaterial->SetVectorParameterValue(TEXT("Color"), VisFoodColor);
		}
	}

	if (AntVisualMaterial == nullptr || FoodVisualMaterial == nullptr)
	{
		return false;
	}

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
		FoodRepresentationComponent->SetCollisionResponseToChannel(BevyMassVisFoodQueryChannel, ECR_Block);
		FoodRepresentationComponent->SetCastShadow(false);
		FoodRepresentationComponent->SetCanEverAffectNavigation(false);
	}

	return true;
}

void UGatherersBevyMassVisualizer::RebuildInstances(
	FMassEntityManager& EntityManager,
	const TArray<FMassEntityHandle>& AntEntities,
	const TArray<FMassEntityHandle>& FoodEntities)
{
	if (AntVisualComponent == nullptr || FoodRepresentationComponent == nullptr)
	{
		return;
	}

	AntVisualComponent->ClearInstances();
	FoodRepresentationComponent->ClearInstances();

	for (const FMassEntityHandle AntEntity : AntEntities)
	{
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			continue;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersPosition& PositionFrag = AntView.GetFragmentData<FGatherersPosition>();
		AntVisualComponent->AddInstance(BuildVisualTransform(PositionFrag.Position, VisAntScale), true);
	}

	for (int32 FoodIndex = 0; FoodIndex < FoodEntities.Num(); ++FoodIndex)
	{
		const FMassEntityHandle FoodEntity = FoodEntities[FoodIndex];
		if (!EntityManager.IsEntityValid(FoodEntity))
		{
			continue;
		}

		FMassEntityView FoodView(EntityManager, FoodEntity);
		const FGatherersMassFoodFragment& FoodFragment = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
		FoodRepresentationComponent->AddInstance(
			BuildVisualTransform(
				ComputeFoodVisualPosition(EntityManager, AntEntities, FoodIndex, FoodFragment),
				VisFoodScale),
			true);
	}
}

void UGatherersBevyMassVisualizer::SyncInstances(
	FMassEntityManager& EntityManager,
	const TArray<FMassEntityHandle>& AntEntities,
	const TArray<FMassEntityHandle>& FoodEntities)
{
	if (AntVisualComponent == nullptr || FoodRepresentationComponent == nullptr)
	{
		return;
	}

	if (AntVisualComponent->GetInstanceCount() != AntEntities.Num()
		|| FoodRepresentationComponent->GetInstanceCount() != FoodEntities.Num())
	{
		RebuildInstances(EntityManager, AntEntities, FoodEntities);
		return;
	}

	for (int32 AntIndex = 0; AntIndex < AntEntities.Num(); ++AntIndex)
	{
		const FMassEntityHandle AntEntity = AntEntities[AntIndex];
		if (!EntityManager.IsEntityValid(AntEntity))
		{
			RebuildInstances(EntityManager, AntEntities, FoodEntities);
			return;
		}

		FMassEntityView AntView(EntityManager, AntEntity);
		const FGatherersPosition& PositionFrag = AntView.GetFragmentData<FGatherersPosition>();
		AntVisualComponent->UpdateInstanceTransform(
			AntIndex,
			BuildVisualTransform(PositionFrag.Position, VisAntScale),
			true,
			AntIndex == AntEntities.Num() - 1 && FoodEntities.Num() == 0,
			true);
	}

	for (int32 FoodIndex = 0; FoodIndex < FoodEntities.Num(); ++FoodIndex)
	{
		const FMassEntityHandle FoodEntity = FoodEntities[FoodIndex];
		if (!EntityManager.IsEntityValid(FoodEntity))
		{
			RebuildInstances(EntityManager, AntEntities, FoodEntities);
			return;
		}

		FMassEntityView FoodView(EntityManager, FoodEntity);
		const FGatherersMassFoodFragment& FoodFragment = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
		FoodRepresentationComponent->UpdateInstanceTransform(
			FoodIndex,
			BuildVisualTransform(
				ComputeFoodVisualPosition(EntityManager, AntEntities, FoodIndex, FoodFragment),
				VisFoodScale),
			true,
			FoodIndex == FoodEntities.Num() - 1,
			true);
	}
}
