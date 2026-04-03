#include "Editor.h"
#include "Misc/AutomationTest.h"
#include "RustUtils.h"
#include "RustPlugin.h"

IMPLEMENT_SIMPLE_AUTOMATION_TEST(
	FRustPluginLoaderSmokeTest,
	"supplemental.RustPlugin.Loader.SmokeTest",
	EAutomationTestFlags::EditorContext | EAutomationTestFlags::EngineFilter)

bool FRustPluginLoaderSmokeTest::RunTest(const FString& Parameters)
{
	FRustPluginModule& Module = GetRustModule();

	TestTrue(TEXT("Rust loader should be loaded"), Module.Plugin.IsLoaded());

	// IsRustOutOfDate should not crash
	const bool bOutOfDate = Module.Plugin.IsRustOutOfDate();
	TestFalse(TEXT("Rust plugin should not be out of date immediately after load"), bOutOfDate);

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
