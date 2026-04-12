#include "RustMassSpikeSubsystem.h"
#include "MassEntitySubsystem.h"
#include "MassEntityManager.h"
#include "StructUtils/InstancedStruct.h"

static constexpr int32 SPIKE_ENTITY_COUNT = 10;

void URustMassSpikeSubsystem::PostInitialize()
{
	Super::PostInitialize();

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

	for (int32 i = 0; i < SPIKE_ENTITY_COUNT; ++i)
	{
		FBobFragment BobData;
		BobData.PositionZ = static_cast<double>(i) * 100.0;
		BobData.Speed = 1.0f + static_cast<float>(i % 5) * 0.5f;

		TArray<FInstancedStruct> Fragments;
		Fragments.Add(FInstancedStruct::Make(BobData));

		FMassEntityHandle Entity = EntityManager.CreateEntity(Fragments);
		SpawnedEntities.Add(Entity);
	}

	UE_LOG(LogTemp, Log, TEXT("RustMassSpike: Spawned %d entities with FBobFragment"), SpawnedEntities.Num());
}

void URustMassSpikeSubsystem::Deinitialize()
{
	UWorld* World = GetWorld();
	if (World != nullptr)
	{
		UMassEntitySubsystem* MassSubsystem = World->GetSubsystem<UMassEntitySubsystem>();
		if (MassSubsystem != nullptr)
		{
			FMassEntityManager& EntityManager = MassSubsystem->GetMutableEntityManager();
			for (const FMassEntityHandle& Entity : SpawnedEntities)
			{
				if (EntityManager.IsEntityValid(Entity))
				{
					EntityManager.DestroyEntity(Entity);
				}
			}
		}
	}
	SpawnedEntities.Empty();
	Super::Deinitialize();
}
