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
	EntityQuery.AddRequirement<FTransformFragment>(EMassFragmentAccess::ReadOnly);
	EntityQuery.AddRequirement<FGatherersPreviousTranslationFragment>(EMassFragmentAccess::ReadWrite);
}

void URustMassPostMovementProcessor::Execute(
	FMassEntityManager& EntityManager,
	FMassExecutionContext& Context)
{
	EntityQuery.ForEachEntityChunk(Context,
		[](FMassExecutionContext& ChunkContext)
	{
		TConstArrayView<FTransformFragment> Transforms = ChunkContext.GetFragmentView<FTransformFragment>();
		TArrayView<FGatherersPreviousTranslationFragment> PrevTranslations = ChunkContext.GetMutableFragmentView<FGatherersPreviousTranslationFragment>();

		for (FMassExecutionContext::FEntityIterator It = ChunkContext.CreateEntityIterator(); It; ++It)
		{
			PrevTranslations[It].Value = Transforms[It].GetTransform().GetTranslation();
		}
	});
}
