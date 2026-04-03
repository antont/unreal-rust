#include "RustBobProcessor.h"
#include "MassExecutionContext.h"
#include "MassEntityManager.h"
#include "Modules/ModuleManager.h"
#include "RustPlugin.h"

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
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (Module.Plugin.Rust.mass_bob_process == nullptr)
	{
		return;
	}

	EntityQuery.ForEachEntityChunk(Context,
		[&Module](FMassExecutionContext& ChunkContext)
	{
		TArrayView<FBobFragment> Fragments = ChunkContext.GetMutableFragmentView<FBobFragment>();
		// Pass fragment array directly to Rust — zero-copy via matching #[repr(C)] layout.
		// The pointer to the first FBobFragment's PositionX field IS the start of the
		// Rust BobFragment data, because FMassFragment is an empty base (EBO applies).
		Module.Plugin.Rust.mass_bob_process(
			&Fragments[0].PositionX,
			Fragments.Num(),
			ChunkContext.GetDeltaTimeSeconds()
		);
	});
}
