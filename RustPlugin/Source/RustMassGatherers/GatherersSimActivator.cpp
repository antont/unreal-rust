#include "GatherersSimActivator.h"
#include "GatherersSubsystem.h"
#include "GatherersBevyMassSubsystem.h"

void AGatherersSimActivator::BeginPlay()
{
	Super::BeginPlay();

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	switch (SimType)
	{
	case EGatherersSimType::Manual:
		{
			UGatherersRustSubsystem* Subsystem = World->GetSubsystem<UGatherersRustSubsystem>();
			if (Subsystem != nullptr)
			{
				Subsystem->InitializeSimulation(AntCount, FoodCount, SimulationBounds, RandomSeed);
				UE_LOG(LogTemp, Log, TEXT("GatherersSimActivator: Initialized Manual sim (%d ants, %d food)"),
					AntCount, FoodCount);
			}
		}
		break;

	case EGatherersSimType::BevyMass:
		{
			UGatherersBevyMassSubsystem* Subsystem = World->GetSubsystem<UGatherersBevyMassSubsystem>();
			if (Subsystem != nullptr)
			{
				Subsystem->InitializeSimulation(AntCount, SimulationBounds, RandomSeed);
				UE_LOG(LogTemp, Log, TEXT("GatherersSimActivator: Initialized BevyMass sim (%d ants)"),
					AntCount);
			}
		}
		break;
	}
}
