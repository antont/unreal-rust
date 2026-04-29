#include "RustMassBevySubsystem.h"

#include "MassCommonFragments.h"
#include "Engine/World.h"
#include "GameFramework/Actor.h"
#include "MassExecutor.h"
#include "MassEntityManager.h"
#include "MassProcessingContext.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "MassNavigationSubsystem.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.h"
#include "RustPlugin.h"
#include "RustUtils.h"
#include "RustMassPostMovementProcessor.h"
#include "Movement/MassMovementProcessors.h"
#include "RustMassVisualizationSetup.h"
#include "MassSimulationSubsystem.h"
#include "MassRepresentationProcessor.h"
#include "MassRepresentationSubsystem.h"
#include "MassRepresentationTypes.h"
#include "MassRepresentationFragments.h"
#include "MassVisualizationLODProcessor.h"
#include "MassUpdateISMProcessor.h"
#include "CollisionShape.h"
#include "StructUtils/StructView.h"
#include "Engine/StaticMesh.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "Materials/MaterialInterface.h"

// ---------------------------------------------------------------------------
// Spatial query trampolines: bounce C function pointers to game-module delegates
// ---------------------------------------------------------------------------

namespace RustMassSpatialQuery
{
	static constexpr int32 MaxQueries = 8;
	static URustMassBevySubsystem::FSpatialQueryCallback* ActiveCallbacks[MaxQueries] = {};

	// Diagnostic counters for the GridHash callback. Aggregated over a single
	// simulation step and logged from RunSimulationProcessorStep when
	// UNREAL_RUST_MASS_TIMING=1. Single-threaded (game thread) so plain ints suffice.
	static uint64 GridHashCalls = 0;
	static uint64 GridHashCandidates = 0;       // items returned by QuerySmall
	static uint64 GridHashCandidatesValid = 0;  // after EntityManager.IsEntityValid
	static uint64 GridHashCandidatesMapped = 0; // after EntityToIndex lookup
	static uint64 GridHashCandidatesPassed = 0; // after filter fragment read
	static uint64 GridHashEncountersWithin = 0; // passed the radius distance check (per candidate)
	static uint64 GridHashEncountersReturned = 0; // calls that returned has_encounter=true

	template<int N>
	uint32_t Trampoline(
		const double* PreviousPos,
		const double* CurrentPos,
		float PickupRadius,
		MassSpatialQueryResult* Out)
	{
		if (ActiveCallbacks[N] && *ActiveCallbacks[N])
		{
			return (*ActiveCallbacks[N])(PreviousPos, CurrentPos, PickupRadius, Out);
		}
		if (Out)
		{
			Out->has_encounter = false;
			Out->entity_index = -1;
		}
		return 0;
	}

	static MassSpatialQueryFn TrampolineFns[MaxQueries] = {
		&Trampoline<0>, &Trampoline<1>, &Trampoline<2>, &Trampoline<3>,
		&Trampoline<4>, &Trampoline<5>, &Trampoline<6>, &Trampoline<7>,
	};
} // namespace RustMassSpatialQuery

namespace RustMassSpatialEnumerate
{
	static constexpr int32 MaxEnumerates = 8;
	static URustMassBevySubsystem::FSpatialEnumerateCallback* ActiveCallbacks[MaxEnumerates] = {};

	template<int32 N>
	static uint32 Trampoline(const double* Center, float Radius, MassSpatialNeighbor* Out, uint32 Max)
	{
		if (ActiveCallbacks[N] && *ActiveCallbacks[N])
		{
			return (*ActiveCallbacks[N])(Center, Radius, Out, Max);
		}
		return 0;
	}

	static MassSpatialEnumerateFn TrampolineFns[MaxEnumerates] = {
		&Trampoline<0>, &Trampoline<1>, &Trampoline<2>, &Trampoline<3>,
		&Trampoline<4>, &Trampoline<5>, &Trampoline<6>, &Trampoline<7>,
	};
} // namespace RustMassSpatialEnumerate

URustMassBevySubsystem::FGridHashCounters URustMassBevySubsystem::GetGridHashCounters()
{
	FGridHashCounters Out;
	Out.Calls = RustMassSpatialQuery::GridHashCalls;
	Out.Candidates = RustMassSpatialQuery::GridHashCandidates;
	Out.CandidatesValid = RustMassSpatialQuery::GridHashCandidatesValid;
	Out.CandidatesMapped = RustMassSpatialQuery::GridHashCandidatesMapped;
	Out.CandidatesPassed = RustMassSpatialQuery::GridHashCandidatesPassed;
	Out.EncountersWithin = RustMassSpatialQuery::GridHashEncountersWithin;
	Out.EncountersReturned = RustMassSpatialQuery::GridHashEncountersReturned;
	return Out;
}

void URustMassBevySubsystem::ResetGridHashCounters()
{
	RustMassSpatialQuery::GridHashCalls = 0;
	RustMassSpatialQuery::GridHashCandidates = 0;
	RustMassSpatialQuery::GridHashCandidatesValid = 0;
	RustMassSpatialQuery::GridHashCandidatesMapped = 0;
	RustMassSpatialQuery::GridHashCandidatesPassed = 0;
	RustMassSpatialQuery::GridHashEncountersWithin = 0;
	RustMassSpatialQuery::GridHashEncountersReturned = 0;
}

// ---------------------------------------------------------------------------
// GridHash spatial-query body (extracted for test reachability).
//
// Grid membership IS the `is_loose` filter — picked-up food is removed from
// the grid on pickup and re-added on drop (see ApplyFoodEvents).
// ---------------------------------------------------------------------------

uint32 ExecuteGridHashSpatialQuery(
	FMassEntityManager& EntityManager,
	const FNavigationObstacleHashGrid2D& Grid,
	const TMap<FMassEntityHandle, int32>& EntityToIndex,
	const UInstancedStaticMeshComponent& ISMC,
	const double* PreviousPos,
	const double* CurrentPos,
	float PickupRadius,
	MassSpatialQueryResult* Out)
{
	if (!Out) return 0;

	Out->has_encounter = false;
	Out->entity_index = -1;
	Out->encounter_position[0] = 0.0;
	Out->encounter_position[1] = 0.0;
	Out->encounter_position[2] = 0.0;

	const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
	const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
	const float PickupRadiusSq = PickupRadius * PickupRadius;

	// Build query bounds covering the swept segment expanded by the radius
	// (2D-grid uses XY; Z extent is ignored).
	FBox QueryBounds(ForceInit);
	QueryBounds += SweepStart;
	QueryBounds += SweepEnd;
	QueryBounds = QueryBounds.ExpandBy(PickupRadius);

	TArray<FMassNavigationObstacleItem, TInlineAllocator<32>> Candidates;
	Grid.QuerySmall(QueryBounds, Candidates);

	++RustMassSpatialQuery::GridHashCalls;
	RustMassSpatialQuery::GridHashCandidates += (uint64)Candidates.Num();

	float BestDistSq = TNumericLimits<float>::Max();
	for (const FMassNavigationObstacleItem& Item : Candidates)
	{
		const FMassEntityHandle Entity = Item.Entity;
		if (!EntityManager.IsEntityValid(Entity)) continue;
		++RustMassSpatialQuery::GridHashCandidatesValid;

		// Reverse-map entity handle → instance index for this group (O(1)).
		const int32* FoundIdx = EntityToIndex.Find(Entity);
		if (!FoundIdx) continue;
		++RustMassSpatialQuery::GridHashCandidatesMapped;
		const int32 Idx = *FoundIdx;
		++RustMassSpatialQuery::GridHashCandidatesPassed;

		FTransform InstanceTransform;
		ISMC.GetInstanceTransform(Idx, InstanceTransform, true);
		const FVector EntityPos = InstanceTransform.GetLocation();

		const FVector Closest = FMath::ClosestPointOnSegment(EntityPos, SweepStart, SweepEnd);
		const float DistSq = FVector::DistSquared(Closest, EntityPos);
		if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
		{
			BestDistSq = DistSq;
			Out->has_encounter = true;
			Out->entity_index = Idx;
			Out->encounter_position[0] = Closest.X;
			Out->encounter_position[1] = Closest.Y;
			Out->encounter_position[2] = Closest.Z;
			++RustMassSpatialQuery::GridHashEncountersWithin;
		}
	}

	if (Out->has_encounter)
	{
		++RustMassSpatialQuery::GridHashEncountersReturned;
	}

	return 1;
}

// ---------------------------------------------------------------------------
// Enumerate-in-radius for SpatialGroupPlugin. Mirrors ExecuteGridHashSpatialQuery
// but returns every in-radius entity (not just the closest) and has no
// closest-point-on-segment arithmetic — inputs are (center, radius) only.
//
// Writes up to `Max` (entity_index, position) pairs into `Out`. Return value
// is the uncapped count — callers test `returned > Max` to detect truncation
// and retry with a larger buffer.
// ---------------------------------------------------------------------------

uint32 ExecuteGridHashEnumerate(
	FMassEntityManager& EntityManager,
	const FNavigationObstacleHashGrid2D& Grid,
	const TMap<FMassEntityHandle, int32>& EntityToIndex,
	const UInstancedStaticMeshComponent& ISMC,
	const double* Center,
	float Radius,
	MassSpatialNeighbor* Out,
	uint32 Max)
{
	const FVector C(Center[0], Center[1], Center[2]);
	const float RadiusSq = Radius * Radius;

	// 2D grid bucketing — XY bounds only; Z is filtered by the RadiusSq test.
	FBox QueryBounds(ForceInit);
	QueryBounds += C;
	QueryBounds = QueryBounds.ExpandBy(Radius);

	TArray<FMassNavigationObstacleItem, TInlineAllocator<64>> Candidates;
	Grid.QuerySmall(QueryBounds, Candidates);

	uint32 Written = 0;
	uint32 Total = 0;
	for (const FMassNavigationObstacleItem& Item : Candidates)
	{
		const FMassEntityHandle Entity = Item.Entity;
		if (!EntityManager.IsEntityValid(Entity)) continue;

		const int32* FoundIdx = EntityToIndex.Find(Entity);
		if (!FoundIdx) continue;
		const int32 Idx = *FoundIdx;

		FTransform InstanceTransform;
		ISMC.GetInstanceTransform(Idx, InstanceTransform, true);
		const FVector EntityPos = InstanceTransform.GetLocation();

		const float DistSq = static_cast<float>(FVector::DistSquared(EntityPos, C));
		if (DistSq > RadiusSq) continue;

		++Total;
		if (Out && Written < Max)
		{
			Out[Written].entity_index = Idx;
			Out[Written]._pad = 0;
			Out[Written].position[0] = EntityPos.X;
			Out[Written].position[1] = EntityPos.Y;
			Out[Written].position[2] = EntityPos.Z;
			++Written;
		}
	}
	return Total;
}

