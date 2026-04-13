#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "UObject/UObjectGlobals.h"
#include "MassEntitySubsystem.h"
#include "Tests/AutomationCommon.h"
#include "Tests/AutomationEditorCommon.h"

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
	FRustMassBobActivatorRegisteredTest,
	"supplemental.RustPlugin.Mass.BobActivatorRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustMassBobActivatorRegisteredTest::RunTest(const FString& Parameters)
{
	UClass* ActivatorClass = FindObject<UClass>(nullptr, TEXT("/Script/RustMassSpike.RustMassSpikeActivator"));
	TestNotNull(TEXT("ARustMassSpikeActivator UClass should be registered"), ActivatorClass);
	return true;
}
