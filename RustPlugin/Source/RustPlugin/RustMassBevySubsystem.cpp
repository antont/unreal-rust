#include "RustMassBevySubsystem.h"

#include "MassCommonFragments.h"
#include "Engine/World.h"
#include "GameFramework/Actor.h"
#include "MassExecutor.h"
#include "MassEntityManager.h"
#include "MassProcessingContext.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "RustMassDynamicProcessor.h"
#include "RustMassScheduleCoordinator.h"
#include "RustPlugin.h"
#include "RustUtils.h"
#include "RustMassPostMovementProcessor.h"
#include "Movement/MassMovementProcessors.h"
#include "RustMassVisualizationSetup.h"
#include "MassSimulationSubsystem.h"
#include "MassRepresentationProcessor.h"
#include "MassVisualizationLODProcessor.h"
#include "MassUpdateISMProcessor.h"
#include "CollisionShape.h"
#include "StructUtils/StructView.h"
#include "Engine/StaticMesh.h"
#include "Materials/MaterialInstanceDynamic.h"
#include "Materials/MaterialInterface.h"

// ---------------------------------------------------------------------------
// Spatial query trampolines: bounce C function pointers to game-module delegates
// ---------------------------------------------------------------------------

namespace RustMassSpatialQuery
{
	static constexpr int32 MaxQueries = 8;
	static URustMassBevySubsystem::FSpatialQueryCallback* ActiveCallbacks[MaxQueries] = {};

	template<int N>
	uint32_t Trampoline(
		const double* PreviousPos,
		const double* CurrentPos,
		float PickupRadius,
		MassSpatialQueryResult* Out)
	{
		if (ActiveCallbacks[N] && *ActiveCallbacks[N])
		{
			return (*ActiveCallbacks[N])(PreviousPos, CurrentPos, PickupRadius, Out);
		}
		if (Out)
		{
			Out->has_encounter = false;
			Out->entity_index = -1;
		}
		return 0;
	}

	static MassSpatialQueryFn TrampolineFns[MaxQueries] = {
		&Trampoline<0>, &Trampoline<1>, &Trampoline<2>, &Trampoline<3>,
		&Trampoline<4>, &Trampoline<5>, &Trampoline<6>, &Trampoline<7>,
	};
} // namespace RustMassSpatialQuery

// ---------------------------------------------------------------------------
// Subsystem lifecycle
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
	bProcessorPipelinesInitialized = false;
}

