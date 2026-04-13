#include "RustMassSpikeActivator.h"
#include "MassEntitySubsystem.h"
#include "MassEntityManager.h"
#include "RustBobFragment.h"
#include "StructUtils/InstancedStruct.h"

void ARustMassSpikeActivator::BeginPlay()
{
	Super::BeginPlay();

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassSubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassSubsystem == nullptr)
	{
		return;
	}

	FMassEntityManager& EntityManager = MassSubsystem->GetMutableEntityManager();

	for (int32 i = 0; i < EntityCount; ++i)
	{
		FBobFragment BobData;
		BobData.PositionZ = static_cast<double>(i) * 100.0;
		BobData.Speed = 1.0f + static_cast<float>(i % 5) * 0.5f;

		TArray<FInstancedStruct> Fragments;
		Fragments.Add(FInstancedStruct::Make(BobData));

		FMassEntityHandle Entity = EntityManager.CreateEntity(Fragments);
		SpawnedEntities.Add(Entity);
	}

	UE_LOG(LogTemp, Log, TEXT("RustMassSpikeActivator: Spawned %d entities with FBobFragment"),
		SpawnedEntities.Num());
}

void ARustMassSpikeActivator::EndPlay(const EEndPlayReason::Type EndPlayReason)
{
	UWorld* World = GetWorld();
	if (World)
	{
		UMassEntitySubsystem* MassSubsystem = World->GetSubsystem<UMassEntitySubsystem>();
		if (MassSubsystem)
		{
			FMassEntityManager& EntityManager = MassSubsystem->GetMutableEntityManager();
			for (const FMassEntityHandle Entity : SpawnedEntities)
			{
				if (EntityManager.IsEntityValid(Entity))
				{
					EntityManager.DestroyEntity(Entity);
				}
			}
		}
	}
	SpawnedEntities.Empty();

	Super::EndPlay(EndPlayReason);
}
