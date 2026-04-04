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

	// --- Cached chunk descriptors (zero-copy, built on first Execute) ---
	bool bChunkCacheValid = false;

	// Primary: one MassChunkData per primary chunk, with pre-built fragment slices
	TArray<TArray<MassFragmentSlice>> CachedPrimarySlices;    // [chunk_idx][frag_idx]
	TArray<MassChunkData> CachedPrimaryChunks;                // [chunk_idx]

	// Global: chunked descriptors across all global chunks
	TArray<TArray<MassGlobalChunkSlice>> CachedChunkSlices;   // [frag_idx][chunk_idx]
	TArray<MassGlobalFragmentChunks> CachedChunkedFrags;      // [frag_idx]
	int32 CachedGlobalEntityCount = 0;

	// Filtered primary fragment metadata (non-tag only), built once
	TArray<const UScriptStruct*> PrimaryFragmentStructs;
	TArray<uint8> PrimaryFragmentAccess;

	/// Extra tag requirements added by the subsystem for population scoping.
	TArray<const UScriptStruct*> ExtraTagRequirements;

	/// System name (from Rust registration) — used for logging and pipeline ordering.
	FString SystemName;

	/// When true, Execute() builds/maintains the cache but does NOT call CachedExecuteFn.
	/// The coordinator processor reads the cache and dispatches via mass_frame_dispatch instead.
	bool bCacheOnly = false;

	/// Registration index (order in which the system was discovered).
	uint32 SystemIndex = 0;

public:
	const FString& GetSystemName() const { return SystemName; }

	/// Enable cache-only mode: cache chunks but don't call Rust directly.
	void SetCacheOnly(bool bInCacheOnly) { bCacheOnly = bInCacheOnly; }

	/// Set the system index (registration order).
	void SetSystemIndex(uint32 InIndex) { SystemIndex = InIndex; }
	uint32 GetSystemIndex() const { return SystemIndex; }

	/// Access cached primary chunks (valid after first Execute).
	const TArray<MassChunkData>& GetCachedPrimaryChunks() const { return CachedPrimaryChunks; }
	bool IsChunkCacheValid() const { return bChunkCacheValid; }

private:

	/// Whether InitFromDescriptor was called successfully.
	bool bInitialized = false;

	/// Whether ConfigureQueries has already run (avoid re-adding requirements).
	bool bQueriesConfigured = false;
};
