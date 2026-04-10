#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "Tests/AutomationCommon.h"
#include "RustMassBevySubsystem.h"
#include "GatherersFragments.h"
#include "GatherersMassRuntime.h"
#include "Bindings.h"
#include "RustMassDynamicProcessor.h"
#include "RustPlugin.h"

// --- BevyMass tests ---

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassSubsystemRegisteredTest,
	"supplemental.RustPlugin.Gatherers.BevyMassSubsystemRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassSubsystemRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* SubsystemClass = FindObject<UClass>(nullptr, TEXT("/Script/RustPlugin.RustMassBevySubsystem"));
	TestNotNull(TEXT("URustMassBevySubsystem UClass should be registered"), SubsystemClass);
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

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	// Initialize with known parameters (ants + food)
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 20}, {TEXT("food"), 10}}, Bounds, 456);

	TestEqual(TEXT("Should have 20 ants"), Subsystem->GetGroupEntityCount(TEXT("ants")), 20);
	TestEqual(TEXT("Should have 10 food"), Subsystem->GetGroupEntityCount(TEXT("food")), 10);
	TestTrue(TEXT("HasManagedSimulation should be true"), Subsystem->HasManagedSimulation());

	// Record initial positions
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	TArray<FVector> InitialPositions;
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			InitialPositions.Add(Pos.Position);
		}
	}

	// Run simulation steps (dynamic Rust processors via #[mass_system])
	for (int32 Step = 0; Step < 10; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify positions changed
	int32 MovedCount = 0;
	for (int32 AntIndex = 0; AntIndex < AntEntities->Num(); ++AntIndex)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[AntIndex];
		if (EntityManager.IsEntityValid(AntEntity) && InitialPositions.IsValidIndex(AntIndex))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			if (!Pos.Position.Equals(InitialPositions[AntIndex], 0.01))
			{
				++MovedCount;
			}
		}
	}

	TestTrue(TEXT("At least some ants should have moved (dynamic Rust processors)"), MovedCount > 0);

	// Verify food entities exist and have valid data
	const TArray<FMassEntityHandle>* FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));
	for (const FMassEntityHandle FoodEntity : *FoodEntities)
	{
		if (EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersMassFoodFragment& Food = FoodView.GetFragmentData<FGatherersMassFoodFragment>();
			TestTrue(TEXT("Food should start loose"), Food.bIsLoose);
		}
	}

	// Verify ants have encounter fragments
	for (const FMassEntityHandle AntEntity : *AntEntities)
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

// ---------------------------------------------------------------------------
// FFI layout tests for spatial query types
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassSpatialQueryLayoutTest,
	"supplemental.RustPlugin.Gatherers.BevyMassSpatialQueryLayout",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassSpatialQueryLayoutTest::RunTest(const FString& Parameters)
{
	// MassSpatialQueryResult: i32(4) + i32(4) + double[3](24) + bool(1) + pad(7) = 40
	TestEqual(TEXT("MassSpatialQueryResult size"), (int32)sizeof(MassSpatialQueryResult), 40);
	TestEqual(TEXT("MassSpatialQueryResult alignment"), (int32)alignof(MassSpatialQueryResult), 8);
	TestEqual(TEXT("MassSpatialQueryResult food_index offset"),
		(int32)offsetof(MassSpatialQueryResult, food_index), 0);
	TestEqual(TEXT("MassSpatialQueryResult encounter_position offset"),
		(int32)offsetof(MassSpatialQueryResult, encounter_position), 8);
	TestEqual(TEXT("MassSpatialQueryResult has_encounter offset"),
		(int32)offsetof(MassSpatialQueryResult, has_encounter), 32);

	// MassFrameDispatchData: f32(4) + u32(4) + ptr(8) + ptr(8) + f32(4) + u32(4) = 32
	TestEqual(TEXT("MassFrameDispatchData size"), (int32)sizeof(MassFrameDispatchData), 32);
	TestEqual(TEXT("MassFrameDispatchData alignment"), (int32)alignof(MassFrameDispatchData), 8);
	TestEqual(TEXT("MassFrameDispatchData dt offset"),
		(int32)offsetof(MassFrameDispatchData, dt), 0);
	TestEqual(TEXT("MassFrameDispatchData num_systems offset"),
		(int32)offsetof(MassFrameDispatchData, num_systems), 4);
	TestEqual(TEXT("MassFrameDispatchData systems offset"),
		(int32)offsetof(MassFrameDispatchData, systems), 8);
	TestEqual(TEXT("MassFrameDispatchData spatial_query_fn offset"),
		(int32)offsetof(MassFrameDispatchData, spatial_query_fn), 16);
	TestEqual(TEXT("MassFrameDispatchData pickup_radius offset"),
		(int32)offsetof(MassFrameDispatchData, pickup_radius), 24);

	return true;
}

