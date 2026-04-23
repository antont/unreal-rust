// PIE-driven frame-cost perf tests.
//
// Unlike the subsystem-only perf tests in RustMassGatherers.spec.cpp (which
// call Subsystem->Tick() or RunSimulationProcessorsForTesting() directly),
// these tests run inside a real PIE session so each measured frame includes
// the full engine tick: render, Slate, actor ticks, other Mass subsystems,
// the global world tick. That's what we want when asking "what does an
// actual editor frame cost at 1k / 10k ants?"
//
// Diagnostic-only. No pass/fail on frame times. Results land in the log as
// `[pie-perf]` lines and in the utrace bracketed by TRACE_BOOKMARK.
//
// Reuses the existing GatherersBevyMass map and overrides the entity counts
// from the test side (ResetSimulation + InitializeSimulation) so no new
// .umap assets need to be committed.
//
// CLI:
//   Automation RunTests supplemental.RustPlugin.Gatherers.PIE;Quit
// Trace capture:
//   -trace=cpu,frame,bookmark -tracefile=<path>

#include "Editor.h"
#include "EngineUtils.h"
#include "HAL/PlatformTime.h"
#include "Misc/AutomationTest.h"
#include "ProfilingDebugging/MiscTrace.h"
#include "Tests/AutomationCommon.h"
#include "Tests/AutomationEditorCommon.h"

#include "RustMassBevySubsystem.h"

namespace
{
	// Warmup: skip first-frame ISMC allocation, shader compile jitter, sim
	// initialization ramp. Measured: 300 frames = 5s at 60 Hz.
	constexpr int32 kWarmupFrames = 60;
	constexpr int32 kMeasuredFrames = 300;

	// Matches BevyMassPerfProfile's bounds so both tests see the same density
	// at comparable scales.
	const FBox kSimBounds(FVector(-5000.0, -5000.0, 0.0), FVector(5000.0, 5000.0, 100.0));
}

// On first tick: reset whatever the activator set up during BeginPlay, then
// re-initialize the sim to the test-specified scale. Subsequent ticks just
// pump editor frames until the warmup budget is reached.
class FRustPIEReinitAndWarmupCommand : public IAutomationLatentCommand
{
public:
	FRustPIEReinitAndWarmupCommand(FAutomationTestBase* InTest, int32 InAnts, int32 InFrames)
		: Test(InTest), Ants(InAnts), Frames(InFrames)
	{
	}

	virtual bool Update() override
	{
		if (!bReinitialized)
		{
			UWorld* World = GEditor ? GEditor->PlayWorld : nullptr;
			Test->TestNotNull(TEXT("PIE world should exist"), World);
			if (World == nullptr)
			{
				return true;
			}
			URustMassBevySubsystem* Subsystem = World->GetSubsystem<URustMassBevySubsystem>();
			Test->TestNotNull(TEXT("RustMassBevySubsystem should exist in PIE world"), Subsystem);
			if (Subsystem == nullptr)
			{
				return true;
			}

			// Wait for the activator's BeginPlay-driven InitializeSimulation to
			// land before we reset + re-initialize at our target scale.
			if (!Subsystem->HasManagedSimulation())
			{
				return false;
			}

			Subsystem->ResetSimulation();
			const int32 Food = Ants * 4;
			const TArray<TPair<FString, int32>> Groups = {
				TPair<FString, int32>(TEXT("ants"), Ants),
				TPair<FString, int32>(TEXT("food"), Food),
			};
			Subsystem->InitializeSimulation(Groups, kSimBounds, 42);
			Test->TestEqual(TEXT("ant count"),
				Subsystem->GetGroupEntityCount(TEXT("ants")), Ants);
			Test->TestEqual(TEXT("food count"),
				Subsystem->GetGroupEntityCount(TEXT("food")), Food);
			bReinitialized = true;
		}

		return ++TickCount >= Frames;
	}

private:
	FAutomationTestBase* Test;
	int32 Ants;
	int32 Frames;
	int32 TickCount = 0;
	bool bReinitialized = false;
};

