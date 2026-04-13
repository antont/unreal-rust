#include "RustMassGenericVisualizer.h"

#include "Engine/StaticMesh.h"
#include "GameFramework/Actor.h"
#include "MassEntityManager.h"
#include "MassEntityView.h"
#include "StructUtils/StructView.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "Materials/MaterialInterface.h"


namespace
{

constexpr TCHAR VisSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");
constexpr TCHAR VisBasicMaterialPath[] = TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial");

// Small palette for per-group colors.
const FLinearColor GroupColors[] = {
	FLinearColor(0.2f, 0.8f, 0.8f, 1.0f),   // cyan (ants)
	FLinearColor(0.75f, 0.01f, 0.01f, 1.0f), // red (food)
	FLinearColor(0.1f, 0.8f, 0.2f, 1.0f),    // green
	FLinearColor(0.9f, 0.7f, 0.1f, 1.0f),    // yellow
};
constexpr int32 NumGroupColors = UE_ARRAY_COUNT(GroupColors);

/** Look up a USTRUCT by C++ type name (e.g. "FGatherersPosition"). */
const UScriptStruct* FindFragmentStructByName(const FString& CppTypeName)
{
	// Strip F/U prefix — UE reflection registers structs without the prefix
	FString SearchName = CppTypeName;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	return FindFirstObject<UScriptStruct>(*SearchName, EFindFirstObjectOptions::NativeFirst);
}

} // anonymous namespace

bool URustMassGenericVisualizer::Initialize(UWorld* World, const RustBindings& Bindings)
{
	if (World == nullptr) return false;
	if (Bindings.get_visualizer_group_count.IsNone() || Bindings.get_visualizer_group_desc.IsNone())
	{
		return false;
	}

	const uint32 GroupCount = Bindings.get_visualizer_group_count.Unwrap()();
	UE_LOG(LogTemp, Display, TEXT("RustMassGenericVisualizer: GroupCount = %u"), GroupCount);
	if (GroupCount == 0) return false;

	// Load shared assets
	UStaticMesh* SphereMesh = LoadObject<UStaticMesh>(nullptr, VisSphereMeshPath);
	UMaterialInterface* BaseMaterial = LoadObject<UMaterialInterface>(nullptr, VisBasicMaterialPath);
	if (SphereMesh == nullptr || BaseMaterial == nullptr) return false;

	// Spawn actor
	if (!VisualizerActor)
	{
		FActorSpawnParameters SpawnParams;
		SpawnParams.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
		VisualizerActor = World->SpawnActor<AActor>(AActor::StaticClass(), FTransform::Identity, SpawnParams);
		if (!VisualizerActor) return false;

		USceneComponent* Root = NewObject<USceneComponent>(VisualizerActor, TEXT("Root"));
		VisualizerActor->AddInstanceComponent(Root);
		Root->RegisterComponent();
		VisualizerActor->SetRootComponent(Root);
	}

	USceneComponent* RootComponent = VisualizerActor->GetRootComponent();

	// Create one ISMC per visual group
	Groups.Reserve(GroupCount);
	for (uint32 i = 0; i < GroupCount; ++i)
	{
		MassVisualizerGroupDesc Desc = {};
		if (Bindings.get_visualizer_group_desc.Unwrap()(i, &Desc) == 0) continue;

		FString GroupName = FString(Desc.name.len, UTF8_TO_TCHAR(Desc.name.ptr));
		FString FragTypeName = FString(Desc.position_fragment_type.len, UTF8_TO_TCHAR(Desc.position_fragment_type.ptr));

		const UScriptStruct* FragStruct = FindFragmentStructByName(FragTypeName);
		if (FragStruct == nullptr)
		{
			UE_LOG(LogTemp, Warning, TEXT("RustMassGenericVisualizer: Could not find USTRUCT '%s' for group '%s'"),
				*FragTypeName, *GroupName);
			continue;
		}

		VisualGroup Group;
		Group.Name = GroupName;
		Group.PositionFragmentStruct = FragStruct;
		Group.PositionOffset = Desc.position_offset;
		Group.Scale = FVector(Desc.scale);

		// Create ISMC
		FName ComponentName(*FString::Printf(TEXT("VisGroup_%s"), *GroupName));
		Group.ISMC = NewObject<UInstancedStaticMeshComponent>(VisualizerActor, ComponentName);
		VisualizerActor->AddInstanceComponent(Group.ISMC);
		Group.ISMC->SetupAttachment(RootComponent);
		Group.ISMC->RegisterComponent();
		Group.ISMC->SetStaticMesh(SphereMesh);

		// Per-group color material
		UMaterialInstanceDynamic* MatInst = UMaterialInstanceDynamic::Create(BaseMaterial, this);
		if (MatInst)
		{
			MatInst->SetVectorParameterValue(TEXT("Color"), GroupColors[i % NumGroupColors]);
		}
		Group.ISMC->SetMaterial(0, MatInst);
		Group.ISMC->SetMobility(EComponentMobility::Movable);
		Group.ISMC->SetCollisionEnabled(ECollisionEnabled::NoCollision);
		Group.ISMC->SetCastShadow(false);
		Group.ISMC->SetCanEverAffectNavigation(false);

		UE_LOG(LogTemp, Display, TEXT("RustMassGenericVisualizer: Added group '%s' (frag='%s', offset=%u, scale=%.2f)"),
			*Group.Name, *FragTypeName, Group.PositionOffset, Desc.scale);
		Groups.Add(MoveTemp(Group));
	}

	UE_LOG(LogTemp, Display, TEXT("RustMassGenericVisualizer: Initialized %d groups"), Groups.Num());
	return Groups.Num() > 0;
}

