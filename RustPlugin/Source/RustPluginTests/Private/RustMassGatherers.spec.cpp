#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "Tests/AutomationCommon.h"
#include "GatherersSubsystem.h"
#include "GatherersBevyMassSubsystem.h"
#include "GatherersFragments.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersSubsystemRegisteredTest,
	"supplemental.RustPlugin.Gatherers.SubsystemRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersSubsystemRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* SubsystemClass = FindObject<UClass>(nullptr, TEXT("/Script/RustMassGatherers.GatherersRustSubsystem"));
	TestNotNull(TEXT("UGatherersRustSubsystem UClass should be registered"), SubsystemClass);
	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersProcessorsRegisteredTest,
	"supplemental.RustPlugin.Gatherers.ProcessorsRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersProcessorsRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* MovementProcessor = FindObject<UClass>(nullptr, TEXT("/Script/RustMassGatherers.GatherersAntMovementProcessor"));
	TestNotNull(TEXT("UGatherersAntMovementProcessor should be registered"), MovementProcessor);

	UClass* FoodProcessor = FindObject<UClass>(nullptr, TEXT("/Script/RustMassGatherers.GatherersFoodInteractionProcessor"));
	TestNotNull(TEXT("UGatherersFoodInteractionProcessor should be registered"), FoodProcessor);
	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersSpawnAndSimulateTest,
	"supplemental.RustPlugin.Gatherers.SpawnAndSimulate",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersSpawnAndSimulateTest::RunTest(const FString& Parameters)
{
	UWorld* World = GEditor->GetEditorWorldContext().World();
	if (!TestNotNull(TEXT("World must exist"), World))
	{
		return false;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!TestNotNull(TEXT("MassEntitySubsystem must exist"), MassEntitySubsystem))
	{
		return false;
	}

	UGatherersRustSubsystem* Subsystem = World->GetSubsystem<UGatherersRustSubsystem>();
	if (!TestNotNull(TEXT("GatherersRustSubsystem must exist"), Subsystem))
	{
		return false;
	}

	// Initialize with known parameters
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation(20, 10, Bounds, 123);

	TestEqual(TEXT("Should have 20 ants"), Subsystem->GetManagedAntCount(), 20);
	TestEqual(TEXT("Should have 10 food"), Subsystem->GetManagedFoodCount(), 10);
	TestTrue(TEXT("HasManagedSimulation should be true"), Subsystem->HasManagedSimulation());

	// Record initial positions
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle AntEntity : Subsystem->ManagedAntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersMassAntFragment& Ant = AntView.GetFragmentData<FGatherersMassAntFragment>();
			InitialPositions.Add(Ant.Position);
		}
	}

	// Run a few simulation steps
	for (int32 Step = 0; Step < 10; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify positions changed
	int32 MovedCount = 0;
	for (int32 AntIndex = 0; AntIndex < Subsystem->ManagedAntEntities.Num(); ++AntIndex)
	{
		const FMassEntityHandle AntEntity = Subsystem->ManagedAntEntities[AntIndex];
		if (EntityManager.IsEntityValid(AntEntity) && InitialPositions.IsValidIndex(AntIndex))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersMassAntFragment& Ant = AntView.GetFragmentData<FGatherersMassAntFragment>();
			if (!Ant.Position.Equals(InitialPositions[AntIndex], 0.01))
			{
				++MovedCount;
			}
		}
	}

	TestTrue(TEXT("At least some ants should have moved after simulation steps"), MovedCount > 0);

	// Clean up
	Subsystem->ResetSimulation();
	TestFalse(TEXT("HasManagedSimulation should be false after reset"), Subsystem->HasManagedSimulation());

	return true;
}

