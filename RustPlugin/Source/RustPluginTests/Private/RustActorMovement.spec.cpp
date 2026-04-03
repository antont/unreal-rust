#include "Editor.h"
#include "EngineUtils.h"
#include "HAL/PlatformTime.h"
#include "Misc/AutomationTest.h"
#include "Tests/AutomationCommon.h"
#include "Tests/AutomationEditorCommon.h"

class FRustWaitForActorMovementCommand : public IAutomationLatentCommand
{
public:
	FRustWaitForActorMovementCommand(FAutomationTestBase* InTest, double InStartTimeSeconds, double InTimeoutSeconds)
		: Test(InTest),
		  StartTimeSeconds(InStartTimeSeconds),
		  TimeoutSeconds(InTimeoutSeconds)
	{
	}

	virtual bool Update() override
	{
		UWorld* World = GEditor ? GEditor->PlayWorld : nullptr;
		Test->TestNotNull(TEXT("PIE world should exist during actor movement check"), World);
		if (World == nullptr)
		{
			return true;
		}

		// Find the first actor in the world that is not a default framework actor
		AActor* TargetActor = nullptr;
		for (TActorIterator<AActor> It(World); It; ++It)
		{
			AActor* Actor = *It;
			if (Actor == nullptr)
			{
				continue;
			}

			// Skip framework actors (lights, cameras, player controllers, etc.)
			// The Rust example bobs the first "real" actor it finds via get_all_actors_of_class
			const FString ClassName = Actor->GetClass()->GetName();
			if (ClassName.Contains(TEXT("Light")) ||
				ClassName.Contains(TEXT("Camera")) ||
				ClassName.Contains(TEXT("Controller")) ||
				ClassName.Contains(TEXT("GameMode")) ||
				ClassName.Contains(TEXT("GameState")) ||
				ClassName.Contains(TEXT("PlayerState")) ||
				ClassName.Contains(TEXT("HUD")) ||
				ClassName.Contains(TEXT("GameSession")) ||
				ClassName.Contains(TEXT("WorldSettings")) ||
				ClassName.Contains(TEXT("Brush")) ||
				ClassName.Contains(TEXT("PlayerStart")) ||
				ClassName.Contains(TEXT("SkyAtmosphere")) ||
				ClassName.Contains(TEXT("SkyLight")) ||
				ClassName.Contains(TEXT("Fog")) ||
				ClassName.Contains(TEXT("PostProcess")) ||
				ClassName.Contains(TEXT("VolumetricCloud")))
			{
				continue;
			}

			TargetActor = Actor;
			break;
		}

		if (TargetActor == nullptr)
		{
			if (FPlatformTime::Seconds() - StartTimeSeconds >= TimeoutSeconds)
			{
				Test->TestNotNull(TEXT("should find a target actor in the world"), TargetActor);
				return true;
			}
			return false;
		}

		const FVector CurrentLocation = TargetActor->GetActorLocation();

		if (!InitialLocation.IsSet())
		{
			InitialLocation = CurrentLocation;
			return false;
		}

		const double ZDelta = FMath::Abs(CurrentLocation.Z - InitialLocation.GetValue().Z);
		if (ZDelta > 1.0)
		{
			Test->TestTrue(TEXT("actor Z position should change due to Rust bob"), true);
			return true;
		}

		if (FPlatformTime::Seconds() - StartTimeSeconds >= TimeoutSeconds)
		{
			Test->AddError(FString::Printf(
				TEXT("actor did not move after %.1f seconds (Z delta: %.4f)"),
				TimeoutSeconds, ZDelta));
			return true;
		}

		return false;
	}

private:
	FAutomationTestBase* Test;
	double StartTimeSeconds;
	double TimeoutSeconds;
	TOptional<FVector> InitialLocation;
};

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustActorMovementAutomationTest,
	"supplemental.RustPlugin.PIE.ActorMovedByRust",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustActorMovementAutomationTest::RunTest(const FString& Parameters)
{
	const bool bOpenedMap = AutomationOpenMap(TEXT("/Game/Blueprints/Level/Main"));
	TestTrue(TEXT("should open example Main map"), bOpenedMap);

	if (!bOpenedMap)
	{
		return false;
	}

	ADD_LATENT_AUTOMATION_COMMAND(FRustWaitForActorMovementCommand(this, FPlatformTime::Seconds(), 5.0));
	ADD_LATENT_AUTOMATION_COMMAND(FEndPlayMapCommand());
	return true;
}