// ---------------------------------------------------------------------------
// Subsystem lifecycle
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
	bProcessorPipelinesInitialized = false;
}

void URustMassBevySubsystem::Deinitialize()
{
	SimulationProcessorPipeline.Reset();
	VisualizationPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	Super::Deinitialize();
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

const TArray<FMassEntityHandle>* URustMassBevySubsystem::GetGroupEntities(const FString& GroupName) const
{
	return EntityGroups.Find(GroupName);
}

UInstancedStaticMeshComponent* URustMassBevySubsystem::GetGroupISMC(const FString& GroupName) const
{
	for (const FCollisionGroupEntry& Entry : CollisionGroups)
	{
		if (Entry.Name == GroupName)
		{
			return Entry.ISMC;
		}
	}
	return nullptr;
}

const TMap<FMassEntityHandle, int32>* URustMassBevySubsystem::GetGroupEntityToIndex(const FString& GroupName) const
{
	for (const FCollisionGroupEntry& Entry : CollisionGroups)
	{
		if (Entry.Name == GroupName && Entry.bOwnedByGridHash)
		{
			return &Entry.EntityToIndex;
		}
	}
	return nullptr;
}

bool URustMassBevySubsystem::RepopulateGridHashForGroupForTesting(const FString& GroupName)
{
	for (FCollisionGroupEntry& Entry : CollisionGroups)
	{
		if (Entry.Name == GroupName && Entry.bOwnedByGridHash)
		{
			PopulateGridHashForGroup(Entry);
			return true;
		}
	}
	return false;
}

bool URustMassBevySubsystem::TryMarkGridHashOwner(const FString& GroupName, const TCHAR* /*LogPrefix*/)
{
	// Multiple groups may co-participate in the nav hash grid: each group's
	// EntityToIndex map keeps QuerySmall results scoped to its own instance
	// index space (see ExecuteGridHashSpatialQuery / ExecuteGridHashEnumerate).
	//
	// However, FoodPickupEvents/FoodDropEvents carry a bare instance index with
	// no group identifier, so only one group can be the authoritative food-event
	// index-space owner. We grant that role to the first group registered here
	// and leave subsequent groups as grid-only participants.
	bool bAnyFoodEventOwner = false;
	FCollisionGroupEntry* Target = nullptr;
	for (FCollisionGroupEntry& CG : CollisionGroups)
	{
		if (CG.bIsFoodEventIndexOwner) bAnyFoodEventOwner = true;
		if (CG.Name == GroupName && !Target) Target = &CG;
	}

	if (!Target) return false;
	if (Target->bOwnedByGridHash) return true; // already participating — idempotent

	Target->bOwnedByGridHash = true;
	if (!bAnyFoodEventOwner)
	{
		Target->bIsFoodEventIndexOwner = true;
	}
	PopulateGridHashForGroup(*Target);
	return true;
}

bool URustMassBevySubsystem::TryMarkGridHashOwnerForTesting(const FString& GroupName)
{
	return TryMarkGridHashOwner(GroupName, TEXT("TryMarkGridHashOwnerForTesting"));
}

void URustMassBevySubsystem::ApplyFoodDropEventForTesting(int32 FoodIdx, const FVector& NewPos)
{
	UWorld* World = GetWorld();
	UMassNavigationSubsystem* NavSubsystem = World ? World->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
	FNavigationObstacleHashGrid2D* Grid = NavSubsystem ? &NavSubsystem->GetObstacleGridMutable() : nullptr;
	UMassEntitySubsystem* MassSubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;
	if (!MassSubsystem) return;
	ApplyOneFoodDropEvent(FoodIdx, NewPos, Grid, MassSubsystem->GetMutableEntityManager());
}

int32 URustMassBevySubsystem::GetGroupEntityCount(const FString& GroupName) const
{
	const TArray<FMassEntityHandle>* Arr = EntityGroups.Find(GroupName);
	if (!Arr) return 0;

	// Count only live entities. Sims like vivarium despawn insects via
	// `commands.entity(e).despawn()` inside a #[mass_system], which flows
	// through the Bevy→Mass bridge and invalidates the handle in the Mass
	// EntityManager — but the handle stays in EntityGroups until reset.
	// Returning the raw Num() would mask predator/prey dynamics from tests.
	UMassEntitySubsystem* MassSys = UWorld::GetSubsystem<UMassEntitySubsystem>(GetWorld());
	if (!MassSys) return Arr->Num();
	const FMassEntityManager& EM = MassSys->GetEntityManager();
	int32 Live = 0;
	for (const FMassEntityHandle& E : *Arr)
	{
		if (EM.IsEntityValid(E)) ++Live;
	}
	return Live;
}

bool URustMassBevySubsystem::HasManagedSimulation() const
{
	for (const auto& Pair : EntityGroups)
	{
		if (Pair.Value.Num() > 0) return true;
	}
	return false;
}

void URustMassBevySubsystem::RegisterSpatialQuery(const FString& QueryName, FSpatialQueryCallback InCallback, float InRadius)
{
	// Preserve the trampoline index if re-registering an existing query
	int32 ExistingTrampolineIndex = -1;
	if (FSpatialQueryEntry* Existing = SpatialQueries.Find(QueryName))
	{
		ExistingTrampolineIndex = Existing->TrampolineIndex;
	}

	FSpatialQueryEntry Entry;
	Entry.Name = QueryName;
	Entry.Callback = MoveTemp(InCallback);
	Entry.Radius = InRadius;
	Entry.TrampolineIndex = ExistingTrampolineIndex;
	SpatialQueries.Add(QueryName, MoveTemp(Entry));
}

void URustMassBevySubsystem::RegisterSpatialEnumerate(const FString& Name, FSpatialEnumerateCallback InCallback, float InRadius)
{
	int32 ExistingTrampolineIndex = -1;
	if (FSpatialEnumerateEntry* Existing = SpatialEnumerates.Find(Name))
	{
		ExistingTrampolineIndex = Existing->TrampolineIndex;
	}

	FSpatialEnumerateEntry Entry;
	Entry.Name = Name;
	Entry.Callback = MoveTemp(InCallback);
	Entry.Radius = InRadius;
	Entry.TrampolineIndex = ExistingTrampolineIndex;
	SpatialEnumerates.Add(Name, MoveTemp(Entry));
}

void URustMassBevySubsystem::SetupSpatialQueriesFromRust()
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (!Module.Plugin.Rust.get_spatial_query_config_count.IsSome() ||
		!Module.Plugin.Rust.get_spatial_query_config_desc.IsSome())
	{
		return;
	}

	uint32 ConfigCount = Module.Plugin.Rust.get_spatial_query_config_count.Unwrap()();

	for (uint32 ConfigIdx = 0; ConfigIdx < ConfigCount; ++ConfigIdx)
	{
		MassSpatialQueryConfigDesc Config = {};
		if (Module.Plugin.Rust.get_spatial_query_config_desc.Unwrap()(ConfigIdx, &Config) == 0)
		{
			continue;
		}

		// Extract config values into captured variables. Utf8Str carries
		// (ptr, len); for empty strings Rust's ptr is a valid-but-dangling
		// placeholder, so skip UTF8_TO_TCHAR entirely when len == 0
		// (strlen on a dangling pointer segfaults).
		auto Utf8ToFString = [](const Utf8Str& S) -> FString {
			if (S.len == 0 || S.ptr == nullptr) return FString();
			return FString(S.len, UTF8_TO_TCHAR(S.ptr));
		};
		FString QueryName = Utf8ToFString(Config.query_name);
		FString QueryGroup = Utf8ToFString(Config.query_group);
		FString FilterFragmentType = Utf8ToFString(Config.filter_fragment_type);
		float Radius = Config.radius;
		uint32 FilterBoolOffset = Config.filter_bool_offset;
		bool FilterBoolMustBe = Config.filter_bool_must_be;
		uint8 QueryType = Config.query_type;
		uint8 CollisionChannelIndex = Config.collision_channel_index;

		// Resolve the filter fragment UScriptStruct for runtime access.
		FString SearchName = FilterFragmentType;
		if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
		{
			SearchName.RightChopInline(1);
		}
		const UScriptStruct* FilterStruct = FindFirstObject<UScriptStruct>(
			*SearchName, EFindFirstObjectOptions::NativeFirst);

		if (!FilterStruct)
		{
			UE_LOG(LogTemp, Warning, TEXT("RustMassBevySubsystem: Could not find UScriptStruct '%s'. "
				"Spatial query filtering disabled for '%s'."), *FilterFragmentType, *QueryName);
		}

		URustMassBevySubsystem* Self = this;

		if (QueryType == 2) // GridHash (UMassNavigationSubsystem)
		{
			// Flag the target group as hash-grid-owned so init/reset/event paths keep it in sync.
			// TryMarkGridHashOwner enforces the single-owner constraint — FoodPickupEvents /
			// FoodDropEvents carry a bare instance index that is implicitly scoped to one group,
			// so a second claim from Rust config is refused + logged (query registration still
			// proceeds, but its callback will return no encounters because EntityToIndex stays
			// empty for the unowned group).
			TryMarkGridHashOwner(QueryGroup, /*LogPrefix=*/TEXT("SetupSpatialQueriesFromRust"));

			// FilterBoolOffset / FilterBoolMustBe / FilterStruct are unused for GridHash:
			// grid membership IS the filter (see ApplyFoodEvents).
			(void)FilterBoolOffset; (void)FilterBoolMustBe; (void)FilterStruct;

			RegisterSpatialQuery(QueryName,
				[Self, QueryGroup](
					const double* PreviousPos, const double* CurrentPos,
					float PickupRadius, MassSpatialQueryResult* Out) -> uint32
				{
					if (!Out) return 0;

					UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);
					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					UMassNavigationSubsystem* NavSubsystem = W ? W->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
					const TMap<FMassEntityHandle, int32>* EntityToIndex = Self->GetGroupEntityToIndex(QueryGroup);
					if (!TargetISMC || !MES || !NavSubsystem || !EntityToIndex)
					{
						Out->has_encounter = false;
						Out->entity_index = -1;
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FNavigationObstacleHashGrid2D& Grid = NavSubsystem->GetObstacleGrid();

					return ExecuteGridHashSpatialQuery(
						EntityManager, Grid, *EntityToIndex, *TargetISMC,
						PreviousPos, CurrentPos, PickupRadius, Out);
				},
				Radius);
		}
		else if (QueryType == 3) // GridHashEnumerate (SpatialGroupPlugin)
		{
			// Same owner handshake as GridHash — UE needs to know this
			// group is hash-grid-managed so init/reset paths populate it.
			TryMarkGridHashOwner(QueryGroup, /*LogPrefix=*/TEXT("SetupSpatialQueriesFromRust/Enumerate"));

			(void)FilterBoolOffset; (void)FilterBoolMustBe; (void)FilterStruct;

			RegisterSpatialEnumerate(QueryName,
				[Self, QueryGroup](
					const double* Center, float Radius,
					MassSpatialNeighbor* Out, uint32 Max) -> uint32
				{
					UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);
					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					UMassNavigationSubsystem* NavSubsystem = W ? W->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
					const TMap<FMassEntityHandle, int32>* EntityToIndex = Self->GetGroupEntityToIndex(QueryGroup);
					if (!TargetISMC || !MES || !NavSubsystem || !EntityToIndex)
					{
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FNavigationObstacleHashGrid2D& Grid = NavSubsystem->GetObstacleGrid();

					return ExecuteGridHashEnumerate(
						EntityManager, Grid, *EntityToIndex, *TargetISMC,
						Center, Radius, Out, Max);
				},
				Radius);
		}
		else if (QueryType == 1) // PhysicsSweep
		{
			if (CollisionChannelIndex > 17)
			{
				UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Invalid collision channel index %d for query '%s' "
					"(max is 17 for ECC_GameTraceChannel18). Skipping."), CollisionChannelIndex, *QueryName);
				continue;
			}

			// Enable collision on the target group's ISMC for physics queries
			ECollisionChannel Channel = static_cast<ECollisionChannel>(ECC_GameTraceChannel1 + CollisionChannelIndex);
			UInstancedStaticMeshComponent* TargetISMC = GetGroupISMC(QueryGroup);
			if (TargetISMC)
			{
				TargetISMC->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
				TargetISMC->SetCollisionResponseToAllChannels(ECR_Ignore);
				TargetISMC->SetCollisionResponseToChannel(Channel, ECR_Block);
				// Flush physics bodies so SweepMultiByChannel can find ISMC instances
				// immediately, without waiting for the next world tick.
				TargetISMC->RecreatePhysicsState();
			}

			RegisterSpatialQuery(QueryName,
				[Self, QueryGroup, FilterBoolOffset, FilterBoolMustBe, FilterStruct, Channel](
					const double* PreviousPos, const double* CurrentPos,
					float PickupRadius, MassSpatialQueryResult* Out) -> uint32
				{
					if (!Out) return 0;

					Out->has_encounter = false;
					Out->entity_index = -1;
					Out->encounter_position[0] = 0.0;
					Out->encounter_position[1] = 0.0;
					Out->encounter_position[2] = 0.0;

					const TArray<FMassEntityHandle>* TargetEntities = Self->GetGroupEntities(QueryGroup);
					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					if (!TargetEntities || !W || !MES)
					{
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
					const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
					const float PickupRadiusSq = PickupRadius * PickupRadius;

					// UE physics sweep
					FCollisionShape Shape = FCollisionShape::MakeSphere(PickupRadius);
					TArray<FHitResult> Hits;
					W->SweepMultiByChannel(Hits, SweepStart, SweepEnd, FQuat::Identity, Channel, Shape);

					// Find nearest hit along sweep direction (earliest collision)
					float BestHitDistance = TNumericLimits<float>::Max();
					for (const FHitResult& Hit : Hits)
					{
						int32 Idx = Hit.Item; // ISMC instance index
						if (!TargetEntities->IsValidIndex(Idx)) continue;

						const FMassEntityHandle Entity = (*TargetEntities)[Idx];
						if (!EntityManager.IsEntityValid(Entity)) continue;

						if (FilterStruct)
						{
							FMassEntityView EntityView(EntityManager, Entity);
							FStructView FragView = EntityView.GetFragmentDataStruct(FilterStruct);
							if (FragView.IsValid())
							{
								const uint8* FragmentBytes = FragView.GetMemory();
								bool FilterValue = *reinterpret_cast<const bool*>(FragmentBytes + FilterBoolOffset);
								if (FilterValue != FilterBoolMustBe) continue;
							}
						}

						if (Hit.Distance < BestHitDistance)
						{
							BestHitDistance = Hit.Distance;
							const FVector HitPoint = Hit.ImpactPoint;
							Out->has_encounter = true;
							Out->entity_index = Idx;
							Out->encounter_position[0] = HitPoint.X;
							Out->encounter_position[1] = HitPoint.Y;
							Out->encounter_position[2] = HitPoint.Z;
						}
					}

					return 1;
				},
				Radius);
		}
		else // IsmcOverlap (default)
		{
			RegisterSpatialQuery(QueryName,
				[Self, QueryGroup, FilterBoolOffset, FilterBoolMustBe, FilterStruct](
					const double* PreviousPos, const double* CurrentPos,
					float PickupRadius, MassSpatialQueryResult* Out) -> uint32
				{
					if (!Out) return 0;

					Out->has_encounter = false;
					Out->entity_index = -1;
					Out->encounter_position[0] = 0.0;
					Out->encounter_position[1] = 0.0;
					Out->encounter_position[2] = 0.0;

					const TArray<FMassEntityHandle>* TargetEntities = Self->GetGroupEntities(QueryGroup);
					UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);

					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					if (!TargetEntities || !TargetISMC || !MES)
					{
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
					const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
					const float PickupRadiusSq = PickupRadius * PickupRadius;

					// ISMC spatial query: sphere or box depending on movement
					TSet<int32> CandidateIndices;
					if (SweepStart.Equals(SweepEnd))
					{
						for (const int32 Idx : TargetISMC->GetInstancesOverlappingSphere(
							SweepEnd, PickupRadius, true))
						{
							if (TargetEntities->IsValidIndex(Idx))
							{
								CandidateIndices.Add(Idx);
							}
						}
					}
					else
					{
						FBox SweptBounds(ForceInit);
						SweptBounds += SweepStart;
						SweptBounds += SweepEnd;
						SweptBounds = SweptBounds.ExpandBy(PickupRadius);
						for (const int32 Idx : TargetISMC->GetInstancesOverlappingBox(SweptBounds, true))
						{
							if (TargetEntities->IsValidIndex(Idx))
							{
								CandidateIndices.Add(Idx);
							}
						}
					}

					// Find nearest matching entity
					float BestDistSq = TNumericLimits<float>::Max();
					for (const int32 Idx : CandidateIndices)
					{
						const FMassEntityHandle Entity = (*TargetEntities)[Idx];
						if (!EntityManager.IsEntityValid(Entity)) continue;

						if (FilterStruct)
						{
							FMassEntityView EntityView(EntityManager, Entity);
							FStructView FragView = EntityView.GetFragmentDataStruct(FilterStruct);
							if (FragView.IsValid())
							{
								const uint8* FragmentBytes = FragView.GetMemory();
								bool FilterValue = *reinterpret_cast<const bool*>(FragmentBytes + FilterBoolOffset);
								if (FilterValue != FilterBoolMustBe) continue;
							}
						}

						FTransform InstanceTransform;
						TargetISMC->GetInstanceTransform(Idx, InstanceTransform, true);
						const FVector EntityPos = InstanceTransform.GetLocation();

						const FVector Closest = FMath::ClosestPointOnSegment(
							EntityPos, SweepStart, SweepEnd);
						const float DistSq = FVector::DistSquared(Closest, EntityPos);
						if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
						{
							BestDistSq = DistSq;
							Out->has_encounter = true;
							Out->entity_index = Idx;
							Out->encounter_position[0] = Closest.X;
							Out->encounter_position[1] = Closest.Y;
							Out->encounter_position[2] = Closest.Z;
						}
					}

					return 1;
				},
				Radius);
		}

		const TCHAR* TypeStr =
			QueryType == 2 ? TEXT("GridHash") :
			QueryType == 1 ? TEXT("PhysicsSweep") :
			TEXT("IsmcOverlap");
		UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-setup spatial query '%s' for group '%s' "
			"(type=%s, radius=%.1f, filter=%s, bool_offset=%d, must_be=%s)"),
			*QueryName, *QueryGroup, TypeStr,
			Radius, *FilterFragmentType, FilterBoolOffset,
			FilterBoolMustBe ? TEXT("true") : TEXT("false"));
	}
}

