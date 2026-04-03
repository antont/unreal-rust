#include "GatherersMassRuntime.h"

#include "Math/RandomStream.h"
#include "GatherersAntSimulation.h"
#include "GatherersFragments.h"

FVector ConsumeAntTurnDirection(FGatherersMassAntFragment& AntFragment)
{
	FRandomStream RandomStream(AntFragment.RandomSeed);
	const FVector TurnDirection = ComputeAntTurnDirection(
		AntFragment.Direction,
		RandomStream.FRandRange(-1.0f, 1.0f),
		AntFragment.TurnJitterRadians);
	AntFragment.RandomSeed = RandomStream.GetCurrentSeed();
	return TurnDirection;
}
