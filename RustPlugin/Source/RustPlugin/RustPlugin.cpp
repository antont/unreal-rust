// Copyright Epic Games, Inc. All Rights Reserved.

#include "RustPlugin.h"
#include "RustPluginStyle.h"
#include "RustPluginCommands.h"
#include "LevelEditor.h"
#include "Widgets/Docking/SDockTab.h"
#include "Widgets/Layout/SBox.h"
#include "Widgets/Text/STextBlock.h"
#include "ToolMenus.h"
#include "DirectoryWatcherModule.h"
#include "IDirectoryWatcher.h"
#include "Misc/Paths.h"
#include "Modules/ModuleManager.h"
#include "RustUtils.h"
#include "Bindings.h"
#include "HAL/PlatformFileManager.h"

#include <stdint.h>

static const FName RustPluginTabName("RustPlugin");

#define LOCTEXT_NAMESPACE "FRustPluginModule"

FString PlatformExtensionName()
{
#if PLATFORM_LINUX || PLATFORM_MAC
	return FString(TEXT("so"));
#elif PLATFORM_WINDOWS
	return FString(TEXT("dll"));
#endif
}

FString FRustLoader::PluginFolderPath()
{
	return FPaths::Combine(*FPaths::ConvertRelativePathToFull(FPaths::ProjectDir()), TEXT("Binaries"));
}

FString FRustLoader::PluginPath()
{
	return FPaths::Combine(*PluginFolderPath(), *PluginFileName());
}

FString FRustLoader::PluginFileName()
{
	return FString::Printf(TEXT("%s.%s"), TEXT("unreal_rust_loader"), *PlatformExtensionName());
}

FString FRustLoader::PluginPdbPath()
{
	return FPaths::Combine(*PluginFolderPath(), TEXT("rustplugin.pdb"));
}

FRustLoader::FRustLoader()
{
}

bool FRustLoader::TryLoad()
{
	// Loading ddls is a bit tricky, see https://fasterthanli.me/articles/so-you-want-to-live-reload-rust
	// The gist is we can't easily hot reload a dll if the dll uses the thread local storage (TLS).
	// The TLS will prevent the dll from being unloaded even when we call `dlclose`. And `dlopen` will return
	// the pointer to the previously loaded dll.
	// Essentially this means hot reloading will do nothing as we can't unload the currently loaded dll.
	// The workaround for this is give each dll a unique name. Here we append the unix timestamp at
	// the end of the file. That way we can force `dlopen` to load the dll.
	// Please note this is a hack, and this _should_ leak and increase the memory every time you hot reload.
	// This behavior is the same on Linux, Windows and most likely all the other platforms.

	FString Path = PluginPath();
	//FString PdbPath = PluginPdbPath();
	FString LocalTargetDllPath = FPaths::Combine(*PluginFolderPath(), PluginFileName());
	if (this->IsLoaded())
	{
		FPlatformProcess::FreeDllHandle(this->Handle);
		this->Handle = nullptr;
	}

	void* LocalHandle = FPlatformProcess::GetDllHandle(*LocalTargetDllPath);

	if (LocalHandle == nullptr)
	{
		UE_LOG(LogTemp, Warning, TEXT("Dll open failed"));
		return false;
	}

	this->Handle = LocalHandle;

	void* LocalBindings = FPlatformProcess::GetDllExport(LocalHandle, TEXT("register_unreal_bindings\0"));
	// void* LocalEditorTick = FPlatformProcess::GetDllExport(LocalHandle, TEXT("editor_tick\0"));
	this->TryLoadFunction = static_cast<TryLoadFn>(FPlatformProcess::GetDllExport(LocalHandle, TEXT("try_load\0")));
	ensure(LocalBindings);
	ensure(this->TryLoadFunction);
	// ensure(LocalEditorTick);
	
	// this->EditorTick = static_cast<TickFn>(LocalEditorTick);

	this->Bindings = static_cast<EntryUnrealBindingsFn>(LocalBindings);

	this->TargetPath = LocalTargetDllPath;
	NeedsInit = true;
	CallEntryPoints();
	return true;
}

void FRustPluginModule::Exit()
{
	if (GEditor)
	{
		GEditor->RequestEndPlayMap();
	}
}

