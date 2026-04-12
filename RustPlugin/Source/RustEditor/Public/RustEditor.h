#pragma once

#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"


class FRustEditorModule : public IModuleInterface
{
public:
    virtual void StartupModule() override;
    virtual void ShutdownModule() override;
    
    void RegisterMenu();
    void OnDumpReflectionApi();
    void RegisterCommands();
    TSharedRef<SWidget> GenerateRustMenu() const;
    TSharedPtr<FUICommandList> RustCommands;
};
