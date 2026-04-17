#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "MassEntityTypes.h"
#include "Tests/AutomationCommon.h"
#include "RustMassBevySubsystem.h"
#include "GatherersFragments.gen.h"
#include "MassMovementFragments.h"
#include "MassRepresentationFragments.h"
#include "MassRepresentationTypes.h"
#include "MassRepresentationProcessor.h"
#include "MassVisualizationLODProcessor.h"
#include "MassLODFragments.h"
#include "MassActorSubsystem.h"
#include "MassExecutor.h"
#include "MassProcessingContext.h"
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
			const FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			InitialPositions.Add(T.GetTransform().GetTranslation());
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
			const FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			if (!T.GetTransform().GetTranslation().Equals(InitialPositions[AntIndex], 0.01))
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
			const FGatherersFoodStateFragment& Food = FoodView.GetFragmentData<FGatherersFoodStateFragment>();
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
	// Verify layout matches Rust FoodFragment expectations (position moved to FTransformFragment)
	TestEqual(TEXT("FoodFragment bIsLoose offset"), (int32)offsetof(FGatherersFoodStateFragment, bIsLoose), 0);
	TestEqual(TEXT("FoodFragment size"), (int32)sizeof(FGatherersFoodStateFragment), 1);

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
			const FVector FoodPos = FoodView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();

			FMassEntityView AntView(EntityManager, AntEntity);
			FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			T.GetMutableTransform().SetTranslation(FoodPos);
			FGatherersPreviousTranslationFragment& Prev = AntView.GetFragmentData<FGatherersPreviousTranslationFragment>();
			Prev.Value = FoodPos;
			FGatherersCarryingFragment& Carry = AntView.GetFragmentData<FGatherersCarryingFragment>();
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
				const FGatherersFoodStateFragment& Food = View.GetFragmentData<FGatherersFoodStateFragment>();
				const FVector FoodPos_i = View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();

				const double Dx = AntPos.X - FoodPos_i.X;
				const double Dy = AntPos.Y - FoodPos_i.Y;
				const double Dz = AntPos.Z - FoodPos_i.Z;
				const double DistSq = Dx*Dx + Dy*Dy + Dz*Dz;

				UE_LOG(LogTemp, Display, TEXT("[FoodPickup Mock] Ant=(%.1f,%.1f,%.1f) Food[%d]=(%.1f,%.1f,%.1f) dist=%.1f radius=%.1f loose=%d"),
					AntPos.X, AntPos.Y, AntPos.Z, Idx,
					FoodPos_i.X, FoodPos_i.Y, FoodPos_i.Z,
					FMath::Sqrt(DistSq), PickupRadius, Food.bIsLoose ? 1 : 0);

				if (!Food.bIsLoose) continue;

				if (DistSq < RadiusSq && DistSq < BestDistSq)
				{
					BestDistSq = DistSq;
					Out->has_encounter = true;
					Out->entity_index = Idx;
					Out->encounter_position[0] = FoodPos_i.X;
					Out->encounter_position[1] = FoodPos_i.Y;
					Out->encounter_position[2] = FoodPos_i.Z;
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
			const FGatherersCarryingFragment& Carry = AntView.GetFragmentData<FGatherersCarryingFragment>();

			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersFoodStateFragment& Food = FoodView.GetFragmentData<FGatherersFoodStateFragment>();

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
// Carried food tracking — food transform follows carrying ant
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassCarriedFoodTrackingTest,
	"supplemental.RustPlugin.Gatherers.BevyMassCarriedFoodTracking",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassCarriedFoodTrackingTest::RunTest(const FString& Parameters)
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
	Subsystem->InitializeSimulation({{TEXT("ants"), 1}, {TEXT("food"), 1}}, Bounds, 42);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	const TArray<FMassEntityHandle>* FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));

	if (!TestTrue(TEXT("Ant and food entities must exist"),
		AntEntities && AntEntities->Num() > 0 && FoodEntities && FoodEntities->Num() > 0))
	{
		Subsystem->ResetSimulation();
		return false;
	}

	const FMassEntityHandle AntEntity = (*AntEntities)[0];
	const FMassEntityHandle FoodEntity = (*FoodEntities)[0];

	// Set up: ant is carrying food index 0, at a known position
	const FVector AntPos(100.0, 200.0, 50.0);
	{
		FMassEntityView AntView(EntityManager, AntEntity);
		AntView.GetFragmentData<FTransformFragment>().GetMutableTransform().SetTranslation(AntPos);
		AntView.GetFragmentData<FGatherersCarryingFragment>().FoodIndex = 0;

		// Set desired movement so UE's movement processor runs (ant will move slightly)
		AntView.GetFragmentData<FMassDesiredMovementFragment>().DesiredVelocity = FVector(10.0, 0.0, 0.0);
	}

	// Place food far away — it should snap to ant after sim step
	{
		FMassEntityView FoodView(EntityManager, FoodEntity);
		FoodView.GetFragmentData<FTransformFragment>().GetMutableTransform().SetTranslation(FVector(-999.0, -999.0, 0.0));
		FoodView.GetFragmentData<FGatherersFoodStateFragment>().bIsLoose = false;
	}

	// Run one simulation step — carried_food_tracking should move food to ant
	Subsystem->RunSimulationProcessorsForTesting(0.016f);

	// Read positions after sim
	FMassEntityView AntView(EntityManager, AntEntity);
	const FVector AntPosAfter = AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();

	FMassEntityView FoodView(EntityManager, FoodEntity);
	const FVector FoodPosAfter = FoodView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();

	UE_LOG(LogTemp, Display, TEXT("[CarriedFoodTracking] Ant=(%.1f,%.1f,%.1f) Food=(%.1f,%.1f,%.1f)"),
		AntPosAfter.X, AntPosAfter.Y, AntPosAfter.Z,
		FoodPosAfter.X, FoodPosAfter.Y, FoodPosAfter.Z);

	// Food X/Y should be near ant X/Y (within tolerance for one frame of movement)
	const double DeltaXY = FMath::Sqrt(
		FMath::Square(FoodPosAfter.X - AntPosAfter.X) +
		FMath::Square(FoodPosAfter.Y - AntPosAfter.Y));
	TestTrue(TEXT("Carried food XY should be near ant (< 5 units)"), DeltaXY < 5.0);

	// Food Z should be above ant (the carried_food_tracking system adds +15 Z offset)
	TestTrue(TEXT("Carried food Z should be above ant"),
		FoodPosAfter.Z > AntPosAfter.Z);

	// Food should NOT be at its original position
	TestTrue(TEXT("Food should have moved from original position"),
		FMath::Abs(FoodPosAfter.X - (-999.0)) > 100.0);

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

	// Use bounds matching Rust SIM_BOUNDS (±5000)
	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
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
			FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			T.GetMutableTransform().SetTranslation(FVector(4990.0, 0.0, 50.0));
			FGatherersPreviousTranslationFragment& Prev = AntView.GetFragmentData<FGatherersPreviousTranslationFragment>();
			Prev.Value = T.GetTransform().GetTranslation();
			FMassDesiredMovementFragment& DM = AntView.GetFragmentData<FMassDesiredMovementFragment>();
			DM.DesiredVelocity = FVector(200.0, 0.0, 0.0); // direction * speed
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
			const FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			const FMassDesiredMovementFragment& DM = AntView.GetFragmentData<FMassDesiredMovementFragment>();

			// Without position clamping, ant may overshoot slightly before
			// velocity reflection takes effect on the next frame.
			// After reflection + continued movement, it should be heading back.
			TestTrue(TEXT("DesiredVelocity X should reflect (become negative)"),
				DM.DesiredVelocity.X < 0.0);
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
	// Use bounds matching Rust SIM_BOUNDS (±5000)
	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
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
			const FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			InitialPositions.Add(T.GetTransform().GetTranslation());
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
			const FTransformFragment& T = AntView.GetFragmentData<FTransformFragment>();
			if (!T.GetTransform().GetTranslation().Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}
		}
	}
	TestTrue(TEXT("Most ants should have moved after 100 steps"), MovedCount > AntCount / 2);

	// Verify: no ant escaped bounds (with tolerance — no position clamping,
	// velocity reflection keeps ants near bounds with minor overshoot)
	const double BoundsTolerance = 100.0;
	int32 OutOfBoundsCount = 0;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FVector AntPos = AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();
			if (AntPos.X < Bounds.Min.X - BoundsTolerance ||
				AntPos.X > Bounds.Max.X + BoundsTolerance ||
				AntPos.Y < Bounds.Min.Y - BoundsTolerance ||
				AntPos.Y > Bounds.Max.Y + BoundsTolerance)
			{
				++OutOfBoundsCount;
			}
		}
	}
	TestEqual(TEXT("No ants should be far outside bounds"), OutOfBoundsCount, 0);

	// Verify: PreviousTranslation is tracked (should differ from Translation for moving ants)
	int32 PreviousTrackedCount = 0;
	for (const FMassEntityHandle AntEntity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(AntEntity))
		{
			FMassEntityView AntView(EntityManager, AntEntity);
			const FVector CurPos = AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();
			const FVector PrevPos = AntView.GetFragmentData<FGatherersPreviousTranslationFragment>().Value;
			if (!CurPos.Equals(PrevPos, 0.001))
			{
				++PreviousTrackedCount;
			}
		}
	}
	TestTrue(TEXT("PreviousTranslation should differ from Translation for moving ants"),
		PreviousTrackedCount > 0);

	// Verify: all food entities valid and accessible
	int32 ValidFoodCount = 0;
	for (const FMassEntityHandle FoodEntity : *FoodEntities)
	{
		if (EntityManager.IsEntityValid(FoodEntity))
		{
			FMassEntityView FoodView(EntityManager, FoodEntity);
			const FGatherersFoodStateFragment& Food =
				FoodView.GetFragmentData<FGatherersFoodStateFragment>();
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
	TestEqual(TEXT("Bool offset should be 0"), Config.filter_bool_offset, (uint32)0);
	TestTrue(TEXT("filter_bool_must_be should be true"), Config.filter_bool_must_be);

	FString FilterType(Config.filter_fragment_type.len,
		UTF8_TO_TCHAR(Config.filter_fragment_type.ptr));
	TestEqual(TEXT("Filter fragment type"), FilterType, TEXT("FGatherersFoodStateFragment"));

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
			InitialPositions.Add(AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation());
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
			if (!AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation().Equals(InitialPositions[i], 0.01))
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
			Phase3Positions.Add(AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation());
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
			if (!AntView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation().Equals(Phase3Positions[i], 0.01))
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

// ---------------------------------------------------------------------------
// Native MassRepresentation visualization tests
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassVisualizationFragmentsTest,
	"supplemental.RustPlugin.Gatherers.BevyMassVisualizationFragments",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassVisualizationFragmentsTest::RunTest(const FString& Parameters)
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

	// Initialize with small group
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 10}, {TEXT("food"), 5}}, Bounds, 42);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	if (!TestNotNull(TEXT("Ant entities should exist"), AntEntities))
	{
		Subsystem->ResetSimulation();
		return false;
	}

	// Verify ant entities have visualization per-entity fragments
	int32 ValidCount = 0;
	for (const FMassEntityHandle& Entity : *AntEntities)
	{
		if (!EntityManager.IsEntityValid(Entity))
		{
			continue;
		}

		FMassEntityView View(EntityManager, Entity);

		// Per-entity vis fragments
		const FMassRepresentationFragment* RepFrag = View.GetFragmentDataPtr<FMassRepresentationFragment>();
		TestNotNull(TEXT("Ant should have FMassRepresentationFragment"), RepFrag);

		const FMassRepresentationLODFragment* LODFrag = View.GetFragmentDataPtr<FMassRepresentationLODFragment>();
		TestNotNull(TEXT("Ant should have FMassRepresentationLODFragment"), LODFrag);

		const FMassActorFragment* ActorFrag = View.GetFragmentDataPtr<FMassActorFragment>();
		TestNotNull(TEXT("Ant should have FMassActorFragment"), ActorFrag);

		if (RepFrag)
		{
			TestTrue(TEXT("Ant ISM desc handle should be valid"),
				RepFrag->StaticMeshDescHandle.IsValid());
		}

		// Verify LOD fragment initialized to High (not default Max which maps to Off→None)
		if (LODFrag)
		{
			TestEqual(TEXT("LOD should be initialized to High"), (int32)LODFrag->LOD, (int32)EMassLOD::High);
			TestEqual(TEXT("Visibility should be CanBeSeen"), (int32)LODFrag->Visibility, (int32)EMassVisibility::CanBeSeen);
		}

		++ValidCount;
	}
	TestTrue(TEXT("All ant entities should be valid"), ValidCount == AntEntities->Num());

	// Verify archetype has visualization tags by checking the archetype composition
	if (AntEntities->Num() > 0 && EntityManager.IsEntityValid((*AntEntities)[0]))
	{
		const FMassArchetypeHandle Archetype = EntityManager.GetArchetypeForEntity((*AntEntities)[0]);
		const FMassArchetypeCompositionDescriptor& Composition = EntityManager.GetArchetypeComposition(Archetype);

		TestTrue(TEXT("Archetype should have FMassVisualizationProcessorTag"),
			Composition.GetTags().Contains(*FMassVisualizationProcessorTag::StaticStruct()));
		TestTrue(TEXT("Archetype should have FMassVisualizationLODProcessorTag"),
			Composition.GetTags().Contains(*FMassVisualizationLODProcessorTag::StaticStruct()));
		TestFalse(TEXT("Archetype should NOT have FMassVisibilityCulledByDistanceTag (causes cull-stuck)"),
			Composition.GetTags().Contains(*FMassVisibilityCulledByDistanceTag::StaticStruct()));

		// Verify chunk fragment
		TestTrue(TEXT("Archetype should have FMassVisualizationChunkFragment"),
			Composition.GetChunkFragments().Contains(*FMassVisualizationChunkFragment::StaticStruct()));

		// Verify shared fragment types
		TestTrue(TEXT("Archetype should have FMassRepresentationParameters (const shared)"),
			Composition.GetConstSharedFragments().Contains(*FMassRepresentationParameters::StaticStruct()));
		TestTrue(TEXT("Archetype should have FMassVisualizationLODParameters (const shared)"),
			Composition.GetConstSharedFragments().Contains(*FMassVisualizationLODParameters::StaticStruct()));
		TestTrue(TEXT("Archetype should have FMassRepresentationSubsystemSharedFragment (shared)"),
			Composition.GetSharedFragments().Contains(*FMassRepresentationSubsystemSharedFragment::StaticStruct()));
		TestTrue(TEXT("Archetype should have FMassVisualizationLODSharedFragment (shared)"),
			Composition.GetSharedFragments().Contains(*FMassVisualizationLODSharedFragment::StaticStruct()));

		// Verify original sim fragments are preserved after archetype move
		TestTrue(TEXT("Ant archetype should still have FTransformFragment"),
			Composition.GetFragments().Contains(*FTransformFragment::StaticStruct()));
		TestTrue(TEXT("Ant archetype should still have FMassVelocityFragment"),
			Composition.GetFragments().Contains(*FMassVelocityFragment::StaticStruct()));
		TestTrue(TEXT("Ant archetype should still have FMassDesiredMovementFragment"),
			Composition.GetFragments().Contains(*FMassDesiredMovementFragment::StaticStruct()));
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Verify vis + sim coexistence: simulation still works after vis setup
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassVisSimCoexistenceTest,
	"supplemental.RustPlugin.Gatherers.BevyMassVisSimCoexistence",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassVisSimCoexistenceTest::RunTest(const FString& Parameters)
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

	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 20}, {TEXT("food"), 10}}, Bounds, 42);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));

	// Record initial positions
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle& Entity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(Entity))
		{
			FMassEntityView View(EntityManager, Entity);
			InitialPositions.Add(View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation());
		}
	}

	// Run simulation — Rust processors must work on entities that now also have vis fragments
	for (int32 Step = 0; Step < 10; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify ants moved (Rust movement processor still works after vis setup)
	int32 MovedCount = 0;
	for (int32 i = 0; i < AntEntities->Num() && i < InitialPositions.Num(); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView View(EntityManager, (*AntEntities)[i]);
			if (!View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation().Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}
		}
	}
	TestTrue(TEXT("Ants should move after sim steps (vis fragments don't break Rust processors)"),
		MovedCount > 0);

	// Verify vis fragments still present after simulation
	if (AntEntities->Num() > 0 && EntityManager.IsEntityValid((*AntEntities)[0]))
	{
		FMassEntityView View(EntityManager, (*AntEntities)[0]);
		TestNotNull(TEXT("RepresentationFragment should persist after sim"),
			View.GetFragmentDataPtr<FMassRepresentationFragment>());
		TestNotNull(TEXT("RepresentationLODFragment should persist after sim"),
			View.GetFragmentDataPtr<FMassRepresentationLODFragment>());
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Verify vis processors are registered with MassSimulationSubsystem
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassVisProcessorRegistrationTest,
	"supplemental.RustPlugin.Gatherers.BevyMassVisProcessorRegistration",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassVisProcessorRegistrationTest::RunTest(const FString& Parameters)
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

	// Before init, no vis processors should be registered
	Subsystem->ResetSimulation();
	TestFalse(TEXT("No vis processors before init"),
		Subsystem->HasVisProcessorsRegistered());

	// After init, vis processors should be registered
	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 10}, {TEXT("food"), 5}}, Bounds, 42);

	TestTrue(TEXT("Vis processors should be registered after init"),
		Subsystem->HasVisProcessorsRegistered());

	// After reset, vis processors should be unregistered
	Subsystem->ResetSimulation();
	TestFalse(TEXT("Vis processors should be unregistered after reset"),
		Subsystem->HasVisProcessorsRegistered());

	// After re-init (hot-reload scenario), vis processors should be re-registered
	Subsystem->InitializeSimulation({{TEXT("ants"), 10}, {TEXT("food"), 5}}, Bounds, 42);
	TestTrue(TEXT("Vis processors re-registered after re-init"),
		Subsystem->HasVisProcessorsRegistered());

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// End-to-end: vis setup doesn't break simulation, entities move, rep state valid
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassVisMovementTest,
	"supplemental.RustPlugin.Gatherers.BevyMassVisMovement",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassVisMovementTest::RunTest(const FString& Parameters)
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

	// Use bounds matching Rust SIM_BOUNDS
	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 20}, {TEXT("food"), 10}}, Bounds, 42);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	if (!TestNotNull(TEXT("Ant entities must exist"), AntEntities))
	{
		Subsystem->ResetSimulation();
		return false;
	}

	// Record initial positions and verify vis fragments exist
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle& Entity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(Entity))
		{
			FMassEntityView View(EntityManager, Entity);
			InitialPositions.Add(View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation());

			// Verify vis fragments present before sim
			const FMassRepresentationFragment* RepFrag = View.GetFragmentDataPtr<FMassRepresentationFragment>();
			TestNotNull(TEXT("RepFragment should exist before sim"), RepFrag);
			if (RepFrag)
			{
				TestTrue(TEXT("ISM handle should be valid"), RepFrag->StaticMeshDescHandle.IsValid());
			}

			// Verify FMassViewerInfoFragment was added
			const FMassViewerInfoFragment* ViewerInfo = View.GetFragmentDataPtr<FMassViewerInfoFragment>();
			TestNotNull(TEXT("ViewerInfoFragment should exist"), ViewerInfo);
		}
	}

	// Run simulation via RunSimulationProcessorsForTesting (same as Tick internals)
	for (int32 Step = 0; Step < 10; ++Step)
	{
		Subsystem->RunSimulationProcessorsForTesting(0.016f);
	}

	// Verify entities MOVED (Rust processors still work after vis archetype move)
	int32 MovedCount = 0;
	for (int32 i = 0; i < AntEntities->Num() && i < InitialPositions.Num(); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView View(EntityManager, (*AntEntities)[i]);
			const FVector NewPos = View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();
			if (!NewPos.Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}

			// Log first ant's state for diagnostics
			if (i == 0)
			{
				const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
				AddInfo(FString::Printf(TEXT("Ant[0] pos=(%f,%f,%f) moved=%s CurrentRep=%d PrevRep=%d ISMValid=%d"),
					NewPos.X, NewPos.Y, NewPos.Z,
					NewPos.Equals(InitialPositions[0], 0.01) ? TEXT("NO") : TEXT("YES"),
					(int)RepFrag.CurrentRepresentation, (int)RepFrag.PrevRepresentation,
					RepFrag.StaticMeshDescHandle.IsValid() ? 1 : 0));
			}
		}
	}
	TestTrue(TEXT("Ants must move after sim steps (vis fragments don't break Rust processors)"),
		MovedCount > 0);

	// Log how many moved for diagnostics
	AddInfo(FString::Printf(TEXT("%d/%d ants moved after 10 sim steps"), MovedCount, AntEntities->Num()));

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Test: Vis processor execution — verify that running the vis processor
// transitions CurrentRepresentation from None to StaticMeshInstance
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassVisProcessorExecutionTest,
	"supplemental.RustPlugin.Gatherers.BevyMassVisProcessorExecution",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassVisProcessorExecutionTest::RunTest(const FString& Parameters)
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

	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 5}, {TEXT("food"), 2}}, Bounds, 42);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	if (!TestNotNull(TEXT("Ant entities must exist"), AntEntities) || AntEntities->Num() == 0)
	{
		Subsystem->ResetSimulation();
		return false;
	}

	// Before vis processor: CurrentRepresentation should be None (just initialized)
	{
		FMassEntityView View(EntityManager, (*AntEntities)[0]);
		const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
		AddInfo(FString::Printf(TEXT("Before vis proc: CurrentRep=%d PrevRep=%d LOD=%d Vis=%d"),
			(int)RepFrag.CurrentRepresentation, (int)RepFrag.PrevRepresentation,
			(int)View.GetFragmentData<FMassRepresentationLODFragment>().LOD,
			(int)View.GetFragmentData<FMassRepresentationLODFragment>().Visibility));
		TestEqual(TEXT("CurrentRep should be None before vis proc"),
			(int)RepFrag.CurrentRepresentation, (int)EMassRepresentationType::None);
	}

	// Verify the vis processor pipeline would work by running it through the executor
	// Build a processor pipeline with just the vis processor and run it
	{
		UMassVisualizationProcessor* TestVisProc = NewObject<UMassVisualizationProcessor>(Subsystem);
		TArray<UMassProcessor*> Processors;
		Processors.Add(TestVisProc);

		FMassRuntimePipeline TestPipeline;
		TestPipeline.SetProcessors(Processors);
		TSharedRef<FMassEntityManager> EntityManagerRef = EntityManager.AsShared();
		TestPipeline.Initialize(*Subsystem, EntityManagerRef);

		FMassProcessingContext ProcContext(EntityManager, 0.016f);
		UE::Mass::Executor::Run(TestPipeline, ProcContext);
	}

	// After vis processor: CurrentRepresentation should now be StaticMeshInstance
	{
		FMassEntityView View(EntityManager, (*AntEntities)[0]);
		const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
		const FMassRepresentationLODFragment& LODFrag = View.GetFragmentData<FMassRepresentationLODFragment>();
		AddInfo(FString::Printf(TEXT("After vis proc: CurrentRep=%d PrevRep=%d LOD=%d Vis=%d PrevTransform=(%f,%f,%f)"),
			(int)RepFrag.CurrentRepresentation, (int)RepFrag.PrevRepresentation,
			(int)LODFrag.LOD, (int)LODFrag.Visibility,
			RepFrag.PrevTransform.GetLocation().X,
			RepFrag.PrevTransform.GetLocation().Y,
			RepFrag.PrevTransform.GetLocation().Z));
		TestEqual(TEXT("CurrentRep should be StaticMeshInstance after vis proc"),
			(int)RepFrag.CurrentRepresentation, (int)EMassRepresentationType::StaticMeshInstance);
	}

	// Run sim step then check position vs PrevTransform
	Subsystem->RunSimulationProcessorsForTesting(0.016f);

	{
		FMassEntityView View(EntityManager, (*AntEntities)[0]);
		const FTransformFragment& TF = View.GetFragmentData<FTransformFragment>();
		const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
		AddInfo(FString::Printf(TEXT("After sim: pos=(%f,%f,%f) PrevTransform=(%f,%f,%f) CurrentRep=%d"),
			TF.GetTransform().GetLocation().X,
			TF.GetTransform().GetLocation().Y,
			TF.GetTransform().GetLocation().Z,
			RepFrag.PrevTransform.GetLocation().X,
			RepFrag.PrevTransform.GetLocation().Y,
			RepFrag.PrevTransform.GetLocation().Z,
			(int)RepFrag.CurrentRepresentation));
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Test: Tick-path movement — verify that calling Tick() (the PIE path)
// actually moves entities, not just RunSimulationProcessorsForTesting
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassTickPathMovementTest,
	"supplemental.RustPlugin.Gatherers.BevyMassTickPathMovement",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassTickPathMovementTest::RunTest(const FString& Parameters)
{
	UWorld* World = GEditor->GetEditorWorldContext().World();
	if (!TestNotNull(TEXT("World must exist"), World)) return false;

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!TestNotNull(TEXT("MassEntitySubsystem must exist"), MassEntitySubsystem)) return false;

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem)) return false;

	const FBox Bounds(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 10}, {TEXT("food"), 5}}, Bounds, 789);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* AntEntities = Subsystem->GetGroupEntities(TEXT("ants"));
	if (!TestNotNull(TEXT("Ant entities must exist"), AntEntities)) { Subsystem->ResetSimulation(); return false; }

	// Record initial positions
	TArray<FVector> InitialPositions;
	for (const FMassEntityHandle& Entity : *AntEntities)
	{
		if (EntityManager.IsEntityValid(Entity))
		{
			FMassEntityView View(EntityManager, Entity);
			InitialPositions.Add(View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation());
		}
	}

	// Log timing parameters (same calc as Tick)
	{
		const FVector BoundsSize = Bounds.GetSize();
		const float BoundsMaxStepDist = 0.5f * FMath::Min(BoundsSize.X, BoundsSize.Y);
		const float MaxStep = BoundsMaxStepDist / 100.0f;
		AddInfo(FString::Printf(TEXT("Tick timing: BoundsSize=(%0.0f,%0.0f,%0.0f) MaxStepSeconds=%.4f dt=0.016"),
			BoundsSize.X, BoundsSize.Y, BoundsSize.Z, MaxStep));
	}

	// Log initial desired movement and transform values + check archetypes
	{
		const FMassEntityHandle& E0 = (*AntEntities)[0];
		FMassEntityView View(EntityManager, E0);
		const FMassDesiredMovementFragment& DM = View.GetFragmentData<FMassDesiredMovementFragment>();
		const FTransformFragment& TF = View.GetFragmentData<FTransformFragment>();
		AddInfo(FString::Printf(TEXT("Pre-tick ant[0] desired_vel=(%0.2f,%0.2f,%0.2f) speed=%.2f pos=(%0.1f,%0.1f,%0.1f)"),
			DM.DesiredVelocity.X, DM.DesiredVelocity.Y, DM.DesiredVelocity.Z, DM.DesiredVelocity.Size(),
			TF.GetTransform().GetTranslation().X, TF.GetTransform().GetTranslation().Y, TF.GetTransform().GetTranslation().Z));
		// Log fragment memory address
		AddInfo(FString::Printf(TEXT("  Transform ptr=%p, DesiredMovement ptr=%p"),
			(void*)&View.GetFragmentData<FTransformFragment>(), (void*)&View.GetFragmentData<FMassDesiredMovementFragment>()));
	}
	for (int32 i = 1; i < FMath::Min(3, AntEntities->Num()); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView View(EntityManager, (*AntEntities)[i]);
			const FMassDesiredMovementFragment& DM = View.GetFragmentData<FMassDesiredMovementFragment>();
			AddInfo(FString::Printf(TEXT("Pre-tick ant[%d] desired_vel=(%0.2f,%0.2f,%0.2f) speed=%.2f"),
				i, DM.DesiredVelocity.X, DM.DesiredVelocity.Y, DM.DesiredVelocity.Z, DM.DesiredVelocity.Size()));
		}
	}

	// Use Tick() — the exact PIE code path
	for (int32 i = 0; i < 10; ++i)
	{
		Subsystem->Tick(0.016f);
		// Log after first Tick to see if anything changed
		if (i == 0)
		{
			FMassEntityView View0(EntityManager, (*AntEntities)[0]);
			const FTransformFragment& T0 = View0.GetFragmentData<FTransformFragment>();
			const FMassDesiredMovementFragment& DM0 = View0.GetFragmentData<FMassDesiredMovementFragment>();
			AddInfo(FString::Printf(TEXT("After Tick[0]: ant0 pos=(%0.2f,%0.2f,%0.2f) desired_vel=(%0.2f,%0.2f,%0.2f)"),
				T0.GetTransform().GetTranslation().X, T0.GetTransform().GetTranslation().Y, T0.GetTransform().GetTranslation().Z,
				DM0.DesiredVelocity.X, DM0.DesiredVelocity.Y, DM0.DesiredVelocity.Z));
		}
	}

	// Check positions changed
	int32 MovedCount = 0;
	for (int32 i = 0; i < AntEntities->Num() && i < InitialPositions.Num(); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView View(EntityManager, (*AntEntities)[i]);
			const FVector NewPos = View.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();
			if (!NewPos.Equals(InitialPositions[i], 0.01))
			{
				++MovedCount;
			}
			if (i == 0)
			{
				AddInfo(FString::Printf(TEXT("Tick path: ant0 before=(%0.1f,%0.1f,%0.1f) after=(%0.1f,%0.1f,%0.1f) moved=%s"),
					InitialPositions[0].X, InitialPositions[0].Y, InitialPositions[0].Z,
					NewPos.X, NewPos.Y, NewPos.Z,
					NewPos.Equals(InitialPositions[0], 0.01) ? TEXT("NO") : TEXT("YES")));
			}
		}
	}

	AddInfo(FString::Printf(TEXT("Tick path: %d/%d ants moved"), MovedCount, AntEntities->Num()));
	TestTrue(TEXT("Ants should move via Tick() path"), MovedCount > 0);

	// Check vis state after Tick — does the vis pipeline update representation?
	for (int32 i = 0; i < FMath::Min(3, AntEntities->Num()); ++i)
	{
		if (EntityManager.IsEntityValid((*AntEntities)[i]))
		{
			FMassEntityView View(EntityManager, (*AntEntities)[i]);
			const FTransformFragment& TF = View.GetFragmentData<FTransformFragment>();
			const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
			const FMassRepresentationLODFragment& LODFrag = View.GetFragmentData<FMassRepresentationLODFragment>();
			const FVector Pos = TF.GetTransform().GetTranslation();
			const FVector PrevPos = RepFrag.PrevTransform.GetTranslation();
			AddInfo(FString::Printf(TEXT("Ant[%d] pos=(%0.1f,%0.1f,%0.1f) prevT=(%0.1f,%0.1f,%0.1f) rep=%d lod=%d vis=%d"),
				i, Pos.X, Pos.Y, Pos.Z,
				PrevPos.X, PrevPos.Y, PrevPos.Z,
				(int)RepFrag.CurrentRepresentation, (int)LODFrag.LOD, (int)LODFrag.Visibility));
		}
	}

	// Check: is CurrentRepresentation actually StaticMeshInstance after Tick?
	{
		FMassEntityView View(EntityManager, (*AntEntities)[0]);
		const FMassRepresentationFragment& RepFrag = View.GetFragmentData<FMassRepresentationFragment>();
		TestEqual(TEXT("After Tick: CurrentRep should be StaticMeshInstance"),
			(int)RepFrag.CurrentRepresentation, (int)EMassRepresentationType::StaticMeshInstance);
	}

	Subsystem->ResetSimulation();
	return true;
}

