#pragma once

#include "GatherersFragments.gen.h"
#include "MassEntityHandle.h"

// Manual additions below — structs not covered by #[derive(MassFragment)] codegen.

struct FGatherersMassFoodEncounter
{
	int32 FoodIndex = -1;
	int32 _Pad = 0;
	FVector EncounterPosition = FVector::ZeroVector;
};

// Verify FGatherersMassFoodEncounter layout matches Rust FoodEncounter #[repr(C)]
static_assert(offsetof(FGatherersMassFoodEncounter, FoodIndex) == 0, "FoodIndex at offset 0");
static_assert(offsetof(FGatherersMassFoodEncounter, EncounterPosition) == 8, "EncounterPosition at offset 8");
static_assert(sizeof(FGatherersMassFoodEncounter) == 32, "FoodEncounter size must be 32");