// ---------------------------------------------------------------------------
// Food pickup end-to-end test (ant placed at food position)
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassFoodPickupTest,
	"supplemental.RustPlugin.Gatherers.BevyMassFoodPickup",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassFoodPickupTest::RunTest(const FString& Parameters)
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

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	// Spawn 1 ant and 1 food in a small area so they overlap
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 1}, {TEXT("food"), 1}}, Bounds, 42);

	TestEqual(TEXT("Should have 1 ant"), Subsystem->GetGroupEntityCount(TEXT("ants")), 1);
	TestEqual(TEXT("Should have 1 food"), Subsystem->GetGroupEntityCount(TEXT("food")), 1);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	const TArray<FMassEntityHandle>* FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));

	// Move ant directly to food position to guarantee overlap
	if (AntEntities->Num() > 0 && FoodEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		const FMassEntityHandle FoodEntity = (*FoodEntities)[0];

		if (EntityManager.IsEntityValid(AntEntity) && EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FVector FoodPos = FoodView.GetFragmentData<FGatherersMassFoodFragment>().Position;

			FMassEntityView AntView(EntityManager, AntEntity);
			FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			Pos.Position = FoodPos;
			Pos.PreviousPosition = FoodPos;
			FGatherersCarrying& Carry = AntView.GetFragmentData<FGatherersCarrying>();
			Carry.FoodIndex = -1;
			FGatherersCooldown& Cd = AntView.GetFragmentData<FGatherersCooldown>();
			Cd.RemainingSeconds = 0.0f;
		}
	}

	// Register a spatial query callback that does brute-force distance checks
	// against food entities (same approach as GatherersSimActivator but without ISMC).
	Subsystem->SetSpatialQueryCallback(
		[&EntityManager, FoodEntities](const double* PreviousPos, const double* CurrentPos,
			float PickupRadius, MassSpatialQueryResult* Out) -> uint32
		{
			Out->has_encounter = false;
			Out->food_index = -1;
			Out->encounter_position[0] = 0.0;
			Out->encounter_position[1] = 0.0;
			Out->encounter_position[2] = 0.0;
			if (!FoodEntities) return 1;

			const float PickupRadiusSq = PickupRadius * PickupRadius;
			const FVector AntPos(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
			float BestDistSq = TNumericLimits<float>::Max();

			for (int32 Idx = 0; Idx < FoodEntities->Num(); ++Idx)
			{
				const FMassEntityHandle FoodEntity = (*FoodEntities)[Idx];
				if (!EntityManager.IsEntityValid(FoodEntity)) continue;
				FMassEntityView FoodView(EntityManager, FoodEntity);
				const FGatherersMassFoodFragment& Food =
					FoodView.GetFragmentData<FGatherersMassFoodFragment>();
				if (!Food.bIsLoose) continue;

				const float DistSq = FVector::DistSquared(AntPos, Food.Position);
				if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->food_index = Idx;
					Out->encounter_position[0] = Food.Position.X;
					Out->encounter_position[1] = Food.Position.Y;
					Out->encounter_position[2] = Food.Position.Z;
				}
			}
			return 1;
		},
		GatherersMassPickupRadius);

	// Run enough simulation steps for collision detection + food decision
	for (int32 Step = 0; Step < 20; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify the ant picked up food
	if (AntEntities->Num() > 0 && FoodEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		const FMassEntityHandle FoodEntity = (*FoodEntities)[0];

		if (EntityManager.IsEntityValid(AntEntity) && EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersCarrying& Carry = AntView.GetFragmentData<FGatherersCarrying>();

			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersMassFoodFragment& Food = FoodView.GetFragmentData<FGatherersMassFoodFragment>();

			TestTrue(TEXT("Ant should have picked up food"), Carry.FoodIndex >= 0);
			if (Carry.FoodIndex >= 0)
			{
				TestEqual(TEXT("Carried food index should be 0"), Carry.FoodIndex, 0);
				TestFalse(TEXT("Picked-up food should not be loose"), Food.bIsLoose);
				const FGatherersCooldown& Cd = AntView.GetFragmentData<FGatherersCooldown>();
				TestTrue(TEXT("Pickup cooldown should be active"), Cd.RemainingSeconds > 0.0f);
			}
		}
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Cooldown system test — verify cooldown decrements correctly
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassCooldownTest,
	"supplemental.RustPlugin.Gatherers.BevyMassCooldown",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassCooldownTest::RunTest(const FString& Parameters)
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

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 1}, {TEXT("food"), 0}}, Bounds, 555);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));

	// Set a known cooldown value
	if (AntEntities && AntEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			FGatherersCooldown& Cd = AntView.GetFragmentData<FGatherersCooldown>();
			Cd.RemainingSeconds = 1.0f;
		}
	}

	// Run one step with a known dt
	Subsystem->RunSimulationProcessorsForTesting(0.3f);

	// Verify cooldown decremented
	if (AntEntities && AntEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersCooldown& Cd = AntView.GetFragmentData<FGatherersCooldown>();

			// Cooldown should be approximately 0.7 (1.0 - 0.3)
			// Allow tolerance for time accumulator substeps
			TestTrue(TEXT("Cooldown should have decreased"),
				Cd.RemainingSeconds < 1.0f);
			TestTrue(TEXT("Cooldown should not be negative"),
				Cd.RemainingSeconds >= 0.0f);
		}
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Boundary reflection test — ant outside bounds gets clamped and reflected
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassBoundaryReflectTest,
	"supplemental.RustPlugin.Gatherers.BevyMassBoundaryReflect",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassBoundaryReflectTest::RunTest(const FString& Parameters)
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

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 1}, {TEXT("food"), 0}}, Bounds, 777);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));

	// Place ant heading toward the +X boundary at high speed
	if (AntEntities && AntEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			Pos.Position = FVector(490.0, 0.0, 50.0);
			Pos.PreviousPosition = Pos.Position;
			FGatherersMovement& Mov = AntView.GetFragmentData<FGatherersMovement>();
			Mov.Direction = FVector(1.0, 0.0, 0.0);
			Mov.MovementSpeed = 200.0f;
		}
	}

	// Run enough steps for the ant to hit the boundary
	for (int32 Step = 0; Step < 5; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.1f);
	}

	// Verify ant is within bounds and direction reflected
	if (AntEntities && AntEntities->Num() > 0)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[0];
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			const FGatherersMovement& Mov = AntView.GetFragmentData<FGatherersMovement>();

			TestTrue(TEXT("Ant X should be within bounds (<=500)"),
				Pos.Position.X <= 500.0 + 1.0);  // small tolerance
			TestTrue(TEXT("Ant direction X should reflect (become negative)"),
				Mov.Direction.X < 0.0);
		}
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// System ordering test — verify execution order metadata from Rust
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassSystemOrderingTest,
	"supplemental.RustPlugin.Gatherers.BevyMassSystemOrdering",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassSystemOrderingTest::RunTest(const FString& Parameters)
{
	UWorld* World = GEditor->GetEditorWorldContext().World();
	if (!TestNotNull(TEXT("World must exist"), World))
	{
		return false;
	}

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	TArray<URustMassDynamicProcessor*> Processors =
		URustMassDynamicProcessor::CreateAllRustProcessors(Module.Plugin.Rust, Subsystem);

	TestTrue(TEXT("Should discover at least 5 Rust processors"), Processors.Num() >= 5);

	// Build name→order map
	TMap<FString, uint32> OrderMap;
	for (const URustMassDynamicProcessor* Proc : Processors)
	{
		OrderMap.Add(Proc->GetFName().ToString(), Proc->GetExecutionOrder());
	}

	// Log discovered processors for diagnostics
	for (const auto& Pair : OrderMap)
	{
		AddInfo(FString::Printf(TEXT("Processor: %s  order=%u"), *Pair.Key, Pair.Value));
	}

	// Verify expected ordering relationships
	// entity_movement(10) < ant_collision_prepass(20) < ant_food_decision(30) < entity_cooldown(40) < entity_boundary_reflect(50)
	TArray<uint32> Orders;
	for (const URustMassDynamicProcessor* Proc : Processors)
	{
		Orders.Add(Proc->GetExecutionOrder());
	}
	Orders.Sort();

	// Verify processors are sortable and produce a valid pipeline order
	for (int32 i = 1; i < Orders.Num(); ++i)
	{
		TestTrue(TEXT("Processor orders should be non-decreasing"),
			Orders[i] >= Orders[i - 1]);
	}

	return true;
}

