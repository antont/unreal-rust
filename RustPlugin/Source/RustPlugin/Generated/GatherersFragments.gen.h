#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "GatherersFragments.gen.generated.h"

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
struct FGatherersBehavior : public FMassFragment
{
	GENERATED_BODY()

	float TurnJitterRadians = PI / 2.0f;
	int32 RandomSeed = 0;
};

static_assert(offsetof(FGatherersBehavior, TurnJitterRadians) == 0, "TurnJitterRadians at offset 0");
static_assert(offsetof(FGatherersBehavior, RandomSeed) == 4, "RandomSeed at offset 4");
static_assert(sizeof(FGatherersBehavior) == 8, "FGatherersBehavior size must be 8");

USTRUCT()
struct FGatherersCarrying : public FMassFragment
{
	GENERATED_BODY()

	int32 FoodIndex = -1;
};

static_assert(offsetof(FGatherersCarrying, FoodIndex) == 0, "FoodIndex at offset 0");
static_assert(sizeof(FGatherersCarrying) == 4, "FGatherersCarrying size must be 4");

USTRUCT()
struct FGatherersMassFoodFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	bool bIsLoose = true;
	uint8 _Pad_25[7] = {};
};

static_assert(offsetof(FGatherersMassFoodFragment, Position) == 0, "Position at offset 0");
static_assert(offsetof(FGatherersMassFoodFragment, bIsLoose) == 24, "bIsLoose at offset 24");
static_assert(sizeof(FGatherersMassFoodFragment) == 32, "FGatherersMassFoodFragment size must be 32");

USTRUCT()
struct FGatherersMovement : public FMassFragment
{
	GENERATED_BODY()

	FVector Direction = FVector(1.0f, 0.0f, 0.0f);
	float MovementSpeed = 100.0f;
	uint8 _Pad_28[4] = {};
};

static_assert(offsetof(FGatherersMovement, Direction) == 0, "Direction at offset 0");
static_assert(offsetof(FGatherersMovement, MovementSpeed) == 24, "MovementSpeed at offset 24");
static_assert(sizeof(FGatherersMovement) == 32, "FGatherersMovement size must be 32");

USTRUCT()
struct FGatherersPosition : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	FVector PreviousPosition = FVector::ZeroVector;
};

static_assert(offsetof(FGatherersPosition, Position) == 0, "Position at offset 0");
static_assert(offsetof(FGatherersPosition, PreviousPosition) == 24, "PreviousPosition at offset 24");
static_assert(sizeof(FGatherersPosition) == 48, "FGatherersPosition size must be 48");

