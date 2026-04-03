#include "GatherersProcessors.h"

#include "MassEntityManager.h"
#include "MassEntityView.h"
#include "MassExecutionContext.h"
#include "GatherersAntSimulation.h"
#include "GatherersMassRuntime.h"
#include "GatherersFragments.h"
#include "GatherersSubsystem.h"

namespace
{
uint8 GatherersProcessorExecutionFlags()
{
	return static_cast<uint8>(EProcessorExecutionFlags::All);
}
}

UGatherersTimeAccumulationProcessor::UGatherersTimeAccumulationProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = GatherersProcessorExecutionFlags();
	ProcessorRequirements.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadWrite);
}

void UGatherersTimeAccumulationProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	Context.GetMutableSubsystemChecked<UGatherersRustSubsystem>().AdvanceAccumulatedSimulationSeconds(
		Context.GetDeltaTimeSeconds());
}

UGatherersAntMovementProcessor::UGatherersAntMovementProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = GatherersProcessorExecutionFlags();
	ProcessorRequirements.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadOnly);
	EntityQuery.RegisterWithProcessor(*this);
}

void UGatherersAntMovementProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	EntityQuery.AddTagRequirement<FGatherersMassAntTag>(EMassFragmentPresence::All);
	EntityQuery.AddRequirement<FGatherersMassAntFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadOnly);
}

void UGatherersAntMovementProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	const FBox SimulationBounds = Context.GetSubsystemChecked<UGatherersRustSubsystem>().GetSimulationBounds();
	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	const FVector BoundsSize = SimulationBounds.GetSize();
	const float BoundsMaxStepDistance = SimulationBounds.IsValid
		? 0.5f * FMath::Min(BoundsSize.X, BoundsSize.Y)
		: TNumericLimits<float>::Max();

	EntityQuery.ForEachEntityChunk(Context, [SimulationBounds, DeltaSeconds, BoundsMaxStepDistance](FMassExecutionContext& ChunkContext)
	{
		const TArrayView<FGatherersMassAntFragment> AntFragments =
			ChunkContext.GetMutableFragmentView<FGatherersMassAntFragment>();
		for (int32 EntityIndex = 0; EntityIndex < ChunkContext.GetNumEntities(); ++EntityIndex)
		{
			FGatherersMassAntFragment& AntFragment = AntFragments[EntityIndex];
			AntFragment.PickupCooldownRemainingSeconds = ComputeRemainingPickupCooldown(
				AntFragment.PickupCooldownRemainingSeconds,
				DeltaSeconds);
			AntFragment.PreviousPosition = AntFragment.Position;
			AntFragment.Position = ComputeAntHeadingMovementStep(
				AntFragment.Position,
				AntFragment.Direction,
				AntFragment.MovementSpeed,
				BoundsMaxStepDistance,
				DeltaSeconds);

			if (!SimulationBounds.IsValid)
			{
				continue;
			}

			FVector InwardBoundaryNormal = FVector::ZeroVector;
			if (AntFragment.Position.X < SimulationBounds.Min.X)
			{
				AntFragment.Position.X = SimulationBounds.Min.X;
				InwardBoundaryNormal += FVector(1.0f, 0.0f, 0.0f);
			}
			else if (AntFragment.Position.X > SimulationBounds.Max.X)
			{
				AntFragment.Position.X = SimulationBounds.Max.X;
				InwardBoundaryNormal += FVector(-1.0f, 0.0f, 0.0f);
			}

			if (AntFragment.Position.Y < SimulationBounds.Min.Y)
			{
				AntFragment.Position.Y = SimulationBounds.Min.Y;
				InwardBoundaryNormal += FVector(0.0f, 1.0f, 0.0f);
			}
			else if (AntFragment.Position.Y > SimulationBounds.Max.Y)
			{
				AntFragment.Position.Y = SimulationBounds.Max.Y;
				InwardBoundaryNormal += FVector(0.0f, -1.0f, 0.0f);
			}

			if (!InwardBoundaryNormal.IsNearlyZero())
			{
				AntFragment.Direction = ComputeBoundaryTurnBackDirection(AntFragment.Direction, InwardBoundaryNormal);
			}
		}
	});
}

