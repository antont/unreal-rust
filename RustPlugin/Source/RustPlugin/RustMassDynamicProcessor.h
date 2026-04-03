// Generic MassEntity processor that dispatches to dynamically registered Rust systems.
// One instance is created per Rust system discovered via get_mass_system_count/descriptor.
// Supports both primary (per-chunk) and global (cross-archetype) queries.

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

	/// Add an extra tag requirement (e.g. to scope this processor to a specific entity population).
	/// Must be called before pipeline Initialize.
	void AddExtraTagRequirement(const UScriptStruct* TagStruct);

	/// Discover all Rust-registered mass systems and create processor instances.
	static TArray<URustMassDynamicProcessor*> CreateAllRustProcessors(
		const RustBindings& Bindings,
		UObject* Outer);

protected:
	virtual void ConfigureQueries(const TSharedRef<FMassEntityManager>& EntityManager) override;
	virtual void Execute(FMassEntityManager& EntityManager, FMassExecutionContext& Context) override;

private:
	/// Primary query — per-chunk iteration (query_scope == 0).
	FMassEntityQuery EntityQuery;

	/// Global query — cross-archetype, all matching entities (query_scope == 1).
	FMassEntityQuery GlobalEntityQuery;

	/// Cached Rust execute function for this system.
	MassSystemExecuteFn CachedExecuteFn = nullptr;

	// --- Primary requirements (query_scope == 0) ---
	TArray<const UScriptStruct*> FragmentStructs;
	TArray<uint8> FragmentAccessModes;
	TArray<uint8> FragmentIsTags;

	// --- Global requirements (query_scope == 1) ---
	TArray<const UScriptStruct*> GlobalFragmentStructs;
	TArray<uint8> GlobalFragmentAccessModes;

	/// Whether this system has any global queries.
	bool bHasGlobalQueries = false;

	/// Extra tag requirements added by the subsystem for population scoping.
	TArray<const UScriptStruct*> ExtraTagRequirements;

	/// System name (from Rust registration) — used for logging and pipeline ordering.
	FString SystemName;

public:
	const FString& GetSystemName() const { return SystemName; }
private:

	/// Whether InitFromDescriptor was called successfully.
	bool bInitialized = false;
};
