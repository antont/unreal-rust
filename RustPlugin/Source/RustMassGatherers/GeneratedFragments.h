#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "GeneratedFragments.generated.h"

// Auto-generated from #[derive(MassFragment)] Rust structs.
// Do not edit manually.

USTRUCT()
struct FGatherersBevyMassAntTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FGatherersMassAntTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FGatherersMassFoodTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FGatherersAntEncounterFragment : public FMassFragment
{
	GENERATED_BODY()

	int32 NearestFoodIndex = -1;
	int32 _NearestPad = 0;
	FVector EncounterPosition = FVector::ZeroVector;
	bool bHasEncounter = false;
	uint8 _Pad[7] = {};
};

static_assert(offsetof(FGatherersAntEncounterFragment, NearestFoodIndex) == 0, "NearestFoodIndex at offset 0");
static_assert(offsetof(FGatherersAntEncounterFragment, _NearestPad) == 4, "_NearestPad at offset 4");
static_assert(offsetof(FGatherersAntEncounterFragment, EncounterPosition) == 8, "EncounterPosition at offset 8");
static_assert(offsetof(FGatherersAntEncounterFragment, bHasEncounter) == 32, "bHasEncounter at offset 32");
static_assert(sizeof(FGatherersAntEncounterFragment) == 40, "FGatherersAntEncounterFragment size must be 40");

USTRUCT()
struct FGatherersMassAntFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	FVector PreviousPosition = FVector::ZeroVector;
	FVector Direction = FVector(1.0f, 0.0f, 0.0f);
	int32 CarriedFoodIndex = -1;
	int32 _CarriedPad = 0;
	float PickupCooldownRemainingSeconds = 0.0f;
	float MovementSpeed = 100.0f;
	float TurnJitterRadians = PI / 2.0f;
	int32 RandomSeed = 0;
};

static_assert(offsetof(FGatherersMassAntFragment, Position) == 0, "Position at offset 0");
static_assert(offsetof(FGatherersMassAntFragment, PreviousPosition) == 24, "PreviousPosition at offset 24");
static_assert(offsetof(FGatherersMassAntFragment, Direction) == 48, "Direction at offset 48");
static_assert(offsetof(FGatherersMassAntFragment, CarriedFoodIndex) == 72, "CarriedFoodIndex at offset 72");
static_assert(offsetof(FGatherersMassAntFragment, _CarriedPad) == 76, "_CarriedPad at offset 76");
static_assert(offsetof(FGatherersMassAntFragment, PickupCooldownRemainingSeconds) == 80, "PickupCooldownRemainingSeconds at offset 80");
static_assert(offsetof(FGatherersMassAntFragment, MovementSpeed) == 84, "MovementSpeed at offset 84");
static_assert(offsetof(FGatherersMassAntFragment, TurnJitterRadians) == 88, "TurnJitterRadians at offset 88");
static_assert(offsetof(FGatherersMassAntFragment, RandomSeed) == 92, "RandomSeed at offset 92");
static_assert(sizeof(FGatherersMassAntFragment) == 96, "FGatherersMassAntFragment size must be 96");

USTRUCT()
struct FGatherersMassFoodFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	bool bIsLoose = true;
	uint8 _Pad[7] = {};
};

static_assert(offsetof(FGatherersMassFoodFragment, Position) == 0, "Position at offset 0");
static_assert(offsetof(FGatherersMassFoodFragment, bIsLoose) == 24, "bIsLoose at offset 24");
static_assert(sizeof(FGatherersMassFoodFragment) == 32, "FGatherersMassFoodFragment size must be 32");

