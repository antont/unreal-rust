#include "Misc/AutomationTest.h"
#include "UObject/UObjectGlobals.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustMassBobProcessorRegisteredTest,
	"supplemental.RustPlugin.Mass.BobProcessorRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustMassBobProcessorRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* ProcessorClass = FindObject<UClass>(nullptr, TEXT("/Script/RustMassSpike.RustBobProcessor"));
	TestNotNull(TEXT("URustBobProcessor UClass should be registered"), ProcessorClass);
	return true;
}