void URustMassBevySubsystem::Deinitialize()
{
	SimulationProcessorPipeline.Reset();
	VisualizationPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	Super::Deinitialize();
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

const TArray<FMassEntityHandle>* URustMassBevySubsystem::GetGroupEntities(const FString& GroupName) const
{
	return EntityGroups.Find(GroupName);
}

UInstancedStaticMeshComponent* URustMassBevySubsystem::GetGroupISMC(const FString& GroupName) const
{
	for (const FCollisionGroupEntry& Entry : CollisionGroups)
	{
		if (Entry.Name == GroupName)
		{
			return Entry.ISMC;
		}
	}
	return nullptr;
}

int32 URustMassBevySubsystem::GetGroupEntityCount(const FString& GroupName) const
{
	const TArray<FMassEntityHandle>* Arr = EntityGroups.Find(GroupName);
	return Arr ? Arr->Num() : 0;
}

bool URustMassBevySubsystem::HasManagedSimulation() const
{
	for (const auto& Pair : EntityGroups)
	{
		if (Pair.Value.Num() > 0) return true;
	}
	return false;
}

void URustMassBevySubsystem::RegisterSpatialQuery(const FString& QueryName, FSpatialQueryCallback InCallback, float InRadius)
{
	// Preserve the trampoline index if re-registering an existing query
	int32 ExistingTrampolineIndex = -1;
	if (FSpatialQueryEntry* Existing = SpatialQueries.Find(QueryName))
	{
		ExistingTrampolineIndex = Existing->TrampolineIndex;
	}

	FSpatialQueryEntry Entry;
	Entry.Name = QueryName;
	Entry.Callback = MoveTemp(InCallback);
	Entry.Radius = InRadius;
	Entry.TrampolineIndex = ExistingTrampolineIndex;
	SpatialQueries.Add(QueryName, MoveTemp(Entry));
}

void URustMassBevySubsystem::SetupSpatialQueriesFromRust()
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (!Module.Plugin.Rust.get_spatial_query_config_count.IsSome() ||
		!Module.Plugin.Rust.get_spatial_query_config_desc.IsSome())
	{
		return;
	}

	uint32 ConfigCount = Module.Plugin.Rust.get_spatial_query_config_count.Unwrap()();

	for (uint32 ConfigIdx = 0; ConfigIdx < ConfigCount; ++ConfigIdx)
	{
		MassSpatialQueryConfigDesc Config = {};
		if (Module.Plugin.Rust.get_spatial_query_config_desc.Unwrap()(ConfigIdx, &Config) == 0)
		{
			continue;
		}

		// Extract config values into captured variables
		FString QueryName = FString(Config.query_name.len, UTF8_TO_TCHAR(Config.query_name.ptr));
		FString QueryGroup = FString(Config.query_group.len, UTF8_TO_TCHAR(Config.query_group.ptr));
		FString FilterFragmentType = FString(Config.filter_fragment_type.len, UTF8_TO_TCHAR(Config.filter_fragment_type.ptr));
		float Radius = Config.radius;
		uint32 FilterBoolOffset = Config.filter_bool_offset;
		bool FilterBoolMustBe = Config.filter_bool_must_be;
		uint8 QueryType = Config.query_type;
		uint8 CollisionChannelIndex = Config.collision_channel_index;

		// Resolve the filter fragment UScriptStruct for runtime access.
		FString SearchName = FilterFragmentType;
		if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
		{
			SearchName.RightChopInline(1);
		}
		const UScriptStruct* FilterStruct = FindFirstObject<UScriptStruct>(
			*SearchName, EFindFirstObjectOptions::NativeFirst);

		if (!FilterStruct)
		{
			UE_LOG(LogTemp, Warning, TEXT("RustMassBevySubsystem: Could not find UScriptStruct '%s'. "
				"Spatial query filtering disabled for '%s'."), *FilterFragmentType, *QueryName);
		}

		URustMassBevySubsystem* Self = this;

		if (QueryType == 1) // PhysicsSweep
		{
			if (CollisionChannelIndex > 17)
			{
				UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Invalid collision channel index %d for query '%s' "
					"(max is 17 for ECC_GameTraceChannel18). Skipping."), CollisionChannelIndex, *QueryName);
				continue;
			}

			// Enable collision on the target group's ISMC for physics queries
			ECollisionChannel Channel = static_cast<ECollisionChannel>(ECC_GameTraceChannel1 + CollisionChannelIndex);
			UInstancedStaticMeshComponent* TargetISMC = GetGroupISMC(QueryGroup);
			if (TargetISMC)
			{
				TargetISMC->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
				TargetISMC->SetCollisionResponseToAllChannels(ECR_Ignore);
				TargetISMC->SetCollisionResponseToChannel(Channel, ECR_Block);
				// Flush physics bodies so SweepMultiByChannel can find ISMC instances
				// immediately, without waiting for the next world tick.
				TargetISMC->RecreatePhysicsState();
			}

			RegisterSpatialQuery(QueryName,
				[Self, QueryGroup, FilterBoolOffset, FilterBoolMustBe, FilterStruct, Channel](
					const double* PreviousPos, const double* CurrentPos,
					float PickupRadius, MassSpatialQueryResult* Out) -> uint32
				{
					if (!Out) return 0;

					Out->has_encounter = false;
					Out->entity_index = -1;
					Out->encounter_position[0] = 0.0;
					Out->encounter_position[1] = 0.0;
					Out->encounter_position[2] = 0.0;

					const TArray<FMassEntityHandle>* TargetEntities = Self->GetGroupEntities(QueryGroup);
					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					if (!TargetEntities || !W || !MES)
					{
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
					const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
					const float PickupRadiusSq = PickupRadius * PickupRadius;

					// UE physics sweep
					FCollisionShape Shape = FCollisionShape::MakeSphere(PickupRadius);
					TArray<FHitResult> Hits;
					W->SweepMultiByChannel(Hits, SweepStart, SweepEnd, FQuat::Identity, Channel, Shape);

					// Find nearest hit along sweep direction (earliest collision)
					float BestHitDistance = TNumericLimits<float>::Max();
					for (const FHitResult& Hit : Hits)
					{
						int32 Idx = Hit.Item; // ISMC instance index
						if (!TargetEntities->IsValidIndex(Idx)) continue;

						const FMassEntityHandle Entity = (*TargetEntities)[Idx];
						if (!EntityManager.IsEntityValid(Entity)) continue;

						if (FilterStruct)
						{
							FMassEntityView EntityView(EntityManager, Entity);
							FStructView FragView = EntityView.GetFragmentDataStruct(FilterStruct);
							if (FragView.IsValid())
							{
								const uint8* FragmentBytes = FragView.GetMemory();
								bool FilterValue = *reinterpret_cast<const bool*>(FragmentBytes + FilterBoolOffset);
								if (FilterValue != FilterBoolMustBe) continue;
							}
						}

						if (Hit.Distance < BestHitDistance)
						{
							BestHitDistance = Hit.Distance;
							const FVector HitPoint = Hit.ImpactPoint;
							Out->has_encounter = true;
							Out->entity_index = Idx;
							Out->encounter_position[0] = HitPoint.X;
							Out->encounter_position[1] = HitPoint.Y;
							Out->encounter_position[2] = HitPoint.Z;
						}
					}

					return 1;
				},
				Radius);
		}
		else // IsmcOverlap (default)
		{
			RegisterSpatialQuery(QueryName,
				[Self, QueryGroup, FilterBoolOffset, FilterBoolMustBe, FilterStruct](
					const double* PreviousPos, const double* CurrentPos,
					float PickupRadius, MassSpatialQueryResult* Out) -> uint32
				{
					if (!Out) return 0;

					Out->has_encounter = false;
					Out->entity_index = -1;
					Out->encounter_position[0] = 0.0;
					Out->encounter_position[1] = 0.0;
					Out->encounter_position[2] = 0.0;

					const TArray<FMassEntityHandle>* TargetEntities = Self->GetGroupEntities(QueryGroup);
					UInstancedStaticMeshComponent* TargetISMC = Self->GetGroupISMC(QueryGroup);

					UWorld* W = Self->GetWorld();
					UMassEntitySubsystem* MES = W ? W->GetSubsystem<UMassEntitySubsystem>() : nullptr;
					if (!TargetEntities || !TargetISMC || !MES)
					{
						return 0;
					}

					FMassEntityManager& EntityManager = MES->GetMutableEntityManager();
					const FVector SweepStart(PreviousPos[0], PreviousPos[1], PreviousPos[2]);
					const FVector SweepEnd(CurrentPos[0], CurrentPos[1], CurrentPos[2]);
					const float PickupRadiusSq = PickupRadius * PickupRadius;

					// ISMC spatial query: sphere or box depending on movement
					TSet<int32> CandidateIndices;
					if (SweepStart.Equals(SweepEnd))
					{
						for (const int32 Idx : TargetISMC->GetInstancesOverlappingSphere(
							SweepEnd, PickupRadius, true))
						{
							if (TargetEntities->IsValidIndex(Idx))
							{
								CandidateIndices.Add(Idx);
							}
						}
					}
					else
					{
						FBox SweptBounds(ForceInit);
						SweptBounds += SweepStart;
						SweptBounds += SweepEnd;
						SweptBounds = SweptBounds.ExpandBy(PickupRadius);
						for (const int32 Idx : TargetISMC->GetInstancesOverlappingBox(SweptBounds, true))
						{
							if (TargetEntities->IsValidIndex(Idx))
							{
								CandidateIndices.Add(Idx);
							}
						}
					}

					// Find nearest matching entity
					float BestDistSq = TNumericLimits<float>::Max();
					for (const int32 Idx : CandidateIndices)
					{
						const FMassEntityHandle Entity = (*TargetEntities)[Idx];
						if (!EntityManager.IsEntityValid(Entity)) continue;

						if (FilterStruct)
						{
							FMassEntityView EntityView(EntityManager, Entity);
							FStructView FragView = EntityView.GetFragmentDataStruct(FilterStruct);
							if (FragView.IsValid())
							{
								const uint8* FragmentBytes = FragView.GetMemory();
								bool FilterValue = *reinterpret_cast<const bool*>(FragmentBytes + FilterBoolOffset);
								if (FilterValue != FilterBoolMustBe) continue;
							}
						}

						FTransform InstanceTransform;
						TargetISMC->GetInstanceTransform(Idx, InstanceTransform, true);
						const FVector EntityPos = InstanceTransform.GetLocation();

						const FVector Closest = FMath::ClosestPointOnSegment(
							EntityPos, SweepStart, SweepEnd);
						const float DistSq = FVector::DistSquared(Closest, EntityPos);
						if (DistSq <= PickupRadiusSq && DistSq < BestDistSq)
						{
							BestDistSq = DistSq;
							Out->has_encounter = true;
							Out->entity_index = Idx;
							Out->encounter_position[0] = Closest.X;
							Out->encounter_position[1] = Closest.Y;
							Out->encounter_position[2] = Closest.Z;
						}
					}

					return 1;
				},
				Radius);
		}

		UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-setup spatial query '%s' for group '%s' "
			"(type=%s, radius=%.1f, filter=%s, bool_offset=%d, must_be=%s)"),
			*QueryName, *QueryGroup,
			QueryType == 1 ? TEXT("PhysicsSweep") : TEXT("IsmcOverlap"),
			Radius, *FilterFragmentType, FilterBoolOffset,
			FilterBoolMustBe ? TEXT("true") : TEXT("false"));
	}
}