// ---------------------------------------------------------------------------
// Processor pipelines
// ---------------------------------------------------------------------------

bool URustMassBevySubsystem::EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (bProcessorPipelinesInitialized)
	{
		return true;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	TSharedRef<FMassEntityManager> EntityManagerRef = EntityManager.AsShared();

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	TArray<URustMassDynamicProcessor*> DynamicProcessors =
		URustMassDynamicProcessor::CreateAllRustProcessors(Module.Plugin.Rust, this);

	TArray<UMassProcessor*> SimProcessors;

	DynamicProcessors.Sort([](const URustMassDynamicProcessor& A, const URustMassDynamicProcessor& B)
	{
		return A.GetExecutionOrder() < B.GetExecutionOrder();
	});

	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		SimProcessors.Add(Proc);
	}

	if (Module.Plugin.Rust.mass_frame_dispatch != nullptr && DynamicProcessors.Num() > 0)
	{
		ScheduleCoordinator = NewObject<URustMassScheduleCoordinator>(this);
		ScheduleCoordinator->InitializeDispatch(Module.Plugin.Rust.mass_frame_dispatch, DynamicProcessors);

		// Build spatial query slots from registered queries
		if (SpatialQueries.Num() > 0)
		{
			TArray<MassSpatialQuerySlot> Slots;
			TArray<TArray<char>> NameBuffers;
			int32 SlotIdx = 0;
			for (auto& Pair : SpatialQueries)
			{
				if (SlotIdx >= RustMassSpatialQuery::MaxQueries)
				{
					UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Too many spatial queries (%d), max is %d. Excess queries will be silently ignored — increase MaxQueries."),
						SpatialQueries.Num(), RustMassSpatialQuery::MaxQueries);
					ensureMsgf(false, TEXT("Spatial query count %d exceeds MaxQueries %d"), SpatialQueries.Num(), RustMassSpatialQuery::MaxQueries);
					break;
				}

				// Store the trampoline index in the entry for stable callback mapping
				Pair.Value.TrampolineIndex = SlotIdx;

				// Convert name to UTF-8 and store in a persistent buffer
				FTCHARToUTF8 Converter(*Pair.Key);
				TArray<char> NameBuf;
				NameBuf.Append(Converter.Get(), Converter.Length());
				NameBuf.Add(0); // null-terminate

				MassSpatialQuerySlot Slot = {};
				Slot.name.ptr = nullptr; // fixed up by coordinator after move
				Slot.name.len = Converter.Length();
				Slot.query_fn = RustMassSpatialQuery::TrampolineFns[SlotIdx];
				Slot.radius = Pair.Value.Radius;
				Slots.Add(Slot);
				NameBuffers.Add(MoveTemp(NameBuf));
				++SlotIdx;
			}
			ScheduleCoordinator->SetSpatialQuerySlots(MoveTemp(Slots), MoveTemp(NameBuffers));
		}

		// Build spatial enumerate slots from registered enumerates (SpatialGroupPlugin).
		if (SpatialEnumerates.Num() > 0)
		{
			TArray<MassSpatialEnumerateSlot> Slots;
			TArray<TArray<char>> NameBuffers;
			int32 SlotIdx = 0;
			for (auto& Pair : SpatialEnumerates)
			{
				if (SlotIdx >= RustMassSpatialEnumerate::MaxEnumerates)
				{
					UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Too many spatial enumerates (%d), max is %d. Excess enumerates will be silently ignored — increase MaxEnumerates."),
						SpatialEnumerates.Num(), RustMassSpatialEnumerate::MaxEnumerates);
					ensureMsgf(false, TEXT("Spatial enumerate count %d exceeds MaxEnumerates %d"), SpatialEnumerates.Num(), RustMassSpatialEnumerate::MaxEnumerates);
					break;
				}

				// Store the trampoline index in the entry for stable callback mapping
				// (bound at tick time in RunSimulationProcessorStep — sweep pattern).
				Pair.Value.TrampolineIndex = SlotIdx;

				// Convert name to UTF-8 and store in a persistent buffer
				FTCHARToUTF8 Converter(*Pair.Key);
				TArray<char> NameBuf;
				NameBuf.Append(Converter.Get(), Converter.Length());
				NameBuf.Add(0); // null-terminate

				MassSpatialEnumerateSlot Slot = {};
				Slot.name.ptr = nullptr; // fixed up by coordinator after move
				Slot.name.len = Converter.Length();
				Slot.enumerate_fn = RustMassSpatialEnumerate::TrampolineFns[SlotIdx];
				Slot.radius = Pair.Value.Radius;
				Slots.Add(Slot);
				NameBuffers.Add(MoveTemp(NameBuf));
				++SlotIdx;
			}
			ScheduleCoordinator->SetSpatialEnumerateSlots(MoveTemp(Slots), MoveTemp(NameBuffers));
		}

		SimProcessors.Add(ScheduleCoordinator);
	}

	// Pre-movement: save PreviousTranslation (for spatial sweeps) + clamp to bounds.
	// Runs BEFORE native movement so PreviousTranslation captures position before this frame's move.
	PostMovementProcessor = NewObject<URustMassPostMovementProcessor>(this);
	SimProcessors.Add(PostMovementProcessor);

	// UE's simple movement processor: reads Velocity, writes Transform.
	// Requires FMassVelocityFragment + FMassSimpleMovementTag on entities.
	// Unlike UMassApplyMovementProcessor, this does NOT opt out on FMassOffLODTag —
	// LOD-culled entities still move, so visualization LOD can't stall the sim.
	NativeMovementProcessor = NewObject<UMassSimpleMovementProcessor>(this);
	SimProcessors.Add(NativeMovementProcessor);

	if (SimProcessors.Num() > 0)
	{
		SimulationProcessorPipeline.SetProcessors(SimProcessors);
		SimulationProcessorPipeline.Initialize(*this, EntityManagerRef);
	}

	bProcessorPipelinesInitialized = true;
	return true;
}

