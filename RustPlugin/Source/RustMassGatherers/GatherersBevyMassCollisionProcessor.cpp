#include "GatherersBevyMassCollisionProcessor.h"

#include "MassEntityManager.h"
#include "MassEntityView.h"
#include "MassExecutionContext.h"
#include "GatherersBevyMassSubsystem.h"
#include "GatherersFragments.h"
#include "GatherersMassRuntime.h"

UGatherersBevyMassCollisionProcessor::UGatherersBevyMassCollisionProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	bAllowMultipleInstances = false;
	ExecutionFlags = static_cast<uint8>(EProcessorExecutionFlags::All);
	EntityQuery.RegisterWithProcessor(*this);
}

void UGatherersBevyMassCollisionProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	EntityQuery.AddTagRequirement<FGatherersBevyMassAntTag>(EMassFragmentPresence::All);
	EntityQuery.AddRequirement<FGatherersMassAntFragment>(EMassFragmentAccess::ReadOnly);
	EntityQuery.AddRequirement<FGatherersAntEncounterFragment>(EMassFragmentAccess::ReadWrite);
}

void UGatherersBevyMassCollisionProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	UWorld* World = EntityManager.GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UGatherersBevyMassSubsystem* Subsystem = World->GetSubsystem<UGatherersBevyMassSubsystem>();
	if (Subsystem == nullptr || Subsystem->FoodRepresentationComponent == nullptr)
	{
		return;
	}

	UInstancedStaticMeshComponent* FoodISM = Subsystem->FoodRepresentationComponent;
	const TArray<FMassEntityHandle>& FoodEntities = Subsystem->ManagedFoodEntities;
	const float PickupRadius = GatherersMassPickupRadius;

	EntityQuery.ForEachEntityChunk(Context,
		[World, FoodISM, &FoodEntities, &EntityManager, PickupRadius]
		(FMassExecutionContext& ChunkContext)
	{
		const TConstArrayView<FGatherersMassAntFragment> Ants =
			ChunkContext.GetFragmentView<FGatherersMassAntFragment>();
		const TArrayView<FGatherersAntEncounterFragment> Encounters =
			ChunkContext.GetMutableFragmentView<FGatherersAntEncounterFragment>();

		for (int32 i = 0; i < ChunkContext.GetNumEntities(); ++i)
		{
			const FGatherersMassAntFragment& Ant = Ants[i];
			FGatherersAntEncounterFragment& Enc = Encounters[i];

			// Reset encounter
			Enc.bHasEncounter = false;
			Enc.NearestFoodEntity = FMassEntityHandle();
			Enc.EncounterPosition = FVector::ZeroVector;

			// Sweep from PreviousPosition to Position
			const FVector SweepStart = Ant.PreviousPosition;
			const FVector SweepEnd = Ant.Position;

			if (SweepStart.Equals(SweepEnd))
			{
				// Stationary — use overlap query
				const TArray<int32> OverlappingIndices =
					FoodISM->GetInstancesOverlappingSphere(SweepStart, PickupRadius, true);

				float BestDistSq = TNumericLimits<float>::Max();
				for (const int32 InstanceIndex : OverlappingIndices)
				{
					if (!FoodEntities.IsValidIndex(InstanceIndex))
					{
						continue;
					}
					const FMassEntityHandle FoodEntity = FoodEntities[InstanceIndex];
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
						Enc.bHasEncounter = true;
						Enc.NearestFoodEntity = FoodEntity;
						Enc.EncounterPosition = SweepStart;
					}
				}
			}
			else
			{
				// Moving — sweep query
				TArray<FHitResult> SweepHits;
				FCollisionQueryParams QueryParams(SCENE_QUERY_STAT(BevyMassFoodSweep), false);
				QueryParams.bTraceComplex = false;

				const bool bHit = World->SweepMultiByChannel(
					SweepHits,
					SweepStart,
					SweepEnd,
					FQuat::Identity,
					ECC_GameTraceChannel1,
					FCollisionShape::MakeSphere(PickupRadius),
					QueryParams);

				if (bHit)
				{
					float BestDistSq = TNumericLimits<float>::Max();
					for (const FHitResult& Hit : SweepHits)
					{
						if (Hit.Component.Get() != FoodISM || !FoodEntities.IsValidIndex(Hit.Item))
						{
							continue;
						}
						const FMassEntityHandle FoodEntity = FoodEntities[Hit.Item];
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
							Enc.bHasEncounter = true;
							Enc.NearestFoodEntity = FoodEntity;
							Enc.EncounterPosition = Hit.ImpactPoint;
						}
					}
				}
			}
		}
	});
}