// ---------------------------------------------------------------------------
// Processor pipelines
// ---------------------------------------------------------------------------

bool URustMassBevySubsystem::EnsureProcessorPipelines(UMassEntitySubsystem& MassEntitySubsystem)
{
	if (bProcessorPipelinesInitialized)
	{
		return true;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem.GetMutableEntityManager();
	TSharedRef<FMassEntityManager> EntityManagerRef = EntityManager.AsShared();

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	TArray<URustMassDynamicProcessor*> DynamicProcessors =
		URustMassDynamicProcessor::CreateAllRustProcessors(Module.Plugin.Rust, this);

	TArray<UMassProcessor*> SimProcessors;

	DynamicProcessors.Sort([](const URustMassDynamicProcessor& A, const URustMassDynamicProcessor& B)
	{
		return A.GetExecutionOrder() < B.GetExecutionOrder();
	});

	for (URustMassDynamicProcessor* Proc : DynamicProcessors)
	{
		SimProcessors.Add(Proc);
	}

	if (Module.Plugin.Rust.mass_frame_dispatch != nullptr && DynamicProcessors.Num() > 0)
	{
		ScheduleCoordinator = NewObject<URustMassScheduleCoordinator>(this);
		ScheduleCoordinator->InitializeDispatch(Module.Plugin.Rust.mass_frame_dispatch, DynamicProcessors);

		// Build spatial query slots from registered queries
		if (SpatialQueries.Num() > 0)
		{
			TArray<MassSpatialQuerySlot> Slots;
			TArray<TArray<char>> NameBuffers;
			int32 SlotIdx = 0;
			for (auto& Pair : SpatialQueries)
			{
				if (SlotIdx >= RustMassSpatialQuery::MaxQueries)
				{
					UE_LOG(LogTemp, Error, TEXT("RustMassBevySubsystem: Too many spatial queries (%d), max is %d. Excess queries will be silently ignored — increase MaxQueries."),
						SpatialQueries.Num(), RustMassSpatialQuery::MaxQueries);
					ensureMsgf(false, TEXT("Spatial query count %d exceeds MaxQueries %d"), SpatialQueries.Num(), RustMassSpatialQuery::MaxQueries);
					break;
				}

				// Store the trampoline index in the entry for stable callback mapping
				Pair.Value.TrampolineIndex = SlotIdx;

				// Convert name to UTF-8 and store in a persistent buffer
				FTCHARToUTF8 Converter(*Pair.Key);
				TArray<char> NameBuf;
				NameBuf.Append(Converter.Get(), Converter.Length());
				NameBuf.Add(0); // null-terminate

				MassSpatialQuerySlot Slot = {};
				Slot.name.ptr = nullptr; // fixed up by coordinator after move
				Slot.name.len = Converter.Length();
				Slot.query_fn = RustMassSpatialQuery::TrampolineFns[SlotIdx];
				Slot.radius = Pair.Value.Radius;
				Slots.Add(Slot);
				NameBuffers.Add(MoveTemp(NameBuf));
				++SlotIdx;
			}
			ScheduleCoordinator->SetSpatialQuerySlots(MoveTemp(Slots), MoveTemp(NameBuffers));
		}
		SimProcessors.Add(ScheduleCoordinator);
	}

	// Pre-movement: save PreviousTranslation (for spatial sweeps) + clamp to bounds.
	// Runs BEFORE native movement so PreviousTranslation captures position before this frame's move.
	PostMovementProcessor = NewObject<URustMassPostMovementProcessor>(this);
	SimProcessors.Add(PostMovementProcessor);

	// UE's native movement processor: reads DesiredMovement, writes Velocity + Transform.
	// Requires FMassDesiredMovementFragment + FMassCodeDrivenMovementTag on entities.
	NativeMovementProcessor = NewObject<UMassApplyMovementProcessor>(this);
	SimProcessors.Add(NativeMovementProcessor);

	if (SimProcessors.Num() > 0)
	{
		SimulationProcessorPipeline.SetProcessors(SimProcessors);
		SimulationProcessorPipeline.Initialize(*this, EntityManagerRef);
	}

	bProcessorPipelinesInitialized = true;
	return true;
}

// ---------------------------------------------------------------------------
// Simulation step
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::RunSimulationProcessorStep(float SimulatedDeltaTime)
{
	if (!HasManagedSimulation())
	{
		return;
	}

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr || !EnsureProcessorPipelines(*MassEntitySubsystem))
	{
		return;
	}

	// Set spatial query trampoline contexts using stored indices
	for (auto& Pair : SpatialQueries)
	{
		const int32 Idx = Pair.Value.TrampolineIndex;
		if (Idx >= 0 && Idx < RustMassSpatialQuery::MaxQueries)
		{
			RustMassSpatialQuery::ActiveCallbacks[Idx] = &Pair.Value.Callback;
		}
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	FMassProcessingContext SimulationContext(EntityManager, SimulatedDeltaTime);
	UE::Mass::Executor::Run(SimulationProcessorPipeline, SimulationContext);

	// Clear after dispatch
	for (int32 i = 0; i < RustMassSpatialQuery::MaxQueries; ++i)
	{
		RustMassSpatialQuery::ActiveCallbacks[i] = nullptr;
	}
}

// ---------------------------------------------------------------------------
// Collision-only ISMCs (for spatial queries — rendering uses native MassRepresentation)
// ---------------------------------------------------------------------------

namespace
{
	constexpr TCHAR CollisionSphereMeshPath[] = TEXT("/Engine/BasicShapes/Sphere.Sphere");

	const UScriptStruct* FindFragmentStructByName(const FString& CppTypeName)
	{
		FString SearchName = CppTypeName;
		if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
		{
			SearchName.RightChopInline(1);
		}
		return FindFirstObject<UScriptStruct>(*SearchName, EFindFirstObjectOptions::NativeFirst);
	}
} // anonymous namespace

void URustMassBevySubsystem::InitializeCollisionISMCs(UWorld* World, const RustBindings& Bindings, FMassEntityManager& EntityManager)
{
	if (!World) return;
	if (Bindings.get_visualizer_group_count.IsNone() || Bindings.get_visualizer_group_desc.IsNone())
	{
		return;
	}

	const uint32 GroupCount = Bindings.get_visualizer_group_count.Unwrap()();
	if (GroupCount == 0) return;

	UStaticMesh* SphereMesh = LoadObject<UStaticMesh>(nullptr, CollisionSphereMeshPath);
	if (!SphereMesh) return;

	// Spawn actor to own collision ISMCs
	if (!CollisionActor)
	{
		FActorSpawnParameters SpawnParams;
		SpawnParams.SpawnCollisionHandlingOverride = ESpawnActorCollisionHandlingMethod::AlwaysSpawn;
		CollisionActor = World->SpawnActor<AActor>(AActor::StaticClass(), FTransform::Identity, SpawnParams);
		if (!CollisionActor) return;

		USceneComponent* Root = NewObject<USceneComponent>(CollisionActor, TEXT("CollisionRoot"));
		CollisionActor->AddInstanceComponent(Root);
		Root->RegisterComponent();
		CollisionActor->SetRootComponent(Root);
	}

	USceneComponent* RootComponent = CollisionActor->GetRootComponent();

	for (uint32 i = 0; i < GroupCount; ++i)
	{
		MassVisualizerGroupDesc Desc = {};
		if (Bindings.get_visualizer_group_desc.Unwrap()(i, &Desc) == 0) continue;

		FString GroupName = FString(Desc.name.len, UTF8_TO_TCHAR(Desc.name.ptr));
		FString FragTypeName = FString(Desc.position_fragment_type.len, UTF8_TO_TCHAR(Desc.position_fragment_type.ptr));

		const UScriptStruct* FragStruct = FindFragmentStructByName(FragTypeName);
		if (!FragStruct)
		{
			UE_LOG(LogTemp, Warning, TEXT("RustMassBevySubsystem: Could not find USTRUCT '%s' for collision group '%s'"),
				*FragTypeName, *GroupName);
			continue;
		}

		FCollisionGroupEntry Entry;
		Entry.Name = GroupName;
		Entry.PositionStruct = FragStruct;
		Entry.PositionOffset = Desc.position_offset;
		Entry.Scale = FVector(Desc.scale);

		FName ComponentName(*FString::Printf(TEXT("Collision_%s"), *GroupName));
		Entry.ISMC = NewObject<UInstancedStaticMeshComponent>(CollisionActor, ComponentName);
		CollisionActor->AddInstanceComponent(Entry.ISMC);
		Entry.ISMC->SetupAttachment(RootComponent);
		Entry.ISMC->RegisterComponent();
		Entry.ISMC->SetStaticMesh(SphereMesh);
		Entry.ISMC->SetMobility(EComponentMobility::Movable);
		Entry.ISMC->SetCollisionEnabled(ECollisionEnabled::NoCollision); // enabled per-group by spatial query setup
		Entry.ISMC->SetCastShadow(false);
		Entry.ISMC->SetCanEverAffectNavigation(false);
		Entry.ISMC->SetHiddenInGame(true); // invisible — rendering via native MassRepresentation
		Entry.ISMC->SetVisibility(false);

		// Populate initial instances from entity positions
		const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
		if (Entities)
		{
			for (const FMassEntityHandle& Entity : *Entities)
			{
				if (!EntityManager.IsEntityValid(Entity)) continue;
				FMassEntityView View(EntityManager, Entity);
				FStructView FragView = View.GetFragmentDataStruct(Entry.PositionStruct);
				const uint8* FragData = FragView.GetMemory();
				if (!FragData) continue;
				const double* Pos = reinterpret_cast<const double*>(FragData + Entry.PositionOffset);
				FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Entry.Scale);
				Entry.ISMC->AddInstance(T, true);
			}
		}

		UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Collision ISMC '%s': %d instances (frag='%s', offset=%u)"),
			*GroupName, Entry.ISMC->GetInstanceCount(), *FragTypeName, Entry.PositionOffset);
		CollisionGroups.Add(MoveTemp(Entry));
	}
}