void URustMassGenericVisualizer::Teardown()
{
	for (auto& Group : Groups)
	{
		if (Group.ISMC)
		{
			Group.ISMC->ClearInstances();
		}
	}
	Groups.Empty();

	if (VisualizerActor)
	{
		VisualizerActor->Destroy();
		VisualizerActor = nullptr;
	}
}

void URustMassGenericVisualizer::RebuildInstances(
	FMassEntityManager& EM,
	const TArray<TArray<FMassEntityHandle>*>& GroupEntities)
{
	const int32 Count = FMath::Min(Groups.Num(), GroupEntities.Num());
	for (int32 g = 0; g < Count; ++g)
	{
		VisualGroup& Group = Groups[g];
		if (!Group.ISMC || !GroupEntities[g]) continue;

		Group.ISMC->ClearInstances();
		const TArray<FMassEntityHandle>& Entities = *GroupEntities[g];

		int32 SkippedInvalid = 0;
		int32 SkippedNullFrag = 0;
		for (int32 i = 0; i < Entities.Num(); ++i)
		{
			if (!EM.IsEntityValid(Entities[i])) { ++SkippedInvalid; continue; }
			FMassEntityView View(EM, Entities[i]);
			FStructView FragView = View.GetFragmentDataStruct(Group.PositionFragmentStruct);
			const uint8* FragData = FragView.GetMemory();
			if (!FragData) { ++SkippedNullFrag; continue; }
			const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
			FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
			Group.ISMC->AddInstance(T, true);
		}
		UE_LOG(LogTemp, Display, TEXT("RustMassGenericVisualizer::RebuildInstances group '%s': %d entities, %d instances added, %d invalid, %d null frag"),
			*Group.Name, Entities.Num(), Group.ISMC->GetInstanceCount(), SkippedInvalid, SkippedNullFrag);
	}
}

void URustMassGenericVisualizer::SyncInstances(
	FMassEntityManager& EM,
	const TArray<TArray<FMassEntityHandle>*>& GroupEntities)
{
	const int32 Count = FMath::Min(Groups.Num(), GroupEntities.Num());
	for (int32 g = 0; g < Count; ++g)
	{
		VisualGroup& Group = Groups[g];
		if (!Group.ISMC || !GroupEntities[g]) continue;

		const TArray<FMassEntityHandle>& Entities = *GroupEntities[g];

		// Rebuild if count mismatch
		if (Group.ISMC->GetInstanceCount() != Entities.Num())
		{
			RebuildInstances(EM, GroupEntities);
			return;
		}

		for (int32 i = 0; i < Entities.Num(); ++i)
		{
			if (!EM.IsEntityValid(Entities[i]))
			{
				RebuildInstances(EM, GroupEntities);
				return;
			}
			FMassEntityView View(EM, Entities[i]);
			FStructView FragView = View.GetFragmentDataStruct(Group.PositionFragmentStruct);
			const uint8* FragData = FragView.GetMemory();
			if (!FragData) continue;
			const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
			FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
			const bool bMarkDirty = (i == Entities.Num() - 1 && g == Count - 1);
			Group.ISMC->UpdateInstanceTransform(i, T, true, bMarkDirty, true);
		}
	}
}

void URustMassGenericVisualizer::RecreateCollisionPhysics()
{
	for (auto& Group : Groups)
	{
		if (Group.ISMC && Group.ISMC->GetCollisionEnabled() != ECollisionEnabled::NoCollision)
		{
			Group.ISMC->RecreatePhysicsState();
		}
	}
}
