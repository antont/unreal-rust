#include "RustMassPostMovementProcessor.h"

#include "MassCommonFragments.h"
#include "MassExecutionContext.h"
#include "Generated/GatherersFragments.gen.h"

URustMassPostMovementProcessor::URustMassPostMovementProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = static_cast<uint8>(EProcessorExecutionFlags::All);
	EntityQuery.RegisterWithProcessor(*this);
}

void URustMassPostMovementProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& /*EntityManager*/)
{
	EntityQuery.AddRequirement<FTransformFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddRequirement<FGatherersPreviousTranslation>(EMassFragmentAccess::ReadWrite);
}

void URustMassPostMovementProcessor::Execute(
	FMassEntityManager& EntityManager,
	FMassExecutionContext& Context)
{
	const bool bHasBounds = SimBounds.IsValid;
	const FVector BoundsMin = SimBounds.Min;
	const FVector BoundsMax = SimBounds.Max;

	EntityQuery.ForEachEntityChunk(Context,
		[bHasBounds, BoundsMin, BoundsMax](FMassExecutionContext& ChunkContext)
	{
		TArrayView<FTransformFragment> Transforms = ChunkContext.GetMutableFragmentView<FTransformFragment>();
		TArrayView<FGatherersPreviousTranslation> PrevTranslations = ChunkContext.GetMutableFragmentView<FGatherersPreviousTranslation>();

		for (FMassExecutionContext::FEntityIterator It = ChunkContext.CreateEntityIterator(); It; ++It)
		{
			FTransform& Transform = Transforms[It].GetMutableTransform();
			FVector Position = Transform.GetTranslation();

			// Store current position for spatial sweep queries next frame
			PrevTranslations[It].Value = Position;

			// Boundary position clamp (velocity reflection is handled by Rust)
			if (bHasBounds)
			{
				Position.X = FMath::Clamp(Position.X, BoundsMin.X, BoundsMax.X);
				Position.Y = FMath::Clamp(Position.Y, BoundsMin.Y, BoundsMax.Y);
				Transform.SetTranslation(Position);
			}
		}
	});
}
