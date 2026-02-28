// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Bindings.h"
#include "Containers/Array.h"
#include "Modules/ModuleInterface.h"
#include "EditorSubsystem.h"
#include "Engine/World.h"
#include "RustClassDef.h"

#include "RustPlugin.generated.h"


class FToolBarBuilder;
class FMenuBuilder;
class FDelegateHandle;
struct FFileChangeData;
class FString;
class ARustGameModeBase;

struct FRustLoader
{
	FString TargetPath;
	void* Handle;
	EntryUnrealBindingsFn Bindings;
	TickFn EditorTick;
	TryLoadFn TryLoadFunction;
	RustBindings Rust;

	TMap<FString, FRustClassDef> Types;
	bool NeedsInit;
	bool IsLoaded();
	bool TryLoad();
	void CallEntryPoints();
	void RegisterTypes();
	void RetrieveReflectionData();
	FString PluginFolderPath();
	FString PluginPath();
	FString PluginFileName();
	FString PluginPdbPath();

	FRustLoader();
};

FString PlatformExtensionName();


class FRustPluginModule : public IModuleInterface
{
public:
	/** IModuleInterface implementation */
	virtual void StartupModule() override;
	virtual void ShutdownModule() override;

	/** This function will be bound to Command (by default it will bring up plugin window) */
	void PluginButtonClicked();
	
	UPackage* GetPackage() const
	{
		return RustPackage;
	}

	FRustLoader Plugin;
	ARustGameModeBase* GameMode;
	UPackage* RustPackage;
	void Exit();
private:
	void RegisterMenus();

	TSharedRef<class SDockTab> OnSpawnPluginTab(const class FSpawnTabArgs& SpawnTabArgs);
	void OnProjectDirectoryChanged(const TArray<FFileChangeData>& Data);

private:
	TSharedPtr<class FUICommandList> PluginCommands;
	FDelegateHandle WatcherHandle;
};


UCLASS()
class URustEditorSubsystem : public UEditorSubsystem, public FTickableGameObject
{
	GENERATED_BODY()
	
	UWorld* GetWorld() const; // Fallback to GWorld
	virtual bool IsTickableInEditor() const { return true; }
	virtual void Tick(float DeltaTime) override;
	virtual void Initialize(FSubsystemCollectionBase& Collection) override;
	TStatId GetStatId() const override { RETURN_QUICK_DECLARE_CYCLE_STAT(URustEditorSubsystem, STATGROUP_Tickables); }
	
};