bool FRustLoader::IsLoaded()
{
	return Handle != nullptr;
}

UE_DISABLE_OPTIMIZATION
void FRustLoader::CallEntryPoints()
{
	if (!IsLoaded())
		return;

	// Pass unreal function pointers to Rust, and also retrieve function pointers from Rust so we can call into Rust
	auto UnrealBindings = CreateBindings();
	if (Bindings(UnrealBindings, &Rust))
	{
		//Rust.initialize_modules();
	}
	else
	{
		// TODO: We had a panic when calling the entry point. We need to handle that better, otherwise unreal will segfault because the rust bindings are nullptrs
	}
}
UE_ENABLE_OPTIMIZATION

void FRustPluginModule::StartupModule()
{
	// This code will execute after your module is loaded into memory; the exact timing is specified in the .uplugin file per-module

	FRustPluginStyle::Initialize();
	FRustPluginStyle::ReloadTextures();

	FRustPluginCommands::Register();

	//PluginCommands = MakeShareable(new FUICommandList);

	//PluginCommands->MapAction(
	//    FRustPluginCommands::Get().OpenPluginWindow,
	//    FExecuteAction::CreateRaw(this, &FRustPluginModule::PluginButtonClicked),
	//    FCanExecuteAction());

	//UToolMenus::RegisterStartupCallback(FSimpleMulticastDelegate::FDelegate::CreateRaw(this, &FRustPluginModule::RegisterMenus));

	//FGlobalTabmanager::Get()->RegisterNomadTabSpawner(RustPluginTabName, FOnSpawnTab::CreateRaw(this, &FRustPluginModule::OnSpawnPluginTab)).SetDisplayName(LOCTEXT("FRustPluginTabTitle", "RustPlugin")).SetMenuType(ETabSpawnerMenuType::Hidden);

	//IDirectoryWatcher* watcher = FModuleManager::LoadModuleChecked<FDirectoryWatcherModule>(
	//		TEXT("DirectoryWatcher")).
	//	Get();
	//watcher->RegisterDirectoryChangedCallback_Handle(
	//	*Plugin.PluginFolderPath(),
	//	IDirectoryWatcher::FDirectoryChanged::CreateRaw(this, &FRustPluginModule::OnProjectDirectoryChanged),
	//	WatcherHandle, IDirectoryWatcher::WatchOptions::IgnoreChangesInSubtree);
	Plugin.TryLoad();

	//TSharedPtr<FUuidGraphPanelPinFactory> UuidFactory = MakeShareable(new FUuidGraphPanelPinFactory());
	//FEdGraphUtilities::RegisterVisualPinFactory(UuidFactory);

	// Register detail customizations
	{
		auto& PropertyModule = FModuleManager::LoadModuleChecked<FPropertyEditorModule>("PropertyEditor");

		//PropertyModule.RegisterCustomClassLayout(
		//	"EntityComponent",
		//	FOnGetDetailCustomizationInstance::CreateStatic(&FRustDetailCustomization::MakeInstance)
		//);

		//PropertyModule.RegisterCustomPropertyTypeLayout(
		//	FRustEvent::StaticStruct()->GetFName(),
		//	FOnGetPropertyTypeCustomizationInstance::CreateStatic(&FRustAnimNotifyDetailCustomization::MakeInstance)
		//);

		//PropertyModule.RegisterCustomClassLayout(
		//	"AnimNotify_RustEvent",
		//	FOnGetDetailCustomizationInstance::CreateStatic(&FRustAnimNotifyDetailCustomization::MakeInstance)
		//);

		PropertyModule.NotifyCustomizationModuleChanged();
	}
}

