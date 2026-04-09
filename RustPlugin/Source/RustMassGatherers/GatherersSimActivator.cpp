#include "GatherersSimActivator.h"
#include "RustMassBevySubsystem.h"
#include "GatherersFragments.h"
#include "GatherersMassRuntime.h"
#include "MassEntitySubsystem.h"
#include "MassEntityManager.h"
#include "MassEntityView.h"

// ---------------------------------------------------------------------------
// Spatial query callback for Rust collision pre-pass
//
// Uses ISMC spatial queries (GetInstancesOverlappingSphere/Box) to find
// food instances near each ant. Physics sweeps don't work here because
// ISMCs don't register per-instance collision bodies on trace channels.
// ---------------------------------------------------------------------------

namespace GatherersSpatialQuery
{
	static const TArray<FMassEntityHandle>* FoodEntities = nullptr;
	static FMassEntityManager* CachedEntityManager = nullptr;
	static UInstancedStaticMeshComponent* CachedFoodISMC = nullptr;
}

static uint32 GatherersSpatialQueryCallback(
	const double* PreviousPos,
	const double* CurrentPos,
	float PickupRadius,
	MassSpatialQueryResult* Out)
{
	using namespace GatherersSpatialQuery;

	if (!Out || !FoodEntities || !CachedEntityManager || !CachedFoodISMC)
	{
		return 0;
	}

	Out->has_encounter = false;
	Out->food_index = -1;
	Out->encounter_position[0] = 0.0;
	Out->encounter_position[1] = 0.0;
	Out->encounter_position[2] = 0.0;

	const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
	const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
	FMassEntityManager& EntityManager = *CachedEntityManager;
	const float PickupRadiusSq = PickupRadius * PickupRadius;

	TSet<int32> CandidateIndices;

	if (SweepStart.Equals(SweepEnd))
	{
		for (const int32 Idx : CachedFoodISMC->GetInstancesOverlappingSphere(
			SweepEnd, PickupRadius, true))
		{
			if (FoodEntities->IsValidIndex(Idx))
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
		for (const int32 Idx : CachedFoodISMC->GetInstancesOverlappingBox(SweptBounds, true))
		{
			if (FoodEntities->IsValidIndex(Idx))
			{
				CandidateIndices.Add(Idx);
			}
		}
	}

	float BestDistSq = TNumericLimits<float>::Max();
	for (const int32 Idx : CandidateIndices)
	{
		const FMassEntityHandle FoodEntity = (*FoodEntities)[Idx];
		if (!EntityManager.IsEntityValid(FoodEntity)) continue;
		FMassEntityView FoodView(EntityManager, FoodEntity);
		const FGatherersMassFoodFragment& Food =
			FoodView.GetFragmentData<FGatherersMassFoodFragment>();
		if (!Food.bIsLoose) continue;

		const FVector Closest = FMath::ClosestPointOnSegment(
			Food.Position, SweepStart, SweepEnd);
		const float DistSq = FVector::DistSquared(Closest, Food.Position);
		if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
		{
			BestDistSq = DistSq;
			Out->has_encounter = true;
			Out->food_index = Idx;
			Out->encounter_position[0] = Closest.X;
			Out->encounter_position[1] = Closest.Y;
			Out->encounter_position[2] = Closest.Z;
		}
	}

	return 1;
}

// ---------------------------------------------------------------------------
// Activator
// ---------------------------------------------------------------------------

void AGatherersSimActivator::BeginPlay()
{
	Super::BeginPlay();

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (Subsystem == nullptr)
	{
		return;
	}

	// Initialize simulation with named groups
	Subsystem->InitializeSimulation(
		{{TEXT("ants"), AntCount}, {TEXT("food"), FoodCount}},
		SimulationBounds, RandomSeed);

	// Register spatial query callback after InitializeSimulation
	// (InitializeSimulation calls ResetSimulation which clears the callback).
	// Pipelines are built lazily on first tick, so the callback will be available.
	Subsystem->SetSpatialQueryCallback(
		[Subsystem](const double* PreviousPos, const double* CurrentPos,
			float PickupRadius, MassSpatialQueryResult* Out) -> uint32
		{
			// Update cached state each call (game-thread-only, safe)
			UWorld* W = Subsystem->GetWorld();
			UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
			GatherersSpatialQuery::FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));
			GatherersSpatialQuery::CachedEntityManager = MES ? &MES->GetMutableEntityManager() : nullptr;
			GatherersSpatialQuery::CachedFoodISMC = Subsystem->GetGroupISMC(TEXT("food"));
			return GatherersSpatialQueryCallback(PreviousPos, CurrentPos, PickupRadius, Out);
		},
		GatherersMassPickupRadius);

	UE_LOG(LogTemp, Log, TEXT("GatherersSimActivator: Initialized BevyMass sim (%d ants, %d food)"),
		AntCount, FoodCount);
}
