#include "RustMassMovementApplyProcessor.h"

#include "MassCommonFragments.h"
#include "MassExecutionContext.h"
#include "MassMovementFragments.h"
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
	EntityQuery.AddRequirement<FMassVelocityFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.AddRequirement<FGatherersPreviousTranslation>(EMassFragmentAccess::ReadWrite);
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
		TArrayView<FMassVelocityFragment> Velocities = ChunkContext.GetMutableFragmentView<FMassVelocityFragment>();
		TArrayView<FGatherersPreviousTranslation> PrevTranslations = ChunkContext.GetMutableFragmentView<FGatherersPreviousTranslation>();

		for (FMassExecutionContext::FEntityIterator It = ChunkContext.CreateEntityIterator(); It; ++It)
		{
			FTransformFragment& TransformFrag = Transforms[It];
			FMassVelocityFragment& Vel = Velocities[It];
			FGatherersPreviousTranslation& Prev = PrevTranslations[It];

			FTransform& Transform = TransformFrag.GetMutableTransform();
			FVector Position = Transform.GetTranslation();

			// Store previous position for spatial sweep queries
			Prev.Value = Position;

			// Apply velocity
			Position += Vel.Value * static_cast<double>(DeltaTime);

			// Boundary clamp + velocity reflection
			if (bHasBounds)
			{
				FVector InwardNormal = FVector::ZeroVector;

				if (Position.X < BoundsMin.X)
				{
					Position.X = BoundsMin.X;
					InwardNormal.X += 1.0;
				}
				else if (Position.X > BoundsMax.X)
				{
					Position.X = BoundsMax.X;
					InwardNormal.X -= 1.0;
				}

				if (Position.Y < BoundsMin.Y)
				{
					Position.Y = BoundsMin.Y;
					InwardNormal.Y += 1.0;
				}
				else if (Position.Y > BoundsMax.Y)
				{
					Position.Y = BoundsMax.Y;
					InwardNormal.Y -= 1.0;
				}

				// Reflect velocity at boundary
				if (!InwardNormal.IsNearlyZero())
				{
					const double Speed = Vel.Value.Size();
					if (Speed > 1e-8)
					{
						const FVector Dir = Vel.Value / Speed;
						const FVector Normal = InwardNormal.GetSafeNormal();
						const FVector Reflected = Dir - 2.0 * FVector::DotProduct(Dir, Normal) * Normal;
						if (!Reflected.IsNearlyZero())
						{
							Vel.Value = Reflected.GetSafeNormal() * Speed;
						}
					}
				}
			}

			Transform.SetTranslation(Position);
		}
	});
}
