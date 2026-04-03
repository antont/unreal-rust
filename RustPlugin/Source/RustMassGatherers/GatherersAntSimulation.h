#pragma once

#include "CoreMinimal.h"

FVector ComputeAntHeadingMovementStep(
	const FVector& CurrentLocation,
	const FVector& HeadingDirection,
	float MovementSpeed,
	float SafeStepDistance,
	float DeltaSeconds);

bool ShouldAntPickUpFood(
	const FVector& AntLocation,
	const FVector& FoodLocation,
	float PickupRadius);

FVector ComputeAntRetargetDirection(
	const FVector& CurrentDirection,
	float RetargetJitterRadians);

FVector ComputeAntTurnDirection(
	const FVector& CurrentDirection,
	float NormalizedJitterAlpha,
	float MaxTurnJitterRadians);

float ComputeRemainingPickupCooldown(
	float CurrentCooldownSeconds,
	float DeltaSeconds);

float ComputePickupCooldownForSeparationDistance(
	float DesiredSeparationDistance,
	float MovementSpeed);

FVector ComputeBoundaryTurnBackDirection(
	const FVector& CurrentDirection,
	const FVector& InwardBoundaryNormal);

FVector ComputeCarriedFoodRelativeLocation(float CarriedFoodHeight);
