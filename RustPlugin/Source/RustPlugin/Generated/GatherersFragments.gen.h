#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "MassCommonFragments.h"
#include "MassMovementFragments.h"
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

	float TurnJitterRadians = 1.5707964f;
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

	bool bIsLoose = true;
};

static_assert(offsetof(FGatherersMassFoodFragment, bIsLoose) == 0, "bIsLoose at offset 0");
static_assert(sizeof(FGatherersMassFoodFragment) == 1, "FGatherersMassFoodFragment size must be 1");

USTRUCT()
struct FGatherersPreviousTranslation : public FMassFragment
{
	GENERATED_BODY()

	FVector Value = FVector::ZeroVector;
};

static_assert(offsetof(FGatherersPreviousTranslation, Value) == 0, "Value at offset 0");
static_assert(sizeof(FGatherersPreviousTranslation) == 24, "FGatherersPreviousTranslation size must be 24");

// FMassVelocityFragment — existing UE type, layout verified:
static_assert(sizeof(FMassVelocityFragment) == 48, "FMassVelocityFragment size must be 48 for Rust interop");

// FTransformFragment — existing UE type, layout verified:
static_assert(sizeof(FTransformFragment) == 96, "FTransformFragment size must be 96 for Rust interop");