// ---------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::RunSimulationProcessorStep(float SimulatedDeltaTime)
{
	TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_SimStep);
	if (!HasManagedSimulation())
	{
		return;
	}

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr || !EnsureProcessorPipelines(*MassEntitySubsystem))
	{
		return;
	}

	// Set spatial query trampoline contexts using stored indices
	for (auto& Pair : SpatialQueries)
	{
		const int32 Idx = Pair.Value.TrampolineIndex;
		if (Idx >= 0 && Idx < RustMassSpatialQuery::MaxQueries)
		{
			RustMassSpatialQuery::ActiveCallbacks[Idx] = &Pair.Value.Callback;
		}
	}

	// Set spatial enumerate trampoline contexts using stored indices
	for (auto& Pair : SpatialEnumerates)
	{
		const int32 Idx = Pair.Value.TrampolineIndex;
		if (Idx >= 0 && Idx < RustMassSpatialEnumerate::MaxEnumerates)
		{
			RustMassSpatialEnumerate::ActiveCallbacks[Idx] = &Pair.Value.Callback;
		}
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	FMassProcessingContext SimulationContext(EntityManager, SimulatedDeltaTime);
	{
		TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_SimPipelineRun);
		UE::Mass::Executor::Run(SimulationProcessorPipeline, SimulationContext);
	}

	// Clear after dispatch
	for (int32 i = 0; i < RustMassSpatialQuery::MaxQueries; ++i)
	{
		RustMassSpatialQuery::ActiveCallbacks[i] = nullptr;
	}
	for (int32 i = 0; i < RustMassSpatialEnumerate::MaxEnumerates; ++i)
	{
		RustMassSpatialEnumerate::ActiveCallbacks[i] = nullptr;
	}

	// Per-step GridHash callback stats (opt-in via UNREAL_RUST_MASS_TIMING=1, same gate
	// as per-system timing so both appear together). When the per-step log is on,
	// reset after emit so each line covers one step. Otherwise leave counters to
	// accumulate — automation tests read them via GetGridHashCounters() and manage
	// their own baseline via ResetGridHashCounters().
	{
		static const bool bTimingEnabled = []{
			const FString V = FPlatformMisc::GetEnvironmentVariable(TEXT("UNREAL_RUST_MASS_TIMING"));
			return V == TEXT("1") || V.Equals(TEXT("true"), ESearchCase::IgnoreCase);
		}();
		if (bTimingEnabled && RustMassSpatialQuery::GridHashCalls > 0)
		{
			const double AvgCands = (double)RustMassSpatialQuery::GridHashCandidates / (double)RustMassSpatialQuery::GridHashCalls;
			UE_LOG(LogTemp, Log,
				TEXT("[gridhash-perf] calls=%llu cands=%llu(avg=%.2f) valid=%llu mapped=%llu passed=%llu within=%llu returned=%llu"),
				(unsigned long long)RustMassSpatialQuery::GridHashCalls,
				(unsigned long long)RustMassSpatialQuery::GridHashCandidates,
				AvgCands,
				(unsigned long long)RustMassSpatialQuery::GridHashCandidatesValid,
				(unsigned long long)RustMassSpatialQuery::GridHashCandidatesMapped,
				(unsigned long long)RustMassSpatialQuery::GridHashCandidatesPassed,
				(unsigned long long)RustMassSpatialQuery::GridHashEncountersWithin,
				(unsigned long long)RustMassSpatialQuery::GridHashEncountersReturned);

			RustMassSpatialQuery::GridHashCalls = 0;
			RustMassSpatialQuery::GridHashCandidates = 0;
			RustMassSpatialQuery::GridHashCandidatesValid = 0;
			RustMassSpatialQuery::GridHashCandidatesMapped = 0;
			RustMassSpatialQuery::GridHashCandidatesPassed = 0;
			RustMassSpatialQuery::GridHashEncountersWithin = 0;
			RustMassSpatialQuery::GridHashEncountersReturned = 0;
		}
	}

	// Drain shadow-entity despawns inside the substep so the next substep sees
	// a consistent view of Bevy and Mass state. Deferring until after all
	// substeps would leave just-despawned entities Mass-valid (and spatial-query
	// visible) for any remaining substeps in the same tick.
	ApplyDespawnEvents();
}

