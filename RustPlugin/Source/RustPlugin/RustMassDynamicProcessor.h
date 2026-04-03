// Generic MassEntity processor that dispatches to dynamically registered Rust systems.
// One instance is created per Rust system discovered via get_mass_system_count/descriptor.

#pragma once

#include "CoreMinimal.h"
#include "MassEntityQuery.h"
#include "MassProcessor.h"
#include "Bindings.h"
#include "RustMassDynamicProcessor.generated.h"

UCLASS()
class RUSTPLUGIN_API URustMassDynamicProcessor : public UMassProcessor
{
	GENERATED_BODY()

public:
	URustMassDynamicProcessor();

	/// Initialize this processor from a Rust system descriptor.
	/// Must be called before ConfigureQueries.
	void InitFromDescriptor(const MassSystemDescriptor& Descriptor);

	/// Discover all Rust-registered mass systems and create processor instances.
	/// Returns the processor classes (actually instances cast to classes for pipeline use).
	static TArray<URustMassDynamicProcessor*> CreateAllRustProcessors(
		const RustBindings& Bindings,
		UObject* Outer);

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	FMassEntityQuery EntityQuery;

	/// Cached Rust execute function for this system.
	MassSystemExecuteFn CachedExecuteFn = nullptr;

	/// Resolved UScriptStruct pointers for each fragment requirement (same order as descriptor).
	TArray<const UScriptStruct*> FragmentStructs;

	/// Access modes for each fragment requirement.
	TArray<uint8> FragmentAccessModes;

	/// Whether each requirement is a tag or fragment.
	TArray<uint8> FragmentIsTags;

	/// System name for logging.
	FString SystemName;

	/// Whether InitFromDescriptor was called successfully.
	bool bInitialized = false;
};