void FRustPluginModule::OnProjectDirectoryChanged(const TArray<FFileChangeData>& Data)
{
	for (FFileChangeData Changed : Data)
	{
		FString Name = FPaths::GetBaseFilename(Changed.Filename);
		FString Ext = FPaths::GetExtension(Changed.Filename, false);

		FString Leaf = FPaths::GetPathLeaf(FPaths::GetPath(Changed.Filename));
		const bool ChangedOrAdded = Changed.Action == FFileChangeData::FCA_Added || Changed.Action ==
			FFileChangeData::FCA_Modified;
		if (Name == TEXT("rustplugin") && Ext == *PlatformExtensionName() && ChangedOrAdded)
		{
			Plugin.TryLoad();
			// Only show notifications when we are in playmode otherwise notifications are a bit too spamy
			if (GEditor != nullptr && GEditor->IsPlaySessionInProgress())
			{
				// Still too spamy

				//FNotificationInfo Info(LOCTEXT("SpawnNotification_Notification", "Hotreload: Rust"));
				//Info.ExpireDuration = 1.0f;
				//FSlateNotificationManager::Get().AddNotification(Info);
			}

			UE_LOG(LogTemp, Display, TEXT("Hotreload: Rust"));

			return;
		}
	}
}

UWorld* URustEditorSubsystem::GetWorld() const
{
	return GEditor->GetEditorWorldContext().World();
}

void URustEditorSubsystem::Tick(float DeltaTime)
{
	auto& RustModule = GetRustModule();
	if (RustModule.Plugin.IsLoaded())
	{
		// RustModule.Plugin.EditorTick(DeltaTime);
		TRACE_CPUPROFILER_EVENT_SCOPE(Hotreload);
		if (RustModule.Plugin.TryLoadFunction(&RustModule.Plugin.Rust))
		{
			UE_LOG(LogTemp, Warning, TEXT("Hotreload"));
		}
	}
}

void URustEditorSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
	Super::Initialize(Collection);
}

void FRustPluginModule::ShutdownModule()
{
	// This function may be called during shutdown to clean up your module.  For modules that support dynamic reloading,
	// we call this function before unloading the module.

	UToolMenus::UnRegisterStartupCallback(this);

	UToolMenus::UnregisterOwner(this);

	FRustPluginStyle::Shutdown();

	FRustPluginCommands::Unregister();

	FGlobalTabmanager::Get()->UnregisterNomadTabSpawner(RustPluginTabName);
}

TSharedRef<SDockTab> FRustPluginModule::OnSpawnPluginTab(const FSpawnTabArgs& SpawnTabArgs)
{
	FText WidgetText = FText::Format(
		LOCTEXT("WindowWidgetText", "BAR Add code to {0} in {1} to override this window's contents"),
		FText::FromString(TEXT("FRustPluginModule::OnSpawnPluginTab")),
		FText::FromString(TEXT("RustPlugin.cpp")));

	return SNew(SDockTab)
		.TabRole(ETabRole::NomadTab)
		[
			// Put your tab content here!
			SNew(SBox)
			.HAlign(HAlign_Center)
			.VAlign(VAlign_Center)
			[
				SNew(STextBlock)
				.Text(WidgetText)
			]
		];
}

void FRustPluginModule::PluginButtonClicked()
{
	FGlobalTabmanager::Get()->TryInvokeTab(RustPluginTabName);
}

void FRustPluginModule::RegisterMenus()
{
	// Owner will be used for cleanup in call to UToolMenus::UnregisterOwner
	FToolMenuOwnerScoped OwnerScoped(this);

	{
		UToolMenu* Menu = UToolMenus::Get()->ExtendMenu("LevelEditor.MainMenu.Window");
		{
			FToolMenuSection& Section = Menu->FindOrAddSection("WindowLayout");
			Section.AddMenuEntryWithCommandList(FRustPluginCommands::Get().OpenPluginWindow, PluginCommands);
		}
	}

	{
		UToolMenu* ToolbarMenu = UToolMenus::Get()->ExtendMenu("LevelEditor.LevelEditorToolBar");
		{
			FToolMenuSection& Section = ToolbarMenu->FindOrAddSection("Settings");
			{
				FToolMenuEntry& Entry = Section.AddEntry(
					FToolMenuEntry::InitToolBarButton(FRustPluginCommands::Get().OpenPluginWindow));
				Entry.SetCommandList(PluginCommands);
			}
		}
	}
}

#undef LOCTEXT_NAMESPACE

// IMPLEMENT_MODULE(FRustPluginModule, RustPlugin)
IMPLEMENT_PRIMARY_GAME_MODULE(FRustPluginModule, RustPlugin, "RustPlugin");
