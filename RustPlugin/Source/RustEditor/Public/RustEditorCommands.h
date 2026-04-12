// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "RustSStyle.h"

class FRustEditorCommands : public TCommands<FRustEditorCommands>
{
public:
	FRustEditorCommands() : TCommands<FRustEditorCommands>
	(
		"RustEditor",
		NSLOCTEXT("Contexts", "Rust", "Rust"),
		NAME_None,
		FRustStyle::GetStyleSetName()
	)
	{
	}

	// TCommands<> interface
	virtual void RegisterCommands() override;
	// End

	TSharedPtr<FUICommandInfo> DumpUnrealReflectionApi;
};
