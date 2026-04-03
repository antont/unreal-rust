#include "GatherersProcessors.h"

#include "MassEntityManager.h"
#include "MassEntityView.h"
#include "MassExecutionContext.h"
#include "Modules/ModuleManager.h"
#include "RustPlugin.h"
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
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (Module.Plugin.Rust.mass_ant_movement == nullptr)
	{
		return;
	}

	const FBox SimulationBounds = Context.GetSubsystemChecked<UGatherersRustSubsystem>().GetSimulationBounds();
	const float DeltaSeconds = Context.GetDeltaTimeSeconds();

	// Pass bounds as raw double[3] arrays matching Rust [f64; 3]
	const double BoundsMin[3] = { SimulationBounds.Min.X, SimulationBounds.Min.Y, SimulationBounds.Min.Z };
	const double BoundsMax[3] = { SimulationBounds.Max.X, SimulationBounds.Max.Y, SimulationBounds.Max.Z };
	const double* BoundsMinPtr = SimulationBounds.IsValid ? BoundsMin : nullptr;
	const double* BoundsMaxPtr = SimulationBounds.IsValid ? BoundsMax : nullptr;

	EntityQuery.ForEachEntityChunk(Context,
		[&Module, DeltaSeconds, BoundsMinPtr, BoundsMaxPtr](FMassExecutionContext& ChunkContext)
	{
		TArrayView<FGatherersMassAntFragment> AntFragments =
			ChunkContext.GetMutableFragmentView<FGatherersMassAntFragment>();
		// Zero-copy: pass fragment array directly to Rust.
		// FGatherersMassAntFragment fields start at offset 0 (EBO) matching Rust AntFragment.
		Module.Plugin.Rust.mass_ant_movement(
			&AntFragments[0].Position.X,
			AntFragments.Num(),
			DeltaSeconds,
			BoundsMinPtr,
			BoundsMaxPtr
		);
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
