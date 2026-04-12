#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "UObject/UObjectGlobals.h"
#include "MassEntitySubsystem.h"
#include "Tests/AutomationCommon.h"
#include "Tests/AutomationEditorCommon.h"
#include "HAL/PlatformTime.h"

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

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustMassBobSubsystemRegisteredTest,
	"supplemental.RustPlugin.Mass.SubsystemRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustMassBobSubsystemRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* SubsystemClass = FindObject<UClass>(nullptr, TEXT("/Script/RustMassSpike.RustMassSpikeSubsystem"));
	TestNotNull(TEXT("URustMassSpikeSubsystem UClass should be registered"), SubsystemClass);
	return true;
}
