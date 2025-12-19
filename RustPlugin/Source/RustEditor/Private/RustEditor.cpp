#include "RustEditor.h"

#include "JsonObjectConverter.h"
#include "Reflection.h"
#include "RustEditorCommands.h"
#include "SourceCodeNavigation.h"

#define LOCTEXT_NAMESPACE "FRustEditorModule"

void FRustEditorModule::StartupModule()
{
	FRustStyle::Initialize();
	RegisterCommands();
	RegisterMenu();
}

void FRustEditorModule::ShutdownModule()
{
}

void FRustEditorModule::RegisterMenu()
{
	UToolMenu* ToolbarMenu = UToolMenus::Get()->ExtendMenu("LevelEditor.LevelEditorToolBar.PlayToolBar");
	FToolMenuSection& Section = ToolbarMenu->FindOrAddSection("PluginTools");

	FToolMenuEntry Entry = FToolMenuEntry::InitComboButton(
		"Rust",
		FUIAction(),
		FOnGetContent::CreateLambda([this]() { return GenerateRustMenu(); }),
		LOCTEXT("Rust_Label", "Rust"),
		LOCTEXT("Rust_Tooltip", "Rust commands"),
		TAttribute<FSlateIcon>::CreateLambda([this]()
		{
			return FSlateIcon(FRustStyle::GetStyleSetName(), "Rust.Toolbar.Header");
		}));

	Section.AddEntry(Entry);
}

void FRustEditorModule::OnDumpReflectionApi()
{
	auto Root = FRustReflection_Root::Collect();
	FString JsonString;
	if (FJsonObjectConverter::UStructToJsonObjectString(Root, JsonString))
	{
		FFileHelper::SaveStringToFile(JsonString, TEXT("C:\\Users\\maikk\\Documents\\unreal-rust\\api.json"));
		// UE_LOG(LogTemp, Log, TEXT("%s"), *JsonString);
	}
}

void FRustEditorModule::RegisterCommands()
{
	FRustEditorCommands::Register();
	RustCommands = MakeShareable(new FUICommandList);
	RustCommands->MapAction(FRustEditorCommands::Get().DumpUnrealReflectionApi,
	                        FExecuteAction::CreateRaw(this, &FRustEditorModule::OnDumpReflectionApi));
}

TSharedRef<SWidget> FRustEditorModule::GenerateRustMenu() const
{
	const FRustEditorCommands& RustEditorCommands = FRustEditorCommands::Get();
	FMenuBuilder MenuBuilder(true, RustCommands);
	MenuBuilder.BeginSection("Project", LOCTEXT("Project", "Project"));
	MenuBuilder.AddMenuEntry(RustEditorCommands.DumpUnrealReflectionApi, NAME_None, TAttribute<FText>(),
	                         TAttribute<FText>(),
	                         FSourceCodeNavigation::GetOpenSourceCodeIDEIcon());
	MenuBuilder.EndSection();

	return MenuBuilder.MakeWidget();
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_MODULE(FRustEditorModule, RustEditor)
