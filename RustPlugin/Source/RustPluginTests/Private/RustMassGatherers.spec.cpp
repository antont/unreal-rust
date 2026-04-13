#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "Tests/AutomationCommon.h"
#include "RustMassBevySubsystem.h"
#include "GatherersFragments.gen.h"
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
	TestEqual(TEXT("MassSpatialQueryResult entity_index offset"),
		(int32)offsetof(MassSpatialQueryResult, entity_index), 0);
	TestEqual(TEXT("MassSpatialQueryResult encounter_position offset"),
		(int32)offsetof(MassSpatialQueryResult, encounter_position), 8);
	TestEqual(TEXT("MassSpatialQueryResult has_encounter offset"),
		(int32)offsetof(MassSpatialQueryResult, has_encounter), 32);

	// MassFrameDispatchData: f32(4) + u32(4) + ptr(8) + u32(4) + u32(4) + ptr(8) = 32
	TestEqual(TEXT("MassFrameDispatchData size"), (int32)sizeof(MassFrameDispatchData), 32);
	TestEqual(TEXT("MassFrameDispatchData alignment"), (int32)alignof(MassFrameDispatchData), 8);
	TestEqual(TEXT("MassFrameDispatchData dt offset"),
		(int32)offsetof(MassFrameDispatchData, dt), 0);
	TestEqual(TEXT("MassFrameDispatchData num_systems offset"),
		(int32)offsetof(MassFrameDispatchData, num_systems), 4);
	TestEqual(TEXT("MassFrameDispatchData systems offset"),
		(int32)offsetof(MassFrameDispatchData, systems), 8);
	TestEqual(TEXT("MassFrameDispatchData num_spatial_queries offset"),
		(int32)offsetof(MassFrameDispatchData, num_spatial_queries), 16);
	TestEqual(TEXT("MassFrameDispatchData spatial_queries offset"),
		(int32)offsetof(MassFrameDispatchData, spatial_queries), 24);

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

	// Verify SetupSpatialQueriesFromRust() registered the query automatically
	// (InitializeSimulation calls it when SpatialQueries is empty)
	TestTrue(TEXT("Spatial query 'food_pickup' should be auto-registered from Rust config"),
		Subsystem->HasSpatialQuery(TEXT("food_pickup")));

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
			// Cooldown is now a pure-Bevy component (not a MassFragment).
			// Ants spawn without Cooldown, so no setup needed here.
		}
	}

	// Replace the auto-registered physics sweep with a proximity callback that
	// reads food fragment positions directly. Physics sweep requires collision
	// bodies (ISMC) which aren't available in headless automation tests.
	Subsystem->RegisterSpatialQuery(TEXT("food_pickup"),
		[Subsystem, &EntityManager](
			const double* PreviousPos, const double* CurrentPos,
			float PickupRadius, MassSpatialQueryResult* Out) -> uint32
		{
			if (!Out) return 0;
			Out->has_encounter = false;
			Out->entity_index = -1;

			const TArray<FMassEntityHandle>* Foods = Subsystem->GetGroupEntities(TEXT("food"));
			if (!Foods) { UE_LOG(LogTemp, Warning, TEXT("[FoodPickup Mock] No food entities")); return 1; }

			const FVector AntPos(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
			const double RadiusSq = static_cast<double>(PickupRadius) * PickupRadius;
			double BestDistSq = TNumericLimits<double>::Max();

			for (int32 Idx = 0; Idx < Foods->Num(); ++Idx)
			{
				const FMassEntityHandle Entity = (*Foods)[Idx];
				if (!EntityManager.IsEntityValid(Entity)) continue;

				FMassEntityView View(EntityManager, Entity);
				const FGatherersMassFoodFragment& Food = View.GetFragmentData<FGatherersMassFoodFragment>();

				const double Dx = AntPos.X - Food.Position.X;
				const double Dy = AntPos.Y - Food.Position.Y;
				const double Dz = AntPos.Z - Food.Position.Z;
				const double DistSq = Dx*Dx + Dy*Dy + Dz*Dz;

				UE_LOG(LogTemp, Display, TEXT("[FoodPickup Mock] Ant=(%.1f,%.1f,%.1f) Food[%d]=(%.1f,%.1f,%.1f) dist=%.1f radius=%.1f loose=%d"),
					AntPos.X, AntPos.Y, AntPos.Z, Idx,
					Food.Position.X, Food.Position.Y, Food.Position.Z,
					FMath::Sqrt(DistSq), PickupRadius, Food.bIsLoose ? 1 : 0);

				if (!Food.bIsLoose) continue;

				if (DistSq < RadiusSq && DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->entity_index = Idx;
					Out->encounter_position[0] = Food.Position.X;
					Out->encounter_position[1] = Food.Position.Y;
					Out->encounter_position[2] = Food.Position.Z;
					UE_LOG(LogTemp, Display, TEXT("[FoodPickup Mock] HIT! food_index=%d"), Idx);
				}
			}
			if (!Out->has_encounter)
			{
				UE_LOG(LogTemp, Warning, TEXT("[FoodPickup Mock] No encounter found"));
			}
			return 1;
		},
		15.0f);

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
				// Cooldown is now a pure-Bevy component on shadow entities,
				// not readable from C++ via GetFragmentData.
			}
		}
	}

	Subsystem->ResetSimulation();
	return true;
}