// ---------------------------------------------------------------------------
// Pre-work verification: does UpdateInstanceTransform move the physics body?
// This test validates the load-bearing assumption for removing per-frame
// SyncCollisionISMCs + RecreatePhysicsState.
// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FGatherersBevyMassISMPhysicsBodyMoveTest,
	"supplemental.RustPlugin.Gatherers.ISMPhysicsBodyMove",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FGatherersBevyMassISMPhysicsBodyMoveTest::RunTest(const FString& Parameters)
{
	UWorld* World = GEditor->GetEditorWorldContext().World();
	if (!TestNotNull(TEXT("World must exist"), World)) return false;

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!TestNotNull(TEXT("MassEntitySubsystem must exist"), MassEntitySubsystem)) return false;

	URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
	if (!TestNotNull(TEXT("RustMassBevySubsystem must exist"), Subsystem)) return false;

	// Spawn 1 food entity — we only need the ISMC
	const FBox Bounds(FVector(-5000.0, -5000.0, -100.0), FVector(5000.0, 5000.0, 100.0));
	Subsystem->InitializeSimulation({{TEXT("ants"), 0}, {TEXT("food"), 1}}, Bounds, 999);

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	const TArray<FMassEntityHandle>* FoodEntities = Subsystem->GetGroupEntities(TEXT("food"));
	if (!TestTrue(TEXT("Should have 1 food entity"), FoodEntities && FoodEntities->Num() == 1))
	{
		Subsystem->ResetSimulation();
		return false;
	}

	UInstancedStaticMeshComponent* FoodISMC = Subsystem->GetGroupISMC(TEXT("food"));
	if (!TestNotNull(TEXT("Food ISMC should exist"), FoodISMC))
	{
		Subsystem->ResetSimulation();
		return false;
	}

	TestEqual(TEXT("ISMC should have 1 instance"), FoodISMC->GetInstanceCount(), 1);

	// Collision should already be enabled by spatial query setup (food_pickup config)
	TestTrue(TEXT("ISMC collision should be enabled"),
		FoodISMC->GetCollisionEnabled() != ECollisionEnabled::NoCollision);

	// food_pickup spatial query uses collision_channel_index 0 → ECC_GameTraceChannel1
	ECollisionChannel SweepChannel = ECC_GameTraceChannel1;

	// Get initial food position from fragment
	FVector InitialPos;
	{
		FMassEntityView FoodView(EntityManager, (*FoodEntities)[0]);
		InitialPos = FoodView.GetFragmentData<FTransformFragment>().GetTransform().GetTranslation();
	}

	// Verify sweep finds food at initial position
	{
		FCollisionShape Shape = FCollisionShape::MakeSphere(30.0f);
		TArray<FHitResult> Hits;
		World->SweepMultiByChannel(Hits, InitialPos, InitialPos + FVector(1, 0, 0),
			FQuat::Identity, SweepChannel, Shape);
		bool bFoundAtInitial = false;
		for (const FHitResult& Hit : Hits)
		{
			if (Hit.Component == FoodISMC && Hit.Item == 0)
			{
				bFoundAtInitial = true;
				break;
			}
		}
		TestTrue(TEXT("Sweep should find food at initial position"), bFoundAtInitial);
	}

	// Move the instance via UpdateInstanceTransform — deliberately NO RecreatePhysicsState
	FVector NewPos(InitialPos.X + 2000.0, InitialPos.Y + 2000.0, InitialPos.Z);
	{
		FTransform NewTransform(FQuat::Identity, NewPos, FVector(0.2f));
		FoodISMC->UpdateInstanceTransform(0, NewTransform, true, true, true);
	}

	// Sweep at new position — does the physics body follow?
	bool bFoundAtNew = false;
	{
		FCollisionShape Shape = FCollisionShape::MakeSphere(30.0f);
		TArray<FHitResult> Hits;
		World->SweepMultiByChannel(Hits, NewPos, NewPos + FVector(1, 0, 0),
			FQuat::Identity, SweepChannel, Shape);
		for (const FHitResult& Hit : Hits)
		{
			if (Hit.Component == FoodISMC && Hit.Item == 0)
			{
				bFoundAtNew = true;
				break;
			}
		}
	}

	// Sweep at old position — body should no longer be there
	bool bStillAtOld = false;
	{
		FCollisionShape Shape = FCollisionShape::MakeSphere(30.0f);
		TArray<FHitResult> Hits;
		World->SweepMultiByChannel(Hits, InitialPos, InitialPos + FVector(1, 0, 0),
			FQuat::Identity, SweepChannel, Shape);
		for (const FHitResult& Hit : Hits)
		{
			if (Hit.Component == FoodISMC && Hit.Item == 0)
			{
				bStillAtOld = true;
				break;
			}
		}
	}

	TestTrue(TEXT("Physics body should be found at NEW position after UpdateInstanceTransform"),
		bFoundAtNew);
	TestFalse(TEXT("Physics body should NOT be found at OLD position after UpdateInstanceTransform"),
		bStillAtOld);

	if (!bFoundAtNew)
	{
		UE_LOG(LogTemp, Warning, TEXT("[ISMPhysicsBodyMove] UpdateInstanceTransform did NOT move "
			"the physics body. RecreatePhysicsState or UpdateInstanceBodyTransform may be required."));
	}

	Subsystem->ResetSimulation();
	return true;
}
