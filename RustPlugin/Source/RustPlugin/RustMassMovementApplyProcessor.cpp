#include "RustMassMovementApplyProcessor.h"

#include "MassCommonFragments.h"
#include "MassExecutionContext.h"
#include "MassMovementFragments.h"
#include "MassRepresentationFragments.h"
#include "Generated/GatherersFragments.gen.h"

URustMassMovementApplyProcessor::URustMassMovementApplyProcessor()
{
	bAutoRegisterWithProcessingPhases = false;
	bRequiresGameThreadExecution = true;
	ExecutionFlags = static_cast<uint8>(EProcessorExecutionFlags::All);
	EntityQuery.RegisterWithProcessor(*this);
}

void URustMassMovementApplyProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& /*EntityManager*/)
{
	EntityQuery.AddRequirement<FTransformFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddRequirement<FMassVelocityFragment>(EMassFragmentAccess::ReadOnly);
	EntityQuery.AddRequirement<FGatherersPreviousTranslation>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddRequirement<FMassRepresentationFragment>(EMassFragmentAccess::ReadWrite, EMassFragmentPresence::Optional);
}

void URustMassMovementApplyProcessor::Execute(
	FMassEntityManager& EntityManager,
	FMassExecutionContext& Context)
{
	const float DeltaTime = Context.GetDeltaTimeSeconds();
	if (DeltaTime <= 0.0f)
	{
		return;
	}

	const bool bHasBounds = SimBounds.IsValid;
	const FVector BoundsMin = SimBounds.Min;
	const FVector BoundsMax = SimBounds.Max;

	EntityQuery.ForEachEntityChunk(Context,
		[DeltaTime, bHasBounds, BoundsMin, BoundsMax](FMassExecutionContext& ChunkContext)
	{
		TArrayView<FTransformFragment> Transforms = ChunkContext.GetMutableFragmentView<FTransformFragment>();
		TConstArrayView<FMassVelocityFragment> Velocities = ChunkContext.GetFragmentView<FMassVelocityFragment>();
		TArrayView<FGatherersPreviousTranslation> PrevTranslations = ChunkContext.GetMutableFragmentView<FGatherersPreviousTranslation>();
		TArrayView<FMassRepresentationFragment> RepFragments = ChunkContext.GetMutableFragmentView<FMassRepresentationFragment>();
		const bool bHasRepFragments = RepFragments.Num() > 0;

		for (FMassExecutionContext::FEntityIterator It = ChunkContext.CreateEntityIterator(); It; ++It)
		{
			FTransformFragment& TransformFrag = Transforms[It];
			const FMassVelocityFragment& Vel = Velocities[It];
			FGatherersPreviousTranslation& Prev = PrevTranslations[It];

			FTransform& Transform = TransformFrag.GetMutableTransform();
			FVector Position = Transform.GetTranslation();

			// Store previous position for spatial sweep queries
			Prev.Value = Position;

			// Update vis PrevTransform for correct motion blur / TAA
			if (bHasRepFragments)
			{
				RepFragments[It].PrevTransform = Transform;
			}

			// Apply velocity
			Position += Vel.Value * static_cast<double>(DeltaTime);

			// Boundary position clamp (velocity reflection is handled by Rust)
			if (bHasBounds)
			{
				Position.X = FMath::Clamp(Position.X, BoundsMin.X, BoundsMax.X);
				Position.Y = FMath::Clamp(Position.Y, BoundsMin.Y, BoundsMax.Y);
			}

			Transform.SetTranslation(Position);
		}
	});
}
