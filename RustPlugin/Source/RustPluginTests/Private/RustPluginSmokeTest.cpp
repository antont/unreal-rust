#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "Modules/ModuleManager.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustPluginModuleSmokeTest,
	"supplemental.RustPlugin.Module.SmokeTest",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustPluginModuleSmokeTest::RunTest(const FString& Parameters)
{
	const bool bModuleLoaded = FModuleManager::Get().IsModuleLoaded(TEXT("RustPlugin"));
	TestTrue(TEXT("RustPlugin module should be loaded"), bModuleLoaded);

	return true;
}

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustPluginTypeRegistrationTest,
	"supplemental.RustPlugin.Types.TestDataAssetRegistered",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustPluginTypeRegistrationTest::RunTest(const FString& Parameters)
{
	UClass* TestDataAssetClass = FindObject<UClass>(nullptr, TEXT("/Script/Rust.TestDataAsset"));
	TestNotNull(TEXT("TestDataAsset UClass should be registered from Rust"), TestDataAssetClass);

	if (TestDataAssetClass != nullptr)
	{
		TestTrue(
			TEXT("TestDataAsset should be a child of UDataAsset"),
			TestDataAssetClass->IsChildOf(UDataAsset::StaticClass()));
	}

	return true;
}
