// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

#define TIME_SCOPE(Name, Expr) \
{ \
const double StartTime = FPlatformTime::Seconds(); \
Expr; \
const double EndTime = FPlatformTime::Seconds(); \
UE_LOG(LogTemp, Display, TEXT("%s took %.3f s\n"), TEXT(Name), (EndTime - StartTime)); \
}

struct RustReflection_Type
{
	virtual ~RustReflection_Type() = default;
	virtual TSharedPtr<FJsonObject> ToJson() = 0;
	int ArrayDim = 0;
};

struct FRustReflection_Property
{
	FString Name;

	TUniquePtr<RustReflection_Type> Type;

	TArray<FString> PropertyFlags;

	TOptional<FText> Documentation;

	int32 Offset;
	uint32 Size;
	int32 Alignment;

	static FRustReflection_Property FromProperty(FProperty* Property);
	TSharedPtr<FJsonObject> ToJson();
};

struct RustReflection_ConcreteType : public RustReflection_Type
{
	FString TypeName;
	TOptional<FString> Hint;
	virtual TSharedPtr<FJsonObject> ToJson() override;
};

struct RustReflection_Bitfield : public RustReflection_Type
{
	uint8 Offset;
	uint8 FieldMask;
	virtual TSharedPtr<FJsonObject> ToJson() override;
};

struct RustReflection_ContainerType : public RustReflection_Type
{
	FString ContainerTypeName;
	TUniquePtr<RustReflection_Type> InnerType;

	virtual TSharedPtr<FJsonObject> ToJson() override;
};

struct RustReflection_MapType : public RustReflection_Type
{
	FString ContainerName;
	TUniquePtr<RustReflection_Type> ValueType;
	TUniquePtr<RustReflection_Type> KeyType;

	virtual TSharedPtr<FJsonObject> ToJson() override;
};

static TUniquePtr<RustReflection_ConcreteType> MakeConcreteType(FString Name,
                                                                int32 ArrayDim,
                                                                TOptional<FString> Hint = TOptional<FString>());
static TUniquePtr<RustReflection_ContainerType> MakeContainerType(FString ContainerTypeName,
                                                                  TUniquePtr<RustReflection_Type> InnerType,
                                                                  int32 ArrayDim);
static TUniquePtr<RustReflection_Type> FromProperty(FProperty* Property);

struct FRustReflection_Param
{
	FRustReflection_Property Property;

	bool bIsRef = false;
	bool bIsOut = false;
	bool bIsReturn = false;

	static FRustReflection_Param FromProperty(FProperty* Property);
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_Function
{
	FString Name;

	TArray<FString> FunctionFlags;

	uint16 ParamSize;

	TArray<FRustReflection_Param> Parameters;

	TOptional<FText> Documentation;

	TMap<FString, FString> Metadata;

	static FRustReflection_Function FromFunction(UFunction* Function);
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_UStruct
{
	FString StructName;

	TOptional<FString> SuperStuctName;

	TMap<FString, FString> Metadata;
	TArray<FString> StructFlags;

	FString Package;

	TArray<FRustReflection_Property> Properties;

	TOptional<FText> Documentation;

	int16 MinAlignment = 0;
	int32 PropertySizes = 0;
	int32 Size = 0;

	bool bIsPlainOldData = false;

	static FRustReflection_UStruct FromStruct(UStruct* Struct);
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_UClass
{
	FString ClassName;

	TOptional<FString> SuperClassName;

	TMap<FString, FString> Metadata;
	TArray<FString> ClassFlags;
	bool bIsInterface = false;

	FString Package;

	int16 MinAlignment = 0;
	int32 PropertySizes = 0;

	TArray<FRustReflection_Property> Properties;

	TArray<FRustReflection_Function> Functions;

	TOptional<FText> Documentation;

	static FRustReflection_UClass FromClass(UClass* Class);
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_EnumEntry
{
	FString Name;

	int64 Value = 0;

	TOptional<FText> Documentation;
};

struct FRustReflection_EnumWithType
{
	UEnum* Enum;
	TUniquePtr<RustReflection_Type> Type;
};

struct FRustReflection_Enum
{
	FString Name;
	TUniquePtr<RustReflection_Type> Type;
	FString Kind;
	FString Package;

	TMap<FString, FString> Metadata;

	TArray<FRustReflection_EnumEntry> Entries;

	static FRustReflection_Enum FromEnum(FRustReflection_EnumWithType& EnumWithType);
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_Opague
{
	FString Name;
	int32 Size;
	int16 Alignment;
	TSharedPtr<FJsonObject> ToJson();
};

struct FRustReflection_Delegate
{
	FString Name;
	FString Package;
	FString Namespace;
	FString Kind;
	uint32 Alignment;
	uint32 Size;
	FRustReflection_Function Function;
	TSharedPtr<FJsonObject> ToJson();
};

static void WriteMetadataField(TSharedRef<FJsonObject> Json, TMap<FString, FString>& Metadata);

struct FRustReflection_Root
{
	// UPROPERTY()
	// TArray<FRustReflection_Enum> Enums;
	//
	// UPROPERTY()
	// TArray<FRustReflection_UStruct> Structs;
	//
	// UPROPERTY()
	// TArray<FRustReflection_UClass> Classes;


	static FRustReflection_Root Collect();
	static void ExportToJson_Classes(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_Structs(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_Delegates(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_Enum(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_GameplayTags(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_ConsoleVariables(TSharedPtr<FJsonObject> Json);
	static void ExportToJson_OpagueTypes(TSharedPtr<FJsonObject> Json);
};
