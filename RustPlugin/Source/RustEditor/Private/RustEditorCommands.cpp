// Fill out your copyright notice in the Description page of Project Settings.


#include "RustEditorCommands.h"

#define LOCTEXT_NAMESPACE "FRustEditorModule"

void FRustEditorCommands::RegisterCommands()
{
	UI_COMMAND(DumpUnrealReflectionApi, "Dump Reflection Api", "Dump", EUserInterfaceActionType::Button, FInputChord());
}

#undef LOCTEXT_NAMESPACE