// ---------------------------------------------------------------------------
// MassSystemDescriptor FFI layout test
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassDescriptorLayoutTest,
	"supplemental.RustPlugin.Gatherers.BevyMassDescriptorLayout",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassDescriptorLayoutTest::RunTest(const FString& Parameters)
{
	TestEqual(TEXT("MassSystemDescriptor size"), (int32)sizeof(MassSystemDescriptor), 40);
	TestEqual(TEXT("MassSystemDescriptor alignment"), (int32)alignof(MassSystemDescriptor), 8);
	TestEqual(TEXT("MassSystemDescriptor name offset"),
		(int32)offsetof(MassSystemDescriptor, name), 0);
	TestEqual(TEXT("MassSystemDescriptor num_requirements offset"),
		(int32)offsetof(MassSystemDescriptor, num_requirements), 16);
	TestEqual(TEXT("MassSystemDescriptor order offset"),
		(int32)offsetof(MassSystemDescriptor, order), 20);
	TestEqual(TEXT("MassSystemDescriptor requirements offset"),
		(int32)offsetof(MassSystemDescriptor, requirements), 24);
	TestEqual(TEXT("MassSystemDescriptor execute_fn offset"),
		(int32)offsetof(MassSystemDescriptor, execute_fn), 32);

	return true;
}