void URustMassBevySubsystem::SyncCollisionISMCs(FMassEntityManager& EntityManager)
{
	for (FCollisionGroupEntry& Group : CollisionGroups)
	{
		if (!Group.ISMC) continue;

		// Only sync groups that have collision enabled (spatial queries active)
		if (Group.ISMC->GetCollisionEnabled() == ECollisionEnabled::NoCollision) continue;

		const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(Group.Name);
		if (!Entities) continue;

		// Rebuild if count mismatch
		if (Group.ISMC->GetInstanceCount() != Entities->Num())
		{
			Group.ISMC->ClearInstances();
			for (const FMassEntityHandle& Entity : *Entities)
			{
				if (!EntityManager.IsEntityValid(Entity)) continue;
				FMassEntityView View(EntityManager, Entity);
				FStructView FragView = View.GetFragmentDataStruct(Group.PositionStruct);
				const uint8* FragData = FragView.GetMemory();
				if (!FragData) continue;
				const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
				FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
				Group.ISMC->AddInstance(T, true);
			}
			continue;
		}

		const bool IsLast = (&Group == &CollisionGroups.Last());
		for (int32 i = 0; i < Entities->Num(); ++i)
		{
			if (!EntityManager.IsEntityValid((*Entities)[i])) continue;
			FMassEntityView View(EntityManager, (*Entities)[i]);
			FStructView FragView = View.GetFragmentDataStruct(Group.PositionStruct);
			const uint8* FragData = FragView.GetMemory();
			if (!FragData) continue;
			const double* Pos = reinterpret_cast<const double*>(FragData + Group.PositionOffset);
			FTransform T(FQuat::Identity, FVector(Pos[0], Pos[1], Pos[2]), Group.Scale);
			const bool bMarkDirty = (i == Entities->Num() - 1 && IsLast);
			Group.ISMC->UpdateInstanceTransform(i, T, true, bMarkDirty, true);
		}
	}
}