UGatherersFoodInteractionProcessor::UGatherersFoodInteractionProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = GatherersProcessorExecutionFlags();
	ProcessorRequirements.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadOnly);
	EntityQuery.RegisterWithProcessor(*this);
}

void UGatherersFoodInteractionProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	EntityQuery.AddTagRequirement<FGatherersMassAntTag>(EMassFragmentPresence::All);
	EntityQuery.AddRequirement<FGatherersMassAntFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadOnly);
}

void UGatherersFoodInteractionProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	const UGatherersRustSubsystem& Subsystem = Context.GetSubsystemChecked<UGatherersRustSubsystem>();

	EntityQuery.ForEachEntityChunk(Context, [&EntityManager, &Subsystem](FMassExecutionContext& ChunkContext)
	{
		const TArrayView<FGatherersMassAntFragment> AntFragments =
			ChunkContext.GetMutableFragmentView<FGatherersMassAntFragment>();
		for (int32 EntityIndex = 0; EntityIndex < ChunkContext.GetNumEntities(); ++EntityIndex)
		{
			FGatherersMassAntFragment& AntFragment = AntFragments[EntityIndex];
			const TArray<FGatherersMassFoodEncounter> NearbyEncounters = Subsystem.QueryLooseFoodEncountersAlongSweep(
				AntFragment.PreviousPosition,
				AntFragment.Position,
				GatherersMassPickupRadius);
			const FGatherersMassFoodEncounter* FirstEncounter = NearbyEncounters.IsEmpty() ? nullptr : &NearbyEncounters[0];

			FGatherersMassFoodFragment* NearbyFood = nullptr;
			if (FirstEncounter != nullptr && FirstEncounter->Entity.IsSet() && EntityManager.IsEntityValid(FirstEncounter->Entity))
			{
				FMassEntityView NearbyFoodView(EntityManager, FirstEncounter->Entity);
				NearbyFood = &NearbyFoodView.GetFragmentData<FGatherersMassFoodFragment>();
			}

			if (AntFragment.CarriedFoodEntity.IsValid() && NearbyFood != nullptr)
			{
				AntFragment.Position = FirstEncounter->EncounterPosition;
				AntFragment.Direction = ConsumeAntTurnDirection(AntFragment);

				if (EntityManager.IsEntityValid(AntFragment.CarriedFoodEntity))
				{
					FMassEntityView CarriedFoodView(EntityManager, AntFragment.CarriedFoodEntity);
					FGatherersMassFoodFragment& CarriedFoodFragment =
						CarriedFoodView.GetFragmentData<FGatherersMassFoodFragment>();
					CarriedFoodFragment.bIsLoose = true;
					CarriedFoodFragment.Position = AntFragment.Position;
				}

				AntFragment.CarriedFoodEntity.Reset();
				AntFragment.PickupCooldownRemainingSeconds = ComputePickupCooldownForSeparationDistance(
					GatherersMassPickupSeparationDistance,
					AntFragment.MovementSpeed);
			}
			else if (!AntFragment.CarriedFoodEntity.IsValid() && AntFragment.PickupCooldownRemainingSeconds <= 0.0f)
			{
				if (NearbyFood != nullptr)
				{
					AntFragment.Position = FirstEncounter->EncounterPosition;
					AntFragment.Direction = ConsumeAntTurnDirection(AntFragment);
					AntFragment.CarriedFoodEntity = FirstEncounter->Entity;
					NearbyFood->bIsLoose = false;
				}
			}
		}
	});
}

UGatherersVisualSyncProcessor::UGatherersVisualSyncProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = GatherersProcessorExecutionFlags();
	ProcessorRequirements.AddSubsystemRequirement<UGatherersRustSubsystem>(EMassFragmentAccess::ReadWrite);
}

void UGatherersVisualSyncProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	Context.GetMutableSubsystemChecked<UGatherersRustSubsystem>().SyncManagedVisuals();
}
