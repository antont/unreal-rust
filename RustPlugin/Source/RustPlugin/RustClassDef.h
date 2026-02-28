// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "UObject/Object.h"
#include "UObject/StrongObjectPtr.h"
#include "RustClassDef.generated.h"

class URustType;

USTRUCT()
struct FRustPropertyDefinition
{
	GENERATED_BODY()
	int Offset;
	FString Name;
	UPROPERTY()
	TObjectPtr<URustType> Type;
};

UCLASS(Abstract)
class RUSTPLUGIN_API URustType : public UObject
{
	GENERATED_BODY()
};

UCLASS()
class RUSTPLUGIN_API URustType_Numeric : public URustType
{
	GENERATED_BODY()
};

USTRUCT(BlueprintType)
struct FRustClassDef
{
	GENERATED_BODY()

	FString Name;
	int Size;
	TArray<FRustPropertyDefinition> PropertyDefinitions;
};

UCLASS(meta=(Impl=FRustClassDef))
class URustExtension_RustClassDef : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()
	UFUNCTION(BlueprintCallable)
	static void AddNumeric(FRustClassDef& Def, FString Name, int Offset);
	
	UFUNCTION(BlueprintCallable)
	static void Register(const FRustClassDef& Def);

	UFUNCTION(BlueprintCallable)
	static FRustClassDef New(FString Name, int Size);
};