// --- BevyMass tests ---

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassSubsystemRegisteredTest,
	"supplemental.RustPlugin.Gatherers.BevyMassSubsystemRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassSubsystemRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* SubsystemClass = FindObject<UClass>(nullptr, TEXT("/Script/RustMassGatherers.GatherersBevyMassSubsystem"));
	TestNotNull(TEXT("UGatherersBevyMassSubsystem UClass should be registered"), SubsystemClass);
	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassSpawnAndSimulateTest,
	"supplemental.RustPlugin.Gatherers.BevyMassSpawnAndSimulate",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassSpawnAndSimulateTest::RunTest(const FString& Parameters)
{
	UWorld* World = GEditor->GetEditorWorldContext().World();
	if (!TestNotNull(TEXT("World must exist"), World))
	{
		return false;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!TestNotNull(TEXT("MassEntitySubsystem must exist"), MassEntitySubsystem))
	{
		return false;
	}

	UGatherersBevyMassSubsystem* Subsystem = World->GetSubsystem<UGatherersBevyMassSubsystem>();
	if (!TestNotNull(TEXT("GatherersBevyMassSubsystem must exist"), Subsystem))
	{
		return false;
	}

	// Initialize with known parameters (ants + food)
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation(20, 10, Bounds, 456);

	TestEqual(TEXT("Should have 20 ants"), Subsystem->GetManagedAntCount(), 20);
	TestEqual(TEXT("Should have 10 food"), Subsystem->GetManagedFoodCount(), 10);
	TestTrue(TEXT("HasManagedSimulation should be true"), Subsystem->HasManagedSimulation());

	// Record initial positions
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle AntEntity : Subsystem->ManagedAntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersMassAntFragment& Ant = AntView.GetFragmentData<FGatherersMassAntFragment>();
			InitialPositions.Add(Ant.Position);
		}
	}

	// Run simulation steps (dynamic Rust processors via #[mass_system])
	for (int32 Step = 0; Step < 10; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify positions changed
	int32 MovedCount = 0;
	for (int32 AntIndex = 0; AntIndex < Subsystem->ManagedAntEntities.Num(); ++AntIndex)
	{
		const FMassEntityHandle AntEntity = Subsystem->ManagedAntEntities[AntIndex];
		if (EntityManager.IsEntityValid(AntEntity) && InitialPositions.IsValidIndex(AntIndex))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersMassAntFragment& Ant = AntView.GetFragmentData<FGatherersMassAntFragment>();
			if (!Ant.Position.Equals(InitialPositions[AntIndex], 0.01))
			{
				++MovedCount;
			}
		}
	}

	TestTrue(TEXT("At least some ants should have moved (dynamic Rust processors)"), MovedCount > 0);

	// Verify food entities exist and have valid data
	for (const FMassEntityHandle FoodEntity : Subsystem->ManagedFoodEntities)
	{
		if (EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersMassFoodFragment& Food = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
			TestTrue(TEXT("Food should start loose"), Food.bIsLoose);
		}
	}

	// Verify ants have encounter fragments
	for (const FMassEntityHandle AntEntity : Subsystem->ManagedAntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersAntEncounterFragment& Encounter =
				AntView.GetFragmentData<FGatherersAntEncounterFragment>();
			// Just verifying the fragment exists and is accessible
			(void)Encounter;
		}
	}

	// Clean up
	Subsystem->ResetSimulation();
	TestFalse(TEXT("HasManagedSimulation should be false after reset"), Subsystem->HasManagedSimulation());

	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassCollisionProcessorRegisteredTest,
	"supplemental.RustPlugin.Gatherers.BevyMassCollisionProcessorRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassCollisionProcessorRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* ProcessorClass = FindObject<UClass>(nullptr,
		TEXT("/Script/RustMassGatherers.GatherersBevyMassCollisionProcessor"));
	TestNotNull(TEXT("UGatherersBevyMassCollisionProcessor UClass should be registered"), ProcessorClass);
	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassFoodFragmentLayoutTest,
	"supplemental.RustPlugin.Gatherers.BevyMassFoodFragmentLayout",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassFoodFragmentLayoutTest::RunTest(const FString& Parameters)
{
	// Verify layout matches Rust FoodFragment expectations
	TestEqual(TEXT("FoodFragment Position offset"), (int32)offsetof(FGatherersMassFoodFragment, Position), 0);
	TestEqual(TEXT("FoodFragment bIsLoose offset"), (int32)offsetof(FGatherersMassFoodFragment, bIsLoose), 24);

	// Verify encounter fragment layout
	TestEqual(TEXT("EncounterFragment NearestFoodIndex offset"),
		(int32)offsetof(FGatherersAntEncounterFragment, NearestFoodIndex), 0);
	TestEqual(TEXT("EncounterFragment EncounterPosition offset"),
		(int32)offsetof(FGatherersAntEncounterFragment, EncounterPosition), 8);
	TestEqual(TEXT("EncounterFragment bHasEncounter offset"),
		(int32)offsetof(FGatherersAntEncounterFragment, bHasEncounter), 32);

	return true;
}
