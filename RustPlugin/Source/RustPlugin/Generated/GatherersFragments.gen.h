#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "Example/MassSimpleMovementTrait.h"
#include "MassCommonFragments.h"
#include "MassMovementFragments.h"
#include "GatherersFragments.gen.generated.h"

// Auto-generated from #[derive(MassFragment)] Rust structs.
// Do not edit manually.

USTRUCT()
struct FGatherersAntTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FGatherersFoodTag : public FMassTag
{
	GENERATED_BODY()
};

// FMassSimpleMovementTag — existing UE type, layout verified:

USTRUCT()
struct FGatherersBehaviorFragment : public FMassFragment
{
	GENERATED_BODY()

	float TurnJitterRadians = 1.5707964f;
	int32 RandomSeed = 0;
};

static_assert(offsetof(FGatherersBehaviorFragment, TurnJitterRadians) == 0, "TurnJitterRadians at offset 0");
static_assert(offsetof(FGatherersBehaviorFragment, RandomSeed) == 4, "RandomSeed at offset 4");
static_assert(sizeof(FGatherersBehaviorFragment) == 8, "FGatherersBehaviorFragment size must be 8");

USTRUCT()
struct FGatherersFoodStateFragment : public FMassFragment
{
	GENERATED_BODY()

	bool bIsLoose = true;
};

static_assert(offsetof(FGatherersFoodStateFragment, bIsLoose) == 0, "bIsLoose at offset 0");
static_assert(sizeof(FGatherersFoodStateFragment) == 1, "FGatherersFoodStateFragment size must be 1");

USTRUCT()
struct FGatherersPreviousTranslationFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Value = FVector::ZeroVector;
};

static_assert(offsetof(FGatherersPreviousTranslationFragment, Value) == 0, "Value at offset 0");
static_assert(sizeof(FGatherersPreviousTranslationFragment) == 24, "FGatherersPreviousTranslationFragment size must be 24");

// FMassVelocityFragment — existing UE type, layout verified:
static_assert(sizeof(FMassVelocityFragment) == 48, "FMassVelocityFragment size must be 48 for Rust interop");

// FTransformFragment — existing UE type, layout verified:
static_assert(sizeof(FTransformFragment) == 96, "FTransformFragment size must be 96 for Rust interop");