void URustMassBevySubsystem::TeardownCollisionISMCs()
{
	for (auto& Group : CollisionGroups)
	{
		if (Group.ISMC)
		{
			Group.ISMC->ClearInstances();
		}
	}
	CollisionGroups.Empty();

	if (CollisionActor)
	{
		CollisionActor->Destroy();
		CollisionActor = nullptr;
	}
}

// ---------------------------------------------------------------------------
// Tick
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::TryAutoInitFromRustDefaults()
{
	if (bAutoInitAttempted) return;
	bAutoInitAttempted = true;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (!Module.Plugin.Rust.get_sim_defaults.IsSome()) return;

	MassSimDefaultsDesc Defaults = {};
	if (Module.Plugin.Rust.get_sim_defaults.Unwrap()(&Defaults) == 0) return;

	TArray<TPair<FString, int32>> GroupCounts;
	for (uint32 i = 0; i < Defaults.num_groups; ++i)
	{
		FString Name = FString(Defaults.groups[i].name.len,
			UTF8_TO_TCHAR(Defaults.groups[i].name.ptr));
		GroupCounts.Add(TPair<FString, int32>(Name, Defaults.groups[i].count));
	}

	FBox Bounds(
		FVector(Defaults.bounds_min[0], Defaults.bounds_min[1], Defaults.bounds_min[2]),
		FVector(Defaults.bounds_max[0], Defaults.bounds_max[1], Defaults.bounds_max[2]));

	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Auto-initializing from Rust defaults "
		"(%d groups, seed=%d)"), GroupCounts.Num(), Defaults.random_seed);

	InitializeSimulation(GroupCounts, Bounds, Defaults.random_seed);
}

