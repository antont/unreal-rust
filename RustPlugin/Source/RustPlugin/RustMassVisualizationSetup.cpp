#include "RustMassVisualizationSetup.h"

#include "MassCommonFragments.h"
#include "MassEntityManager.h"
#include "MassEntityTypes.h"
#include "MassRepresentationFragments.h"
#include "MassRepresentationSubsystem.h"
#include "MassRepresentationTypes.h"
#include "MassLODFragments.h"
#include "MassLODTypes.h"
#include "MassLODCalculator.h"
#include "MassActorSubsystem.h"
#include "MassRepresentationProcessor.h"
#include "MassVisualizationLODProcessor.h"
#include "Engine/StaticMesh.h"
#include "Materials/MaterialInterface.h"

bool URustMassVisualizationSetup::SetupGroupVisualization(
	UWorld* World,
	FMassEntityManager& EntityManager,
	const TArray<FMassEntityHandle>& Entities,
	UStaticMesh* Mesh,
	UMaterialInterface* Material,
	FVector Scale,
	const float LODDistances[4])
{
	if (!World || !Mesh || Entities.Num() == 0)
	{
		return false;
	}

	// Get the MassRepresentation subsystem
	UMassRepresentationSubsystem* RepSubsystem = World->GetSubsystem<UMassRepresentationSubsystem>();
	if (!RepSubsystem)
	{
		UE_LOG(LogTemp, Warning, TEXT("RustMassVisualizationSetup: No MassRepresentationSubsystem"));
		return false;
	}

	// -----------------------------------------------------------------------
	// 1. Register static mesh descriptor with the representation subsystem
	// -----------------------------------------------------------------------
	FStaticMeshInstanceVisualizationDesc MeshDesc;
	FMassStaticMeshInstanceVisualizationMeshDesc MeshDetail;
	MeshDetail.Mesh = Mesh;
	MeshDetail.LocalTransform = FTransform(FQuat::Identity, FVector::ZeroVector, Scale);
	MeshDetail.bCastShadows = false;
	MeshDetail.MinLODSignificance = 0.0f;
	MeshDetail.MaxLODSignificance = static_cast<float>(EMassLOD::Max);
	MeshDetail.Mobility = EComponentMobility::Movable;
	if (Material)
	{
		MeshDetail.MaterialOverrides.Add(Material);
	}
	MeshDesc.Meshes.Add(MeshDetail);

	FStaticMeshInstanceVisualizationDescHandle DescHandle =
		RepSubsystem->FindOrAddStaticMeshDesc(MeshDesc);
	if (!DescHandle.IsValid())
	{
		UE_LOG(LogTemp, Warning, TEXT("RustMassVisualizationSetup: Failed to register mesh descriptor"));
		return false;
	}

	// -----------------------------------------------------------------------
	// 2. Build representation parameters (all LODs use ISM for placeholder)
	// -----------------------------------------------------------------------
	FMassRepresentationParameters RepParams;
	RepParams.LODRepresentation[EMassLOD::High] = EMassRepresentationType::StaticMeshInstance;
	RepParams.LODRepresentation[EMassLOD::Medium] = EMassRepresentationType::StaticMeshInstance;
	RepParams.LODRepresentation[EMassLOD::Low] = EMassRepresentationType::StaticMeshInstance;
	RepParams.LODRepresentation[EMassLOD::Off] = EMassRepresentationType::StaticMeshInstance;
	RepParams.ComputeCachedValues();

	// -----------------------------------------------------------------------
	// 3. Build LOD parameters
	// -----------------------------------------------------------------------
	FMassVisualizationLODParameters LODParams;
	LODParams.BaseLODDistance[EMassLOD::High] = LODDistances[0];
	LODParams.BaseLODDistance[EMassLOD::Medium] = LODDistances[1];
	LODParams.BaseLODDistance[EMassLOD::Low] = LODDistances[2];
	LODParams.BaseLODDistance[EMassLOD::Off] = LODDistances[3];
	LODParams.VisibleLODDistance[EMassLOD::High] = 0.0f;
	LODParams.VisibleLODDistance[EMassLOD::Medium] = LODDistances[1] * 0.8f;
	LODParams.VisibleLODDistance[EMassLOD::Low] = LODDistances[2] * 0.8f;
	LODParams.VisibleLODDistance[EMassLOD::Off] = LODDistances[3] * 0.8f;
	LODParams.LODMaxCount[EMassLOD::High] = 100;
	LODParams.LODMaxCount[EMassLOD::Medium] = 500;
	LODParams.LODMaxCount[EMassLOD::Low] = 5000;
	LODParams.LODMaxCount[EMassLOD::Off] = 0;

	// -----------------------------------------------------------------------
	// 4. Create shared fragment values
	// -----------------------------------------------------------------------

	// Const shared: RepresentationParameters, LODParameters
	FConstSharedStruct RepParamsFragment = EntityManager.GetOrCreateConstSharedFragment(RepParams);
	FConstSharedStruct LODParamsFragment = EntityManager.GetOrCreateConstSharedFragment(LODParams);

	// Shared: RepresentationSubsystem reference, LOD calculator
	FMassRepresentationSubsystemSharedFragment SubsystemSharedFragmentData;
	SubsystemSharedFragmentData.RepresentationSubsystem = RepSubsystem;
	FSharedStruct SubsystemFragment = EntityManager.GetOrCreateSharedFragment(SubsystemSharedFragmentData);

	FMassVisualizationLODSharedFragment LODSharedFragmentData;
	LODSharedFragmentData.LODCalculator.Initialize(
		LODParams.BaseLODDistance,
		LODParams.BufferHysteresisOnDistancePercentage / 100.f,
		LODParams.LODMaxCount,
		/*InLODMaxCountPerViewer=*/nullptr,
		LODParams.DistanceToFrustum,
		LODParams.DistanceToFrustumHysteresis,
		LODParams.VisibleLODDistance);
	LODSharedFragmentData.FilterTag = LODParams.FilterTag;
	FSharedStruct LODSharedFragment = EntityManager.GetOrCreateSharedFragment<FMassVisualizationLODSharedFragment>(
		FConstStructView::Make(LODParams), LODSharedFragmentData);

	// -----------------------------------------------------------------------
	// 5. Build target archetype: original entity composition + vis composition
	//    (including chunk fragments, which AddCompositionToEntity_GetDelta cannot add)
	// -----------------------------------------------------------------------
	UE_LOG(LogTemp, Display, TEXT("RustMassVisualizationSetup: Adding vis to %d entities"), Entities.Num());

	// Find the first valid entity to get the source archetype
	FMassEntityHandle FirstValidEntity;
	for (const FMassEntityHandle& Entity : Entities)
	{
		if (EntityManager.IsEntityValid(Entity))
		{
			FirstValidEntity = Entity;
			break;
		}
	}
	if (!FirstValidEntity.IsSet())
	{
		UE_LOG(LogTemp, Warning, TEXT("RustMassVisualizationSetup: No valid entities found"));
		return false;
	}

	// Get the source archetype's composition and build a combined composition
	const FMassArchetypeHandle SourceArchetype = EntityManager.GetArchetypeForEntity(FirstValidEntity);
	FMassArchetypeCompositionDescriptor CombinedComposition = EntityManager.GetArchetypeComposition(SourceArchetype);

	// Add vis per-entity fragments
	CombinedComposition.GetFragments().Add(*FMassRepresentationFragment::StaticStruct());
	CombinedComposition.GetFragments().Add(*FMassRepresentationLODFragment::StaticStruct());
	CombinedComposition.GetFragments().Add(*FMassActorFragment::StaticStruct());
	CombinedComposition.GetFragments().Add(*FMassViewerInfoFragment::StaticStruct());

	// Add vis tags
	CombinedComposition.GetTags().Add(*FMassVisualizationProcessorTag::StaticStruct());
	CombinedComposition.GetTags().Add(*FMassVisualizationLODProcessorTag::StaticStruct());
	CombinedComposition.GetTags().Add(*FMassCollectDistanceLODViewerInfoTag::StaticStruct());
	// Note: FMassVisibilityCulledByDistanceTag omitted intentionally.
	// Entities with that tag that start culled never get updated due to chunk filter optimization.
	// Without it, entities match CloseEntityQuery and always get LOD updates.

	// Add chunk fragment (critical for vis processor query matching)
	CombinedComposition.GetChunkFragments().Add(*FMassVisualizationChunkFragment::StaticStruct());

	// Add shared fragment types
	CombinedComposition.GetSharedFragments().Add(*FMassRepresentationSubsystemSharedFragment::StaticStruct());
	CombinedComposition.GetSharedFragments().Add(*FMassVisualizationLODSharedFragment::StaticStruct());
	CombinedComposition.GetConstSharedFragments().Add(*FMassRepresentationParameters::StaticStruct());
	CombinedComposition.GetConstSharedFragments().Add(*FMassVisualizationLODParameters::StaticStruct());

	// Create the target archetype with the full combined composition
	FMassArchetypeHandle TargetArchetype = EntityManager.CreateArchetype(CombinedComposition);

	// Build shared fragment values for the move
	FMassArchetypeSharedFragmentValues SharedFragmentValues;
	SharedFragmentValues.Add(SubsystemFragment);
	SharedFragmentValues.Add(LODSharedFragment);
	SharedFragmentValues.Add(RepParamsFragment);
	SharedFragmentValues.Add(LODParamsFragment);
	SharedFragmentValues.Sort();

	// -----------------------------------------------------------------------
	// 6. Move all entities to the target archetype and initialize vis fragments
	// -----------------------------------------------------------------------
	int32 SetupCount = 0;
	int32 LoggedCount = 0;
	for (const FMassEntityHandle& Entity : Entities)
	{
		if (!EntityManager.IsEntityValid(Entity))
		{
			continue;
		}

		// Move entity to the combined archetype (preserves existing fragment data)
		EntityManager.MoveEntityToAnotherArchetype(Entity, TargetArchetype, &SharedFragmentValues);

		// Initialize the representation fragment with our ISM handle
		FMassEntityView View(EntityManager, Entity);
		FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
		RepFrag.StaticMeshDescHandle = DescHandle;
		RepFrag.CurrentRepresentation = EMassRepresentationType::None;
		RepFrag.PrevRepresentation = EMassRepresentationType::None;
		RepFrag.HighResTemplateActorIndex = INDEX_NONE;
		RepFrag.LowResTemplateActorIndex = INDEX_NONE;
		const FTransform& EntityTransform = View.GetFragmentData<FTransformFragment>().GetTransform();

		// Diagnostic: log rotation bytes for the first few entities to confirm
		// what Rust actually wrote into the Transform fragment.
		if (LoggedCount < 3)
		{
			const FQuat Rot = EntityTransform.GetRotation();
			const FVector Trans = EntityTransform.GetTranslation();
			const FVector EntityScale = EntityTransform.GetScale3D();
			const double Norm2 = Rot.X * Rot.X + Rot.Y * Rot.Y + Rot.Z * Rot.Z + Rot.W * Rot.W;
			UE_LOG(LogTemp, Display,
				TEXT("RustMassVisualizationSetup: entity[%d] rot=(%.6f,%.6f,%.6f,%.6f) |rot|^2=%.6f trans=(%.2f,%.2f,%.2f) scale=(%.4f,%.4f,%.4f)"),
				LoggedCount,
				Rot.X, Rot.Y, Rot.Z, Rot.W, Norm2,
				Trans.X, Trans.Y, Trans.Z,
				EntityScale.X, EntityScale.Y, EntityScale.Z);
			++LoggedCount;
		}

		RepFrag.PrevTransform = EntityTransform;

		// Initialize LOD fragment to High so the first vis processor pass
		// sees a valid LOD (default is EMassLOD::Max which maps to Off → None).
		FMassRepresentationLODFragment& LODFrag = View.GetFragmentData<FMassRepresentationLODFragment>();
		LODFrag.LOD = EMassLOD::High;
		LODFrag.PrevLOD = EMassLOD::High;
		LODFrag.LODSignificance = 0.0f;
		LODFrag.Visibility = EMassVisibility::CanBeSeen;
		LODFrag.PrevVisibility = EMassVisibility::CanBeSeen;

		++SetupCount;
	}

	UE_LOG(LogTemp, Display,
		TEXT("RustMassVisualizationSetup: Configured %d/%d entities with native visualization"),
		SetupCount, Entities.Num());

	return SetupCount > 0;
}
