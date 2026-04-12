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

		// Capture all actor positions on the first frame, then check if any moved
		if (!bCapturedInitial)
		{
			for (TActorIterator<AActor> It(World); It; ++It)
			{
				AActor* Actor = *It;
				if (Actor != nullptr)
				{
					InitialPositions.Add(Actor, Actor->GetActorLocation());
				}
			}
			bCapturedInitial = true;
			return false;
		}

		// Check if any actor's Z position has changed significantly
		for (TActorIterator<AActor> It(World); It; ++It)
		{
			AActor* Actor = *It;
			if (Actor == nullptr)
			{
				continue;
			}

			const FVector* Initial = InitialPositions.Find(Actor);
			if (Initial == nullptr)
			{
				continue;
			}

			const double ZDelta = FMath::Abs(Actor->GetActorLocation().Z - Initial->Z);
			if (ZDelta > 1.0)
			{
				Test->TestTrue(TEXT("an actor's Z position should change due to Rust bob"), true);
				return true;
			}
		}

		if (FPlatformTime::Seconds() - StartTimeSeconds >= TimeoutSeconds)
		{
			Test->AddError(TEXT("no actor moved after timeout — expected Rust code to bob an actor"));
			return true;
		}

		return false;
	}

private:
	FAutomationTestBase* Test;
	double StartTimeSeconds;
	double TimeoutSeconds;
	bool bCapturedInitial = false;
	TMap<AActor*, FVector> InitialPositions;
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