void URustMassBevySubsystem::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	if (!HasManagedSimulation())
	{
		TryAutoInitFromRustDefaults();
		if (!HasManagedSimulation()) return;
	}

	const float SimulatedSecondsThisFrame = FMath::Max(0.0f, DeltaTime);
	SimulationTimeAccumulatorSeconds += SimulatedSecondsThisFrame;



	const FVector BoundsSize = SimulationBounds.GetSize();
	const float BoundsMaxStepDistance = SimulationBounds.IsValid
		? 0.5f * FMath::Min(BoundsSize.X, BoundsSize.Y)
		: 500.0f;
	const float DefaultMovementSpeed = 100.0f;
	const float MaxStepSeconds = BoundsMaxStepDistance / FMath::Max(KINDA_SMALL_NUMBER, DefaultMovementSpeed);

	int32 StepsExecutedThisFrame = 0;
	constexpr int32 MaxStepsPerTick = 4096;
	while (SimulationTimeAccumulatorSeconds >= MaxStepSeconds
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(MaxStepSeconds);
		SimulationTimeAccumulatorSeconds -= MaxStepSeconds;
		++StepsExecutedThisFrame;
	}

	if (SimulationTimeAccumulatorSeconds > KINDA_SMALL_NUMBER
		&& StepsExecutedThisFrame < MaxStepsPerTick)
	{
		RunSimulationProcessorStep(SimulationTimeAccumulatorSeconds);
		SimulationTimeAccumulatorSeconds = 0.0f;
		++StepsExecutedThisFrame;
	}




	// Run visualization pipeline (Representation only) after simulation updates transforms
	if (VisProcessor)
	{
		UWorld* World = GetWorld();
		UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;
		if (MassEntitySubsystem)
		{
			FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
			FMassProcessingContext VisContext(EntityManager, DeltaTime);
			UE::Mass::Executor::Run(VisualizationPipeline, VisContext);
		}
	}

	// Apply targeted ISM updates for food drops (no per-frame full sync needed)
	if (CollisionGroups.Num() > 0)
	{
		FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
		if (Module.Plugin.Rust.get_food_drop_events.IsSome())
		{
			constexpr uint32_t MaxDropEvents = 64;
			FoodDropEvent DropEvents[MaxDropEvents];
			uint32_t Count = Module.Plugin.Rust.get_food_drop_events.Unwrap()(DropEvents, MaxDropEvents);
			ensureMsgf(Count < MaxDropEvents, TEXT("Food drop events saturated buffer (%u). Some ISM updates were lost."), Count);
			for (uint32_t i = 0; i < Count; ++i)
			{
				const FoodDropEvent& Evt = DropEvents[i];
				for (auto& Group : CollisionGroups)
				{
					if (Group.ISMC && Group.ISMC->GetCollisionEnabled() != ECollisionEnabled::NoCollision)
					{
						FTransform T(FQuat::Identity, FVector(Evt.position[0], Evt.position[1], Evt.position[2]), Group.Scale);
						Group.ISMC->UpdateInstanceTransform(Evt.food_index, T, true, true, true);
					}
				}
			}
		}
	}
}

TStatId URustMassBevySubsystem::GetStatId() const
{
	RETURN_QUICK_DECLARE_CYCLE_STAT(URustMassBevySubsystem, STATGROUP_Tickables);
}

// ---------------------------------------------------------------------------
// Init / Reset
// ---------------------------------------------------------------------------