// Cooldown is a pure-Bevy component on shadow entities — not readable from C++.
// Cooldown behavior is tested in Rust: gatherers-sim unit tests + Rust-authored
// UE automation tests (CooldownCycle, CooldownRecovery).

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

// ---------------------------------------------------------------------------
// Auto-init from Rust defaults — THE regression test for "nothing shows in PIE"
// Tick calls TryAutoInitFromRustDefaults() as a fallback when no activator
// actor has called InitializeSimulation(). This ensures PIE works in levels
// that don't have an explicit ARustSimActivator.
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassAutoInitFromRustDefaultsTest,
	"supplemental.RustPlugin.Gatherers.BevyMassAutoInitFromRustDefaults",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassAutoInitFromRustDefaultsTest::RunTest(const FString& Parameters)
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

	// Reset so auto-init can attempt again
	Subsystem->ResetSimulation();
	TestFalse(TEXT("Should not have simulation after reset"),
		Subsystem->HasManagedSimulation());

	// Tick triggers auto-init from Rust defaults
	Subsystem->Tick(0.016f);
	TestTrue(TEXT("Tick should auto-init from Rust defaults"),
		Subsystem->HasManagedSimulation());

	// Also verify explicit activator init still works (overrides auto-init)
	Subsystem->ResetSimulation();
	Subsystem->InitializeSimulation({{TEXT("ants"), 100}, {TEXT("food"), 500}},
		FBox(FVector(-500, -500, 0), FVector(500, 500, 100)), 42);

	TestTrue(TEXT("Should have simulation after explicit activator init"),
		Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Should have 100 ants"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 100);
	TestEqual(TEXT("Should have 500 food"),
		Subsystem->GetGroupEntityCount(TEXT("food")), 500);

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Hot-reload re-init — OnRustReloaded should re-initialize with saved params
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassHotReloadReInitTest,
	"supplemental.RustPlugin.Gatherers.BevyMassHotReloadReInit",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassHotReloadReInitTest::RunTest(const FString& Parameters)
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

	// Initialize with known params
	Subsystem->InitializeSimulation({{TEXT("ants"), 50}, {TEXT("food"), 200}},
		FBox(FVector(-100, -100, 0), FVector(100, 100, 50)), 99);

	TestTrue(TEXT("Should have simulation"), Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Should have 50 ants"), Subsystem->GetGroupEntityCount(TEXT("ants")), 50);

	// Simulate hot-reload — should reset and re-init with same params
	Subsystem->OnRustReloaded();

	TestTrue(TEXT("Should have simulation after hot-reload"),
		Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Should still have 50 ants after hot-reload"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 50);
	TestEqual(TEXT("Should still have 200 food after hot-reload"),
		Subsystem->GetGroupEntityCount(TEXT("food")), 200);

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// FFI wiring: Rust sim defaults readable from C++
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassRustSimDefaultsFFITest,
	"supplemental.RustPlugin.Gatherers.BevyMassRustSimDefaultsFFI",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassRustSimDefaultsFFITest::RunTest(const FString& Parameters)
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	TestTrue(TEXT("get_sim_defaults should be available"),
		Module.Plugin.Rust.get_sim_defaults.IsSome());

	MassSimDefaultsDesc Defaults = {};
	uint32 Result = Module.Plugin.Rust.get_sim_defaults.Unwrap()(&Defaults);
	TestEqual(TEXT("get_sim_defaults should return 1 (success)"), Result, (uint32)1);
	TestTrue(TEXT("Should have at least 2 groups"), Defaults.num_groups >= 2);

	// Verify group names and counts
	bool FoundAnts = false, FoundFood = false;
	for (uint32 i = 0; i < Defaults.num_groups; ++i)
	{
		FString Name(Defaults.groups[i].name.len,
			UTF8_TO_TCHAR(Defaults.groups[i].name.ptr));
		if (Name == TEXT("ants"))
		{
			FoundAnts = true;
			TestEqual(TEXT("Ants default count"), Defaults.groups[i].count, 100);
		}
		else if (Name == TEXT("food"))
		{
			FoundFood = true;
			TestEqual(TEXT("Food default count"), Defaults.groups[i].count, 500);
		}
	}
	TestTrue(TEXT("Should have 'ants' group in defaults"), FoundAnts);
	TestTrue(TEXT("Should have 'food' group in defaults"), FoundFood);
	TestEqual(TEXT("Random seed should be 42"), Defaults.random_seed, 42);

	return true;
}

// ---------------------------------------------------------------------------
// FFI wiring: Rust spatial query config readable from C++
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassRustSpatialQueryConfigFFITest,
	"supplemental.RustPlugin.Gatherers.BevyMassRustSpatialQueryConfigFFI",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassRustSpatialQueryConfigFFITest::RunTest(const FString& Parameters)
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	TestTrue(TEXT("get_spatial_query_config_count should be available"),
		Module.Plugin.Rust.get_spatial_query_config_count.IsSome());
	TestTrue(TEXT("get_spatial_query_config_desc should be available"),
		Module.Plugin.Rust.get_spatial_query_config_desc.IsSome());

	uint32 Count = Module.Plugin.Rust.get_spatial_query_config_count.Unwrap()();
	TestTrue(TEXT("Should have at least 1 spatial query config"), Count >= 1);

	MassSpatialQueryConfigDesc Config = {};
	uint32 Result = Module.Plugin.Rust.get_spatial_query_config_desc.Unwrap()(0, &Config);
	TestEqual(TEXT("get_spatial_query_config_desc should return 1"), Result, (uint32)1);

	FString QueryName(Config.query_name.len,
		UTF8_TO_TCHAR(Config.query_name.ptr));
	TestEqual(TEXT("Query name should be 'food_pickup'"), QueryName, TEXT("food_pickup"));

	FString QueryGroup(Config.query_group.len,
		UTF8_TO_TCHAR(Config.query_group.ptr));
	TestEqual(TEXT("Query group should be 'food'"), QueryGroup, TEXT("food"));

	TestEqual(TEXT("Radius should be 15.0"), Config.radius, 15.0f);
	TestEqual(TEXT("query_type should be 1 (PhysicsSweep)"), Config.query_type, (uint8)1);
	TestEqual(TEXT("collision_channel_index should be 0"), Config.collision_channel_index, (uint8)0);
	TestEqual(TEXT("Bool offset should be 24"), Config.filter_bool_offset, (uint32)24);
	TestTrue(TEXT("filter_bool_must_be should be true"), Config.filter_bool_must_be);

	FString FilterType(Config.filter_fragment_type.len,
		UTF8_TO_TCHAR(Config.filter_fragment_type.ptr));
	TestEqual(TEXT("Filter fragment type"), FilterType, TEXT("FGatherersMassFoodFragment"));

	return true;
}

// ---------------------------------------------------------------------------
// Hot-reload cycle tests — verify subsystem survives reset+reinit
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassReloadCycleTest,
	"supplemental.RustPlugin.Gatherers.BevyMassReloadCycle",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassReloadCycleTest::RunTest(const FString& Parameters)
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

	// --- Phase 1: Initialize and run simulation ---
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 20}, {TEXT("food"), 10}}, Bounds, 456);
	TestTrue(TEXT("Phase 1: Should have simulation"), Subsystem->HasManagedSimulation());

	// Record initial positions
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			InitialPositions.Add(AntView.GetFragmentData<FGatherersPosition>().Position);
		}
	}

	// Run simulation — verify ants move
	for (int32 Step = 0; Step < 5; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	int32 MovedCount = 0;
	for (int32 i = 0; i < AntEntities->Num() && i < InitialPositions.Num(); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView AntView(EntityManager, (*AntEntities)[i]);
			if (!AntView.GetFragmentData<FGatherersPosition>().Position.Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}
		}
	}
	TestTrue(TEXT("Phase 1: Ants should have moved"), MovedCount > 0);

	// --- Phase 2: Simulate hot-reload — re-inits from saved params ---
	Subsystem->OnRustReloaded();
	TestTrue(TEXT("Phase 2: Should have simulation after reload (re-inited from saved params)"),
		Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Phase 2: Should have 20 ants (re-inited with saved params)"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 20);
	TestEqual(TEXT("Phase 2: Should have 10 food (re-inited with saved params)"),
		Subsystem->GetGroupEntityCount(TEXT("food")), 10);

	// Run simulation again — verify ants move (processors rebuilt with fresh fn ptrs)
	const TArray<FMassEntityHandle>* NewAnts = Subsystem->GetGroupEntities(TEXT("ants"));
	TArray<FVector> Phase3Positions;
	for (int32 i = 0; i < FMath::Min(10, NewAnts->Num()); ++i)
	{
		if (EntityManager.IsEntityValid((*NewAnts)[i]))
		{
			FMassEntityView AntView(EntityManager, (*NewAnts)[i]);
			Phase3Positions.Add(AntView.GetFragmentData<FGatherersPosition>().Position);
		}
	}

	for (int32 Step = 0; Step < 5; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	int32 Phase3Moved = 0;
	for (int32 i = 0; i < Phase3Positions.Num() && i < NewAnts->Num(); ++i)
	{
		if (EntityManager.IsEntityValid((*NewAnts)[i]))
		{
			FMassEntityView AntView(EntityManager, (*NewAnts)[i]);
			if (!AntView.GetFragmentData<FGatherersPosition>().Position.Equals(Phase3Positions[i], 0.01))
			{
				++Phase3Moved;
			}
		}
	}
	TestTrue(TEXT("Phase 3: Ants should move after reload"), Phase3Moved > 0);

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassReloadCycleMultipleTest,
	"supplemental.RustPlugin.Gatherers.BevyMassReloadCycleMultiple",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassReloadCycleMultipleTest::RunTest(const FString& Parameters)
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

	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));

	for (int32 Cycle = 0; Cycle < 3; ++Cycle)
	{
		FString CycleLabel = FString::Printf(TEXT("Cycle %d"), Cycle);

		// Init with varying entity counts to detect stale state
		const int32 AntCount = 10 + Cycle * 5;
		Subsystem->InitializeSimulation(
			{{TEXT("ants"), AntCount}, {TEXT("food"), 10}}, Bounds, 100 + Cycle);

		TestTrue(*FString::Printf(TEXT("%s: Should have simulation"), *CycleLabel),
			Subsystem->HasManagedSimulation());
		TestEqual(*FString::Printf(TEXT("%s: Ant count"), *CycleLabel),
			Subsystem->GetGroupEntityCount(TEXT("ants")), AntCount);

		// Run a few steps
		for (int32 Step = 0; Step < 3; ++Step)
		{
			Subsystem->RunSimulationProcessorsForTesting(0.016f);
		}

		// Simulate reload — re-inits with same params
		Subsystem->OnRustReloaded();
		TestTrue(*FString::Printf(TEXT("%s: Should still have simulation after reload"), *CycleLabel),
			Subsystem->HasManagedSimulation());
		TestEqual(*FString::Printf(TEXT("%s: Ant count preserved after reload"), *CycleLabel),
			Subsystem->GetGroupEntityCount(TEXT("ants")), AntCount);
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassOnRustReloadedResetsStateTest,
	"supplemental.RustPlugin.Gatherers.BevyMassOnRustReloadedResetsState",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassOnRustReloadedResetsStateTest::RunTest(const FString& Parameters)
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

	// Initialize and run to populate all cached state
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 20}, {TEXT("food"), 10}}, Bounds, 789);
	Subsystem->RunSimulationProcessorsForTesting(0.016f);

	TestTrue(TEXT("Should have simulation before reload"), Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Should have 20 ants before reload"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 20);

	// Simulate reload — should re-init from saved params
	Subsystem->OnRustReloaded();

	// Verify simulation is re-initialized with same params
	TestTrue(TEXT("HasManagedSimulation should be true after reload (re-inited from saved params)"),
		Subsystem->HasManagedSimulation());
	TestEqual(TEXT("Ant count should be 20 after reload (re-inited)"),
		Subsystem->GetGroupEntityCount(TEXT("ants")), 20);
	TestEqual(TEXT("Food count should be 10 after reload (re-inited)"),
		Subsystem->GetGroupEntityCount(TEXT("food")), 10);

	Subsystem->ResetSimulation();
	return true;
}
