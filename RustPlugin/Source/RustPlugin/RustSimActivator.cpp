#include "RustSimActivator.h"
#include "RustMassBevySubsystem.h"
#include "RustPlugin.h"

void ARustSimActivator::BeginPlay()
{
	Super::BeginPlay();

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (Subsystem == nullptr)
	{
		UE_LOG(LogTemp, Error, TEXT("RustSimActivator: No URustMassBevySubsystem found."));
		return;
	}

	// Read Rust-registered sim defaults
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	TArray<TPair<FString, int32>> FinalGroupCounts;
	FBox FinalBounds = SimulationBounds;
	int32 FinalSeed = RandomSeed;

	bool bHaveRustDefaults = false;
	if (Module.Plugin.Rust.get_sim_defaults.IsSome())
	{
		MassSimDefaultsDesc Defaults = {};
		if (Module.Plugin.Rust.get_sim_defaults.Unwrap()(&Defaults) != 0)
		{
			bHaveRustDefaults = true;

			// Use Rust defaults as baseline
			for (uint32 i = 0; i < Defaults.num_groups; ++i)
			{
				FString Name = FString(Defaults.groups[i].name.len,
					UTF8_TO_TCHAR(Defaults.groups[i].name.ptr));
				FinalGroupCounts.Add(TPair<FString, int32>(Name, Defaults.groups[i].count));
			}

			FinalBounds = FBox(
				FVector(Defaults.bounds_min[0], Defaults.bounds_min[1], Defaults.bounds_min[2]),
				FVector(Defaults.bounds_max[0], Defaults.bounds_max[1], Defaults.bounds_max[2]));
			FinalSeed = Defaults.random_seed;
		}
	}

	if (!bHaveRustDefaults && !bOverrideDefaults)
	{
		UE_LOG(LogTemp, Warning, TEXT("RustSimActivator: No Rust defaults and bOverrideDefaults is false. "
			"Using actor UPROPERTY values as fallback."));
	}

	// Apply UPROPERTY overrides
	if (bOverrideDefaults)
	{
		FinalBounds = SimulationBounds;
		FinalSeed = RandomSeed;

		if (GroupCounts.Num() > 0)
		{
			FinalGroupCounts.Empty();
			for (const auto& Pair : GroupCounts)
			{
				FinalGroupCounts.Add(TPair<FString, int32>(Pair.Key, Pair.Value));
			}
		}
	}

	if (FinalGroupCounts.Num() == 0)
	{
		UE_LOG(LogTemp, Warning, TEXT("RustSimActivator: No entity groups configured. Simulation will be empty."));
	}

	// Initialize simulation — spatial queries are auto-setup by the subsystem
	Subsystem->InitializeSimulation(FinalGroupCounts, FinalBounds, FinalSeed);

	int32 TotalEntities = 0;
	for (const auto& Pair : FinalGroupCounts)
	{
		TotalEntities += Pair.Value;
	}
	UE_LOG(LogTemp, Log, TEXT("RustSimActivator: Started simulation (%d groups, %d total entities, seed=%d)"),
		FinalGroupCounts.Num(), TotalEntities, FinalSeed);
}