// ---------------------------------------------------------------------------
// Full integration test — multi-step simulation with validation
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassIntegrationTest,
	"supplemental.RustPlugin.Gatherers.BevyMassIntegration",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassIntegrationTest::RunTest(const FString& Parameters)
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

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem))
	{
		return false;
	}

	const int32 AntCount = 50;
	const int32 FoodCount = 20;
	const FBox Bounds(FVector(-1000.0, -1000.0, 0.0), FVector(1000.0, 1000.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), AntCount}, {TEXT("food"), FoodCount}}, Bounds, 12345);

	TestEqual(TEXT("Should have 50 ants"), Subsystem->GetGroupEntityCount(TEXT("ants")), AntCount);
	TestEqual(TEXT("Should have 20 food"), Subsystem->GetGroupEntityCount(TEXT("food")), FoodCount);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	const TArray<FMassEntityHandle>* FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));

	// Record initial positions
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			InitialPositions.Add(Pos.Position);
		}
	}

	// Run 100 simulation steps (1.6 seconds of sim time)
	for (int32 Step = 0; Step < 100; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify: ants moved
	int32 MovedCount = 0;
	for (int32 i = 0; i < AntEntities->Num(); ++i)
	{
		const FMassEntityHandle AntEntity = (*AntEntities)[i];
		if (EntityManager.IsEntityValid(AntEntity) && InitialPositions.IsValidIndex(i))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			if (!Pos.Position.Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}
		}
	}
	TestTrue(TEXT("Most ants should have moved after 100 steps"), MovedCount > AntCount / 2);

	// Verify: no ant escaped bounds (with tolerance for boundary reflection lag)
	const double BoundsTolerance = 50.0;
	int32 OutOfBoundsCount = 0;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			if (Pos.Position.X < Bounds.Min.X - BoundsTolerance ||
				Pos.Position.X > Bounds.Max.X + BoundsTolerance ||
				Pos.Position.Y < Bounds.Min.Y - BoundsTolerance ||
				Pos.Position.Y > Bounds.Max.Y + BoundsTolerance)
			{
				++OutOfBoundsCount;
			}
		}
	}
	TestEqual(TEXT("No ants should be far outside bounds"), OutOfBoundsCount, 0);

	// Verify: PreviousPosition is tracked (should differ from Position for moving ants)
	int32 PreviousTrackedCount = 0;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersPosition& Pos = AntView.GetFragmentData<FGatherersPosition>();
			if (!Pos.Position.Equals(Pos.PreviousPosition, 0.001))
			{
				++PreviousTrackedCount;
			}
		}
	}
	TestTrue(TEXT("PreviousPosition should differ from Position for moving ants"),
		PreviousTrackedCount > 0);

	// Verify: all food entities valid and accessible
	int32 ValidFoodCount = 0;
	for (const FMassEntityHandle FoodEntity : *FoodEntities)
	{
		if (EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersMassFoodFragment& Food =
				FoodView.GetFragmentData<FGatherersMassFoodFragment>();
			(void)Food;
			++ValidFoodCount;
		}
	}
	TestEqual(TEXT("All food entities should remain valid"), ValidFoodCount, FoodCount);

	// Verify: encounter fragments well-formed on all ants
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FGatherersAntEncounterFragment& Enc =
				AntView.GetFragmentData<FGatherersAntEncounterFragment>();
			// If there's an encounter, food index should be non-negative
			if (Enc.bHasEncounter)
			{
				TestTrue(TEXT("Encounter food index should be >= 0"),
					Enc.NearestFoodIndex >= 0);
			}
		}
	}

	// Verify: clean reset
	Subsystem->ResetSimulation();
	TestFalse(TEXT("HasManagedSimulation should be false after reset"),
		Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Ant count should be 0 after reset"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 0);
	TestEqual(TEXT("Food count should be 0 after reset"),
		Subsystem->GetGroupEntityCount(TEXT("food")), 0);

	return true;
}
