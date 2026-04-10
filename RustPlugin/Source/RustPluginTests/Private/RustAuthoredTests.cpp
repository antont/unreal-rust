// RustAuthoredTests.cpp — Generic runner for Rust-authored UE automation tests.
//
// Uses IMPLEMENT_COMPLEX_AUTOMATION_TEST so each Rust test appears as its own
// entry in the UE test browser (supplemental.RustPlugin.RustTests.<TestName>).
//
// Test logic lives in Rust (game crates register via inventory::submit!).
// This C++ file is one-time infrastructure — never modified per-game.

#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "MassEntitySubsystem.h"
#include "MassEntityView.h"
#include "RustMassBevySubsystem.h"
#include "Bindings.h"
#include "RustPlugin.h"

// ---------------------------------------------------------------------------
// Test callback implementations — wrap URustMassBevySubsystem methods for Rust
// ---------------------------------------------------------------------------

static uint32_t CB_InitSim(void* Ctx, const MassInitSimulationParams* Params)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);

	TArray<TPair<FString, int32>> GroupCounts;
	for (uint32_t i = 0; i < Params->num_groups; ++i)
	{
		FString Name(Params->groups[i].name.len,
			UTF8_TO_TCHAR(Params->groups[i].name.ptr));
		GroupCounts.Add(TPair<FString, int32>(Name, Params->groups[i].count));
	}

	FBox Bounds(
		FVector(Params->bounds_min[0], Params->bounds_min[1], Params->bounds_min[2]),
		FVector(Params->bounds_max[0], Params->bounds_max[1], Params->bounds_max[2]));

	Sub->InitializeSimulation(GroupCounts, Bounds, Params->random_seed);
	return 1;
}

static void CB_StepSim(void* Ctx, float Dt, uint32_t Count)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	for (uint32_t i = 0; i < Count; ++i)
	{
		Sub->RunSimulationProcessorsForTesting(Dt);
	}
}

static void CB_ResetSim(void* Ctx)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	Sub->ResetSimulation();
}

static void CB_Tick(void* Ctx, float Dt)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	Sub->Tick(Dt);
}

static void CB_OnRustReloaded(void* Ctx)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	Sub->OnRustReloaded();
}

static int32_t CB_EntityCount(void* Ctx, Utf8Str Group)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	FString GroupName(Group.len, UTF8_TO_TCHAR(Group.ptr));
	return Sub->GetGroupEntityCount(GroupName);
}

static uint32_t CB_HasManagedSim(void* Ctx)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	return Sub->HasManagedSimulation() ? 1 : 0;
}

static uint32_t CB_HasSpatialQuery(void* Ctx, Utf8Str Name)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	FString QueryName(Name.len, UTF8_TO_TCHAR(Name.ptr));
	return Sub->HasSpatialQuery(QueryName) ? 1 : 0;
}

static uint32_t CB_ReadFragment(void* Ctx, Utf8Str Group, uint32_t Index,
	Utf8Str FragmentType, uint8_t* Out, uint32_t Size)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	FString GroupName(Group.len, UTF8_TO_TCHAR(Group.ptr));
	FString TypeName(FragmentType.len, UTF8_TO_TCHAR(FragmentType.ptr));
	return Sub->ReadFragmentData(GroupName, (int32)Index, TypeName, Out, (int32)Size) ? 1 : 0;
}

static uint32_t CB_WriteFragment(void* Ctx, Utf8Str Group, uint32_t Index,
	Utf8Str FragmentType, const uint8_t* Data, uint32_t Size)
{
	auto* Sub = static_cast<URustMassBevySubsystem*>(Ctx);
	FString GroupName(Group.len, UTF8_TO_TCHAR(Group.ptr));
	FString TypeName(FragmentType.len, UTF8_TO_TCHAR(FragmentType.ptr));
	return Sub->WriteFragmentData(GroupName, (int32)Index, TypeName, Data, (int32)Size) ? 1 : 0;
}

static MassTestCallbacks BuildTestCallbacks(URustMassBevySubsystem* Subsystem)
{
	MassTestCallbacks Callbacks;
	Callbacks.opaque = Subsystem;
	Callbacks.init_sim = CB_InitSim;
	Callbacks.step_sim = CB_StepSim;
	Callbacks.reset_sim = CB_ResetSim;
	Callbacks.tick = CB_Tick;
	Callbacks.on_rust_reloaded = CB_OnRustReloaded;
	Callbacks.entity_count = CB_EntityCount;
	Callbacks.has_managed_sim = CB_HasManagedSim;
	Callbacks.has_spatial_query = CB_HasSpatialQuery;
	Callbacks.read_fragment = CB_ReadFragment;
	Callbacks.write_fragment = CB_WriteFragment;
	return Callbacks;
}

// ---------------------------------------------------------------------------
// Complex automation test — discovers and runs Rust-authored tests
// ---------------------------------------------------------------------------

IMPLEMENT_COMPLEX_AUTOMATION_TEST(
	FRustAuthoredMassTests,
	"supplemental.RustPlugin.RustTests",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

void FRustAuthoredMassTests::GetTests(
	TArray<FString>& OutBeautifiedNames,
	TArray<FString>& OutTestCommands) const
{
	FRustPluginModule& Module = FModuleManager::GetModuleChecked<FRustPluginModule>("RustPlugin");
	if (Module.Plugin.Rust.get_mass_test_count.IsNone() ||
		Module.Plugin.Rust.get_mass_test_desc.IsNone())
	{
		return;
	}

	uint32 Count = Module.Plugin.Rust.get_mass_test_count.Unwrap()();
	for (uint32 i = 0; i < Count; ++i)
	{
		MassTestDesc Desc;
		if (Module.Plugin.Rust.get_mass_test_desc.Unwrap()(i, &Desc))
		{
			FString Name(Desc.name.len, UTF8_TO_TCHAR(Desc.name.ptr));
			OutBeautifiedNames.Add(Name);
			OutTestCommands.Add(Name);
		}
	}
}

bool FRustAuthoredMassTests::RunTest(const FString& Parameters)
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
	if (Module.Plugin.Rust.run_mass_test.IsNone())
	{
		AddError(TEXT("run_mass_test function not available in Rust bindings"));
		return false;
	}

	// Build callbacks and run the Rust test
	MassTestCallbacks Callbacks = BuildTestCallbacks(Subsystem);

	// Convert FString to UTF-8 for Rust
	FTCHARToUTF8 Utf8Name(*Parameters);
	Utf8Str NameStr;
	NameStr.ptr = Utf8Name.Get();
	NameStr.len = Utf8Name.Length();

	MassTestResult Result = Module.Plugin.Rust.run_mass_test.Unwrap()(NameStr, &Callbacks);

	if (!Result.passed)
	{
		FString ErrorMsg;
		if (Result.error_ptr && Result.error_len > 0)
		{
			ErrorMsg = FString(Result.error_len, UTF8_TO_TCHAR(Result.error_ptr));
		}
		else
		{
			ErrorMsg = TEXT("(no error message)");
		}
		AddError(FString::Printf(TEXT("Rust test '%s' failed: %s"), *Parameters, *ErrorMsg));
	}

	// Always reset simulation after each test
	Subsystem->ResetSimulation();

	return Result.passed != 0;
}