// On each Update() (one per editor tick), records the wall-clock elapsed
// since the previous Update() into Samples. After N samples, summarizes into
// a single `[pie-perf] <scenario>` line and returns true. Brackets the
// window with TRACE_BOOKMARK so Insights can slice warmup vs measured.
class FRustPIEMeasureCommand : public IAutomationLatentCommand
{
public:
	FRustPIEMeasureCommand(FAutomationTestBase* InTest, FString InScenario, int32 InFrames)
		: Test(InTest), Scenario(MoveTemp(InScenario)), Frames(InFrames)
	{
		Samples.Reserve(Frames);
	}

	virtual bool Update() override
	{
		const double Now = FPlatformTime::Seconds();
		if (!bStarted)
		{
			TRACE_BOOKMARK(TEXT("PIEPerf: Begin %s"), *Scenario);
			WallStart = Now;
			LastTick = Now;
			bStarted = true;
			return false;
		}

		Samples.Add((Now - LastTick) * 1000.0);
		LastTick = Now;

		if (Samples.Num() < Frames)
		{
			return false;
		}

		const double WallMs = (Now - WallStart) * 1000.0;
		TRACE_BOOKMARK(TEXT("PIEPerf: End %s"), *Scenario);

		TArray<double> Sorted = Samples;
		Sorted.Sort();
		const double Min = Sorted[0];
		const double Max = Sorted.Last();
		const double P50 = Sorted[Sorted.Num() / 2];
		const double P99 = Sorted[(Sorted.Num() * 99) / 100];
		double Sum = 0.0;
		for (double S : Samples) { Sum += S; }
		const double Avg = Sum / Samples.Num();

		Test->AddInfo(FString::Printf(
			TEXT("[pie-perf] %s %d frames: avg=%.3fms p50=%.3fms p99=%.3fms min=%.3fms max=%.3fms total=%.1fms wall=%.1fms"),
			*Scenario, Samples.Num(), Avg, P50, P99, Min, Max, Sum, WallMs));

		return true;
	}

private:
	FAutomationTestBase* Test;
	FString Scenario;
	int32 Frames;
	TArray<double> Samples;
	double WallStart = 0.0;
	double LastTick = 0.0;
	bool bStarted = false;
};

static bool RunPIEPerfScenario(FAutomationTestBase* Test, int32 Ants, const TCHAR* Scenario)
{
	Test->AddExpectedError(TEXT("Template actor type .* is not referring to a valid type"),
		EAutomationExpectedErrorFlags::Contains, -1);

	const bool bOpenedMap = AutomationOpenMap(TEXT("/Game/Gatherers/GatherersBevyMass"));
	Test->TestTrue(TEXT("should open GatherersBevyMass map"), bOpenedMap);
	if (!bOpenedMap)
	{
		return false;
	}

	ADD_LATENT_AUTOMATION_COMMAND(FRustPIEReinitAndWarmupCommand(Test, Ants, kWarmupFrames));
	ADD_LATENT_AUTOMATION_COMMAND(FRustPIEMeasureCommand(Test, Scenario, kMeasuredFrames));
	ADD_LATENT_AUTOMATION_COMMAND(FEndPlayMapCommand());
	return true;
}

// ---------------------------------------------------------------------------

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustPIEFrameCost1kTest,
	"supplemental.RustPlugin.Gatherers.PIE.FrameCost1k",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustPIEFrameCost1kTest::RunTest(const FString& Parameters)
{
	return RunPIEPerfScenario(this, 1000, TEXT("1k"));
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustPIEFrameCost10kTest,
	"supplemental.RustPlugin.Gatherers.PIE.FrameCost10k",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustPIEFrameCost10kTest::RunTest(const FString& Parameters)
{
	return RunPIEPerfScenario(this, 10000, TEXT("10k"));
}
