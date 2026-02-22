// Fill out your copyright notice in the Description page of Project Settings.


#include "RustExtension.h"

FHitResult URustExtension_FHitResult::New()
{
	return FHitResult();
}

UObject* URustExtension_Core::NewObject(UObject* Outer, TSubclassOf<UObject> Class, FName Name)
{
	return ::NewObject<UObject>(Outer, Class.Get(), Name);
}

FName URustExtension_Core::FNameNone()
{
	return NAME_None;
}

void URustExtension_RustTypeDef::AddNumeric(FRustTypeDef& Def, FRustTypeInfo TypeInfo)
{
	auto Numeric = MakeUnique<FRustType_Numeric>();
	Numeric->TypeInfo = TypeInfo;
	Def.Types.Add(MoveTemp(Numeric));
}