// ---------------------------------------------------------------------------
// Collision-only ISMCs (for spatial queries — rendering uses native MassRepresentation)
// ---------------------------------------------------------------------------

namespace
{
	constexpr TCHAR CollisionSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");

	const UScriptStruct* FindFragmentStructByName(const FString& CppTypeName)
	{
		FString SearchName = CppTypeName;
		if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
		{
			SearchName.RightChopInline(1);
		}
		return FindFirstObject<UScriptStruct>(*SearchName, EFindFirstObjectOptions::NativeFirst);
	}
} // anonymous namespace

void URustMassBevySubsystem::InitializeCollisionISMCs(UWorld* World, const RustBindings& Bindings, FMassEntityManager& EntityManager)
{
	if (!World) return;
	if (Bindings.get_visualizer_group_count.IsNone() || Bindings.get_visualizer_group_desc.IsNone())
	{
		return;
	}

	const uint32 GroupCount = Bindings.get_visualizer_group_count.Unwrap()();
	if (GroupCount == 0) return;

	UStaticMesh* SphereMesh = LoadObject<UStaticMesh>(nullptr, CollisionSphereMeshPath);
	if (!SphereMesh) return;

	// Spawn actor to own collision ISMCs
	if (!CollisionActor)
	{
		FActorSpawnParameters SpawnParams;
		SpawnParams.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
		CollisionActor = World->SpawnActor<AActor>(AActor::StaticClass(), FTransform::Identity, SpawnParams);
		if (!CollisionActor) return;

		USceneComponent* Root = NewObject<USceneComponent>(CollisionActor, TEXT("CollisionRoot"));
		CollisionActor->AddInstanceComponent(Root);
		Root->RegisterComponent();
		CollisionActor->SetRootComponent(Root);
	}

	USceneComponent* RootComponent = CollisionActor->GetRootComponent();

	for (uint32 i = 0; i < GroupCount; ++i)
	{
		MassVisualizerGroupDesc Desc = {};
		if (Bindings.get_visualizer_group_desc.Unwrap()(i, &Desc) == 0) continue;

		FString GroupName = FString(Desc.name.len, UTF8_TO_TCHAR(Desc.name.ptr));
		FString FragTypeName = FString(Desc.position_fragment_type.len, UTF8_TO_TCHAR(Desc.position_fragment_type.ptr));

		const UScriptStruct* FragStruct = FindFragmentStructByName(FragTypeName);
		if (!FragStruct)
		{
			UE_LOG(LogTemp, Warning, TEXT("RustMassBevySubsystem: Could not find USTRUCT '%s' for collision group '%s'"),
				*FragTypeName, *GroupName);
			continue;
		}

		FCollisionGroupEntry Entry;
		Entry.Name = GroupName;
		Entry.PositionStruct = FragStruct;
		Entry.PositionOffset = Desc.position_offset;
		Entry.Scale = FVector(Desc.scale);

		FName ComponentName(*FString::Printf(TEXT("Collision_%s"), *GroupName));
		Entry.ISMC = NewObject<UInstancedStaticMeshComponent>(CollisionActor, ComponentName);
		CollisionActor->AddInstanceComponent(Entry.ISMC);
		Entry.ISMC->SetupAttachment(RootComponent);
		Entry.ISMC->RegisterComponent();
		Entry.ISMC->SetStaticMesh(SphereMesh);
		Entry.ISMC->SetMobility(EComponentMobility::Movable);
		Entry.ISMC->SetCollisionEnabled(ECollisionEnabled::NoCollision); // enabled per-group by spatial query setup
		Entry.ISMC->SetCastShadow(false);
		Entry.ISMC->SetCanEverAffectNavigation(false);
		Entry.ISMC->SetHiddenInGame(true); // invisible — rendering via native MassRepresentation
		Entry.ISMC->SetVisibility(false);

		// Populate initial instances from entity positions
		const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
		if (Entities)
		{
			for (const FMassEntityHandle& Entity : *Entities)
			{
				if (!EntityManager.IsEntityValid(Entity)) continue;
				FMassEntityView View(EntityManager, Entity);
				FStructView FragView = View.GetFragmentDataStruct(Entry.PositionStruct);
				const uint8* FragData = FragView.GetMemory();
				if (!FragData) continue;
				const double* Pos = reinterpret_cast<const double*>(FragData + Entry.PositionOffset);
				FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Entry.Scale);
				Entry.ISMC->AddInstance(T, true);
			}
		}

		UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Collision ISMC '%s': %d instances (frag='%s', offset=%u)"),
			*GroupName, Entry.ISMC->GetInstanceCount(), *FragTypeName, Entry.PositionOffset);
		CollisionGroups.Add(MoveTemp(Entry));
	}
}

void URustMassBevySubsystem::SyncCollisionISMCs(FMassEntityManager& EntityManager)
{
	for (FCollisionGroupEntry& Group : CollisionGroups)
	{
		if (!Group.ISMC) continue;

		// Only sync groups that have collision enabled (spatial queries active)
		if (Group.ISMC->GetCollisionEnabled() == ECollisionEnabled::NoCollision) continue;

		const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
		if (!Entities) continue;

		// Rebuild if count mismatch
		if (Group.ISMC->GetInstanceCount() != Entities->Num())
		{
			Group.ISMC->ClearInstances();
			for (const FMassEntityHandle& Entity : *Entities)
			{
				if (!EntityManager.IsEntityValid(Entity)) continue;
				FMassEntityView View(EntityManager, Entity);
				FStructView FragView = View.GetFragmentDataStruct(Group.PositionStruct);
				const uint8* FragData = FragView.GetMemory();
				if (!FragData) continue;
				const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
				FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
				Group.ISMC->AddInstance(T, true);
			}
			continue;
		}

		const bool IsLast = (&Group == &CollisionGroups.Last());
		for (int32 i = 0; i < Entities->Num(); ++i)
		{
			if (!EntityManager.IsEntityValid((*Entities)[i])) continue;
			FMassEntityView View(EntityManager, (*Entities)[i]);
			FStructView FragView = View.GetFragmentDataStruct(Group.PositionStruct);
			const uint8* FragData = FragView.GetMemory();
			if (!FragData) continue;
			const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
			FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
			const bool bMarkDirty = (i == Entities->Num() - 1 && IsLast);
			Group.ISMC->UpdateInstanceTransform(i, T, true, bMarkDirty, true);
		}
	}
}

void URustMassBevySubsystem::TeardownCollisionISMCs()
{
	for (auto& Group : CollisionGroups)
	{
		if (Group.bOwnedByGridHash)
		{
			ClearGridHashForGroup(Group);
		}
		if (Group.ISMC)
		{
			Group.ISMC->ClearInstances();
		}
	}
	CollisionGroups.Empty();

	if (CollisionActor)
	{
		CollisionActor->Destroy();
		CollisionActor = nullptr;
	}
}

namespace
{
	// Build a minimal XY AABB centered on a world-space position for the 2D hash grid.
	// The grid ignores Z for binning; a zero-sized box at the point's XY puts it in one cell.
	FBox MakeGridItemBounds(const FVector& Pos)
	{
		return FBox(FVector(Pos.X, Pos.Y, 0.0), FVector(Pos.X, Pos.Y, 0.0));
	}
} // anonymous namespace

void URustMassBevySubsystem::PopulateGridHashForGroup(FCollisionGroupEntry& Group)
{
	if (!Group.bOwnedByGridHash) return;

	UWorld* World = GetWorld();
	UMassNavigationSubsystem* NavSubsystem = World ? World->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
	if (!NavSubsystem) return;

	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
	if (!Entities || !Group.ISMC) return;

	// Idempotent: two GridHash query configs can legally target the same group.
	// The first populate owns the grid; subsequent calls share it. Without this
	// guard, every extra call re-inserts every entity into the nav grid (duplicates
	// inflate QuerySmall results and desync pickup/drop mutation — removes clear
	// only one copy, leaving phantom obstacles). Reset via ClearGridHashForGroup.
	if (Group.GridCellLocations.Num() > 0)
	{
		UE_LOG(LogTemp, Verbose, TEXT("RustMassBevySubsystem: Skipping duplicate populate for group '%s' (%d items already in grid)"),
			*Group.Name, Group.GridCellLocations.Num());
		return;
	}

	FNavigationObstacleHashGrid2D& Grid = NavSubsystem->GetObstacleGridMutable();
	Group.GridCellLocations.Reset();
	Group.GridCellLocations.Reserve(Entities->Num());
	Group.InGrid.Init(false, Entities->Num());
	Group.EntityToIndex.Reset();
	Group.EntityToIndex.Reserve(Entities->Num());

	for (int32 i = 0; i < Entities->Num(); ++i)
	{
		FMassNavigationObstacleItem Item;
		Item.Entity = (*Entities)[i];
		Item.ItemFlags = EMassNavigationObstacleFlags::None;

		Group.EntityToIndex.Add(Item.Entity, i);

		FTransform InstanceTransform;
		if (!Group.ISMC->GetInstanceTransform(i, InstanceTransform, true))
		{
			Group.GridCellLocations.Add(FNavigationObstacleHashGrid2D::FCellLocation());
			continue;
		}
		const FVector Pos = InstanceTransform.GetLocation();
		const FNavigationObstacleHashGrid2D::FCellLocation Cell = Grid.Add(Item, MakeGridItemBounds(Pos));
		Group.GridCellLocations.Add(Cell);
		Group.InGrid[i] = true;
	}

	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Populated nav hash grid for group '%s' (%d items)"),
		*Group.Name, Entities->Num());
}