void URustMassBevySubsystem::InitializeSimulation(
	const TArray<TPair<FString, int32>>& GroupCounts,
	const FBox& Bounds,
	int32 RandomSeed)
{
	ResetSimulation();

	// Save params for hot-reload re-initialization
	SavedGroupCounts = GroupCounts;
	SavedBounds = Bounds;
	SavedRandomSeed = RandomSeed;
	bHasSavedInitParams = true;

	UWorld* World = GetWorld();
	if (World == nullptr)
	{
		return;
	}

	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (MassEntitySubsystem == nullptr)
	{
		return;
	}

	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
	RustMassSpawnSetEntityManager(&EntityManager);
	SimulationBounds = Bounds;

	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");

	if (Module.Plugin.Rust.mass_init_simulation.IsSome())
	{
		// Build params with named groups
		TArray<MassEntityGroupDesc> Descs;
		TArray<TArray<uint8>> NameBuffers; // keep name bytes alive
		for (const auto& Pair : GroupCounts)
		{
			TArray<uint8> NameBytes;
			FTCHARToUTF8 Converter(*Pair.Key);
			NameBytes.Append(reinterpret_cast<const uint8*>(Converter.Get()), Converter.Length());
			NameBytes.Add(0); // null-terminate

			MassEntityGroupDesc Desc = {};
			Desc.name.ptr = reinterpret_cast<const char*>(NameBytes.GetData());
			Desc.name.len = Converter.Length();
			Desc.count = Pair.Value;
			Descs.Add(Desc);
			NameBuffers.Add(MoveTemp(NameBytes));
		}
		// Fix up pointers after NameBuffers is stable (no more Add)
		for (int32 i = 0; i < Descs.Num(); ++i)
		{
			Descs[i].name.ptr = reinterpret_cast<const char*>(NameBuffers[i].GetData());
		}

		MassInitSimulationParams Params = {};
		Params.groups = Descs.GetData();
		Params.num_groups = Descs.Num();
		Params.bounds_min[0] = Bounds.Min.X; Params.bounds_min[1] = Bounds.Min.Y; Params.bounds_min[2] = Bounds.Min.Z;
		Params.bounds_max[0] = Bounds.Max.X; Params.bounds_max[1] = Bounds.Max.Y; Params.bounds_max[2] = Bounds.Max.Z;
		Params.random_seed = RandomSeed;

		MassInitSimulationResult Result = {};
		Module.Plugin.Rust.mass_init_simulation.Unwrap()(&Params, &Result);

		for (uint32 g = 0; g < Result.num_groups; ++g)
		{
			const MassEntityGroupResult& Group = Result.groups[g];
			FString GroupName = FString(Group.name.len, UTF8_TO_TCHAR(Group.name.ptr));
			TArray<FMassEntityHandle>& Entities = EntityGroups.FindOrAdd(GroupName);
			for (uint32 i = 0; i < Group.count; ++i)
			{
				Entities.Add(FMassEntityHandle(Group.handles[i].index, Group.handles[i].serial_number));
			}
		}
	}

	// Initialize collision-only ISMCs (for spatial queries)
	InitializeCollisionISMCs(World, Module.Plugin.Rust, EntityManager);

	// Setup native MassRepresentation visualization
	{
		if (!VisualizationSetup)
		{
			VisualizationSetup = NewObject<URustMassVisualizationSetup>(this);
		}

		UStaticMesh* SphereMesh = LoadObject<UStaticMesh>(nullptr, TEXT("/Engine/BasicShapes/Sphere.Sphere"));
		UMaterialInterface* BaseMaterial = LoadObject<UMaterialInterface>(nullptr, TEXT("/Engine/BasicShapes/BasicShapeMaterial.BasicShapeMaterial"));

		static const FLinearColor GroupColors[] = {
			FLinearColor(0.2f, 0.8f, 0.8f, 1.0f),   // cyan (ants)
			FLinearColor(0.75f, 0.01f, 0.01f, 1.0f),  // red (food)
			FLinearColor(0.1f, 0.8f, 0.2f, 1.0f),     // green
			FLinearColor(0.9f, 0.7f, 0.1f, 1.0f),     // yellow
		};
		static constexpr int32 NumGroupColors = UE_ARRAY_COUNT(GroupColors);

		if (SphereMesh)
		{
			int32 GroupIdx = 0;
			for (const auto& Pair : EntityGroups)
			{
				// Determine scale from collision group (if available) or default
				FVector GroupScale = FVector(0.2f);
				for (const FCollisionGroupEntry& CG : CollisionGroups)
				{
					if (CG.Name == Pair.Key)
					{
						GroupScale = CG.Scale;
						break;
					}
				}

				UMaterialInterface* GroupMaterial = nullptr;
				if (BaseMaterial)
				{
					UMaterialInstanceDynamic* MatInst = UMaterialInstanceDynamic::Create(BaseMaterial, this);
					if (MatInst)
					{
						MatInst->SetVectorParameterValue(TEXT("Color"), GroupColors[GroupIdx % NumGroupColors]);
						GroupMaterial = MatInst;
					}
				}

				float LODDistances[4] = { 0.0f, 2000.0f, 5000.0f, 15000.0f };
				VisualizationSetup->SetupGroupVisualization(
					World, EntityManager, Pair.Value,
					SphereMesh, GroupMaterial, GroupScale, LODDistances);

				UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Native vis configured for group '%s' (%d entities)"),
					*Pair.Key, Pair.Value.Num());
				++GroupIdx;
			}
		}

		// Build a visualization pipeline that we run from our own Tick.
		// (Don't register with MassSimulationSubsystem — we control timing.)
		// NOTE: LOD processor omitted — our custom pipeline doesn't run the
		// LODCollector that populates FMassViewerInfoFragment, so the LOD
		// processor defaults all entities to LOD=Off / CulledByDistance.
		// Without LOD evaluation, entities stay at High LOD (set at init)
		// and the vis processor updates ISM transforms every frame.
		if (!VisProcessor)
		{
			VisProcessor = NewObject<UMassVisualizationProcessor>(this);
		}
		{
			TArray<UMassProcessor*> VisProcessors;
			VisProcessors.Add(VisProcessor);
			// UMassUpdateISMProcessor: feeds current + prev transforms to ISM batching,
			// then saves PrevTransform = current for next frame (motion blur / TAA).
			VisProcessors.Add(NewObject<UMassUpdateISMProcessor>(this));
			VisualizationPipeline.SetProcessors(VisProcessors);
			TSharedRef<FMassEntityManager> EMRef = EntityManager.AsShared();
			VisualizationPipeline.Initialize(*this, EMRef);
			UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Built vis pipeline (Representation + UpdateISM, no LOD)"));
		}
	}

	int32 TotalEntities = 0;
	for (const auto& Pair : EntityGroups)
	{
		TotalEntities += Pair.Value.Num();
	}
	UE_LOG(LogTemp, Log, TEXT("RustMassBevySubsystem: Initialized %d entity groups (%d total entities)"),
		EntityGroups.Num(), TotalEntities);

	// Auto-setup spatial queries from Rust-registered config (if none already set)
	if (SpatialQueries.Num() == 0)
	{
		SetupSpatialQueriesFromRust();
	}
}

void URustMassBevySubsystem::ResetSimulation()
{
	UWorld* World = GetWorld();
	UMassEntitySubsystem* MassEntitySubsystem = World ? World->GetSubsystem<UMassEntitySubsystem>() : nullptr;

	TeardownCollisionISMCs();

	// Reset vis pipeline and processors before tearing down entities
	VisualizationPipeline.Reset();
	VisProcessor = nullptr;
	VisLODProcessor = nullptr;

	if (VisualizationSetup)
	{
		VisualizationSetup->Teardown();
	}

	if (MassEntitySubsystem != nullptr)
	{
		FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();
		for (auto& Pair : EntityGroups)
		{
			for (const FMassEntityHandle Entity : Pair.Value)
			{
				if (EntityManager.IsEntityValid(Entity))
				{
					EntityManager.DestroyEntity(Entity);
				}
			}
		}
	}

	EntityGroups.Empty();
	SimulationBounds = FBox(EForceInit::ForceInit);
	SimulationTimeAccumulatorSeconds = 0.0f;
	SimulationProcessorPipeline.Reset();
	bProcessorPipelinesInitialized = false;
	ScheduleCoordinator = nullptr;
	NativeMovementProcessor = nullptr;
	PostMovementProcessor = nullptr;
	SpatialQueries.Empty();
	bAutoInitAttempted = false;
}

