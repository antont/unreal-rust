#include "RustBobProcessor.h"
#include "MassExecutionContext.h"
#include "MassEntityManager.h"
#include "RustUtils.h"

URustBobProcessor::URustBobProcessor()
{
	bAutoRegisterWithProcessingPhases = true;
	bRequiresGameThreadExecution = true;
}

void URustBobProcessor::ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager)
{
	EntityQuery.AddRequirement<FBobFragment>(EMassFragmentAccess::ReadWrite);
	EntityQuery.RegisterWithProcessor(*this);
}

void URustBobProcessor::Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context)
{
	// TODO: call into Rust — stub for RED
}
