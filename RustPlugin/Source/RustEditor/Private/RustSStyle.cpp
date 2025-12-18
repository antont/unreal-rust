#include "RustSStyle.h"
#include "Framework/Application/SlateApplication.h"
#include "Interfaces/IPluginManager.h"
#include "Styling/SlateStyleRegistry.h"
#include "Styling/SlateStyleMacros.h"

#define RootToContentDir Style->RootToContentDir

TSharedPtr<FSlateStyleSet> FRustStyle::StyleInstance = nullptr;

void FRustStyle::Initialize()
{
	if (!StyleInstance.IsValid())
	{
		StyleInstance = Create();
		FSlateStyleRegistry::RegisterSlateStyle(*StyleInstance);
	}
}

void FRustStyle::Shutdown()
{
	FSlateStyleRegistry::UnRegisterSlateStyle(*StyleInstance);
	ensure(StyleInstance.IsUnique());
	StyleInstance.Reset();
}

void FRustStyle::ReloadTextures()
{
	if (FSlateApplication::IsInitialized())
	{
		FSlateApplication::Get().GetRenderer()->ReloadTextureResources();
	}
}

const ISlateStyle& FRustStyle::Get()
{
	return *StyleInstance;
}

FName FRustStyle::GetStyleSetName()
{
	static FName StyleSetName(TEXT("RustStyle"));
	return StyleSetName;
}

TSharedRef<FSlateStyleSet> FRustStyle::Create()
{
	TSharedRef<FSlateStyleSet> Style = MakeShareable(new FSlateStyleSet("RustStyle"));
	Style->SetContentRoot(IPluginManager::Get().FindPlugin(UE_PLUGIN_NAME)->GetBaseDir() / TEXT("Resources"));
	Style->Set("Rust.Toolbar.Header", new IMAGE_BRUSH(TEXT("rust-lang-logo-white-128x128"), FVector2D(128,128)));
	return Style;
}