void URustMassBevySubsystem::ClearGridHashForGroup(FCollisionGroupEntry& Group)
{
	if (!Group.bOwnedByGridHash) return;

	UWorld* World = GetWorld();
	UMassNavigationSubsystem* NavSubsystem = World ? World->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
	if (!NavSubsystem)
	{
		Group.GridCellLocations.Reset();
		return;
	}

	FNavigationObstacleHashGrid2D& Grid = NavSubsystem->GetObstacleGridMutable();
	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
	if (Entities)
	{
		for (int32 i = 0; i < Entities->Num() && i < Group.GridCellLocations.Num(); ++i)
		{
			if (!Group.InGrid.IsValidIndex(i) || !Group.InGrid[i]) continue;
			FMassNavigationObstacleItem Item;
			Item.Entity = (*Entities)[i];
			Grid.Remove(Item, Group.GridCellLocations[i]);
		}
	}
	Group.GridCellLocations.Reset();
	Group.InGrid.Empty();
	Group.EntityToIndex.Reset();
}

void URustMassBevySubsystem::ApplyDespawnEvents()
{
	TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_ApplyDespawnEvents);

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (!Module.Plugin.Rust.get_despawned_shadows.IsSome()) return;

	UWorld* World = GetWorld();
	UMassEntitySubsystem* MassSys = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;
	if (!MassSys) return;
	FMassEntityManager& EntityManager = MassSys->GetMutableEntityManager();

	UMassNavigationSubsystem* NavSubsystem = World->GetSubsystem<UMassNavigationSubsystem>();
	FNavigationObstacleHashGrid2D* Grid = NavSubsystem ? &NavSubsystem->GetObstacleGridMutable() : nullptr;

	constexpr uint32_t BatchSize = 64;
	MassDespawnedShadow Events[BatchSize];
	for (;;)
	{
		uint32_t Count = Module.Plugin.Rust.get_despawned_shadows.Unwrap()(Events, BatchSize);
		for (uint32_t i = 0; i < Count; ++i)
		{
			// Length-aware: `group.ptr` comes from Rust's interned-string table
			// (Box::leak of a boxed str), which is NOT null-terminated. Walking
			// past `len` would read past the allocation.
			FUTF8ToTCHAR GroupConverter(Events[i].group.ptr, static_cast<int32>(Events[i].group.len));
			const FString GroupName(GroupConverter.Length(), GroupConverter.Get());
			const int32 Idx = int32(Events[i].index);

			TArray<FMassEntityHandle>* Handles = EntityGroups.Find(GroupName);
			if (!Handles || !Handles->IsValidIndex(Idx)) continue;
			const FMassEntityHandle Handle = (*Handles)[Idx];

			for (FCollisionGroupEntry& CG : CollisionGroups)
			{
				if (CG.Name != GroupName || !CG.bOwnedByGridHash) continue;
				if (Grid && CG.InGrid.IsValidIndex(Idx) && CG.InGrid[Idx]
					&& CG.GridCellLocations.IsValidIndex(Idx))
				{
					FMassNavigationObstacleItem Item;
					Item.Entity = Handle;
					Grid->Remove(Item, CG.GridCellLocations[Idx]);
					CG.InGrid[Idx] = false;
				}
				CG.EntityToIndex.Remove(Handle);
			}

			if (EntityManager.IsEntityValid(Handle))
			{
				EntityManager.DestroyEntity(Handle);
			}
		}
		if (Count < BatchSize) break;
	}
}

void URustMassBevySubsystem::ApplyFoodEvents()
{
	TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_ApplyFoodEvents);
	if (CollisionGroups.Num() == 0) return;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	const bool bHasDropFn = Module.Plugin.Rust.get_food_drop_events.IsSome();
	const bool bHasPickupFn = Module.Plugin.Rust.get_food_pickup_events.IsSome();
	if (!bHasDropFn && !bHasPickupFn) return;

	UWorld* World = GetWorld();
	UMassNavigationSubsystem* NavSubsystem = World ? World->GetSubsystem<UMassNavigationSubsystem>() : nullptr;
	FNavigationObstacleHashGrid2D* Grid = NavSubsystem ? &NavSubsystem->GetObstacleGridMutable() : nullptr;
	UMassEntitySubsystem* MassSubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;
	if (!MassSubsystem) return;
	FMassEntityManager& EntityManager = MassSubsystem->GetMutableEntityManager();

	constexpr uint32_t BatchSize = 64;

	// 1) Drain pickups first — remove from grid, so a same-frame drop of the same
	//    index re-adds cleanly. (Gatherers never does that, but keep order defensive.)
	if (bHasPickupFn)
	{
		int32 PickupIndices[BatchSize];
		for (;;)
		{
			uint32_t Count = Module.Plugin.Rust.get_food_pickup_events.Unwrap()(PickupIndices, BatchSize);
			for (uint32_t i = 0; i < Count; ++i)
			{
				const int32 FoodIdx = PickupIndices[i];
				for (auto& Group : CollisionGroups)
				{
					// Food-event indices are scoped to the single food-event index owner.
					if (!Group.bIsFoodEventIndexOwner || !Grid) continue;
					if (!Group.InGrid.IsValidIndex(FoodIdx) || !Group.InGrid[FoodIdx]) continue;
					if (!Group.GridCellLocations.IsValidIndex(FoodIdx)) continue;

					const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
					if (!Entities || !Entities->IsValidIndex(FoodIdx)) continue;

					FMassNavigationObstacleItem Item;
					Item.Entity = (*Entities)[FoodIdx];
					Grid->Remove(Item, Group.GridCellLocations[FoodIdx]);
					Group.InGrid[FoodIdx] = false;
				}
			}
			if (Count < BatchSize) break;
		}
	}

	// 2) Drain drops — re-add to grid at the new position, and push the ISMC transform.
	if (bHasDropFn)
	{
		FoodDropEvent DropEvents[BatchSize];
		for (;;)
		{
			uint32_t Count = Module.Plugin.Rust.get_food_drop_events.Unwrap()(DropEvents, BatchSize);
			for (uint32_t i = 0; i < Count; ++i)
			{
				const FoodDropEvent& Evt = DropEvents[i];
				const FVector NewPos(Evt.position[0], Evt.position[1], Evt.position[2]);
				ApplyOneFoodDropEvent(Evt.food_index, NewPos, Grid, EntityManager);
			}
			if (Count < BatchSize) break;
		}
	}
}

void URustMassBevySubsystem::ApplyOneFoodDropEvent(int32 FoodIdx, const FVector& NewPos,
	FNavigationObstacleHashGrid2D* Grid, FMassEntityManager& EntityManager)
{
	for (auto& Group : CollisionGroups)
	{
		if (!Group.ISMC) continue;

		// `FoodDropEvent::food_index` is an instance index scoped to the single
		// food-event index owner (see FFI doc comments on GetFoodDropEventsFn /
		// GetFoodPickupEventsFn). Applying the drop to any other nav-grid group
		// would teleport same-indexed instances in unrelated groups — scope
		// strictly to the food-event owner.
		if (!Group.bIsFoodEventIndexOwner) continue;

		const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
		if (!Entities || !Entities->IsValidIndex(FoodIdx)) continue;

		// Skip drops targeting a tombstoned (despawned) slot — otherwise we'd
		// push an ISMC transform and re-add a dead entity to the nav grid,
		// making a just-despawned shadow visible to spatial queries again.
		const FMassEntityHandle Handle = (*Entities)[FoodIdx];
		if (!EntityManager.IsEntityValid(Handle)) continue;

		// Keep ISMC transforms current — callback reads position from the ISMC.
		FTransform T(FQuat::Identity, NewPos, Group.Scale);
		if (Group.ISMC->IsValidInstance(FoodIdx))
		{
			Group.ISMC->UpdateInstanceTransform(FoodIdx, T, true, true, true);
		}

		if (Grid == nullptr) continue;

		if (!Group.GridCellLocations.IsValidIndex(FoodIdx)) continue;

		FMassNavigationObstacleItem Item;
		Item.Entity = Handle;
		const FBox NewBounds = MakeGridItemBounds(NewPos);

		if (Group.InGrid.IsValidIndex(FoodIdx) && Group.InGrid[FoodIdx])
		{
			// Already in grid (unusual — covers out-of-order events or non-gatherers sims).
			const FNavigationObstacleHashGrid2D::FCellLocation NewLoc =
				Grid->Move(Item, Group.GridCellLocations[FoodIdx], NewBounds);
			Group.GridCellLocations[FoodIdx] = NewLoc;
		}
		else
		{
			Group.GridCellLocations[FoodIdx] = Grid->Add(Item, NewBounds);
			if (Group.InGrid.IsValidIndex(FoodIdx))
			{
				Group.InGrid[FoodIdx] = true;
			}
		}
	}
}

// ---------------------------------------------------------------------------
// Tick
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::TryAutoInitFromRustDefaults()
{
	if (bAutoInitAttempted) return;
	bAutoInitAttempted = true;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (!Module.Plugin.Rust.get_sim_defaults.IsSome()) return;

	MassSimDefaultsDesc Defaults = {};
	if (Module.Plugin.Rust.get_sim_defaults.Unwrap()(&Defaults) == 0) return;

	TArray<TPair<FString, int32>> GroupCounts;
	for (uint32 i = 0; i < Defaults.num_groups; ++i)
	{
		FString Name = FString(Defaults.groups[i].name.len,
			UTF8_TO_TCHAR(Defaults.groups[i].name.ptr));
		GroupCounts.Add(TPair<FString, int32>(Name, Defaults.groups[i].count));
	}

	FBox Bounds(
		FVector(Defaults.bounds_min[0], Defaults.bounds_min[1], Defaults.bounds_min[2]),
		FVector(Defaults.bounds_max[0], Defaults.bounds_max[1], Defaults.bounds_max[2]));

	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-initializing from Rust defaults "
		"(%d groups, seed=%d)"), GroupCounts.Num(), Defaults.random_seed);

	InitializeSimulation(GroupCounts, Bounds, Defaults.random_seed);
}