void URustMassBevySubsystem::OnRustReloaded()
{
	UE_LOG(LogTemp, Display, TEXT("RustMassBevySubsystem: Rust hot-reload detected, re-initializing simulation"));
	if (bHasSavedInitParams)
	{
		// Re-init with the same params the activator actor originally provided.
		// InitializeSimulation calls ResetSimulation() internally.
		InitializeSimulation(SavedGroupCounts, SavedBounds, SavedRandomSeed);
	}
	else
	{
		ResetSimulation();
	}
}

void URustMassBevySubsystem::RunSimulationProcessorsForTesting(float DeltaTime)
{
	RunSimulationProcessorStep(FMath::Max(0.0f, DeltaTime));

	// Apply targeted ISM updates for food drops
	if (CollisionGroups.Num() > 0)
	{
		FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
		if (Module.Plugin.Rust.get_food_drop_events.IsSome())
		{
			constexpr uint32_t MaxDropEvents = 64;
			FoodDropEvent DropEvents[MaxDropEvents];
			uint32_t Count = Module.Plugin.Rust.get_food_drop_events.Unwrap()(DropEvents, MaxDropEvents);
			ensureMsgf(Count < MaxDropEvents, TEXT("Food drop events saturated buffer (%u). Some ISM updates were lost."), Count);
			for (uint32_t i = 0; i < Count; ++i)
			{
				const FoodDropEvent& Evt = DropEvents[i];
				for (auto& Group : CollisionGroups)
				{
					if (Group.ISMC && Group.ISMC->GetCollisionEnabled() != ECollisionEnabled::NoCollision)
					{
						FTransform T(FQuat::Identity, FVector(Evt.position[0], Evt.position[1], Evt.position[2]), Group.Scale);
						Group.ISMC->UpdateInstanceTransform(Evt.food_index, T, true, true, true);
					}
				}
			}
		}
	}
}

bool URustMassBevySubsystem::ReadFragmentData(const FString& GroupName, int32 EntityIndex,
	const FString& FragmentTypeName, void* OutData, int32 DataSize) const
{
	if (!OutData || DataSize <= 0) return false;

	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
	if (!Entities || EntityIndex < 0 || EntityIndex >= Entities->Num()) return false;

	UWorld* World = GetWorld();
	if (!World) return false;
	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!MassEntitySubsystem) return false;
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();

	const FMassEntityHandle Entity = (*Entities)[EntityIndex];
	if (!EntityManager.IsEntityValid(Entity)) return false;

	// Find UScriptStruct by name
	// Strip F/U prefix for UE object lookup (same pattern as SetupSpatialQueriesFromRust)
	FString SearchName = FragmentTypeName;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	UScriptStruct* FragmentStruct = FindFirstObject<UScriptStruct>(
		*SearchName, EFindFirstObjectOptions::NativeFirst);
	if (!FragmentStruct) return false;
	if (FragmentStruct->GetStructureSize() != DataSize) return false;

	FStructView FragmentView = EntityManager.GetFragmentDataStruct(Entity, FragmentStruct);
	if (!FragmentView.IsValid()) return false;

	FMemory::Memcpy(OutData, FragmentView.GetMemory(), DataSize);
	return true;
}

bool URustMassBevySubsystem::WriteFragmentData(const FString& GroupName, int32 EntityIndex,
	const FString& FragmentTypeName, const void* InData, int32 DataSize)
{
	if (!InData || DataSize <= 0) return false;

	const TArray<FMassEntityHandle>* Entities = EntityGroups.Find(GroupName);
	if (!Entities || EntityIndex < 0 || EntityIndex >= Entities->Num()) return false;

	UWorld* World = GetWorld();
	if (!World) return false;
	UMassEntitySubsystem* MassEntitySubsystem = World->GetSubsystem<UMassEntitySubsystem>();
	if (!MassEntitySubsystem) return false;
	FMassEntityManager& EntityManager = MassEntitySubsystem->GetMutableEntityManager();

	const FMassEntityHandle Entity = (*Entities)[EntityIndex];
	if (!EntityManager.IsEntityValid(Entity)) return false;

	// Strip F/U prefix for UE object lookup (same pattern as SetupSpatialQueriesFromRust)
	FString SearchName = FragmentTypeName;
	if (SearchName.Len() > 1 && (SearchName[0] == TEXT('F') || SearchName[0] == TEXT('U')))
	{
		SearchName.RightChopInline(1);
	}
	UScriptStruct* FragmentStruct = FindFirstObject<UScriptStruct>(
		*SearchName, EFindFirstObjectOptions::NativeFirst);
	if (!FragmentStruct) return false;
	if (FragmentStruct->GetStructureSize() != DataSize) return false;

	FStructView FragmentView = EntityManager.GetFragmentDataStruct(Entity, FragmentStruct);
	if (!FragmentView.IsValid()) return false;

	FMemory::Memcpy(FragmentView.GetMemory(), InData, DataSize);
	return true;
}
