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
struct FVivariumBirdTag : public FMassTag
{
	GENERATED_BODY()
};

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
struct FVivariumFlockingFragment : public FMassFragment
{
	GENERATED_BODY()

	float SeparationWeight = 0.0f;
	float AlignmentWeight = 0.0f;
	float CohesionWeight = 0.0f;
};

static_assert(offsetof(FVivariumFlockingFragment, SeparationWeight) == 0, "SeparationWeight at offset 0");
static_assert(offsetof(FVivariumFlockingFragment, AlignmentWeight) == 4, "AlignmentWeight at offset 4");
static_assert(offsetof(FVivariumFlockingFragment, CohesionWeight) == 8, "CohesionWeight at offset 8");
static_assert(sizeof(FVivariumFlockingFragment) == 12, "FVivariumFlockingFragment size must be 12");

USTRUCT()
struct FVivariumPreviousTranslationFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Value = FVector::ZeroVector;
};

static_assert(offsetof(FVivariumPreviousTranslationFragment, Value) == 0, "Value at offset 0");
static_assert(sizeof(FVivariumPreviousTranslationFragment) == 24, "FVivariumPreviousTranslationFragment size must be 24");

USTRUCT()
struct FVivariumWanderFragment : public FMassFragment
{
	GENERATED_BODY()

	float Strength = 0.0f;
	uint32 RandomSeed = 0;
};

static_assert(offsetof(FVivariumWanderFragment, Strength) == 0, "Strength at offset 0");
static_assert(offsetof(FVivariumWanderFragment, RandomSeed) == 4, "RandomSeed at offset 4");
static_assert(sizeof(FVivariumWanderFragment) == 8, "FVivariumWanderFragment size must be 8");