void URustMassBevySubsystem::Tick(float DeltaTime)
{
	TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_Tick);
	Super::Tick(DeltaTime);

	if (!HasManagedSimulation())
	{
		TryAutoInitFromRustDefaults();
		if (!HasManagedSimulation()) return;
	}

	const float SimulatedSecondsThisFrame = FMath::Max(0.0f, DeltaTime);
	SimulationTimeAccumulatorSeconds += SimulatedSecondsThisFrame;



	const FVector BoundsSize = SimulationBounds.GetSize();
	const float BoundsMaxStepDistance = SimulationBounds.IsValid
		? 0.5f * FMath::Min(BoundsSize.X, BoundsSize.Y)
		: 500.0f;
	const float DefaultMovementSpeed = 100.0f;
	const float MaxStepSeconds = BoundsMaxStepDistance / FMath::Max(KINDA_SMALL_NUMBER, DefaultMovementSpeed);

	int32 StepsExecutedThisFrame = 0;
	constexpr int32 MaxStepsPerTick = 4096;
	while (SimulationTimeAccumulatorSeconds >= MaxStepSeconds
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(MaxStepSeconds);
		SimulationTimeAccumulatorSeconds -= MaxStepSeconds;
		++StepsExecutedThisFrame;
	}

	if (SimulationTimeAccumulatorSeconds > KINDA_SMALL_NUMBER
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(SimulationTimeAccumulatorSeconds);
		SimulationTimeAccumulatorSeconds = 0.0f;
		++StepsExecutedThisFrame;
	}




	// Run visualization pipeline (Representation only) after simulation updates transforms
	if (VisProcessor)
	{
		TRACE_CPUPROFILER_EVENT_SCOPE(RustMass_VisPipeline);
		UWorld* World = GetWorld();
		UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;
		if (MassEntitySubsystem)
		{
			FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
			FMassProcessingContext VisContext(EntityManager, DeltaTime);
			UE::Mass::Executor::Run(VisualizationPipeline, VisContext);
		}
	}

#if RUST_MASS_VALIDATE_ROTATIONS
	// ---------------------------------------------------------------------
	// PIE diagnostic: detect rotation corruption BEFORE the PostPhysics
	// broadcast asserts in UpdateInstanceTransform. Enabled via
	//   RustPlugin.Build.cs: PublicDefinitions.Add("RUST_MASS_VALIDATE_ROTATIONS=1");
	// Checks three surfaces that feed ISM UpdateInstanceTransform:
	//   1. FTransformFragment (source rotation read by UpdateISMProcessor)
	//   2. FMassRepresentationFragment::PrevTransform (written per-frame)
	//   3. FMassISMCSharedData::{Current,Prev}Transforms (post-multiply
	//      batched buffers consumed by EndVisualChanges)
	// Emits one summary line per tick + first 3 failure details per surface.
	// ---------------------------------------------------------------------
	{
		UWorld* ValidationWorld = GetWorld();
		UMassEntitySubsystem* VMSubsystem = ValidationWorld ? ValidationWorld->GetSubsystem<UMassEntitySubsystem>() : nullptr;
		UMassRepresentationSubsystem* VRSubsystem = ValidationWorld ? ValidationWorld->GetSubsystem<UMassRepresentationSubsystem>() : nullptr;
		if (VMSubsystem && VRSubsystem)
		{
			FMassEntityManager& EM = VMSubsystem->GetMutableEntityManager();
			int32 FragBad = 0;
			int32 PrevBad = 0;
			int32 BufCurBad = 0;
			int32 BufPrevBad = 0;
			int32 FragLogged = 0;
			int32 PrevLogged = 0;
			int32 BufCurLogged = 0;
			int32 BufPrevLogged = 0;
			int32 TotalEntities = 0;
			int32 TotalBufEntries = 0;

			auto CheckQuat = [](const FQuat& Q) -> double {
				return Q.X * Q.X + Q.Y * Q.Y + Q.Z * Q.Z + Q.W * Q.W;
			};

			// Surfaces 1 & 2: per-entity fragments.
			for (const auto& GroupPair : EntityGroups)
			{
				for (const FMassEntityHandle& E : GroupPair.Value)
				{
					if (!EM.IsEntityValid(E)) continue;
					++TotalEntities;
					FMassEntityView View(EM, E);
					const FTransform& T = View.GetFragmentData<FTransformFragment>().GetTransform();
					const double N2 = CheckQuat(T.GetRotation());
					if (FMath::Abs(N2 - 1.0) > 0.01)
					{
						++FragBad;
						if (FragLogged < 3)
						{
							const FQuat R = T.GetRotation();
							const FVector P = T.GetTranslation();
							UE_LOG(LogTemp, Error,
								TEXT("[RustMass] TransformFrag[%s idx=%d] rot=(%.6f,%.6f,%.6f,%.6f) |rot|^2=%.6f pos=(%.2f,%.2f,%.2f)"),
								*GroupPair.Key, E.Index, R.X, R.Y, R.Z, R.W, N2, P.X, P.Y, P.Z);
							++FragLogged;
						}
					}
					if (FMassRepresentationFragment* RF = View.GetFragmentDataPtr<FMassRepresentationFragment>())
					{
						const double Np = CheckQuat(RF->PrevTransform.GetRotation());
						if (FMath::Abs(Np - 1.0) > 0.01)
						{
							++PrevBad;
							if (PrevLogged < 3)
							{
								const FQuat R = RF->PrevTransform.GetRotation();
								const FVector P = RF->PrevTransform.GetTranslation();
								UE_LOG(LogTemp, Error,
									TEXT("[RustMass] RepFrag::PrevTransform[%s idx=%d] rot=(%.6f,%.6f,%.6f,%.6f) |rot|^2=%.6f pos=(%.2f,%.2f,%.2f)"),
									*GroupPair.Key, E.Index, R.X, R.Y, R.Z, R.W, Np, P.X, P.Y, P.Z);
								++PrevLogged;
							}
						}
					}
				}
			}

			// Surface 3: ISM shared-data batched buffers (exact path crashes).
			for (int32 DescIdx = 0; DescIdx < 32; ++DescIdx)
			{
				const FMassISMCSharedData* Shared = VRSubsystem->GetISMCSharedDataForDescriptionIndex(DescIdx);
				if (!Shared) continue;
				auto Check = [&](TConstArrayView<FTransform> Buf, int32& BadCount, int32& LogCount, const TCHAR* Which) {
					for (int32 i = 0; i < Buf.Num(); ++i)
					{
						const double N2 = CheckQuat(Buf[i].GetRotation());
						if (FMath::Abs(N2 - 1.0) > 0.01)
						{
							++BadCount;
							if (LogCount < 3)
							{
								const FQuat R = Buf[i].GetRotation();
								const FVector P = Buf[i].GetTranslation();
								const FVector S = Buf[i].GetScale3D();
								UE_LOG(LogTemp, Error,
									TEXT("[RustMass] ISMShared[desc=%d].%s[%d] rot=(%.6f,%.6f,%.6f,%.6f) |rot|^2=%.6f pos=(%.2f,%.2f,%.2f) scale=(%.3f,%.3f,%.3f)"),
									DescIdx, Which, i, R.X, R.Y, R.Z, R.W, N2, P.X, P.Y, P.Z, S.X, S.Y, S.Z);
								++LogCount;
							}
						}
					}
				};
				Check(Shared->GetStaticMeshInstanceTransforms(), BufCurBad, BufCurLogged, TEXT("Current"));
				Check(Shared->GetStaticMeshInstancePrevTransforms(), BufPrevBad, BufPrevLogged, TEXT("Prev"));
				TotalBufEntries += Shared->GetStaticMeshInstanceTransforms().Num();
			}

			const int32 TotalBad = FragBad + PrevBad + BufCurBad + BufPrevBad;
			if (TotalBad > 0)
			{
				UE_LOG(LogTemp, Error,
					TEXT("[RustMass] ROTATION CORRUPTION detected (tick, before PostPhysics broadcast): "
						 "TransformFrag=%d RepFrag::Prev=%d ISMShared.Current=%d ISMShared.Prev=%d "
						 "(entities=%d bufEntries=%d)"),
					FragBad, PrevBad, BufCurBad, BufPrevBad, TotalEntities, TotalBufEntries);
			}
			else
			{
				static int32 TickCounter = 0;
				if (++TickCounter <= 5 || TickCounter % 60 == 0)
				{
					UE_LOG(LogTemp, Display,
						TEXT("[RustMass] Rotation validation OK @ tick %d: entities=%d bufEntries=%d"),
						TickCounter, TotalEntities, TotalBufEntries);
				}
			}
		}
	}
#endif  // RUST_MASS_VALIDATE_ROTATIONS

	ApplyFoodEvents();
}

TStatId URustMassBevySubsystem::GetStatId() const
{
	RETURN_QUICK_DECLARE_CYCLE_STAT(URustMassBevySubsystem, STATGROUP_Tickables);
}

