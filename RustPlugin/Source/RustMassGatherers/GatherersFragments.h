#pragma once

#include "CoreMinimal.h"
#include "MassEntityTypes.h"
#include "MassEntityHandle.h"
#include "GatherersFragments.generated.h"

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
struct FGatherersBevyMassAntTag : public FMassTag
{
	GENERATED_BODY()
};

USTRUCT()
struct FGatherersMassAntFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	FVector PreviousPosition = FVector::ZeroVector;
	FVector Direction = FVector(1.0f, 0.0f, 0.0f);
	FMassEntityHandle CarriedFoodEntity;
	float PickupCooldownRemainingSeconds = 0.0f;
	float MovementSpeed = 100.0f;
	float TurnJitterRadians = PI / 2.0f;
	int32 RandomSeed = 0;
};

// Verify layout matches Rust AntFragment #[repr(C)]
static_assert(offsetof(FGatherersMassAntFragment, Position) == 0, "Position at offset 0 (EBO)");
static_assert(offsetof(FGatherersMassAntFragment, PreviousPosition) == 24, "PreviousPosition at offset 24");
static_assert(offsetof(FGatherersMassAntFragment, Direction) == 48, "Direction at offset 48");
static_assert(offsetof(FGatherersMassAntFragment, CarriedFoodEntity) == 72, "CarriedFoodEntity at offset 72");
static_assert(offsetof(FGatherersMassAntFragment, PickupCooldownRemainingSeconds) == 80, "PickupCooldownRemainingSeconds at offset 80");
static_assert(offsetof(FGatherersMassAntFragment, MovementSpeed) == 84, "MovementSpeed at offset 84");
static_assert(offsetof(FGatherersMassAntFragment, TurnJitterRadians) == 88, "TurnJitterRadians at offset 88");
static_assert(offsetof(FGatherersMassAntFragment, RandomSeed) == 92, "RandomSeed at offset 92");

USTRUCT()
struct FGatherersMassFoodFragment : public FMassFragment
{
	GENERATED_BODY()

	FVector Position = FVector::ZeroVector;
	bool bIsLoose = true;
};

struct FGatherersMassFoodEncounter
{
	FMassEntityHandle Entity;
	FVector EncounterPosition = FVector::ZeroVector;
};

// Verify FGatherersMassFoodEncounter layout matches Rust FoodEncounter #[repr(C)]
static_assert(offsetof(FGatherersMassFoodEncounter, Entity) == 0, "Entity at offset 0");
static_assert(offsetof(FGatherersMassFoodEncounter, EncounterPosition) == 8, "EncounterPosition at offset 8");
static_assert(sizeof(FGatherersMassFoodEncounter) == 32, "FoodEncounter size must be 32");

/// Fragment written by C++ collision pre-pass, read by Rust food decision system.
USTRUCT()
struct FGatherersAntEncounterFragment : public FMassFragment
{
	GENERATED_BODY()

	/// Entity handle of nearest food (index + serial), or [0,0] if none.
	FMassEntityHandle NearestFoodEntity;
	/// Position where the encounter occurred.
	FVector EncounterPosition = FVector::ZeroVector;
	/// Whether an encounter was detected this frame.
	bool bHasEncounter = false;
};

// Verify layout matches Rust AntEncounterFragment #[repr(C)]
static_assert(offsetof(FGatherersAntEncounterFragment, NearestFoodEntity) == 0, "NearestFoodEntity at offset 0");
static_assert(offsetof(FGatherersAntEncounterFragment, EncounterPosition) == 8, "EncounterPosition at offset 8");
static_assert(offsetof(FGatherersAntEncounterFragment, bHasEncounter) == 32, "bHasEncounter at offset 32");
