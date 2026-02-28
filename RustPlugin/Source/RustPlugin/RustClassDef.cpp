// Fill out your copyright notice in the Description page of Project Settings.


#include "RustClassDef.h"

#include "RustPlugin.h"
#include "RustUtils.h"

void URustExtension_RustClassDef::AddNumeric(FRustClassDef& Def, FString Name, int Offset)
{
	FRustPropertyDefinition Property;
	Property.Name = Name;
	Property.Offset = Offset;
	Property.Type = NewObject<URustType_Numeric>(GetTransientPackage());
	Def.PropertyDefinitions.Add(MoveTemp(Property));
}

void URustExtension_RustClassDef::Register(const FRustClassDef& Def)
{
	GetRustModule().Plugin.Types.Add(Def.Name, Def);
}

FRustClassDef URustExtension_RustClassDef::New(FString Name, int Size)
{
	auto Def =  FRustClassDef();
	Def.Size = Size;
	Def.Name = Name;
	return Def;
}