// ---------------------------------------------------------------------------
// Init / Reset
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::InitializeSimulation(
	const TArray<TPair<FString, int32>>& GroupCounts,
	const FBox& Bounds,
	int32 RandomSeed)
{
	ResetSimulation();

	// Save params for hot-reload re-initialization
	SavedGroupCounts = GroupCounts;
	SavedBounds = Bounds;
	SavedRandomSeed = RandomSeed;
	bHasSavedInitParams = true;

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr)
	{
		return;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	RustMassSpawnSetEntityManager(&EntityManager);
	SimulationBounds = Bounds;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (Module.Plugin.Rust.mass_init_simulation.IsSome())
	{
		// Build params with named groups
		TArray<MassEntityGroupDesc> Descs;
		TArray<TArray<uint8>> NameBuffers; // keep name bytes alive
		for (const auto& Pair : GroupCounts)
		{
			TArray<uint8> NameBytes;
			FTCHARToUTF8 Converter(*Pair.Key);
			NameBytes.Append(reinterpret_cast<const uint8*>(Converter.Get()), Converter.Length());
			NameBytes.Add(0); // null-terminate

			MassEntityGroupDesc Desc = {};
			Desc.name.ptr = reinterpret_cast<const char*>(NameBytes.GetData());
			Desc.name.len = Converter.Length();
			Desc.count = Pair.Value;
			Descs.Add(Desc);
			NameBuffers.Add(MoveTemp(NameBytes));
		}
		// Fix up pointers after NameBuffers is stable (no more Add)
		for (int32 i = 0; i < Descs.Num(); ++i)
		{
			Descs[i].name.ptr = reinterpret_cast<const char*>(NameBuffers[i].GetData());
		}

		MassInitSimulationParams Params = {};
		Params.groups = Descs.GetData();
		Params.num_groups = Descs.Num();
		Params.bounds_min[0] = Bounds.Min.X; Params.bounds_min[1] = Bounds.Min.Y; Params.bounds_min[2] = Bounds.Min.Z;
		Params.bounds_max[0] = Bounds.Max.X; Params.bounds_max[1] = Bounds.Max.Y; Params.bounds_max[2] = Bounds.Max.Z;
		Params.random_seed = RandomSeed;

		MassInitSimulationResult Result = {};
		Module.Plugin.Rust.mass_init_simulation.Unwrap()(&Params, &Result);

		for (uint32 g = 0; g < Result.num_groups; ++g)
		{
			const MassEntityGroupResult& Group = Result.groups[g];
			FString GroupName = FString(Group.name.len, UTF8_TO_TCHAR(Group.name.ptr));
			TArray<FMassEntityHandle>& Entities = EntityGroups.FindOrAdd(GroupName);
			for (uint32 i = 0; i < Group.count; ++i)
			{
				Entities.Add(FMassEntityHandle(Group.handles[i].index, Group.handles[i].serial_number));
			}
		}
	}

	// Initialize collision-only ISMCs (for spatial queries)
	InitializeCollisionISMCs(World, Module.Plugin.Rust, EntityManager);

	// Setup native MassRepresentation visualization
	{
		if (!VisualizationSetup)
		{
			VisualizationSetup = NewObject<URustMassVisualizationSetup>(this);
		}

		UStaticMesh* SphereMesh = LoadObject<UStaticMesh>(nullptr, TEXT("/Engine/BasicShapes/Sphere.Sphere"));
		UMaterialInterface* BaseMaterial = LoadObject<UMaterialInterface>(nullptr, TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial"));

		static const FLinearColor GroupColors[] = {
			FLinearColor(0.2f, 0.8f, 0.8f, 1.0f),   // cyan (ants)
			FLinearColor(0.75f, 0.01f, 0.01f, 1.0f),  // red (food)
			FLinearColor(0.1f, 0.8f, 0.2f, 1.0f),     // green
			FLinearColor(0.9f, 0.7f, 0.1f, 1.0f),     // yellow
		};
		static constexpr int32 NumGroupColors = UE_ARRAY_COUNT(GroupColors);

		if (SphereMesh)
		{
			int32 GroupIdx = 0;
			for (const auto& Pair : EntityGroups)
			{
				// Determine scale from collision group (if available) or default
				FVector GroupScale = FVector(0.2f);
				for (const FCollisionGroupEntry& CG : CollisionGroups)
				{
					if (CG.Name == Pair.Key)
					{
						GroupScale = CG.Scale;
						break;
					}
				}

				UMaterialInterface* GroupMaterial = nullptr;
				if (BaseMaterial)
				{
					UMaterialInstanceDynamic* MatInst = UMaterialInstanceDynamic::Create(BaseMaterial, this);
					if (MatInst)
					{
						MatInst->SetVectorParameterValue(TEXT("Color"), GroupColors[GroupIdx % NumGroupColors]);
						GroupMaterial = MatInst;
					}
				}

				float LODDistances[4] = { 0.0f, 2000.0f, 5000.0f, 15000.0f };
				VisualizationSetup->SetupGroupVisualization(
					World, EntityManager, Pair.Value,
					SphereMesh, GroupMaterial, GroupScale, LODDistances);

				UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Native vis configured for group '%s' (%d entities)"),
					*Pair.Key, Pair.Value.Num());
				++GroupIdx;
			}
		}

		// Build a visualization pipeline that we run from our own Tick.
		// (Don't register with MassSimulationSubsystem — we control timing.)
		// NOTE: LOD processor omitted — our custom pipeline doesn't run the
		// LODCollector that populates FMassViewerInfoFragment, so the LOD
		// processor defaults all entities to LOD=Off / CulledByDistance.
		// Without LOD evaluation, entities stay at High LOD (set at init)
		// and the vis processor updates ISM transforms every frame.
		if (!VisProcessor)
		{
			VisProcessor = NewObject<UMassVisualizationProcessor>(this);
		}
		{
			TArray<UMassProcessor*> VisProcessors;
			VisProcessors.Add(VisProcessor);
			// UMassUpdateISMProcessor: feeds current + prev transforms to ISM batching,
			// then saves PrevTransform = current for next frame (motion blur / TAA).
			VisProcessors.Add(NewObject<UMassUpdateISMProcessor>(this));
			VisualizationPipeline.SetProcessors(VisProcessors);
			TSharedRef<FMassEntityManager> EMRef = EntityManager.AsShared();
			VisualizationPipeline.Initialize(*this, EMRef);
			UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Built vis pipeline (Representation + UpdateISM, no LOD)"));
		}
	}

	int32 TotalEntities = 0;
	for (const auto& Pair : EntityGroups)
	{
		TotalEntities += Pair.Value.Num();
	}
	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Initialized %d entity groups (%d total entities)"),
		EntityGroups.Num(), TotalEntities);

	// Auto-setup spatial queries from Rust-registered config (if none already set)
	if (SpatialQueries.Num() == 0)
	{
		SetupSpatialQueriesFromRust();
	}
}

void URustMassBevySubsystem::ResetSimulation()
{
	UWorld* World = GetWorld();
	UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;

	TeardownCollisionISMCs();

	// Reset vis pipeline and processors before tearing down entities
	VisualizationPipeline.Reset();
	VisProcessor = nullptr;
	VisLODProcessor = nullptr;

	if (VisualizationSetup)
	{
		VisualizationSetup->Teardown();
	}

	if (MassEntitySubsystem != nullptr)
	{
		FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
		for (auto& Pair : EntityGroups)
		{
			for (const FMassEntityHandle Entity : Pair.Value)
			{
				if (EntityManager.IsEntityValid(Entity))
				{
					EntityManager.DestroyEntity(Entity);
				}
			}
		}
	}

	EntityGroups.Empty();
	SimulationBounds = FBox(EForceInit::ForceInit);
	SimulationTimeAccumulatorSeconds = 0.0f;
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	ScheduleCoordinator = nullptr;
	NativeMovementProcessor = nullptr;
	PostMovementProcessor = nullptr;
	SpatialQueries.Empty();
	bAutoInitAttempted = false;
}

void URustMassBevySubsystem::OnRustReloaded()
{
	UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Rust hot-reload detected, re-initializing simulation"));
	if (bHasSavedInitParams)
	{
		// Re-init with the same params the activator actor originally provided.
		// InitializeSimulation calls ResetSimulation() internally.
		InitializeSimulation(SavedGroupCounts, SavedBounds, SavedRandomSeed);
	}
	else
	{
		ResetSimulation();
	}
}

void URustMassBevySubsystem::RunSimulationProcessorsForTesting(float DeltaTime)
{
	RunSimulationProcessorStep(FMath::Max(0.0f, DeltaTime));
	ApplyFoodEvents();
}

bool URustMassBevySubsystem::ReadFragmentData(const FString& GroupName, int32 EntityIndex,
	const FString& FragmentTypeName, void* OutData, int32 DataSize) const
{
	if (!OutData || DataSize <= 0) return false;

	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
	if (!Entities || EntityIndex < 0 || EntityIndex >= Entities->Num()) return false;

	UWorld* World = GetWorld();
	if (!World) return false;
	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!MassEntitySubsystem) return false;
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();

	const FMassEntityHandle Entity = (*Entities)[EntityIndex];
	if (!EntityManager.IsEntityValid(Entity)) return false;

	// Find UScriptStruct by name
	// Strip F/U prefix for UE object lookup (same pattern as SetupSpatialQueriesFromRust)
	FString SearchName = FragmentTypeName;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	UScriptStruct* FragmentStruct = FindFirstObject<UScriptStruct>(
		*SearchName, EFindFirstObjectOptions::NativeFirst);
	if (!FragmentStruct) return false;
	if (FragmentStruct->GetStructureSize() != DataSize) return false;

	FStructView FragmentView = EntityManager.GetFragmentDataStruct(Entity, FragmentStruct);
	if (!FragmentView.IsValid()) return false;

	FMemory::Memcpy(OutData, FragmentView.GetMemory(), DataSize);
	return true;
}

bool URustMassBevySubsystem::WriteFragmentData(const FString& GroupName, int32 EntityIndex,
	const FString& FragmentTypeName, const void* InData, int32 DataSize)
{
	if (!InData || DataSize <= 0) return false;

	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
	if (!Entities || EntityIndex < 0 || EntityIndex >= Entities->Num()) return false;

	UWorld* World = GetWorld();
	if (!World) return false;
	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!MassEntitySubsystem) return false;
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();

	const FMassEntityHandle Entity = (*Entities)[EntityIndex];
	if (!EntityManager.IsEntityValid(Entity)) return false;

	// Strip F/U prefix for UE object lookup (same pattern as SetupSpatialQueriesFromRust)
	FString SearchName = FragmentTypeName;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	UScriptStruct* FragmentStruct = FindFirstObject<UScriptStruct>(
		*SearchName, EFindFirstObjectOptions::NativeFirst);
	if (!FragmentStruct) return false;
	if (FragmentStruct->GetStructureSize() != DataSize) return false;

	FStructView FragmentView = EntityManager.GetFragmentDataStruct(Entity, FragmentStruct);
	if (!FragmentView.IsValid()) return false;

	FMemory::Memcpy(FragmentView.GetMemory(), InData, DataSize);
	return true;
}
