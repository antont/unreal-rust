#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "Example/MassSimpleMovementTrait.h"
#include "MassCommonFragments.h"
#include "MassMovementFragments.h"
#include "VivariumFragments.gen.generated.h"

// Auto-generated from #[derive(MassFragment)] Rust structs.
// Do not edit manually.

// FMassSimpleMovementTag — existing UE type, layout verified:

USTRUCT()
struct FVivariumBoundaryWrapTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FVivariumInsectTag : public FMassTag
{
	GENERATED_BODY()
};

// FMassVelocityFragment — existing UE type, layout verified:
static_assert(sizeof(FMassVelocityFragment) == 48, "FMassVelocityFragment size must be 48 for Rust interop");

// FTransformFragment — existing UE type, layout verified:
static_assert(sizeof(FTransformFragment) == 96, "FTransformFragment size must be 96 for Rust interop");

USTRUCT()
struct FVivariumBrownianMotionFragment : public FMassFragment
{
	GENERATED_BODY()

	float WanderStrength = 0.0f;
	uint32 RandomSeed = 0;
};

static_assert(offsetof(FVivariumBrownianMotionFragment, WanderStrength) == 0, "WanderStrength at offset 0");
static_assert(offsetof(FVivariumBrownianMotionFragment, RandomSeed) == 4, "RandomSeed at offset 4");
static_assert(sizeof(FVivariumBrownianMotionFragment) == 8, "FVivariumBrownianMotionFragment size must be 8");

USTRUCT()
struct FVivariumPreviousTranslationFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Value = FVector::ZeroVector;
};

static_assert(offsetof(FVivariumPreviousTranslationFragment, Value) == 0, "Value at offset 0");
static_assert(sizeof(FVivariumPreviousTranslationFragment) == 24, "FVivariumPreviousTranslationFragment size must be 24");

