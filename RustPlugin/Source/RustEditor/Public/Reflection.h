// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "Reflection.generated.h"

USTRUCT()
struct FRustReflection_Type
{
	GENERATED_BODY()

	UPROPERTY()
	FString Type;

	UPROPERTY()
	TOptional<FString> ContainerType;

	UPROPERTY()
	TOptional<FString> KeyType;

	static FRustReflection_Type FromProperty(FProperty* Property);
};

USTRUCT()
struct FRustReflection_Property
{
	GENERATED_BODY()

	UPROPERTY()
	FName Name;

	UPROPERTY()
	FRustReflection_Type Type;

	UPROPERTY()
	TArray<FString> PropertyFlags;

	UPROPERTY()
	TOptional<FText> Documentation;

	static FRustReflection_Property FromProperty(FProperty* Property);
};

USTRUCT()
struct FRustReflection_Param
{
	GENERATED_BODY()

	UPROPERTY()
	FName Name;

	UPROPERTY()
	FRustReflection_Type Type;

	bool bIsRef = false;
	bool bIsOut = false;
	bool bIsReturn = false;

	UPROPERTY()
	TOptional<FText> Documentation;

	static FRustReflection_Param FromProperty(FProperty* Property);
};

USTRUCT()
struct FRustReflection_Function
{
	GENERATED_BODY()
	UPROPERTY()
	FName Name;

	UPROPERTY()
	TArray<FString> FunctionFlags;

	UPROPERTY()
	TArray<FRustReflection_Param> Parameters;

	UPROPERTY()
	TOptional<FText> Documentation;

	UPROPERTY()
	TMap<FString, FString> Metadata;

	static FRustReflection_Function FromFunction(UFunction* Function);
};

USTRUCT()
struct FRustReflection_UClass
{
	GENERATED_BODY()
	UPROPERTY()
	FString ClassName;

	UPROPERTY()
	TOptional<FString> SuperClassName;

	UPROPERTY()
	TArray<FString> ClassFlags;

	UPROPERTY()
	TArray<FRustReflection_Property> Properties;
	UPROPERTY()
	TArray<FRustReflection_Function> Functions;

	UPROPERTY()
	TOptional<FText> Documentation;

	static FRustReflection_UClass FromClass(UClass* Class);
};

USTRUCT()
struct FRustReflection_EnumEntry
{
	GENERATED_BODY()
	
	UPROPERTY()
	FString Name;
	
	UPROPERTY()
	int64 Value;
	
	UPROPERTY()
	TOptional<FText> Documentation;
};

USTRUCT()
struct FRustReflection_Enum
{
	GENERATED_BODY()

	UPROPERTY()
	FString Name;
	
	UPROPERTY()
	TArray<FRustReflection_EnumEntry> Entries;
	
	static FRustReflection_Enum FromEnum(UEnum* Enum);
};

USTRUCT()
struct FRustReflection_Root
{
	GENERATED_BODY()
	
	UPROPERTY()
	TArray<FRustReflection_Enum> Enums;

	UPROPERTY()
	TArray<FRustReflection_UClass> Classes;
	

	static FRustReflection_Root Collect();
};
